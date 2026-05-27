use std::collections::HashMap;
use std::sync::OnceLock;
use serde_json::Value;

static INSTANCE: OnceLock<I18n> = OnceLock::new();

pub struct I18n {
    translations: HashMap<String, Value>,
}

impl I18n {
    pub fn init() {
        INSTANCE.get_or_init(|| {
            let mut translations = HashMap::new();

            let Ok(entries) = std::fs::read_dir("locales") else {
                eprintln!("locales/ directory not found");
                return I18n { translations };
            };

            for entry in entries.flatten() {
                let path = entry.path();
                let is_json = path.extension().map(|e| e == "json").unwrap_or(false);
                if !is_json { continue; }

                let Some(lang) = path.file_stem().and_then(|s| s.to_str()).map(String::from) else {
                    continue;
                };

                match std::fs::read_to_string(&path) {
                    Err(e) => eprintln!("Failed to read locales/{}.json: {}", lang, e),
                    Ok(content) => match serde_json::from_str(&content) {
                        Err(e) => eprintln!("Failed to parse locales/{}.json: {}", lang, e),
                        Ok(json) => { translations.insert(lang, json); }
                    }
                }
            }

            I18n { translations }
        });
    }

    pub fn t(lang: &str, key: &str) -> String {
        let Some(i18n) = INSTANCE.get() else {
            return key.to_string();
        };

        i18n.lookup(lang, key)
            .or_else(|| i18n.lookup("en", key))
            .unwrap_or_else(|| String::from(""))
    }

    fn lookup(&self, lang: &str, key: &str) -> Option<String> {
        let mut current = self.translations.get(lang)?;
        for part in key.split('.') {
            current = current.get(part)?;
        }
        current.as_str().map(|s| s.to_string())
    }
}
