use std::{env, process};

use greg::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1); // more user-friendly method of exiting program (compared to panic)
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = greg::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
