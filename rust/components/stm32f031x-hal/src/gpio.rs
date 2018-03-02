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

/// Alternate function mode
pub struct Alternate<Mode> {
    _mode: PhantomData<Mode>,
}

/// Push-Pull mode
pub struct PushPull;

/// Open-Drain mode
pub struct OpenDrain;

/// Alternate function enumeration
pub enum AlternateFunction {
    AF0,
    AF1,
    AF2,
    AF3,
    AF4,
    AF5,
    AF6,
    AF7
}

/// Analog mode
pub struct Analog;

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
        $($PXi:ident: ($pxi:ident, $i:expr, $Mode:ty),)+
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
                Output, Alternate, PushPull, OpenDrain, AlternateFunction,
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
                    /// Configures the pin into a push-pull output
                    pub fn into_output_push_pull(self, _: &mut Regs) -> $PXi<Output<PushPull>> {
                        let offset = (2 * $i) % 32;
                        // The following read-writes are safe because we have locked Regs and there
                        // is only ever one.
                        unsafe { (*$GPIOX::ptr())
                            .moder.modify(|r, w| { w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset)) }) };
                        unsafe { (*$GPIOX::ptr())
                            .otyper.modify(|r, w| { w.bits(r.bits() & !(0b1 << ($i % 32))) }) };

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin into an open-drain output
                    pub fn into_output_open_drain(self, _: &mut Regs) -> $PXi<Output<OpenDrain>> {
                        let offset = (2 * $i) % 32;
                        // The following read-writes are safe because we have locked Regs and there
                        // is only ever one.
                        unsafe { (*$GPIOX::ptr())
                            .moder.modify(|r, w| { w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset)) }) };
                        unsafe { (*$GPIOX::ptr())
                            .otyper.modify(|r, w| { w.bits(r.bits() | (0b1 << ($i % 32))) }) };

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin into a floating input
                    pub fn into_input_floating(self, _: &mut Regs) -> $PXi<Input<Floating>> {
                        let offset = (2 * $i) % 32;
                        // The following read-writes are safe because we have locked Regs and there
                        // is only ever one.
                        unsafe { (*$GPIOX::ptr())
                            .moder.modify(|r, w| { w.bits(r.bits() & !(0b11 << offset)) }) };
                        unsafe { (*$GPIOX::ptr())
                            .pupdr.modify(|r, w| { w.bits(r.bits() & !(0b11 << offset)) }) };

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin into a pulled up input
                    pub fn into_input_pullup(self, _: &mut Regs) -> $PXi<Input<PullUp>> {
                        let offset = (2 * $i) % 32;
                        // The following read-writes are safe because we have locked Regs and there
                        // is only ever one.
                        unsafe { (*$GPIOX::ptr())
                            .moder.modify(|r, w| { w.bits(r.bits() & !(0b11 << offset)) }) };
                        unsafe { (*$GPIOX::ptr())
                            .pupdr.modify(|r, w| { w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset)) }) };

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin into a pulled down input
                    pub fn into_input_pulldown(self, _: &mut Regs) -> $PXi<Input<PullDown>> {
                        let offset = (2 * $i) % 32;
                        // The following read-writes are safe because we have locked Regs and there
                        // is only ever one.
                        unsafe { (*$GPIOX::ptr())
                            .moder.modify(|r, w| { w.bits(r.bits() & !(0b11 << offset)) }) };
                        unsafe { (*$GPIOX::ptr())
                            .pupdr.modify(|r, w| { w.bits((r.bits() & !(0b11 << offset)) | (0b10 << offset)) }) };

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin into alternate mode with a push pull driver
                    ///
                    /// This is unsafe because it does no checking on which alternate function is
                    /// selected and whether that is valid for this particular pin. It is intended
                    /// that this will be used by another HAL module to implement a pin-specific
                    /// trait to change the pin.
                    #[allow(dead_code)]
                    pub(crate) unsafe fn into_alternate_push_pull(self, alt: AlternateFunction, r: &mut Regs) -> $PXi<Alternate<PushPull>> {
                        let offset = (2 * $i) % 32;
                        // The following read-writes are data race safe because we have locked Regs and there
                        // is only ever one.
                        (*$GPIOX::ptr())
                            .moder.modify(|r, w| { w.bits((r.bits() & !(0b11 << offset)) | (0b10 << offset)) });
                        (*$GPIOX::ptr())
                            .otyper.modify(|r, w| { w.bits(r.bits() & !(0b1 << ($i % 32))) });
                        self.into_alternate_impl(alt, r)
                    }

                    /// Configures the pin into alternate mode with an open-drain driver
                    ///
                    /// This is unsafe because it does no checking on which alternate function is
                    /// selected and whether that is valid for this particular pin. It is intended
                    /// that this will be used by another HAL module to implement a pin-specific
                    /// trait to change the pin.
                    #[allow(dead_code)]
                    pub(crate) unsafe fn into_alternate_open_drain(self, alt: AlternateFunction, r: &mut Regs) -> $PXi<Alternate<OpenDrain>> {
                        let offset = (2 * $i) % 32;
                        // The following read-writes are data race safe because we have locked Regs and there
                        // is only ever one.
                        (*$GPIOX::ptr())
                            .moder.modify(|r, w| { w.bits((r.bits() & !(0b11 << offset)) | (0b10 << offset)) });
                        (*$GPIOX::ptr())
                            .otyper.modify(|r, w| { w.bits(r.bits() & !(0b1 << ($i % 32))) });
                        self.into_alternate_impl(alt, r)
                    }

                    /// Sets the appropriate AFR for this pin
                    unsafe fn into_alternate_impl<AltMode>(self, alt: AlternateFunction, _: &mut Regs) -> $PXi<Alternate<AltMode>> {
                        let alternate_function = match alt {
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
                                .afrl.modify(|r, w| { w.bits((r.bits() & !(0b1111 << offset)) | (alternate_function << offset)) });
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
                    pub(crate) unsafe fn into_analog(self, _: &mut Regs) -> $PXi<Analog> {
                        let offset = (2 * $i) % 32;
                        // The following read-writes are data race safe because we have locked Regs and there
                        // is only ever one.
                        (*$GPIOX::ptr())
                            .moder.modify(|r, w| { w.bits(r.bits() | (0b11 << offset)) });
                        $PXi { _mode: PhantomData }
                    }
                }

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
      PA0: (pa0, 0, Input<Floating>),
      PA1: (pa1, 1, Input<Floating>),
      PA2: (pa2, 2, Input<Floating>),
      PA3: (pa3, 3, Input<Floating>),
      PA4: (pa4, 4, Input<Floating>),
      PA5: (pa5, 5, Input<Floating>),
      PA6: (pa6, 6, Input<Floating>),
      PA7: (pa7, 7, Input<Floating>),
      PA8: (pa8, 8, Input<Floating>),
      PA9: (pa9, 9, Input<Floating>),
      PA10: (pa10, 10, Input<Floating>),
      PA11: (pa11, 11, Input<Floating>),
      PA12: (pa12, 12, Input<Floating>),
      PA13: (pa13, 13, Input<Floating>),
      PA14: (pa14, 14, Input<Floating>),
      PA15: (pa15, 15, Input<Floating>),
]);

gpio!(GPIOB, gpiob, iopben, PBx, [
      PB0: (pb0, 0, Input<Floating>),
      PB1: (pb1, 1, Input<Floating>),
      PB2: (pb2, 2, Input<Floating>),
      PB3: (pb3, 3, Input<Floating>),
      PB4: (pb4, 4, Input<Floating>),
      PB5: (pb5, 5, Input<Floating>),
      PB6: (pb6, 6, Input<Floating>),
      PB7: (pb7, 7, Input<Floating>),
]);

