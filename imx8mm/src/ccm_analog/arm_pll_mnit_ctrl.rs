#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ARM_PLL_MNIT_CTRL {
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
pub struct ICPR {
    bits: u8,
}
impl ICPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AFC_ENR {
    bits: bool,
}
impl AFC_ENR {
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
#[doc = r" Value of the field"]
pub struct EXTAFCR {
    bits: u8,
}
impl EXTAFCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FEED_ENR {
    bits: bool,
}
impl FEED_ENR {
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
#[doc = "Possible values of the field `FSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSELR {
    #[doc = "FEED_OUT = FREF"]
    FSEL_0,
    #[doc = "FEED_OUT = FEED"]
    FSEL_1,
}
impl FSELR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FSELR::FSEL_0 => false,
            FSELR::FSEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FSELR {
        match value {
            false => FSELR::FSEL_0,
            true => FSELR::FSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `FSEL_0`"]
    #[inline]
    pub fn is_fsel_0(&self) -> bool {
        *self == FSELR::FSEL_0
    }
    #[doc = "Checks if the value of the field is `FSEL_1`"]
    #[inline]
    pub fn is_fsel_1(&self) -> bool {
        *self == FSELR::FSEL_1
    }
}
#[doc = "Possible values of the field `AFCINIT_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFCINIT_SELR {
    #[doc = "nominal delay"]
    AFCINIT_SEL_0,
    #[doc = "nominal delay * 2"]
    AFCINIT_SEL_1,
}
impl AFCINIT_SELR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AFCINIT_SELR::AFCINIT_SEL_0 => false,
            AFCINIT_SELR::AFCINIT_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AFCINIT_SELR {
        match value {
            false => AFCINIT_SELR::AFCINIT_SEL_0,
            true => AFCINIT_SELR::AFCINIT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `AFCINIT_SEL_0`"]
    #[inline]
    pub fn is_afcinit_sel_0(&self) -> bool {
        *self == AFCINIT_SELR::AFCINIT_SEL_0
    }
    #[doc = "Checks if the value of the field is `AFCINIT_SEL_1`"]
    #[inline]
    pub fn is_afcinit_sel_1(&self) -> bool {
        *self == AFCINIT_SELR::AFCINIT_SEL_1
    }
}
#[doc = r" Value of the field"]
pub struct PBIAS_CTRL_ENR {
    bits: bool,
}
impl PBIAS_CTRL_ENR {
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
#[doc = "Possible values of the field `PBIAS_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBIAS_CTRLR {
    #[doc = "0.50*VDD"]
    PBIAS_CTRL_0,
    #[doc = "0.67*VDD"]
    PBIAS_CTRL_1,
}
impl PBIAS_CTRLR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PBIAS_CTRLR::PBIAS_CTRL_0 => false,
            PBIAS_CTRLR::PBIAS_CTRL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PBIAS_CTRLR {
        match value {
            false => PBIAS_CTRLR::PBIAS_CTRL_0,
            true => PBIAS_CTRLR::PBIAS_CTRL_1,
        }
    }
    #[doc = "Checks if the value of the field is `PBIAS_CTRL_0`"]
    #[inline]
    pub fn is_pbias_ctrl_0(&self) -> bool {
        *self == PBIAS_CTRLR::PBIAS_CTRL_0
    }
    #[doc = "Checks if the value of the field is `PBIAS_CTRL_1`"]
    #[inline]
    pub fn is_pbias_ctrl_1(&self) -> bool {
        *self == PBIAS_CTRLR::PBIAS_CTRL_1
    }
}
#[doc = r" Value of the field"]
pub struct AFC_SELR {
    bits: bool,
}
impl AFC_SELR {
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
#[doc = r" Value of the field"]
pub struct FOUT_MASKR {
    bits: bool,
}
impl FOUT_MASKR {
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
#[doc = r" Value of the field"]
pub struct LRD_ENR {
    bits: bool,
}
impl LRD_ENR {
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
#[doc = r" Proxy"]
pub struct _ICPW<'a> {
    w: &'a mut W,
}
impl<'a> _ICPW<'a> {
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
pub struct _AFC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _AFC_ENW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EXTAFCW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTAFCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FEED_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FEED_ENW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FSEL`"]
pub enum FSELW {
    #[doc = "FEED_OUT = FREF"]
    FSEL_0,
    #[doc = "FEED_OUT = FEED"]
    FSEL_1,
}
impl FSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FSELW::FSEL_0 => false,
            FSELW::FSEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FEED_OUT = FREF"]
    #[inline]
    pub fn fsel_0(self) -> &'a mut W {
        self.variant(FSELW::FSEL_0)
    }
    #[doc = "FEED_OUT = FEED"]
    #[inline]
    pub fn fsel_1(self) -> &'a mut W {
        self.variant(FSELW::FSEL_1)
    }
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFCINIT_SEL`"]
pub enum AFCINIT_SELW {
    #[doc = "nominal delay"]
    AFCINIT_SEL_0,
    #[doc = "nominal delay * 2"]
    AFCINIT_SEL_1,
}
impl AFCINIT_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AFCINIT_SELW::AFCINIT_SEL_0 => false,
            AFCINIT_SELW::AFCINIT_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AFCINIT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _AFCINIT_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFCINIT_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "nominal delay"]
    #[inline]
    pub fn afcinit_sel_0(self) -> &'a mut W {
        self.variant(AFCINIT_SELW::AFCINIT_SEL_0)
    }
    #[doc = "nominal delay * 2"]
    #[inline]
    pub fn afcinit_sel_1(self) -> &'a mut W {
        self.variant(AFCINIT_SELW::AFCINIT_SEL_1)
    }
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PBIAS_CTRL_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PBIAS_CTRL_ENW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PBIAS_CTRL`"]
pub enum PBIAS_CTRLW {
    #[doc = "0.50*VDD"]
    PBIAS_CTRL_0,
    #[doc = "0.67*VDD"]
    PBIAS_CTRL_1,
}
impl PBIAS_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PBIAS_CTRLW::PBIAS_CTRL_0 => false,
            PBIAS_CTRLW::PBIAS_CTRL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PBIAS_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _PBIAS_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PBIAS_CTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "0.50*VDD"]
    #[inline]
    pub fn pbias_ctrl_0(self) -> &'a mut W {
        self.variant(PBIAS_CTRLW::PBIAS_CTRL_0)
    }
    #[doc = "0.67*VDD"]
    #[inline]
    pub fn pbias_ctrl_1(self) -> &'a mut W {
        self.variant(PBIAS_CTRLW::PBIAS_CTRL_1)
    }
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AFC_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _AFC_SELW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FOUT_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _FOUT_MASKW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LRD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _LRD_ENW<'a> {
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
        const OFFSET: u8 = 21;
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
    #[doc = "Bits 0:1 - Controls the charge-pump current"]
    #[inline]
    pub fn icp(&self) -> ICPR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ICPR { bits }
    }
    #[doc = "Bit 2 - If AFC_ENB=0, AFC is enabled and VCO is calibrated automatically"]
    #[inline]
    pub fn afc_en(&self) -> AFC_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AFC_ENR { bits }
    }
    #[doc = "Bits 3:7 - Monitoring pin"]
    #[inline]
    pub fn extafc(&self) -> EXTAFCR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXTAFCR { bits }
    }
    #[doc = "Bit 13 - FEED_OUT enable pin"]
    #[inline]
    pub fn feed_en(&self) -> FEED_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FEED_ENR { bits }
    }
    #[doc = "Bit 14 - Monitoring frequency select pin"]
    #[inline]
    pub fn fsel(&self) -> FSELR {
        FSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - AFC initial delay select pin"]
    #[inline]
    pub fn afcinit_sel(&self) -> AFCINIT_SELR {
        AFCINIT_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - PBIAS voltage pull-down enable pin"]
    #[inline]
    pub fn pbias_ctrl_en(&self) -> PBIAS_CTRL_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PBIAS_CTRL_ENR { bits }
    }
    #[doc = "Bit 18 - PBIAS pull-down initial voltage control pin"]
    #[inline]
    pub fn pbias_ctrl(&self) -> PBIAS_CTRLR {
        PBIAS_CTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - AFC Mode select"]
    #[inline]
    pub fn afc_sel(&self) -> AFC_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AFC_SELR { bits }
    }
    #[doc = "Bit 20 - Scaler's re-initialization time control pin\\[3\\]"]
    #[inline]
    pub fn fout_mask(&self) -> FOUT_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FOUT_MASKR { bits }
    }
    #[doc = "Bit 21 - Monitoring pin. AFC operation mode select pin"]
    #[inline]
    pub fn lrd_en(&self) -> LRD_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LRD_ENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2621569 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Controls the charge-pump current"]
    #[inline]
    pub fn icp(&mut self) -> _ICPW {
        _ICPW { w: self }
    }
    #[doc = "Bit 2 - If AFC_ENB=0, AFC is enabled and VCO is calibrated automatically"]
    #[inline]
    pub fn afc_en(&mut self) -> _AFC_ENW {
        _AFC_ENW { w: self }
    }
    #[doc = "Bits 3:7 - Monitoring pin"]
    #[inline]
    pub fn extafc(&mut self) -> _EXTAFCW {
        _EXTAFCW { w: self }
    }
    #[doc = "Bit 13 - FEED_OUT enable pin"]
    #[inline]
    pub fn feed_en(&mut self) -> _FEED_ENW {
        _FEED_ENW { w: self }
    }
    #[doc = "Bit 14 - Monitoring frequency select pin"]
    #[inline]
    pub fn fsel(&mut self) -> _FSELW {
        _FSELW { w: self }
    }
    #[doc = "Bit 16 - AFC initial delay select pin"]
    #[inline]
    pub fn afcinit_sel(&mut self) -> _AFCINIT_SELW {
        _AFCINIT_SELW { w: self }
    }
    #[doc = "Bit 17 - PBIAS voltage pull-down enable pin"]
    #[inline]
    pub fn pbias_ctrl_en(&mut self) -> _PBIAS_CTRL_ENW {
        _PBIAS_CTRL_ENW { w: self }
    }
    #[doc = "Bit 18 - PBIAS pull-down initial voltage control pin"]
    #[inline]
    pub fn pbias_ctrl(&mut self) -> _PBIAS_CTRLW {
        _PBIAS_CTRLW { w: self }
    }
    #[doc = "Bit 19 - AFC Mode select"]
    #[inline]
    pub fn afc_sel(&mut self) -> _AFC_SELW {
        _AFC_SELW { w: self }
    }
    #[doc = "Bit 20 - Scaler's re-initialization time control pin\\[3\\]"]
    #[inline]
    pub fn fout_mask(&mut self) -> _FOUT_MASKW {
        _FOUT_MASKW { w: self }
    }
    #[doc = "Bit 21 - Monitoring pin. AFC operation mode select pin"]
    #[inline]
    pub fn lrd_en(&mut self) -> _LRD_ENW {
        _LRD_ENW { w: self }
    }
}
