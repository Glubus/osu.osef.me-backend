pub mod queue;
pub mod processor;

use std::sync::{Arc, Mutex};
use crate::db::DatabaseManager;
use minacalc_rs::Calc;

static mut CALC: Option<Calc> = None;
static PROCESSOR: Mutex<Option<Arc<BeatmapProcessor>>> = Mutex::new(None);

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

    pub fn instance() -> Arc<Self> {
        let mut processor = PROCESSOR.lock().unwrap();
        if processor.is_none() {
            *processor = Some(Arc::new(Self::new(None)));
        }
        processor.as_ref().unwrap().clone()
    }

    pub fn initialize(&mut self, db: DatabaseManager) {
        self.db = Some(db);
        
        // Initialiser le Calc global une seule fois
        unsafe {
            if CALC.is_none() {
                CALC = Some(Calc::new().unwrap());
            }
        }
    }
}
