#![deny(unsafe_code)]
#![no_std]
#![no_main]
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_itm;
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate f3; // provides memory.x
#[allow(unused_imports)]
use cortex_m::{asm::bkpt,iprint,iprintln,peripheral::ITM};
use cortex_m_rt::entry;


#[entry]
fn main()->! {
    let mut itm = cortex_m::Peripherals::take().unwrap().ITM;
    iprintln!(&mut itm.stim[0],"Hello, world!");
    panic!("Something happened!");
    //loop {}
}

