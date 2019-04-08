#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TIER {
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
#[doc = "Possible values of the field `ATCTEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATCTEIER {
    #[doc = "Disabled."]
    ATCTEIE_0,
    #[doc = "Interrupt enabled. Generate an interrupt if TIDR\\[ATCTE\\] is set. Write 1 to this bit will clear bit TIDR\\[ATCTE\\]."]
    ATCTEIE_1,
}
impl ATCTEIER {
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
            ATCTEIER::ATCTEIE_0 => false,
            ATCTEIER::ATCTEIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ATCTEIER {
        match value {
            false => ATCTEIER::ATCTEIE_0,
            true => ATCTEIER::ATCTEIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ATCTEIE_0`"]
    #[inline]
    pub fn is_atcteie_0(&self) -> bool {
        *self == ATCTEIER::ATCTEIE_0
    }
    #[doc = "Checks if the value of the field is `ATCTEIE_1`"]
    #[inline]
    pub fn is_atcteie_1(&self) -> bool {
        *self == ATCTEIER::ATCTEIE_1
    }
}
#[doc = "Possible values of the field `ATTEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATTEIER {
    #[doc = "Disabled."]
    ATTEIE_0,
    #[doc = "Interrupt enabled. Generate an interrupt if TIDR\\[ATTE\\] is set. Write 1 to this bit will clear bit TIDR\\[ATTE\\]."]
    ATTEIE_1,
}
impl ATTEIER {
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
            ATTEIER::ATTEIE_0 => false,
            ATTEIER::ATTEIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ATTEIER {
        match value {
            false => ATTEIER::ATTEIE_0,
            true => ATTEIER::ATTEIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ATTEIE_0`"]
    #[inline]
    pub fn is_atteie_0(&self) -> bool {
        *self == ATTEIER::ATTEIE_0
    }
    #[doc = "Checks if the value of the field is `ATTEIE_1`"]
    #[inline]
    pub fn is_atteie_1(&self) -> bool {
        *self == ATTEIER::ATTEIE_1
    }
}
#[doc = "Possible values of the field `ITTEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITTEIER {
    #[doc = "Disabled."]
    ITTEIE_0,
    #[doc = "Interrupt enabled. Generate an interrupt if TIDR\\[ITTE\\] is set. Write 1 to this bit will clear bit TIDR\\[ITTE\\]."]
    ITTEIE_1,
}
impl ITTEIER {
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
            ITTEIER::ITTEIE_0 => false,
            ITTEIER::ITTEIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ITTEIER {
        match value {
            false => ITTEIER::ITTEIE_0,
            true => ITTEIER::ITTEIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ITTEIE_0`"]
    #[inline]
    pub fn is_itteie_0(&self) -> bool {
        *self == ITTEIER::ITTEIE_0
    }
    #[doc = "Checks if the value of the field is `ITTEIE_1`"]
    #[inline]
    pub fn is_itteie_1(&self) -> bool {
        *self == ITTEIER::ITTEIE_1
    }
}
#[doc = "Values that can be written to the field `ATCTEIE`"]
pub enum ATCTEIEW {
    #[doc = "Disabled."]
    ATCTEIE_0,
    #[doc = "Interrupt enabled. Generate an interrupt if TIDR\\[ATCTE\\] is set. Write 1 to this bit will clear bit TIDR\\[ATCTE\\]."]
    ATCTEIE_1,
}
impl ATCTEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ATCTEIEW::ATCTEIE_0 => false,
            ATCTEIEW::ATCTEIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ATCTEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ATCTEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ATCTEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn atcteie_0(self) -> &'a mut W {
        self.variant(ATCTEIEW::ATCTEIE_0)
    }
    #[doc = "Interrupt enabled. Generate an interrupt if TIDR\\[ATCTE\\] is set. Write 1 to this bit will clear bit TIDR\\[ATCTE\\]."]
    #[inline]
    pub fn atcteie_1(self) -> &'a mut W {
        self.variant(ATCTEIEW::ATCTEIE_1)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ATTEIE`"]
pub enum ATTEIEW {
    #[doc = "Disabled."]
    ATTEIE_0,
    #[doc = "Interrupt enabled. Generate an interrupt if TIDR\\[ATTE\\] is set. Write 1 to this bit will clear bit TIDR\\[ATTE\\]."]
    ATTEIE_1,
}
impl ATTEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ATTEIEW::ATTEIE_0 => false,
            ATTEIEW::ATTEIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ATTEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ATTEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ATTEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn atteie_0(self) -> &'a mut W {
        self.variant(ATTEIEW::ATTEIE_0)
    }
    #[doc = "Interrupt enabled. Generate an interrupt if TIDR\\[ATTE\\] is set. Write 1 to this bit will clear bit TIDR\\[ATTE\\]."]
    #[inline]
    pub fn atteie_1(self) -> &'a mut W {
        self.variant(ATTEIEW::ATTEIE_1)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ITTEIE`"]
pub enum ITTEIEW {
    #[doc = "Disabled."]
    ITTEIE_0,
    #[doc = "Interrupt enabled. Generate an interrupt if TIDR\\[ITTE\\] is set. Write 1 to this bit will clear bit TIDR\\[ITTE\\]."]
    ITTEIE_1,
}
impl ITTEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ITTEIEW::ITTEIE_0 => false,
            ITTEIEW::ITTEIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ITTEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ITTEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ITTEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn itteie_0(self) -> &'a mut W {
        self.variant(ITTEIEW::ITTEIE_0)
    }
    #[doc = "Interrupt enabled. Generate an interrupt if TIDR\\[ITTE\\] is set. Write 1 to this bit will clear bit TIDR\\[ITTE\\]."]
    #[inline]
    pub fn itteie_1(self) -> &'a mut W {
        self.variant(ITTEIEW::ITTEIE_1)
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
    #[doc = "Bit 29 - Average temperature critical threshold exceeded interrupt enable."]
    #[inline]
    pub fn atcteie(&self) -> ATCTEIER {
        ATCTEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Average temperature threshold exceeded interrupt enable."]
    #[inline]
    pub fn atteie(&self) -> ATTEIER {
        ATTEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Immediate temperature threshold exceeded interrupt enable."]
    #[inline]
    pub fn itteie(&self) -> ITTEIER {
        ITTEIER::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 29 - Average temperature critical threshold exceeded interrupt enable."]
    #[inline]
    pub fn atcteie(&mut self) -> _ATCTEIEW {
        _ATCTEIEW { w: self }
    }
    #[doc = "Bit 30 - Average temperature threshold exceeded interrupt enable."]
    #[inline]
    pub fn atteie(&mut self) -> _ATTEIEW {
        _ATTEIEW { w: self }
    }
    #[doc = "Bit 31 - Immediate temperature threshold exceeded interrupt enable."]
    #[inline]
    pub fn itteie(&mut self) -> _ITTEIEW {
        _ITTEIEW { w: self }
    }
}
