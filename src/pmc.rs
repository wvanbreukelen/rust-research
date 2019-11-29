//#![rustc_const_unstable]
// https://github.com/rust-lang/rust/issues/49146

pub use sam3x8e as target;
use crate::peripherals::Peripheral;


pub struct PMCControl {
    rf: Option<target::PMC>
}

impl PMCControl {
    pub fn enable_master_clk(&self) {
        match &self.rf {
            None => {}
            Some(x) => x.pmc_mckr.write_with_zero(|w| unsafe { w.bits(0x01) })
        }
    }

    pub fn get_master_clk(&self) -> u32 {
        match &self.rf {
            None => 0,
            Some(x) => x.pmc_mckr.read().bits()
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
}

pub const PMC: PMCControl = PMCControl { rf: None };