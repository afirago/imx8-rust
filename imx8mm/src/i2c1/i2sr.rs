#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::I2SR {
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
#[doc = "Possible values of the field `RXAK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXAKR {
    #[doc = "An \"acknowledge\" signal was received after the completion of an 8-bit data transmission on the bus."]
    RXAK_0,
    #[doc = "A \"No acknowledge\" signal was detected at the ninth clock."]
    RXAK_1,
}
impl RXAKR {
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
            RXAKR::RXAK_0 => false,
            RXAKR::RXAK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXAKR {
        match value {
            false => RXAKR::RXAK_0,
            true => RXAKR::RXAK_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXAK_0`"]
    #[inline]
    pub fn is_rxak_0(&self) -> bool {
        *self == RXAKR::RXAK_0
    }
    #[doc = "Checks if the value of the field is `RXAK_1`"]
    #[inline]
    pub fn is_rxak_1(&self) -> bool {
        *self == RXAKR::RXAK_1
    }
}
#[doc = "Possible values of the field `IIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IIFR {
    #[doc = "No I2C interrupt pending."]
    IIF_0,
    #[doc = "An interrupt is pending.This causes a processor interrupt request (if the interrupt enable is asserted \\[IIEN = 1\\]). The interrupt is set when one of the following occurs: One byte transfer is completed (the interrupt is set at the falling edge of the ninth clock). An address is received that matches its own specific address in Slave Receive mode. Arbitration is lost."]
    IIF_1,
}
impl IIFR {
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
            IIFR::IIF_0 => false,
            IIFR::IIF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IIFR {
        match value {
            false => IIFR::IIF_0,
            true => IIFR::IIF_1,
        }
    }
    #[doc = "Checks if the value of the field is `IIF_0`"]
    #[inline]
    pub fn is_iif_0(&self) -> bool {
        *self == IIFR::IIF_0
    }
    #[doc = "Checks if the value of the field is `IIF_1`"]
    #[inline]
    pub fn is_iif_1(&self) -> bool {
        *self == IIFR::IIF_1
    }
}
#[doc = "Possible values of the field `SRW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRWR {
    #[doc = "Slave receive, master writing to slave"]
    SRW_0,
    #[doc = "Slave transmit, master reading from slave"]
    SRW_1,
}
impl SRWR {
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
            SRWR::SRW_0 => false,
            SRWR::SRW_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRWR {
        match value {
            false => SRWR::SRW_0,
            true => SRWR::SRW_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRW_0`"]
    #[inline]
    pub fn is_srw_0(&self) -> bool {
        *self == SRWR::SRW_0
    }
    #[doc = "Checks if the value of the field is `SRW_1`"]
    #[inline]
    pub fn is_srw_1(&self) -> bool {
        *self == SRWR::SRW_1
    }
}
#[doc = "Possible values of the field `IAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IALR {
    #[doc = "No arbitration lost."]
    IAL_0,
    #[doc = "Arbitration is lost."]
    IAL_1,
}
impl IALR {
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
            IALR::IAL_0 => false,
            IALR::IAL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IALR {
        match value {
            false => IALR::IAL_0,
            true => IALR::IAL_1,
        }
    }
    #[doc = "Checks if the value of the field is `IAL_0`"]
    #[inline]
    pub fn is_ial_0(&self) -> bool {
        *self == IALR::IAL_0
    }
    #[doc = "Checks if the value of the field is `IAL_1`"]
    #[inline]
    pub fn is_ial_1(&self) -> bool {
        *self == IALR::IAL_1
    }
}
#[doc = "Possible values of the field `IBB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBBR {
    #[doc = "Bus is idle. If a Stop signal is detected, IBB is cleared."]
    IBB_0,
    #[doc = "Bus is busy. When Start is detected, IBB is set."]
    IBB_1,
}
impl IBBR {
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
            IBBR::IBB_0 => false,
            IBBR::IBB_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IBBR {
        match value {
            false => IBBR::IBB_0,
            true => IBBR::IBB_1,
        }
    }
    #[doc = "Checks if the value of the field is `IBB_0`"]
    #[inline]
    pub fn is_ibb_0(&self) -> bool {
        *self == IBBR::IBB_0
    }
    #[doc = "Checks if the value of the field is `IBB_1`"]
    #[inline]
    pub fn is_ibb_1(&self) -> bool {
        *self == IBBR::IBB_1
    }
}
#[doc = "Possible values of the field `IAAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IAASR {
    #[doc = "Not addressed"]
    IAAS_0,
    #[doc = "Addressed as a slave. Set when its own address (I2C_IADR) matches the calling address."]
    IAAS_1,
}
impl IAASR {
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
            IAASR::IAAS_0 => false,
            IAASR::IAAS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IAASR {
        match value {
            false => IAASR::IAAS_0,
            true => IAASR::IAAS_1,
        }
    }
    #[doc = "Checks if the value of the field is `IAAS_0`"]
    #[inline]
    pub fn is_iaas_0(&self) -> bool {
        *self == IAASR::IAAS_0
    }
    #[doc = "Checks if the value of the field is `IAAS_1`"]
    #[inline]
    pub fn is_iaas_1(&self) -> bool {
        *self == IAASR::IAAS_1
    }
}
#[doc = "Possible values of the field `ICF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICFR {
    #[doc = "Transfer is in progress."]
    ICF_0,
    #[doc = "Transfer is complete. This bit is set by the falling edge of the ninth clock of the last byte transfer."]
    ICF_1,
}
impl ICFR {
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
            ICFR::ICF_0 => false,
            ICFR::ICF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ICFR {
        match value {
            false => ICFR::ICF_0,
            true => ICFR::ICF_1,
        }
    }
    #[doc = "Checks if the value of the field is `ICF_0`"]
    #[inline]
    pub fn is_icf_0(&self) -> bool {
        *self == ICFR::ICF_0
    }
    #[doc = "Checks if the value of the field is `ICF_1`"]
    #[inline]
    pub fn is_icf_1(&self) -> bool {
        *self == ICFR::ICF_1
    }
}
#[doc = "Values that can be written to the field `IIF`"]
pub enum IIFW {
    #[doc = "No I2C interrupt pending."]
    IIF_0,
    #[doc = "An interrupt is pending.This causes a processor interrupt request (if the interrupt enable is asserted \\[IIEN = 1\\]). The interrupt is set when one of the following occurs: One byte transfer is completed (the interrupt is set at the falling edge of the ninth clock). An address is received that matches its own specific address in Slave Receive mode. Arbitration is lost."]
    IIF_1,
}
impl IIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IIFW::IIF_0 => false,
            IIFW::IIF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IIFW<'a> {
    w: &'a mut W,
}
impl<'a> _IIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No I2C interrupt pending."]
    #[inline]
    pub fn iif_0(self) -> &'a mut W {
        self.variant(IIFW::IIF_0)
    }
    #[doc = "An interrupt is pending.This causes a processor interrupt request (if the interrupt enable is asserted \\[IIEN = 1\\]). The interrupt is set when one of the following occurs: One byte transfer is completed (the interrupt is set at the falling edge of the ninth clock). An address is received that matches its own specific address in Slave Receive mode. Arbitration is lost."]
    #[inline]
    pub fn iif_1(self) -> &'a mut W {
        self.variant(IIFW::IIF_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IAL`"]
pub enum IALW {
    #[doc = "No arbitration lost."]
    IAL_0,
    #[doc = "Arbitration is lost."]
    IAL_1,
}
impl IALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IALW::IAL_0 => false,
            IALW::IAL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IALW<'a> {
    w: &'a mut W,
}
impl<'a> _IALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No arbitration lost."]
    #[inline]
    pub fn ial_0(self) -> &'a mut W {
        self.variant(IALW::IAL_0)
    }
    #[doc = "Arbitration is lost."]
    #[inline]
    pub fn ial_1(self) -> &'a mut W {
        self.variant(IALW::IAL_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Received acknowledge"]
    #[inline]
    pub fn rxak(&self) -> RXAKR {
        RXAKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - I2C interrupt"]
    #[inline]
    pub fn iif(&self) -> IIFR {
        IIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - Slave read/write"]
    #[inline]
    pub fn srw(&self) -> SRWR {
        SRWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Arbitration lost"]
    #[inline]
    pub fn ial(&self) -> IALR {
        IALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - I2C bus busy bit"]
    #[inline]
    pub fn ibb(&self) -> IBBR {
        IBBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - I2C addressed as a slave bit"]
    #[inline]
    pub fn iaas(&self) -> IAASR {
        IAASR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - Data transferring bit. While one byte of data is transferred, ICF is cleared."]
    #[inline]
    pub fn icf(&self) -> ICFR {
        ICFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 129 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - I2C interrupt"]
    #[inline]
    pub fn iif(&mut self) -> _IIFW {
        _IIFW { w: self }
    }
    #[doc = "Bit 4 - Arbitration lost"]
    #[inline]
    pub fn ial(&mut self) -> _IALW {
        _IALW { w: self }
    }
}
