#![no_std]
#![no_main]
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt;
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate hal; // provides memory.x
use cortex_m_rt::entry;
use hal::{delay::Delay,prelude::*,pac};


use hal::gpio::gpioe::{PEx,Parts};
use hal::gpio::{Output, PushPull};

pub fn init() -> (Parts/*, &'static rcc::RegisterBlock,*/, Delay) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    // clock configuration using the default settings (all clocks run at 8 MHz)
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let delay = Delay::new(cp.SYST, clocks);
    (dp.GPIOE.split(&mut rcc.ahb)/*(&*GPIOE::ptr(), &*RCC::ptr()*/,delay)
}


#[entry]
fn main()->! {
    let (mut gpioe,mut delay) = init();
    let period = 50_u16;
    let mut arr:[PEx<Output<PushPull>>;8] = [
        gpioe.pe8.into_push_pull_output(&mut  gpioe.moder, &mut gpioe.otyper).downgrade(),
        gpioe.pe9.into_push_pull_output(&mut  gpioe.moder, &mut gpioe.otyper).downgrade(),
        gpioe.pe10.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade(),
        gpioe.pe11.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade(),
        gpioe.pe12.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade(),
        gpioe.pe13.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade(),
        gpioe.pe14.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade(),
        gpioe.pe15.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade()
        ];
    loop {
        for curr in 0..arr.len() {
            let next  = (curr+1) % arr.len();
            arr[next].set_high().unwrap();
            delay.delay_ms(period);
            arr[curr].set_low().unwrap();
            delay.delay_ms(period);
        }
    }
}

