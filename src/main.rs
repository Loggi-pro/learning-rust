mod test; //add test.rs
          //also add #[path = "foo.rs"] before to import file at path
          //also can import a directory

pub fn main() {
    let _x = test::other_function();
}
