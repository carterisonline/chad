use std::sync::Arc;

use anyhow::Result;

use once_cell::sync::Lazy;
use parking_lot::Mutex;

pub static STATUS: Lazy<Arc<Mutex<Vec<String>>>> = Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

pub fn get_status(id: usize) -> Result<String> {
    Ok(String::new())
}
