//on overflow happens panic
//integer
#[allow(dead_code)]
pub fn run() {
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup; //pattern matching
    println!("The value of y is:{}", y);
}
