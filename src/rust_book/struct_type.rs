struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn user_factory(email: String, username: String) -> User {
    User {
        email: email,
        username, //we can short it, if name is same
        active: true,
        sign_in_count: 1,
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("======Declare struct:======");
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("User email is '{}'", user1.email);
    //
    println!("======Using factory:======");
    let user2 = user_factory(
        String::from("factory@mail.com"),
        String::from("otherusername"),
    );
    println!(
        "Username of user constructed from factory '{}'",
        user2.username
    );
    //
    println!("======Struct update syntax:======");
    let user3 = User {
        email: String::from("newmail@mail.com"),
        username: String::from("newusername"),
        ..user1
    };
    println!(
        "Struct update user='{{active:{}, sign_in_count:{}}}'",
        user3.active, user3.sign_in_count
    );
    //Tuple struct
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    //
    println!("======Example of using struct:======");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of rectangle is {} square pixels", area(&rect1));
    //
    println!("======Example of derived trait for Debug print:======");
    println!("Rect1 is {:#?} (Debug trait)", rect1);
    //
    println!("======Example of method:======");
    println!(
        "The area of rectangle (calced by method) is {} square pixels",
        rect1.area()
    );
    //
    println!("======Example of method with parameter:======");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    //
    println!("======Example of associated(static) function:======");
    let square = Rectangle::square(10);
    println!("Square rect is {:?}", square);
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)] //trait Debug
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

//implement method of struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    //static (without self)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
