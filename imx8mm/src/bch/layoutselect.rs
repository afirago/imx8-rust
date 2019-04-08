#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LAYOUTSELECT {
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
        R {
            bits: self.register.get(),
        }
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
#[doc = r" Value of the field"]
pub struct CS0_SELECTR {
    bits: u8,
}
impl CS0_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CS1_SELECTR {
    bits: u8,
}
impl CS1_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CS2_SELECTR {
    bits: u8,
}
impl CS2_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CS3_SELECTR {
    bits: u8,
}
impl CS3_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CS4_SELECTR {
    bits: u8,
}
impl CS4_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CS5_SELECTR {
    bits: u8,
}
impl CS5_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CS6_SELECTR {
    bits: u8,
}
impl CS6_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CS7_SELECTR {
    bits: u8,
}
impl CS7_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CS8_SELECTR {
    bits: u8,
}
impl CS8_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CS9_SELECTR {
    bits: u8,
}
impl CS9_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CS10_SELECTR {
    bits: u8,
}
impl CS10_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CS11_SELECTR {
    bits: u8,
}
impl CS11_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CS12_SELECTR {
    bits: u8,
}
impl CS12_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CS13_SELECTR {
    bits: u8,
}
impl CS13_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CS14_SELECTR {
    bits: u8,
}
impl CS14_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CS15_SELECTR {
    bits: u8,
}
impl CS15_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CS0_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _CS0_SELECTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CS1_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _CS1_SELECTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CS2_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _CS2_SELECTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CS3_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _CS3_SELECTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CS4_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _CS4_SELECTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CS5_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _CS5_SELECTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CS6_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _CS6_SELECTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CS7_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _CS7_SELECTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CS8_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _CS8_SELECTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CS9_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _CS9_SELECTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CS10_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _CS10_SELECTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CS11_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _CS11_SELECTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CS12_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _CS12_SELECTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CS13_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _CS13_SELECTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CS14_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _CS14_SELECTW<'a> {
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
#[doc = r" Proxy"]
pub struct _CS15_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _CS15_SELECTW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Selects which layout is used for chip select 0."]
    #[inline]
    pub fn cs0_select(&self) -> CS0_SELECTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CS0_SELECTR { bits }
    }
    #[doc = "Bits 2:3 - Selects which layout is used for chip select 1."]
    #[inline]
    pub fn cs1_select(&self) -> CS1_SELECTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CS1_SELECTR { bits }
    }
    #[doc = "Bits 4:5 - Selects which layout is used for chip select 2."]
    #[inline]
    pub fn cs2_select(&self) -> CS2_SELECTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CS2_SELECTR { bits }
    }
    #[doc = "Bits 6:7 - Selects which layout is used for chip select 3."]
    #[inline]
    pub fn cs3_select(&self) -> CS3_SELECTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CS3_SELECTR { bits }
    }
    #[doc = "Bits 8:9 - Selects which layout is used for chip select 4."]
    #[inline]
    pub fn cs4_select(&self) -> CS4_SELECTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CS4_SELECTR { bits }
    }
    #[doc = "Bits 10:11 - Selects which layout is used for chip select 5."]
    #[inline]
    pub fn cs5_select(&self) -> CS5_SELECTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CS5_SELECTR { bits }
    }
    #[doc = "Bits 12:13 - Selects which layout is used for chip select 6."]
    #[inline]
    pub fn cs6_select(&self) -> CS6_SELECTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CS6_SELECTR { bits }
    }
    #[doc = "Bits 14:15 - Selects which layout is used for chip select 7."]
    #[inline]
    pub fn cs7_select(&self) -> CS7_SELECTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CS7_SELECTR { bits }
    }
    #[doc = "Bits 16:17 - Selects which layout is used for chip select 8."]
    #[inline]
    pub fn cs8_select(&self) -> CS8_SELECTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CS8_SELECTR { bits }
    }
    #[doc = "Bits 18:19 - Selects which layout is used for chip select 9."]
    #[inline]
    pub fn cs9_select(&self) -> CS9_SELECTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CS9_SELECTR { bits }
    }
    #[doc = "Bits 20:21 - Selects which layout is used for chip select 10."]
    #[inline]
    pub fn cs10_select(&self) -> CS10_SELECTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CS10_SELECTR { bits }
    }
    #[doc = "Bits 22:23 - Selects which layout is used for chip select 11."]
    #[inline]
    pub fn cs11_select(&self) -> CS11_SELECTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CS11_SELECTR { bits }
    }
    #[doc = "Bits 24:25 - Selects which layout is used for chip select 12."]
    #[inline]
    pub fn cs12_select(&self) -> CS12_SELECTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CS12_SELECTR { bits }
    }
    #[doc = "Bits 26:27 - Selects which layout is used for chip select 13."]
    #[inline]
    pub fn cs13_select(&self) -> CS13_SELECTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CS13_SELECTR { bits }
    }
    #[doc = "Bits 28:29 - Selects which layout is used for chip select 14."]
    #[inline]
    pub fn cs14_select(&self) -> CS14_SELECTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CS14_SELECTR { bits }
    }
    #[doc = "Bits 30:31 - Selects which layout is used for chip select 15."]
    #[inline]
    pub fn cs15_select(&self) -> CS15_SELECTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CS15_SELECTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3840206052 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Selects which layout is used for chip select 0."]
    #[inline]
    pub fn cs0_select(&mut self) -> _CS0_SELECTW {
        _CS0_SELECTW { w: self }
    }
    #[doc = "Bits 2:3 - Selects which layout is used for chip select 1."]
    #[inline]
    pub fn cs1_select(&mut self) -> _CS1_SELECTW {
        _CS1_SELECTW { w: self }
    }
    #[doc = "Bits 4:5 - Selects which layout is used for chip select 2."]
    #[inline]
    pub fn cs2_select(&mut self) -> _CS2_SELECTW {
        _CS2_SELECTW { w: self }
    }
    #[doc = "Bits 6:7 - Selects which layout is used for chip select 3."]
    #[inline]
    pub fn cs3_select(&mut self) -> _CS3_SELECTW {
        _CS3_SELECTW { w: self }
    }
    #[doc = "Bits 8:9 - Selects which layout is used for chip select 4."]
    #[inline]
    pub fn cs4_select(&mut self) -> _CS4_SELECTW {
        _CS4_SELECTW { w: self }
    }
    #[doc = "Bits 10:11 - Selects which layout is used for chip select 5."]
    #[inline]
    pub fn cs5_select(&mut self) -> _CS5_SELECTW {
        _CS5_SELECTW { w: self }
    }
    #[doc = "Bits 12:13 - Selects which layout is used for chip select 6."]
    #[inline]
    pub fn cs6_select(&mut self) -> _CS6_SELECTW {
        _CS6_SELECTW { w: self }
    }
    #[doc = "Bits 14:15 - Selects which layout is used for chip select 7."]
    #[inline]
    pub fn cs7_select(&mut self) -> _CS7_SELECTW {
        _CS7_SELECTW { w: self }
    }
    #[doc = "Bits 16:17 - Selects which layout is used for chip select 8."]
    #[inline]
    pub fn cs8_select(&mut self) -> _CS8_SELECTW {
        _CS8_SELECTW { w: self }
    }
    #[doc = "Bits 18:19 - Selects which layout is used for chip select 9."]
    #[inline]
    pub fn cs9_select(&mut self) -> _CS9_SELECTW {
        _CS9_SELECTW { w: self }
    }
    #[doc = "Bits 20:21 - Selects which layout is used for chip select 10."]
    #[inline]
    pub fn cs10_select(&mut self) -> _CS10_SELECTW {
        _CS10_SELECTW { w: self }
    }
    #[doc = "Bits 22:23 - Selects which layout is used for chip select 11."]
    #[inline]
    pub fn cs11_select(&mut self) -> _CS11_SELECTW {
        _CS11_SELECTW { w: self }
    }
    #[doc = "Bits 24:25 - Selects which layout is used for chip select 12."]
    #[inline]
    pub fn cs12_select(&mut self) -> _CS12_SELECTW {
        _CS12_SELECTW { w: self }
    }
    #[doc = "Bits 26:27 - Selects which layout is used for chip select 13."]
    #[inline]
    pub fn cs13_select(&mut self) -> _CS13_SELECTW {
        _CS13_SELECTW { w: self }
    }
    #[doc = "Bits 28:29 - Selects which layout is used for chip select 14."]
    #[inline]
    pub fn cs14_select(&mut self) -> _CS14_SELECTW {
        _CS14_SELECTW { w: self }
    }
    #[doc = "Bits 30:31 - Selects which layout is used for chip select 15."]
    #[inline]
    pub fn cs15_select(&mut self) -> _CS15_SELECTW {
        _CS15_SELECTW { w: self }
    }
}
