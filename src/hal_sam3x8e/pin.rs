use sam3x8e;

use crate::hal::pin::*;
use crate::hal_sam3x8e::core::*;

// Macro for PIOA, PIOB, PIOC, PIOD generation
macro_rules! add_control_pio {
    ($TARGET:ident, $PIOX:ident) => {
        impl<'a, ENABLED, DIRECTION> PinConfigure<sam3x8e::$PIOX, ENABLED, DIRECTION>
            for Pin<'a, $TARGET::$PIOX, ENABLED, DIRECTION>
        {
            fn disable(&self) -> Pin<$TARGET::$PIOX, IsDisabled, Unknown> {
                self.port
                    .odr
                    .write_with_zero(|w| unsafe { w.bits(1 << self.port_offset) });

                Pin {
                    port: self.port,
                    port_offset: self.port_offset,
                    direction: Unknown,
                    state: IsDisabled,
                }
            }

            fn as_output(&self) -> Pin<$TARGET::$PIOX, IsEnabled, IsOutput> {
                self.port
                    .oer
                    .write_with_zero(|w| unsafe { w.bits(1 << self.port_offset) });

                Pin {
                    port: self.port,
                    port_offset: self.port_offset,
                    direction: IsOutput,
                    state: IsEnabled,
                }
            }
            // https://stackoverflow.com/questions/47759124/returning-a-generic-struct-from-new?rq=1
            fn as_input(&self) -> Pin<$TARGET::$PIOX, IsEnabled, IsInput> {
                self.port
                    .ier
                    .write_with_zero(|w| unsafe { w.bits(1 << self.port_offset) });

                Pin {
                    port: self.port,
                    port_offset: self.port_offset,
                    direction: IsInput,
                    state: IsEnabled,
                }
            }

            fn enable_pullup(&self) {
                self.port
                    .puer
                    .write_with_zero(|w| unsafe { w.bits(1 << self.port_offset) });
            }

            fn disable_pullup(&self) {
                self.port
                    .pudr
                    .write_with_zero(|w| unsafe { w.bits(1 << self.port_offset) });
            }
        }

        impl PinWrite for Pin<'_, $TARGET::$PIOX, IsEnabled, IsOutput> {
            fn set_high(&self) {
                self.port
                    .sodr
                    .write_with_zero(|w| unsafe { w.bits(1 << self.port_offset) });
            }

            fn set_low(&self) {
                self.port
                    .codr
                    .write_with_zero(|w| unsafe { w.bits(1 << self.port_offset) });
            }
        }

        impl PinRead for Pin<'_, $TARGET::$PIOX, IsEnabled, IsInput> {
            fn get_state(&self) -> bool {
                (self.port.pdsr.read().bits() & (1 << self.port_offset)) != 0
            }
        }
    };
}

add_control_pio!(sam3x8e, PIOA);
add_control_pio!(sam3x8e, PIOB);
add_control_pio!(sam3x8e, PIOC);
add_control_pio!(sam3x8e, PIOD);
