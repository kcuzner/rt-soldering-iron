//! RCC abstraction
//!
//! Based on the STM32F103xx HAL
//!

use core::cmp;

use stm32f031x::{rcc, RCC};

use flash::ACR;
use time::{Hertz, U32Ext};

pub trait RccExt {
    /// Consumes this RCC into an `Rcc` peripheral
    fn constrain(self) -> Rcc;
}

impl RccExt for RCC {
    fn constrain(self) -> Rcc {
        Rcc {
            ahb: AHB { _0: () },
            apb1: APB1 { _0: () },
            apb2: APB2 { _0: () },
            cfgr: CFGR {
                hclk: None,
                pclk: None,
                sysclk: None,
            }
        }
    }
}

/// Reset and Clock Control abstraction
pub struct Rcc {
    pub ahb: AHB,
    pub apb1: APB1,
    pub apb2: APB2,
    pub cfgr: CFGR
}

pub struct AHB {
    _0: (),
}

impl AHB {
    //pub(crate) fn enr(&mut self) -> &rcc::AHBENR {
    pub fn enr(&mut self) -> &rcc::AHBENR {
        // This proxy is the only way to access this register and is thus
        // exclusive (note the mut self)
        unsafe { &(*RCC::ptr()).ahbenr }
    }
}

pub struct APB1 {
    _0: (),
}

impl APB1 {
    pub(crate) fn enr(&mut self) -> &rcc::APB1ENR {
        // This proxy is the only way to access this register and is thus
        // exclusivel (note the mut self)
        unsafe { &(*RCC::ptr()).apb1enr }
    }
}

pub struct APB2 {
    _0: (),
}

impl APB2 {
    //pub(crate) fn enr(&mut self) -> &rcc::APB2ENR {
    pub fn enr(&mut self) -> &rcc::APB2ENR {
        // This proxy is the only way to access this register and is thus
        // exclusive (note the mut self)
        unsafe { &(*RCC::ptr()).apb2enr }
    }
}

const HSI: u32 = 8000000; //Hz

/// Uncommitted clock configuration
pub struct CFGR {
    pub hclk: Option<u32>,
    pub pclk: Option<u32>,
    pub sysclk: Option<u32>,
}

impl CFGR {
    pub fn hclk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>
    {
        self.hclk = Some(freq.into().0);
        self
    }

    pub fn pclk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>
    {
        self.pclk = Some(freq.into().0);
        self
    }

    pub fn sysclk<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>
    {
        self.sysclk = Some(freq.into().0);
        self
    }

    pub fn freeze(self, acr: &mut ACR) -> Clocks {
        let pllmul = (4 * self.sysclk.unwrap_or(HSI) + HSI) / HSI / 2;
        let pllmul = cmp::min(cmp::max(pllmul, 2), 16);
        let pllmul_bits = if pllmul == 2 {
            None
        } else {
            Some(pllmul as u8 - 2)
        };

        let sysclk = pllmul * HSI / 2;

        assert!(sysclk < 48000000);

        let hpre_bits = self.hclk
            .map(|hclk| match sysclk / hclk {
                0 => unreachable!(),
                1 => 0b0111,
                2 => 0b1000,
                3...5 => 0b1001,
                6...11 => 0b1010,
                12...39 => 0b1011,
                40...95 => 0b1100,
                96...191 => 0b1101,
                192...383 => 0b1110,
                _ => 0b1111
            })
            .unwrap_or(0b0111);

        let hclk = sysclk / (1 << (hpre_bits - 0b0111));

        assert!(hclk < 48000000);

        let ppre_bits = self.pclk
            .map(|pclk| match hclk / pclk {
                0 => unreachable!(),
                1 => 0b011,
                2 => 0b100,
                3...5 => 0b101,
                6...11 => 0b110,
                _ => 0b111
            })
            .unwrap_or(0b011);

        let pclk = hclk / (1 << (ppre_bits - 0b011));

        assert!(pclk < 48000000);

        //adjust flash wait states
        unsafe {
            acr.acr().write(|w| {
                w.latency().bits(if sysclk <= 24000000 {
                    0b000
                } else {
                    0b001
                })
            })
        };

        let rcc = unsafe { &*RCC::ptr() };
        if let Some(pllmul_bits) = pllmul_bits {
            // use PLL as source from HSI
            // TODO: HSE
            rcc.cfgr.write(|w| unsafe { w.pllmul().bits(pllmul_bits) });
            rcc.cr.write(|w| w.pllon().bit(true));
            while rcc.cr.read().pllrdy().bit() { }
            rcc.cfgr.modify(|_, w| unsafe {
                w.ppre().bits(ppre_bits)
                    .hpre().bits(hpre_bits)
                    .sw().bits(0b10)
            });
        }
        else {
            rcc.cfgr.write(|w| unsafe {
                w.ppre().bits(ppre_bits)
                    .hpre().bits(hpre_bits)
                    .sw().bits(0b00)
            });
        }

        Clocks {
            hclk: hclk.hz(),
            pclk: pclk.hz(),
            sysclk: sysclk.hz(),
            ppre: ppre_bits,
        }
    }
}

/// Frozen clock speeds. The presence of this struct indicates that the clock state can no longer
/// be changed
#[derive(Copy, Clone)]
pub struct Clocks {
    hclk: Hertz,
    pclk: Hertz,
    sysclk: Hertz,
    ppre: u8
}

impl Clocks {
    pub fn hclk(&self) -> Hertz {
        self.hclk
    }

    pub fn pclk(&self) -> Hertz {
        self.pclk
    }

    pub fn sysclk(&self) -> Hertz {
        self.sysclk
    }

    pub(crate) fn ppre(&self) -> u8 {
        self.ppre
    }
}


