//#![rustc_const_unstable]
// https://github.com/rust-lang/rust/issues/49146

pub use sam3x8e as target;
use crate::peripherals::Peripheral;

const MAINFRDY: u32 = 0x00010000;
const MAINF_MASK: u32 = 0x0000ffff;
const SLOW_CLOCK_FREQUENCY_HZ: u32 = 32_768;

pub struct PMCControl {
    rf: Option<target::PMC>
}

impl PMCControl {
    pub fn get_master_clk(&self) -> u32 {
        match &self.rf {
            None => 0,
            Some(x) => x.ckgr_mcfr.read().bits()
        }
    }

    pub fn enable_peripheral(&self, p: Peripheral) {
        match &self.rf {
            None => {},
            Some(x) => x.pmc_pcer0.write_with_zero(|w| unsafe {w.bits(p.mask())})
        }
    }

    pub fn disable_peripheral(&self, p: Peripheral) {
        match &self.rf {
            None => {},
            Some(x) => x.pmc_pcdr0.write_with_zero(|w| unsafe {w.bits(p.mask())})
        }
    }

    pub fn set_pmc(&mut self, _rf: target::PMC) {
        self.rf = Some(_rf);
    }

    pub fn get_main_clock_frequency_hz<'b>(&self) -> u32 {
        let main_clock_frequency_within_16_slow_clock_cycles = {
            while self.get_master_clk() & MAINFRDY == 0 {}
            self.get_master_clk() & MAINF_MASK
        };

        main_clock_frequency_within_16_slow_clock_cycles * SLOW_CLOCK_FREQUENCY_HZ / 16
    }
}

pub static mut PMC: PMCControl = PMCControl { rf: None };

//pub const PMC: PMCControl = PMCControl { rf: None };
//pub static PMC: PMCControl = PMCControl { rf: None };
//let PMC: &'static mut P

//pub static PMC_instance: &'static mut PMCControl = singleton!(: PMCControl = PMCControl { rf: None }).unwrap();