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
    let half_period = 500_u16;
    loop {
        leds[0].on();
        delay.delay_ms(half_period);
        leds[0].off();
        delay.delay_ms(half_period);
    }
}

