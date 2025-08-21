pub mod processor;
pub mod queue;

use crate::db::DatabaseManager;
use minacalc_rs::Calc;
use std::sync::{Arc, Mutex};

static mut CALC: Option<Calc> = None;
static PROCESSOR: Mutex<Option<Arc<Mutex<BeatmapProcessor>>>> = Mutex::new(None);

#[derive(Clone)]
pub struct BeatmapProcessor {
    is_processing: bool,
    db: Option<DatabaseManager>,
}

impl BeatmapProcessor {
    pub fn new(db: Option<DatabaseManager>) -> Self {
        Self {
            is_processing: false,
            db,
        }
    }

    pub fn instance() -> BeatmapProcessor {
        let mut processor = PROCESSOR.lock().unwrap();
        if processor.is_none() {
            *processor = Some(Arc::new(Mutex::new(Self::new(None))));
        }
        processor.as_ref().unwrap().lock().unwrap().clone()
    }

    pub fn instance_mut() -> std::sync::MutexGuard<'static, Option<Arc<Mutex<BeatmapProcessor>>>> {
        let mut processor = PROCESSOR.lock().unwrap();
        if processor.is_none() {
            *processor = Some(Arc::new(Mutex::new(Self::new(None))));
        }
        processor
    }

    pub fn initialize(db: DatabaseManager) {
        // Récupérer le processor existant et le modifier
        let mut processor_guard = PROCESSOR.lock().unwrap();
        if let Some(processor_arc) = processor_guard.as_ref() {
            let mut processor = processor_arc.lock().unwrap();
            processor.db = Some(db);
        } else {
            // Si le processor n'existe pas encore, le créer
            *processor_guard = Some(Arc::new(Mutex::new(Self::new(Some(db)))));
        }
        drop(processor_guard); // Libérer le lock

        // Vérifier que la modification a bien été prise en compte
        let processor_guard = PROCESSOR.lock().unwrap();
        if let Some(processor_arc) = processor_guard.as_ref() {
            let processor = processor_arc.lock().unwrap();
            if processor.db.is_none() {
                println!("Database is still none after initialization");
            } else {
                println!("Database initialized successfully");
            }
        }

        // Initialiser le Calc global une seule fois
        unsafe {
            let calc_ptr = &raw mut CALC;

            // lire la valeur via raw pointer
            if (*calc_ptr).is_none() {
                // écrire dans la valeur via raw pointer
                *calc_ptr = Some(Calc::new().unwrap());
            }
        }
    }
}
