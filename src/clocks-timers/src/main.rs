#![no_std]
#![no_main]
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt;
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate hal; // provides memory.x
use cortex_m_rt::entry;
use hal::{prelude::*,pac};
use pac::{TIM6};
use hal::timer::{Timer};
use hal::gpio::gpioe::{PEx,Parts};
use hal::gpio::{Output, PushPull};
use core::cell::{RefCell};

//2 refcells here - one for changing option, second for mutable Timer
struct Wrap(RefCell<Option<RefCell<Timer<TIM6>>>>);
static G_TIM: Wrap = Wrap(RefCell::new(None));
unsafe impl Sync for Wrap{}
use hal::nb;
impl Wrap {
    fn get(&self)->& mut Timer<TIM6>{
        //all variables must to be refs or pointers;
        //we can do here get_mut(), but self is not mutable, so we need to workaround it by ptr;
        let  a =self.0.as_ptr(); //get pointer from refcell
        let b = unsafe{a.as_mut()}.unwrap(); //convert to reference
        let c = b.as_mut().unwrap(); //get reference from inside option
        let d = c.get_mut(); //get reference from inside refcell
        d
    }
}

pub fn init() -> Parts {
    //let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    // clock configuration using the default settings (all clocks run at 8 MHz)
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let  timer = Timer::tim6(dp.TIM6,1.khz(),clocks,&mut rcc.apb1);
    // Move the timer into our global storage
    *G_TIM.0.borrow_mut() = Some(RefCell::new(timer));
    dp.GPIOE.split(&mut rcc.ahb)
}



#[inline(never)]
fn delay(timer:&mut Timer<TIM6>, ms: u32) {
    timer.start((1000/ms).hz());
    nb::block!(timer.wait()).unwrap();
}

#[entry]
fn main()->! {
    let mut gpioe = init();
    
    let period = 500_u32;
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
        let mut t =G_TIM.get();
    loop {
        
        for curr in 0..arr.len() {
            let next  = (curr+1) % arr.len();
            arr[next].set_high().unwrap();
            delay(&mut t,period);
            arr[curr].set_low().unwrap();
            delay(&mut t,period);
        }
    }
}

