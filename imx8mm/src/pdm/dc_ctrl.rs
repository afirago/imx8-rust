#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DC_CTRL {
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
pub struct DCCONFIG0R {
    bits: u8,
}
impl DCCONFIG0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCCONFIG1R {
    bits: u8,
}
impl DCCONFIG1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCCONFIG2R {
    bits: u8,
}
impl DCCONFIG2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCCONFIG3R {
    bits: u8,
}
impl DCCONFIG3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCCONFIG4R {
    bits: u8,
}
impl DCCONFIG4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCCONFIG5R {
    bits: u8,
}
impl DCCONFIG5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCCONFIG6R {
    bits: u8,
}
impl DCCONFIG6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCCONFIG7R {
    bits: u8,
}
impl DCCONFIG7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DCCONFIG0W<'a> {
    w: &'a mut W,
}
impl<'a> _DCCONFIG0W<'a> {
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
pub struct _DCCONFIG1W<'a> {
    w: &'a mut W,
}
impl<'a> _DCCONFIG1W<'a> {
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
pub struct _DCCONFIG2W<'a> {
    w: &'a mut W,
}
impl<'a> _DCCONFIG2W<'a> {
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
pub struct _DCCONFIG3W<'a> {
    w: &'a mut W,
}
impl<'a> _DCCONFIG3W<'a> {
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
pub struct _DCCONFIG4W<'a> {
    w: &'a mut W,
}
impl<'a> _DCCONFIG4W<'a> {
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
pub struct _DCCONFIG5W<'a> {
    w: &'a mut W,
}
impl<'a> _DCCONFIG5W<'a> {
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
pub struct _DCCONFIG6W<'a> {
    w: &'a mut W,
}
impl<'a> _DCCONFIG6W<'a> {
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
pub struct _DCCONFIG7W<'a> {
    w: &'a mut W,
}
impl<'a> _DCCONFIG7W<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Channel 0 DC Remover Configuration."]
    #[inline]
    pub fn dcconfig0(&self) -> DCCONFIG0R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCCONFIG0R { bits }
    }
    #[doc = "Bits 2:3 - Channel 1 DC Remover Configuration."]
    #[inline]
    pub fn dcconfig1(&self) -> DCCONFIG1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCCONFIG1R { bits }
    }
    #[doc = "Bits 4:5 - Channel 2 DC Remover Configuration."]
    #[inline]
    pub fn dcconfig2(&self) -> DCCONFIG2R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCCONFIG2R { bits }
    }
    #[doc = "Bits 6:7 - Channel 3 DC Remover Configuration."]
    #[inline]
    pub fn dcconfig3(&self) -> DCCONFIG3R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCCONFIG3R { bits }
    }
    #[doc = "Bits 8:9 - Channel 4 DC Remover Configuration."]
    #[inline]
    pub fn dcconfig4(&self) -> DCCONFIG4R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCCONFIG4R { bits }
    }
    #[doc = "Bits 10:11 - Channel 5 DC Remover Configuration."]
    #[inline]
    pub fn dcconfig5(&self) -> DCCONFIG5R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCCONFIG5R { bits }
    }
    #[doc = "Bits 12:13 - Channel 6 DC Remover Configuration."]
    #[inline]
    pub fn dcconfig6(&self) -> DCCONFIG6R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCCONFIG6R { bits }
    }
    #[doc = "Bits 14:15 - Channel 7 DC Remover Configuration."]
    #[inline]
    pub fn dcconfig7(&self) -> DCCONFIG7R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCCONFIG7R { bits }
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
    #[doc = "Bits 0:1 - Channel 0 DC Remover Configuration."]
    #[inline]
    pub fn dcconfig0(&mut self) -> _DCCONFIG0W {
        _DCCONFIG0W { w: self }
    }
    #[doc = "Bits 2:3 - Channel 1 DC Remover Configuration."]
    #[inline]
    pub fn dcconfig1(&mut self) -> _DCCONFIG1W {
        _DCCONFIG1W { w: self }
    }
    #[doc = "Bits 4:5 - Channel 2 DC Remover Configuration."]
    #[inline]
    pub fn dcconfig2(&mut self) -> _DCCONFIG2W {
        _DCCONFIG2W { w: self }
    }
    #[doc = "Bits 6:7 - Channel 3 DC Remover Configuration."]
    #[inline]
    pub fn dcconfig3(&mut self) -> _DCCONFIG3W {
        _DCCONFIG3W { w: self }
    }
    #[doc = "Bits 8:9 - Channel 4 DC Remover Configuration."]
    #[inline]
    pub fn dcconfig4(&mut self) -> _DCCONFIG4W {
        _DCCONFIG4W { w: self }
    }
    #[doc = "Bits 10:11 - Channel 5 DC Remover Configuration."]
    #[inline]
    pub fn dcconfig5(&mut self) -> _DCCONFIG5W {
        _DCCONFIG5W { w: self }
    }
    #[doc = "Bits 12:13 - Channel 6 DC Remover Configuration."]
    #[inline]
    pub fn dcconfig6(&mut self) -> _DCCONFIG6W {
        _DCCONFIG6W { w: self }
    }
    #[doc = "Bits 14:15 - Channel 7 DC Remover Configuration."]
    #[inline]
    pub fn dcconfig7(&mut self) -> _DCCONFIG7W {
        _DCCONFIG7W { w: self }
    }
}
