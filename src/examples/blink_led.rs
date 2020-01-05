#![no_std]
#![no_main]

extern crate cortex_m;
extern crate panic_halt;
pub use cortex_m::peripheral::syst;
use cortex_m::peripheral::Peripherals as CorePeripherals;
use cortex_m_rt::entry;

pub use sam3x8e as target;
mod hal_sam3x8e;

//pub use stm32f407 as target;
//mod hal_stm32f407;

mod hal;

use crate::hal::clock::*;
use crate::hal::pin::*;
use crate::hal::serial::*;
use crate::hal::time::*;
//use crate::hal_stm32f407::core::*;
use crate::hal_sam3x8e::core::*;

// Help: https://rust-embedded.github.io/book/start/registers.html
#[entry]
fn main() -> ! {
    // The CorePeripherals structure contains all the available hardware peripherals which are available for the entire Cortex M family.
    // By calling the CorePeripherals::take() method, we take ownership over these peripherals. We do the same for the microcontroller specific
    // peripherals.
    if let (Some(cp), Some(dp)) = (CorePeripherals::take(), target::Peripherals::take()) {
        setup_core_clock(&dp.PMC, &dp.EFC0, &dp.EFC1);

        // Disable the watchdog by invoking the svd2rust generated API.
        dp.WDT.mr.write(|w| w.wddis().set_bit());

        unsafe {
            //setup_core_clock(&dp.RCC, Some(168_000_000), false); // 168 MHz, use external clock
            //dp.RCC.ahb1enr.write(|w| w.gpioaen().set_bit());
            CLOCK.set_hw_device(dp.PMC);
            CLOCK.enable_peripheral(GPIOA);
            //CLOCK.enable_peripheral(GPIOE);
        }

        let mut on = false;

        // Create a timer.
        let mut t = Time::new(cp.SYST);

        // Initialize the pins.
        //let p9 = create_pin(&dp.PIOA, 9);
        let p9 = create_pin!(&dp.PIOA, 9);
        let p8 = create_pin!(&dp.PIOA, 8);

        let ser = Serial::new(dp.UART, 115200, p9.as_output(), p8.as_input());
        // Serial has now ownership over p8 and p9.
        ser.begin();

        let l = create_pin!(&dp.PIOB, GPIOPins::OnboardLed);
        let led = l.as_output();

        //let pa6 = create_pin(&dp.PIOA, 6);
        //let pa6_output = pa6.as_output();

        //let pa7 = create_pin(&dp.PIOA, 7);
        //let pa7_output = pa7.as_output();

        //let pa0 = create_pin(&dp.PIOA, 0);
        //let button = pa0.as_input();

        //let led = create_output_pin!(&dp.PIOB, GPIOPins::OnboardLed);
        //let led = pin13.as_output();

        //button.disable_pullup();

        // pin13_output.set_high();

        loop {
            t.busy_delay_ms(50);

            if on {
                //ser.write_str_blocking("Turning LED on...\r\n");
                led.set_high();
            //pa7_output.set_low();
            } else {
                //ser.write_str_blocking("Turning LED off...\r\n");
                led.set_low();
                //pa7_output.set_high();
            }

            //pa7_output.set_state(button.get_state());

            on = !on;
        }
    }

    loop {}
}
