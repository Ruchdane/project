use crate::{menu::MenuItem, prelude::*};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

// Struct to represent the project configuration data
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ProjectConfig {
    name: String,
    description: String,
    tags: Vec<String>,
    path: PathBuf,
    command: Option<Vec<String>>,
    env: HashMap<String, String>,
}

impl std::fmt::Display for ProjectConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {}", self.name, self.description)
    }
}
impl MenuItem for ProjectConfig {
    fn display_detail(&self) -> String {
        format!(
            "{} | {} | {}",
            self.name,
            self.description,
            self.tags.join(",")
        )
    }
}

impl ProjectConfig {
    pub fn get_projects_path(entries: fs::ReadDir) -> Vec<PathBuf> {
        entries
            .filter_map(|entry| {
                let path = entry.unwrap().path();
                if path.extension().unwrap_or_default() == "yml" {
                    Some(path)
                } else {
                    None
                }
            })
            .collect()
    }
    pub fn get_env(&self) -> &HashMap<String, String> {
        &self.env
    }
    pub fn get_command(&self) -> &Option<Vec<String>> {
        &self.command
    }

    // Function to parse the project configuration data from a configuration file
    pub fn from_file(config_file: &Path) -> Result<Self> {
        let config_content = std::fs::read_to_string(config_file)?;
        let config: ProjectConfig = serde_yaml::from_str(&config_content)?;
        debug!("Loaded {}", config.name);
        Ok(config)
    }

    pub(crate) fn get_workdir(&self) -> &PathBuf {
        &self.path
    }
}
