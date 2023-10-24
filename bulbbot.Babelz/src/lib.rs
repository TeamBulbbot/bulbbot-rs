use rust_i18n::{i18n, Backend};
// use std::collections::HashMap;
i18n!("locales", fallback = "en");

pub struct I18n {
    // translations: HashMap<String, HashMap<String, String>>,
}

impl I18n {
    pub fn new() -> Self {
        /* let f = std::fs::File::open("./locales/app.yaml").unwrap();
        let translations: HashMap<String, HashMap<String, String>> =
            serde_yaml::from_reader(f).unwrap();

        Self {
            translations
         }*/

        Self {}
    }
}

impl Backend for I18n {
    fn available_locales(&self) -> Vec<&str> {
        _RUST_I18N_BACKEND.available_locales()
    }

    fn translate(&self, locale: &str, key: &str) -> Option<&str> {
        let val = _RUST_I18N_BACKEND.translate(locale, key);
        if val.is_none() {
            _RUST_I18N_BACKEND.translate("en", key)
        } else {
            val
        }
    }
}
