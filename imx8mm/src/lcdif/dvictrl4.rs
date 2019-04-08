#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DVICTRL4 {
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
pub struct H_FILL_CNTR {
    bits: u8,
}
impl H_FILL_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CR_FILL_VALUER {
    bits: u8,
}
impl CR_FILL_VALUER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CB_FILL_VALUER {
    bits: u8,
}
impl CB_FILL_VALUER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Y_FILL_VALUER {
    bits: u8,
}
impl Y_FILL_VALUER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _H_FILL_CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _H_FILL_CNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CR_FILL_VALUEW<'a> {
    w: &'a mut W,
}
impl<'a> _CR_FILL_VALUEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CB_FILL_VALUEW<'a> {
    w: &'a mut W,
}
impl<'a> _CB_FILL_VALUEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _Y_FILL_VALUEW<'a> {
    w: &'a mut W,
}
impl<'a> _Y_FILL_VALUEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:7 - Number of active video samples that have to be filled with the filler data in the front and back portions of the active horizontal interval"]
    #[inline]
    pub fn h_fill_cnt(&self) -> H_FILL_CNTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        H_FILL_CNTR { bits }
    }
    #[doc = "Bits 8:15 - Value of CR component of filler data."]
    #[inline]
    pub fn cr_fill_value(&self) -> CR_FILL_VALUER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CR_FILL_VALUER { bits }
    }
    #[doc = "Bits 16:23 - Value of CB component of filler data"]
    #[inline]
    pub fn cb_fill_value(&self) -> CB_FILL_VALUER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CB_FILL_VALUER { bits }
    }
    #[doc = "Bits 24:31 - Value of Y component of filler data"]
    #[inline]
    pub fn y_fill_value(&self) -> Y_FILL_VALUER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Y_FILL_VALUER { bits }
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
    #[doc = "Bits 0:7 - Number of active video samples that have to be filled with the filler data in the front and back portions of the active horizontal interval"]
    #[inline]
    pub fn h_fill_cnt(&mut self) -> _H_FILL_CNTW {
        _H_FILL_CNTW { w: self }
    }
    #[doc = "Bits 8:15 - Value of CR component of filler data."]
    #[inline]
    pub fn cr_fill_value(&mut self) -> _CR_FILL_VALUEW {
        _CR_FILL_VALUEW { w: self }
    }
    #[doc = "Bits 16:23 - Value of CB component of filler data"]
    #[inline]
    pub fn cb_fill_value(&mut self) -> _CB_FILL_VALUEW {
        _CB_FILL_VALUEW { w: self }
    }
    #[doc = "Bits 24:31 - Value of Y component of filler data"]
    #[inline]
    pub fn y_fill_value(&mut self) -> _Y_FILL_VALUEW {
        _Y_FILL_VALUEW { w: self }
    }
}
