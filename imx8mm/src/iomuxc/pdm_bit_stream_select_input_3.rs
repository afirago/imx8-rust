#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PDM_BIT_STREAM_SELECT_INPUT_3 {
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
#[doc = "Possible values of the field `DAISY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAISYR {
    #[doc = "Selecting ALT4 mode of pad SAI5_RXD3 for PDM_BIT_STREAM_3."]
    SAI5_RXD3_ALT4,
    #[doc = "Selecting ALT3 mode of pad SAI1_RXD3 for PDM_BIT_STREAM_3."]
    SAI1_RXD3_ALT3,
}
impl DAISYR {
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
            DAISYR::SAI5_RXD3_ALT4 => false,
            DAISYR::SAI1_RXD3_ALT3 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DAISYR {
        match value {
            false => DAISYR::SAI5_RXD3_ALT4,
            true => DAISYR::SAI1_RXD3_ALT3,
        }
    }
    #[doc = "Checks if the value of the field is `SAI5_RXD3_ALT4`"]
    #[inline]
    pub fn is_sai5_rxd3_alt4(&self) -> bool {
        *self == DAISYR::SAI5_RXD3_ALT4
    }
    #[doc = "Checks if the value of the field is `SAI1_RXD3_ALT3`"]
    #[inline]
    pub fn is_sai1_rxd3_alt3(&self) -> bool {
        *self == DAISYR::SAI1_RXD3_ALT3
    }
}
#[doc = "Values that can be written to the field `DAISY`"]
pub enum DAISYW {
    #[doc = "Selecting ALT4 mode of pad SAI5_RXD3 for PDM_BIT_STREAM_3."]
    SAI5_RXD3_ALT4,
    #[doc = "Selecting ALT3 mode of pad SAI1_RXD3 for PDM_BIT_STREAM_3."]
    SAI1_RXD3_ALT3,
}
impl DAISYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DAISYW::SAI5_RXD3_ALT4 => false,
            DAISYW::SAI1_RXD3_ALT3 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DAISYW<'a> {
    w: &'a mut W,
}
impl<'a> _DAISYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DAISYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selecting ALT4 mode of pad SAI5_RXD3 for PDM_BIT_STREAM_3."]
    #[inline]
    pub fn sai5_rxd3_alt4(self) -> &'a mut W {
        self.variant(DAISYW::SAI5_RXD3_ALT4)
    }
    #[doc = "Selecting ALT3 mode of pad SAI1_RXD3 for PDM_BIT_STREAM_3."]
    #[inline]
    pub fn sai1_rxd3_alt3(self) -> &'a mut W {
        self.variant(DAISYW::SAI1_RXD3_ALT3)
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
    #[doc = "Bit 0 - Input Select (DAISY) Field"]
    #[inline]
    pub fn daisy(&self) -> DAISYR {
        DAISYR::_from({
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
    #[doc = "Bit 0 - Input Select (DAISY) Field"]
    #[inline]
    pub fn daisy(&mut self) -> _DAISYW {
        _DAISYW { w: self }
    }
}
