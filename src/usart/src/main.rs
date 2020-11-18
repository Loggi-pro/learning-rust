//#![deny(unsafe_code)]
#![no_std]
#![no_main]
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt;
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate hal; // provides memory.x
use cortex_m_rt::entry;
use hal::{prelude::*,pac,serial::Serial,pac::usart1,time::MonoTimer};
use pac::{USART1};
use cortex_m::{iprintln,peripheral::ITM};
use heapless::{consts, Vec};
use core::result::Result;

macro_rules! uprint {
    ($serial:expr,$($arg:tt)*) => {
        $serial.write_fmt(format_args!($($arg)*)).ok()
    };
}
macro_rules! uprintln {
    (&serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\n"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\n"), $($arg)*)
    };
}

use core::fmt::{self,Write};
struct SerialPort {
    usart1: &'static mut usart1::RegisterBlock,
}

impl SerialPort{
    
    fn write_byte(&mut self,byte:u8)->Result<(),u8>{
        while self.usart1.isr.read().txe().bit_is_clear() {continue;}
        unsafe {
            self.usart1.tdr.write(|w| w.tdr().bits(u16::from(byte)));
         } 
         Ok(())
    }
    fn read_byte(&mut self)->Result<u8,()>{
        while self.usart1.isr.read().rxne().bit_is_clear() {continue;}
        let byte = self.usart1.rdr.read().rdr().bits() as u8;
        Ok(byte)
    }
}

impl Write for SerialPort {
    fn write_str(&mut self,s:&str)->fmt::Result {
        //implement write string
        for c in s.as_bytes().iter() {
            self.write_byte(*c).unwrap();
        }
        Ok(())
    }
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.write_byte(c as u8).unwrap();
        Ok(())
    }
}

pub fn init() -> (&'static mut usart1::RegisterBlock, MonoTimer, ITM) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let (tx, rx) = match () {
        //#[cfg(feature = "adapter")]
        () => {
            let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

            let tx = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
            let rx = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);

            (tx, rx)
        }
        /*#[cfg(not(feature = "adapter"))]
        () => {
            let mut gpioc = dp.GPIOC.split(&mut rcc.ahb);

            let tx = gpioc.pc4.into_af7(&mut gpioc.moder, &mut gpioc.afrl);
            let rx = gpioc.pc5.into_af7(&mut gpioc.moder, &mut gpioc.afrl);

            (tx, rx)
        }*/
    };

    Serial::usart1(dp.USART1, (tx, rx), 115_200.bps(), clocks, &mut rcc.apb2);
    // If you are having trouble sending/receiving data to/from the
    // HC-05 bluetooth module, try this configuration instead:
    // Serial::usart1(dp.USART1, (tx, rx), 9600.bps(), clocks, &mut rcc.apb2);

    unsafe {
        (
            &mut *(USART1::ptr() as *mut _),
            MonoTimer::new(cp.DWT, clocks),
            cp.ITM,
        )
    }
}



#[entry]
fn main()->! {
    let (usart1,mono_timer,mut itm) = init();

    let s = b"The quick brown fox jumps over the lazy dog";
    let instant = mono_timer.now();
    for c in s.iter() {
        while usart1.isr.read().txe().bit_is_clear() {continue;}
        unsafe {
            usart1.tdr.write(|w| w.tdr().bits(u16::from(*c)));
            }   
    }
    let elapsed = instant.elapsed();
    iprintln!(&mut itm.stim[0],"`for` loop took {} ticks({} us)",elapsed, elapsed as f32 / mono_timer.frequency().0 as f32*1e6);
    //second variant
    let mut serial = SerialPort{usart1};
    uprintln!(serial,"The answer is {}",40+2);
    //make buffer (250 bytes of capacity)
    let mut buffer: Vec<u8,consts::U250> = Vec::new(); 
    loop {
        buffer.clear();
        loop {
            let mut _byte = serial.read_byte().unwrap();
            if buffer.push(_byte).is_err() {
                //buffer full
                serial.write_str("error: buffer full\n\r").unwrap();
                break;
            }
        
            if _byte==13 {
                for b in buffer.iter().rev().chain(&[b'\n',b'\r']) {
                    serial.write_char(*b as char).unwrap();
                }
                break;
            }
        }

    }
}


