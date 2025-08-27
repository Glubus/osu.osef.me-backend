use crate::db::DatabaseManager;
use minacalc_rs::Calc;
use std::ptr;
use std::sync::{Arc, Mutex, Once};

static INIT: Once = Once::new();
static mut CALC: *mut Calc = ptr::null_mut();
static PROCESSOR: Mutex<Option<Arc<Mutex<BeatmapProcessor>>>> = Mutex::new(None);

#[derive(Clone)]
pub struct BeatmapProcessor {
    pub db: Option<DatabaseManager>,
}

impl BeatmapProcessor {
    pub fn new(db: Option<DatabaseManager>) -> Self {
        Self { db }
    }

    pub fn instance() -> BeatmapProcessor {
        let mut processor = PROCESSOR.lock().unwrap();
        if processor.is_none() {
            *processor = Some(Arc::new(Mutex::new(Self::new(None))));
        }
        processor.as_ref().unwrap().lock().unwrap().clone()
    }

    pub fn initialize(db: DatabaseManager) {
        let mut processor_guard = PROCESSOR.lock().unwrap();
        if let Some(processor_arc) = processor_guard.as_ref() {
            let mut processor = processor_arc.lock().unwrap();
            processor.db = Some(db);
        } else {
            *processor_guard = Some(Arc::new(Mutex::new(Self::new(Some(db)))));
        }
        drop(processor_guard);

        // Initialize CALC once, thread-safely
        INIT.call_once(|| unsafe {
            if let Ok(calc) = Calc::new() {
                let boxed_calc = Box::into_raw(Box::new(calc));
                CALC = boxed_calc;
            }
        });
    }

    pub fn get_calc() -> Option<&'static Calc> {
        unsafe { if CALC.is_null() { None } else { Some(&*CALC) } }
    }
}
