#[allow(dead_code)]
pub fn run() {
    println!("For destructuring");
    let v = vec!['a','b','c'];
    for (index,value) in v.iter().enumerate(){
        println!("{} is at index {}",value,index);
    }
    //
    println!("Variable assignment destructuring:");
    let (x,y,z) = (1,2,3);
    println!("x:{},y:{},z:{}",x,y,z);

    //
    println!("Struct destructuring:");
    let p = Point{x:0,y:6};
    let Point {x:a,y:b} = p; //create two variables a and b
    println!("a={},b={}",a,b);
    //
    println!("Struct destructuring with same variable names:");
    let p = Point{x:0,y:6};
    let Point {x,y} = p; //create two variables a and b
    println!("x={},y={}",x,y);
    //
    println!("Struct destructuring with matching:");
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
    //
    println!("Enum destructuring (nested):");
    let _msg = Message::Quit{};
    let _msg = Message::Other{};
    let _msg = Message::Move{x:10,y:20};
    let _msg = Message::Write(String::from("text"));
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
    //
    println!("Destructuring Structs and Tuples:");
    let ((fet,inches),Point{x,y,}) = ((3,10),Point{x:3,y:-10});
    println!("fet={},inches={},x={},y={}",fet,inches,x,y);
    //
    println!("Ignore remaining parts of a value:");
    let p = Point{x:0,y:6};
    match p {
        Point{x,..} =>println!("x is {}",x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }


}

struct Point {
    x:i32,y:i32
}
enum Color {
    Hsv(i32, i32, i32),
}


enum Message {
    Quit,
    Other,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

