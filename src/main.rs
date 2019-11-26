#![no_std]
#![no_main]
//#[macro_use]
// Source: https://github.com/stm32-rs/stm32f4xx-hal/blob/9dab1701bc68efe3a1df8eb3b93c866d7ef1fa0e/src/lib.rs
//#[cfg(not(feature = "device-selected"))]
// compile_error!("This crate requires one of the following device features enabled:
//         stm32f401
//         stm32f405
//         stm32f407
//         stm32f410
//         stm32f411
//         stm32f412
//         stm32f413
//         stm32f415
//         stm32f417
//         stm32f423
//         stm32f427
//         stm32f429
//         stm32f437
//         stm32f439
//         stm32f446
//         stm32f469
//         stm32f479");
//#![deny(unsafe_code)]
extern crate panic_halt;

use core::mem::size_of;
pub use cortex_m::peripheral::syst;
use cortex_m::peripheral::Peripherals as CorePeripherals;
use cortex_m_rt::entry;
//extern crate sam3x8e;

//#[macro_use]
//extern crate static_assertions;

pub use sam3x8e as target;

mod pin;
mod serial;
mod time;
use crate::pin::{Configuration, Writer};
use crate::time::Delay;





// Help: https://rust-embedded.github.io/book/start/registers.html
#[entry]
fn main() -> ! {
    if let (Some(cp), Some(dp)) = (CorePeripherals::take(), target::Peripherals::take()) {
        let mut on = false;
        let pmc = dp.PMC;
        let watchdog = dp.WDT;

        // Do something.
        let mut t = time::Time::syst(cp.SYST);

        //pmc.pmc_pcer0
        //    .write_with_zero(|w| unsafe { w.bits(0x3F << 11) });


        // Disable the watchdog.
        //watchdog.mr.write(|w| w.wddis().set_bit());

        // Init the clock.
        pmc.pmc_mckr.write_with_zero(|w| unsafe {w.bits(0x01)});


        let p1 = pin::create(&dp.PIOB, 1 << 27);
        let p2 = pin::create(&dp.PIOA, 1 << 27);

        // IsDisabled, Unknown, IsValid
        let ser = serial::Serial::new(dp.UART, &pmc, 115200, p1, p2);

        ser.begin(&dp.PIOA, &pmc);

        // pmc.pmc_pcer0
        //     .write_with_zero(|w| w.
        //     pid11().set_bit().
        //     pid12().set_bit()); // Enable PIOA, PIOB

        let pin13 = pin::create(&dp.PIOB, 1 << 27);
        let pin13_output = pin13.as_output();



        //t.delay_ms(4_000_000);∂∂

        //while !t.has_wrapped() {};
        
        //unsafe { ser.write_str_blocking(b"Turning LED on...".as_ptr()) };
        //ser.write();



        

        t.delay_ms(2_000_000);

        loop {
            match ser.read_byte() {
                Ok(x) => {},
                _ => {}
            }
            if t.has_wrapped() {
                // Turn on the LED!
                if on {
                    unsafe { ser.write_str_blocking(b"Turning LED on...\n".as_ptr()) };
                    pin13_output.set_high();
                    
                //dp.PIOB.codr.write_with_zero(|w| w.p27().set_bit() );
                } else {
                    unsafe { ser.write_str_blocking(b"Turning LED off...\n".as_ptr()) };
                    pin13_output.set_low();

                    //pin13_inp.get_state(&dp.PIOB);
                    //dp.PIOB.sodr.write_with_zero(|w| w.p27().set_bit() );
                }
                on = !on;
                t.delay_ms(2_000_000);
            }
        }

        //let x = time::Time::now_ticks();
    }

    loop {}
}
