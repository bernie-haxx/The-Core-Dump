use std::{env, process};

use grep_lite::{Config, run};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Problem passing arguments: {}", e);
        process::exit(1);
    }
}
