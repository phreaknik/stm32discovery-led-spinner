#![feature(const_fn)]
#![feature(used)]
#![no_std]

// version = "0.2.9"
#[macro_use]
extern crate cortex_m;

// version = "0.2.3"
extern crate cortex_m_rt;

// version = "0.1.0"
#[macro_use]
extern crate cortex_m_rtfm as rtfm;

// version = "0.4.1"
extern crate f3;

use f3::stm32f30x;
use rtfm::{P0, P1, T0, T1, TMax};
use stm32f30x::interrupt::Exti0;

fn init(_prio: P0, _thr: &TMax) {}

fn idle(_prio: P0, _thr: T0) -> ! {
    rtfm::request(t1);

    // Sleep
    loop {
        rtfm::wfi();
    }
}

tasks!(stm32f30x, {
    t1: Task {
        interrupt: Exti0,
        priority: P1,
        enabled: true,
    },
});

fn t1(_task: Exti0, _prio: P1, _thr: T1) {
    rtfm::bkpt();

    hprintln!("The quick brown fox jumps over the lazy dog.");

    rtfm::bkpt();
}