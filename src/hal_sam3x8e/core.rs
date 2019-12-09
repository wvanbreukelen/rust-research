#![no_std]

use crate::hal::pmc::*;

pub static mut PMC: PMCControl<sam3x8e::PMC> = PMCControl { rf: None };

pub const MAINFRDY: u32 = 0x00010000;
pub const MAINF_MASK: u32 = 0x0000ffff;
pub const SLOW_CLOCK_FREQUENCY_HZ: u32 = 32_768;

pub const PMC_MCKR_PRES_CLK_2: u32 = (0x1u32 << 4);
pub const PMC_MCKR_CSS_PLLA_CLK: u32 = (0x2u32 << 0);
pub const SYS_BOARD_MCKR: u32 = (PMC_MCKR_PRES_CLK_2 | PMC_MCKR_CSS_PLLA_CLK);
pub const PMC_MCKR_CSS_MAIN_CLK: u32 = (0x1u32 << 0);
pub const PMC_MCKR_CSS_MSK: u32 = (0x3u32 << 0);
pub const EEFC_FMR_FWS_POS: u32 = 8;
pub const EEFC_FMR_FWS_MSK: u32 = (0xFu32 << EEFC_FMR_FWS_POS);

// Taken from: https://github.com/klangner/sam3x/blob/master/src/hal/peripherals.rs
pub const SUPC      : u32 = 0 ; // Supply Controller
pub const RSTC      : u32 = 1 ; // Reset Controller
pub const RTC       : u32 = 2 ; // Real-time Clock
pub const RTT       : u32 = 3 ; // Real-time Timer
pub const WDT       : u32 = 4 ; // Watchdog Timer
pub const P_PMC       : u32 = 5 ; // Power Management Controller
pub const EEFC0     : u32 = 6 ; // Enhanced Embedded Flash Controller 0
pub const EEFC1     : u32 = 7 ; // Enhanced Embedded Flash Controller 1
pub const UART      : u32 = 8 ; // Universal Asynchronous Receiver Transmitter
pub const SMC_SDRAMC: u32 = 9 ; // Static Memory Controller / Synchronous
// Dynamic RAM Controller
pub const SDRAMC    : u32 = 10; // Synchronous Dynamic RAM Controller
pub const PIOA      : u32 = 11; // Parallel I/O Controller A
pub const PIOB      : u32 = 12; // Parallel I/O Controller B
pub const PIOC      : u32 = 13; // Parallel I/O Controller C
pub const PIOD      : u32 = 14; // Parallel I/O Controller D
pub const PIOE      : u32 = 15; // Parallel I/O Controller E
pub const PIOF      : u32 = 16; // Parallel I/O Controller F
pub const USART0    : u32 = 17; // Universal Synchronous Asynchronous Receiver
// Transmitter 0
pub const USART1    : u32 = 18; // Universal Synchronous Asynchronous Receiver
// Transmitter 1
pub const USART2    : u32 = 19; // Universal Synchronous Asynchronous Receiver
// Transmitter 2
pub const USART3    : u32 = 20; // Universal Synchronous Asynchronous Receiver
// Transmitter 3
pub const HSMCI     : u32 = 21; // High Speed Multimedia Card Interface
pub const TWI0      : u32 = 22; // Two-Wire Interface 0
pub const TWI1      : u32 = 23; // Two-Wire Interface 1
pub const SPI0      : u32 = 24; // Serial Peripheral Interface 0
pub const SPI1      : u32 = 25; // Serial Peripheral Interface 1
pub const SSC       : u32 = 26; // Synchronous Serial Controller
pub const TC0       : u32 = 27; // Timer Counter Channel 0
pub const TC1       : u32 = 28; // Timer Counter Channel 1
pub const TC2       : u32 = 29; // Timer Counter Channel 2
pub const TC3       : u32 = 30; // Timer Counter Channel 3
pub const TC4       : u32 = 31; // Timer Counter Channel 4
pub const TC5       : u32 = 32; // Timer Counter Channel 5
pub const TC6       : u32 = 33; // Timer Counter Channel 6
pub const TC7       : u32 = 34; // Timer Counter Channel 7
pub const TC8       : u32 = 35; // Timer Counter Channel 8
pub const PWM       : u32 = 36; // Pulse Width Modulation Controller
pub const ADC       : u32 = 37; // ADC Controller
pub const DACC      : u32 = 38; // DAC Controller
pub const DMAC      : u32 = 39; // DMA Controller
pub const UOTGHS    : u32 = 40; // USB OTG High Speed
pub const TRNG      : u32 = 41; // True Random Number Generator
pub const EMAC      : u32 = 42; // Ethernet MAC
pub const CAN0      : u32 = 43; // CAN Controller 0
pub const CAN1      : u32 = 44; // CAN Controller 1


impl Peripheral {
    pub fn id(&self) -> u32 {
        *self as u32
    }

    pub fn index(&self) -> usize {
        self.id() as usize / 32
    }

    pub fn mask(&self) -> u32 {
        0x1 << (self.id() % 32)
    }
}


#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Peripheral {
    Supc      = SUPC,
    Rstc      = RSTC,
    Rtc       = RTC,
    Rtt       = RTT,
    Wdt       = WDT,
    Pmc       = P_PMC,
    Eefc0     = EEFC0,
    Eefc1     = EEFC1,
    Uart      = UART,
    SmcSdramc = SMC_SDRAMC,
    Sdramc    = SDRAMC,
    PioA      = PIOA,
    PioB      = PIOB,
    PioC      = PIOC,
    PioD      = PIOD,
    PioE      = PIOE,
    PioF      = PIOF,
    Usart0    = USART0,
    Usart1    = USART1,
    Usart2    = USART2,
    Usart3    = USART3,
    Hsmci     = HSMCI,
    Twi0      = TWI0,
    Twi1      = TWI1,
    Spi0      = SPI0,
    Spi1      = SPI1,
    Ssc       = SSC,
    Tc0       = TC0,
    Tc1       = TC1,
    Tc2       = TC2,
    Tc3       = TC3,
    Tc4       = TC4,
    Tc5       = TC5,
    Tc6       = TC6,
    Tc7       = TC7,
    Tc8       = TC8,
    Pwm       = PWM,
    Adc       = ADC,
    Dacc      = DACC,
    Dmac      = DMAC,
    UtogHs    = UOTGHS,
    Trng      = TRNG,
    Emac      = EMAC,
    Can0      = CAN0,
    Can1      = CAN1,
}

const fn EEFC_FMR_FWS(value: u32) -> u32 {
    (EEFC_FMR_FWS_MSK & ((value) << EEFC_FMR_FWS_POS))
}

pub fn setup_clocks(pmc: &sam3x8e::PMC, efc0: &sam3x8e::EFC0, efc1: &sam3x8e::EFC1) {
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
        .modify(|r, w| unsafe { w.bits((r.bits() & !(PMC_MCKR_CSS_MSK)) | PMC_MCKR_CSS_MAIN_CLK) });

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
        w.bits((SYS_BOARD_MCKR & (!PMC_MCKR_CSS_MSK)) | PMC_MCKR_CSS_MAIN_CLK)
    });
    while !pmc.pmc_sr.read().mckrdy().bit_is_set() {}

    // 6. Switch to PLLA
    pmc.pmc_mckr.write(|w| unsafe { w.bits(SYS_BOARD_MCKR) });
    while !pmc.pmc_sr.read().mckrdy().bit_is_set() {}
}
