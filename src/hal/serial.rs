//#![deny(warnings)]

use crate::pin::Configuration;
use crate::pin::{InputPin, OutputPin};
use core::convert::TryFrom;
use core::mem::transmute;
use core::ops::*;
use core::u16;
pub use cortex_m::peripheral::syst;
use nb;
pub use sam3x8e as target;

use crate::peripherals;
use crate::pin;
use crate::pmc::PMC;
//mod pin;

pub struct Serial<'pins> {
    handle: target::UART,
    clock_div: u32,
    pin_tx: pin::Pin<'pins, target::PIOA, pin::IsEnabled, pin::IsOutput>,
    pin_rx: pin::Pin<'pins, target::PIOA, pin::IsEnabled, pin::IsInput>,
}

trait HWWriter {
    fn write_byte<'b>(&self, ch: u8) -> nb::Result<(), ()>;
    fn read_byte(&self) -> nb::Result<u8, ()>;
    fn disable(&self);
    fn enable(&self);
}

impl<'pins> Serial<'pins> {
    // Pin<ENABLED, DIRECTION, VALID>
    //pub fn create_serial_handle<'a>(_handle: target::UART, pin_tx: pin::Pin<pin::IsEnabled, pin::IsOutput, pin::IsValid>, pin_rx: pin::Pin<pin::IsEnabled, pin::IsInput, pin::IsValid>, baudrate: u32) -> Serial {
    // Pin: IsDisabled, Unknown, IsValid

    // Disadvantage of Rust. We need to be explicit about all generic parameters. We cannot perform function overloading.

    pub fn new(
        _handle: target::UART,
        baudrate: u32,
        _pin_tx: pin::Pin<'pins, target::PIOA, pin::IsEnabled, pin::IsOutput>,
        _pin_rx: pin::Pin<'pins, target::PIOA, pin::IsEnabled, pin::IsInput>,
    ) -> Self {
        // Return a new instance
        return Self {
            handle: _handle,
            //clock_div: calc_uart_divider(unsafe { PMC.get_main_clock_frequency_hz() }, baudrate),
            clock_div: calc_uart_divider(84_000_000, baudrate),
            pin_tx: _pin_tx,
            pin_rx: _pin_rx,
        };
    }

    pub fn begin<'b>(&self){
        // Set pins into right mode
        self.pin_tx.enable_pullup();
        self.pin_tx.switch_to_a();
        self.pin_rx.enable_pullup();
        self.pin_rx.switch_to_a();

        unsafe { PMC.enable_peripheral(peripherals::Peripheral::Uart) }; // Enable UART
        unsafe { PMC.enable_peripheral(peripherals::Peripheral::PioA) }; // Enable PIOA
        unsafe { PMC.enable_peripheral(peripherals::Peripheral::PioB) }; // Enable PIOA

        // Disable UART
        self.disable();

        // Disable parity bits, go in normal mode.
        self.handle.mr.write_with_zero(|w| w.par().no());

        // Disable interrupts.
        self.handle
            .idr
            .write_with_zero(|w| unsafe { w.bits(0xFFFFFFFF) }); // Disable interrupts.

        // Enable UART
        self.enable();
    }

    pub fn write_array_blocking(&self, arr: &[u8; 4]) {
        for x in 0..4 {
            self.write_byte_blocking(arr[x]);
        }
    }

    pub fn read_array_blocking(&self, arr: &mut [u8]) -> nb::Result<(), ()> {
        for ch in arr.iter_mut() {
            match self.read_byte_blocking() {
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

    pub fn write_int(&self, val: u32) {
        let mut bytes: [u8; 4];
        let minus = false;

        // if val < 0 {
        //     minus = true;
        //     val = -val;
        // }

        bytes = unsafe { transmute(val) };

        for byte in bytes.iter() {
            //if *byte != 0 {
            self.write_byte_blocking(*byte + '0' as u8);
            //}
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
}

impl HWWriter for Serial<'_> {
    fn enable(&self) {
        // Set the baudrate
        self.handle
            .brgr
            .write_with_zero(|w| unsafe { w.bits(self.clock_div) });

        // Enable UART
        self.handle
            .cr
            .write_with_zero(|w| w.txen().set_bit().rxen().set_bit());
    }

    fn disable(&self) {
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
    }

    fn write_byte<'b>(&self, ch: u8) -> nb::Result<(), ()> {
        if !self.handle.sr.read().txrdy().bit_is_set() {
            return Err(nb::Error::WouldBlock); // Or Error::WouldBlock
        }

        self.handle
            .thr
            .write_with_zero(|w| unsafe { w.txchr().bits(ch) });
        return Ok(());
    }

    fn read_byte(&self) -> nb::Result<u8, ()> {
        if !self.handle.sr.read().rxrdy().bit() {
            return Err(nb::Error::WouldBlock);
        }

        return Ok(self.handle.rhr.read().rxchr().bits());
    }
}

const fn calc_uart_divider(clock: u32, baudrate: u32) -> u32 {
    clock / (baudrate * 16)
}

// Macro for setting up a serial device other then UART.
