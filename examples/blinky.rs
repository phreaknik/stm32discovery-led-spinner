#![feature(used)]
#![no_std]

// version = "0.2.4"
extern crate cortex_m;

// version = "0.2.0"
extern crate cortex_m_rt;

// version = "0.4.0"
extern crate f3;

use cortex_m::asm;
use f3::led::{self, LEDS};
use f3::stm32f30x::{GPIOE, RCC, TIM7};
use f3::timer::Timer;

/// Timer frequency
const FREQUENCY: u32 = 1;

#[inline(never)]
fn main() {
    // Critical section
    cortex_m::interrupt::free(
        |cs| {
            // Exclusive access to the peripherals
            let gpioe = GPIOE.borrow(cs);
            let rcc = RCC.borrow(cs);
            let tim7 = TIM7.borrow(cs);

            // Configure the PEx pins as output pins
            led::init(gpioe, rcc);

            // Configure TIM7 for periodic timeouts
            let timer = Timer(tim7);
            timer.init(rcc, FREQUENCY);

            // Start the timer
            timer.resume();

            let mut state = false;
            loop {
                // Wait for an update event *and* clear the update event flag
                while timer.clear_update_flag().is_err() {}

                // Toggle the state
                state = !state;

                // Blink the LED
                if state {
                    LEDS[0].on();
                } else {
                    LEDS[0].off();
                }
            }
        },
    );

}

// This part is the same as before
#[allow(dead_code)]
#[used]
#[link_section = ".rodata.interrupts"]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
