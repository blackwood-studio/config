use std::fs::read_to_string;

use anyhow::Error;
use serde::de::DeserializeOwned;
use toml::from_str;

pub struct Load {  }

impl Load {
    pub fn from<T>(path: &str) -> Result<T, Error> 
    where T: DeserializeOwned {
        let config_str = read_to_string(path)?;
        let config_struct: T = from_str(&config_str)?;
        Ok(config_struct)
    }
}
