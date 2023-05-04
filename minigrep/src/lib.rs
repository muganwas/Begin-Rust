use std::{error::Error, fs};

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            let error = "Usage: minigrep <query> <filename>";
            return Err(error);
        }
        Ok(Self {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    println!("Searching for: {}", config.query);
    println!("With text:\n{}", content);
    Ok(())
}
