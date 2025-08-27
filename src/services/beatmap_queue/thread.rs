use crate::services::beatmap_queue::processor::BeatmapProcessor;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use tracing::{error, info};

static PROCESSING_THREAD_RUNNING: AtomicBool = AtomicBool::new(false);

impl BeatmapProcessor {
    pub fn start_processing_thread(&mut self) {
        if PROCESSING_THREAD_RUNNING.load(Ordering::Relaxed) {
            error!("Processing thread already running, impossible to restart");
            return;
        }

        PROCESSING_THREAD_RUNNING.store(true, Ordering::Relaxed);
        self.spawn_processing_thread();
    }

    fn spawn_processing_thread(&self) {    
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                info!("Processing thread started");

                loop {
                    let processor = BeatmapProcessor::instance();
                    if let Ok(Some(pending)) = processor.pending_beatmap().await {
                        let _ = crate::services::beatmap_queue::handler::handle_pending(&pending).await;
                    } else {
                        tokio::time::sleep(Duration::from_secs(10)).await;
                    }
                }
            });
        });
    }
}
