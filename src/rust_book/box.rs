enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::rust_book::r#box::List::{Cons, Nil};
//box - is a smart pointer

#[allow(dead_code)]
pub fn run() {
    let b = Box::new(5);
    println!("b = {}", b);
    //
    let x = 5;
    let y = Box::new(x);
    println!("Value {} is same as  {}", x, *y);
    //
    println!("Create recurcisve type cons list:");
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    //
    println!("Test own smart pointer type:");
    let x = 5;
    let y = MyStackBox::new(x);
    println!("Value {} is same as  {}", x, *y);
    //
    println!("Deref coercion:Rust calls as many times derefs as necessary");

    //rust convert &Box<String> to &String by deref operator
    //rust then again call deref on &String and convert it to &str
    let m = MyStackBox::new(String::from("Rust"));
    hello(&m);
}

struct MyStackBox<T>(T);
impl<T> MyStackBox<T> {
    fn new(x: T) -> MyStackBox<T> {
        MyStackBox(x)
    }
}
use std::ops::Deref;
//need to implement deref trait
impl<T> Deref for MyStackBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
