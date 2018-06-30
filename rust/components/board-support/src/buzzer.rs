//! Buzzer abstraction

use cortex_m;

use stm32f031x;
use stm32f031x::{TIM1, NVIC};
use stm32f031x_hal::rcc::{APB2, Clocks};
use stm32f031x_hal::pwm::{PwmExt, IntoPwm, pwm1};
use stm32f031x_hal::gpio::gpioa;
use stm32f031x_hal::time::Hertz;

use embedded_hal::PwmPin;

static mut BUZZER_COUNTDOWN: u32 = 0;

pub struct Buzzer {
    pwm: pwm1::Parts,
    ch1: pwm1::CH1,
}

impl Buzzer {
    /// Creates a new buzzer
    ///
    /// This requires that TIM1 be moved in here. This enfoces that there can
    /// only ever be one Buzzer at a time.
    pub fn new<Mode>(tim: TIM1, apb2: &mut APB2, nvic: &mut NVIC, pa8: gpioa::PA8<Mode>, gpioa: &mut gpioa::Regs) -> Self {
        let mut pwm = tim.constrain_pwm(apb2);
        let mut ch1 = pwm.ch1(pa8.into_pwm(gpioa));
        ch1.enable();
        pwm.dier().modify(|_, w| w.uie().bit(true));

        nvic.enable(stm32f031x::Interrupt::TIM1_BRK_UP_IRQ);
        Buzzer { pwm: pwm, ch1: ch1 }
    }

    /// Beeps the buzzer
    pub fn beep<T>(&mut self, ms: u16, freq: T, clocks: Clocks)
    where T: Into<Hertz> + Clone
    {
        cortex_m::interrupt::free(|_| {
            self.pwm.set_period(clocks, freq.clone());
            let max_duty = self.ch1.get_max_duty();
            self.ch1.set_duty(max_duty / 2);
            self.pwm.enable();
            self.pwm.commit();

            //This mutable static access is ok because do it during a critical
            //section.
            unsafe { BUZZER_COUNTDOWN = (ms as u32) * (freq.into().0) / 1000 };
        });
    }
}

interrupt!(TIM1_BRK_UP_IRQ, buzzer_isr);

/// TIM1 IRQ which modifies the global BUZZER_COUNTDOWN state and turns off TIM1
/// when required.
fn buzzer_isr() {
    if unsafe { (*TIM1::ptr()).sr.read().uif().bit_is_set() } {
        if unsafe { BUZZER_COUNTDOWN == 0 } {
            // Safe because all other accesses to cr1 occur in a critical section
            unsafe { (*TIM1::ptr()).cr1.modify(|_, w| w.cen().bit(false)) };
        }
        else {
            unsafe { BUZZER_COUNTDOWN -= 1; }
        }
    }
    unsafe { (*TIM1::ptr()).sr.write(|w| w.bits(0)) }
}

