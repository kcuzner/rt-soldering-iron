//! GPIO Abstraction
//!
//! Based on the STM32F103xx HAL

use core::marker::PhantomData;

use rcc::AHB;

pub trait GpioExt {
    /// Type to split the GPIO into
    type Parts;

    /// Consumes this and splits it into parts
    fn split(self, ahb: &mut AHB) -> Self::Parts;
}

/// Input mode
pub struct Input<Mode> {
    _mode: PhantomData<Mode>,
}

/// Floating input
pub struct Floating;

/// Pull-up input
pub struct PullUp;

/// Pull-down input
pub struct PullDown;

/// Output mode
pub struct Output<Mode> {
    _mode: PhantomData<Mode>,
}

/// Alternate function 0
pub struct AF0<Mode> {
    _mode: PhantomData<Mode>
}
/// Alternate function 1
pub struct AF1<Mode> {
    _mode: PhantomData<Mode>
}
/// Alternate function 2
pub struct AF2<Mode> {
    _mode: PhantomData<Mode>
}
/// Alternate function 3
pub struct AF3<Mode> {
    _mode: PhantomData<Mode>
}
/// Alternate function 4
pub struct AF4<Mode> {
    _mode: PhantomData<Mode>
}
/// Alternate function 5
pub struct AF5<Mode> {
    _mode: PhantomData<Mode>
}
/// Alternate function 6
pub struct AF6<Mode> {
    _mode: PhantomData<Mode>
}
/// Alternate function 7
pub struct AF7<Mode> {
    _mode: PhantomData<Mode>
}

/// Push-Pull mode
pub struct PushPull;

/// Open-Drain mode
pub struct OpenDrain;

/// Open-Drain mode with a pullup
pub struct OpenDrainPullUp;

pub(crate) trait IntoAlternate<Mode> {
    type Regs;
    type Output;
    fn into_alternate(self, regs: &mut Self::Regs) -> Self::Output;
}

/// Analog mode
pub struct Analog;

/// Alternate function used for configuration of pins
enum AlternateFunction {
    AF0, AF1, AF2, AF3, AF4, AF5, AF6, AF7
}

