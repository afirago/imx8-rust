#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEBUG1 {
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
pub struct ERASED_ZERO_COUNTR {
    bits: u16,
}
impl ERASED_ZERO_COUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RSVDR {
    bits: u32,
}
impl RSVDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `DEBUG1_PREERASECHK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBUG1_PREERASECHKR {
    #[doc = "Turn off pre-erase check"]
    DEBUG1_PREERASECHK_0,
    #[doc = "Turn on pre-erase check"]
    DEBUG1_PREERASECHK_1,
}
impl DEBUG1_PREERASECHKR {
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
            DEBUG1_PREERASECHKR::DEBUG1_PREERASECHK_0 => false,
            DEBUG1_PREERASECHKR::DEBUG1_PREERASECHK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEBUG1_PREERASECHKR {
        match value {
            false => DEBUG1_PREERASECHKR::DEBUG1_PREERASECHK_0,
            true => DEBUG1_PREERASECHKR::DEBUG1_PREERASECHK_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEBUG1_PREERASECHK_0`"]
    #[inline]
    pub fn is_debug1_preerasechk_0(&self) -> bool {
        *self == DEBUG1_PREERASECHKR::DEBUG1_PREERASECHK_0
    }
    #[doc = "Checks if the value of the field is `DEBUG1_PREERASECHK_1`"]
    #[inline]
    pub fn is_debug1_preerasechk_1(&self) -> bool {
        *self == DEBUG1_PREERASECHKR::DEBUG1_PREERASECHK_1
    }
}
#[doc = "Values that can be written to the field `DEBUG1_PREERASECHK`"]
pub enum DEBUG1_PREERASECHKW {
    #[doc = "Turn off pre-erase check"]
    DEBUG1_PREERASECHK_0,
    #[doc = "Turn on pre-erase check"]
    DEBUG1_PREERASECHK_1,
}
impl DEBUG1_PREERASECHKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DEBUG1_PREERASECHKW::DEBUG1_PREERASECHK_0 => false,
            DEBUG1_PREERASECHKW::DEBUG1_PREERASECHK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEBUG1_PREERASECHKW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBUG1_PREERASECHKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEBUG1_PREERASECHKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Turn off pre-erase check"]
    #[inline]
    pub fn debug1_preerasechk_0(self) -> &'a mut W {
        self.variant(DEBUG1_PREERASECHKW::DEBUG1_PREERASECHK_0)
    }
    #[doc = "Turn on pre-erase check"]
    #[inline]
    pub fn debug1_preerasechk_1(self) -> &'a mut W {
        self.variant(DEBUG1_PREERASECHKW::DEBUG1_PREERASECHK_1)
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
    #[doc = "Bits 0:8 - The zero counts on one page."]
    #[inline]
    pub fn erased_zero_count(&self) -> ERASED_ZERO_COUNTR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        ERASED_ZERO_COUNTR { bits }
    }
    #[doc = "Bits 9:30 - This field is reserved."]
    #[inline]
    pub fn rsvd(&self) -> RSVDR {
        let bits = {
            const MASK: u32 = 4194303;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RSVDR { bits }
    }
    #[doc = "Bit 31 - Blank page enables pre-erase check."]
    #[inline]
    pub fn debug1_preerasechk(&self) -> DEBUG1_PREERASECHKR {
        DEBUG1_PREERASECHKR::_from({
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
    #[doc = "Bit 31 - Blank page enables pre-erase check."]
    #[inline]
    pub fn debug1_preerasechk(&mut self) -> _DEBUG1_PREERASECHKW {
        _DEBUG1_PREERASECHKW { w: self }
    }
}
