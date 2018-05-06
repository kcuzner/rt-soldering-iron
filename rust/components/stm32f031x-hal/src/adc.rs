//! ADC Abstraction

use core::marker::PhantomData;

use nb;

use stm32f031x::{ADC, adc};

use gpio;
use rcc::{APB2};

/// Trait applied to pins that are analog capable
pub trait IntoAnalog<AnalogPin> {
    /// GPIO regs
    type Regs;

    /// Switches this pin into an analog input
    fn into_analog_input(self, regs: &mut Self::Regs) -> AnalogPin;
}

/// Trait applied when a particular pin in a particular mode can be used as an analog input
///
/// This is unsafe because not everything can be an analog pin
pub unsafe trait AnalogPin {
    const CHANNEL: u32;
}

/// Macro for declaring an analog pin, to reduce code repetition
macro_rules! analog_pin {
    ($CHx:expr, ($PXi:ident, $gpiox:ident)) => {
        impl<Mode> IntoAnalog<gpio::$gpiox::$PXi<gpio::Analog>> for gpio::$gpiox::$PXi<Mode> {
            type Regs = gpio::$gpiox::Regs;

            fn into_analog_input(self, regs: &mut Self::Regs) -> gpio::$gpiox::$PXi<gpio::Analog> {
                // The invoker of this macro knows that this pin really is analog
                // so this is safe.
                unsafe { self.into_analog(regs) }
            }
        }

        unsafe impl AnalogPin for gpio::$gpiox::$PXi<gpio::Analog> {
            const CHANNEL: u32 = $CHx;
        }
    }
}

analog_pin!(0, (PA0, gpioa));
analog_pin!(1, (PA1, gpioa));
analog_pin!(2, (PA2, gpioa));
analog_pin!(3, (PA3, gpioa));
analog_pin!(4, (PA4, gpioa));
analog_pin!(5, (PA5, gpioa));
analog_pin!(6, (PA6, gpioa));
analog_pin!(7, (PA7, gpioa));
analog_pin!(8, (PB0, gpiob));
analog_pin!(9, (PB1, gpiob));

/// Extention trait for the ADC instance
pub trait AdcExt {
    /// Bus type for clock enables
    type Bus;

    type Constrained;

    fn constrain(self, bus: &mut Self::Bus) -> Self::Constrained;
}

impl AdcExt for ADC {
    type Bus = APB2;
    type Constrained = Uncalibrated;

    fn constrain(self, bus: &mut APB2) -> Uncalibrated {
        Uncalibrated::new(self, bus)
    }
}

/// Resolution of the analog to digital converter
#[derive(Copy, Clone)]
pub enum AdcResolution {
    /// 12-bit resolution
    TwelveBit,
    /// 10-bit resolution
    TenBit,
    /// 8-bit resolution
    EightBit,
    /// 6-bit resolution
    SixBit,
}

/// Alignment of an ADC value
#[derive(Copy, Clone)]
pub enum AdcAlignment {
    /// Left alignment
    Left,
    /// Right alignment
    Right,
}

/// Holds a raw ADC value
///
/// Created directly from the current value of the data register
#[derive(Copy, Clone)]
pub struct AdcValue {
    resolution: AdcResolution,
    alignment: AdcAlignment,
    value: u16,
}

impl AdcValue {
    /// Creates a new AdcValue from the contents of DR
    ///
    /// This function is unsafe because it performs several sequential reads. The caller must
    /// ensure that it has exclusive access to the ADC before calling.
    unsafe fn new() -> AdcValue {
        let value = (*ADC::ptr()).dr.read().data().bits();
        let cfgr1 = (*ADC::ptr()).cfgr1.read();
        let resolution = match cfgr1.res().bits() {
            0b00 => AdcResolution::TwelveBit,
            0b01 => AdcResolution::TenBit,
            0b10 => AdcResolution::EightBit,
            0b11 => AdcResolution::SixBit,
            _ => unreachable!(),
        };
        let alignment = match cfgr1.align().bit() {
            true => AdcAlignment::Left,
            false => AdcAlignment::Right,
        };
        AdcValue {
            resolution: resolution,
            alignment: alignment,
            value: value
        }
    }

