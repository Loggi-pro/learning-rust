mod minigrep;
use std::env;
use std::process;

//this must be main function
#[allow(dead_code)]
pub fn run() {
    //parse argument list
    let args: Vec<String> = env::args().collect();
    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
    }
}
