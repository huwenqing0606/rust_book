//extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
	
	let config = Config::new(&args).unwrap_or_else(|error|
    {
        println!("Problem parsing results: {}", error);
        process::exit(1);
    });
	
	println!("Searching is for '{}'", config.query);
	println!("In file '{}'", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
