#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OUT_CTRL {
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
pub struct OUTGAIN0R {
    bits: u8,
}
impl OUTGAIN0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OUTGAIN1R {
    bits: u8,
}
impl OUTGAIN1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OUTGAIN2R {
    bits: u8,
}
impl OUTGAIN2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OUTGAIN3R {
    bits: u8,
}
impl OUTGAIN3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OUTGAIN4R {
    bits: u8,
}
impl OUTGAIN4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OUTGAIN5R {
    bits: u8,
}
impl OUTGAIN5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OUTGAIN6R {
    bits: u8,
}
impl OUTGAIN6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OUTGAIN7R {
    bits: u8,
}
impl OUTGAIN7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _OUTGAIN0W<'a> {
    w: &'a mut W,
}
impl<'a> _OUTGAIN0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OUTGAIN1W<'a> {
    w: &'a mut W,
}
impl<'a> _OUTGAIN1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OUTGAIN2W<'a> {
    w: &'a mut W,
}
impl<'a> _OUTGAIN2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OUTGAIN3W<'a> {
    w: &'a mut W,
}
impl<'a> _OUTGAIN3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OUTGAIN4W<'a> {
    w: &'a mut W,
}
impl<'a> _OUTGAIN4W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OUTGAIN5W<'a> {
    w: &'a mut W,
}
impl<'a> _OUTGAIN5W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OUTGAIN6W<'a> {
    w: &'a mut W,
}
impl<'a> _OUTGAIN6W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OUTGAIN7W<'a> {
    w: &'a mut W,
}
impl<'a> _OUTGAIN7W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:3 - Channel 0 Decimation Filter Output Gain."]
    #[inline]
    pub fn outgain0(&self) -> OUTGAIN0R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OUTGAIN0R { bits }
    }
    #[doc = "Bits 4:7 - Channel 1 Decimation Filter Output Gain."]
    #[inline]
    pub fn outgain1(&self) -> OUTGAIN1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OUTGAIN1R { bits }
    }
    #[doc = "Bits 8:11 - Channel 2 Decimation Filter Output Gain."]
    #[inline]
    pub fn outgain2(&self) -> OUTGAIN2R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OUTGAIN2R { bits }
    }
    #[doc = "Bits 12:15 - Channel 3 Decimation Filter Output Gain."]
    #[inline]
    pub fn outgain3(&self) -> OUTGAIN3R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OUTGAIN3R { bits }
    }
    #[doc = "Bits 16:19 - Channel 4 Decimation Filter Output Gain."]
    #[inline]
    pub fn outgain4(&self) -> OUTGAIN4R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OUTGAIN4R { bits }
    }
    #[doc = "Bits 20:23 - Channel 5 Decimation Filter Output Gain."]
    #[inline]
    pub fn outgain5(&self) -> OUTGAIN5R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OUTGAIN5R { bits }
    }
    #[doc = "Bits 24:27 - Channel 6 Decimation Filter Output Gain."]
    #[inline]
    pub fn outgain6(&self) -> OUTGAIN6R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OUTGAIN6R { bits }
    }
    #[doc = "Bits 28:31 - Channel 7 Decimation Filter Output Gain."]
    #[inline]
    pub fn outgain7(&self) -> OUTGAIN7R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OUTGAIN7R { bits }
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
    #[doc = "Bits 0:3 - Channel 0 Decimation Filter Output Gain."]
    #[inline]
    pub fn outgain0(&mut self) -> _OUTGAIN0W {
        _OUTGAIN0W { w: self }
    }
    #[doc = "Bits 4:7 - Channel 1 Decimation Filter Output Gain."]
    #[inline]
    pub fn outgain1(&mut self) -> _OUTGAIN1W {
        _OUTGAIN1W { w: self }
    }
    #[doc = "Bits 8:11 - Channel 2 Decimation Filter Output Gain."]
    #[inline]
    pub fn outgain2(&mut self) -> _OUTGAIN2W {
        _OUTGAIN2W { w: self }
    }
    #[doc = "Bits 12:15 - Channel 3 Decimation Filter Output Gain."]
    #[inline]
    pub fn outgain3(&mut self) -> _OUTGAIN3W {
        _OUTGAIN3W { w: self }
    }
    #[doc = "Bits 16:19 - Channel 4 Decimation Filter Output Gain."]
    #[inline]
    pub fn outgain4(&mut self) -> _OUTGAIN4W {
        _OUTGAIN4W { w: self }
    }
    #[doc = "Bits 20:23 - Channel 5 Decimation Filter Output Gain."]
    #[inline]
    pub fn outgain5(&mut self) -> _OUTGAIN5W {
        _OUTGAIN5W { w: self }
    }
    #[doc = "Bits 24:27 - Channel 6 Decimation Filter Output Gain."]
    #[inline]
    pub fn outgain6(&mut self) -> _OUTGAIN6W {
        _OUTGAIN6W { w: self }
    }
    #[doc = "Bits 28:31 - Channel 7 Decimation Filter Output Gain."]
    #[inline]
    pub fn outgain7(&mut self) -> _OUTGAIN7W {
        _OUTGAIN7W { w: self }
    }
}
