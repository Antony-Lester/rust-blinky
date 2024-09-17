#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use cortex_m_rt::entry;
use stm32f4xx_hal::{self as hal};

use crate::hal::{pac, prelude::*};


#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        
        let gpioa = dp.GPIOA.split();
        let mut led1 = gpioa.pa6.into_push_pull_output();
        let mut led2 = gpioa.pa7.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = cp.SYST.delay(&clocks);

        loop {
            // Toggle LED1 on and LED2 off
            led1.set_high();
            led2.set_low();
            delay.delay_ms(1000_u32);

            // Toggle LED1 off and LED2 on
            led1.set_low();
            led2.set_high();
            delay.delay_ms(1000_u32);
        }
    }

    loop {}
}