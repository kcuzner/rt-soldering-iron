//! Board support crate for the RT Soldering Iron

#![no_std]
#![feature(used)]

extern crate bare_take_mut;
extern crate cortex_m;
extern crate embedded_hal;
extern crate stm32f031x_hal;
extern crate stm32f031x;
extern crate nb;

mod buzzer;
mod ssd1306;
pub mod systick;

pub use buzzer::Buzzer;

// Expose declared ISRs

pub use buzzer::TIM1_BRK_UP_IRQ;
pub use systick::SYS_TICK;

