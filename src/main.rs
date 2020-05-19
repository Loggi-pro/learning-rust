#[derive(Debug)] //for debug print
struct Position {
    x: i8,
    y: i8,
}

fn main() {
    let pos = Position { x: 1, y: 2 };
    println!("My position is: {{{:?}}}", pos); //or :#? for more verbose output
}
