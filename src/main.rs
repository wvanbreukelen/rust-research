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
mod time;
mod serial;
use crate::pin::{Configuration, Write, Read};
use crate::time::Delay;

pub struct Blink<PORT> {
    state: bool,
    pin: pin::Pin<pin::IsEnabled, pin::IsOutput, pin::IsValid, PORT>
}

// Help: https://rust-embedded.github.io/book/start/registers.html
#[entry]
fn main() -> ! {
    if let (Some(cp), Some(dp)) = (CorePeripherals::take(), target::Peripherals::take()) {
        let mut on = false;
        let pmc = dp.PMC;
        let watchdog = dp.WDT;
        //let _uart = dp.UART;

        //let mut dev = pin::Device {pioa: dp.PIOA, piob: dp.PIOB, pioc: dp.PIOC, piod: dp.PIOD};

        //let pin13 = pin::Pin::create<piob>();

        pmc.pmc_pcer0
            .write_with_zero(|w| unsafe { w.bits(0x3F << 11) });

        // Disable the watchdog.
        watchdog.mr.write(|w| w.wddis().set_bit());
        // Enable output to p27.
        //piob.oer.write_with_zero(|w| w.p27().set_bit());

        // Turnoff onboard LED.
        //piob.codr.write_with_zero(|w| w.p27().set_bit() );

         // This pin has now ownership over dp.PIOB
                                                    //pin13.enable();

        //let _size = size_of::<pin::Pin<target::PIOB, pin::IsDisabled, pin::Unknown>>();
        //const_assert_eq!(0, size_of::<pin::IsEnabled>());

        //let pin13_output = pin13.as_output();

        //pin13_output.disable(&dp.PIOB);

        //let pin13_input = pin13_output.as_input(&dp.PIOB);

        //pin::pin1

        //pin13_output.set_low();

        let pin13_output = pin::create(1 << 27, target::PIOB::ptr()).as_output();

        // Do something.
        let mut t = time::Time::syst(cp.SYST);

        t.delay_ms(2_000_000);

        loop {
            if t.has_wrapped() {
                // Turn on the LED!
                if on {
                    pin13_output.set_high();
                    //dp.PIOB.codr.write_with_zero(|w| w.p27().set_bit() );
                } else {
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
