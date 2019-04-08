#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SDMA_LOCK {
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
#[doc = "Possible values of the field `LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKR {
    #[doc = "LOCK disengaged."]
    LOCK_0,
    #[doc = "LOCK enabled."]
    LOCK_1,
}
impl LOCKR {
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
            LOCKR::LOCK_0 => false,
            LOCKR::LOCK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCKR {
        match value {
            false => LOCKR::LOCK_0,
            true => LOCKR::LOCK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_0`"]
    #[inline]
    pub fn is_lock_0(&self) -> bool {
        *self == LOCKR::LOCK_0
    }
    #[doc = "Checks if the value of the field is `LOCK_1`"]
    #[inline]
    pub fn is_lock_1(&self) -> bool {
        *self == LOCKR::LOCK_1
    }
}
#[doc = "Possible values of the field `SRESET_LOCK_CLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRESET_LOCK_CLRR {
    #[doc = "Software Reset does not clear the LOCK bit."]
    SRESET_LOCK_CLR_0,
    #[doc = "Software Reset clears the LOCK bit."]
    SRESET_LOCK_CLR_1,
}
impl SRESET_LOCK_CLRR {
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
            SRESET_LOCK_CLRR::SRESET_LOCK_CLR_0 => false,
            SRESET_LOCK_CLRR::SRESET_LOCK_CLR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRESET_LOCK_CLRR {
        match value {
            false => SRESET_LOCK_CLRR::SRESET_LOCK_CLR_0,
            true => SRESET_LOCK_CLRR::SRESET_LOCK_CLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRESET_LOCK_CLR_0`"]
    #[inline]
    pub fn is_sreset_lock_clr_0(&self) -> bool {
        *self == SRESET_LOCK_CLRR::SRESET_LOCK_CLR_0
    }
    #[doc = "Checks if the value of the field is `SRESET_LOCK_CLR_1`"]
    #[inline]
    pub fn is_sreset_lock_clr_1(&self) -> bool {
        *self == SRESET_LOCK_CLRR::SRESET_LOCK_CLR_1
    }
}
#[doc = "Values that can be written to the field `LOCK`"]
pub enum LOCKW {
    #[doc = "LOCK disengaged."]
    LOCK_0,
    #[doc = "LOCK enabled."]
    LOCK_1,
}
impl LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCKW::LOCK_0 => false,
            LOCKW::LOCK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LOCK disengaged."]
    #[inline]
    pub fn lock_0(self) -> &'a mut W {
        self.variant(LOCKW::LOCK_0)
    }
    #[doc = "LOCK enabled."]
    #[inline]
    pub fn lock_1(self) -> &'a mut W {
        self.variant(LOCKW::LOCK_1)
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
#[doc = "Values that can be written to the field `SRESET_LOCK_CLR`"]
pub enum SRESET_LOCK_CLRW {
    #[doc = "Software Reset does not clear the LOCK bit."]
    SRESET_LOCK_CLR_0,
    #[doc = "Software Reset clears the LOCK bit."]
    SRESET_LOCK_CLR_1,
}
impl SRESET_LOCK_CLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRESET_LOCK_CLRW::SRESET_LOCK_CLR_0 => false,
            SRESET_LOCK_CLRW::SRESET_LOCK_CLR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRESET_LOCK_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _SRESET_LOCK_CLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRESET_LOCK_CLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Software Reset does not clear the LOCK bit."]
    #[inline]
    pub fn sreset_lock_clr_0(self) -> &'a mut W {
        self.variant(SRESET_LOCK_CLRW::SRESET_LOCK_CLR_0)
    }
    #[doc = "Software Reset clears the LOCK bit."]
    #[inline]
    pub fn sreset_lock_clr_1(self) -> &'a mut W {
        self.variant(SRESET_LOCK_CLRW::SRESET_LOCK_CLR_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - The LOCK bit is used to restrict access to update SDMA script memory through ROM channel zero scripts and through the OnCE interface under Arm platform control"]
    #[inline]
    pub fn lock(&self) -> LOCKR {
        LOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - The SRESET_LOCK_CLR bit determine if the LOCK bit is cleared on a software reset triggered by writing to the RESET register"]
    #[inline]
    pub fn sreset_lock_clr(&self) -> SRESET_LOCK_CLRR {
        SRESET_LOCK_CLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bit 0 - The LOCK bit is used to restrict access to update SDMA script memory through ROM channel zero scripts and through the OnCE interface under Arm platform control"]
    #[inline]
    pub fn lock(&mut self) -> _LOCKW {
        _LOCKW { w: self }
    }
    #[doc = "Bit 1 - The SRESET_LOCK_CLR bit determine if the LOCK bit is cleared on a software reset triggered by writing to the RESET register"]
    #[inline]
    pub fn sreset_lock_clr(&mut self) -> _SRESET_LOCK_CLRW {
        _SRESET_LOCK_CLRW { w: self }
    }
}
