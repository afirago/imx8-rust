#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONFIG {
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
#[doc = "Possible values of the field `CSM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSMR {
    #[doc = "static"]
    CSM_0,
    #[doc = "dynamic low power"]
    CSM_1,
    #[doc = "dynamic with no loop"]
    CSM_2,
    #[doc = "dynamic"]
    CSM_3,
}
impl CSMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSMR::CSM_0 => 0,
            CSMR::CSM_1 => 1,
            CSMR::CSM_2 => 2,
            CSMR::CSM_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSMR {
        match value {
            0 => CSMR::CSM_0,
            1 => CSMR::CSM_1,
            2 => CSMR::CSM_2,
            3 => CSMR::CSM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CSM_0`"]
    #[inline]
    pub fn is_csm_0(&self) -> bool {
        *self == CSMR::CSM_0
    }
    #[doc = "Checks if the value of the field is `CSM_1`"]
    #[inline]
    pub fn is_csm_1(&self) -> bool {
        *self == CSMR::CSM_1
    }
    #[doc = "Checks if the value of the field is `CSM_2`"]
    #[inline]
    pub fn is_csm_2(&self) -> bool {
        *self == CSMR::CSM_2
    }
    #[doc = "Checks if the value of the field is `CSM_3`"]
    #[inline]
    pub fn is_csm_3(&self) -> bool {
        *self == CSMR::CSM_3
    }
}
#[doc = "Possible values of the field `ACR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACRR {
    #[doc = "Arm platform DMA interface frequency equals twice core frequency"]
    ACR_0,
    #[doc = "Arm platform DMA interface frequency equals core frequency"]
    ACR_1,
}
impl ACRR {
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
            ACRR::ACR_0 => false,
            ACRR::ACR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACRR {
        match value {
            false => ACRR::ACR_0,
            true => ACRR::ACR_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACR_0`"]
    #[inline]
    pub fn is_acr_0(&self) -> bool {
        *self == ACRR::ACR_0
    }
    #[doc = "Checks if the value of the field is `ACR_1`"]
    #[inline]
    pub fn is_acr_1(&self) -> bool {
        *self == ACRR::ACR_1
    }
}
#[doc = "Possible values of the field `RTDOBS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTDOBSR {
    #[doc = "RTD pins disabled"]
    RTDOBS_0,
    #[doc = "RTD pins enabled"]
    RTDOBS_1,
}
impl RTDOBSR {
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
            RTDOBSR::RTDOBS_0 => false,
            RTDOBSR::RTDOBS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTDOBSR {
        match value {
            false => RTDOBSR::RTDOBS_0,
            true => RTDOBSR::RTDOBS_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTDOBS_0`"]
    #[inline]
    pub fn is_rtdobs_0(&self) -> bool {
        *self == RTDOBSR::RTDOBS_0
    }
    #[doc = "Checks if the value of the field is `RTDOBS_1`"]
    #[inline]
    pub fn is_rtdobs_1(&self) -> bool {
        *self == RTDOBSR::RTDOBS_1
    }
}
#[doc = "Possible values of the field `DSPDMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSPDMAR {
    #[doc = "- Reset Value"]
    DSPDMA_0,
    #[doc = "- Reserved"]
    DSPDMA_1,
}
impl DSPDMAR {
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
            DSPDMAR::DSPDMA_0 => false,
            DSPDMAR::DSPDMA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DSPDMAR {
        match value {
            false => DSPDMAR::DSPDMA_0,
            true => DSPDMAR::DSPDMA_1,
        }
    }
    #[doc = "Checks if the value of the field is `DSPDMA_0`"]
    #[inline]
    pub fn is_dspdma_0(&self) -> bool {
        *self == DSPDMAR::DSPDMA_0
    }
    #[doc = "Checks if the value of the field is `DSPDMA_1`"]
    #[inline]
    pub fn is_dspdma_1(&self) -> bool {
        *self == DSPDMAR::DSPDMA_1
    }
}
#[doc = "Values that can be written to the field `CSM`"]
pub enum CSMW {
    #[doc = "static"]
    CSM_0,
    #[doc = "dynamic low power"]
    CSM_1,
    #[doc = "dynamic with no loop"]
    CSM_2,
    #[doc = "dynamic"]
    CSM_3,
}
impl CSMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSMW::CSM_0 => 0,
            CSMW::CSM_1 => 1,
            CSMW::CSM_2 => 2,
            CSMW::CSM_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSMW<'a> {
    w: &'a mut W,
}
impl<'a> _CSMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "static"]
    #[inline]
    pub fn csm_0(self) -> &'a mut W {
        self.variant(CSMW::CSM_0)
    }
    #[doc = "dynamic low power"]
    #[inline]
    pub fn csm_1(self) -> &'a mut W {
        self.variant(CSMW::CSM_1)
    }
    #[doc = "dynamic with no loop"]
    #[inline]
    pub fn csm_2(self) -> &'a mut W {
        self.variant(CSMW::CSM_2)
    }
    #[doc = "dynamic"]
    #[inline]
    pub fn csm_3(self) -> &'a mut W {
        self.variant(CSMW::CSM_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACR`"]
pub enum ACRW {
    #[doc = "Arm platform DMA interface frequency equals twice core frequency"]
    ACR_0,
    #[doc = "Arm platform DMA interface frequency equals core frequency"]
    ACR_1,
}
impl ACRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACRW::ACR_0 => false,
            ACRW::ACR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACRW<'a> {
    w: &'a mut W,
}
impl<'a> _ACRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Arm platform DMA interface frequency equals twice core frequency"]
    #[inline]
    pub fn acr_0(self) -> &'a mut W {
        self.variant(ACRW::ACR_0)
    }
    #[doc = "Arm platform DMA interface frequency equals core frequency"]
    #[inline]
    pub fn acr_1(self) -> &'a mut W {
        self.variant(ACRW::ACR_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTDOBS`"]
pub enum RTDOBSW {
    #[doc = "RTD pins disabled"]
    RTDOBS_0,
    #[doc = "RTD pins enabled"]
    RTDOBS_1,
}
impl RTDOBSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTDOBSW::RTDOBS_0 => false,
            RTDOBSW::RTDOBS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTDOBSW<'a> {
    w: &'a mut W,
}
impl<'a> _RTDOBSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTDOBSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RTD pins disabled"]
    #[inline]
    pub fn rtdobs_0(self) -> &'a mut W {
        self.variant(RTDOBSW::RTDOBS_0)
    }
    #[doc = "RTD pins enabled"]
    #[inline]
    pub fn rtdobs_1(self) -> &'a mut W {
        self.variant(RTDOBSW::RTDOBS_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DSPDMA`"]
pub enum DSPDMAW {
    #[doc = "- Reset Value"]
    DSPDMA_0,
    #[doc = "- Reserved"]
    DSPDMA_1,
}
impl DSPDMAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DSPDMAW::DSPDMA_0 => false,
            DSPDMAW::DSPDMA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSPDMAW<'a> {
    w: &'a mut W,
}
impl<'a> _DSPDMAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSPDMAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "- Reset Value"]
    #[inline]
    pub fn dspdma_0(self) -> &'a mut W {
        self.variant(DSPDMAW::DSPDMA_0)
    }
    #[doc = "- Reserved"]
    #[inline]
    pub fn dspdma_1(self) -> &'a mut W {
        self.variant(DSPDMAW::DSPDMA_1)
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
        const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:1 - Selects the Context Switch Mode"]
    #[inline]
    pub fn csm(&self) -> CSMR {
        CSMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Arm platform DMA / SDMA Core Clock Ratio"]
    #[inline]
    pub fn acr(&self) -> ACRR {
        ACRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Indicates if Real-Time Debug pins are used: They do not toggle by default in order to reduce power consumption"]
    #[inline]
    pub fn rtdobs(&self) -> RTDOBSR {
        RTDOBSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - This bit's function is reserved and should be configured as zero."]
    #[inline]
    pub fn dspdma(&self) -> DSPDMAR {
        DSPDMAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Selects the Context Switch Mode"]
    #[inline]
    pub fn csm(&mut self) -> _CSMW {
        _CSMW { w: self }
    }
    #[doc = "Bit 4 - Arm platform DMA / SDMA Core Clock Ratio"]
    #[inline]
    pub fn acr(&mut self) -> _ACRW {
        _ACRW { w: self }
    }
    #[doc = "Bit 11 - Indicates if Real-Time Debug pins are used: They do not toggle by default in order to reduce power consumption"]
    #[inline]
    pub fn rtdobs(&mut self) -> _RTDOBSW {
        _RTDOBSW { w: self }
    }
    #[doc = "Bit 12 - This bit's function is reserved and should be configured as zero."]
    #[inline]
    pub fn dspdma(&mut self) -> _DSPDMAW {
        _DSPDMAW { w: self }
    }
}
