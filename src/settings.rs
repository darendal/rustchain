use std::env;
use config::{ConfigError, Config, File, Environment};

#[derive(Debug, Deserialize)]
pub struct BlockSettings {
    pub number_of_zeroes: usize,
    pub chain_directory: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub block_settings: BlockSettings
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name("config/default"));


        let env = env::var("RUN_MODE").unwrap_or("development".into());

        println!("{}", env);

        s.merge(File::with_name(&format!("config/{}", env)).required(false));

        s.try_into()
    }
}