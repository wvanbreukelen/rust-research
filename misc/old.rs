  
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