pub use cortex_m::peripheral::syst;
pub use sam3x8e as target;

use target::generic;

// Trait objects with difference types, Rust doesn't have inheritance: https://doc.rust-lang.org/1.30.0/book/2018-edition/ch17-02-trait-objects.html

pub struct IsDisabled;
pub struct IsEnabled;
pub struct IsInput;
pub struct IsOutput;
pub struct Unknown;

pub struct Pin<'a, T, ENABLED, DIRECTION> {
    port: &'a T, // _CODR
    pin_mask: u32,
    enabled: ENABLED,
    direction: DIRECTION, // is output
                          // is input
                          // is bidirectional
}

pub trait Configuration<T, STATE, DIRECTION> {
    fn disable(self);

    fn as_output(&self) -> Pin<T, IsEnabled, IsOutput>;
    fn as_input(&self) -> Pin<T, IsEnabled, IsInput>;
    //fn enable_pullup();
    //fn disable_pullup();
}

pub trait Write {
    fn set_state(&self, s: bool);
    fn set_high(&self);
    fn set_low(&self);
}

pub trait Read {
    fn get_state(&self) -> bool;
}

pub fn create<'a, T>(_port: &'a T, _pin_mask: u32) -> Pin<'a, T, IsDisabled, Unknown> {
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
        impl<ENABLED, DIRECTION> Configuration<target::$PIOX, ENABLED, DIRECTION>
            for Pin<'_, target::$PIOX, ENABLED, DIRECTION>
        {
            fn disable(self) {
                self.port
                    .odr
                    .write_with_zero(|w| unsafe { w.bits(self.pin_mask) });
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

        impl Write for Pin<'_, target::$PIOX, IsEnabled, IsOutput> {
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
                    .codr
                    .write_with_zero(|w| unsafe { w.bits(self.pin_mask) });
            }

            fn set_low(&self) {
                self.port
                    .codr
                    .write_with_zero(|w| unsafe { w.bits(self.pin_mask) });
            }
        }
    };
}

add_control_pio!(PIOA);
add_control_pio!(PIOB);
add_control_pio!(PIOC);
add_control_pio!(PIOD);
