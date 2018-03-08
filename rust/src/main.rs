//! Firmware for the rt soldering iron

#![feature(used, const_fn, asm, naked_functions, const_size_of, const_unsafe_cell_new, const_ptr_null_mut, never_type)]
#![no_std]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;
extern crate bare_metal;
extern crate embedded_hal;
extern crate stm32f031x;
extern crate stm32f031x_hal;
extern crate nb;

use core::u16;

use cortex_m::asm;
use stm32f031x::{GPIOA, RCC, TIM1};
use stm32f031x_hal::rcc::RccExt;
use stm32f031x_hal::time::{U32Ext};
use stm32f031x_hal::flash::{FlashExt};
use stm32f031x_hal::gpio::GpioExt;

pub use board::{SYS_TICK, TIM1_BRK_UP_IRQ};
pub use debug::{HARD_FAULT, HARD_FAULT_STACK};

mod board;
mod debug;

static mut TEST_STACK: [u8; 256] = [0; 256];

fn test() {
    let mut core_peripherals = stm32f031x::CorePeripherals::take().unwrap();
    let mut peripherals = stm32f031x::Peripherals::take().unwrap();
    let mut nvic = core_peripherals.NVIC;
    let mut rcc = peripherals.RCC.constrain();
    let mut gpioa = peripherals.GPIOA.split(&mut rcc.ahb);
    let mut flash = peripherals.FLASH.constrain();
    let tim1 = peripherals.TIM1;

    rcc.cfgr = rcc.cfgr.sysclk(8.mhz())
        .hclk(8.mhz())
        .pclk(8.mhz());
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut buzzer = board::Buzzer::new(tim1, &mut rcc.apb2, &mut nvic, gpioa.pa8, &mut gpioa.regs);

    buzzer.beep(100, 1000.hz(), clocks.clone());
    loop { }
}

fn main() {
    test()
}

// As we are not using interrupts, we just register a dummy catch all handler
/*#[allow(dead_code)]
#[used]
#[link_section = ".vector_table.interrupts"]
static INTERRUPTS: [extern "C" fn(); 32] = [default_handler; 32];

extern "C" fn default_handler() {}*/

default_handler!(exception_handler);
fn exception_handler() {}
