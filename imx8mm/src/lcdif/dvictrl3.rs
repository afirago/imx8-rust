#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DVICTRL3 {
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
pub struct V_LINES_CNTR {
    bits: u16,
}
impl V_LINES_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct V2_BLANK_END_LINER {
    bits: u16,
}
impl V2_BLANK_END_LINER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct V2_BLANK_START_LINER {
    bits: u16,
}
impl V2_BLANK_START_LINER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _V_LINES_CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _V_LINES_CNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _V2_BLANK_END_LINEW<'a> {
    w: &'a mut W,
}
impl<'a> _V2_BLANK_END_LINEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _V2_BLANK_START_LINEW<'a> {
    w: &'a mut W,
}
impl<'a> _V2_BLANK_START_LINEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 20;
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
    #[doc = "Bits 0:9 - Total number of vertical lines per frame (generally 525 or 625)"]
    #[inline]
    pub fn v_lines_cnt(&self) -> V_LINES_CNTR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        V_LINES_CNTR { bits }
    }
    #[doc = "Bits 10:19 - Vertical line number in the beginning part of Field1 where second Vertical Blanking interval ends."]
    #[inline]
    pub fn v2_blank_end_line(&self) -> V2_BLANK_END_LINER {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        V2_BLANK_END_LINER { bits }
    }
    #[doc = "Bits 20:29 - Vertical line number towards the end of Field2 where second Vertical Blanking interval starts."]
    #[inline]
    pub fn v2_blank_start_line(&self) -> V2_BLANK_START_LINER {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        V2_BLANK_START_LINER { bits }
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
    #[doc = "Bits 0:9 - Total number of vertical lines per frame (generally 525 or 625)"]
    #[inline]
    pub fn v_lines_cnt(&mut self) -> _V_LINES_CNTW {
        _V_LINES_CNTW { w: self }
    }
    #[doc = "Bits 10:19 - Vertical line number in the beginning part of Field1 where second Vertical Blanking interval ends."]
    #[inline]
    pub fn v2_blank_end_line(&mut self) -> _V2_BLANK_END_LINEW {
        _V2_BLANK_END_LINEW { w: self }
    }
    #[doc = "Bits 20:29 - Vertical line number towards the end of Field2 where second Vertical Blanking interval starts."]
    #[inline]
    pub fn v2_blank_start_line(&mut self) -> _V2_BLANK_START_LINEW {
        _V2_BLANK_START_LINEW { w: self }
    }
}
