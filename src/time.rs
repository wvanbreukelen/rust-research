pub use cortex_m::peripheral::syst;
pub use sam3x8e as target;

//use cortex_m::peripheral::syst;
//pub use sam3x8e as target;

// https://doc.rust-lang.org/stable/rust-by-example/trait.html
// https://github.com/stm32-rs/stm32f4xx-hal/blob/master/src/timer.rs
// https://stackoverflow.com/questions/24047686/default-function-arguments-in-rust
pub trait BusyDelay {
    fn busy_delay_us(&mut self, delay: u32);
    fn busy_delay_ms(&mut self, delay: u32);
    fn busy_delay_s(&mut self, delay: u32);
}

pub trait Delay {
    fn delay_us(&mut self, delay: u32);
    fn delay_ms(&mut self, delay: u32);
    fn delay_s(&mut self, delay: u32);

    fn has_wrapped(&mut self) -> bool;
}

pub struct Time<HWTIMER> {
    hw_timer: HWTIMER
}

impl Time<cortex_m::peripheral::SYST> {
    pub fn syst(mut syst: cortex_m::peripheral::SYST) -> Self { // target::SYST
        syst.set_clock_source(syst::SystClkSource::Core);
        syst.enable_counter();
        return Time { hw_timer: syst };
    }

    pub fn now_ticks() -> u32 { // Suppress warning by: https://www.reddit.com/r/rust/comments/8xs2it/warning_item_is_never_used_when_making_a_library/
        return cortex_m::peripheral::SYST::get_current();
    }

    pub fn _now_us() {}
    pub fn _now_ms() {}
}

impl BusyDelay for Time<cortex_m::peripheral::SYST> {
    fn busy_delay_ms(&mut self, delay: u32) {
        self.hw_timer.clear_current();
        self.hw_timer.set_reload(delay);
        
        while !self.hw_timer.has_wrapped() {} // Not pure...
    }

    fn busy_delay_us(&mut self, _delay: u32)  {}
    fn busy_delay_s(&mut self, _delay: u32) {}
}

impl Delay for Time<cortex_m::peripheral::SYST> {
    fn delay_ms(&mut self, delay: u32) {
        self.hw_timer.clear_current();
        self.hw_timer.set_reload(delay);
    }

    fn delay_us(&mut self, _delay: u32)  {}
    fn delay_s(&mut self, _delay: u32) {}

    fn has_wrapped(&mut self) -> bool {
        return self.hw_timer.has_wrapped();
    }
}