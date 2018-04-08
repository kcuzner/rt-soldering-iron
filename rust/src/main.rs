//! Firmware for the rt soldering iron

#![feature(used, const_fn, asm, naked_functions, const_size_of, const_unsafe_cell_new, const_ptr_null_mut, never_type, generators, generator_trait)]
#![no_std]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;
extern crate bare_metal;
extern crate embedded_hal;
extern crate stm32f031x;
extern crate stm32f031x_hal;
extern crate board_support;
#[macro_use(await)]
extern crate nb;

use core::ops::Generator;

use cortex_m::asm;
use stm32f031x::{GPIOA, RCC, TIM1};
use stm32f031x_hal::rcc::RccExt;
use stm32f031x_hal::time::{U32Ext};
use stm32f031x_hal::flash::{FlashExt};
use stm32f031x_hal::gpio::GpioExt;
use stm32f031x_hal::i2c::{I2CExt, IntoScl, IntoSda, I2cTiming, I2cTimingSetting, MasterI2cError};

use embedded_hal::digital::OutputPin;

pub use board_support::{TIM1_BRK_UP_IRQ, SYS_TICK};
pub use debug::{HARD_FAULT, HARD_FAULT_STACK};

mod debug;

static mut TEST_STACK: [u8; 256] = [0; 256];
static CMD: [u8; 2] = [0, 0xae];

fn test() {
    let core_peripherals = stm32f031x::CorePeripherals::take().unwrap();
    let peripherals = stm32f031x::Peripherals::take().unwrap();
    let mut syst = core_peripherals.SYST;
    let mut nvic = core_peripherals.NVIC;
    let mut rcc = peripherals.RCC.constrain();
    let mut gpioa = peripherals.GPIOA.split(&mut rcc.ahb);
    let mut gpiob = peripherals.GPIOB.split(&mut rcc.ahb);
    let mut flash = peripherals.FLASH.constrain();
    let tim1 = peripherals.TIM1;

    rcc.cfgr = rcc.cfgr.sysclk(48.mhz())
        .hclk(48.mhz())
        .pclk(8.mhz());
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    board_support::systick::calibrate(&mut syst, clocks.clone());

    let mut reset = gpiob.pb3.into_output_open_drain_pull_up(&mut gpiob.regs);
    for i in 0..80100 {
        reset.set_low();
    }
    reset.set_high();

    let mut i2c = peripherals.I2C1.constrain(&mut rcc.apb1)
        .bind(gpiob.pb6.into_scl(&mut gpiob.regs), gpiob.pb7.into_sda(&mut gpiob.regs))
        .master(I2cTiming::new(I2cTimingSetting::Fast).unwrap());
    let mut buzzer = board_support::Buzzer::new(tim1, &mut rcc.apb2, &mut nvic, gpioa.pa8, &mut gpioa.regs);

    buzzer.beep(100, 1000.hz(), clocks.clone());

    let mut addr_fn = move || {
        loop {
            let mut trans = i2c.begin_write(0x3C, &CMD);
            match await!(trans.end_write()) {
                Ok(_) => {},
                Err(MasterI2cError::Nack) => {},// buzzer.beep(100, 1000.hz(), clocks.clone()),
                Err(_) => {},
            }
            i2c = trans.finish();
        }
    };
    let mut beep_fn = move || {
        loop {
            let now = board_support::systick::now();
            await!(board_support::systick::wait_until(now + 500)).unwrap();
            buzzer.beep(100, 1500.hz(), clocks.clone());
        }
    };
    loop {
        unsafe { addr_fn.resume() };
        unsafe { beep_fn.resume() };
    }
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
