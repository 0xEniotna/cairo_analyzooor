use indicatif::ProgressBar;
use eyre::{eyre, Result};
use clap::Parser;

mod cli;

use cli::args::Args;
use cli::config::Config;

// use analyzer::analyzer::Analyzer;

fn main() -> Result<()> {
    let cli_agrs = Args::parse();
    
    // create config file
    let config = match cli_agrs.config {
    // config file provided
    Some(config_file) => Config::load_config(&config_file),
    None => {
        if cli_agrs.file.len() == 0 {
            // error!("Fuzzer needs a contract path using --contract");
            eyre!("Analyzooor needs a file path using --file");
            // process::exit(1);
        }

        Config {
            file: cli_agrs.file,
        }
    }
    };
    
    // // create the analyzoooor
    // let mut analyzer = Analyzer::new(&config);

    // analyzer.scan();
    
    Ok(())
}