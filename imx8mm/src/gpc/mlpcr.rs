#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MLPCR {
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
#[doc = "Possible values of the field `MEMLP_CTL_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMLP_CTL_DISR {
    #[doc = "Enable RAM low power control"]
    MEMLP_CTL_DIS_0,
    #[doc = "Disable RAM low power control"]
    MEMLP_CTL_DIS_1,
}
impl MEMLP_CTL_DISR {
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
            MEMLP_CTL_DISR::MEMLP_CTL_DIS_0 => false,
            MEMLP_CTL_DISR::MEMLP_CTL_DIS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MEMLP_CTL_DISR {
        match value {
            false => MEMLP_CTL_DISR::MEMLP_CTL_DIS_0,
            true => MEMLP_CTL_DISR::MEMLP_CTL_DIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEMLP_CTL_DIS_0`"]
    #[inline]
    pub fn is_memlp_ctl_dis_0(&self) -> bool {
        *self == MEMLP_CTL_DISR::MEMLP_CTL_DIS_0
    }
    #[doc = "Checks if the value of the field is `MEMLP_CTL_DIS_1`"]
    #[inline]
    pub fn is_memlp_ctl_dis_1(&self) -> bool {
        *self == MEMLP_CTL_DISR::MEMLP_CTL_DIS_1
    }
}
#[doc = "Possible values of the field `MEMLP_RET_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMLP_RET_SELR {
    #[doc = "retention mode 2"]
    MEMLP_RET_SEL_0,
    #[doc = "retention mode 1"]
    MEMLP_RET_SEL_1,
}
impl MEMLP_RET_SELR {
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
            MEMLP_RET_SELR::MEMLP_RET_SEL_0 => false,
            MEMLP_RET_SELR::MEMLP_RET_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MEMLP_RET_SELR {
        match value {
            false => MEMLP_RET_SELR::MEMLP_RET_SEL_0,
            true => MEMLP_RET_SELR::MEMLP_RET_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEMLP_RET_SEL_0`"]
    #[inline]
    pub fn is_memlp_ret_sel_0(&self) -> bool {
        *self == MEMLP_RET_SELR::MEMLP_RET_SEL_0
    }
    #[doc = "Checks if the value of the field is `MEMLP_RET_SEL_1`"]
    #[inline]
    pub fn is_memlp_ret_sel_1(&self) -> bool {
        *self == MEMLP_RET_SELR::MEMLP_RET_SEL_1
    }
}
#[doc = "Possible values of the field `ROMLP_PDN_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROMLP_PDN_DISR {
    #[doc = "Enable ROM shut down control(should also enable RAM low power control);"]
    ROMLP_PDN_DIS_0,
    #[doc = "Disable ROM shut down control"]
    ROMLP_PDN_DIS_1,
}
impl ROMLP_PDN_DISR {
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
            ROMLP_PDN_DISR::ROMLP_PDN_DIS_0 => false,
            ROMLP_PDN_DISR::ROMLP_PDN_DIS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ROMLP_PDN_DISR {
        match value {
            false => ROMLP_PDN_DISR::ROMLP_PDN_DIS_0,
            true => ROMLP_PDN_DISR::ROMLP_PDN_DIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `ROMLP_PDN_DIS_0`"]
    #[inline]
    pub fn is_romlp_pdn_dis_0(&self) -> bool {
        *self == ROMLP_PDN_DISR::ROMLP_PDN_DIS_0
    }
    #[doc = "Checks if the value of the field is `ROMLP_PDN_DIS_1`"]
    #[inline]
    pub fn is_romlp_pdn_dis_1(&self) -> bool {
        *self == ROMLP_PDN_DISR::ROMLP_PDN_DIS_1
    }
}
#[doc = r" Value of the field"]
pub struct MEMLP_ENT_CNTR {
    bits: u8,
}
impl MEMLP_ENT_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MEM_EXT_CNTR {
    bits: u8,
}
impl MEM_EXT_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MEMLP_RET_PGENR {
    bits: u8,
}
impl MEMLP_RET_PGENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `MEMLP_CTL_DIS`"]
pub enum MEMLP_CTL_DISW {
    #[doc = "Enable RAM low power control"]
    MEMLP_CTL_DIS_0,
    #[doc = "Disable RAM low power control"]
    MEMLP_CTL_DIS_1,
}
impl MEMLP_CTL_DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MEMLP_CTL_DISW::MEMLP_CTL_DIS_0 => false,
            MEMLP_CTL_DISW::MEMLP_CTL_DIS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MEMLP_CTL_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _MEMLP_CTL_DISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MEMLP_CTL_DISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable RAM low power control"]
    #[inline]
    pub fn memlp_ctl_dis_0(self) -> &'a mut W {
        self.variant(MEMLP_CTL_DISW::MEMLP_CTL_DIS_0)
    }
    #[doc = "Disable RAM low power control"]
    #[inline]
    pub fn memlp_ctl_dis_1(self) -> &'a mut W {
        self.variant(MEMLP_CTL_DISW::MEMLP_CTL_DIS_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MEMLP_RET_SEL`"]
pub enum MEMLP_RET_SELW {
    #[doc = "retention mode 2"]
    MEMLP_RET_SEL_0,
    #[doc = "retention mode 1"]
    MEMLP_RET_SEL_1,
}
impl MEMLP_RET_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MEMLP_RET_SELW::MEMLP_RET_SEL_0 => false,
            MEMLP_RET_SELW::MEMLP_RET_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MEMLP_RET_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _MEMLP_RET_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MEMLP_RET_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "retention mode 2"]
    #[inline]
    pub fn memlp_ret_sel_0(self) -> &'a mut W {
        self.variant(MEMLP_RET_SELW::MEMLP_RET_SEL_0)
    }
    #[doc = "retention mode 1"]
    #[inline]
    pub fn memlp_ret_sel_1(self) -> &'a mut W {
        self.variant(MEMLP_RET_SELW::MEMLP_RET_SEL_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ROMLP_PDN_DIS`"]
pub enum ROMLP_PDN_DISW {
    #[doc = "Enable ROM shut down control(should also enable RAM low power control);"]
    ROMLP_PDN_DIS_0,
    #[doc = "Disable ROM shut down control"]
    ROMLP_PDN_DIS_1,
}
impl ROMLP_PDN_DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ROMLP_PDN_DISW::ROMLP_PDN_DIS_0 => false,
            ROMLP_PDN_DISW::ROMLP_PDN_DIS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ROMLP_PDN_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _ROMLP_PDN_DISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ROMLP_PDN_DISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable ROM shut down control(should also enable RAM low power control);"]
    #[inline]
    pub fn romlp_pdn_dis_0(self) -> &'a mut W {
        self.variant(ROMLP_PDN_DISW::ROMLP_PDN_DIS_0)
    }
    #[doc = "Disable ROM shut down control"]
    #[inline]
    pub fn romlp_pdn_dis_1(self) -> &'a mut W {
        self.variant(ROMLP_PDN_DISW::ROMLP_PDN_DIS_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MEMLP_ENT_CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _MEMLP_ENT_CNTW<'a> {
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
pub struct _MEM_EXT_CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _MEM_EXT_CNTW<'a> {
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
pub struct _MEMLP_RET_PGENW<'a> {
    w: &'a mut W,
}
impl<'a> _MEMLP_RET_PGENW<'a> {
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
    #[doc = "Bit 0 - RAM low-power control"]
    #[inline]
    pub fn memlp_ctl_dis(&self) -> MEMLP_CTL_DISR {
        MEMLP_CTL_DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Retention select"]
    #[inline]
    pub fn memlp_ret_sel(&self) -> MEMLP_RET_SELR {
        MEMLP_RET_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - ROM shut down control"]
    #[inline]
    pub fn romlp_pdn_dis(&self) -> ROMLP_PDN_DISR {
        ROMLP_PDN_DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:15 - Delay counter to make sure all clock off after pll_dis_req is issued by smc"]
    #[inline]
    pub fn memlp_ent_cnt(&self) -> MEMLP_ENT_CNTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MEMLP_ENT_CNTR { bits }
    }
    #[doc = "Bits 16:23 - Delay counter to start existing from memory low power"]
    #[inline]
    pub fn mem_ext_cnt(&self) -> MEM_EXT_CNTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MEM_EXT_CNTR { bits }
    }
    #[doc = "Bits 24:31 - Delay counter for \"retnx\" and \"pgen\""]
    #[inline]
    pub fn memlp_ret_pgen(&self) -> MEMLP_RET_PGENR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MEMLP_RET_PGENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 16843008 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - RAM low-power control"]
    #[inline]
    pub fn memlp_ctl_dis(&mut self) -> _MEMLP_CTL_DISW {
        _MEMLP_CTL_DISW { w: self }
    }
    #[doc = "Bit 1 - Retention select"]
    #[inline]
    pub fn memlp_ret_sel(&mut self) -> _MEMLP_RET_SELW {
        _MEMLP_RET_SELW { w: self }
    }
    #[doc = "Bit 2 - ROM shut down control"]
    #[inline]
    pub fn romlp_pdn_dis(&mut self) -> _ROMLP_PDN_DISW {
        _ROMLP_PDN_DISW { w: self }
    }
    #[doc = "Bits 8:15 - Delay counter to make sure all clock off after pll_dis_req is issued by smc"]
    #[inline]
    pub fn memlp_ent_cnt(&mut self) -> _MEMLP_ENT_CNTW {
        _MEMLP_ENT_CNTW { w: self }
    }
    #[doc = "Bits 16:23 - Delay counter to start existing from memory low power"]
    #[inline]
    pub fn mem_ext_cnt(&mut self) -> _MEM_EXT_CNTW {
        _MEM_EXT_CNTW { w: self }
    }
    #[doc = "Bits 24:31 - Delay counter for \"retnx\" and \"pgen\""]
    #[inline]
    pub fn memlp_ret_pgen(&mut self) -> _MEMLP_RET_PGENW {
        _MEMLP_RET_PGENW { w: self }
    }
}
