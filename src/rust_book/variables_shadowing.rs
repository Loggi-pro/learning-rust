#[allow(dead_code)]
pub fn run() {
    let spaces = "    "; //not mutable
    let spaces = spaces.len(); //shadow here (can be other type)
    println!("The count of spaces is: {}", spaces);
}
