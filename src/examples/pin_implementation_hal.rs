pub struct IsDisabled;
pub struct IsEnabled;
pub struct IsInput;
pub struct IsOutput;
pub struct Unknown;

pub struct Pin<STATE, DIRECTION> {
    pub state: STATE,
    pub direction: DIRECTION,
    pub pin_mask: u32
}

pub trait PinWrite {
    fn set_high(&self);
    fn set_low(&self);
}

pub trait PinRead {
    fn get_state(&self) -> bool;
}

impl<STATE, DIRECTION> Pin<STATE, DIRECTION> {
    fn to_input(&self) -> Pin<IsEnabled, IsInput> {
        // Enable pin and set to input.

        Pin {
            state: IsEnabled,
            direction: IsInput,
            pin_mask: self.pin_mask
        }
    }

    fn to_output(&self) -> Pin<IsEnabled, IsOutput> {
        // Enable pin and set to output.

        Pin {
            state: IsEnabled,
            direction: IsOutput,
            pin_mask: self.pin_mask
        }
    }
}

impl PinWrite for Pin<IsEnabled, IsOutput> {
    fn set_high(&self) {}
    fn set_low(&self) {}
}

impl PinRead for Pin<IsEnabled, IsInput> {
     fn get_state(&self) -> bool {
         false
     }
}

fn new_pin(_pin_mask: u32) -> Pin<IsDisabled, Unknown> {
    Pin { state: IsDisabled, direction: Unknown, pin_mask: _pin_mask }
}

