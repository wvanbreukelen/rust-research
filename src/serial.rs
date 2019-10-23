pub use cortex_m::peripheral::syst;
pub use sam3x8e as target;

pub struct SerialHandle<HWUART> {
    hw_uart: HWUART,
    clock_divider: u32
}

pub trait SerialControl {
    fn disable();
}

pub trait SerialRead {
    fn read_byte(&self) -> char;
    fn is_available(&self) -> bool;
}

pub trait SerialWrite {
    fn write_byte(&mut self, b: char);
}

impl SerialControl for SerialHandle<target::UART> {
    fn disable() {

    }
}

impl SerialRead for SerialHandle<target::UART> {
    fn read_byte(&self) -> char {

    }

    fn is_available(&self) -> bool {
        false
    }
}

impl SerialWrite for SerialHandle<target::UART> {
    fn write_byte(&mut self, b: char) {}
}


pub fn create_serial_handle(_hw_uart: target::UART, pin_tx: Pin, pin_rx: Pin baudrate: u32) -> SerialHandle<target::UART> {
    // Maybe check the baudrate?

    // Setup the UART

    unsafe { sam3x8e::piob::codr::write(1 << 27) }

    // Return a new instance
    return SerialHandle { hw_uart: _hw_uart, clock_divider: 5241600 / baudrate};
}

// Macro for setting up a serial device other then UART.

