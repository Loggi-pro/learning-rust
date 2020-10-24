#[allow(dead_code)]
pub fn run() {
    println!("Function parameters as patterns:");
    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
