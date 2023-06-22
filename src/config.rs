use std::{env, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    version: String,
    pub projects_configs_path: PathBuf,
    pub command: String,
}

impl Config {
    pub fn get() -> Self {
        Self::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut projects_configs_path = PathBuf::from(env::var("HOME").unwrap_or_default());
        projects_configs_path.push("Projects");
        projects_configs_path.push(".launch");
        Self {
            version: "0.1.0".to_owned(),
            command: "dmenu".to_owned(),
            projects_configs_path,
        }
    }
}
