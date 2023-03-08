use cairo_lang_formatter::{CairoFormatter, FormatOutcome, FormatterConfig, StdinFmt};
use cli::config::Config;

#[derive(Clone)]
pub struct Analyzer {
    config: Config,
}

impl Analyzer {
    pub fn new(config: &Config) -> Self {
        let mut analyzer = Analyzer {
            config: config
        };
        analyzer
    }
    
    pub fn scan(&mut self) {
        let config = FormatterConfig::default();
        let stdin_fmt = StdinFmt::new(config);
        let outcome = stdin_fmt.format();
        match outcome {
            FormatOutcome::Success(formatted) => println!("{}", formatted),
            FormatOutcome::Error(error) => eprintln!("{}", error),
        }
    }
}   