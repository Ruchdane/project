use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

// Struct to represent the project configuration data
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ProjectConfig {
    name: String,
    description: String,
    tags: Vec<String>,
}

impl ProjectConfig {
    // Function to parse the project configuration data from a configuration file
    pub fn from_file(config_file: &Path) -> Result<Self> {
        let desc_file = config_file.with_extension("yml.desc");
        let config_content = std::fs::read_to_string(desc_file)?;
        let config: ProjectConfig = serde_yaml::from_str(&config_content)?;
        debug!("Loaded {}", config.name);
        Ok(config)
    }

    pub(crate) fn display(&self) -> String {
        format!(
            "{} | {} | {}",
            self.name,
            self.description,
            self.tags.join(",")
        )
    }
}
