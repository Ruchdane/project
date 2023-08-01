use std::{env, path::PathBuf};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    version: String,
    pub projects_configs_path: PathBuf,
    pub menu_command: Vec<String>,
    pub default_project_command: Vec<String>,
}

impl Config {
    pub fn load() -> Self {
        let Some(proj_dirs) = ProjectDirs::from("dev","ruchdane","project") else { 
            error!("No Project dir: using default config");
            return Config::default()
        };
        let mut config_path = PathBuf::from(proj_dirs.config_dir());
        if !config_path.exists() {
            if let Err(err) = std::fs::create_dir_all(&config_path) {
                error!(
                    "Failed to create config directory {}: {}",
                    config_path.to_string_lossy(),
                    err
                );
                return Config::default();
            }
        }
        config_path.push("config.yaml");
        debug!(
            "Loading config from {}",
            config_path.clone().to_string_lossy()
        );
        let config_string = match std::fs::read_to_string(config_path.clone()) {
            Ok(config_string) => config_string,
            Err(err) => {
                let config = Config::default();
                if err.kind() == std::io::ErrorKind::NotFound {
                    let config_string =
                        serde_yaml::to_string(&config).expect("could not stringify default config");
                    std::fs::write(config_path.clone(), config_string).unwrap_or_else(|err| {
                        error!(
                            "couldn't write config file to {} because:\n\t{:?}",
                            config_path.to_string_lossy(),
                            err
                        )
                    });
                }
                error!("Error reading config: {:#?}", err);
                error!("Using default config");
                return config;
            }
        };
        match serde_yaml::from_str(config_string.as_str()) {
            Ok(config) => config,
            Err(err) => {
                debug!("Error reading config: {:#?}", err);
                debug!("Using default config");
                Config::default()
            }
        }
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
