#[allow(dead_code)]
pub fn run() {
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("Was three (match)"),
        _ => (),
    }
    //other way for only one match case
    //better syntax
    if let Some(3) = some_u8_value {
        println!("Was three (if let)");
    }
    //with else
    if let Some(4) = some_u8_value {
        println!("four");
    } else {
        println!("Was not four(if let else)")
    }
}
