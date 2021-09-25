use std::sync::{Arc, Mutex};
use std::thread;

use once_cell::sync::Lazy;
use rust_bert::bert::{BertConfigResources, BertModelResources, BertVocabResources};
use rust_bert::pipelines::common::ModelType;
use rust_bert::pipelines::question_answering::{QuestionAnsweringConfig, QuestionAnsweringModel};
use rust_bert::resources::{RemoteResource, Resource};
use wikipedia::http::default::Client;
use wikipedia::Wikipedia;

pub mod question_answering;

pub static QA_MODEL: Lazy<Arc<Mutex<QuestionAnsweringModel>>> = Lazy::new(|| {
    thread::spawn(|| {
        let config = QuestionAnsweringConfig::new(
            ModelType::Bert,
            Resource::Remote(RemoteResource::from_pretrained(BertModelResources::BERT_QA)),
            Resource::Remote(RemoteResource::from_pretrained(
                BertConfigResources::BERT_QA,
            )),
            Resource::Remote(RemoteResource::from_pretrained(BertVocabResources::BERT_QA)),
            None,
            true,
            false,
            None,
        );
        Arc::new(Mutex::new(QuestionAnsweringModel::new(config).unwrap()))
    })
    .join()
    .unwrap()
});

pub static WIKI_CLIENT: Lazy<Wikipedia<Client>> = Lazy::new(|| Wikipedia::default());
