//#![deny(warnings)]

pub use cortex_m::peripheral::syst;
pub use sam3x8e as target;
use nb;
use core::convert::TryFrom;
use core::u16;


use crate::pin;

//mod pin;

const SERIAL_BASE_CLOCK: u32 = 5241600;

pub struct Serial {
    handle: target::UART,
    clock_div: u16,
}

pub enum SerialError {
    TxNotReady,
    RxEmpty,
    Timeout
}

impl Serial {
    // Pin<ENABLED, DIRECTION, VALID>
    //pub fn create_serial_handle<'a>(_handle: target::UART, pin_tx: pin::Pin<pin::IsEnabled, pin::IsOutput, pin::IsValid>, pin_rx: pin::Pin<pin::IsEnabled, pin::IsInput, pin::IsValid>, baudrate: u32) -> Serial {
    // Pin: IsDisabled, Unknown, IsValid

    // Disadvantage of Rust. We need to be explicit about all generic parameters. We cannot perform function overloading.

    pub fn new<'a, P1, P2>(
        _handle: target::UART,
        baudrate: u32,
        _pin_tx: pin::Pin<P1, pin::IsDisabled, pin::Unknown, pin::IsValid>,
        _pin_rx: pin::Pin<P2, pin::IsDisabled, pin::Unknown, pin::IsValid>,
    ) -> Self {
  

        // Setup the UART.

        //unsafe { sam3x8e::piob::codr::write(1 << 27) }

        // Costs runtime performance as the evaluation is upon compile-time.
        let div = calc_divider_safe(baudrate);
        //assert!(div.is_none());

        // Return a new instance
        return Self {
            handle: _handle,
            clock_div: div.unwrap()
        };
    }

    pub fn disable(&self) {}

    pub fn begin<'a>(&self, pio: &'a target::PIOA, pmc: &'a target::PMC) {
        pmc.pmc_pcer0
            .write_with_zero(|w| w.
            pid11().set_bit()); // Enable PIOA

        // Set pins to right mode.
        pio.pdr.write_with_zero(|w| w.p8().set_bit());
        pio.absr.write_with_zero(|w| w.p8().clear_bit());
        pio.pdr.write_with_zero(|w| w.p9().set_bit());
        pio.absr.write_with_zero(|w| w.p9().clear_bit());

        pmc.pmc_pcer0.write_with_zero(|w| w.pid8().set_bit()); // Enable UART

        // Disable UART
        self.handle.cr.write_with_zero(|w|
            w.
            rstrx().set_bit().
            rsttx().set_bit().
            rxdis().set_bit().
            txdis().set_bit()
        );


        // Set the baudrate
        // self.handle
        //    .brgr
        //    .write_with_zero(|w| unsafe { w.cd().bits(self.clock_div) });
        self.handle
           .brgr
           .write_with_zero(|w| unsafe { w.cd().bits(546) }); // 9600 baud


        // Disable parity bits.
        self.handle.mr.write_with_zero(|w| w.par().no() );



        // Disable interrupts.
        // self.handle.idr.write_with_zero(|w| w
        //     .rxrdy().set_bit()
        //     .txrdy().set_bit()
        //     .endrx().set_bit()
        //     .endtx().set_bit()
        //     .ovre().set_bit()
        //     .frame().set_bit()
        //     .pare().set_bit()
        //     .txempty().set_bit()
        //     .txbufe().set_bit()
        //     .rxbuff().set_bit()
        // );

        self.handle.idr.write_with_zero(|w| unsafe {w.bits(0xFFFFFFFF)}); // Disable interrupts.

        // Enable UART
        self.handle.cr.write_with_zero(|w|
            w.rxen().set_bit().txen().set_bit()
        );
    }

    pub fn write_blocking(&self, ch: u8) -> Result<(), SerialError> {
        return nb::block!(self.write_byte(ch));
    }

    pub fn read_blocking(&self) -> Result<u8, SerialError> {
        return nb::block!(self.read_byte());
    }

    pub fn write_array_blocking(&self, arr: &[u8]) -> nb::Result<(), SerialError> {
        for &ch in arr.iter() {
            match nb::block!(self.write_byte(ch)) {
                Err(e @ SerialError::Timeout) => return Err(nb::Error::Other(e)),
                Ok(()) => {},
                _ => {}
            }
        }

        return Ok(())
    }

    pub fn read_array_blocking(&self, arr: &mut [u8]) -> nb::Result<(), SerialError> {
        for ch in arr.iter_mut() {
            match nb::block!(self.read_byte()) {
                Err(e @ SerialError::Timeout) => return Err(nb::Error::Other(e)),
                Ok(ch_read) => *ch = ch_read,
                _ => {}
            }
        }

        return Ok(())
    }

    pub unsafe fn write_str_blocking(&self, mut ptr: *const u8)  {
        while *ptr != b'\0' { // Highly unsafe
            match nb::block!(self.write_byte(*ptr)) {
               Err(_e @ SerialError::Timeout) => return,
               _ => {}
            }
            ptr = ptr.add(1);
        }
    }

    pub fn write_byte(&self, ch: u8) -> nb::Result<(), SerialError> {
        if !self.handle.sr.read().txrdy().bit_is_set() {
          return Err(nb::Error::Other(SerialError::TxNotReady));
        }

        //self.handle.thr.write_with_zero(|w| unsafe { w.txchr().bits(ch) });
        self.handle.thr.write_with_zero(|w| unsafe {w.bits(ch as u32)});
        return Ok(())
    }

    fn read_byte(&self) -> nb::Result<u8, SerialError> {
        if !self.handle.sr.read().rxrdy().bit() {
            return Err(nb::Error::Other(SerialError::RxEmpty));
        }

        return Ok(self.handle.rhr.read().rxchr().bits());
    }
}



fn calc_divider_safe(baudrate: u32) -> Option<u16> {
    return u16::try_from(SERIAL_BASE_CLOCK / baudrate as u32).ok();
}

// const fn calc_divider_safe(baudrate: u16) -> Option<u16> {
//     //5241600 / baudrate as u16
//     //assert_eq!(5241600.checked_add(), baudrate);

//     return u16::try_from(5241600u32 / baudrate).ok();
//     //return 5241600u32 / baudrate;
// }

// Macro for setting up a serial device other then UART.
