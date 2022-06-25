use deepl_api::*;

let mut deepl;

pub fn set_api_key(key: String) {
    deepl=Deepl::new(key);
}

pub fn translate(text: String, lang: String) -> String {
    if (usage_limit() == 0) {
        return;
    }
    let texts = TranslateTextList {
        source_language: Some("DE".to_string()),
        target_language: lang,
        texts: vec!(text),
    };
    let translated = deepl.translate(None, texts).unwrap();
    return translated.texts[0].to_string();
}

pub fn usage_limit() -> i64 {
    let usage_information = deepl.usage_information().unwrap();
    return usage_information.characters_limit;
}
