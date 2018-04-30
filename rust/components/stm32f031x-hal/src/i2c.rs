//! I2C abstraction

use stm32f031x::{I2C1, i2c1};

use nb;

use core::convert::From;
use take_mut;

use gpio::{gpioa, gpiob, AF1, AF4, OpenDrain, IntoAlternate};
use gpio::gpioa::{PA9, PA10};
use gpio::gpiob::{PB6, PB7};
use rcc::{APB1, Clocks};
use time::U32Ext;

/// Trait applied to pins that are SCL-capable
pub trait IntoScl<SclPin> {
    /// GPIO regs
    type Regs;

    /// Switches this pin into an scl pin
    fn into_scl(self, regs: &mut Self::Regs) -> SclPin;
}

/// Trait applied when a particular pin in a particular mode can be used as SCL
/// for a particular I2C block
pub unsafe trait SclPin<I2C> {
}

/// Trait applied to pins that are SDA-capable
pub trait IntoSda<SdaPin> {
    /// GPIO regs
    type Regs;

    /// Switches this pin into an SDA pin
    fn into_sda(self, regs: &mut Self::Regs) -> SdaPin;
}

/// Trait applied when a particular pin in a particular mode can be used as SDA
/// for a particular I2C block
pub unsafe trait SdaPin<I2C> {
}


pub trait I2CExt
{
    /// Bus type for the clock enables
    type Bus;

    type Constrained;

    fn constrain(self, bus: &mut Self::Bus) -> Self::Constrained;
}


//No macros for now...there's only one I2C on the STM32F031

/// PA9 is an I2C1_SCL pin
unsafe impl SclPin<I2C1> for PA9<AF4<OpenDrain>> {
}

/// PA9 can be configured to be an I2C1_SCL pin
impl<Mode> IntoScl<PA9<AF4<OpenDrain>>> for PA9<Mode> {
    type Regs = gpioa::Regs;
    fn into_scl(self, regs: &mut Self::Regs) -> PA9<AF4<OpenDrain>> {
        IntoAlternate::<AF4<OpenDrain>>::into_alternate(self, regs)
    }
}

/// PA10 is an I2C1_SDA pin
unsafe impl SdaPin<I2C1> for PA10<AF4<OpenDrain>> {
}

///PA10 can be configured to be an I2C1_SDA pin
impl<Mode> IntoSda<PA10<AF4<OpenDrain>>> for PA10<Mode> {
    type Regs = gpioa::Regs;
    fn into_sda(self, regs: &mut Self::Regs) -> PA10<AF4<OpenDrain>> {
        IntoAlternate::<AF4<OpenDrain>>::into_alternate(self, regs)
    }
}

/// PB6 is an I2C1_SCL pin
unsafe impl SclPin<I2C1> for PB6<AF1<OpenDrain>> {
}

/// PB6 can be configured to be an I2C1_SCL pin
impl<Mode> IntoScl<PB6<AF1<OpenDrain>>> for PB6<Mode> {
    type Regs = gpiob::Regs;
    fn into_scl(self, regs: &mut Self::Regs) -> PB6<AF1<OpenDrain>> {
        IntoAlternate::<AF1<OpenDrain>>::into_alternate(self, regs)
    }
}

/// PB7 is an I2C1_SDA pin
unsafe impl SdaPin<I2C1> for PB7<AF1<OpenDrain>> {
}

/// PB7 can be configured to be an I2C1_SDA pin
impl<Mode> IntoSda<PB7<AF1<OpenDrain>>> for PB7<Mode> {
    type Regs = gpiob::Regs;
    fn into_sda(self, regs: &mut Self::Regs) -> PB7<AF1<OpenDrain>> {
        IntoAlternate::<AF1<OpenDrain>>::into_alternate(self, regs)
    }
}

impl I2CExt for I2C1 {
    type Bus = APB1;

    type Constrained = I2c;

    fn constrain(self, bus: &mut APB1) -> I2c {
        bus.enr().modify(|_, w| w.i2c1en().bit(true));
        I2c { _0: () }
    }
}

/// I2c HAL for a particular peripheral
pub struct I2c {
    _0: ()
}

impl I2c {
    pub fn bind<Sda, Scl>(self, _scl: Scl, _sda: Sda) -> BoundI2c
    where Sda: SdaPin<I2C1>, Scl: SclPin<I2C1>
    {
        // we don't need to do anything in particular here. This is just to consume
        // both this object and the pins.
        BoundI2c { _0: () }
    }
}

