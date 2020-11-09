#![deny(unsafe_code)]
#![no_std]
#![no_main]
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt;
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate f3; // provides memory.x
use cortex_m_rt::entry;

#[entry]
fn main()->! {

    let _y;
    let mut x = 42;
    _y = x;
    loop {
        x = x+1;
    }
}

