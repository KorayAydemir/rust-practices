use killgrep;

use std::collections::HashMap;
use std::{env, process};
pub fn main() {
    let args: Vec<String> = env::args().collect();
    let config = killgrep::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    //killgrep::run(config).unwrap_or_else(|err| println!("{err}"));
    if let Err(e) = killgrep::run(config) {
        eprintln!("Application error: {e}");
    }
}
