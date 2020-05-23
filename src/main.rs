#[derive(Debug)] //for debug print
struct Position {
    x: i8,
    y: i8,
}
impl Position {
    fn update_y(&mut self) {
        self.y -= 10;
    }

    fn update_xy(&mut self) {
        self.x += 1;
        self.update_y();
    }
}

fn main() {
    let mut pos = Position { x: 1, y: 2 };
    pos.update_xy(); //call method
    Position::update_xy(&mut pos); //or this one: universal function call

    println!("My position is: {{{:?}}}", pos); //or :#? for more verbose output
}
