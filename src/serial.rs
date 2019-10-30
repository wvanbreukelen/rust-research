pub use cortex_m::peripheral::syst;
pub use sam3x8e as target;

use crate::pin;

//mod pin;

pub struct Serial {
    handle: target::UART,
    clock_div: u32
}

pub trait Configure {
    fn disable();
}

pub trait Read {
    fn read_byte(&self) -> char;
    //fn read_string() -> str;
}

pub trait Write {
    fn write_byte(&self, b: char);
}

// Pin<ENABLED, DIRECTION, VALID>
//pub fn create_serial_handle<'a>(_handle: target::UART, pin_tx: pin::Pin<pin::IsEnabled, pin::IsOutput, pin::IsValid>, pin_rx: pin::Pin<pin::IsEnabled, pin::IsInput, pin::IsValid>, baudrate: u32) -> Serial {
pub fn create_serial_handle(_handle: target::UART, baudrate: u32) -> Serial {
    // Claim the required pins.

    // Set pins to right mode.

    // Setup the UART.

    //unsafe { sam3x8e::piob::codr::write(1 << 27) }

    // Return a new instance
    return Serial { handle: _handle, clock_div: (5241600 / baudrate) };
}

// Macro for setting up a serial device other then UART.