    /// Gets the resolution of this ADC value
    pub fn resolution(&self) -> AdcResolution {
        self.resolution
    }

    /// Gets the alignment of this ADC value
    pub fn alignment(&self) -> AdcAlignment {
        self.alignment
    }

    /// Gets the raw ADC value
    pub fn raw(&self) -> u16 {
        self.value
    }

    /// Gets the right-aligned raw ADC value
    pub fn raw_right(&self) -> u16 {
        let preshift = match self.alignment() {
            AdcAlignment::Left => match self.resolution() {
                AdcResolution::TwelveBit => 4,
                AdcResolution::TenBit => 6,
                AdcResolution::EightBit => 8,
                AdcResolution::SixBit => 2,
            },
            _ => 0,
        };
        self.raw() >> preshift
    }
}

impl From<AdcValue> for u8 {
    /// Converts an ADC value into a u8, scaling to the range by shifting
    fn from(v: AdcValue) -> u8 {
        let shift = match v.resolution() {
            AdcResolution::TwelveBit => 4,
            AdcResolution::TenBit => 2,
            AdcResolution::EightBit => 0,
            AdcResolution::SixBit => -2,
        };
        (v.raw_right() >> shift) as u8
    }
}

impl From<AdcValue> for u16 {
    /// Converts an ADC value into a u16, scaling to the range by shifting
    fn from(v: AdcValue) -> u16 {
        let shift = match v.resolution() {
            AdcResolution::TwelveBit => -2,
            AdcResolution::TenBit => -4,
            AdcResolution::EightBit => -8,
            AdcResolution::SixBit => -10,
        };
        (v.raw_right() >> shift) as u16
    }
}

impl From<AdcValue> for u32 {
    /// Converts an ADC value into a u32, scaling to the range by shifting
    fn from(v: AdcValue) -> u32 {
        let sixteen: u16 = v.into();
        (sixteen as u32) << 16
    }
}

/// Steps performed during the calibration phase
enum CalibrationState {
    Disabling,
    Calibrating,
    Enabling,
}

/// Uncalibrated ADC
///
/// The ADC is uncalibrated out of reset, so this instance will immediately
/// begin calibration. Once a CalibratedToken is produced, a Calibrated ADC
/// can be created.
pub struct Uncalibrated {
    state: CalibrationState
}

impl Uncalibrated {
    /// Creates a new uncalibrated ADC instance
    ///
    /// By default, the ADC is assumed to be uncalibrated, so this is the only
    /// thing that can be constructed from an ADC instance.
    pub fn new(_adc: ADC, bus: &mut APB2) -> Self {
        // Enable the ADC clock
        bus.enr().modify(|_, w| w.adcen().bit(true));
        // We are a proxy for ADC, so this is safe
        // Step 1: Disable the ADC
        unsafe { (*ADC::ptr()).cr.modify(|_, w| w.addis().bit(true)) };
        Uncalibrated { state: CalibrationState::Disabling }
    }

