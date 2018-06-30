#![doc = "Peripheral access API for STM32F031X microcontrollers (generated using svd2rust v0.13.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.13.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG_IRQ();
    fn PVD_IRQ();
    fn RTC_IRQ();
    fn FLASH_IRQ();
    fn RCC_IRQ();
    fn EXTI0_1_IRQ();
    fn EXTI2_3_IRQ();
    fn EXTI4_15_IRQ();
    fn TSC_IRQ();
    fn DMA_CH1_IRQ();
    fn DMA_CH2_3_IRQ();
    fn DMA_CH4_5_IRQ();
    fn ADC_COMP_IRQ();
    fn TIM1_BRK_UP_IRQ();
    fn TIM1_CC_IRQ();
    fn TIM2_IRQ();
    fn TIM3_IRQ();
    fn TIM14_IRQ();
    fn TIM16_IRQ();
    fn TIM17_IRQ();
    fn I2C1_IRQ();
    fn SPI1_IRQ();
    fn USART1_IRQ();
    fn CEC_IRQ();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 31] = [Vector { _handler: WWDG_IRQ }, Vector { _handler: PVD_IRQ }, Vector { _handler: RTC_IRQ }, Vector { _handler: FLASH_IRQ }, Vector { _handler: RCC_IRQ }, Vector { _handler: EXTI0_1_IRQ }, Vector { _handler: EXTI2_3_IRQ }, Vector { _handler: EXTI4_15_IRQ }, Vector { _handler: TSC_IRQ }, Vector { _handler: DMA_CH1_IRQ }, Vector { _handler: DMA_CH2_3_IRQ }, Vector { _handler: DMA_CH4_5_IRQ }, Vector { _handler: ADC_COMP_IRQ }, Vector { _handler: TIM1_BRK_UP_IRQ }, Vector { _handler: TIM1_CC_IRQ }, Vector { _handler: TIM2_IRQ }, Vector { _handler: TIM3_IRQ }, Vector { _reserved: 0 }, Vector { _reserved: 0 }, Vector { _handler: TIM14_IRQ }, Vector { _reserved: 0 }, Vector { _handler: TIM16_IRQ }, Vector { _handler: TIM17_IRQ }, Vector { _handler: I2C1_IRQ }, Vector { _reserved: 0 }, Vector { _handler: SPI1_IRQ }, Vector { _reserved: 0 }, Vector { _handler: USART1_IRQ }, Vector { _reserved: 0 }, Vector { _reserved: 0 }, Vector { _handler: CEC_IRQ }];
