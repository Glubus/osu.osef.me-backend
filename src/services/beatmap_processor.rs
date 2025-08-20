use std::sync::{Arc, Mutex, Once};
use std::collections::VecDeque;
use std::thread;
use std::time::Duration;
use anyhow::Result;
use tracing::{info, warn, error, debug};
use crate::db::DatabaseManager;
use crate::services::osu_api::OsuApiService;
use crate::models::beatmap::beatmap::Beatmap;
use crate::models::beatmap::beatmapset::Beatmapset;
use rosu_v2::model::GameMode;
use crate::helpers::beatmap::osu_file_from_url;
use crate::services::etterna_rating::osu_to_notes;
use crate::services::etterna_rating::calculate_etterna_rating;
use minacalc_rs::Calc;
use crate::models::ratings::msd::MSD;
use crate::models::beatmap::failed_query::FailedQuery;
static mut CALC: Option<Calc> = None;
static PROCESSOR: Mutex<Option<Arc<BeatmapProcessor>>> = Mutex::new(None);

pub struct BeatmapProcessor {
    queue: Arc<Mutex<VecDeque<String>>>,
    is_processing: Arc<Mutex<bool>>,
    db: Arc<Mutex<Option<DatabaseManager>>>,
}

impl BeatmapProcessor {
    /// Retourne le nombre d'éléments restants dans la queue
    pub fn queue_size(&self) -> usize {
        self.queue.lock().unwrap().len()
    }

    pub fn instance() -> Arc<Self> {
        let mut processor = PROCESSOR.lock().unwrap();
        if processor.is_none() {
            *processor = Some(Arc::new(BeatmapProcessor {
                queue: Arc::new(Mutex::new(VecDeque::new())),
                is_processing: Arc::new(Mutex::new(false)),
                db: Arc::new(Mutex::new(None)),
            }));
        }
        processor.as_ref().unwrap().clone()
    }

    pub fn initialize(&self, db: DatabaseManager) {
        let mut db_guard = self.db.lock().unwrap();
        *db_guard = Some(db);
        // Initialiser le Calc global une seule fois
        unsafe {
            if (*&raw const CALC).is_none() {
                CALC = Some(Calc::new().unwrap());
            }
        }
    }

    pub async fn add_checksums(&self, checksums: Vec<String>) -> Result<()> {
        let mut queue = self.queue.lock().unwrap();
        
        // Limiter à 50 checksums maximum
        let checksums_to_add: Vec<String> = checksums.into_iter().take(50).collect();
        
        for checksum in checksums_to_add {
            queue.push_back(checksum);
        }

        // Démarrer le thread de traitement s'il n'est pas déjà en cours
        self.start_processing_thread();
        
        Ok(())
    }

