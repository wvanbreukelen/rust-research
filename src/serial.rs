pub use cortex_m::peripheral::syst;
pub use sam3x8e as target;

use crate::pin;

//mod pin;

pub struct Serial {
    handle: target::UART,
    clock_div: u32,
}

impl Serial {
    pub fn disable(&self) {}

    pub fn begin(&self) {
        self.handle.cr.write_with_zero(|w| unsafe {
            w.rstrx().set_bit();
            w.rsttx().set_bit();
            w.rxdis().set_bit();
            w.txdis().set_bit()
        });

        self.handle
            .brgr
            .write_with_zero(|w| unsafe { w.bits(clock_div) });

        // Disable parity bits.
        self.handle.mr.write_with_zero(|w| unsafe { w.par().no() });
    }
}

// Pin<ENABLED, DIRECTION, VALID>
//pub fn create_serial_handle<'a>(_handle: target::UART, pin_tx: pin::Pin<pin::IsEnabled, pin::IsOutput, pin::IsValid>, pin_rx: pin::Pin<pin::IsEnabled, pin::IsInput, pin::IsValid>, baudrate: u32) -> Serial {
// Pin: IsDisabled, Unknown, IsValid

// Disadvantage of Rust. We need to be explicit abount all generic parameters. We cannot perform function overloading.
pub fn create<P1, P2>(
    _handle: target::UART,
    baudrate: u32,
    pin_tx: pin::Pin<P1, pin::IsDisabled, pin::Unknown, pin::IsValid>,
    pin_rx: pin::Pin<P2, pin::IsDisabled, pin::Unknown, pin::IsValid>,
) -> Serial {
    // Set pins to right mode.

    // Setup the UART.

    //unsafe { sam3x8e::piob::codr::write(1 << 27) }

    // Return a new instance
    return Serial {
        handle: _handle,
        clock_div: (5241600 / baudrate),
    };
}

// Macro for setting up a serial device other then UART.
