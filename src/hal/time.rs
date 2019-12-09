use nb;
use core::{num::Wrapping};
use cortex_m::peripheral::{syst::SystClkSource, SYST};
//use crate::pmc::PMC;

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
    fn delay_us(&mut self, delay: u32) -> MillisCountDown<Time>;
    fn delay_ms(&mut self, delay: u32) -> MillisCountDown<Time>;
    fn delay_s(&mut self, delay: u32) -> MillisCountDown<Time>;
}

/// Trait that abstracts a counter that increases as milliseconds go by.
///
/// Factored out to leave the door open for different SysTick counters, such as
/// counting via interrupts.
pub trait CountsMillis {
    /// Returns a value that must not increment faster than once per
    /// millisecond, and will wrap around.
    fn count(&mut self) -> Wrapping<u32>;
}

pub struct Time {
    //pub sys_countdown: cortex_m_systick_countdown::PollingSysTick,
    pub syst: SYST,
    pub ticks_per_ms: u32,
    pub counter: Wrapping<u32>
    //counter: cortex_m_systick_countdown::MillisCountDown,
}

pub struct MillisCountDown<'a, CM: CountsMillis> {
    counter: &'a mut CM,
    target_millis: Option<Wrapping<u32>>,
}

impl<'a, CM: CountsMillis> MillisCountDown<'a, CM>  {
    pub fn new(counter: &'a mut CM) -> Self {
        MillisCountDown {
            target_millis: None,
            counter,
        }
    }

    /// Underlying version of `CountDown`’s `start` that takes a `u32` of
    /// milliseconds rather than a `Duration`.
    ///
    /// Use this if you want to avoid the `u64`s in `Duration`.
    pub fn start_ms(&mut self, ms: u32) {
        self.target_millis = Some(self.counter.count() + Wrapping(ms));
    }

    /// Underlying implementation of `CountDown`’s `wait` that works directly on
    /// our underlying u32 ms values and can be used by any `CountDown` trait
    /// implementations.
    ///
    /// Calling this method before `start`, or after it has already returned
    /// `Ok` will panic.
    pub fn wait_ms(&mut self) -> nb::Result<(), ()> {
        // Rollover-safe duration check derived from:
        // https://playground.arduino.cc/Code/TimingRollover/
        if (self.counter.count() - self.target_millis.unwrap()).0 as i32 > 0 {
            self.target_millis.take();
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl BusyDelay for Time {
    fn busy_delay_ms(&mut self, delay: u32) {
        let mut counter = MillisCountDown::new(self);
        counter.start_ms(delay);
        nb::block!(counter.wait_ms()).unwrap();
    }

    fn busy_delay_us(&mut self, delay: u32) {
        self.busy_delay_ms(delay / 1000);
    }
    fn busy_delay_s(&mut self, delay: u32) {
        self.busy_delay_ms(delay * 100);
    }
}

impl Delay for Time {
    fn delay_ms(&mut self, delay: u32) -> MillisCountDown<Time> {
        let mut counter = MillisCountDown::new(self);

        counter.start_ms(delay);

        counter
    }

    fn delay_us(&mut self, delay: u32) -> MillisCountDown<Time> {
        self.delay_ms(delay / 1000)
    }
    fn delay_s(&mut self, delay: u32) -> MillisCountDown<Time> {
        self.delay_ms(delay * 100)
    }
}