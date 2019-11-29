use core::num::Wrapping;
use cortex_m::peripheral::syst;
use cortex_m_systick_countdown::*;
use sam3x8e as target;

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
    fn delay_us(&self, delay: u32) -> MillisCountDown<PollingSysTick>;
    fn delay_ms(&self, delay: u32) -> MillisCountDown<PollingSysTick>;
    fn delay_s(&self, delay: u32) -> MillisCountDown<PollingSysTick>;

    fn has_wrapped<'a>(&self, countdown: &'a mut MillisCountDown<PollingSysTick>) -> bool;
}

pub struct Time {
    sys_countdown: cortex_m_systick_countdown::PollingSysTick,
}

impl Time {
    pub fn new(syst: cortex_m::peripheral::SYST) -> Self {
        Time {
            sys_countdown: PollingSysTick::new(syst, &SysTickCalibration::built_in().unwrap()),
        }
    }

    pub fn get_ms(&self) -> Wrapping<u32> {
        self.sys_countdown.count()
    }
}

impl BusyDelay for Time {
    fn busy_delay_ms(&mut self, delay: u32) {
        let mut counter = cortex_m_systick_countdown::MillisCountDown::new(&self.sys_countdown);

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
    fn delay_ms(&self, delay: u32) -> MillisCountDown<PollingSysTick> {
        let mut counter = cortex_m_systick_countdown::MillisCountDown::new(&self.sys_countdown);

        counter.start_ms(delay);

        counter
    }

    fn delay_us(&self, delay: u32) -> MillisCountDown<PollingSysTick> {
        self.delay_ms(delay / 1000)
    }
    fn delay_s(&self, delay: u32) -> MillisCountDown<PollingSysTick> {
        self.delay_ms(delay * 100)
    }

    fn has_wrapped<'a>(&self, countdown: &'a mut MillisCountDown<PollingSysTick>) -> bool {
        match countdown.wait_ms() {
            Err(nb::Error::WouldBlock) => false,
            _ => true,
        }
    }
}
