use anyhow::Result;

use crate::classification::CAT_MODEL;

pub fn classify<'a>(text: &'a str, topics: &'a [&str]) -> Result<String> {
    let model = (*CAT_MODEL).lock();

    let output = model.predict(&[text], topics, None, 128);

    Ok(output[0].text.clone())
}
