//#![deny(warnings)]

use crate::pin::Configuration;
use crate::pin::{InputPin, OutputPin};
use core::convert::TryFrom;
use core::u16;
pub use cortex_m::peripheral::syst;
use nb;
pub use sam3x8e as target;

use crate::pin;

//mod pin;

const MAINFRDY: u32 = 0x00010000;
const MAINF_MASK: u32 = 0x0000ffff;
const SLOW_CLOCK_FREQUENCY_HZ: u32 = 32_768;

pub struct Serial<'pins> {
    handle: target::UART,
    clock_div: u32,
    pin_tx: pin::Pin<'pins, target::PIOA, pin::IsEnabled, pin::IsOutput>,
    pin_rx: pin::Pin<'pins, target::PIOA, pin::IsEnabled, pin::IsInput>,
}

pub enum SerialError {
    TxNotReady,
    RxEmpty,
    Timeout,
}

impl<'pins> Serial<'pins> {
    // Pin<ENABLED, DIRECTION, VALID>
    //pub fn create_serial_handle<'a>(_handle: target::UART, pin_tx: pin::Pin<pin::IsEnabled, pin::IsOutput, pin::IsValid>, pin_rx: pin::Pin<pin::IsEnabled, pin::IsInput, pin::IsValid>, baudrate: u32) -> Serial {
    // Pin: IsDisabled, Unknown, IsValid

    // Disadvantage of Rust. We need to be explicit about all generic parameters. We cannot perform function overloading.

    pub fn new(
        _handle: target::UART,
        pmc: &target::PMC,
        baudrate: u32,
        _pin_tx: pin::Pin<'pins, target::PIOA, pin::IsEnabled, pin::IsOutput>,
        _pin_rx: pin::Pin<'pins, target::PIOA, pin::IsEnabled, pin::IsInput>,
    ) -> Self {
        // Return a new instance
        return Self {
            handle: _handle,
            clock_div: calc_divider_safe(main_clock_frequency_hz(pmc), baudrate),
            pin_tx: _pin_tx,
            pin_rx: _pin_rx,
        };
    }

    pub fn disable(&self) {}

    pub fn begin<'b>(&self, pio: &'b target::PIOA, pmc: &'b target::PMC) {
        self.pin_tx.enable_pullup();
        self.pin_tx.switch_to_a();

        self.pin_rx.enable_pullup();
        self.pin_rx.switch_to_a();

        pmc.pmc_pcer0
            .write_with_zero(|w| w.pid11().set_bit().pid12().set_bit()); // Enable PIOA, PIOB
                                                                         // Set pins to right mode.

        //pio.puer.write_with_zero(|w| w.p9().set_bit()); // Tx pin pullup

        pmc.pmc_pcer0.write_with_zero(|w| w.pid8().set_bit()); // Enable UART clock

        // Disable UART
        self.handle.cr.write_with_zero(|w| {
            w.rstrx()
                .set_bit()
                .rsttx()
                .set_bit()
                .rxdis()
                .set_bit()
                .txdis()
                .set_bit()
        });

        // Set the baudrate
        self.handle
            .brgr
            .write_with_zero(|w| unsafe { w.bits(self.clock_div) });

        // Disable parity bits, go in normal mode.
        self.handle.mr.write_with_zero(|w| w.par().no());

        self.handle
            .idr
            .write_with_zero(|w| unsafe { w.bits(0xFFFFFFFF) }); // Disable interrupts.

        // Enable UART
        self.handle
            .cr
            .write_with_zero(|w| w.txen().set_bit().rxen().set_bit());
    }

    pub fn write_array_blocking(&self, arr: &[u8]) {
        for &ch in arr.iter() {
            self.write_byte_blocking(ch);
        }
    }

    pub fn read_array_blocking(&self, arr: &mut [u8]) -> nb::Result<(), ()> {
        for ch in arr.iter_mut() {
            match self.read_byte_blocking() {
                //Err(e @ SerialError::Timeout) => return Err(()),
                Some(ch_read) => *ch = ch_read,
                None => {}
            }
        }

        return Ok(());
    }

    pub fn write_str_blocking(&self, s: &str) {
        for ch in s.chars() {
            self.write_byte_blocking(ch as u8);
        }
    }

    pub fn write_byte_blocking(&self, ch: u8) {
        nb::block!(self.write_byte(ch)).unwrap();
    }

    pub fn read_byte_blocking(&self) -> Option<u8> {
        match nb::block!(self.read_byte()) {
            Ok(ch_read) => Some(ch_read),
            _ => None,
        }
    }

    fn write_byte<'b>(&self, ch: u8) -> nb::Result<(), ()> {
        if !self.handle.sr.read().txrdy().bit_is_set() {
            return Err(nb::Error::WouldBlock); // Or Error::WouldBlock
        }

        self.handle
            .thr
            .write_with_zero(|w| unsafe { w.txchr().bits(ch) });
        //self.handle.thr.write_with_zero(|w| unsafe {w.bits(0xFF as u32)});
        return Ok(());
    }

    fn read_byte(&self) -> nb::Result<u8, ()> {
        if !self.handle.sr.read().rxrdy().bit() {
            return Err(nb::Error::WouldBlock);
        }

        return Ok(self.handle.rhr.read().rxchr().bits());
    }
}

fn main_clock_frequency_hz<'b>(pmc: &'b target::PMC) -> u32 {
    let main_clock_frequency_within_16_slow_clock_cycles = unsafe {
        while pmc.ckgr_mcfr.read().bits() & MAINFRDY == 0 {}
        pmc.ckgr_mcfr.read().bits() & MAINF_MASK
    };

    main_clock_frequency_within_16_slow_clock_cycles * SLOW_CLOCK_FREQUENCY_HZ / 16
}

// fn calc_divider_safe(clock: u32, baudrate: u32) -> Option<u16> {
//     return u16::try_from(clock / (baudrate * 16) as u32).ok();
// }

fn calc_divider_safe(clock: u32, baudrate: u32) -> u32 {
    //return u16::try_from(clock / (baudrate * 16) as u32).ok();
    clock / (baudrate * 16)
}

// Macro for setting up a serial device other then UART.
