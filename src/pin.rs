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
#[derive(Copy, Clone)]
pub struct Pin<'a, PORT, ENABLED, DIRECTION> {
    port: &'a PORT, // _CODR
    pin_mask: u32,
    enabled: ENABLED,
    direction: DIRECTION, // is output
}

pub trait Configuration<PORT, STATE, DIRECTION> {
    fn disable(&self) -> Pin<PORT, IsDisabled, Unknown>;

    fn as_output(&self) -> Pin<PORT, IsEnabled, IsOutput>;
    fn as_input(&self) -> Pin<PORT, IsEnabled, IsInput>;

    fn handoff(&self) -> Pin<PORT, IsDisabled, Unknown>;
}

pub trait OutputPin {
    fn set_state(&self, s: bool);
    fn set_high(&self);
    fn set_low(&self);

    fn enable_pullup(&self);
    fn disable_pullup(&self);

    fn switch_to_a(&self);
}

pub trait InputPin {
    fn get_state(&self) -> bool;

    fn enable_pullup(&self);
    fn disable_pullup(&self);

    fn switch_to_a(&self);
}

pub fn create<'a, PORT>(_port: &'a PORT, _pin_mask: u32) -> Pin<'a, PORT, IsDisabled, Unknown> {
    return Pin {
        port: _port,
        pin_mask: _pin_mask,
        direction: Unknown,
        enabled: IsDisabled,
    };
}

// Macro for PIOA, PIOB, PIOC, PIOD generation
macro_rules! add_control_pio {
    ($PIOX:ident) => {
        impl<'a, ENABLED, DIRECTION> Configuration<target::$PIOX, ENABLED, DIRECTION>
            for Pin<'a, target::$PIOX, ENABLED, DIRECTION>
        {
            fn disable(&self) -> Pin<target::$PIOX, IsDisabled, Unknown> {
                self.port
                    .odr
                    .write_with_zero(|w| unsafe { w.bits(self.pin_mask) });

                return Pin {
                    port: self.port,
                    pin_mask: self.pin_mask,
                    direction: Unknown,
                    enabled: IsDisabled,
                };
            }

            fn handoff(&self) -> Pin<target::$PIOX, IsDisabled, Unknown> {
                return Pin {
                    port: self.port,
                    pin_mask: self.pin_mask,
                    direction: Unknown,
                    enabled: IsDisabled,
                };
            }

            fn as_output(&self) -> Pin<target::$PIOX, IsEnabled, IsOutput> {
                self.port
                    .oer
                    .write_with_zero(|w| unsafe { w.bits(self.pin_mask) });

                return Pin {
                    port: self.port,
                    pin_mask: self.pin_mask,
                    direction: IsOutput,
                    enabled: IsEnabled,
                };
            }
            // https://stackoverflow.com/questions/47759124/returning-a-generic-struct-from-new?rq=1
            fn as_input(&self) -> Pin<target::$PIOX, IsEnabled, IsInput> {
                self.port
                    .ier
                    .write_with_zero(|w| unsafe { w.bits(self.pin_mask) });

                return Pin {
                    port: self.port,
                    pin_mask: self.pin_mask,
                    direction: IsInput,
                    enabled: IsEnabled,
                };
            }
        }

        impl OutputPin for Pin<'_, target::$PIOX, IsEnabled, IsOutput> {
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

            fn enable_pullup(&self) {
                self.port
                    .puer
                    .write_with_zero(|w| unsafe { w.bits(self.pin_mask) });
            }

            fn disable_pullup(&self) {
                self.port
                    .pudr
                    .write_with_zero(|w| unsafe { w.bits(self.pin_mask) });
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

            fn switch_to_a(&self) {
                self.port
                    .pdr
                    .write_with_zero(|w| unsafe { w.bits(self.pin_mask) });
                let cur_absr = self.port.absr.read().bits();
                self.port
                    .absr
                    .write_with_zero(|w| unsafe { w.bits(cur_absr & (!self.pin_mask)) });
                // Not working...
            }
        }

        impl InputPin for Pin<'_, target::$PIOX, IsEnabled, IsInput> {
            fn switch_to_a(&self) {
                self.port
                    .pdr
                    .write_with_zero(|w| unsafe { w.bits(self.pin_mask) });
                let cur_absr = self.port.absr.read().bits();
                self.port
                    .absr
                    .write_with_zero(|w| unsafe { w.bits(cur_absr & (!self.pin_mask)) });
                // Not working...
            }

            fn get_state(&self) -> bool {
                return true;
            }

            fn enable_pullup(&self) {
                self.port
                    .puer
                    .write_with_zero(|w| unsafe { w.bits(self.pin_mask) });
            }

            fn disable_pullup(&self) {
                self.port
                    .pudr
                    .write_with_zero(|w| unsafe { w.bits(self.pin_mask) });
            }
        }
    };
}

add_control_pio!(PIOA);
add_control_pio!(PIOB);
add_control_pio!(PIOC);
add_control_pio!(PIOD);
