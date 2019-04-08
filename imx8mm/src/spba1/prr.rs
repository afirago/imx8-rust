#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRR {
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
#[doc = "Possible values of the field `RARA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RARAR {
    #[doc = "Access to peripheral is not allowed."]
    PROHIBITED,
    #[doc = "Access to peripheral is granted."]
    ALLOWED,
}
impl RARAR {
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
            RARAR::PROHIBITED => false,
            RARAR::ALLOWED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RARAR {
        match value {
            false => RARAR::PROHIBITED,
            true => RARAR::ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `PROHIBITED`"]
    #[inline]
    pub fn is_prohibited(&self) -> bool {
        *self == RARAR::PROHIBITED
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline]
    pub fn is_allowed(&self) -> bool {
        *self == RARAR::ALLOWED
    }
}
#[doc = "Possible values of the field `RARB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RARBR {
    #[doc = "Access to peripheral is not allowed."]
    PROHIBITED,
    #[doc = "Access to peripheral is granted."]
    ALLOWED,
}
impl RARBR {
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
            RARBR::PROHIBITED => false,
            RARBR::ALLOWED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RARBR {
        match value {
            false => RARBR::PROHIBITED,
            true => RARBR::ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `PROHIBITED`"]
    #[inline]
    pub fn is_prohibited(&self) -> bool {
        *self == RARBR::PROHIBITED
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline]
    pub fn is_allowed(&self) -> bool {
        *self == RARBR::ALLOWED
    }
}
#[doc = "Possible values of the field `RARC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RARCR {
    #[doc = "Access to peripheral is not allowed."]
    PROHIBITED,
    #[doc = "Access to peripheral is granted."]
    ALLOWED,
}
impl RARCR {
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
            RARCR::PROHIBITED => false,
            RARCR::ALLOWED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RARCR {
        match value {
            false => RARCR::PROHIBITED,
            true => RARCR::ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `PROHIBITED`"]
    #[inline]
    pub fn is_prohibited(&self) -> bool {
        *self == RARCR::PROHIBITED
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline]
    pub fn is_allowed(&self) -> bool {
        *self == RARCR::ALLOWED
    }
}
#[doc = "Possible values of the field `ROI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROIR {
    #[doc = "Unowned resource."]
    UNOWNED,
    #[doc = "The resource is owned by master A port."]
    MASTER_A,
    #[doc = "The resource is owned by master B port."]
    MASTER_B,
    #[doc = "The resource is owned by master C port."]
    MASTER_C,
}
impl ROIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ROIR::UNOWNED => 0,
            ROIR::MASTER_A => 1,
            ROIR::MASTER_B => 2,
            ROIR::MASTER_C => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ROIR {
        match value {
            0 => ROIR::UNOWNED,
            1 => ROIR::MASTER_A,
            2 => ROIR::MASTER_B,
            3 => ROIR::MASTER_C,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UNOWNED`"]
    #[inline]
    pub fn is_unowned(&self) -> bool {
        *self == ROIR::UNOWNED
    }
    #[doc = "Checks if the value of the field is `MASTER_A`"]
    #[inline]
    pub fn is_master_a(&self) -> bool {
        *self == ROIR::MASTER_A
    }
    #[doc = "Checks if the value of the field is `MASTER_B`"]
    #[inline]
    pub fn is_master_b(&self) -> bool {
        *self == ROIR::MASTER_B
    }
    #[doc = "Checks if the value of the field is `MASTER_C`"]
    #[inline]
    pub fn is_master_c(&self) -> bool {
        *self == ROIR::MASTER_C
    }
}
#[doc = "Possible values of the field `RMO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMOR {
    #[doc = "The resource is unowned."]
    UNOWNED,
    #[doc = "The resource is owned by another master."]
    ANOTHER_MASTER,
    #[doc = "The resource is owned by the requesting master."]
    REQUESTING_MASTER,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RMOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RMOR::UNOWNED => 0,
            RMOR::ANOTHER_MASTER => 2,
            RMOR::REQUESTING_MASTER => 3,
            RMOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RMOR {
        match value {
            0 => RMOR::UNOWNED,
            2 => RMOR::ANOTHER_MASTER,
            3 => RMOR::REQUESTING_MASTER,
            i => RMOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UNOWNED`"]
    #[inline]
    pub fn is_unowned(&self) -> bool {
        *self == RMOR::UNOWNED
    }
    #[doc = "Checks if the value of the field is `ANOTHER_MASTER`"]
    #[inline]
    pub fn is_another_master(&self) -> bool {
        *self == RMOR::ANOTHER_MASTER
    }
    #[doc = "Checks if the value of the field is `REQUESTING_MASTER`"]
    #[inline]
    pub fn is_requesting_master(&self) -> bool {
        *self == RMOR::REQUESTING_MASTER
    }
}
#[doc = "Values that can be written to the field `RARA`"]
pub enum RARAW {
    #[doc = "Access to peripheral is not allowed."]
    PROHIBITED,
    #[doc = "Access to peripheral is granted."]
    ALLOWED,
}
impl RARAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RARAW::PROHIBITED => false,
            RARAW::ALLOWED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RARAW<'a> {
    w: &'a mut W,
}
impl<'a> _RARAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RARAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Access to peripheral is not allowed."]
    #[inline]
    pub fn prohibited(self) -> &'a mut W {
        self.variant(RARAW::PROHIBITED)
    }
    #[doc = "Access to peripheral is granted."]
    #[inline]
    pub fn allowed(self) -> &'a mut W {
        self.variant(RARAW::ALLOWED)
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
#[doc = "Values that can be written to the field `RARB`"]
pub enum RARBW {
    #[doc = "Access to peripheral is not allowed."]
    PROHIBITED,
    #[doc = "Access to peripheral is granted."]
    ALLOWED,
}
impl RARBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RARBW::PROHIBITED => false,
            RARBW::ALLOWED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RARBW<'a> {
    w: &'a mut W,
}
impl<'a> _RARBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RARBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Access to peripheral is not allowed."]
    #[inline]
    pub fn prohibited(self) -> &'a mut W {
        self.variant(RARBW::PROHIBITED)
    }
    #[doc = "Access to peripheral is granted."]
    #[inline]
    pub fn allowed(self) -> &'a mut W {
        self.variant(RARBW::ALLOWED)
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
#[doc = "Values that can be written to the field `RARC`"]
pub enum RARCW {
    #[doc = "Access to peripheral is not allowed."]
    PROHIBITED,
    #[doc = "Access to peripheral is granted."]
    ALLOWED,
}
impl RARCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RARCW::PROHIBITED => false,
            RARCW::ALLOWED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RARCW<'a> {
    w: &'a mut W,
}
impl<'a> _RARCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RARCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Access to peripheral is not allowed."]
    #[inline]
    pub fn prohibited(self) -> &'a mut W {
        self.variant(RARCW::PROHIBITED)
    }
    #[doc = "Access to peripheral is granted."]
    #[inline]
    pub fn allowed(self) -> &'a mut W {
        self.variant(RARCW::ALLOWED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Resource Access Right"]
    #[inline]
    pub fn rara(&self) -> RARAR {
        RARAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Resource Access Right"]
    #[inline]
    pub fn rarb(&self) -> RARBR {
        RARBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Resource Access Right"]
    #[inline]
    pub fn rarc(&self) -> RARCR {
        RARCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:17 - Resource Owner ID"]
    #[inline]
    pub fn roi(&self) -> ROIR {
        ROIR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - Requesting Master Owner"]
    #[inline]
    pub fn rmo(&self) -> RMOR {
        RMOR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 7 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Resource Access Right"]
    #[inline]
    pub fn rara(&mut self) -> _RARAW {
        _RARAW { w: self }
    }
    #[doc = "Bit 1 - Resource Access Right"]
    #[inline]
    pub fn rarb(&mut self) -> _RARBW {
        _RARBW { w: self }
    }
    #[doc = "Bit 2 - Resource Access Right"]
    #[inline]
    pub fn rarc(&mut self) -> _RARCW {
        _RARCW { w: self }
    }
}
