//extern crate minigrep;

use std::env;
use std::process;
use std::io::prelude::*;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
	let mut stderr = std::io::stderr();
	
	let config = Config::new(&args).unwrap_or_else(|error|
    {
        writeln!(
			&mut stderr,
			"Problem parsing arguments: {}",
			error
			).expect("Could not write to stderr");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        writeln!(
			&mut stderr,
			"Application error: {}", 
			e
			).expect("Could not write to stderr");
        process::exit(1);
    }
}
