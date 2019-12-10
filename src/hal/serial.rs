use crate::hal::pin;
use core::mem::transmute;

pub struct Serial<'pins, UART, PIN_PORT_TX, PIN_PORT_RX> {
    pub handle: UART,
    pub clock_div: u32,
    pub pin_tx: pin::Pin<'pins, PIN_PORT_TX, pin::IsEnabled, pin::IsOutput>,
    pub pin_rx: pin::Pin<'pins, PIN_PORT_RX, pin::IsEnabled, pin::IsInput>,
}

pub trait SerialConfigure {
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

    fn write_int(&self, val: u32) {
        let bytes: [u8; 4];
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
// Macro for setting up a serial device other then UART.
