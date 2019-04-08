#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TESTREG {
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
pub struct TXCNTR {
    bits: u8,
}
impl TXCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RXCNTR {
    bits: u8,
}
impl RXCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `LBC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBCR {
    #[doc = "Not connected."]
    LBC_0,
    #[doc = "Transmitter and receiver sections internally connected for Loopback."]
    LBC_1,
}
impl LBCR {
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
            LBCR::LBC_0 => false,
            LBCR::LBC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LBCR {
        match value {
            false => LBCR::LBC_0,
            true => LBCR::LBC_1,
        }
    }
    #[doc = "Checks if the value of the field is `LBC_0`"]
    #[inline]
    pub fn is_lbc_0(&self) -> bool {
        *self == LBCR::LBC_0
    }
    #[doc = "Checks if the value of the field is `LBC_1`"]
    #[inline]
    pub fn is_lbc_1(&self) -> bool {
        *self == LBCR::LBC_1
    }
}
#[doc = r" Proxy"]
pub struct _TXCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LBC`"]
pub enum LBCW {
    #[doc = "Not connected."]
    LBC_0,
    #[doc = "Transmitter and receiver sections internally connected for Loopback."]
    LBC_1,
}
impl LBCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LBCW::LBC_0 => false,
            LBCW::LBC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LBCW<'a> {
    w: &'a mut W,
}
impl<'a> _LBCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LBCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not connected."]
    #[inline]
    pub fn lbc_0(self) -> &'a mut W {
        self.variant(LBCW::LBC_0)
    }
    #[doc = "Transmitter and receiver sections internally connected for Loopback."]
    #[inline]
    pub fn lbc_1(self) -> &'a mut W {
        self.variant(LBCW::LBC_1)
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
    #[doc = "Bits 0:6 - TXFIFO Counter. This field indicates the number of words in the TXFIFO."]
    #[inline]
    pub fn txcnt(&self) -> TXCNTR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXCNTR { bits }
    }
    #[doc = "Bits 8:14 - RXFIFO Counter. This field indicates the number of words in the RXFIFO."]
    #[inline]
    pub fn rxcnt(&self) -> RXCNTR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RXCNTR { bits }
    }
    #[doc = "Bit 31 - Loop Back Control"]
    #[inline]
    pub fn lbc(&self) -> LBCR {
        LBCR::_from({
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
    #[doc = "Bits 0:6 - TXFIFO Counter. This field indicates the number of words in the TXFIFO."]
    #[inline]
    pub fn txcnt(&mut self) -> _TXCNTW {
        _TXCNTW { w: self }
    }
    #[doc = "Bits 8:14 - RXFIFO Counter. This field indicates the number of words in the RXFIFO."]
    #[inline]
    pub fn rxcnt(&mut self) -> _RXCNTW {
        _RXCNTW { w: self }
    }
    #[doc = "Bit 31 - Loop Back Control"]
    #[inline]
    pub fn lbc(&mut self) -> _LBCW {
        _LBCW { w: self }
    }
}
