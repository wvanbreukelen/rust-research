use sam3x8e;

use crate::hal::clock::*;
use crate::hal::pin::*;
use crate::hal_sam3x8e::core::*;

impl PMCConfigure<sam3x8e::PMC> for PMCControl<sam3x8e::PMC> {
    fn set_hw_device(&mut self, pmc: sam3x8e::PMC) {
        self.rf = Some(pmc);
    }
}

impl PMCRead for PMCControl<sam3x8e::PMC> {
    fn get_master_clk(&self) -> u32 {
        match &self.rf {
            None => 0,
            Some(x) => x.ckgr_mcfr.read().bits(),
        }
    }

    fn get_main_clock_frequency_hz<'b>(&self) -> u32 {
        let main_clock_frequency_within_16_slow_clock_cycles = {
            while self.get_master_clk() & MAINFRDY == 0 {}
            self.get_master_clk() & MAINF_MASK
        };

        main_clock_frequency_within_16_slow_clock_cycles * SLOW_CLOCK_FREQUENCY_HZ / 16
    }
}

impl PMCWrite<PeripheralListing> for PMCControl<sam3x8e::PMC> {
    fn enable_peripheral(&self, p: PeripheralListing) {
        match &self.rf {
            None => {}
            Some(x) => x.pmc_pcer0.write_with_zero(|w| unsafe { w.bits(p.offset) }),
        }
    }

    fn disable_peripheral(&self, p: PeripheralListing) {
        match &self.rf {
            None => {}
            Some(x) => x.pmc_pcdr0.write_with_zero(|w| unsafe { w.bits(p.offset) }),
        }
    }
}
