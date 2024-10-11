use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub keys: HashMap<String, String>,
}

fn parse_yaml<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}

pub fn set_config_as_env_vars() {
    let config = parse_yaml("config.yaml").unwrap();

    for (key, value) in &config.keys {
        env::set_var(key, value);
    }
}