#[doc = r" Macro to override a device specific interrupt handler"]
#[doc = r""]
#[doc = r" # Syntax"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!("]
#[doc = r"     // Name of the interrupt"]
#[doc = r"     $Name:ident,"]
#[doc = r""]
#[doc = r"     // Path to the interrupt handler (a function)"]
#[doc = r"     $handler:path,"]
#[doc = r""]
#[doc = r"     // Optional, state preserved across invocations of the handler"]
#[doc = r"     state: $State:ty = $initial_state:expr,"]
#[doc = r" );"]
#[doc = r" ```"]
#[doc = r""]
#[doc = r" Where `$Name` must match the name of one of the variants of the `Interrupt`"]
#[doc = r" enum."]
#[doc = r""]
#[doc = r" The handler must have signature `fn()` is no state was associated to it;"]
#[doc = r" otherwise its signature must be `fn(&mut $State)`."]
#[cfg(feature = "rt")]
#[macro_export]
macro_rules! interrupt {
    ($Name:ident, $handler:path,state: $State:ty = $initial_state:expr) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            static mut STATE: $State = $initial_state;
            let _ = $crate::Interrupt::$Name;
            let f: fn(&mut $State) = $handler;
            f(&mut STATE)
        }
    };
    ($Name:ident, $handler:path) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            let _ = $crate::Interrupt::$Name;
            let f: fn() = $handler;
            f()
        }
    };
}
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - Window Watchdog interrupt"]
    WWDG_IRQ,
    #[doc = "1 - PVD through EXTI line detection"]
    PVD_IRQ,
    #[doc = "2 - RTC global interrupt"]
    RTC_IRQ,
    #[doc = "3 - Flash global interrupt"]
    FLASH_IRQ,
    #[doc = "4 - RCC global interrupt"]
    RCC_IRQ,
    #[doc = "5 - EXTI Line[1:0] interrupts"]
    EXTI0_1_IRQ,
    #[doc = "6 - EXTI Line[3:2] interrupts"]
    EXTI2_3_IRQ,
    #[doc = "7 - EXTI Line15 and EXTI4 interrupts"]
    EXTI4_15_IRQ,
    #[doc = "8 - Touch sensing interrupt"]
    TSC_IRQ,
    #[doc = "9 - DMA channel 1 interrupt"]
    DMA_CH1_IRQ,
    #[doc = "10 - DMA channel 2 and 3 interrupts"]
    DMA_CH2_3_IRQ,
    #[doc = "11 - DMA channel 4 and 5 interrupts"]
    DMA_CH4_5_IRQ,
    #[doc = "12 - ADC and comparator 1 and 2"]
    ADC_COMP_IRQ,
    #[doc = "13 - TIM1 Break, update, trigger and"]
    TIM1_BRK_UP_IRQ,
    #[doc = "14 - TIM1 Capture Compare interrupt"]
    TIM1_CC_IRQ,
    #[doc = "15 - TIM2 global interrupt"]
    TIM2_IRQ,
    #[doc = "16 - TIM3 global interrupt"]
    TIM3_IRQ,
    #[doc = "19 - TIM14 global interrupt"]
    TIM14_IRQ,
    #[doc = "21 - TIM16 global interrupt"]
    TIM16_IRQ,
    #[doc = "22 - TIM17 global interrupt"]
    TIM17_IRQ,
    #[doc = "23 - I2C1 global interrupt"]
    I2C1_IRQ,
    #[doc = "25 - SPI1_global_interrupt"]
    SPI1_IRQ,
    #[doc = "27 - USART1 global interrupt"]
    USART1_IRQ,
    #[doc = "30 - CEC global interrupt"]
    CEC_IRQ,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::WWDG_IRQ => 0,
            Interrupt::PVD_IRQ => 1,
            Interrupt::RTC_IRQ => 2,
            Interrupt::FLASH_IRQ => 3,
            Interrupt::RCC_IRQ => 4,
            Interrupt::EXTI0_1_IRQ => 5,
            Interrupt::EXTI2_3_IRQ => 6,
            Interrupt::EXTI4_15_IRQ => 7,
            Interrupt::TSC_IRQ => 8,
            Interrupt::DMA_CH1_IRQ => 9,
            Interrupt::DMA_CH2_3_IRQ => 10,
            Interrupt::DMA_CH4_5_IRQ => 11,
            Interrupt::ADC_COMP_IRQ => 12,
            Interrupt::TIM1_BRK_UP_IRQ => 13,
            Interrupt::TIM1_CC_IRQ => 14,
            Interrupt::TIM2_IRQ => 15,
            Interrupt::TIM3_IRQ => 16,
            Interrupt::TIM14_IRQ => 19,
            Interrupt::TIM16_IRQ => 21,
            Interrupt::TIM17_IRQ => 22,
            Interrupt::I2C1_IRQ => 23,
            Interrupt::SPI1_IRQ => 25,
            Interrupt::USART1_IRQ => 27,
            Interrupt::CEC_IRQ => 30,
        }
    }
}
#[doc(hidden)]
pub mod interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[doc = "cyclic redundancy check calculation unit"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crc::RegisterBlock {
        1073885184 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    fn deref(&self) -> &crc::RegisterBlock {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "cyclic redundancy check calculation unit"]
pub mod crc;
#[doc = "General-purpose I/Os"]
pub struct GPIOF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOF {}
impl GPIOF {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiof::RegisterBlock {
        1207964672 as *const _
    }
}
impl Deref for GPIOF {
    type Target = gpiof::RegisterBlock;
    fn deref(&self) -> &gpiof::RegisterBlock {
        unsafe { &*GPIOF::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpiof;
#[doc = "GPIOC"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiof::RegisterBlock {
        1207961600 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpiof::RegisterBlock;
    fn deref(&self) -> &gpiof::RegisterBlock {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "GPIOB"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiof::RegisterBlock {
        1207960576 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpiof::RegisterBlock;
    fn deref(&self) -> &gpiof::RegisterBlock {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1207959552 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioa;
#[doc = "Serial peripheral interface"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073819648 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Serial peripheral interface"]
pub mod spi1;
#[doc = "Power control"]
pub struct PWR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWR {}
impl PWR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwr::RegisterBlock {
        1073770496 as *const _
    }
}
impl Deref for PWR {
    type Target = pwr::RegisterBlock;
    fn deref(&self) -> &pwr::RegisterBlock {
        unsafe { &*PWR::ptr() }
    }
}
#[doc = "Power control"]
pub mod pwr;
#[doc = "Inter-integrated circuit"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c1::RegisterBlock {
        1073763328 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Inter-integrated circuit"]
pub mod i2c1;
#[doc = "Independent watchdog"]
pub struct IWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IWDG {}
impl IWDG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const iwdg::RegisterBlock {
        1073754112 as *const _
    }
}
impl Deref for IWDG {
    type Target = iwdg::RegisterBlock;
    fn deref(&self) -> &iwdg::RegisterBlock {
        unsafe { &*IWDG::ptr() }
    }
}
#[doc = "Independent watchdog"]
pub mod iwdg;
#[doc = "Window watchdog"]
pub struct WWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDG {}
impl WWDG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wwdg::RegisterBlock {
        1073753088 as *const _
    }
}
impl Deref for WWDG {
    type Target = wwdg::RegisterBlock;
    fn deref(&self) -> &wwdg::RegisterBlock {
        unsafe { &*WWDG::ptr() }
    }
}
#[doc = "Window watchdog"]
pub mod wwdg;
#[doc = "Advanced-timers"]
pub struct TIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM1 {}
impl TIM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim1::RegisterBlock {
        1073818624 as *const _
    }
}
impl Deref for TIM1 {
    type Target = tim1::RegisterBlock;
    fn deref(&self) -> &tim1::RegisterBlock {
        unsafe { &*TIM1::ptr() }
    }
}
#[doc = "Advanced-timers"]
pub mod tim1;
#[doc = "General-purpose-timers"]
pub struct TIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM2 {}
impl TIM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim2::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for TIM2 {
    type Target = tim2::RegisterBlock;
    fn deref(&self) -> &tim2::RegisterBlock {
        unsafe { &*TIM2::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod tim2;
#[doc = "TIM3"]
pub struct TIM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM3 {}
impl TIM3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim2::RegisterBlock {
        1073742848 as *const _
    }
}
impl Deref for TIM3 {
    type Target = tim2::RegisterBlock;
    fn deref(&self) -> &tim2::RegisterBlock {
        unsafe { &*TIM3::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub struct TIM14 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM14 {}
impl TIM14 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim14::RegisterBlock {
        1073750016 as *const _
    }
}
impl Deref for TIM14 {
    type Target = tim14::RegisterBlock;
    fn deref(&self) -> &tim14::RegisterBlock {
        unsafe { &*TIM14::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod tim14;
#[doc = "External interrupt/event controller"]
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTI {}
impl EXTI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const exti::RegisterBlock {
        1073808384 as *const _
    }
}
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    fn deref(&self) -> &exti::RegisterBlock {
        unsafe { &*EXTI::ptr() }
    }
}
#[doc = "External interrupt/event controller"]
pub mod exti;
#[doc = "DMA controller"]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    fn deref(&self) -> &dma::RegisterBlock {
        unsafe { &*DMA::ptr() }
    }
}
#[doc = "DMA controller"]
pub mod dma;
#[doc = "Reset and clock control"]
pub struct RCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCC {}
impl RCC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rcc::RegisterBlock {
        1073876992 as *const _
    }
}
impl Deref for RCC {
    type Target = rcc::RegisterBlock;
    fn deref(&self) -> &rcc::RegisterBlock {
        unsafe { &*RCC::ptr() }
    }
}
#[doc = "Reset and clock control"]
pub mod rcc;
#[doc = "System configuration controller"]
pub struct SYSCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCFG {}
impl SYSCFG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const syscfg::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for SYSCFG {
    type Target = syscfg::RegisterBlock;
    fn deref(&self) -> &syscfg::RegisterBlock {
        unsafe { &*SYSCFG::ptr() }
    }
}
#[doc = "System configuration controller"]
pub mod syscfg;
#[doc = "Analog-to-digital converter"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc::RegisterBlock {
        1073816576 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    fn deref(&self) -> &adc::RegisterBlock {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "Analog-to-digital converter"]
pub mod adc;
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073821696 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod usart1;
#[doc = "Real-time clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1073752064 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-time clock"]
pub mod rtc;
#[doc = "General-purpose-timers"]
pub struct TIM16 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM16 {}
impl TIM16 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim16::RegisterBlock {
        1073824768 as *const _
    }
}
impl Deref for TIM16 {
    type Target = tim16::RegisterBlock;
    fn deref(&self) -> &tim16::RegisterBlock {
        unsafe { &*TIM16::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod tim16;
#[doc = "TIM17"]
pub struct TIM17 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM17 {}
impl TIM17 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim16::RegisterBlock {
        1073825792 as *const _
    }
}
impl Deref for TIM17 {
    type Target = tim16::RegisterBlock;
    fn deref(&self) -> &tim16::RegisterBlock {
        unsafe { &*TIM17::ptr() }
    }
}
#[doc = "Touch sensing controller"]
pub struct TSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TSC {}
impl TSC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tsc::RegisterBlock {
        1073889280 as *const _
    }
}
impl Deref for TSC {
    type Target = tsc::RegisterBlock;
    fn deref(&self) -> &tsc::RegisterBlock {
        unsafe { &*TSC::ptr() }
    }
}
#[doc = "Touch sensing controller"]
pub mod tsc;
#[doc = "HDMI-CEC controller"]
pub struct CEC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CEC {}
impl CEC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cec::RegisterBlock {
        1073772544 as *const _
    }
}
impl Deref for CEC {
    type Target = cec::RegisterBlock;
    fn deref(&self) -> &cec::RegisterBlock {
        unsafe { &*CEC::ptr() }
    }
}
#[doc = "HDMI-CEC controller"]
pub mod cec;
#[doc = "Flash"]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const flash::RegisterBlock {
        1073881088 as *const _
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    fn deref(&self) -> &flash::RegisterBlock {
        unsafe { &*FLASH::ptr() }
    }
}
#[doc = "Flash"]
pub mod flash;
#[doc = "Debug support"]
pub struct DBGMCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DBGMCU {}
impl DBGMCU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dbgmcu::RegisterBlock {
        1073829888 as *const _
    }
}
impl Deref for DBGMCU {
    type Target = dbgmcu::RegisterBlock;
    fn deref(&self) -> &dbgmcu::RegisterBlock {
        unsafe { &*DBGMCU::ptr() }
    }
}
#[doc = "Debug support"]
pub mod dbgmcu;
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "GPIOF"]
    pub GPIOF: GPIOF,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "PWR"]
    pub PWR: PWR,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "IWDG"]
    pub IWDG: IWDG,
    #[doc = "WWDG"]
    pub WWDG: WWDG,
    #[doc = "TIM1"]
    pub TIM1: TIM1,
    #[doc = "TIM2"]
    pub TIM2: TIM2,
    #[doc = "TIM3"]
    pub TIM3: TIM3,
    #[doc = "TIM14"]
    pub TIM14: TIM14,
    #[doc = "EXTI"]
    pub EXTI: EXTI,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "RCC"]
    pub RCC: RCC,
    #[doc = "SYSCFG"]
    pub SYSCFG: SYSCFG,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "TIM16"]
    pub TIM16: TIM16,
    #[doc = "TIM17"]
    pub TIM17: TIM17,
    #[doc = "TSC"]
    pub TSC: TSC,
    #[doc = "CEC"]
    pub CEC: CEC,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "DBGMCU"]
    pub DBGMCU: DBGMCU,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| if unsafe { DEVICE_PERIPHERALS } { None } else { Some(unsafe { Peripherals::steal() }) })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            CRC: CRC { _marker: PhantomData },
            GPIOF: GPIOF { _marker: PhantomData },
            GPIOC: GPIOC { _marker: PhantomData },
            GPIOB: GPIOB { _marker: PhantomData },
            GPIOA: GPIOA { _marker: PhantomData },
            SPI1: SPI1 { _marker: PhantomData },
            PWR: PWR { _marker: PhantomData },
            I2C1: I2C1 { _marker: PhantomData },
            IWDG: IWDG { _marker: PhantomData },
            WWDG: WWDG { _marker: PhantomData },
            TIM1: TIM1 { _marker: PhantomData },
            TIM2: TIM2 { _marker: PhantomData },
            TIM3: TIM3 { _marker: PhantomData },
            TIM14: TIM14 { _marker: PhantomData },
            EXTI: EXTI { _marker: PhantomData },
            DMA: DMA { _marker: PhantomData },
            RCC: RCC { _marker: PhantomData },
            SYSCFG: SYSCFG { _marker: PhantomData },
            ADC: ADC { _marker: PhantomData },
            USART1: USART1 { _marker: PhantomData },
            RTC: RTC { _marker: PhantomData },
            TIM16: TIM16 { _marker: PhantomData },
            TIM17: TIM17 { _marker: PhantomData },
            TSC: TSC { _marker: PhantomData },
            CEC: CEC { _marker: PhantomData },
            FLASH: FLASH { _marker: PhantomData },
            DBGMCU: DBGMCU { _marker: PhantomData },
        }
    }
}
