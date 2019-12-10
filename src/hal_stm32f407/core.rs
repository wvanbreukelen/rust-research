#![no_std]

use crate::hal::clock::*;

pub static mut CLOCK: PMCControl<stm32f407::RCC> = PMCControl { rf: None };

const HSI: u32 = 16_000_000; // Hz

#[derive(Clone, Copy)]
pub enum RCCPort { ONE, TWO, THREE }

#[derive(Clone, Copy)]
pub struct PeripheralListing {
    pub offset: u32,
    pub rcc_port: RCCPort
}

pub const GPIOA      : PeripheralListing = PeripheralListing { offset: 0, rcc_port : RCCPort::ONE }; // Parallel I/O Controller A
pub const GPIOB      : PeripheralListing = PeripheralListing { offset: 1, rcc_port : RCCPort::ONE }; // Parallel I/O Controller B
pub const GPIOC      : PeripheralListing = PeripheralListing { offset: 2, rcc_port : RCCPort::ONE }; // Parallel I/O Controller C
pub const GPIOD      : PeripheralListing = PeripheralListing { offset: 3, rcc_port : RCCPort::ONE }; // Parallel I/O Controller D
pub const GPIOE      : PeripheralListing = PeripheralListing { offset: 4, rcc_port : RCCPort::ONE }; // Parallel I/O Controller E
pub const GPIOF      : PeripheralListing = PeripheralListing { offset: 5, rcc_port : RCCPort::ONE }; // Parallel I/O Controller F
pub const GPIOG      : PeripheralListing = PeripheralListing { offset: 6, rcc_port : RCCPort::ONE }; // Parallel I/O Controller F
pub const GPIOH      : PeripheralListing = PeripheralListing { offset: 7, rcc_port : RCCPort::ONE }; // Parallel I/O Controller F
pub const GPIOI      : PeripheralListing = PeripheralListing { offset: 8, rcc_port : RCCPort::ONE }; // Parallel I/O Controller F
pub const GPIOJ      : PeripheralListing = PeripheralListing { offset: 9, rcc_port : RCCPort::ONE }; // Parallel I/O Controller F
pub const USART1     : PeripheralListing = PeripheralListing { offset: 4, rcc_port : RCCPort::TWO }; // Universal Synchronous Asynchronous Receiver


// Source: https://github.com/stm32-rs/stm32f4xx-hal/blob/e94c88ec85488445bbef10542e21173a99781364/src/rcc.rs
pub fn setup_core_clock(rcc: &stm32f407::RCC, desired_core_clk: Option<u32>, use_hse: bool) -> (bool, u32) {
    // /let mut hse: u32;
    //let mut hclk: Option<u32>;
    //let mut pclk1: Option<u32>;
    //let mut pclk2: Option<u32>;
    //let mut sysclk: u32;


    let pllsrcclk = HSI;
    let sysclk = desired_core_clk.unwrap_or(pllsrcclk);

    // Sysclk output divisor must be one of 2, 4, 6 or 8
    let sysclk_div = core::cmp::min(8, (432_000_000 / sysclk) & !1);

    // Input divisor from PLL source clock, must result to frequency in
    // the range from 1 to 2 MHz
    let pllm_min = (pllsrcclk + 1_999_999) / 2_000_000;
    let pllm_max = pllsrcclk / 1_000_000;

    // Find the lowest pllm value that minimize the difference between
    // requested sysclk and actual sysclk.
    let pllm = (pllm_min..=pllm_max).min_by_key(|pllm| {
        let vco_in = pllsrcclk / pllm;
        let plln = sysclk * sysclk_div / vco_in;
        sysclk - (vco_in * plln / sysclk_div)
    }).unwrap();

    let vco_in = pllsrcclk / pllm;
    assert!(vco_in >= 1_000_000 && vco_in <= 2_000_000);

    // Main scaler, must result in >= 100MHz (>= 192MHz for F401)
    // and <= 432MHz, min 50, max 432
    let plln = sysclk * sysclk_div / vco_in;

    let pllp = (sysclk_div / 2) - 1;

    // Calculate real system clock
    let sysclk = vco_in * plln / sysclk_div;

    if sysclk != pllsrcclk {
        // use PLL as source
        // rcc.pllcfgr.write(|w| unsafe {
        //     w
        //         .bits(pllm as u8)
        //         .plln()
        //         .bits(plln as u16)
        //         .pllp()
        //         .bits(pllp as u8)
        //         .pllsrc()
        //         .bit(self.hse.is_some())
        // });
        // ((pllm as u8)) | 
        // ((plln as u32) << 7) | 

        rcc.pllcfgr.write(|w| unsafe {
            w.bits((pllm as u32) | ((plln as u32) << 7) | ((pllp as u32) << 15)). // Set pllm & plln & pllp
            pllsrc().bit(use_hse)
        });

        (true, sysclk)
    } else {
        (false, pllsrcclk)
    }
}

// impl Peripheral {
//     pub fn id(&self) -> u32 {
//         *self as u32
//     }

//     pub fn index(&self) -> usize {
//         self.id() as usize / 32
//     }

//     pub fn mask(&self) -> u32 {
//         0x1 << (self.id() % 32)
//     }
// }


// #[derive(Clone, Copy)]
// //#[repr(u32)]
// pub enum Peripheral {
//     GpioA        = GPIOA,
//     GpioB        = GPIOB,
//     GpioC        = GPIOC,
//     GpioD        = GPIOD,
//     GpioE        = GPIOE,
//     GpioF        = GPIOF,
//     Usart1       = USART1
//     // Supc      = SUPC,
//     // Rstc      = RSTC,
//     // Rtc       = RTC,
//     // Rtt       = RTT,
//     // Wdt       = WDT,
//     // Pmc       = P_PMC,
//     // Eefc0     = EEFC0,
//     // Eefc1     = EEFC1,
//     // Uart      = UART,
//     // SmcSdramc = SMC_SDRAMC,
//     // Sdramc    = SDRAMC,
//     // PioA      = PIOA,
//     // PioB      = PIOB,
//     // PioC      = PIOC,
//     // PioD      = PIOD,
//     // PioE      = PIOE,
//     // PioF      = PIOF,
//     // Usart0    = USART0,
//     // Usart1    = USART1,
//     // Usart2    = USART2,
//     // Usart3    = USART3,
//     // Hsmci     = HSMCI,
//     // Twi0      = TWI0,
//     // Twi1      = TWI1,
//     // Spi0      = SPI0,
//     // Spi1      = SPI1,
//     // Ssc       = SSC,
//     // Tc0       = TC0,
//     // Tc1       = TC1,
//     // Tc2       = TC2,
//     // Tc3       = TC3,
//     // Tc4       = TC4,
//     // Tc5       = TC5,
//     // Tc6       = TC6,
//     // Tc7       = TC7,
//     // Tc8       = TC8,
//     // Pwm       = PWM,
//     // Adc       = ADC,
//     // Dacc      = DACC,
//     // Dmac      = DMAC,
//     // UtogHs    = UOTGHS,
//     // Trng      = TRNG,
//     // Emac      = EMAC,
//     // Can0      = CAN0,
//     // Can1      = CAN1,
// }