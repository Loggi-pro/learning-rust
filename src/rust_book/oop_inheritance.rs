pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, //dyn for dynamic dispatch (because compiler dont know exact types)
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: i32,
    pub height: i32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) {
        //code to actually draw a button
        println!("Draw Button: {:?}", self);
    }
}

#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("Draw SelectBox: {:?}", self);
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Inheritance in rust implemented by traits");
    println!("Make vector of heterogenous collection");
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();
}
