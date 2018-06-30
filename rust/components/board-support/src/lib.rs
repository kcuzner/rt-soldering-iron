//! Board support crate for the RT Soldering Iron

#![no_std]
#![feature(used, never_type)]

extern crate bare_take_mut;
extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;
extern crate embedded_hal;
extern crate stm32f031x_hal;
#[macro_use]
extern crate stm32f031x;
extern crate nb;

mod buzzer;
pub mod ssd1306;
pub mod systick;

pub use buzzer::Buzzer;

// Expose declared ISRs

pub use buzzer::TIM1_BRK_UP_IRQ;
pub use systick::SysTick;

