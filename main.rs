#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::*;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};

#[entry]
fn main() -> ! {

    let p = embassy_stm32::init(Default::default());
    let b = Input::new(p.PB0, Pull::Down);
    let mut led = Output::new(p.PC13, Level::High, Speed::Low);

    loop {
        if b.is_high() {
            info!("high");
            led.set_high();
        } else {
            info!("low");
            led.set_low();
        }
    }
}