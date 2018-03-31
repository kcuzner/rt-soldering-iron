//! I2C abstraction

use stm32f031x::{I2C1, i2c1};

use nb;

use gpio::{gpioa, gpiob, AF1, AF4, OpenDrain, IntoAlternate};
use gpio::gpioa::{PA9, PA10};
use gpio::gpiob::{PB6, PB7};
use rcc::APB1;

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

pub struct BoundI2c {
    _0: ()
}

/// I2C peripheral that has been bound to pins
impl BoundI2c {
    /// Configures the peripheral as a master
    pub fn master(self) -> MasterI2c {
        unsafe { (*I2C1::ptr()).timingr.modify(|_, w| w.bits(0)) };
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

impl MasterI2cError {
    /// Recovers the peripheral from an error and prepares it for new transactions
    pub fn recover(self) -> MasterI2c {
        MasterI2c { _0: () }
    }
}

/// Common function for querying for a mask to appear on the ISR. This is unsafe
/// because the caller needs to guarantee exclusive access to the I2C peripheral
/// before calling.
pub unsafe fn i2c_wait_for_mask<F>(mask_fn: F) -> nb::Result<(), MasterI2cError>
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
        return Ok(())
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
    /// At the end of the write, the MasterI2c will be re-instantiated.
    pub fn begin_write<'a>(self, addr: u8, data: &'a [u8]) -> MasterWriteTransaction<'a> {
        unsafe { (*I2C1::ptr()).cr1.write(|w| w.pe().bit(true)) }
        // set up address, register byte, and buffer data
        unsafe { (*I2C1::ptr()).cr2.write(|w| w.autoend().bit(true)
                                          .nbytes().bits(data.len() as u8)
                                          .start().bit(true)
                                          .sadd8().bits(addr)) }
        unsafe { (*I2C1::ptr()).isr.write(|w| w.txe().bit(true)) }
        unsafe { (*I2C1::ptr()).txdr.write(|w| w.txdata().bits(data[0])) }
        MasterWriteTransaction { data: data, index: 1 }
    }
}

pub enum MasterWriteResult<'a> {
    Continue(MasterWriteTransaction<'a>),
    Done(MasterI2c)
}

pub struct MasterWriteTransaction<'a> {
    data: &'a [u8],
    index: usize,
}

impl<'a> MasterWriteTransaction<'a> {
    /// Attempts to end this write. This should be called periodically in order
    /// to keep the transaction going.
    pub fn end_write(&mut self) -> nb::Result<(), MasterI2cError> {
        match unsafe {i2c_wait_for_mask(|r| r.stopf().bit() || r.txis().bit()) } {
            Err(nb::Error::WouldBlock) => return Err(nb::Error::WouldBlock), 
            Err(nb::Error::Other(e)) => return Err(nb::Error::Other(e)),
            Ok(()) => {
                unsafe { (*I2C1::ptr()).txdr.write(|w| w.txdata().bits(self.data[self.index])) }
                self.index += 1;
            }
        }
        if unsafe { (*I2C1::ptr()).isr.read().stopf().bit() } {
            return Ok(());
        }
        else {
            return Err(nb::Error::WouldBlock);
        }
    }

    /// Finishes this transaction, whether or not the write has been completed.
    /// This method must be called in order to perform another transaction.
    pub fn finish(self) -> MasterI2c {
        unsafe { (*I2C1::ptr()).cr1.write(|w| w) }
        MasterI2c { _0: () }
    }
}

