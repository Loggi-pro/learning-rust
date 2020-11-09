#![deny(unsafe_code)]
#![no_std]
#![no_main]
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt;
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate f3; // provides memory.x
use cortex_m_rt::entry;
use aux5::{prelude::*,Delay,Leds};

#[entry]
fn main()->! {
    let (mut delay,mut leds): (Delay, Leds) = aux5::init();
    let period = 50_u16;
    loop {
        for curr in 0..leds.len() {
            let next  = (curr+1) % leds.len();
            leds[next].on();
            delay.delay_ms(period);
            leds[curr].off();
            delay.delay_ms(period);
            
        }
    }
}

