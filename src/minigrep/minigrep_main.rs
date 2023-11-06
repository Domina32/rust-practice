use crate::minigrep::minigrep_lib;
use std::env;
use std::process;

pub fn minigrep_main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep_lib::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep_lib::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
