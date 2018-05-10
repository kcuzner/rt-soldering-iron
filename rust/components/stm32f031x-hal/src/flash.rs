//! Flash abstraction
//!
//! Based on the STM32F103xx HAL
//!

use stm32f031x::{flash, FLASH};

pub trait FlashExt {
    fn constrain(self) -> Parts;
}

impl FlashExt for FLASH {
    fn constrain(self) -> Parts {
        Parts {
            acr: ACR { _0: () }
        }
    }
}

pub struct Parts {
    pub acr: ACR
}

pub struct ACR {
    _0: (),
}

impl ACR {
    pub(crate) fn acr(&mut self) -> &flash::ACR {
        // This proxy grants exclusive access (&mut self)
        unsafe { &(*FLASH::ptr()).acr }
    }
}

