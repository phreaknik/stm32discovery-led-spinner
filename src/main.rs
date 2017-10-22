#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

extern crate f3;
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;

use cortex_m::peripheral::SystClkSource;
use f3::led::{self, LEDS};
use f3::prelude::*;
use f3::Serial;
use f3::serial::Event;
use f3::time::Hertz;
use rtfm::{app, Threshold};

const FREQUENCY: u32 = 4; // Hz
const BAUD_RATE: Hertz = Hertz(115_200);

// TASKS & RESOURCES
app! {
    device: f3::stm32f30x,

    resources: {
        static ON: bool = false;
    },

    tasks: {
        SYS_TICK: {
            path: toggle,
            resources: [ON],
        },
        USART1_EXTI25: {
            path: loopback,
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
    **r.ON = !**r.ON;

    if **r.ON {
        LEDS[0].on();
    } else {
        LEDS[0].off();
    }
}

// Send back the received byte
fn loopback(_t: &mut Threshold, r: USART1_EXTI25::Resources) {
    let serial = Serial(&**r.USART1);

    let byte = serial.read().unwrap();
    serial.write(byte).unwrap();
}