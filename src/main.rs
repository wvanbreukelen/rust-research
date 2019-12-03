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
        // Init the clocks.
        unsafe {
            PMC.setup_pmc(dp.PMC, &dp.EFC0, &dp.EFC1);
            PMC.enable_peripheral(Peripheral::PioA);
        }

        let mut on = false;
        let watchdog = dp.WDT;

        //let mut has_ref = cp.SYST.has_reference_clock();

        // Do something.
        //let mut t = time::Time::new(cp.SYST);

        let mut syst = cp.SYST;

        syst.set_clock_source(syst::SystClkSource::External);
        syst.set_reload(10500); // 10_500 ticks is 1 ms in real life, so 1 us is 1050 ticks in real life.
        syst.enable_counter();

        // Disable the watchdog.
        watchdog.mr.write(|w| w.wddis().set_bit());

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
            let mut counter: u32 = 0;
            while counter < 500 {
                // 500 ms wait
                while !syst.has_wrapped() {}

                counter += 1;
            }

            //t.busy_delay_ms(1000);
            //t.sys_countdown.delay_ms(1000);

            // let mut count_down = MillisCountDown::new(&t.sys_countdown);
            // count_down.start_ms(2000);
            // nb::block!(count_down.wait_ms()).unwrap();

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
