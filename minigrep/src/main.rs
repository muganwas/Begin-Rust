use minigrep::{run, Config};
use std::{env, process};

fn main() {
    let config: Config = Config::new(env::args()).unwrap_or_else(|error| {
        eprintln!("{}", error);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        //we don't care about the value, just the error
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
