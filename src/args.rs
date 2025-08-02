use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct WayIdleArgs {
    /// Config file to access instead of default
    #[arg(short, long)]
    config_file: Option<PathBuf>,
}

impl WayIdleArgs {
    pub fn config_file(&self) -> Option<PathBuf> {
        self.config_file.clone()
    }
}
