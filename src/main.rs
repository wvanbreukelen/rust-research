#![no_std]
#![no_main]

//#![deny(unsafe_code)]
#[macro_use(singleton)]
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
use crate::pmc::PMC;


// Help: https://rust-embedded.github.io/book/start/registers.html
#[entry]
fn main() -> ! {
    if let (Some(cp), Some(dp)) = (CorePeripherals::take(), target::Peripherals::take()) {
        setup_clocks(&dp.PMC, &dp.EFC0, &dp.EFC1);
        
        

        let mut on = false;
        let watchdog = dp.WDT;

        

        // Do something.
        let mut t = time::Time::new(cp.SYST);

        // Disable the watchdog.
        watchdog.mr.write(|w| w.wddis().set_bit());

        // Init the clocks.
        unsafe {
            PMC.set_pmc(dp.PMC);
            PMC.enable_peripheral(Peripheral::PioA);
            let mut freq = PMC.get_main_clock_frequency_hz(); // 13_209_600 Hz = 13.29 MHz (slow clock)

            freq += 1;
        }

        
        

        // Initialize the pins.
        let p9 = pin::create(&dp.PIOA, 1 << 9);
        let p8 = pin::create(&dp.PIOA, 1 << 8);

        let ser = serial::Serial::new(dp.UART, 115200, p9.as_output(), p8.as_input());
        // Serial has now ownership over p8 and p9.
        ser.begin();

        let pin13 = pin::create(&dp.PIOB, 1 << 27);
        let pin13_output = pin13.as_output();

        //ser.write_str_blocking("Hello World!\r\n");

        loop {
            t.busy_delay_ms(1000);

            if on {
                //ser.write_str_blocking("Turning LED on...\r\n");
                pin13_output.set_high();
            } else {
                //ser.write_str_blocking("Turning LED off...\r\n");
                pin13_output.set_low();
            }
            on = !on;
        }
    }

    loop {}
}

const PMC_MCKR_PRES_CLK_2: u32 = (0x1u32 << 4);
const PMC_MCKR_CSS_PLLA_CLK: u32 = (0x2u32 << 0);
const SYS_BOARD_MCKR: u32 = (PMC_MCKR_PRES_CLK_2 | PMC_MCKR_CSS_PLLA_CLK);
const PMC_MCKR_CSS_MAIN_CLK: u32 = (0x1u32 << 0);
const PMC_MCKR_CSS_Msk: u32 = (0x3u32 << 0);
const EEFC_FMR_FWS_Pos: u32 = 8;
const EEFC_FMR_FWS_Msk: u32 = (0xFu32 << EEFC_FMR_FWS_Pos);

const fn EEFC_FMR_FWS(value: u32) -> u32 {
    ((EEFC_FMR_FWS_Msk & ((value) << EEFC_FMR_FWS_Pos)))
}

fn setup_clocks(pmc: &target::PMC, efc0: &target::EFC0, efc1: &target::EFC1) {
    efc0.fmr.write(|w| unsafe { w.bits(EEFC_FMR_FWS(4))});
    efc1.fmr.write(|w| unsafe { w.bits(EEFC_FMR_FWS(4))});

    pmc.pmc_wpmr.write(|w| w.wpkey().passwd());

    if !pmc.ckgr_mor.read().moscsel().bit_is_set() {
        // 1. Enable external crystal
        pmc.ckgr_mor.write(|w| unsafe {
        w.key().passwd().
        moscxtst().bits(0x8).
        moscrcen().set_bit().
        moscxten().set_bit()
        //moscrcf()._12_mhz()
        });

        // Wait to complete...
        while !pmc.pmc_sr.read().moscxts().bit_is_set() {};
    }

    // 2. Select external crystal as clock source.
    pmc.ckgr_mor.write(|w| unsafe {
        w.key().passwd().
        moscxtst().bits(0x8).
        moscrcen().set_bit().
        moscxten().set_bit().
        //moscrcf()._12_mhz().
        moscsel().set_bit()
    }); // Long waiting time.

    while !pmc.pmc_sr.read().moscsels().bit_is_set() {};

    // 3. Switch to master clock.
    //pmc.pmc_mckr.modify(|r, w| unsafe { w.bits((r.bits() & !(PMC_MCKR_CSS_Msk)) | PMC_MCKR_CSS_MAIN_CLK) });

    //while !pmc.pmc_sr.read().mckrdy().bit_is_set() {};

    // 4. Initialize PLLA.
    pmc.ckgr_pllar.write(|w| unsafe { 
        w.one().set_bit().
        mula().bits(0xD). // Set PLLA multiplier to ...
        pllacount().bits(0x3F).
        diva().bits(0x1)
    });

    while !pmc.pmc_sr.read().locka().bit_is_set() {};

    // 5. Switch to main clock (don't know if this is needed)
    pmc.pmc_mckr.modify(|_, w| unsafe { w.bits((SYS_BOARD_MCKR & (!PMC_MCKR_CSS_Msk)) | PMC_MCKR_CSS_MAIN_CLK) } );
    while !pmc.pmc_sr.read().mckrdy().bit_is_set() {};

    // 6. Switch to PLLA
    pmc.pmc_mckr.write(|w| unsafe { w.bits(SYS_BOARD_MCKR) } );
    while !pmc.pmc_sr.read().mckrdy().bit_is_set() {};
}