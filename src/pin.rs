pub use cortex_m::peripheral::syst;
pub use sam3x8e as target;

// Trait objects with difference types, Rust doesn't have inheritance: https://doc.rust-lang.org/1.30.0/book/2018-edition/ch17-02-trait-objects.html

pub struct IsDisabled;
pub struct IsEnabled;
pub struct IsInput;
pub struct IsOutput;
pub struct IsValid;
pub struct IsInvalid;
pub struct Unknown;

pub struct Pin<ENABLED, DIRECTION, VALID, PORT> {
    pin_mask: u32,
    enabled: ENABLED,
    direction: DIRECTION, // is output
                          // is input
                          // is bidirectional
    valid: VALID,
    port: *const PORT
}

pub trait Configuration<STATE, DIRECTION, VALID, PORT> {
    fn disable(&self) -> Pin<IsDisabled, Unknown, IsValid, PORT>;

    fn as_output(&self) -> Pin<IsEnabled, IsOutput, IsValid, PORT>;
    fn as_input(&self) -> Pin<IsEnabled, IsInput, IsValid, PORT>;

    fn handoff(&self) -> Pin<IsDisabled, Unknown, IsInvalid, PORT>;
}

pub trait Write {
    fn set_state(&self, s: bool);
    fn set_high(&self);
    fn set_low(&self);
}

pub trait Read {
    fn get_state(&self) -> bool;
}

pub fn create<PORT>(_pin_mask: u32, _port: *const PORT) -> Pin<IsDisabled, Unknown, IsValid, PORT> {
    return Pin {
        pin_mask: _pin_mask,
        direction: Unknown,
        enabled: IsDisabled,
        valid: IsValid,
        port: _port
    };
}
// Macro for PIOA, PIOB, PIOC, PIOD generation
macro_rules! add_control_pio {
    ($PIOX:ident) => {
        impl<ENABLED, DIRECTION, VALID> Configuration<ENABLED, DIRECTION, VALID, target::$PIOX::RegisterBlock>
            for Pin<ENABLED, DIRECTION, VALID, target::$PIOX::RegisterBlock>
        {
            fn disable(&self) -> Pin<IsDisabled, Unknown, IsValid, target::$PIOX::RegisterBlock> {
                unsafe { (*self.port)
                    .odr
                    .write_with_zero(|w| w.bits(self.pin_mask)) };

                return Pin {
                    pin_mask: self.pin_mask,
                    direction: Unknown,
                    enabled: IsDisabled,
                    valid: IsValid,
                    port: self.port
                };
            }

            fn handoff(&self) -> Pin<IsDisabled, Unknown, IsInvalid, target::$PIOX::RegisterBlock> {
                return Pin {
                    pin_mask: self.pin_mask,
                    direction: Unknown,
                    enabled: IsDisabled,
                    valid: IsInvalid,
                    port: self.port
                };
            }

            fn as_output(&self) -> Pin<IsEnabled, IsOutput, IsValid, target::$PIOX::RegisterBlock> {
                unsafe {
                (*self.port)
                    .oer
                    .write_with_zero(|w| w.bits(self.pin_mask)) };

                return Pin {
                    pin_mask: self.pin_mask,
                    direction: IsOutput,
                    enabled: IsEnabled,
                    valid: IsValid,
                    port: self.port
                };
            }
            // https://stackoverflow.com/questions/47759124/returning-a-generic-struct-from-new?rq=1
            fn as_input(&self) -> Pin<IsEnabled, IsInput, IsValid, target::$PIOX::RegisterBlock> {
                unsafe {
                (*self.port)
                    .ier
                    .write_with_zero(|w| w.bits(self.pin_mask)) };

                return Pin {
                    pin_mask: self.pin_mask,
                    direction: IsInput,
                    enabled: IsEnabled,
                    valid: IsValid,
                    port: self.port
                };
            }
        }

        impl Write for Pin<IsEnabled, IsOutput, IsValid, target::$PIOX::RegisterBlock> {
            fn set_state(&self, s: bool) {
                if s {
                    unsafe {
                    (*self.port)
                        .codr
                        .write_with_zero(|w| w.bits(self.pin_mask)) };
                } else {
                    unsafe {
                    (*self.port)
                        .codr
                        .write_with_zero(|w| w.bits(self.pin_mask)) };
                }
            }

            fn set_high(&self) {
                unsafe {
                (*self.port)
                    .sodr
                    .write_with_zero(|w| w.bits(self.pin_mask)) };
            }

            fn set_low(&self) {
                unsafe {
                (*self.port)
                    .codr
                    .write_with_zero(|w| w.bits(self.pin_mask)) };
            }
        }

        impl Read for Pin<IsEnabled, IsInput, IsValid, target::$PIOX::RegisterBlock> {
            fn get_state(&self) -> bool {
                // return ( port.PIO_PDSR & mask ) != 0;
                unsafe {   
                return ((*self.port).pdsr.read().bits() & self.pin_mask) != 0 };
            }
        }
    };
}

add_control_pio!(pioa);
add_control_pio!(piob);
add_control_pio!(pioc);
add_control_pio!(piod);