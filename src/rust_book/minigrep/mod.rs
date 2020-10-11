mod minigrep;
use std::env;
use std::process;
//this is must be main file


//this must be main function

//to check you can run: cargo run frog poem.txt
//or cargo run body poem.txt
//for case insensitive search in file: $Env:CASE_INSENSITIVE=1; cargo run to poem.txt
//call after Remove-Item Env:CASE_INSENSITIVE to remove env var
#[allow(dead_code)]
pub fn run() {
    //parse argument list
    let args: Vec<String> = env::args().collect();
    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

