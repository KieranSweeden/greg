use std::{env, process};

use greg::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1); // more user-friendly method of exiting program (compared to panic)
    });

    if let Err(e) = greg::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
