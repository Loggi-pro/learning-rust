#[allow(dead_code)]
pub fn run() {
    println!("The result is {}", breaked_loop_expression());
    println!("The result is {}", breaked_loop_statement());
    conditional_loop();
    foreach_loop();
    reverse_example();
}
#[allow(dead_code)]
fn endless_loop() {
    loop {
        println!("again!");
    }
}

fn breaked_loop_expression() -> i32 {
    let mut counter = 0;
    loop {
        //same as let result = loop ... ; return result
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    } //use ; here to convert this expression to statement
}

fn breaked_loop_statement() -> i32 {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    return result;
}

fn conditional_loop() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn foreach_loop() {
    let a = [10, 20, 30, 40, 50];

    for el in a.iter() {
        println!("the value is: {}", el);
    }
}

fn reverse_example() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
