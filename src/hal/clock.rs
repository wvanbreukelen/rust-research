//#![rustc_const_unstable]
// https://github.com/rust-lang/rust/issues/49146


pub struct PMCControl<PMC> {
    pub rf: Option<PMC>,
}

pub trait PMCConfigure<DEVICES> {
    fn set_hw_device(&mut self, devs: DEVICES);
}

pub trait PMCRead {
    fn get_master_clk(&self) -> u32;
    fn get_main_clock_frequency_hz(&self) -> u32;
}

pub trait PMCWrite<PERIPHERAL> {
    fn enable_peripheral(&self, p: PERIPHERAL);
    fn disable_peripheral(&self, p: PERIPHERAL);
}