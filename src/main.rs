#![no_std]
#![no_main]
//#![deny(unsafe_code)]
extern crate cortex_m;
extern crate panic_halt;

pub use cortex_m::peripheral::syst;
use cortex_m::peripheral::Peripherals as CorePeripherals;
use cortex_m_rt::entry;

pub use sam3x8e as target;

mod pin;
mod serial;
mod time;
mod pmc;
mod peripherals;
use crate::pin::{Configuration, OutputPin};
use crate::time::BusyDelay;
use crate::peripherals::Peripheral;

// Help: https://rust-embedded.github.io/book/start/registers.html
#[entry]
fn main() -> ! {
    if let (Some(cp), Some(dp)) = (CorePeripherals::take(), target::Peripherals::take()) {
        let mut on = false;
        let watchdog = dp.WDT;

        // Do something.
        let mut t = time::Time::new(cp.SYST);

        // Disable the watchdog.
        watchdog.mr.write(|w| w.wddis().set_bit());

        // Init the clocks.
        pmc::PMC.set_pmc(dp.PMC);
        pmc::PMC.enable_master_clk();
        pmc::PMC.enable_peripheral(Peripheral::PioA);

        // Initialize the pins.
        let p9 = pin::create(&dp.PIOA, 1 << 9);
        let p8 = pin::create(&dp.PIOA, 1 << 8);

        let ser = serial::Serial::new(dp.UART, 115200, p9.as_output(), p8.as_input());
        // Serial has now ownership over p8 and p9.
        ser.begin();

        let pin13 = pin::create(&dp.PIOB, 1 << 27);
        let pin13_output = pin13.as_output();

        loop {
            t.busy_delay_ms(1000);

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
