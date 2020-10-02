use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
#[allow(dead_code)]
pub fn run() {
    //try open file
    let f = File::open("hello.txt");
    match f {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    //same but with lambdas
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    // using expect
    let _f = File::open("hello.txt").expect("Failed to open hello.txt");
    //
    println!("Error propagation example (long version)");
    let s = read_username_from_file();
    println!("Readed from file: {:?}", s);
    //
    println!("Error propagation example (short version)");
    let s = read_username_from_file_short();
    println!("Readed from file: {:?}", s);
    //call_panic();
}

#[allow(dead_code)]
fn call_panic() {
    panic!("crash and burn");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
