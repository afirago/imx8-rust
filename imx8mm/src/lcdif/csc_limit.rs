#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSC_LIMIT {
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
pub struct Y_MAXR {
    bits: u8,
}
impl Y_MAXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct Y_MINR {
    bits: u8,
}
impl Y_MINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CBCR_MAXR {
    bits: u8,
}
impl CBCR_MAXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CBCR_MINR {
    bits: u8,
}
impl CBCR_MINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _Y_MAXW<'a> {
    w: &'a mut W,
}
impl<'a> _Y_MAXW<'a> {
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
pub struct _Y_MINW<'a> {
    w: &'a mut W,
}
impl<'a> _Y_MINW<'a> {
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
pub struct _CBCR_MAXW<'a> {
    w: &'a mut W,
}
impl<'a> _CBCR_MAXW<'a> {
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
pub struct _CBCR_MINW<'a> {
    w: &'a mut W,
}
impl<'a> _CBCR_MINW<'a> {
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
    #[doc = "Bits 0:7 - Upper limit of Y after RGB to 4:2:2 YCbCr conversion"]
    #[inline]
    pub fn y_max(&self) -> Y_MAXR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Y_MAXR { bits }
    }
    #[doc = "Bits 8:15 - Lower limit of Y after RGB to 4:2:2 YCbCr conversion"]
    #[inline]
    pub fn y_min(&self) -> Y_MINR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        Y_MINR { bits }
    }
    #[doc = "Bits 16:23 - Upper limit of Cb and Cr after RGB to 4:2:2 YCbCr conversion"]
    #[inline]
    pub fn cbcr_max(&self) -> CBCR_MAXR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CBCR_MAXR { bits }
    }
    #[doc = "Bits 24:31 - Lower limit of Cb and Cr after RGB to 4:2:2 YCbCr conversion"]
    #[inline]
    pub fn cbcr_min(&self) -> CBCR_MINR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CBCR_MINR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 16711935 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Upper limit of Y after RGB to 4:2:2 YCbCr conversion"]
    #[inline]
    pub fn y_max(&mut self) -> _Y_MAXW {
        _Y_MAXW { w: self }
    }
    #[doc = "Bits 8:15 - Lower limit of Y after RGB to 4:2:2 YCbCr conversion"]
    #[inline]
    pub fn y_min(&mut self) -> _Y_MINW {
        _Y_MINW { w: self }
    }
    #[doc = "Bits 16:23 - Upper limit of Cb and Cr after RGB to 4:2:2 YCbCr conversion"]
    #[inline]
    pub fn cbcr_max(&mut self) -> _CBCR_MAXW {
        _CBCR_MAXW { w: self }
    }
    #[doc = "Bits 24:31 - Lower limit of Cb and Cr after RGB to 4:2:2 YCbCr conversion"]
    #[inline]
    pub fn cbcr_min(&mut self) -> _CBCR_MINW {
        _CBCR_MINW { w: self }
    }
}
