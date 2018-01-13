#![feature(used, const_fn, asm, naked_functions, const_size_of)]
#![no_std]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;
extern crate bare_metal;
extern crate stm32f031x;

use core::u16;

use cortex_m::asm;
use stm32f031x::{GPIOA, RCC, TIM1};

pub use rtos::PENDSV;

mod rtos;

static mut TEST_STACK: [u8; 256] = [0; 256];

fn test() {
    asm::bkpt();
}

fn main() {
    rtos::add_task(test, unsafe{ &TEST_STACK[..] });

    cortex_m::interrupt::free(|cs| {
        let gpioa = GPIOA.borrow(cs);
        let rcc = RCC.borrow(cs);
        let tim1 = TIM1.borrow(cs);

        rcc.ahbenr.modify(|_, w| w.iopaen().bit(true));
        rcc.apb2enr.modify(|_, w| w.tim1en().bit(true));

        tim1.psc.write(|w| unsafe { w.psc().bits(0) });
        tim1.ccmr1_output.modify(|_, w| unsafe {
            w.oc1m().bits(0b110).oc1pe().bit(true)
        });
        tim1.ccer.modify(|_, w| w.cc1e().bit(true));
        tim1.bdtr.modify(|_, w| w.moe().bit(true));
        tim1.arr.write(|w| unsafe { w.arr().bits(8000) });
        tim1.ccr1.write(|w| unsafe { w.ccr1().bits(4000) });

        gpioa.afrh.modify(|_, w| unsafe { w.afrh8().bits(0b0010) });
        gpioa.moder.modify(|_, w| { w.moder8().output() });

        tim1.cr1.modify(|_, w| w.cen().bit(true));
        tim1.egr.write(|w| w.ug().bit(true));
        loop {}
    });
}

// As we are not using interrupts, we just register a dummy catch all handler
/*#[allow(dead_code)]
#[used]
#[link_section = ".vector_table.interrupts"]
static INTERRUPTS: [extern "C" fn(); 32] = [default_handler; 32];

extern "C" fn default_handler() {}*/

default_handler!(exception_handler);

fn exception_handler() {}
