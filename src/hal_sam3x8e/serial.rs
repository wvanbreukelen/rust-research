use sam3x8e;

use crate::hal::clock::*;
use crate::hal::pin::*;
use crate::hal::serial::*;
use crate::hal_sam3x8e::core::*;

impl<'pins> Serial<'pins, sam3x8e::UART, sam3x8e::PIOA, sam3x8e::PIOA> {
    // Pin<ENABLED, DIRECTION, VALID>
    //pub fn create_serial_handle<'a>(_handle: target::UART, pin_tx: pin::Pin<pin::IsEnabled, pin::IsOutput, pin::IsValid>, pin_rx: pin::Pin<pin::IsEnabled, pin::IsInput, pin::IsValid>, baudrate: u32) -> Serial {
    // Pin: IsDisabled, Unknown, IsValid

    // Disadvantage of Rust. We need to be explicit about all generic parameters. We cannot perform function overloading.
}

impl<'pins> SerialConfigure<'pins, sam3x8e::UART, sam3x8e::PIOA, sam3x8e::PIOA>
    for Serial<'pins, sam3x8e::UART, sam3x8e::PIOA, sam3x8e::PIOA>
{
    fn new(
        _handle: sam3x8e::UART,
        baudrate: u32,
        _pin_tx: Pin<'pins, sam3x8e::PIOA, IsEnabled, IsOutput>,
        _pin_rx: Pin<'pins, sam3x8e::PIOA, IsEnabled, IsInput>,
    ) -> Self {
        // Return a new instance
        return Self {
            handle: _handle,
            clock_div: calc_uart_divider(84_000_000, baudrate),
            pin_tx: _pin_tx,
            pin_rx: _pin_rx,
        };
    }

    fn begin<'b>(&self) {
        // Set pins into right mode
        self.pin_tx.enable_pullup();
        switch_to_a(&self.pin_tx);
        self.pin_rx.enable_pullup();
        switch_to_a(&self.pin_rx);

        unsafe { CLOCK.enable_peripheral(UART) }; // Enable UART
        unsafe { CLOCK.enable_peripheral(GPIOA) }; // Enable PIOA
        unsafe { CLOCK.enable_peripheral(GPIOB) }; // Enable PIOA

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
}

impl SerialWrite for Serial<'_, sam3x8e::UART, sam3x8e::PIOA, sam3x8e::PIOA> {
    fn write_byte<'b>(&self, ch: u8) -> nb::Result<(), ()> {
        if !self.handle.sr.read().txrdy().bit_is_set() {
            return Err(nb::Error::WouldBlock); // Or Error::WouldBlock
        }

        self.handle
            .thr
            .write_with_zero(|w| unsafe { w.txchr().bits(ch) });
        return Ok(());
    }
}

impl SerialRead for Serial<'_, sam3x8e::UART, sam3x8e::PIOA, sam3x8e::PIOA> {
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

fn switch_to_a<'a, MODE>(pin: &Pin<'a, sam3x8e::PIOA, IsEnabled, MODE>) {
    pin.port
        .pdr
        .write_with_zero(|w| unsafe { w.bits(1 << pin.port_offset) });
    let cur_absr = pin.port.absr.read().bits();
    pin.port
        .absr
        .write_with_zero(|w| unsafe { w.bits(cur_absr & (!(1 << pin.port_offset))) });
    // Not working...
}