/// Macro for creating a GPIO abstraction
///
/// $GPIOX: GPIO struct to extend
/// $gpiox: mod name for the GPIO
/// $iopenxr: IOPENR register for this GPIO
/// $PXx: struct name for a general pin
/// $PXi: struct name for a particular pin
/// $pxi: property name in the Parts struct
/// $i: Pin index
/// $MODE: Default pin mode
macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, $iopenxr:ident, $PXx: ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, $Mode:ty, [$($AFn:ident,)*]),)+
    ]) => {
        /// GPIO
        pub mod $gpiox {
            use core::marker::PhantomData;
            use hal::digital::{OutputPin, InputPin};
            use stm32f031x::$GPIOX;
            use rcc::AHB;
            use super::{
                GpioExt,
                Input, Floating, PullUp, PullDown,
                Output, PushPull, OpenDrain, OpenDrainPullUp,
                IntoAlternate, AlternateFunction,
                Analog
            };

            /// GPIO Parts
            pub struct Parts {
                pub regs: Regs,
                $(
                    ///Pin
                    pub $pxi: $PXi<$Mode>,
                )+
            }

            /// Register struct for ensuring safety when modifying pins
            pub struct Regs {
                _0: (),
            }

            impl GpioExt for $GPIOX {
                type Parts = Parts;

                fn split(self, ahb: &mut AHB) -> Parts {
                    ahb.enr().modify(|_, w| w.$iopenxr().bit(true));

                    Parts {
                        regs: Regs { _0: () },
                        $(
                            $pxi: $PXi { _mode: PhantomData },
                        )+
                    }
                }
            }

            $(
                ///Pin
                pub struct $PXi<Mode> {
                    _mode: PhantomData<Mode>,
                }

                impl<Mode> $PXi<Mode> {
                    /// Configures the pin as passed.
                    ///
                    /// Unsafe because no checking of the consistency between ToMode and the passed
                    /// arguments is perfomed. The purpose of this function is to facilitate DRY.
                    unsafe fn configure<ToMode>(self, _: &mut Regs, moder: u32, otyper: bool, pupdr: u32) -> $PXi<ToMode> {
                        let offset = (2 * $i) % 32;
                        //mask off any unruly bits
                        let moder = moder & 0x3;
                        let pupdr = pupdr & 0x3;
                        (*$GPIOX::ptr())
                            .moder.modify(|r, w| { w.bits((r.bits() & !(0b11 << offset)) | (moder << offset)) });
                        if otyper {
                            (*$GPIOX::ptr())
                                .otyper.modify(|r, w| { w.bits(r.bits() | (0b1 << ($i % 32))) });
                        }
                        else {
                            (*$GPIOX::ptr())
                                .otyper.modify(|r, w| { w.bits(r.bits() & !(0b1 << ($i % 32))) });
                        }
                        (*$GPIOX::ptr())
                            .pupdr.modify(|r, w| { w.bits((r.bits() & (!0b11 << offset)) | (pupdr << offset)) });
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin into a push-pull output
                    pub fn into_output_push_pull(self, r: &mut Regs) -> $PXi<Output<PushPull>> {
                        unsafe { self.configure::<Output<PushPull>>(r, 0b01, false, 0) }
                    }

                    /// Configures the pin into an open-drain output
                    pub fn into_output_open_drain(self, r: &mut Regs) -> $PXi<Output<OpenDrain>> {
                        unsafe { self.configure::<Output<OpenDrain>>(r, 0b01, true, 0) }
                    }

                    /// Configures the pin into an open-drain output with the pullup enabled
                    pub fn into_output_open_drain_pull_up(self, r: &mut Regs) -> $PXi<Output<OpenDrainPullUp>> {
                        unsafe { self.configure::<Output<OpenDrainPullUp>>(r, 0b01, true, 0b01) }
                    }

                    /// Configures the pin into a floating input
                    pub fn into_input_floating(self, r: &mut Regs) -> $PXi<Input<Floating>> {
                        unsafe { self.configure::<Input<Floating>>(r, 0, false, 0) }
                    }

                    /// Configures the pin into a pulled up input
                    pub fn into_input_pullup(self, r: &mut Regs) -> $PXi<Input<PullUp>> {
                        unsafe { self.configure::<Input<PullUp>>(r, 0, false, 0b01) }
                    }

                    /// Configures the pin into a pulled down input
                    pub fn into_input_pulldown(self, r: &mut Regs) -> $PXi<Input<PullDown>> {
                        unsafe { self.configure::<Input<PullDown>>(r, 0, false, 0b10) }
                    }

                    /// Configures the pin into alternate mode with a push pull driver
                    ///
                    /// This is unsafe because it does no checking on which alternate function is
                    /// selected and whether that is valid for this particular pin.
                    #[allow(dead_code)]
                    unsafe fn into_alternate_push_pull<AF>(self, af: AlternateFunction, r: &mut Regs) -> $PXi<AF>
                    {
                        self.configure::<AF>(r, 0b10, false, 0b00)
                            .into_alternate_impl(af, r)
                    }

                    /// Configures the pin into alternate mode with an open-drain driver
                    ///
                    /// This is unsafe because it does no checking on which alternate function is
                    /// selected and whether that is valid for this particular pin.
                    #[allow(dead_code)]
                    unsafe fn into_alternate_open_drain<AF>(self, af: AlternateFunction, r: &mut Regs) -> $PXi<AF>
                    {
                        self.configure::<AF>(r, 0b10, true, 0b00)
                            .into_alternate_impl(af, r)
                    }

                    /// Sets the appropriate AFR for this pin
                    unsafe fn into_alternate_impl<AF>(self, af: AlternateFunction, _: &mut Regs) -> $PXi<AF>
                    {
                        let alternate_function = match af {
                            AlternateFunction::AF0 => 0b0000,
                            AlternateFunction::AF1 => 0b0001,
                            AlternateFunction::AF2 => 0b0010,
                            AlternateFunction::AF3 => 0b0011,
                            AlternateFunction::AF4 => 0b0100,
                            AlternateFunction::AF5 => 0b0101,
                            AlternateFunction::AF6 => 0b0110,
                            AlternateFunction::AF7 => 0b0111
                        };
                        if $i < 8 {
                            let offset = (4 * $i) % 32;
                            (*$GPIOX::ptr())
                                .afrl.modify(|r, w| { w.bits((r.bits() & !(0b1111 << offset)) | (alternate_function << offset)) });
                        }
                        else {
                            let offset = (4 * ($i - 8)) % 32;
                            (*$GPIOX::ptr())
                                .afrh.modify(|r, w| { w.bits((r.bits() & !(0b1111 << offset)) | (alternate_function << offset)) });
                        }

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures this pin into analog mode
                    ///
                    /// This is unsafe because not all banks actually map to an analog pin. All
                    /// GPIOs do implement this setting, however. It is intended that this will be
                    /// used by another HAL module to implement a pin-specific trait to change the
                    /// pin.
                    #[allow(dead_code)]
                    pub(crate) unsafe fn into_analog(self, r: &mut Regs) -> $PXi<Analog> {
                        self.configure::<Analog>(r, 0b11, false, 0)
                    }
                }

                $(
                    /// Implements an open drain alternate function
                    impl<Mode> IntoAlternate<super::$AFn<OpenDrain>> for $PXi<Mode> {
                        type Regs = Regs;
                        type Output = $PXi<super::$AFn<OpenDrain>>;
                        fn into_alternate(self, regs: &mut Regs) -> $PXi<super::$AFn<OpenDrain>> {
                            unsafe { self.into_alternate_open_drain(AlternateFunction::$AFn, regs) }
                        }
                    }
                    /// Implements a push pull alternate function
                    impl<Mode> IntoAlternate<super::$AFn<PushPull>> for $PXi<Mode> {
                        type Regs = Regs;
                        type Output = $PXi<super::$AFn<PushPull>>;
                        fn into_alternate(self, regs: &mut Regs) -> $PXi<super::$AFn<PushPull>> {
                            unsafe { self.into_alternate_push_pull(AlternateFunction::$AFn, regs) }
                        }
                    }
                )*

                impl<Mode> OutputPin for $PXi<Output<Mode>> {
                    fn is_high(&self) -> bool {
                        !self.is_low()
                    }

                    fn is_low(&self) -> bool {
                        // Atomic read with no side effects
                        unsafe { (*$GPIOX::ptr()).odr.read().bits() & (1 << $i) == 0 }
                    }

                    fn set_high(&mut self) {
                        // Atomic write to stateless register (BSRR)
                        unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << $i)) }
                    }

                    fn set_low(&mut self) {
                        // Atomic write to stateless register (BSRR)
                        unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << ($i + 16))) }
                    }
                }

                impl<Mode> InputPin for $PXi<Input<Mode>> {
                    fn is_high(&self) -> bool {
                        !self.is_low()
                    }
                    
                    fn is_low(&self) -> bool {
                        // Atomic read with no side effects
                        unsafe { (*$GPIOX::ptr()).idr.read().bits() & (1 << $i) == 0 }
                    }
                }
            )+
        }
    }
}

