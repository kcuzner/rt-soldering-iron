//! Board abstractions for easy use

use nb;
use cortex_m;

use stm32f031x;
use stm32f031x::{RCC, TIM1, NVIC};
use stm32f031x_hal::rcc::APB2;

static mut TICK_COUNT: u32 = 0;

#[no_mangle]
#[used]
pub extern "C" fn SYS_TICK() {
    unsafe { TICK_COUNT += 1; }
}

pub fn wait_until(tick: u32) -> nb::Result<u32, !> {
    cortex_m::interrupt::free(|_| unsafe {
        if TICK_COUNT >= tick {
            Ok(TICK_COUNT)
        }
        else {
            Err(nb::Error::WouldBlock)
        }
    })
}

static mut BUZZER_COUNTDOWN: u32 = 0;

pub struct Buzzer {
    tim: TIM1
}

impl Buzzer {
    /// Creates a new buzzer
    ///
    /// This requires that TIM1 be moved in here. This enfoces that there can
    /// only ever be one Buzzer at a time.
    pub fn new(tim: TIM1, apb2: &mut APB2, nvic: &mut NVIC) -> Self {
        apb2.enr().modify(|_, w| w.tim1en().bit(true));
        tim.psc.write(|w| unsafe { w.psc().bits(0) });
        tim.ccmr1_output.modify(|_, w| unsafe {
            w.oc1m().bits(0b110).oc1pe().bit(true)
        });
        tim.ccer.modify(|_, w| w.cc1e().bit(true));
        tim.bdtr.modify(|_, w| w.moe().bit(true));
        tim.dier.modify(|_, w| w.uie().bit(true));

        nvic.enable(stm32f031x::Interrupt::TIM1_BRK_UP_IRQ);
        Buzzer { tim: tim }
    }

    /// Beeps the buzzer
    pub fn beep(&self, ms: u16, freq: u16) {
        cortex_m::interrupt::free(|_| {
            let period = (8000000 / freq as u32) as u16;
            self.tim.arr.write(|w| unsafe { w.arr().bits(period) });
            self.tim.ccr1.write(|w| unsafe { w.ccr1().bits(period/2) });
            self.tim.cr1.modify(|_, w| w.cen().bit(true));
            self.tim.egr.write(|w| w.ug().bit(true));

            //This mutable static access is ok because do it during a critical
            //section.
            unsafe { BUZZER_COUNTDOWN = (ms as u32) * (freq as u32) / 1000 };
        });
    }
}

/// TIM1 IRQ which modifies the global BUZZER_COUNTDOWN state and turns off TIM1
/// when required.
#[no_mangle]
#[used]
pub extern "C" fn TIM1_BRK_UP_IRQ() {
    if unsafe { (*TIM1::ptr()).sr.read().uif().bit_is_set() } {
        if unsafe { BUZZER_COUNTDOWN == 0 } {
            unsafe { (*TIM1::ptr()).cr1.modify(|_, w| w.cen().bit(false)) };
        }
        else {
            unsafe { BUZZER_COUNTDOWN -= 1; }
        }
    }
    unsafe { (*TIM1::ptr()).sr.write(|w| w.bits(0)) }
}

