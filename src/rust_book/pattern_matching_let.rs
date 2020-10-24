#[allow(dead_code)]
pub fn run() {
    println!("Example of 'if let' syntax");
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8,_> = "34".parse();
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {},  as the background",color);
    }else if is_tuesday {
        println!("Tuesday is green day!");
    }else if let Ok(age) = age {
        if age>30 { //shadows here
            println!("Using purple as the background color");

        }else {
            println!("Using orange as the background color!");
        }
    } else {
        println!("Using blue as the background color");
    }
    //
    println!("Example of 'while let' syntax");
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop(){
        println!("{}",top);
    }
    //
    println!("Ignore unused Variable:");
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);


}

