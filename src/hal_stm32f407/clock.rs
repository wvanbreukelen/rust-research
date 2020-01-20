use stm32f407;

use crate::hal::clock::*;
use crate::hal_stm32f407::core::*;

impl PMCConfigure<stm32f407::RCC> for PMCControl<stm32f407::RCC> {
    fn set_hw_device(&mut self, dev: stm32f407::RCC) {
        self.rf = Some(dev);
    }
}

// impl PMCRead for PMCControl<stm32f407::RCC> {
//     fn get_master_clk(&self) -> u32 {
//         match &self.rf {
//             None => 0,
//             Some(x) => x.ckgr_mcfr.read().bits(),
            
//         }
//     }

//     fn get_main_clock_frequency_hz<'b>(&self) -> u32 {
//         // let main_clock_frequency_within_16_slow_clock_cycles = {
//         //     while self.get_master_clk() & MAINFRDY == 0 {}
//         //     self.get_master_clk() & MAINF_MASK
//         // };

//         // main_clock_frequency_within_16_slow_clock_cycles * SLOW_CLOCK_FREQUENCY_HZ / 16
//         0
//     }
// }

impl PMCWrite<PeripheralListing> for PMCControl<stm32f407::RCC> {
    fn enable_peripheral(&self, p: PeripheralListing) {
        unsafe {
            match &self.rf {
                None => {}
                Some(x) => match p.rcc_port {
                    RCCPort::ONE => x.ahb1enr.modify(|r, w| w.bits(r.bits() | (1 << p.offset))),
                    RCCPort::TWO => x.ahb2enr.modify(|r, w| w.bits(r.bits() | (1 << p.offset))),
                    RCCPort::THREE => x.ahb3enr.modify(|r, w| w.bits(r.bits() | (1 << p.offset))), 
                }
            }
        }
    }

    fn disable_peripheral(&self, p: PeripheralListing) {
        unsafe {
            match &self.rf {
                None => {}
                Some(x) => match p.rcc_port {
                    RCCPort::ONE => x.ahb1enr.modify(|r, w| w.bits(r.bits() & !(1 << p.offset))),
                    RCCPort::TWO => x.ahb2enr.modify(|r, w| w.bits(r.bits() & !(1 << p.offset))),
                    RCCPort::THREE => x.ahb3enr.modify(|r, w| w.bits(r.bits() & !(1 << p.offset))) 
                }
            }
        }
    }


}

fn call_rcc_port(listing : &PeripheralListing) {
    
}

