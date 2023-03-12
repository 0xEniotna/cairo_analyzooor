use eyre::{eyre, Result};
use clap::Parser;

mod cli;
mod format;
use cli::args::Args;
use cli::config::Config;

mod analyzer;
use analyzer::analyzer::Analyzer;


use std::thread;
use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};


fn main() -> Result<()> {
    let cli_agrs = Args::parse();
    
    // create config file
    let config = match cli_agrs.config {
        // config file provided
        Some(config_file) => Config::load_config(&config_file),
        None => {
            if cli_agrs.file.is_empty() {
                return Err(eyre!("Analyzooor needs a file path using --file"));
                // process::exit(1);
            }

            Config {
                file: cli_agrs.file,
            }
        }
    };
    
    // create the analyzoooor
    let mut analyzer = Analyzer::new(&config);
    
    
    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");

    let pb = m.add(ProgressBar::new(100));
    pb.set_style(sty.clone());
    
    m.println("starting!").unwrap();

    let m_clone = m.clone();
    let h1 = thread::spawn(move || {
        m_clone.println("Format check...").unwrap();
        for i in 0..100 {
            pb.set_message(format!("{}", i + 1));
            pb.inc(1);
            thread::sleep(Duration::from_millis(10));
        }
        
        let res = analyzer.scan();
        
        m_clone.println("Format is done!").unwrap();
        pb.finish_with_message("done");
    });

    let _ = h1.join();
    m.clear().unwrap();
    
    Ok(())
}
