#[allow(dead_code)]
pub fn run() {
    println!("Newtype pattern (good wrapper, incapsulation):");
    let m = Meters{0:10};
    println!("Newtype: {:?}",m);
    //
    println!("Type alias:");
    let km:Kilometers = 5;
    println!("Type kilometers treated the same as value type i32 (like typedef):{:?}",km);
    //
    println!("Never type,that never returns: fn bar() -> ! {{}}" );
    println!("used as return value of 'continue','loop','panic!' etc" );
    //
    println!("Dynamically sized types and sized traits:" );
    generic_known_size(5);
    generic_unknown_size(&String::from("string"));
}
#[derive(Debug)]
struct Meters(u32);
//
type Kilometers = i32;

//this function will work only on types that have a known size at compile time
fn generic_known_size<T: Sized+std::fmt::Debug>(t: T) {
    println!("dyn print:{:?}",t);
}
//its work for all types
fn generic_unknown_size<T: ?Sized+std::fmt::Debug>(t: &T) {
    println!("dyn print:{:?}",t);
}