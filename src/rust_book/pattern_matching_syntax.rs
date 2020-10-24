#[allow(dead_code)]
pub fn run() {
    println!("Match literals:");
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"), //all other variables
    }
    //
    println!("Match named variables:");
    let x = Some(1);
    let y = 10;
    match x {
        Some(1) => println!("Got 1"),
        Some(2) => println!("Got 2"),
        Some(3) => println!("Got 3"),
        Some(y)=>println!("Matched y={}",y),
        _=> println!("Default case, x = {:?}",x),
    }
    println!("at the end: x={:?},y={}",x,y);
    //
    println!("Match multiple patterns:");
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    //
    println!("Match reanges of values:");
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
    //
    println!("Ignore values in patterns:");
    foo(3,4);
    //
    println!("Ignore parts of a value with a nested _:");
    //user should not be allowed to overwrite an existing customization of a setting 
    //but can unset the setting and give it a value if it is currently unset.
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match(setting_value, new_setting_value){
        (Some(_),Some(_))=>{
            println!("Can't overwrtie an axisting customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("Setting is {:?}", setting_value);
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
    //
    println!("Match guard:");
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);
    //
    println!("Match guard with multiple patterns:");
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
    //
    println!("Bindings");
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {id: id_variable @ 2..=7} => { //create variable name for range
            println!("Found an id in range: {}",id_variable)
        }
        Message::Hello{id:10..=12}=>{
            println!("Found and if in another range");
        }
        Message::Hello{id}=>println!("Found some other id: {}",id),
    }
}

enum Message {
    Hello { id: i32 },
}





fn foo(_:i32,y:i32){
    println!("This code only uses the y parameter: {}",y);
}