    pub fn poll(&mut self) -> nb::Result<CalibrationToken, !> {
        // We are a proxy for ADC, so pointer ops are safe (note mut self)
        match self.state {
            CalibrationState::Disabling => {
                // Have we finished disabling the ADC?
                if !unsafe { (*ADC::ptr()).cr.read().aden().bit() } {
                    //we have, so let's begin calibration
                    unsafe { (*ADC::ptr()).cfgr1.modify(|_, w| w.dmaen().bit(false)) };
                    unsafe { (*ADC::ptr()).cr.modify(|_, w| w.adcal().bit(true)) };
                    self.state = CalibrationState::Calibrating;
                }
                Err(nb::Error::WouldBlock)
            },
            CalibrationState::Calibrating => {
                // Have we finished calibration? adcal is cleared by hardware
                if !unsafe { (*ADC::ptr()).cr.read().adcal().bit() } {
                    // we have, so enable the ADC
                    unsafe { (*ADC::ptr()).isr.write(|w| w.adrdy().bit(true)) };
                    unsafe { (*ADC::ptr()).cr.modify(|_, w| w.aden().bit(true)) };
                    self.state = CalibrationState::Enabling;
                }
                Err(nb::Error::WouldBlock)
            },
            CalibrationState::Enabling => {
                // Have we enabled the ADC?
                if unsafe { (*ADC::ptr()).isr.read().adrdy().bit() } {
                    // we have, so produce a calibration token
                    Ok(CalibrationToken::new())
                }
                else {
                    Err(nb::Error::WouldBlock)
                }
            }
        }
    }
}

/// Token produced when the ADC has been calibrated
pub struct CalibrationToken {
    _0: ()
}

impl CalibrationToken {
    /// Creates a new token meaning that the ADC has been calibrated
    fn new() -> Self {
        CalibrationToken { _0: () }
    }

    pub fn finish(self, adc: Uncalibrated) -> Calibrated {
        Calibrated::new(self, adc)
    }
}

/// Calibrated ADC
pub struct Calibrated {
    _0: ()
}

impl Calibrated {
    /// Creates a new calibrated ADC instance
    fn new(_ct: CalibrationToken, adc: Uncalibrated) -> Self {
        Calibrated { _0: () }
    }

    /// Begins a single analog conversion
    pub fn single<P>(self, pin: P) -> SingleConversion<P>
        where P: AnalogPin
    {
        SingleConversion::new(self, pin)
    }

    /// Begins a conversion sequence
    pub fn sequence(self, seq: ConversionSequence) -> SequencedConversion {
        SequencedConversion::new(self, seq)
    }
}

/// ADC conversion of a single pin
pub struct SingleConversion<P> where P: AnalogPin {
    adc: Calibrated,
    pin: P,
}

impl<P: AnalogPin> SingleConversion<P> {
    /// Begins a new conversion
    ///
    /// This can either be created directly by way of this method, or by calling
    /// the "single" method on the Calibrated instance
    pub fn new(adc: Calibrated, pin: P) -> Self {
        // adc is the proxy for ADC and we own it now. We are the proxy.
        // Clear the EOC flag
        unsafe { (*ADC::ptr()).isr.write(|w| w.eoc().bit(true)) };
        // Set the channel
        let channel_mask = 1 << P::CHANNEL;
        unsafe { (*ADC::ptr()).chselr.write(|w| w.bits(channel_mask)) }
        // Start the conversion
        unsafe { (*ADC::ptr()).cr.modify(|_, w| w.adstart().bit(true)) }
        SingleConversion { adc: adc, pin: pin }
    }

    /// Polls the conversion to completion
    pub fn poll(&mut self) -> nb::Result<ConversionComplete<P>, !> {
        // We own the proxy, so these accesses are safe
        if unsafe { (*ADC::ptr()).isr.read().eoc().bit() } {
            // The conversion is complete
            Ok(ConversionComplete::new())
        }
        else {
            Err(nb::Error::WouldBlock)
        }
    }
}

/// Conversion complete token.
///
/// This token is created when a conversion for a pin has been completed
pub struct ConversionComplete<P> where P: AnalogPin {
    _0: PhantomData<P>
}

impl<P: AnalogPin> ConversionComplete<P> {
    /// Creates a new conversion complete token
    fn new() -> Self {
        ConversionComplete { _0: PhantomData }
    }
    /// Finishes a single conversion
    ///
    /// This returns the ADC object, the pin that was being used for conversion,
    /// and the value
    pub fn finish(self, conv: SingleConversion<P>) -> (Calibrated, P, AdcValue) {
        // we have the conv, which has the adc, so this is safe
        let value = unsafe { AdcValue::new() };
        (conv.adc, conv.pin, value)
    }
}

