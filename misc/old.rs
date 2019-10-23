  
#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m::peripheral::{syst, Peripherals};
extern crate sam3x8e;
//use sam3x8e::{rtt, piob};

pub fn delay_ms(_x: u32) {
    // unsafe {
    //     let y = (*rtt()).vr.read() + x;
    //     // FIXME: deal with overflow!
    //     while (*rtt()).vr.read() < y {}
    // }
}

// Help: https://rust-embedded.github.io/book/start/registers.html
#[entry]
fn main() -> ! {
    //unsafe { (*piob()).per.write(1 << 27) }
    //unsafe { (*piob()).oer.write(1 << 27) }
    //unsafe { (*rtt()).mr.write(32768 / 1000) }
    let mut on = true;

    // Kill the watchdog.
    //wdt.wdt = wdt.mr.watchdog;
    //wdt.wdt = wdt.mr.watchdog;

    //sam3x8e::WDT::st = 0;

    let core_perip = Peripherals::take().unwrap();
    let spec_perip = sam3x8e::Peripherals::take().unwrap();

    
    let piob = spec_perip.PIOB;
    let pmc = spec_perip.PMC;



    let watchdog = spec_perip.WDT;
    //let mut systick = core_perip.SYST;
    let mut systick = core_perip.SYST;

    pmc.pmc_pcer0.write_with_zero(|w| unsafe { w.bits(0x3F << 11)});


    // Disable the watchdog.
    watchdog.mr.write(|w| w.wddis().set_bit() );

    
    // Enable output to p27
    piob.oer.write_with_zero(|w| w.p27().set_bit());


    // Enable interrupt handling or so...
    

    //systick.disable_interrupt();
    systick.set_clock_source(syst::SystClkSource::Core);
    systick.set_reload(8_400_000);
    //systick.clear_current();
    systick.enable_counter();
    //systick.enable_interrupt();

    loop {
        while !systick.has_wrapped() {}

        if on {
            piob.sodr.write_with_zero(|w| w.p27().set_bit() );
        } else {
            piob.codr.write_with_zero(|w| w.p27().set_bit() );
        }
        
        on = !on;
    }

    //if systick.is_counter_enabled() {
        // Set
        //piob.sodr.write_with_zero(|w| w.p27().set_bit() );
    //} else {
        // Clear led
        //piob.codr.write_with_zero(|w| w.p27().set_bit() );
    //}

    //while !systick.has_wrapped() {
        // Loop
    //}

    //piob.codr.write_with_zero(|w| w.p27().set_bit() );
    
    //loop {}

    // Raw bit writing: periph.reg.write(|w| unsafe { w.bits(rawbits) });

    //piob.sodr.write_with_zero(|w| unsafe { w.bits(1 << 27) });
    //piob.codr.write_with_zero(|w| unsafe { w.bits(1 << 27) });

    //loop  {
        //hprintln!("Not yet.").unwrap();
        //piob.sodr.write_with_zero(|w| w.p27().set_bit() );
        // Loop
        //if systick.has_wrapped() {
            //systick.clear_current();
            //piob.codr.write_with_zero(|w| unsafe { w.bits(1 << 27) });
            //if on {
                //piob.codr.write_with_zero(|w| w.p27().set_bit() );
            //} else {
                //piob.sodr.write_with_zero(|w| w.p27().set_bit() );
            //}
            

            //on = !on;

            

            //nsafe { (*sam3x8e::piob()).codr.write(1 << 27) }
        //} else {
            //piob.sodr.write_with_zero(|w| w.p27().set_bit() );
            //piob.sodr.write_with_zero(|w| unsafe { w.bits(1 << 27) });
            //unsafe { (*sam3x8e::piob()).sodr.write(1 << 27) }
        //}
        
    //}


    // Read the watchdog.
    //let test = watchdog.mr.read().wddis().bit_is_set();
    

    //et mut watchdog = peripherals.
    // Kill the watchdog.
    
    //watchdog.mr = watchdog.wddis;
}

pub use cortex_m::peripheral::syst;
pub use sam3x8e as target;

use target::generic;


pub struct Pin<'a, T> {
    port: &'a T, // _CODR
    pin_mask: u32,
    // is output
    // is input
    // is bidirectional
}

pub trait Control {
    fn enable(&self);
    //fn disable(&self);
    //fn get_state(&self) -> bool;
    fn set_state(&self, s: bool);
    //fn enable_pullup();
    //fn disable_pullup();
    //fn set_high();
    //fn set_low();
}


