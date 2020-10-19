struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("  Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

use std::ops::Deref;
//need to implement deref trait
impl Deref for CustomSmartPointer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.data
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("Auto drop:");
    {
        let _c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let _d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("  CustomSmartPointers created.");
    } //all destroyed here
    println!("  All pointers dropped");
    //
    println!("Example of manual drop:");
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("  CustomSmartPointer with data {} created.", *c);
    drop(c);
    println!("  CustomSmartPointer dropped before end of main.");
}
