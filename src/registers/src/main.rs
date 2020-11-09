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
    init();
    unsafe{
        //magic address;
        const GPIOE_BSRR:u32 = 0x4800_1018; 
        // Turn on the "North" LED (red)
        core::ptr::write_volatile(GPIOE_BSRR as *mut u32,1<<9);
        
            // Turn on the "East" LED (green)
            core::ptr::write_volatile(GPIOE_BSRR as *mut u32,1<<11);
            
            
            // Turn off the "North" LED
            core::ptr::write_volatile(GPIOE_BSRR as *mut u32,1<<(9+16));
            // Turn off the "East" LED
            core::ptr::write_volatile(GPIOE_BSRR as *mut u32,1<<(11+16));
            //if not use write_volatile, this writing will be optimized out to one
            //look in disasm for 'str' command
        }

        loop{}
}

