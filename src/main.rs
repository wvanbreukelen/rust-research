#![no_std]
#![no_main]

extern crate cortex_m;
extern crate panic_halt;
pub use cortex_m::peripheral::syst;
use cortex_m::peripheral::Peripherals as CorePeripherals;
use cortex_m_rt::entry;

pub use sam3x8e;
mod hal_sam3x8e;
mod hal;

use crate::hal::pmc::*;
use crate::hal::time::*;
use crate::hal::pin::*;
use crate::hal::serial::*;
use crate::hal_sam3x8e::core::*;

// Help: https://rust-embedded.github.io/book/start/registers.html
#[entry]
fn main() -> ! {
    if let (Some(cp), Some(dp)) = (CorePeripherals::take(), sam3x8e::Peripherals::take()) {
        // Disable the watchdog.
        dp.WDT.mr.write(|w| w.wddis().set_bit());

        // Init the clocks.
        unsafe {
            setup_core_clock(&dp.PMC, &dp.EFC0, &dp.EFC1);
            PMC.set_hw_pmc(dp.PMC);
            PMC.enable_peripheral(Peripheral::PioA);
        }

        let mut on = false;

        // Create a timer.
        let mut t = Time::new(cp.SYST);

        // Initialize the pins.
        let p9 = create_pin(&dp.PIOA, 1 << 9);
        let p8 = create_pin(&dp.PIOA, 1 << 8);

        let ser = Serial::new(dp.UART, 115200, p9.as_output(), p8.as_input());
        // Serial has now ownership over p8 and p9.
        ser.begin();

        let pin13 = create_pin(&dp.PIOB, 1 << 27);
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
