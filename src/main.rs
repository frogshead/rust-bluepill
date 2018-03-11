#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]

extern crate cortex_m;
extern crate stm32f103xx_hal as hal;
#[macro_use(block)]
extern crate nb;

use hal::prelude::*;
use hal::stm32f103xx;
use hal::timer::Timer;

fn main() {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f103xx::Peripherals::take().unwrap();

    // What we do with constrain
    let mut flash = dp.FLASH.constrain();
    // RCC = Reset and Clock Control
    let mut rcc = db.RCC.constrain();

    // Try with different clock configurations
    let mut clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioc = db.GPIOC.split(&mut rcc.apb2);

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let mut timer = Timer::syst(cp.SYST, 1.hz(), clocks);
    loop {
        block!(timer.wait()).unwrap();
        led.set_high();
        block!(timer.wait()).unwrap();
        led.set_low();
    }
}
