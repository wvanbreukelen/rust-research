use stm32f407;

use crate::hal::pin::*;

// Macro for PIOA, PIOB, PIOC, PIOD generation
macro_rules! add_control_pio {
    ($TARGET:ident, $PIOX:ident) => {
        impl<'a, ENABLED, DIRECTION> PinConfigure<stm32f407::$PIOX, ENABLED, DIRECTION>
            for Pin<'a, $TARGET::$PIOX, ENABLED, DIRECTION>
        {
            fn disable(&self) -> Pin<$TARGET::$PIOX, IsDisabled, Unknown> {
                self.port
                    .odr
                    .write_with_zero(|w| unsafe { w.bits(self.port_offset) });

                Pin {
                    port: self.port,
                    port_offset: self.port_offset,
                    direction: Unknown,
                    state: IsDisabled,
                }
            }

            fn as_output(&self) -> Pin<$TARGET::$PIOX, IsEnabled, IsOutput> {
                let offset = 2 * self.port_offset;

                self.disable_pullup();

                self.port.otyper.modify(|r, w| unsafe {
                    w.bits(r.bits() & !(0x01 << self.port_offset))
                });

                self.port.moder.modify(|r, w| unsafe {
                    w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset))
                });


                Pin {
                    port: self.port,
                    port_offset: self.port_offset,
                    direction: IsOutput,
                    state: IsEnabled,
                }
            }
            // https://stackoverflow.com/questions/47759124/returning-a-generic-struct-from-new?rq=1
            fn as_input(&self) -> Pin<$TARGET::$PIOX, IsEnabled, IsInput> {
                let offset = 2 * self.port_offset;

                //self.disable_pullup();

                self.port.moder.modify(|r, w| unsafe {
                    w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset))
                });

                Pin {
                    port: self.port,
                    port_offset: self.port_offset,
                    direction: IsInput,
                    state: IsEnabled,
                }
            }

            fn enable_pullup(&self) {
                let offset = 2 * self.port_offset;

                self.port.pupdr.modify(|r, w| unsafe {
                    w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset))
                });
            }

            fn disable_pullup(&self) {
                let offset = 2 * self.port_offset;

                self.port.pupdr.modify(|r, w| unsafe {
                    w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset))
                });
            }
        }

        impl PinWrite for Pin<'_, $TARGET::$PIOX, IsEnabled, IsOutput> {
            fn set_high(&self) {
                // self.port
                //     .sodr
                //     .write_with_zero(|w| unsafe { w.bits(self.pin_mask) });

                self.port.bsrr.write(|w| unsafe { w.bits(1 << self.port_offset) });
            } 

            fn set_low(&self) {
                // self.port
                //     .codr
                //     .write_with_zero(|w| unsafe { w.bits(self.pin_mask) });
                self.port.bsrr.write(|w| unsafe { w.bits(1 << (self.port_offset + 16)) });
            }
        }

        impl PinRead for Pin<'_, $TARGET::$PIOX, IsEnabled, IsInput> {
            fn get_state(&self) -> bool {
                // (self.port.pdsr.read().bits() & self.pin_mask) != 0
                self.port.idr.read().bits() & (1 << self.port_offset) == 1
            }
        }
    };
}

add_control_pio!(stm32f407, GPIOA);
add_control_pio!(stm32f407, GPIOB);
add_control_pio!(stm32f407, GPIOC);
add_control_pio!(stm32f407, GPIOD);
add_control_pio!(stm32f407, GPIOE);
add_control_pio!(stm32f407, GPIOF);
add_control_pio!(stm32f407, GPIOG);
add_control_pio!(stm32f407, GPIOH);
add_control_pio!(stm32f407, GPIOI);