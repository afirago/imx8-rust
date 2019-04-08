#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AUDIO_PLL2_SSCG_CTRL {
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
#[doc = "Possible values of the field `SEL_PF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_PFR {
    #[doc = "Down spread"]
    SEL_PF_0,
    #[doc = "Up spread"]
    SEL_PF_1,
    #[doc = "Center spread"]
    SEL_PF_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SEL_PFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SEL_PFR::SEL_PF_0 => 0,
            SEL_PFR::SEL_PF_1 => 1,
            SEL_PFR::SEL_PF_2 => 2,
            SEL_PFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SEL_PFR {
        match value {
            0 => SEL_PFR::SEL_PF_0,
            1 => SEL_PFR::SEL_PF_1,
            2 => SEL_PFR::SEL_PF_2,
            i => SEL_PFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SEL_PF_0`"]
    #[inline]
    pub fn is_sel_pf_0(&self) -> bool {
        *self == SEL_PFR::SEL_PF_0
    }
    #[doc = "Checks if the value of the field is `SEL_PF_1`"]
    #[inline]
    pub fn is_sel_pf_1(&self) -> bool {
        *self == SEL_PFR::SEL_PF_1
    }
    #[doc = "Checks if the value of the field is `SEL_PF_2`"]
    #[inline]
    pub fn is_sel_pf_2(&self) -> bool {
        *self == SEL_PFR::SEL_PF_2
    }
}
#[doc = r" Value of the field"]
pub struct PLL_MRAT_CTLR {
    bits: u8,
}
impl PLL_MRAT_CTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PLL_MFREQ_CTLR {
    bits: u8,
}
impl PLL_MFREQ_CTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SSCG_ENR {
    bits: bool,
}
impl SSCG_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Values that can be written to the field `SEL_PF`"]
pub enum SEL_PFW {
    #[doc = "Down spread"]
    SEL_PF_0,
    #[doc = "Up spread"]
    SEL_PF_1,
    #[doc = "Center spread"]
    SEL_PF_2,
}
impl SEL_PFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SEL_PFW::SEL_PF_0 => 0,
            SEL_PFW::SEL_PF_1 => 1,
            SEL_PFW::SEL_PF_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEL_PFW<'a> {
    w: &'a mut W,
}
impl<'a> _SEL_PFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEL_PFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Down spread"]
    #[inline]
    pub fn sel_pf_0(self) -> &'a mut W {
        self.variant(SEL_PFW::SEL_PF_0)
    }
    #[doc = "Up spread"]
    #[inline]
    pub fn sel_pf_1(self) -> &'a mut W {
        self.variant(SEL_PFW::SEL_PF_1)
    }
    #[doc = "Center spread"]
    #[inline]
    pub fn sel_pf_2(self) -> &'a mut W {
        self.variant(SEL_PFW::SEL_PF_2)
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
#[doc = r" Proxy"]
pub struct _PLL_MRAT_CTLW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_MRAT_CTLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLL_MFREQ_CTLW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_MFREQ_CTLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SSCG_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SSCG_ENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:1 - Value of modulation method control"]
    #[inline]
    pub fn sel_pf(&self) -> SEL_PFR {
        SEL_PFR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:9 - Value of modulation rate control"]
    #[inline]
    pub fn pll_mrat_ctl(&self) -> PLL_MRAT_CTLR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PLL_MRAT_CTLR { bits }
    }
    #[doc = "Bits 12:19 - Value of modulation frequency control"]
    #[inline]
    pub fn pll_mfreq_ctl(&self) -> PLL_MFREQ_CTLR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PLL_MFREQ_CTLR { bits }
    }
    #[doc = "Bit 31 - SSCG Enable"]
    #[inline]
    pub fn sscg_en(&self) -> SSCG_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SSCG_ENR { bits }
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
    #[doc = "Bits 0:1 - Value of modulation method control"]
    #[inline]
    pub fn sel_pf(&mut self) -> _SEL_PFW {
        _SEL_PFW { w: self }
    }
    #[doc = "Bits 4:9 - Value of modulation rate control"]
    #[inline]
    pub fn pll_mrat_ctl(&mut self) -> _PLL_MRAT_CTLW {
        _PLL_MRAT_CTLW { w: self }
    }
    #[doc = "Bits 12:19 - Value of modulation frequency control"]
    #[inline]
    pub fn pll_mfreq_ctl(&mut self) -> _PLL_MFREQ_CTLW {
        _PLL_MFREQ_CTLW { w: self }
    }
    #[doc = "Bit 31 - SSCG Enable"]
    #[inline]
    pub fn sscg_en(&mut self) -> _SSCG_ENW {
        _SSCG_ENW { w: self }
    }
}
