use crate::hal::pin;
use core::mem::transmute;
use core::num::Wrapping;

pub struct Serial<'pins, UART, PIN_PORT_TX, PIN_PORT_RX> {
    pub handle: UART,
    pub clock_div: u32,
    pub pin_tx: pin::Pin<'pins, PIN_PORT_TX, pin::IsEnabled, pin::IsOutput>,
    pub pin_rx: pin::Pin<'pins, PIN_PORT_RX, pin::IsEnabled, pin::IsInput>,
}

pub trait SerialConfigure<'pins, UART, PIN_PORT_TX, PIN_PORT_RX> {
    fn new(
        _handle: UART,
        baudrate: u32,
        _pin_tx: pin::Pin<'pins, PIN_PORT_TX, pin::IsEnabled, pin::IsOutput>,
        _pin_rx: pin::Pin<'pins, PIN_PORT_RX, pin::IsEnabled, pin::IsInput>,
    ) -> Self;
    fn begin(&self);
    fn disable(&self);
    fn enable(&self);
}

pub trait SerialRead {
    fn read_byte(&self) -> nb::Result<u8, ()>;

    fn read_array_blocking(&self, arr: &mut [u8]) -> nb::Result<(), ()> {
        for ch in arr.iter_mut() {
            match self.read_byte_blocking() {
                Some(ch_read) => *ch = ch_read,
                None => {}
            }
        }

        return Ok(());
    }

    fn read_byte_blocking(&self) -> Option<u8> {
        match nb::block!(self.read_byte()) {
            Ok(ch_read) => Some(ch_read),
            _ => None,
        }
    }
}

pub trait SerialWrite {
    fn write_byte<'b>(&self, ch: u8) -> nb::Result<(), ()>;

    fn add_digit(&self, c: Wrapping<u8>, hex_base: Wrapping<u8>) -> Wrapping<u8> {
        if c > Wrapping(9) {
            c += hex_base - Wrapping(10);
        } else {
            c += Wrapping('0' as u8);
        }

        c
    }

    fn write_int(&self, mut val: Wrapping<i32>) {
        let bytes: [u8; 4];
        let mut minus = false;

        if val < Wrapping(0) {
            minus = true;
            val *= Wrapping(-1);
        }

        let mut index = 0;

        if val == Wrapping(0) {
            self.write_byte_blocking(self.add_digit(Wrapping(x % ), 16).0 as u8);
        }

        while val > Wrapping(0) {

            //bytes[index] = (val % 9)
        }

        // bytes = unsafe { transmute(val) };

        // for byte in bytes.iter() {
        //     //if *byte != 0 {
        //     self.write_byte_blocking(*byte + '0' as u8);
        //     //}
        // }
    }

    fn write_array_blocking(&self, arr: &[u8; 4]) {
        for x in 0..4 {
            self.write_byte_blocking(arr[x]);
        }
    }

    fn write_str_blocking(&self, s: &str) {
        for ch in s.chars() {
            self.write_byte_blocking(ch as u8);
        }
    }

    fn write_byte_blocking(&self, ch: u8) {
        nb::block!(self.write_byte(ch)).unwrap();
    }
}
