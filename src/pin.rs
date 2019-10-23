#[macro_use]

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
    direction: DIRECTION

    // is output
    // is input
    // is bidirectional
}
impl<T, ENABLED, DIRECTION> Pin<T, ENABLED, DIRECTION> {
    // fn shallow_copy_config<'a, ENABLED, DIRECTION>(_port: &'a target::$PIOX, _pin_mask: u32, _enabled: ENABLED, _direction: DIRECTION) -> Pin<'a, target::$PIOX, ENABLED, DIRECTION> {
    //     return Pin {port: _port, pin_mask: _pin_mask, direction: _direction, enabled: _enabled};
    // }

    pub fn copy(_p: T, _pin_mask: u32, _enabled: ENABLED, _direction: DIRECTION) -> Self {
        Pin { port: _p, pin_mask: _pin_mask, enabled: _enabled, direction: _direction }
    }
}

pub trait Configuration<T, STATE, DIRECTION> {
    //fn enable(&self);
    fn disable(self);

    fn set_as_output(self) -> Pin<T, IsEnabled, IsOutput>;
    fn set_as_input(self) -> Pin<T, IsEnabled, IsInput>;
    //fn shallow_copy_config<'a, ENABLED, DIRECTION>(self, _enabled: ENABLED, _direction: DIRECTION) -> Pin<'a, T, ENABLED, DIRECTION>
    
    //fn enable_pullup();
    //fn disable_pullup();
}
    

pub trait Write {
    fn set_state(self, s: bool);
    //fn set_high();
}

pub trait Read {
    //fn get_state(&self) -> bool;
    //fn set_low();
}


pub fn create<T>(_port: &'a T, _pin_mask: u32) -> Pin<T, IsDisabled, Unknown> {
    return Pin {port: _port, pin_mask: _pin_mask, direction: Unknown, enabled: IsDisabled};
}



// Macro for PIOA, PIOB, PIOC, PIOD generation
macro_rules! add_control_pio {
    ($PIOX:ident) =>
    {

        impl<T, ENABLED, DIRECTION> Configuration<T, ENABLED, DIRECTION> for Pin<'_, target::$PIOX, ENABLED, DIRECTION> {            
            fn disable(self) {
                self.port.odr.write_with_zero(|w| unsafe { w.bits(self.pin_mask)});
            }

            fn set_as_output(self) -> Pin<'_, T, IsEnabled, IsOutput>{
                self.port.oer.write_with_zero(|w| unsafe { w.bits(self.pin_mask)});

                //return Pin {port: self.port, pin_mask: self.pin_mask, direction: IsOutput, enabled: IsEnabled};
                //return self.shallow_copy_config(self.port, self.pin_mask, IsEnabled, IsOutput);
            }

            fn set_as_input(self) {
                self.port.ier.write_with_zero(|w| unsafe { w.bits(self.pin_mask)});

                //return shallow_copy_config(self.port, self.pin_mask, IsEnabled, IsInput);
            }
        }

        impl Write for Pin<'_, target::$PIOX, IsEnabled, IsOutput> {
            fn set_state(self, s: bool) {

            }
        }

        // impl Configuration for Pin<'_, target::$PIOX, Enabled, Output> {
        //     fn set_state(&self, s: bool) {
        //         if s {
        //             self.port.codr.write_with_zero(|w| unsafe { w.bits(self.pin_mask)});
        //         } else {
        //             self.port.codr.write_with_zero(|w| unsafe { w.bits(self.pin_mask)});
        //         }
        //     }
        // }


    }

}


//add_control_pio!(PIOA);
add_control_pio!(PIOB);
//add_control_pio!(PIOC);
//add_control_pio!(PIOD);