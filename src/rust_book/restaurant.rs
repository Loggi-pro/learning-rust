mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
use front_of_house::hosting; //idiomatic way for functions
use front_of_house::hosting::add_to_waitlist; //dont do like this
use std::collections::HashMap; //idiomatic way for struct\enums
use std::io::Result as IoResult; //use as

//exporting
//pub use front_of_house::hosting; //we allow to caller use this
#[allow(dead_code)]
pub fn run() {
    //absolute path
    crate::rust_book::restaurant::front_of_house::hosting::add_to_waitlist();
    //relative path
    front_of_house::hosting::add_to_waitlist();
    //
    hosting::add_to_waitlist(); //do like this
    add_to_waitlist(); //dont do like this
                       //
    let mut map = HashMap::new(); //do like this
    map.insert(1, 2);
    let _a = some_func();
    //
}

fn some_func() -> IoResult<bool> {
    Ok(true)
}
