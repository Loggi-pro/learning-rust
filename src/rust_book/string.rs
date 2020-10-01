#[allow(dead_code)]
pub fn run() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s = s1 + &s2;
    println!("Concated strings:{}", s);
    //
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s = format!("Formatted strings:{}{}", s1, s2);
    println!("{}", s);
    println!("Iterating string: {}", "नमस्ते");
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    println!("Iterating raw bytes: {}", "नमस्ते");
    for c in "नमस्ते".bytes() {
        println!("{}", c);
    }
}
