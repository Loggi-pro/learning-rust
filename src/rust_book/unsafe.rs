
#[allow(dead_code)]
pub fn run() {
    println!("1. Dereference a raw pointer:");
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("      r1 is: {}",*r1);
        println!("      r2 is: {}",*r2);
    }
    let address = 0x012345usize;
    let r = address as *const i32;
    println!("  we can make pointer to address={:?}",r);
    //
    println!("2. Call an unsafe function or method:");
    unsafe {
        dangerous();
    }
    println!("  Creating a Safe Abstraction over Unsafe Code: split slice");
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (a, b) = split_at_mut(v.as_mut(),3);
    println!("      First slice {:?}, Second slice {:?}",a,b);
    //assert_eq!(a, &mut [1, 2, 3]);
    //assert_eq!(b, &mut [4, 5, 6]);
    println!("  Using extern Functions to Call External Code: ");
    println!("  use extern keyword on function declaration.");
    /*unsafe{
        println!("Absolute value of -3 according to C: {}",abs(-3));
    }*/
}

unsafe fn dangerous(){}

use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr(); //get raw pointer of slice

    assert!(mid <= len);
    //it is unsafe to get two mutables from one source. 
    //but it is ok to get it from two DIFFERENT part of one source
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

//extern "C"{
   // fn abs(input:i32)->i32;
//}