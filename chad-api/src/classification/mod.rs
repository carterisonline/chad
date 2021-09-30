use std::sync::Arc;
use std::thread;

use once_cell::sync::Lazy;
use parking_lot::Mutex;
use rust_bert::bart::{
    BartConfigResources, BartMergesResources, BartModelResources, BartVocabResources,
};
use rust_bert::pipelines::common::ModelType;
use rust_bert::pipelines::zero_shot_classification::{
    ZeroShotClassificationConfig, ZeroShotClassificationModel,
};
use rust_bert::resources::{RemoteResource, Resource};

mod classify;
pub use classify::classify;

pub static CAT_MODEL: Lazy<Arc<Mutex<ZeroShotClassificationModel>>> = Lazy::new(|| {
    thread::spawn(|| {
        Arc::new(Mutex::new(
            ZeroShotClassificationModel::new(Default::default()).unwrap(),
        ))
    })
    .join()
    .unwrap()
});
