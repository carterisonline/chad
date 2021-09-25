use rust_bert::pipelines::question_answering::QaInput;

use super::{QA_MODEL, WIKI_CLIENT};

pub async fn answer_wiki_question(question: &str) -> Option<String> {
    if let Ok(articles) = (*WIKI_CLIENT).search(question) {
        if let Some(first_article) = articles.get(0) {
            if let Ok(summary) = (*WIKI_CLIENT)
                .page_from_title(first_article.clone())
                .get_summary()
            {
                let neural_question = QaInput {
                    question: question.to_string(),
                    context: summary,
                };

                let qa_model = (*QA_MODEL).lock().unwrap();
                if let Some(prediction) = qa_model.predict(&[neural_question], 1, 32).get(0) {
                    if let Some(answer) = prediction.get(0) {
                        return Some(answer.answer.clone());
                    }
                }
            }
        }
    }

    return None;
}
