use core::{num::Wrapping, time::Duration};
use cortex_m::peripheral::{syst::SystClkSource, SYST};
use sam3x8e as target;
use nb;

use crate::pmc::PMC;

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
    syst: SYST,
    ticks_per_ms: u32,
    counter: Wrapping<u32>
    //counter: cortex_m_systick_countdown::MillisCountDown,
}

pub struct MillisCountDown<'a, CM: CountsMillis> {
    counter: &'a mut CM,
    target_millis: Option<Wrapping<u32>>,
}

impl Time {
    pub fn new(mut _syst: SYST) -> Self {
        let _ticks_per_ms: u32 = get_calib_ticks_10ms().unwrap() * 10; // Convert ticks per 10 ms to ticks per 1 ms

        _syst.disable_interrupt();
        _syst.set_clock_source(SystClkSource::External); // Important! Use external oscillator instead of the build-in one.
        _syst.enable_counter();
        _syst.set_reload(_ticks_per_ms);

        Time {
            syst: _syst,
            ticks_per_ms: _ticks_per_ms,
            counter: Wrapping(0)
            //sys_countdown: PollingSysTick::new(syst, &SysTickCalibration::built_in().unwrap()),
            //sys_countdown: PollingSysTick::new(syst, &SysTickCalibration::from_clock_hz(84_000_000))
            //counter: cortex_m_systick_countdown::MillisCountDown::new(&self.sys_countdown)
        }
    }

    pub fn get_ms(&mut self) -> Wrapping<u32> {
        self.count()
    }
}

impl CountsMillis for Time {
    fn count(&mut self) -> Wrapping<u32> {
        // This is all unsafe because incrementing the internal count happens as
        // a side effect of reading it. We’re ok with that, because we know that
        // we have sole control over the SYST singleton, so we’re the only ones
        // who will see the wrapping.
        if self.syst.has_wrapped() {
            // Disabled interrupts because += is non-atomic.
            cortex_m::interrupt::free(|_| {
                (self.counter) += Wrapping(1);
            });
        }

        self.counter
    }
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

fn get_calib_ticks_10ms() -> Option<u32> {
    let calibrated_tick_value = cortex_m::peripheral::SYST::get_ticks_per_10ms();

    if calibrated_tick_value == 0 {
        None
    } else {
        // Leave one clock cycle for checking the overflow
        // Source: https://github.com/fionawhim/cortex-m-systick-countdown/blob/develop/src/lib.rs
        Some((calibrated_tick_value + 1) / 10 - 1)
    }
}
