use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Cli {
    /// Folder to download
    #[arg(short, long)]
    pub file: PathBuf,

    /// Extra debug information
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,

    /// Install dependencies
    #[arg(long, default_value_t = false)]
    pub install: bool,

    /// Setup configs
    #[arg(short, long, default_value_t = false)]
    pub config: bool,

    /// Setup shortcuts
    #[arg(long, default_value_t = false)]
    pub shortcuts: bool,
}

