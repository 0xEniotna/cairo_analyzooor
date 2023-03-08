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
    
    // pub fn scan(&mut self) {
    //     let config = FormatterConfig::default();
    //     let stdin_fmt = StdinFmt::new(config);
    //     let outcome = stdin_fmt.format();
    //     match outcome {
    //         FormatOutcome::Success(formatted) => println!("{}", formatted),
    //         FormatOutcome::Error(error) => eprintln!("{}", error),
    //     }
    // }
}   