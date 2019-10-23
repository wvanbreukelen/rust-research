
#![no_std]
#![no_main]
#[macro_use]

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

use cortex_m_rt::entry;
pub use cortex_m::peripheral::syst;
use cortex_m::peripheral::Peripherals as CorePeripherals;
//extern crate sam3x8e;

pub use sam3x8e as target;

mod time;
mod pin;
//mod serial;
use crate::time::Delay;
use crate::pin::Configuration;



// Help: https://rust-embedded.github.io/book/start/registers.html
#[entry]
fn main() -> ! {
    if let (Some(cp), Some(dp)) = (
        CorePeripherals::take(),
        target::Peripherals::take()
    ) {
        let mut on = false;
            
        let pmc = dp.PMC;
        let watchdog = dp.WDT;
        let uart = dp.UART;


        

        //let pin13 = pin::Pin::create<piob>();

        //create_pin!(piob);

        pmc.pmc_pcer0.write_with_zero(|w| unsafe { w.bits(0x3F << 11)});

        // Disable the watchdog.
        watchdog.mr.write(|w| w.wddis().set_bit() );
    
        // Enable output to p27.
        //piob.oer.write_with_zero(|w| w.p27().set_bit());

        // Turnoff onboard LED.
        //piob.codr.write_with_zero(|w| w.p27().set_bit() );

        let pin13 = pin::create(&dp.PIOB, 1 << 27); // This pin has now ownership over dp.PIOB
        //pin13.enable();
        //pin13.set_state(true);

        pin13.


        // Do something.
        let mut t = time::Time::syst(cp.SYST);

        t.delay_ms(2_000_000);

        loop {
            
            if t.has_wrapped() {
                // Turn on the LED!
                if on {
                    //pin13.set_state(true);
                } else {
                    //pin13.set_state(false);
                }
                
                on = !on;
                t.delay_ms(2_000_000);
            }
        }

        //let x = time::Time::now_ticks();
    }

    loop {}
}