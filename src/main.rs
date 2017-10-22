#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

extern crate f3;
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate cast;

use cortex_m::peripheral::SystClkSource;
use f3::led::{self, LEDS};
use f3::prelude::*;
use f3::Serial;
use f3::serial::Event;
use f3::time::Hertz;
use rtfm::{app, Threshold};
use cast::{usize};

const FREQUENCY: u32 = 16; // Hz
const BAUD_RATE: Hertz = Hertz(115_200);

// TASKS & RESOURCES
app! {
    device: f3::stm32f30x,

    resources: {
        static CURRENT_LED: u8 = 7;
    },

    tasks: {
        SYS_TICK: {
            path: toggle,
            priority: 1,
            resources: [CURRENT_LED],
        },
        USART1_EXTI25: {
            path: loopback,
            priority: 1,
            resources: [USART1],
        }
    }
}

// INITIALIZATION
fn init(p: init::Peripherals, _r: init::Resources) {
    led::init(p.GPIOE, p.RCC);

    p.SYST.set_clock_source(SystClkSource::Core);
    p.SYST.set_reload(8_000_000 / FREQUENCY);
    p.SYST.enable_interrupt();
    p.SYST.enable_counter();

    let serial = Serial(p.USART1);
    serial.init(BAUD_RATE.invert(), Some(p.DMA1), p.GPIOA, p.RCC);
    serial.listen(Event::Rxne);
}

// IDLE LOOP
fn idle() -> ! {
    // Sleep
    loop {
        rtfm::wfi();
    }
}

// TASKS
// Toggle the state of the LED
fn toggle(_t: &mut Threshold, r: SYS_TICK::Resources) {
    // Turn off old LED
    LEDS[usize(**r.CURRENT_LED)].off();

    // Update CURRENT_LED
    if **r.CURRENT_LED == 7 {
        **r.CURRENT_LED = 0;
    }
    else {
        **r.CURRENT_LED += 1;
    }

    // Turn on new LED
    LEDS[usize(**r.CURRENT_LED)].on();
}

// Send back the received byte
fn loopback(_t: &mut Threshold, r: USART1_EXTI25::Resources) {
    let serial = Serial(&**r.USART1);

    let byte = serial.read().unwrap();
    serial.write(byte).unwrap();
}