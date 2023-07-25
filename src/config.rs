use std::{env, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    version: String,
    pub projects_configs_path: PathBuf,
    pub menu_command: Vec<String>,
    pub default_project_command: Vec<String>,
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
        projects_configs_path.push(".projects");
        Self {
            projects_configs_path,
            version: "0.1.0".to_owned(),
            menu_command: vec!["rofi".to_owned(), "-dmenu".to_owned()],
            default_project_command: vec![
                "zellij".to_owned(),
                "-l".to_owned(),
                "$layout".to_owned(),
                "attach".to_owned(),
                "--create".to_owned(),
                "$name".to_owned(),
            ],
        }
    }
}
