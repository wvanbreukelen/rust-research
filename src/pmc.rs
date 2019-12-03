//#![rustc_const_unstable]
// https://github.com/rust-lang/rust/issues/49146

use crate::peripherals::Peripheral;
pub use sam3x8e as target;

const MAINFRDY: u32 = 0x00010000;
const MAINF_MASK: u32 = 0x0000ffff;
const SLOW_CLOCK_FREQUENCY_HZ: u32 = 32_768;

const PMC_MCKR_PRES_CLK_2: u32 = (0x1u32 << 4);
const PMC_MCKR_CSS_PLLA_CLK: u32 = (0x2u32 << 0);
const SYS_BOARD_MCKR: u32 = (PMC_MCKR_PRES_CLK_2 | PMC_MCKR_CSS_PLLA_CLK);
const PMC_MCKR_CSS_MAIN_CLK: u32 = (0x1u32 << 0);
const PMC_MCKR_CSS_Msk: u32 = (0x3u32 << 0);
const EEFC_FMR_FWS_Pos: u32 = 8;
const EEFC_FMR_FWS_Msk: u32 = (0xFu32 << EEFC_FMR_FWS_Pos);

const fn EEFC_FMR_FWS(value: u32) -> u32 {
    (EEFC_FMR_FWS_Msk & ((value) << EEFC_FMR_FWS_Pos))
}

pub struct PMCControl {
    rf: Option<target::PMC>,
}

impl PMCControl {
    pub fn get_master_clk(&self) -> u32 {
        match &self.rf {
            None => 0,
            Some(x) => x.ckgr_mcfr.read().bits(),
        }
    }

    pub fn enable_peripheral(&self, p: Peripheral) {
        match &self.rf {
            None => {}
            Some(x) => x.pmc_pcer0.write_with_zero(|w| unsafe { w.bits(p.mask()) }),
        }
    }

    pub fn disable_peripheral(&self, p: Peripheral) {
        match &self.rf {
            None => {}
            Some(x) => x.pmc_pcdr0.write_with_zero(|w| unsafe { w.bits(p.mask()) }),
        }
    }

    pub fn setup_pmc(&mut self, _rf: target::PMC, efc0: &target::EFC0, efc1: &target::EFC1) {
        setup_clocks(&_rf, &efc0, &efc1);

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

fn setup_clocks(pmc: &target::PMC, efc0: &target::EFC0, efc1: &target::EFC1) {
    efc0.fmr.write(|w| unsafe { w.bits(EEFC_FMR_FWS(4)) });
    efc1.fmr.write(|w| unsafe { w.bits(EEFC_FMR_FWS(4)) });

    pmc.pmc_wpmr.write(|w| w.wpkey().passwd());

    if !pmc.ckgr_mor.read().moscsel().bit_is_set() {
        // 1. Enable external crystal
        pmc.ckgr_mor.write(|w| unsafe {
            w.key()
                .passwd()
                .moscxtst()
                .bits(0x8)
                .moscrcen()
                .set_bit()
                .moscxten()
                .set_bit()
            //moscrcf()._12_mhz()
        });

        // Wait to complete...
        while !pmc.pmc_sr.read().moscxts().bit_is_set() {}
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

    while !pmc.pmc_sr.read().moscsels().bit_is_set() {}

    // 3. Switch to master clock.
    pmc.pmc_mckr
        .modify(|r, w| unsafe { w.bits((r.bits() & !(PMC_MCKR_CSS_Msk)) | PMC_MCKR_CSS_MAIN_CLK) });

    while !pmc.pmc_sr.read().mckrdy().bit_is_set() {}

    // 4. Initialize PLLA.
    pmc.ckgr_pllar.write(|w| unsafe {
        w.one().set_bit().
        mula().bits(0xD). // Set PLLA multiplier to ...
        pllacount().bits(0x3F).
        diva().bits(0x1)
    });

    while !pmc.pmc_sr.read().locka().bit_is_set() {}

    // 5. Switch to main clock (don't know if this is needed)
    pmc.pmc_mckr.modify(|_, w| unsafe {
        w.bits((SYS_BOARD_MCKR & (!PMC_MCKR_CSS_Msk)) | PMC_MCKR_CSS_MAIN_CLK)
    });
    while !pmc.pmc_sr.read().mckrdy().bit_is_set() {}

    // 6. Switch to PLLA
    pmc.pmc_mckr.write(|w| unsafe { w.bits(SYS_BOARD_MCKR) });
    while !pmc.pmc_sr.read().mckrdy().bit_is_set() {}
}

pub static mut PMC: PMCControl = PMCControl { rf: None };

//pub const PMC: PMCControl = PMCControl { rf: None };
//pub static PMC: PMCControl = PMCControl { rf: None };
//let PMC: &'static mut P

//pub static PMC_instance: &'static mut PMCControl = singleton!(: PMCControl = PMCControl { rf: None }).unwrap();
