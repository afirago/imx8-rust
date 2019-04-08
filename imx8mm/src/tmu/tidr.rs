#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TIDR {
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
#[doc = "Possible values of the field `ATCTE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATCTER {
    #[doc = "No threshold exceeded."]
    ATCTE_0,
    #[doc = "Average temperature critical threshold, as defined by TMHTACTR, has been exceeded."]
    ATCTE_1,
}
impl ATCTER {
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
            ATCTER::ATCTE_0 => false,
            ATCTER::ATCTE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ATCTER {
        match value {
            false => ATCTER::ATCTE_0,
            true => ATCTER::ATCTE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ATCTE_0`"]
    #[inline]
    pub fn is_atcte_0(&self) -> bool {
        *self == ATCTER::ATCTE_0
    }
    #[doc = "Checks if the value of the field is `ATCTE_1`"]
    #[inline]
    pub fn is_atcte_1(&self) -> bool {
        *self == ATCTER::ATCTE_1
    }
}
#[doc = "Possible values of the field `ATTE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATTER {
    #[doc = "No threshold exceeded."]
    ATTE_0,
    #[doc = "Average temperature threshold, as defined by TMHTATR, has been exceeded."]
    ATTE_1,
}
impl ATTER {
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
            ATTER::ATTE_0 => false,
            ATTER::ATTE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ATTER {
        match value {
            false => ATTER::ATTE_0,
            true => ATTER::ATTE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ATTE_0`"]
    #[inline]
    pub fn is_atte_0(&self) -> bool {
        *self == ATTER::ATTE_0
    }
    #[doc = "Checks if the value of the field is `ATTE_1`"]
    #[inline]
    pub fn is_atte_1(&self) -> bool {
        *self == ATTER::ATTE_1
    }
}
#[doc = "Possible values of the field `ITTE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITTER {
    #[doc = "No threshold exceeded."]
    ITTE_0,
    #[doc = "Immediate temperature threshold, as defined by TMHTITR, has been exceeded. This includes an out-of-range measured temperature above 125degree C."]
    ITTE_1,
}
impl ITTER {
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
            ITTER::ITTE_0 => false,
            ITTER::ITTE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ITTER {
        match value {
            false => ITTER::ITTE_0,
            true => ITTER::ITTE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ITTE_0`"]
    #[inline]
    pub fn is_itte_0(&self) -> bool {
        *self == ITTER::ITTE_0
    }
    #[doc = "Checks if the value of the field is `ITTE_1`"]
    #[inline]
    pub fn is_itte_1(&self) -> bool {
        *self == ITTER::ITTE_1
    }
}
#[doc = "Values that can be written to the field `ATCTE`"]
pub enum ATCTEW {
    #[doc = "No threshold exceeded."]
    ATCTE_0,
    #[doc = "Average temperature critical threshold, as defined by TMHTACTR, has been exceeded."]
    ATCTE_1,
}
impl ATCTEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ATCTEW::ATCTE_0 => false,
            ATCTEW::ATCTE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ATCTEW<'a> {
    w: &'a mut W,
}
impl<'a> _ATCTEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ATCTEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No threshold exceeded."]
    #[inline]
    pub fn atcte_0(self) -> &'a mut W {
        self.variant(ATCTEW::ATCTE_0)
    }
    #[doc = "Average temperature critical threshold, as defined by TMHTACTR, has been exceeded."]
    #[inline]
    pub fn atcte_1(self) -> &'a mut W {
        self.variant(ATCTEW::ATCTE_1)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ATTE`"]
pub enum ATTEW {
    #[doc = "No threshold exceeded."]
    ATTE_0,
    #[doc = "Average temperature threshold, as defined by TMHTATR, has been exceeded."]
    ATTE_1,
}
impl ATTEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ATTEW::ATTE_0 => false,
            ATTEW::ATTE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ATTEW<'a> {
    w: &'a mut W,
}
impl<'a> _ATTEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ATTEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No threshold exceeded."]
    #[inline]
    pub fn atte_0(self) -> &'a mut W {
        self.variant(ATTEW::ATTE_0)
    }
    #[doc = "Average temperature threshold, as defined by TMHTATR, has been exceeded."]
    #[inline]
    pub fn atte_1(self) -> &'a mut W {
        self.variant(ATTEW::ATTE_1)
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
#[doc = "Values that can be written to the field `ITTE`"]
pub enum ITTEW {
    #[doc = "No threshold exceeded."]
    ITTE_0,
    #[doc = "Immediate temperature threshold, as defined by TMHTITR, has been exceeded. This includes an out-of-range measured temperature above 125degree C."]
    ITTE_1,
}
impl ITTEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ITTEW::ITTE_0 => false,
            ITTEW::ITTE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ITTEW<'a> {
    w: &'a mut W,
}
impl<'a> _ITTEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ITTEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No threshold exceeded."]
    #[inline]
    pub fn itte_0(self) -> &'a mut W {
        self.variant(ITTEW::ITTE_0)
    }
    #[doc = "Immediate temperature threshold, as defined by TMHTITR, has been exceeded. This includes an out-of-range measured temperature above 125degree C."]
    #[inline]
    pub fn itte_1(self) -> &'a mut W {
        self.variant(ITTEW::ITTE_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 28 - Average temperature critical threshold exceeded. Write 1 to clear."]
    #[inline]
    pub fn atcte(&self) -> ATCTER {
        ATCTER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Average temperature threshold exceeded. Write 1 to clear."]
    #[inline]
    pub fn atte(&self) -> ATTER {
        ATTER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Immediate temperature threshold exceeded. Write 1 to clear."]
    #[inline]
    pub fn itte(&self) -> ITTER {
        ITTER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
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
    #[doc = "Bit 28 - Average temperature critical threshold exceeded. Write 1 to clear."]
    #[inline]
    pub fn atcte(&mut self) -> _ATCTEW {
        _ATCTEW { w: self }
    }
    #[doc = "Bit 29 - Average temperature threshold exceeded. Write 1 to clear."]
    #[inline]
    pub fn atte(&mut self) -> _ATTEW {
        _ATTEW { w: self }
    }
    #[doc = "Bit 30 - Immediate temperature threshold exceeded. Write 1 to clear."]
    #[inline]
    pub fn itte(&mut self) -> _ITTEW {
        _ITTEW { w: self }
    }
}
