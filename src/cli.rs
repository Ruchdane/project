use clap::{command, Parser};
use clap_verbosity_flag::Verbosity;

#[derive(Parser, Debug)]
#[command(version = "1.0", author = "Ruchdane AMADOU")]
pub struct Opts {
    /// Sets the projects directory
    #[arg(short, long, value_name = "DIR")]
    pub projects_dir: Option<String>,

    /// Sets the Rofi theme file
    #[arg(short, long, value_name = "FILE")]
    pub rofi_theme: Option<String>,

    /// Sets verbosity level
    #[command(flatten)]
    pub verbose: Verbosity,
}