/// I2C Predefined Timing Setting
pub enum I2cTimingSetting {
    Standard,
    Fast,
    FastPlus,
}

#[derive(Debug)]
pub enum I2cTimingError {
    BadSysclkSetting
}

/// Structure representing bus timing
pub struct I2cTiming {
    presc: u8,
    scll: u8,
    sclh: u8,
    sdadel: u8,
    scldel: u8,
}

impl I2cTiming {
    /// Creates a new I2C timing.
    ///
    /// Consider using From<I2cTimingSetting> instead of calling this function
    /// directly.
    pub fn new(presc: u8, scll: u8, sclh: u8, sdadel: u8, scldel: u8) -> Self {
        I2cTiming {
            presc: presc, scll: scll, sclh: sclh, sdadel: sdadel, scldel: scldel,
        }
    }
}

impl From<I2cTimingSetting> for I2cTiming {
    /// Creates a timing from a predefined value.
    ///
    /// This uses the datasheet-provided presets for various sysclk frequencies.
    /// Since CFGR3 isn't exposed in this HAL, we assume that HSI is the
    /// I2C clock. Based on the passed clocks and the passed timing setting,
    /// this creates an I2cTiming which represents the required settings for
    /// the I2C peripheral to meet that timing.
    fn from(setting: I2cTimingSetting) -> I2cTiming {
        match setting {
            I2cTimingSetting::Standard => I2cTiming::new(1, 0x13, 0xf, 0x2, 0x4),
            I2cTimingSetting::Fast => I2cTiming::new(0, 0x9, 0x3, 0x1, 0x3),
            I2cTimingSetting::FastPlus => I2cTiming::new(0, 0x6, 3, 0x0, 0x1),
        }
    }
}

/// Trait for registers that can have timing applied
trait ApplyTiming {
    /// Applies timing to this register
    fn apply_timing(&mut self, timing: I2cTiming) -> &mut Self;
}

impl ApplyTiming for i2c1::timingr::W {
    fn apply_timing(&mut self, timing: I2cTiming) -> &mut Self {
        unsafe {
            self.presc().bits(timing.presc)
                .scll().bits(timing.scll)
                .sclh().bits(timing.sclh)
                .sdadel().bits(timing.sdadel)
                .scldel().bits(timing.scldel)
        }
    }
}

pub struct BoundI2c {
    _0: ()
}

/// I2C peripheral that has been bound to pins
impl BoundI2c {
    /// Configures the peripheral as a master
    pub fn master(self, timing: I2cTiming) -> MasterI2c {
        unsafe { (*I2C1::ptr()).timingr.modify(|_, w| w.apply_timing(timing)) };
        MasterI2c { _0: () }
    }
}

/// I2C error that occurs in master mode
#[derive(Debug)]
pub enum MasterI2cError {
    ArbitrationLoss,
    BusError,
    Nack
}

/// Common function for querying for a mask to appear on the ISR. This is unsafe
/// because the caller needs to guarantee exclusive access to the I2C peripheral
/// before calling.
pub unsafe fn i2c_wait_for_mask<F>(mask_fn: F) -> nb::Result<i2c1::isr::R, MasterI2cError>
where F: FnOnce(&i2c1::isr::R) -> bool
{
    let r = (*I2C1::ptr()).isr.read();
    if r.arlo().bit() {
        return Err(nb::Error::Other(MasterI2cError::ArbitrationLoss));
    }
    else if r.berr().bit() {
        return Err(nb::Error::Other(MasterI2cError::BusError));
    }
    else if r.nackf().bit() {
        return Err(nb::Error::Other(MasterI2cError::Nack));
    }
    else if mask_fn(&r) {
        return Ok(r)
    }
    return Err(nb::Error::WouldBlock);
}

pub struct MasterI2c {
    _0: ()
}

