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
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    //Using factory
    println!("User email is '{}'", user1.email);
    let user2 = user_factory(
        String::from("factory@mail.com"),
        String::from("otherusername"),
    );
    println!(
        "Username of user constructed from factory '{}'",
        user2.username
    );
    //Struct update syntax
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
    //Example of using struct
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of rectangle is {} square pixels", area(&rect1));
    //Example of derived trait for Debug print
    println!("Rect1 is {:#?} (Debug trait)", rect1);
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
