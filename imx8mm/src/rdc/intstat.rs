#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTSTAT {
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
#[doc = "Possible values of the field `INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTR {
    #[doc = "No Interrupt Pending"]
    INT_0,
    #[doc = "Interrupt Pending"]
    INT_1,
}
impl INTR {
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
            INTR::INT_0 => false,
            INTR::INT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTR {
        match value {
            false => INTR::INT_0,
            true => INTR::INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_0`"]
    #[inline]
    pub fn is_int_0(&self) -> bool {
        *self == INTR::INT_0
    }
    #[doc = "Checks if the value of the field is `INT_1`"]
    #[inline]
    pub fn is_int_1(&self) -> bool {
        *self == INTR::INT_1
    }
}
#[doc = "Values that can be written to the field `INT`"]
pub enum INTW {
    #[doc = "No Interrupt Pending"]
    INT_0,
    #[doc = "Interrupt Pending"]
    INT_1,
}
impl INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INTW::INT_0 => false,
            INTW::INT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTW<'a> {
    w: &'a mut W,
}
impl<'a> _INTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Interrupt Pending"]
    #[inline]
    pub fn int_0(self) -> &'a mut W {
        self.variant(INTW::INT_0)
    }
    #[doc = "Interrupt Pending"]
    #[inline]
    pub fn int_1(self) -> &'a mut W {
        self.variant(INTW::INT_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Interrupt Status"]
    #[inline]
    pub fn int(&self) -> INTR {
        INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
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
    #[doc = "Bit 0 - Interrupt Status"]
    #[inline]
    pub fn int(&mut self) -> _INTW {
        _INTW { w: self }
    }
}
