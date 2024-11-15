use std::env;
use std::process;

use clxrity_lilgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = clxrity_lilgrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}