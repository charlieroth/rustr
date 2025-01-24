use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "nsec")]
    pub nsec: String,
    #[serde(rename = "relays")]
    pub relays: Vec<String>,
}

impl Config {
    pub fn load(config_file_path: &str) -> anyhow::Result<Self> {
        let config_file_str = std::fs::read_to_string(config_file_path).unwrap();
        let config: Config = serde_yml::from_str(&config_file_str).unwrap();
        Ok(config)
    }
}
