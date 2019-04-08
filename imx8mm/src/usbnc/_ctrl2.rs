#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::_CTRL2 {
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
pub struct VBUS_SOURCE_SELR {
    bits: u8,
}
impl VBUS_SOURCE_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `AUTURESUME_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTURESUME_ENR {
    #[doc = "Default"]
    AUTURESUME_EN_0,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl AUTURESUME_ENR {
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
            AUTURESUME_ENR::AUTURESUME_EN_0 => false,
            AUTURESUME_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUTURESUME_ENR {
        match value {
            false => AUTURESUME_ENR::AUTURESUME_EN_0,
            i => AUTURESUME_ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AUTURESUME_EN_0`"]
    #[inline]
    pub fn is_auturesume_en_0(&self) -> bool {
        *self == AUTURESUME_ENR::AUTURESUME_EN_0
    }
}
#[doc = "Possible values of the field `LOWSPEED_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOWSPEED_ENR {
    #[doc = "Default"]
    LOWSPEED_EN_0,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl LOWSPEED_ENR {
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
            LOWSPEED_ENR::LOWSPEED_EN_0 => false,
            LOWSPEED_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOWSPEED_ENR {
        match value {
            false => LOWSPEED_ENR::LOWSPEED_EN_0,
            i => LOWSPEED_ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOWSPEED_EN_0`"]
    #[inline]
    pub fn is_lowspeed_en_0(&self) -> bool {
        *self == LOWSPEED_ENR::LOWSPEED_EN_0
    }
}
#[doc = "Possible values of the field `UTMI_CLK_VLD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UTMI_CLK_VLDR {
    #[doc = "Default"]
    UTMI_CLK_VLD_0,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UTMI_CLK_VLDR {
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
            UTMI_CLK_VLDR::UTMI_CLK_VLD_0 => false,
            UTMI_CLK_VLDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UTMI_CLK_VLDR {
        match value {
            false => UTMI_CLK_VLDR::UTMI_CLK_VLD_0,
            i => UTMI_CLK_VLDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UTMI_CLK_VLD_0`"]
    #[inline]
    pub fn is_utmi_clk_vld_0(&self) -> bool {
        *self == UTMI_CLK_VLDR::UTMI_CLK_VLD_0
    }
}
#[doc = r" Proxy"]
pub struct _VBUS_SOURCE_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _VBUS_SOURCE_SELW<'a> {
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
#[doc = "Values that can be written to the field `AUTURESUME_EN`"]
pub enum AUTURESUME_ENW {
    #[doc = "Default"]
    AUTURESUME_EN_0,
}
impl AUTURESUME_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUTURESUME_ENW::AUTURESUME_EN_0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUTURESUME_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTURESUME_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUTURESUME_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Default"]
    #[inline]
    pub fn auturesume_en_0(self) -> &'a mut W {
        self.variant(AUTURESUME_ENW::AUTURESUME_EN_0)
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
#[doc = "Values that can be written to the field `LOWSPEED_EN`"]
pub enum LOWSPEED_ENW {
    #[doc = "Default"]
    LOWSPEED_EN_0,
}
impl LOWSPEED_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOWSPEED_ENW::LOWSPEED_EN_0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOWSPEED_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _LOWSPEED_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOWSPEED_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Default"]
    #[inline]
    pub fn lowspeed_en_0(self) -> &'a mut W {
        self.variant(LOWSPEED_ENW::LOWSPEED_EN_0)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UTMI_CLK_VLD`"]
pub enum UTMI_CLK_VLDW {
    #[doc = "Default"]
    UTMI_CLK_VLD_0,
}
impl UTMI_CLK_VLDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UTMI_CLK_VLDW::UTMI_CLK_VLD_0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UTMI_CLK_VLDW<'a> {
    w: &'a mut W,
}
impl<'a> _UTMI_CLK_VLDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UTMI_CLK_VLDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Default"]
    #[inline]
    pub fn utmi_clk_vld_0(self) -> &'a mut W {
        self.variant(UTMI_CLK_VLDW::UTMI_CLK_VLD_0)
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
    #[doc = "Bits 0:1 - VBUS source select when detect VBUS wakeup event, it is for UTMI PHY only (UH core has no such feature)"]
    #[inline]
    pub fn vbus_source_sel(&self) -> VBUS_SOURCE_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VBUS_SOURCE_SELR { bits }
    }
    #[doc = "Bit 2 - Auto Resume Enable"]
    #[inline]
    pub fn auturesume_en(&self) -> AUTURESUME_ENR {
        AUTURESUME_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Set if AUTURESUME_EN is set and works on low speed."]
    #[inline]
    pub fn lowspeed_en(&self) -> LOWSPEED_ENR {
        LOWSPEED_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Indicate whether the UTMI clock to the USB PHY is valid. Write 1 to clear"]
    #[inline]
    pub fn utmi_clk_vld(&self) -> UTMI_CLK_VLDR {
        UTMI_CLK_VLDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1593835520 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - VBUS source select when detect VBUS wakeup event, it is for UTMI PHY only (UH core has no such feature)"]
    #[inline]
    pub fn vbus_source_sel(&mut self) -> _VBUS_SOURCE_SELW {
        _VBUS_SOURCE_SELW { w: self }
    }
    #[doc = "Bit 2 - Auto Resume Enable"]
    #[inline]
    pub fn auturesume_en(&mut self) -> _AUTURESUME_ENW {
        _AUTURESUME_ENW { w: self }
    }
    #[doc = "Bit 3 - Set if AUTURESUME_EN is set and works on low speed."]
    #[inline]
    pub fn lowspeed_en(&mut self) -> _LOWSPEED_ENW {
        _LOWSPEED_ENW { w: self }
    }
    #[doc = "Bit 31 - Indicate whether the UTMI clock to the USB PHY is valid. Write 1 to clear"]
    #[inline]
    pub fn utmi_clk_vld(&mut self) -> _UTMI_CLK_VLDW {
        _UTMI_CLK_VLDW { w: self }
    }
}
