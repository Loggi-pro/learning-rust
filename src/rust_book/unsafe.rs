
#[allow(dead_code)]
pub fn run() {
    println!("1. Dereference a raw pointer:");
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 is: {}",*r1);
        println!("r2 is: {}",*r2);
    }
    let address = 0x012345usize;
    let r = address as *const i32;
    println!("  we can make pointer to address={:?}",r);
}