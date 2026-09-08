//! Used to identify a string `query` from a .txt file.
//!
//! contains the public `Config` struct for getting the `env::args()`
//! from the command line arguments.
//!
use std::{env, process};

use grep_lite::{Config, run};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
