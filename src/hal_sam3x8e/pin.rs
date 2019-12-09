use sam3x8e;

use crate::hal::pin::*;

// Macro for PIOA, PIOB, PIOC, PIOD generation
macro_rules! add_control_pio {
    ($TARGET:ident, $PIOX:ident) => {
        impl<'a, ENABLED, DIRECTION> PinConfigure<sam3x8e::$PIOX, ENABLED, DIRECTION>
            for Pin<'a, $TARGET::$PIOX, ENABLED, DIRECTION>
        {
            fn disable(&self) -> Pin<$TARGET::$PIOX, IsDisabled, Unknown> {
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

            fn as_output(&self) -> Pin<$TARGET::$PIOX, IsEnabled, IsOutput> {
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
            fn as_input(&self) -> Pin<$TARGET::$PIOX, IsEnabled, IsInput> {
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

        impl PinWrite for Pin<'_, $TARGET::$PIOX, IsEnabled, IsOutput> {
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
        }

        impl PinRead for Pin<'_, $TARGET::$PIOX, IsEnabled, IsInput> {
            fn get_state(&self) -> bool {
                (self.port.pdsr.read().bits() & self.pin_mask) != 0
            }
        }
    };
}

add_control_pio!(sam3x8e, PIOA);
add_control_pio!(sam3x8e, PIOB);
add_control_pio!(sam3x8e, PIOC);
add_control_pio!(sam3x8e, PIOD);