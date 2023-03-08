use clap::{self, Parser};

/// Static analyzer for Cairo1
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// File to analyze
    #[arg(
    long,
    help = "Set the path of the contract to analyze",
    name = "file",
    default_value = ""
    )]
    pub file: String,
    /// Load config file
    #[arg(long, help = "Load config file", name = "CONFIG")]
    pub config: Option<String>,
}