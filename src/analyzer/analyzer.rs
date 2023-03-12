use crate::{
    cli::config::Config,
    format::format::{format_file},
    // formatter::format::{format_file},
};

use eyre::Result;


#[derive(Clone)]
pub struct Analyzer {
    pub file : String
}

impl Analyzer {
    pub fn new(config: &Config) -> Self {
        Analyzer {
            file: config.file.clone()   
        }
       
    }
    pub fn scan(&mut self) -> Result<()> {
        format_file(&self.file)?;
        Ok(())
    }
}   