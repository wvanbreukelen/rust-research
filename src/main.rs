#![no_std]
#![no_main]

extern crate cortex_m;
extern crate panic_halt;
pub use cortex_m::peripheral::syst;
use cortex_m::peripheral::Peripherals as CorePeripherals;
use cortex_m_rt::entry;

// pub use sam3x8e as target;
// mod hal_sam3x8e;

pub use stm32f407 as target;
mod hal_stm32f407;

mod hal;

use crate::hal::clock::*;
use crate::hal::time::*;
use crate::hal::pin::*;
use crate::hal::serial::*;
use crate::hal_stm32f407::core::*;
//use crate::hal_sam3x8e::core::*;

// Help: https://rust-embedded.github.io/book/start/registers.html
#[entry]
fn main() -> ! {
    if let (Some(cp), Some(dp)) = (CorePeripherals::take(), target::Peripherals::take()) {
        // Disable the watchdog.
        //dp.WDT.mr.write(|w| w.wddis().set_bit());

        // Init the clocks.
        unsafe {
            //setup_core_clock(&dp.RCC, Some(168_000_000), false); // 168 MHz, use external clock
            //dp.RCC.ahb1enr.write(|w| w.gpioaen().set_bit());
            CLOCK.set_hw_device(dp.RCC);
            CLOCK.enable_peripheral(GPIOA);
            CLOCK.enable_peripheral(GPIOE);
        }

        let mut on = false;

        // Create a timer.
        let mut t = Time::new(cp.SYST);

        // Initialize the pins.
        //let p9 = create_pin(&dp.PIOA, 1 << 9);
        //let p8 = create_pin(&dp.PIOA, 1 << 8);

        //let ser = Serial::new(dp.UART, 115200, p9.as_output(), p8.as_input());
        // Serial has now ownership over p8 and p9.
        //ser.begin();

        
        let pa6 = create_pin(&dp.GPIOA, 6);
        let pa6_output = pa6.as_output();

        let pa7 = create_pin(&dp.GPIOA, 7);
        let pa7_output = pa7.as_output();

        let pa0 = create_pin(&dp.GPIOA, 0);
        let button = pa0.as_input();

        button.disable_pullup();

        // pin13_output.set_high();

        loop {
            t.busy_delay_ms(10);

            if on {
                //ser.write_str_blocking("Turning LED on...\r\n");
                pa6_output.set_high();
                //pa7_output.set_low();
            } else {
                //ser.write_str_blocking("Turning LED off...\r\n");
                pa6_output.set_low();
                //pa7_output.set_high();
            }

            //pa7_output.set_state(button.get_state());


            on = !on;
        }
    }

    loop {}
}