pub fn create<'a, T>(_port: &'a T, _pin_mask: u32) -> Pin<T> {
    return Pin {port: _port, pin_mask: _pin_mask};
}

impl Control for Pin<'_, target::PIOA> {
    fn enable(&self) {
        self.port.oer.write_with_zero(|w| unsafe { w.bits(self.pin_mask)});
    }

    fn set_state(&self, s: bool) {
        if s {
            self.port.codr.write_with_zero(|w| unsafe { w.bits(self.pin_mask)});
        } else {
            self.port.codr.write_with_zero(|w| unsafe { w.bits(self.pin_mask)});
        }
    }
}

impl Control for Pin<'_, target::PIOB> {
   fn enable(&self) {
        self.port.oer.write_with_zero(|w| unsafe { w.bits(self.pin_mask)});
    }

    fn set_state(&self, s: bool) {
        if s {
            self.port.codr.write_with_zero(|w| unsafe { w.bits(self.pin_mask)});
        } else {
            self.port.sodr.write_with_zero(|w| unsafe { w.bits(self.pin_mask)});
        }
    }
}

impl Control for Pin<'_, target::PIOC> {
   fn enable(&self) {
        self.port.oer.write_with_zero(|w| unsafe { w.bits(self.pin_mask)});
    }

    fn set_state(&self, s: bool) {
        if s {
            self.port.codr.write_with_zero(|w| unsafe { w.bits(self.pin_mask)});
        } else {
            self.port.sodr.write_with_zero(|w| unsafe { w.bits(self.pin_mask)});
        }
    }
}

impl Control for Pintarget::PIOD> {
    fn enable(&self) {
        self.port.oer.write_with_zero(|w| unsafe { w.bits(self.pin_mask)});
    }

    fn set_state(&self, s: bool) {
        if s {
            self.port.codr.write_with_zero(|w| unsafe { w.bits(self.pin_mask)});
        } else {
            self.port.sodr.write_with_zero(|w| unsafe { w.bits(self.pin_mask)});
        }
    }
}




// Macro for PIOA, PIOB, PIOC, PIOD generation
macro_rules! create_pin {
    ($PORT:ident) =>
        {
            
        }
}

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

pub trait Configuration<T, STATE, DIRECTION> {
    //fn enable(&self);
    fn disable(&self);

    fn set_as_output(&self) -> Pin<T, IsEnabled, IsOutput>;
    fn set_as_input(&self) -> Pin<T, IsEnabled, IsInput>;
    
    //fn enable_pullup();
    //fn disable_pullup();
}
    

pub trait Write {
    fn set_state(&self, s: bool);
    //fn set_high();
}

pub trait Read {
    //fn get_state(&self) -> bool;
    //fn set_low();
}


pub fn create<'a, T>(_port: &'a T, _pin_mask: u32) -> Pin<T, IsDisabled, Unknown> {
    return Pin {port: _port, pin_mask: _pin_mask, direction: Unknown, enabled: IsDisabled};
}



// Macro for PIOA, PIOB, PIOC, PIOD generation
macro_rules! add_control_pio {
    ($PIOX:ident) =>
    {
        fn shallow_copy_config<'a, ENABLED, DIRECTION>(_port: &'a target::$PIOX, _pin_mask: u32, _enabled: ENABLED, _direction: DIRECTION) -> Pin<'a, target::$PIOX, ENABLED, DIRECTION> {
            return Pin {port: _port, pin_mask: _pin_mask, direction: _direction, enabled: _enabled};
        }

        impl<T, ENABLED, DIRECTION> Configuration<T, ENABLED, DIRECTION> for Pin<'_, target::$PIOX, ENABLED, DIRECTION> {
            fn disable(&self) {
                self.port.odr.write_with_zero(|w| unsafe { w.bits(self.pin_mask)});
            }

            fn set_as_output(&self) {
                self.port.oer.write_with_zero(|w| unsafe { w.bits(self.pin_mask)});

                //return Pin {port: self.port, pin_mask: self.pin_mask, direction: IsOutput, enabled: IsEnabled};
                //return shallow_copy_config(self.port, self.pin_mask, IsEnabled, IsOutput);
            }

            fn set_as_input(&self) {
                self.port.ier.write_with_zero(|w| unsafe { w.bits(self.pin_mask)});

                //return shallow_copy_config(self.port, self.pin_mask, IsEnabled, IsInput);
            }
        }

        impl Write for Pin<'_, target::$PIOX, IsEnabled, IsOutput> {
            fn set_state(&self, s: bool) {

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