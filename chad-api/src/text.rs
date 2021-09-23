use json_gettext::{static_json_gettext_build, JSONGetText};
use once_cell::sync::Lazy;

use crate::PlainResponse;

static TEXT: Lazy<JSONGetText> = Lazy::new(|| {
    static_json_gettext_build!(
        "en_US";
        "en_US" => "lang/en_US.json"
    )
    .unwrap()
});

#[inline(always)]
pub fn get_text<K: AsRef<str>>(key: K) -> String {
    (*TEXT).get_text(key).unwrap().to_string()
}

#[inline(always)]
pub fn get_text_lonely<K: AsRef<str>>(key: K) -> String {
    PlainResponse::new(get_text(key)).to_string()
}
