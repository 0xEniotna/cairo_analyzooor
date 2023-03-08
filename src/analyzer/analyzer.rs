use crate::{cli::config::Config};

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
    
    pub fn scan(&mut self) {
        todo!()
    }
}   