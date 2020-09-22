#[allow(dead_code)]
pub fn run() {
    another_function(five());
    let _x = 5;
    //expression
    let y = {
        let x = 3;
        x + 1 //dont need semicolon here
    };

    println!("The value of y is: {}", y);
}

fn another_function(x: i32) {
    println!("Another function x is {}", x);
}

fn five() -> i32 {
    5 //it is an expression
}