    fn start_processing_thread(&self) {
        let is_processing = self.is_processing.clone();
        let queue = self.queue.clone();
        let db = self.db.clone();

        let mut processing = is_processing.lock().unwrap();
        if *processing {
            return; // Thread déjà en cours
        }
        *processing = true;
        drop(processing);

        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                Self::process_queue(queue, is_processing, db).await;
            });
        });
    }

    async fn process_queue(
        queue: Arc<Mutex<VecDeque<String>>>,
        is_processing: Arc<Mutex<bool>>,
        db: Arc<Mutex<Option<DatabaseManager>>>,
    ) {
        loop {
            let checksum = {
                let mut queue_guard = queue.lock().unwrap();
                queue_guard.pop_front()
            };

            match checksum {
                Some(checksum) => {
                    if let Err(e) = Self::process_single_checksum(&db, checksum.clone()).await {
                        let db_guard = db.lock().unwrap();  
                        if let Some(db_ref) = db_guard.as_ref() {
                            FailedQuery::insert(db_ref.get_pool(), &checksum).await.unwrap();
                        }
                        error!("Erreur lors du traitement du checksum: {}", e);
                    }
                }
                None => {
                    // Queue vide, arrêter le thread
                    let mut processing = is_processing.lock().unwrap();
                    *processing = false;
                    break;
                }
            }

            // Limite de 1 requête par seconde
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }

    async fn process_single_checksum(
        db: &Arc<Mutex<Option<DatabaseManager>>>,
        checksum: String,
    ) -> Result<()> {
        // Récupérer les services depuis les mutex
        let db_guard = db.lock().unwrap();
        let db_ref = db_guard.as_ref().ok_or_else(|| anyhow::anyhow!("Database not initialized"))?;
        
        if FailedQuery::exists_by_hash(db_ref.get_pool(), &checksum).await? {
            return Err(anyhow::anyhow!("Checksum {} existe déjà dans failed_query, ignoré", checksum));
        }

        // Vérifier si le checksum existe déjà dans la base
        if Beatmap::exists_by_checksum(db_ref.get_pool(), &checksum).await? {
            return Ok(());
        }
        
        // Utiliser le singleton de l'API
        let osu_api = OsuApiService::instance();

        // Récupérer le beatmap depuis l'API osu!
        let beatmap_extended = osu_api.beatmap_by_checksum(checksum.clone()).await?;

        // Vérifier si c'est une map mania (mode = 3)
        if beatmap_extended.mode != GameMode::Mania {
            return Err(anyhow::anyhow!("Beatmap non mania, ignoré : {}", beatmap_extended.mode));
        }

        // Vérifier si circle_size = 4.0
        if beatmap_extended.cs != 4.0 {
            return Err(anyhow::anyhow!("Beatmap non mania avec CS=4.0, ignoré : {}", beatmap_extended.cs));
        }
        
        // Gérer le beatmapset
        let beatmapset_id = if let Some(mapset_extended) = beatmap_extended.mapset.as_ref() {
            let beatmapset = Beatmapset::from(*beatmap_extended.mapset.clone().unwrap());
            
            // Vérifier si le beatmapset existe déjà
            if Beatmapset::exists_by_osu_id(db_ref.get_pool(), beatmapset.osu_id).await? {
                // Récupérer l'ID du beatmapset existant
                if let Some(existing_beatmapset) = Beatmapset::find_by_osu_id(db_ref.get_pool(), beatmapset.osu_id).await? {
                    existing_beatmapset.id
                } else {
                    return Err(anyhow::anyhow!("Beatmapset existe mais impossible à récupérer"));
                }
            } else {
                // Insérer le nouveau beatmapset
                Some(beatmapset.insert_into_db(db_ref.get_pool()).await?)
            }
        } else {
            None
        };
        
        // Convertir et insérer le beatmap avec le beatmapset_id
        let mut beatmap = Beatmap::from(beatmap_extended);
        beatmap.beatmapset_id = beatmapset_id;
        let _id = beatmap.insert_into_db(db_ref.get_pool()).await?;

        // Récupérer le fichier osu!
        match osu_file_from_url(&beatmap.file_path).await {
            Ok(osu_file) => {
                // Calculer le MSD
                let notes = osu_to_notes(&osu_file).map_err(|e| anyhow::anyhow!("{}", e))?;
                unsafe {
                    if let Some(calc) = &*&raw const CALC {
                        let msd = calculate_etterna_rating(&notes, calc).map_err(|e| anyhow::anyhow!("{}", e))?;
                        let mut msd : MSD = MSD::from(msd.msds[3]);
                        msd.beatmap_id = Some(_id);
                        let _insert_msd = msd.insert_into_db(db_ref.get_pool()).await?;
                    } else {
                        error!("Calc non initialisé");
                    }
                }
            }
            Err(e) => {
                error!("Erreur lors de la récupération du fichier osu!: {}", e);
                return Err(anyhow::anyhow!("Erreur lors de la récupération du fichier osu!: {}", e));
            }
        }



        Ok(())
    }
}
