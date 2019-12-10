#![no_std]
pub use cortex_m::peripheral::syst;
//pub use sam3x8e as target;

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
    pub port: &'a PORT,
    pub port_offset: u32,
    pub state: STATE,
    pub direction: DIRECTION, // is output
}

pub trait PinConfigure<PORT, STATE, DIRECTION> {
    fn disable(&self) -> Pin<PORT, IsDisabled, Unknown>;

    fn as_output(&self) -> Pin<PORT, IsEnabled, IsOutput>;
    fn as_input(&self) -> Pin<PORT, IsEnabled, IsInput>;

    fn enable_pullup(&self);
    fn disable_pullup(&self);
}

pub trait PinWrite {
    fn set_state(&self, s: bool) {
        if s {
            self.set_high();
        } else {
            self.set_low();
        }
    }

    fn set_high(&self);
    fn set_low(&self);
}

pub trait PinRead {
    fn get_state(&self) -> bool;

    fn is_low(&self) -> bool {
        !self.get_state()
    }

    fn is_high(&self) -> bool {
        self.get_state()
    }
}

pub fn create_pin<'a, PORT>(_port: &'a PORT, _port_offset: u32) -> Pin<'a, PORT, IsDisabled, Unknown> {
    return Pin {
        port: _port,
        port_offset: _port_offset,
        direction: Unknown,
        state: IsDisabled,
    };
}
