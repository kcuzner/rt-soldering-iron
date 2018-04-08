//! PWM Abstraction
//!
//! Based on the STM32F103xx HAL

/// Trait applied to pins that are PWM-capable
pub trait IntoPwm<ChannelPin> {
    /// GPIO regs
    type Regs;

    /// Switches this pin into the PWM function
    fn into_pwm(self, regs: &mut Self::Regs) -> ChannelPin;
}

pub trait PwmExt {
    /// Bus type for clock enables
    type Bus;

    /// PWM Control parts
    type Parts;

    fn constrain_pwm(self, bus: &mut Self::Bus) -> Self::Parts;
}

pub unsafe trait ChannelPin<Channel> {
}

macro_rules! pwm {
    ($TIMX: ident, $timx:ident, $pwmx: ident, $busx:ident, $busxenr:ident, [
        $($CHi:ident: ($name:ident, $ccmrx:ident, $ccmrxocxm:ident,
                       $ccmrxocxpe:ident, $ccrx:ident, $ccenr:ident, [
                       $($PXi:ident: ($gpiox:ident, $af:ident),)+]),)+
    ]) => {
        pub mod $pwmx {
            use cortex_m;
            use hal::PwmPin;
            use stm32f031x::{$TIMX, $timx};
            use rcc::{$busx, Clocks};
            use gpio;
            use gpio::{PushPull, IntoAlternate};
            use time::Hertz;
            use super::{PwmExt, IntoPwm, ChannelPin};

            impl PwmExt for $TIMX {
                type Bus = $busx;

                type Parts = Parts;

                fn constrain_pwm(self, bus: &mut Self::Bus) -> Self::Parts
                {
                    bus.enr().modify(|_, w| w.$busxenr().bit(true));

                    unsafe { (*$TIMX::ptr()).bdtr.modify(|_, w| w.moe().bit(true)) }

                    Parts {
                        _0: ()
                    }
                }
            }

            pub struct Parts {
                _0: ()
            }

            impl Parts {
                /// Commits any changes to the timer immediately
                pub fn commit(&self) {
                    // This is an atomic write to a write-only stateless register
                    unsafe { (*$TIMX::ptr()).egr.write(|w| w.ug().bit(true)) };
                }

                /// Grants access to the DIER register
                pub fn dier(&mut self) -> &$timx::DIER {
                    // This proxy ensures exclusive access
                    unsafe { &(*$TIMX::ptr()).dier }
                }

                pub fn enable(&mut self) {
                    // This proxy ensure exclusive access
                    unsafe { &(*$TIMX::ptr()).cr1.modify(|_, w| w.cen().bit(true)) };
                }

                pub fn disable(&mut self) {
                    // This proxy ensures exclusive access
                    unsafe { &(*$TIMX::ptr()).cr1.modify(|_, w| w.cen().bit(false)) };
                }

                /// Sets the period of this timer
                pub fn set_period<T>(&mut self, clocks: Clocks, freq: T)
                where T: Into<Hertz>
                {
                    //TODO: Is this even safe? All of the PWM output frequencies will change
                    let period = (clocks.timclk().0 / freq.into().0) as u16;
                    // This proxy grants exclusive access
                    unsafe { (*$TIMX::ptr()).psc.write(|w| { w.psc().bits(0) }) };
                    unsafe { (*$TIMX::ptr()).arr.write(|w| { w.arr().bits(period) }) };
                }

                $(
                    /// Break out PWM channel with a pin.
                    pub fn $name<T>(&mut self, pin: T) -> $CHi
                    where T: ChannelPin<$CHi>
                    {
                        //TODO: This will no longer work with multiple pins per channel
                        // This proxy provides exclusive access to $TIMx
                        unsafe { (*$TIMX::ptr()).$ccmrx.modify(|_, w| {
                            w.$ccmrxocxm().bits(0b110).$ccmrxocxpe().bit(true)
                        }) };

                        // We trust the macro invoker to have verified this is a valid alternate
                        // function and will do the thing we expect.
                        $CHi { _0: () }
                    }
                )+
            }

            $(
                pub struct $CHi {
                    _0: ()
                }

                impl PwmPin for $CHi {
                    type Duty = u16;

                    fn enable(&mut self) {
                        //TODO: Oh noes, we need bitbanding.
                        cortex_m::interrupt::free(|_| {
                            unsafe { (*$TIMX::ptr())
                                .ccer.modify(|_, w| { w.$ccenr().bit(true) }) };
                        });
                    }

                    fn disable(&mut self) {
                        //TODO: Oh noes, we need bitbanding.
                        cortex_m::interrupt::free(|_| {
                            unsafe { (*$TIMX::ptr())
                                .ccer.modify(|_, w| { w.$ccenr().bit(false) }) };
                        });
                    }

                    fn get_duty(&self) -> Self::Duty {
                        // Atomic read and this proxy ensure exclusive access
                        unsafe { (*$TIMX::ptr()).$ccrx.read().$ccrx().bits() }
                    }

                    fn get_max_duty(&self) -> Self::Duty {
                        // Atomic read
                        unsafe { (*$TIMX::ptr()).arr.read().arr().bits() }
                    }

                    fn set_duty(&mut self, duty: Self::Duty) {
                        // This proxy ensures exclusive access and this is an atomic write.
                        unsafe { (*$TIMX::ptr()).$ccrx.write(|w| w.$ccrx().bits(duty)) }
                    }
                }

                $(
                    unsafe impl<Mode> ChannelPin<$CHi> for gpio::$gpiox::$PXi<gpio::$af<Mode>> {
                    }
                    impl<Mode> IntoPwm<gpio::$gpiox::$PXi<gpio::$af<PushPull>>> for gpio::$gpiox::$PXi<Mode> {
                        type Regs = gpio::$gpiox::Regs;
                        fn into_pwm(self, regs: &mut gpio::$gpiox::Regs) -> gpio::$gpiox::$PXi<gpio::$af<PushPull>> {
                            IntoAlternate::<gpio::$af<PushPull>>::into_alternate(self, regs)
                        }
                    }
                )+
            )+
        }
    }
}

pwm!(TIM1, tim1,  pwm1, APB2, tim1en, [
     CH1: (ch1, ccmr1_output, oc1m, oc1pe, ccr1, cc1e, [
           PA8: (gpioa, AF2),
     ]),
]);

