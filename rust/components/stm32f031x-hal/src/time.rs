//! Units of time for the HAL
//!
//! Based on the STM32F103xx HAL

/// Hertz
#[derive(Clone, Copy)]
pub struct Hertz(pub u32);

#[derive(Clone, Copy)]
pub struct KiloHertz(pub u32);

#[derive(Clone, Copy)]
pub struct MegaHertz(pub u32);

/// Extension trait for extending the u32 type conveniently
pub trait U32Ext {
    fn hz(self) -> Hertz;

    fn khz(self) -> KiloHertz;

    fn mhz(self) -> MegaHertz;
}

impl U32Ext for u32 {
    fn hz(self) -> Hertz {
        Hertz(self)
    }

    fn khz(self) -> KiloHertz {
        KiloHertz(self)
    }

    fn mhz(self) -> MegaHertz {
        MegaHertz(self)
    }
}

impl Into<Hertz> for KiloHertz {
    fn into(self) -> Hertz {
        Hertz(self.0 * 1000)
    }
}

impl Into<Hertz> for MegaHertz {
    fn into(self) -> Hertz {
        Hertz(self.0 * 1000000)
    }
}

impl Into<KiloHertz> for MegaHertz {
    fn into(self) -> KiloHertz {
        KiloHertz(self.0 * 1000)
    }
}

