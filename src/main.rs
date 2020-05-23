#[derive(Debug)] //for debug print
struct Position {
    x: i8,
    y: i8,
}

fn update_y(pos: &mut Position) {
    pos.y -= 10;
}

fn update_xy(pos: &mut Position) {
    pos.x += 1;
    update_y(pos); //here pos== &mut Position
}

fn main() {
    let mut pos = Position { x: 1, y: 2 };
    update_xy(&mut pos);

    println!("My position is: {{{:?}}}", pos); //or :#? for more verbose output
}
