use core::{num::Wrapping};
use cortex_m::peripheral::{syst::SystClkSource, SYST};

use crate::hal::time::*;

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