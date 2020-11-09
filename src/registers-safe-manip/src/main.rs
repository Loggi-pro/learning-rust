#![no_std]
#![no_main]
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_itm;
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate f3; // provides memory.x
#[allow(unused_imports)]
use cortex_m::{asm::bkpt,iprint,iprintln,peripheral::ITM};
use cortex_m_rt::entry;
use f3::{
    hal:: {
        prelude::*,
        stm32f30x::gpioc,
        stm32f30x::{self,GPIOE},
    },
    led::Leds
};

#[inline(never)]
pub fn init()->(ITM,&'static gpioc::RegisterBlock){
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    Leds::new(dp.GPIOE.split(&mut rcc.ahb));
    (cp.ITM,unsafe{&*GPIOE::ptr()})
}

#[entry]
fn main()->! {
    let gpio = init().1;
    gpio.bsrr.write(|w| w.bs9().set_bit());
    gpio.bsrr.write(|w| w.bs11().set_bit());
    gpio.bsrr.write(|w| w.br9().set_bit());
    gpio.bsrr.write(|w| w.br11().set_bit());

    loop{}
}