/// Scan direction to use during sequenced conversion
#[derive(Copy, Clone)]
pub enum ScanDirection {
    /// Ascending scan direction
    Ascending,
    /// Descending scan direction
    Descending,
}

/// Represents a sequenced conversion
///
/// This consumes pins and is in turn consumed by the Initialized ADC to perform a sequenced
/// conversion.
pub struct ConversionSequence {
    channel_mask: u32,
    scan: ScanDirection,
}

impl ConversionSequence {
    /// Creates a new empty conversion sequence
    pub fn new() -> Self {
        ConversionSequence { channel_mask: 0, scan: ScanDirection::Ascending }
    }

    pub fn scan_direction(&self) -> ScanDirection {
        self.scan
    }

    /// Sets the scan direction for this conversion sequence
    pub fn set_scan_direction(&mut self, d: ScanDirection) {
        self.scan = d;
    }

    /// Adds a channel to this conversion
    ///
    /// Channels are added to the mask and will be sample in ascending order, not order of adding.
    /// This is due to the way the STM32 ADC works.
    ///
    /// This returns a `PinToken<P>` which can be used to restore the pin and remove it from this
    /// conversion sequence.
    pub fn add<P>(&mut self, pin: P) -> PinToken<P> where P: AnalogPin {
        // There should be a one-to-one relationship between pins and channels, so we do not have
        // fear of issuing a pintoken for a duplicate channel. In addition, the pintoken must be
        // released by this conversion sequence before it could be used in another (since `add`
        // only takes in `AnalogPin`.
        self.channel_mask |= 1 << P::CHANNEL;
        PinToken::new(pin)
    }

    /// Removes a channel from this conversion
    ///
    /// If the channel is not part of this conversion sequence, the `PinToken<P>` is passed back to
    /// the caller inside an Err.
    pub fn remove<P>(&mut self, token: PinToken<P>) -> Result<P, PinToken<P>> where P: AnalogPin {
        let pin_mask = 1 << P::CHANNEL;
        if self.channel_mask & pin_mask > 0 {
            Ok(token.pin())
        }
        else {
            Err(token)
        }
    }

    /// Gets the mask for this conversion sequence
    fn channel_mask(&self) -> u32 {
        self.channel_mask
    }
}

/// Token object used for interacting with conversions in a conversion sequence
/// for a particular pin.
///
/// This serves the purpose of preventing the `AnalogPin` `P` from being used in any other
/// conversion sequences.
pub struct PinToken<P> where P: AnalogPin {
    pin: P,
}

impl<P: AnalogPin> PinToken<P> {
    /// Creates a new pin token for a pin
    fn new(pin: P) -> Self {
        PinToken { pin: pin }
    }

    /// Gets the pin from this token, consuming it in the process
    ///
    /// This is safe because if this is accidentally called, there will still only be one `P`
    /// in existence.
    fn pin(self) -> P {
        self.pin
    }
}

/// ADC conversion of multiple pins in a sequence
pub struct SequencedConversion {
    adc: Calibrated,
    sequence: ConversionSequence,
}

impl SequencedConversion {
    /// Begins a new sequenced conversion
    ///
    /// This can also be performed by invoking the `Calibrated::sequence` method.
    pub fn new(adc: Calibrated, sequence: ConversionSequence) -> Self {
        // adc is the proxy for ADC and we own it now. We are the proxy.
        // Clear the EOC flag
        unsafe { (*ADC::ptr()).isr.write(|w| w.eoc().bit(true)) };
        // Set the channel
        let channel_mask = sequence.channel_mask();
        unsafe { (*ADC::ptr()).chselr.write(|w| w.bits(channel_mask)) }
        // Start the conversion
        unsafe { (*ADC::ptr()).cr.modify(|_, w| w.adstart().bit(true)) }
        SequencedConversion { adc: adc, sequence: sequence }
    }
}

