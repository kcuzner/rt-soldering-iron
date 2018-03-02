//! PWM Abstraction
//!
//! Based on the STM32F103xx HAL

use stm32f031x::{TIM1, TIM2, TIM3, TIM14};

use gpio::{gpioa, Alternate, PushPull, AlternateFunction};
use gpio::gpioa::PA8;

/// Trait applied to pins that are PWM-capable
pub trait ConfigurePwm<P, R> {
    fn map_pwm(self, regs: &mut R) -> P;
}

/// Implementation
impl<Mode> ConfigurePwm<PA8<Alternate<PushPull>>, gpioa::Regs> for PA8<Mode> {
    fn map_pwm(self, regs: &mut gpioa::Regs) -> PA8<Alternate<PushPull>> {
        unsafe { self.into_alternate_push_pull(AlternateFunction::AF2, regs) }
    }
}

pub trait PwmExt {
    /// Bus type for clock enables
    type Bus;

    fn pwm(self, bus: &mut Self::Bus);
}

