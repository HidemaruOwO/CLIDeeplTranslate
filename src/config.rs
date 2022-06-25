use confy;
use confy::ConfyError;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct AppConfig {
    api_key: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            api_key: "".to_string(),
        }
    }
}

pub fn write_api_key(api_key: String) -> Result<(), ConfyError> {
    let mut cfg = confy::load::<AppConfig>("cli_deepl")?;
    cfg.api_key = api_key;
    confy::store("cli_deepl", cfg)?;
    Ok(());
    return;
}

pub fn load_config() -> Result<AppConfig, ConfyError> {
    let mut cfg = confy::load::<AppConfig>("cli_deepl")?;
    Ok(());
    return cfg;
}
