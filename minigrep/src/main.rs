use minigrep::{run, Config};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap_or_else(|error| {
        println!("{}", error);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        //we don't care about the value, just the error
        println!("Application error: {}", e);
        process::exit(1);
    };
}
