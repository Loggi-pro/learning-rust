#[allow(dead_code)]
pub fn run() {
    println!("Pass function as argument:");
    let answer = do_twice(add_one,5);
    println!("The answer is (regular function): {}",answer);
    //
    let answer = do_twice(|x| x+1,5);
    println!("The answer is (closure): {}",answer);
    //
    println!("Returning closures:");
    println!("The answer is (returned closure): {}",returns_closures()(answer));

}

fn add_one(x:i32)->i32{
    x+1
}

fn do_twice(f:fn(i32)->i32,arg:i32)->i32 {
    f(arg)+f(arg)
}

//
fn returns_closures()->impl Fn(i32)->i32 { //Box here because compiler doesn't know size of return type
    |x|x+1
}