// This configuration is specifically for the QFN32 package

gpio!(GPIOA, gpioa, iopaen, PAx, [
      PA0: (pa0, 0, Input<Floating>, [AF1, AF2,]),
      PA1: (pa1, 1, Input<Floating>, [AF0, AF1, AF2,]),
      PA2: (pa2, 2, Input<Floating>, [AF1, AF2,]),
      PA3: (pa3, 3, Input<Floating>, [AF1, AF2,]),
      PA4: (pa4, 4, Input<Floating>, [AF0, AF1, AF4,]),
      PA5: (pa5, 5, Input<Floating>, [AF0, AF2,]),
      PA6: (pa6, 6, Input<Floating>, [AF0, AF1, AF2, AF5, AF6,]),
      PA7: (pa7, 7, Input<Floating>, [AF0, AF1, AF2, AF4, AF5, AF6,]),
      PA8: (pa8, 8, Input<Floating>, [AF0, AF1, AF2, AF3,]),
      PA9: (pa9, 9, Input<Floating>, [AF1, AF2, AF4,]),
      PA10: (pa10, 10, Input<Floating>, [AF0, AF1, AF2, AF4,]),
      PA11: (pa11, 11, Input<Floating>, [AF0, AF1, AF2,]),
      PA12: (pa12, 12, Input<Floating>, [AF0, AF1, AF2,]),
      PA13: (pa13, 13, Input<Floating>, [AF0, AF1,]),
      PA14: (pa14, 14, Input<Floating>, [AF0, AF1,]),
      PA15: (pa15, 15, Input<Floating>, [AF0, AF1, AF2, AF3,]),
]);

gpio!(GPIOB, gpiob, iopben, PBx, [
      PB0: (pb0, 0, Input<Floating>, [AF0, AF1, AF2,]),
      PB1: (pb1, 1, Input<Floating>, [AF0, AF1, AF2,]),
      PB2: (pb2, 2, Input<Floating>, []),
      PB3: (pb3, 3, Input<Floating>, [AF0, AF1, AF2,]),
      PB4: (pb4, 4, Input<Floating>, [AF0, AF1, AF2,]),
      PB5: (pb5, 5, Input<Floating>, [AF0, AF1, AF2, AF3,]),
      PB6: (pb6, 6, Input<Floating>, [AF0, AF1, AF2,]),
      PB7: (pb7, 7, Input<Floating>, [AF0, AF1, AF2,]),
]);