impl MasterI2c {
    /// Begins a write transaction with this peripheral. This consumes the proxy
    /// so that the write transaction is the exclusive operation going on.
    ///
    /// The MasterWrite state is advanced by calling its poll() function. The
    /// result of this function is a "next" struct which consumes the MasterWrite
    /// and produces a struct for the next state of the transaction.
    ///
    /// When poll() produces a MasterWriteResult::Continue, the inner MasterNextWrite
    /// consumes the existing MasterWrite along with the next byte to send.
    ///
    /// When poll() produces a MasterWriteResult::Done, the passed number of bytes
    /// has been sent and the inner MasterNextDone consumes the MasterWrite
    /// to re-instantiate the MasterI2c.
    ///
    /// At any time, the finish() function on the MasterWrite may be called to
    /// abort the write and re-instantiate the MasterI2c.
    ///
    /// addr: 7-bit address
    /// len: Length of transaction
    pub fn begin_write(self, addr: u8, len: u8) -> MasterWrite {
        unsafe { (*I2C1::ptr()).cr1.write(|w| w.pe().bit(true)) }
        // set up address, register byte, and buffer data
        unsafe { (*I2C1::ptr()).cr2.write(|w| w.autoend().bit(true)
                                          .nbytes().bits(len)
                                          .start().bit(true)
                                          .sadd1().bits(addr >> 1)) }
        unsafe { (*I2C1::ptr()).isr.write(|w| w.txe().bit(true)) }
        MasterWrite::new(len)
    }
}

/// Ongoing master write transaction
pub struct MasterWrite {
    remaining: u8,
}

/// When polling concludes, this is produced and will determine the valid next
/// state for the master transaction.
pub enum MasterWriteResult {
    /// Advance the data to the next byte
    Advance(MasterWriteAdvance),
    /// The transaction is complete and no further bytes will be accepted. The
    /// caller should call MasterWrite::finish
    Complete,
}

impl MasterWrite {
    /// Creates a new write state machine
    fn new(remaining: u8) -> Self {
        MasterWrite { remaining: remaining }
    }
    /// Polls the master to determine the next state of this write
    pub fn poll(&mut self) -> nb::Result<MasterWriteResult, MasterI2cError> {
        // Safe, because this is the exclusive proxy to access I2C1.
        match unsafe { i2c_wait_for_mask(|r| r.stopf().bit() || r.txis().bit()) } {
            Err(nb::Error::WouldBlock) => return Err(nb::Error::WouldBlock),
            Err(nb::Error::Other(e)) => return Err(nb::Error::Other(e)),
            Ok(r) => {
                if r.stopf().bit() {
                    return Ok(MasterWriteResult::Complete);
                }
                else if self.remaining > 0 {
                    return Ok(MasterWriteResult::Advance(MasterWriteAdvance::new()));
                }
                else {
                    unreachable!();
                }
            }
        }
    }

    /// Completes or aborts this transaction. May be called at any time.
    pub fn finish(self) -> MasterI2c {
        unsafe { (*I2C1::ptr()).cr1.write(|w| w.bits(0)) }
        MasterI2c { _0: () }
    }
}

pub struct MasterWriteAdvance {
    _0: ()
}

impl MasterWriteAdvance {
    /// Create a new masterwriteadvance. Only use in this mod.
    fn new() -> Self {
        MasterWriteAdvance { _0: () }
    }
    /// Advance the write to the next byte
    pub fn advance(self, write: MasterWrite, data: u8) -> MasterWrite {
        unsafe { (*I2C1::ptr()).txdr.write(|w| w.txdata().bits(data)) }
        MasterWrite { remaining: write.remaining - 1 }
    }
}

/// Allows writing of static slices to the I2C peripheral
///
/// Currently, this only supports statics so that we don't accidentally run
/// into "cannot borrow across yield" errors if we use Generators.
struct I2cSliceWrite {
    write: MasterWrite,
    data: &'static [u8],
    index: usize,
}

impl I2cSliceWrite {
    /// Creates a new static write transaction
    fn new(master: MasterI2c, addr: u8, data: &'static [u8]) -> Self {
        I2cSliceWrite {
            write: master.begin_write(addr, data.len() as u8),
            data: data,
            index: 0,
        }
    }

    /// Nonblocking poll funciton for this write tarnsaction
    fn poll(&mut self) -> nb::Result<(), MasterI2cError> {
        match self.write.poll() {
            Err(nb::Error::WouldBlock) => Err(nb::Error::WouldBlock),
            Err(nb::Error::Other(e)) => Err(nb::Error::Other(e)),
            Ok(r) => match r {
                MasterWriteResult::Complete => Ok(()),
                MasterWriteResult::Advance(t) => {
                    let byte = self.data[self.index];
                    take_mut::take(&mut self.write, |w| {
                        t.advance(w, byte)
                    });
                    self.index += 1;
                    Err(nb::Error::WouldBlock)
                }
            }
        }
    }

    /// Finishes or aborts this transaction
    fn finish(self) -> MasterI2c {
        self.write.finish()
    }
}

