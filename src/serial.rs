//#![deny(warnings)]

pub use cortex_m::peripheral::syst;
pub use sam3x8e as target;
use nb;
use core::convert::TryFrom;
use core::u16;


use crate::pin;

//mod pin;

const MAINFRDY  : u32 = 0x00010000;
const MAINF_MASK: u32 = 0x0000ffff;
const SLOW_CLOCK_FREQUENCY_HZ: u32 = 32_768;


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
        pmc: &'a target::PMC,
        baudrate: u32,
        _pin_tx: pin::Pin<P1, pin::IsDisabled, pin::Unknown, pin::IsValid>,
        _pin_rx: pin::Pin<P2, pin::IsDisabled, pin::Unknown, pin::IsValid>,
    ) -> Self {
        // Costs runtime performance as the evaluation is upon compile-time.
        let div = calc_divider_safe(main_clock_frequency_hz(pmc), baudrate);

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
            pid11().set_bit().
            pid12().set_bit()); // Enable PIOA, PIOB
        // Set pins to right mode.
        pio.pdr.write_with_zero(|w| unsafe { w.bits(0x300) });

        pio.puer.write_with_zero(|w| w.p9().set_bit()); // Tx pin pullup
        pio.absr.write_with_zero(|w| unsafe {w.bits(!(0x1 << 9))}); // Select  peripheral a

        //pio.pdr.write_with_zero(|w| w.p8().set_bit());
        //let mut absrVal = pio.absr.read();
        //pio.absr.write_with_zero(|w| unsafe { w.bits(!(0x1 << 8)) }); // Does this really clear? w.p8().clear_bit()
        //absrVal = pio.absr.read();
        //pio.pdr.write_with_zero(|w| w.p9().set_bit());
        //pio.absr.write_with_zero(|w| unsafe { w.bits(absrVal.bits() & !(0x1 << 9)) });

        pmc.pmc_pcer0.write_with_zero(|w| w.pid8().set_bit()); // Enable UART clock

        // Disable UART
        // self.handle.cr.write_with_zero(|w|
        //     w.
        //     rstrx().set_bit().
        //     rsttx().set_bit().
        //     rxdis().set_bit().
        //     txdis().set_bit()
        // );
        self.handle.cr.write_with_zero(|w| unsafe { w.bits(0xAC) }); // Disable UART


        // Set the baudrate
        self.handle
           .brgr
           .write_with_zero(|w| unsafe { w.cd().bits(self.clock_div) });


        // Disable parity bits, go in normal mode.
        //self.handle.mr.write_with_zero(|w| w.par().no() );
        self.handle.mr.write_with_zero(|w| unsafe {w.bits(0x800)});



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

        //self.handle.idr.write_with_zero(|w| unsafe {w.bits(0xFFFFFFFF)}); // Disable interrupts.

        // Enable UART
        self.handle.cr.write_with_zero(|w|
            w.txen().set_bit().
            rxen().set_bit()
        );
        //self.handle.cr.write_with_zero(|w| unsafe {w.bits(0x20)});
        //let mut val = b'a' as u32;

        //self.handle.thr.write_with_zero(|w| unsafe {w.bits(val)});
        //self.handle.thr.write_with_zero(|w| unsafe { w.txchr().bits(0xFF) });
        //self.handle.thr.write_with_zero(|w| unsafe {w.bits(val)});
        //self.handle.thr.write_with_zero(|w| unsafe {w.bits(val)});
        //self.handle.thr.write_with_zero(|w| unsafe {w.bits(val)});

        //loop {}
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

    pub fn write_byte(&self, mut ch: u8) -> nb::Result<(), SerialError> {
        if !self.handle.sr.read().txrdy().bit_is_set() {
          return Err(nb::Error::Other(SerialError::TxNotReady)); // Or Error::WouldBlock
        }

        self.handle.thr.write_with_zero(|w| unsafe { w.txchr().bits(ch) });
        
        //self.handle.thr.write_with_zero(|w| unsafe {w.bits(0xFF as u32)});
        return Ok(())
    }

    pub fn read_byte(&self) -> nb::Result<u8, SerialError> {
        if !self.handle.sr.read().rxrdy().bit() {
            return Err(nb::Error::Other(SerialError::RxEmpty));
        }

        return Ok(self.handle.rhr.read().rxchr().bits());
    }
}

fn main_clock_frequency_hz<'a>(pmc: &'a target::PMC) -> u32 {
    let main_clock_frequency_within_16_slow_clock_cycles = unsafe {
        while pmc.ckgr_mcfr.read().bits() & MAINFRDY == 0 {}
        pmc.ckgr_mcfr.read().bits() & MAINF_MASK
    };

    main_clock_frequency_within_16_slow_clock_cycles
        * SLOW_CLOCK_FREQUENCY_HZ / 16
}

fn calc_divider_safe(clock: u32, baudrate: u32) -> Option<u16> {
    return u16::try_from(clock / (baudrate * 16) as u32).ok();
}

// Macro for setting up a serial device other then UART.
