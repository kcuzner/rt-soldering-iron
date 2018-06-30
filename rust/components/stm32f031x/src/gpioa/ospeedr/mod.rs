#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OSPEEDR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `OSPEEDR15`"]
pub type OSPEEDR15R = ::gpiof::ospeedr::OSPEEDR15R;
#[doc = "Possible values of the field `OSPEEDR14`"]
pub type OSPEEDR14R = ::gpiof::ospeedr::OSPEEDR15R;
#[doc = "Possible values of the field `OSPEEDR13`"]
pub type OSPEEDR13R = ::gpiof::ospeedr::OSPEEDR15R;
#[doc = "Possible values of the field `OSPEEDR12`"]
pub type OSPEEDR12R = ::gpiof::ospeedr::OSPEEDR15R;
#[doc = "Possible values of the field `OSPEEDR11`"]
pub type OSPEEDR11R = ::gpiof::ospeedr::OSPEEDR15R;
#[doc = "Possible values of the field `OSPEEDR10`"]
pub type OSPEEDR10R = ::gpiof::ospeedr::OSPEEDR15R;
#[doc = "Possible values of the field `OSPEEDR9`"]
pub type OSPEEDR9R = ::gpiof::ospeedr::OSPEEDR15R;
#[doc = "Possible values of the field `OSPEEDR8`"]
pub type OSPEEDR8R = ::gpiof::ospeedr::OSPEEDR15R;
#[doc = "Possible values of the field `OSPEEDR7`"]
pub type OSPEEDR7R = ::gpiof::ospeedr::OSPEEDR15R;
#[doc = "Possible values of the field `OSPEEDR6`"]
pub type OSPEEDR6R = ::gpiof::ospeedr::OSPEEDR15R;
#[doc = "Possible values of the field `OSPEEDR5`"]
pub type OSPEEDR5R = ::gpiof::ospeedr::OSPEEDR15R;
#[doc = "Possible values of the field `OSPEEDR4`"]
pub type OSPEEDR4R = ::gpiof::ospeedr::OSPEEDR15R;
#[doc = "Possible values of the field `OSPEEDR3`"]
pub type OSPEEDR3R = ::gpiof::ospeedr::OSPEEDR15R;
#[doc = "Possible values of the field `OSPEEDR2`"]
pub type OSPEEDR2R = ::gpiof::ospeedr::OSPEEDR15R;
#[doc = "Possible values of the field `OSPEEDR1`"]
pub type OSPEEDR1R = ::gpiof::ospeedr::OSPEEDR15R;
#[doc = "Possible values of the field `OSPEEDR0`"]
pub type OSPEEDR0R = ::gpiof::ospeedr::OSPEEDR15R;
#[doc = "Values that can be written to the field `OSPEEDR15`"]
pub type OSPEEDR15W = ::gpiof::ospeedr::OSPEEDR15W;
#[doc = r" Proxy"]
pub struct _OSPEEDR15W<'a> {
    w: &'a mut W,
}
impl<'a> _OSPEEDR15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSPEEDR15W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::LOW)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn medium(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::MEDIUM)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR14`"]
pub type OSPEEDR14W = ::gpiof::ospeedr::OSPEEDR15W;
#[doc = r" Proxy"]
pub struct _OSPEEDR14W<'a> {
    w: &'a mut W,
}
impl<'a> _OSPEEDR14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSPEEDR14W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::LOW)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn medium(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::MEDIUM)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR13`"]
pub type OSPEEDR13W = ::gpiof::ospeedr::OSPEEDR15W;
#[doc = r" Proxy"]
pub struct _OSPEEDR13W<'a> {
    w: &'a mut W,
}
impl<'a> _OSPEEDR13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSPEEDR13W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::LOW)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn medium(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::MEDIUM)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR12`"]
pub type OSPEEDR12W = ::gpiof::ospeedr::OSPEEDR15W;
#[doc = r" Proxy"]
pub struct _OSPEEDR12W<'a> {
    w: &'a mut W,
}
impl<'a> _OSPEEDR12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSPEEDR12W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::LOW)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn medium(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::MEDIUM)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR11`"]
pub type OSPEEDR11W = ::gpiof::ospeedr::OSPEEDR15W;
#[doc = r" Proxy"]
pub struct _OSPEEDR11W<'a> {
    w: &'a mut W,
}
impl<'a> _OSPEEDR11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSPEEDR11W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::LOW)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn medium(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::MEDIUM)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR10`"]
pub type OSPEEDR10W = ::gpiof::ospeedr::OSPEEDR15W;
#[doc = r" Proxy"]
pub struct _OSPEEDR10W<'a> {
    w: &'a mut W,
}
impl<'a> _OSPEEDR10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSPEEDR10W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::LOW)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn medium(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::MEDIUM)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR9`"]
pub type OSPEEDR9W = ::gpiof::ospeedr::OSPEEDR15W;
#[doc = r" Proxy"]
pub struct _OSPEEDR9W<'a> {
    w: &'a mut W,
}
impl<'a> _OSPEEDR9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSPEEDR9W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::LOW)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn medium(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::MEDIUM)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR8`"]
pub type OSPEEDR8W = ::gpiof::ospeedr::OSPEEDR15W;
#[doc = r" Proxy"]
pub struct _OSPEEDR8W<'a> {
    w: &'a mut W,
}
impl<'a> _OSPEEDR8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSPEEDR8W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::LOW)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn medium(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::MEDIUM)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR7`"]
pub type OSPEEDR7W = ::gpiof::ospeedr::OSPEEDR15W;
#[doc = r" Proxy"]
pub struct _OSPEEDR7W<'a> {
    w: &'a mut W,
}
impl<'a> _OSPEEDR7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSPEEDR7W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::LOW)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn medium(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::MEDIUM)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR6`"]
pub type OSPEEDR6W = ::gpiof::ospeedr::OSPEEDR15W;
#[doc = r" Proxy"]
pub struct _OSPEEDR6W<'a> {
    w: &'a mut W,
}
impl<'a> _OSPEEDR6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSPEEDR6W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::LOW)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn medium(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::MEDIUM)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR5`"]
pub type OSPEEDR5W = ::gpiof::ospeedr::OSPEEDR15W;
#[doc = r" Proxy"]
pub struct _OSPEEDR5W<'a> {
    w: &'a mut W,
}
impl<'a> _OSPEEDR5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSPEEDR5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::LOW)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn medium(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::MEDIUM)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR4`"]
pub type OSPEEDR4W = ::gpiof::ospeedr::OSPEEDR15W;
#[doc = r" Proxy"]
pub struct _OSPEEDR4W<'a> {
    w: &'a mut W,
}
impl<'a> _OSPEEDR4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSPEEDR4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::LOW)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn medium(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::MEDIUM)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR3`"]
pub type OSPEEDR3W = ::gpiof::ospeedr::OSPEEDR15W;
#[doc = r" Proxy"]
pub struct _OSPEEDR3W<'a> {
    w: &'a mut W,
}
impl<'a> _OSPEEDR3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSPEEDR3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::LOW)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn medium(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::MEDIUM)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR2`"]
pub type OSPEEDR2W = ::gpiof::ospeedr::OSPEEDR15W;
#[doc = r" Proxy"]
pub struct _OSPEEDR2W<'a> {
    w: &'a mut W,
}
impl<'a> _OSPEEDR2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSPEEDR2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::LOW)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn medium(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::MEDIUM)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR1`"]
pub type OSPEEDR1W = ::gpiof::ospeedr::OSPEEDR15W;
#[doc = r" Proxy"]
pub struct _OSPEEDR1W<'a> {
    w: &'a mut W,
}
impl<'a> _OSPEEDR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSPEEDR1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::LOW)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn medium(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::MEDIUM)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSPEEDR0`"]
pub type OSPEEDR0W = ::gpiof::ospeedr::OSPEEDR15W;
#[doc = r" Proxy"]
pub struct _OSPEEDR0W<'a> {
    w: &'a mut W,
}
impl<'a> _OSPEEDR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSPEEDR0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::LOW)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn medium(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::MEDIUM)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(::gpiof::ospeedr::OSPEEDR15W::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr15(&self) -> OSPEEDR15R {
        OSPEEDR15R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr14(&self) -> OSPEEDR14R {
        OSPEEDR14R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr13(&self) -> OSPEEDR13R {
        OSPEEDR13R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr12(&self) -> OSPEEDR12R {
        OSPEEDR12R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr11(&self) -> OSPEEDR11R {
        OSPEEDR11R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr10(&self) -> OSPEEDR10R {
        OSPEEDR10R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr9(&self) -> OSPEEDR9R {
        OSPEEDR9R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr8(&self) -> OSPEEDR8R {
        OSPEEDR8R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr7(&self) -> OSPEEDR7R {
        OSPEEDR7R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr6(&self) -> OSPEEDR6R {
        OSPEEDR6R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr5(&self) -> OSPEEDR5R {
        OSPEEDR5R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr4(&self) -> OSPEEDR4R {
        OSPEEDR4R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr3(&self) -> OSPEEDR3R {
        OSPEEDR3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr2(&self) -> OSPEEDR2R {
        OSPEEDR2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr1(&self) -> OSPEEDR1R {
        OSPEEDR1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr0(&self) -> OSPEEDR0R {
        OSPEEDR0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr15(&mut self) -> _OSPEEDR15W {
        _OSPEEDR15W { w: self }
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr14(&mut self) -> _OSPEEDR14W {
        _OSPEEDR14W { w: self }
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr13(&mut self) -> _OSPEEDR13W {
        _OSPEEDR13W { w: self }
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr12(&mut self) -> _OSPEEDR12W {
        _OSPEEDR12W { w: self }
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr11(&mut self) -> _OSPEEDR11W {
        _OSPEEDR11W { w: self }
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr10(&mut self) -> _OSPEEDR10W {
        _OSPEEDR10W { w: self }
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr9(&mut self) -> _OSPEEDR9W {
        _OSPEEDR9W { w: self }
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr8(&mut self) -> _OSPEEDR8W {
        _OSPEEDR8W { w: self }
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr7(&mut self) -> _OSPEEDR7W {
        _OSPEEDR7W { w: self }
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr6(&mut self) -> _OSPEEDR6W {
        _OSPEEDR6W { w: self }
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr5(&mut self) -> _OSPEEDR5W {
        _OSPEEDR5W { w: self }
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr4(&mut self) -> _OSPEEDR4W {
        _OSPEEDR4W { w: self }
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr3(&mut self) -> _OSPEEDR3W {
        _OSPEEDR3W { w: self }
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr2(&mut self) -> _OSPEEDR2W {
        _OSPEEDR2W { w: self }
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr1(&mut self) -> _OSPEEDR1W {
        _OSPEEDR1W { w: self }
    }
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline]
    pub fn ospeedr0(&mut self) -> _OSPEEDR0W {
        _OSPEEDR0W { w: self }
    }
}
