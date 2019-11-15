pub use cortex_m::peripheral::syst;
pub use sam3x8e as target;

use core::mem;

// Trait objects with difference types, Rust doesn't have inheritance: https://doc.rust-lang.org/1.30.0/book/2018-edition/ch17-02-trait-objects.html

pub struct IsDisabled;
pub struct IsEnabled;
pub struct IsInput;
pub struct IsOutput;
pub struct IsValid;
pub struct IsInvalid;
pub struct Unknown;

#[macro_use]

// Macro for PIOA, PIOB, PIOC, PIOD generation
// https://stackoverflow.com/questions/51932944/how-to-match-rusts-if-expressions-in-a-macro
pub struct Pin<'a, PORT, ENABLED, DIRECTION, VALID> {
    port: &'a PORT, // _CODR
    pin_mask: u32,
    enabled: ENABLED,
    direction: DIRECTION, // is output
    // is input
    // is bidirectional
    valid: VALID,
}

pub trait Configuration<PORT, STATE, DIRECTION, VALID> {
    fn disable(&self) -> Pin<PORT, IsDisabled, Unknown, IsValid>;

    fn as_output(&self) -> Pin<PORT, IsEnabled, IsOutput, IsValid>;
    fn as_input(&self) -> Pin<PORT, IsEnabled, IsInput, IsValid>;

    fn handoff(&self) -> Pin<PORT, IsDisabled, Unknown, IsInvalid>;
}

pub trait Writer {
    fn set_state(&self, s: bool);
    fn set_high(&self);
    fn set_low(&self);
    fn get_state(&self) -> bool;
}

pub fn create<'a, PORT>(
    _port: &'a PORT,
    _pin_mask: u32,
) -> Pin<'a, PORT, IsDisabled, Unknown, IsValid> {
    return Pin {
        port: _port,
        pin_mask: _pin_mask,
        direction: Unknown,
        enabled: IsDisabled,
        valid: IsValid,
    };
}
// Macro for PIOA, PIOB, PIOC, PIOD generation
macro_rules! add_control_pio {
    ($PIOX:ident) => {
        impl<'a, ENABLED, DIRECTION, VALID> Configuration<target::$PIOX, ENABLED, DIRECTION, VALID>
            for Pin<'a, target::$PIOX, ENABLED, DIRECTION, VALID>
        {
            fn disable(&self) -> Pin<target::$PIOX, IsDisabled, Unknown, IsValid> {
                self.port
                    .odr
                    .write_with_zero(|w| unsafe { w.bits(self.pin_mask) });

                return Pin {
                    port: self.port,
                    pin_mask: self.pin_mask,
                    direction: Unknown,
                    enabled: IsDisabled,
                    valid: IsValid,
                };
            }

            fn handoff(&self) -> Pin<target::$PIOX, IsDisabled, Unknown, IsInvalid> {
                return Pin {
                    port: self.port,
                    pin_mask: self.pin_mask,
                    direction: Unknown,
                    enabled: IsDisabled,
                    valid: IsInvalid,
                };
            }

            fn as_output(&self) -> Pin<target::$PIOX, IsEnabled, IsOutput, IsValid> {
                self.port
                    .oer
                    .write_with_zero(|w| unsafe { w.bits(self.pin_mask) });

                return Pin {
                    port: self.port,
                    pin_mask: self.pin_mask,
                    direction: IsOutput,
                    enabled: IsEnabled,
                    valid: IsValid,
                };
            }
            // https://stackoverflow.com/questions/47759124/returning-a-generic-struct-from-new?rq=1
            fn as_input(&self) -> Pin<target::$PIOX, IsEnabled, IsInput, IsValid> {
                self.port
                    .ier
                    .write_with_zero(|w| unsafe { w.bits(self.pin_mask) });

                return Pin {
                    port: self.port,
                    pin_mask: self.pin_mask,
                    direction: IsInput,
                    enabled: IsEnabled,
                    valid: IsValid,
                };
            }
        }

        impl Writer for Pin<'_, target::$PIOX, IsEnabled, IsOutput, IsValid> {
            fn set_state(&self, s: bool) {
                if s {
                    self.port
                        .codr
                        .write_with_zero(|w| unsafe { w.bits(self.pin_mask) });
                } else {
                    self.port
                        .codr
                        .write_with_zero(|w| unsafe { w.bits(self.pin_mask) });
                }
            }

            fn set_high(&self) {
                self.port
                    .sodr
                    .write_with_zero(|w| unsafe { w.bits(self.pin_mask) });
            }

            fn set_low(&self) {
                self.port
                    .codr
                    .write_with_zero(|w| unsafe { w.bits(self.pin_mask) });
            }

            fn get_state(&self) -> bool {
                return true;
            }
        }
    };
}

add_control_pio!(PIOA);
add_control_pio!(PIOB);
add_control_pio!(PIOC);
add_control_pio!(PIOD);

//pub static pin13: Pin<IsDisabled, Unknown, IsValid, target::piob::RegisterBlock> =
//    create(1 << 27, target::PIOB::ptr());
