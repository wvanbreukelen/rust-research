#![no_std]
pub use cortex_m::peripheral::syst;
pub use sam3x8e as target;

// #[macro_use]
// // Source: https://github.com/stm32-rs/stm32f4xx-hal/blob/9dab1701bc68efe3a1df8eb3b93c866d7ef1fa0e/src/lib.rs
// #[cfg(not(feature = "device-selected"))]
// compile_error!("This crate requires one of the following device features enabled:
//         sam3x8e
//         stm....");

// Trait objects with difference types, Rust doesn't have inheritance: https://doc.rust-lang.org/1.30.0/book/2018-edition/ch17-02-trait-objects.html

pub struct IsDisabled;
pub struct IsEnabled;
pub struct IsInput;
pub struct IsOutput;
pub struct Unknown;

// Macro for PIOA, PIOB, PIOC, PIOD generation
// https://stackoverflow.com/questions/51932944/how-to-match-rusts-if-expressions-in-a-macro
pub struct Pin<'a, PORT, STATE, DIRECTION> {
    port: &'a PORT,
    pin_mask: u32,
    state: STATE,
    direction: DIRECTION, // is output
}

pub trait Configuration<PORT, STATE, DIRECTION> {
    fn disable(&self) -> Pin<PORT, IsDisabled, Unknown>;

    fn as_output(&self) -> Pin<PORT, IsEnabled, IsOutput>;
    fn as_input(&self) -> Pin<PORT, IsEnabled, IsInput>;
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
        state: IsDisabled,
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

                Pin {
                    port: self.port,
                    pin_mask: self.pin_mask,
                    direction: Unknown,
                    state: IsDisabled,
                }
            }

            fn as_output(&self) -> Pin<target::$PIOX, IsEnabled, IsOutput> {
                self.port
                    .oer
                    .write_with_zero(|w| unsafe { w.bits(self.pin_mask) });

                Pin {
                    port: self.port,
                    pin_mask: self.pin_mask,
                    direction: IsOutput,
                    state: IsEnabled,
                }
            }
            // https://stackoverflow.com/questions/47759124/returning-a-generic-struct-from-new?rq=1
            fn as_input(&self) -> Pin<target::$PIOX, IsEnabled, IsInput> {
                self.port
                    .ier
                    .write_with_zero(|w| unsafe { w.bits(self.pin_mask) });

                Pin {
                    port: self.port,
                    pin_mask: self.pin_mask,
                    direction: IsInput,
                    state: IsEnabled,
                }
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
                let cur_absr: u32 = self.port.absr.read().bits();
                self.port
                    .absr
                    .write_with_zero(|w| unsafe { w.bits(cur_absr & (!self.pin_mask)) });
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
                (self.port.pdsr.read().bits() & self.pin_mask) != 0
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
