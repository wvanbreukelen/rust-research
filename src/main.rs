#![no_std]
#![no_main]

//#![deny(unsafe_code)]
#[macro_use(singleton)]
extern crate cortex_m;
extern crate panic_halt;

use core::str;
pub use cortex_m::peripheral::syst;
use cortex_m::peripheral::Peripherals as CorePeripherals;
use cortex_m_rt::entry;
use cortex_m_systick_countdown::*;
 use crate::time::Delay;

pub use sam3x8e as target;

mod peripherals;
mod pin;
mod pmc;
mod serial;
mod time;
use crate::peripherals::Peripheral;
use crate::pin::{Configuration, OutputPin};
use crate::pmc::PMC;
use crate::time::BusyDelay;

// Help: https://rust-embedded.github.io/book/start/registers.html
#[entry]
fn main() -> ! {
    if let (Some(cp), Some(dp)) = (CorePeripherals::take(), target::Peripherals::take()) {
        // Disable the watchdog.
        dp.WDT.mr.write(|w| w.wddis().set_bit());

        // Init the clocks.
        unsafe {
            PMC.setup_pmc(dp.PMC, &dp.EFC0, &dp.EFC1);
            PMC.enable_peripheral(Peripheral::PioA);
        }

        let mut on = false;

        // Do something.
        let mut t = time::Time::new(cp.SYST);

        // Initialize the pins.
        let p9 = pin::create(&dp.PIOA, 1 << 9);
        let p8 = pin::create(&dp.PIOA, 1 << 8);

        let ser = serial::Serial::new(dp.UART, 115200, p9.as_output(), p8.as_input());
        // Serial has now ownership over p8 and p9.
        ser.begin();

        let pin13 = pin::create(&dp.PIOB, 1 << 27);
        let pin13_output = pin13.as_output();

        pin13_output.set_high();

        loop {
            t.busy_delay_ms(250);

            if on {
                ser.write_str_blocking("Turning LED on...\r\n");
                pin13_output.set_high();
            } else {
                ser.write_str_blocking("Turning LED off...\r\n");
                pin13_output.set_low();
            }
            on = !on;
        }
    }

    loop {}
}
