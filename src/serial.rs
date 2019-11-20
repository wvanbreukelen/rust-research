pub use cortex_m::peripheral::syst;
pub use sam3x8e as target;
use core::convert::TryFrom;

use core::u16;


use crate::pin;

//mod pin;

const SERIAL_BASE_CLOCK: u32 = 5241600;

pub struct Serial {
    handle: target::UART,
    clock_div: u16,
}

impl Serial {
    pub fn disable(&self) {}

    pub fn begin(&self) {
        // Disable UART
        self.handle.cr.write_with_zero(|w|
            w.rstrx().set_bit().rsttx().set_bit().rxdis().set_bit().txdis().set_bit()
        );

        // Set the baudrate
        self.handle
            .brgr
            .write_with_zero(|w| unsafe { w.cd().bits(self.clock_div) });

        // Disable parity bits.
        self.handle.mr.write_with_zero(|w| w.par().no() );

        // Disable interrupts.
        self.handle.idr.write_with_zero(|w| w
            .rxrdy().set_bit()
            .txrdy().set_bit()
            .endrx().set_bit()
            .endtx().set_bit()
            .ovre().set_bit()
            .frame().set_bit()
            .pare().set_bit()
            .txempty().set_bit()
            .txbufe().set_bit()
            .rxbuff().set_bit()
        );

        // Enable UART
        self.handle.cr.write_with_zero(|w|
            w.rxen().set_bit().txen().set_bit()
        );
    }

    //pub fn write(&self, char c) {
        //while (self.handle.sr // TXRDY: Transmitter ready should be set before sending anything!
    //}
}

// Pin<ENABLED, DIRECTION, VALID>
//pub fn create_serial_handle<'a>(_handle: target::UART, pin_tx: pin::Pin<pin::IsEnabled, pin::IsOutput, pin::IsValid>, pin_rx: pin::Pin<pin::IsEnabled, pin::IsInput, pin::IsValid>, baudrate: u32) -> Serial {
// Pin: IsDisabled, Unknown, IsValid

// Disadvantage of Rust. We need to be explicit about all generic parameters. We cannot perform function overloading.

pub fn create<'a, P1, P2>(
    _handle: target::UART,
    pio: &'a target::PIOA,
    baudrate: u32,
    pin_tx: pin::Pin<P1, pin::IsDisabled, pin::Unknown, pin::IsValid>,
    pin_rx: pin::Pin<P2, pin::IsDisabled, pin::Unknown, pin::IsValid>,
) -> Serial {
    // Set pins to right mode.
    pio.pdr.write_with_zero(|w| w.p8().set_bit());
    pio.absr.write_with_zero(|w| w.p8().set_bit());
    pio.pdr.write_with_zero(|w| w.p9().set_bit());
    pio.absr.write_with_zero(|w| w.p9().set_bit());

    // Setup the UART.

    //unsafe { sam3x8e::piob::codr::write(1 << 27) }

    // Costs runtime performance as the evaluation is upon compile-time.
    let div = calc_divider_safe(baudrate);
    assert!(div.is_none());

    // Return a new instance
    return Serial {
        handle: _handle,
        clock_div: div.unwrap()
    };
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
