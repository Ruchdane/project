#[macro_use]
extern crate log;

use std::fs;
use std::path::PathBuf;
mod command_parser;
mod config;
mod menu;
mod prelude;
use clap::Parser;
use cli::Opts;
use project::ProjectConfig;

use crate::command_parser::{command_from_array, parse_command};

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
    let projects_dir = opts
        .projects_dir
        .map(PathBuf::from)
        .unwrap_or_else(|| conf.projects_configs_path);
    debug!("Projects Locatin: {}", projects_dir.display());

    // Get list of project configuration files
    let projects_configs: Vec<ProjectConfig> = match fs::read_dir(&projects_dir) {
        Ok(entries) => ProjectConfig::get_projects_path(entries)
            .into_iter()
            .filter_map(|path| match ProjectConfig::from_file(&path) {
                Ok(value) => Some(value),
                Err(err) => {
                    error!(
                        "Failed to load project config at {}\n\t Because {:?}",
                        path.to_string_lossy(),
                        err
                    );
                    None
                }
            })
            .collect(),
        Err(err) => {
            error!("Failed to read project configuration files: {}", err);
            return;
        }
    };
    debug!("{} Projects Found", projects_configs.len());

    let menu_result = match menu::menu(command_from_array(conf.menu_command), &projects_configs) {
        Ok(v) => v,
        Err(err) => {
            error!("{:?}", err);
            None
        }
    };
    if let Some(project) = menu_result {
        let project = projects_configs
            .get(project)
            .expect("Project index out of bound");
        debug!("Selected {}", project);
        parse_command(
            project
                .get_command()
                .clone()
                .unwrap_or(conf.default_project_command),
            project.get_env(),
        )
        .current_dir(project.get_workdir())
        .status()
        .expect("Failed to launch project command");
    } else {
        debug!("No project selected!");
    }
}
