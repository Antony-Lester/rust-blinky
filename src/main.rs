#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use cortex_m_rt::entry;
use stm32f4xx_hal::{self as hal };
use crate::hal::{pac, prelude::*};
use cortex_m::iprintln;

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(mut cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take().map(| cp| cp),
    ) {
        // Set up the LED's
        let gpioa = dp.GPIOA.split();
        let mut led1 = gpioa.pa6.into_push_pull_output();
        let mut led2 = gpioa.pa7.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = cp.SYST.delay(&clocks);

        // Initialize ITM
        let stim= &mut cp.ITM.stim[0];

        let mut count: i32 = 0;
        loop {
            //fizzBuzz
            if count % 3 == 0 && count % 5 == 0 {
                led1.set_high();
                led2.set_high();
                iprintln!(stim, "FizzBuzz");
            } else if count % 3 == 0 {
                led1.set_high();
                iprintln!(stim, "Fizz");
            } else if count % 5 == 0 {
                led2.set_high();
                iprintln!(stim, "Buzz");
            } else {
                iprintln!(stim, "{}", count);
            }

            delay.delay_ms(1000_u32);
            led1.set_low();
            led2.set_low();

            if count == i32::MAX {
                count = 0;
            } else {
                count += 1;
            }

        }
    }

    loop {}
}