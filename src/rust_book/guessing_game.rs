use rand::Rng;
use std::cmp::Ordering;
use std::io::{stdin, BufRead, BufReader};
fn read_number<R: BufRead>(reader: &mut R) -> Result<i32, String> {
    let mut guess = String::new();
    if reader.read_line(&mut guess).is_err() {
        return Err("Failed to read line".to_string());
    }

    match guess.trim().parse::<i32>() {
        //like exception check
        Ok(result) => Ok(result),
        Err(_) => Err(std::format!(
            "Input string is not a number: {}.",
            guess.trim()
        )),
    }
}
#[allow(dead_code)]
pub fn run() {
    println!("Hello, cargo!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!(
        "Try guess the number (the secret number is: {})!",
        secret_number
    );

    let mut reader = BufReader::new(stdin()); //create BufReader from standart input
    loop {
        println!("Please input your guess:");
        let guess = match read_number(&mut reader) {
            Ok(n) => n,
            Err(m) => {
                println!("{}", m);
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
