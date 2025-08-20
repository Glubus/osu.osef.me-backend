use anyhow::Result;
use rosu_v2::prelude::*;
use std::sync::{Arc, Mutex, Once};
use std::collections::HashMap;
use tracing::debug;

static API_SERVICE: Mutex<Option<Arc<OsuApiService>>> = Mutex::new(None);

#[derive(Clone)]
pub struct OsuApiService {
    client: Arc<Osu>,
}

impl OsuApiService {
    pub fn instance() -> Arc<Self> {
        let service = API_SERVICE.lock().unwrap();
        service.as_ref().expect("OsuApiService not initialized. Call initialize() first.").clone()
    }

    pub async fn initialize(client_id: u64, client_secret: String) -> Result<()> {
        let client = Osu::new(client_id, client_secret).await?;
        
        let mut service = API_SERVICE.lock().unwrap();
        *service = Some(Arc::new(OsuApiService {
            client: Arc::new(client),
        }));
        
        Ok(())
    }

    // checksum = hash of the beatmap file
    pub async fn beatmap_by_checksum(&self, checksum: String) -> Result<BeatmapExtended> {
        let beatmap = self.client.beatmap().checksum(checksum).await?;
        Ok(beatmap)
    } 
}