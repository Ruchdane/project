#[macro_use]
extern crate log;

use std::path::PathBuf;
use std::{fs, process::Command};
mod config;
mod menu;
mod prelude;
use clap::Parser;
use cli::Opts;
use project::ProjectConfig;

mod cli;
mod project;
fn main() {
    // Path to your project configuration files
    let opts: Opts = Opts::parse();
    let conf = config::Config::get();
    pretty_env_logger::formatted_builder()
        .filter_level(opts.verbose.log_level_filter())
        .init();
    debug!("Using config {:?}", conf);

    // Path to your project configuration files
    let mut projects_dir = opts
        .projects_dir
        .map(PathBuf::from)
        .unwrap_or_else(|| conf.projects_configs_path);
    debug!("Projects Locatin: {}", projects_dir.display());

    // Get list of project configuration files
    let projects_configs = match fs::read_dir(&projects_dir) {
        Ok(entries) => entries
            .filter_map(|entry| {
                let path = entry.unwrap().path();
                if path.extension().unwrap_or_default() == "yml" {
                    Some(path)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>(),
        Err(err) => {
            error!("Failed to read project configuration files: {}", err);
            return;
        }
    };

    debug!("{} Project loaded successuflly", projects_configs.len());
    // Prepare project configurations
    let config_files: &[PathBuf] = &projects_configs;
    let mut project_configs = Vec::new();

    for config_file in config_files {
        match ProjectConfig::from_file(config_file) {
            Ok(project_config) => project_configs.push((
                config_file.file_name().unwrap().to_string_lossy(),
                project_config,
            )),
            Err(e) => error!(
                "Could not load config for {} \n\tBecause: {:?}",
                config_file.display(),
                e
            ),
        };
    }

    // Prepare menu items
    let menu_items: Vec<String> = project_configs
        .iter()
        .map(|config| format!("{}:{} ", config.0, config.1.display()))
        .collect();

    let binding = menu::menu(menu_items, opts.rofi_theme.as_deref());
    if binding.is_empty() {
        debug!("No project selected");
        return;
    }
    let selected = binding.split_once(':').unwrap().0;

    debug!("Selected {}", selected);
    projects_dir.push(selected);
    // Launch the selected configuration file with tmuxp
    if !selected.is_empty() {
        Command::new("tmuxp")
            .arg("load")
            .arg(projects_dir)
            .status()
            .expect("Failed to launch tmuxp");
    }
}
