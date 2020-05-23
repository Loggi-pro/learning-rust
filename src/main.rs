#[derive(Debug)] //for debug print
struct Position {
    x: i8,
    y: i8,
}

fn main() {
    let mut pos = Position { x: 1, y: 2 };
    let pos2 = &mut pos;
    pos2.x = 12;

    println!("My position is: {{{:?}}}", pos); //or :#? for more verbose output
}
