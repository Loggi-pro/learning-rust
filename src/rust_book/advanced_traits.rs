#[allow(dead_code)]
pub fn run() {
    println!("1.Operator overloading by trait:");
    let p1 = Point{x:1,y:0};
    let p2 = Point{x:2,y:3};
    let p3 = p1+p2;
    println!(" point {:?}+{:?}={:?}",Point{x:1,y:0},Point{x:2,y:3},p3);
    println!("1.2.Implement overloading for different types:");
    let mm = Millimeters(100);
    let m  = Meters(1);
    println!(" Add meter to mm={:?}",mm+m);
    //
    println!("2.Calling methods with same name: 'fly'");
    println!("We can specify what method to use: Human, Wizard or Pilot one");
    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);
    //
    println!("2.2.It is work because trait fly accept param self, but if not?");
    println!("This not returns what we want: A baby dog is called a {}", Dog::baby_name());
    println!("This returns what we want: A baby dog is called a {}", <Dog as Animal>::baby_name());
    //
    println!("3.Using supertraits(traits inside other traits:");
    println!("Test print value outlined by asterisk:");
    Point{x:1,y:1}.outline_print();
    //
    println!("4.Using newtype pattern to implement external traits on external types");
    let mut w = Wrapper(vec![String::from("hello"),String::from("world")]);
    w.insert(1, String::from("damned"));
    println!("w= {}",w);
}

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

//trait Add implemented like this
/*
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
*/

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
#[derive(Debug)]
struct Millimeters(u32);
#[derive(Debug)]
struct Meters(u32);
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}


//
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Magic Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}


//overloading
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}
//supertraits
use std::fmt;

trait OutlinePrint: fmt::Display { //this trait uses fmt::Display
    fn outline_print(&self) {
        let output = self.to_string(); //we can use to_string here, becouse any type with fmt::Display must implement it
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
impl OutlinePrint for Point {}
impl fmt::Display for Point{
    fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result {
        write!(f,"({},{})",self.x,self.y)
    }
}
//newtype pattern ()
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result {
        write!(f,"[{}]",self.0.join(", "))
    }
}
use std::ops::{Deref,DerefMut};
impl Deref for Wrapper {
    type Target = Vec<String>;
    fn deref(&self)->&Vec<String>{
        &self.0
    }
}
impl DerefMut for Wrapper {
    fn deref_mut(&mut self)->&mut Vec<String>{
        &mut self.0
    }
}