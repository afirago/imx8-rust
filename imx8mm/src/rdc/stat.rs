#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STAT {
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
pub struct DIDR {
    bits: u8,
}
impl DIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PDS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDSR {
    #[doc = "Power Down Domain is OFF"]
    PDS_0,
    #[doc = "Power Down Domain is ON"]
    PDS_1,
}
impl PDSR {
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
            PDSR::PDS_0 => false,
            PDSR::PDS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDSR {
        match value {
            false => PDSR::PDS_0,
            true => PDSR::PDS_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDS_0`"]
    #[inline]
    pub fn is_pds_0(&self) -> bool {
        *self == PDSR::PDS_0
    }
    #[doc = "Checks if the value of the field is `PDS_1`"]
    #[inline]
    pub fn is_pds_1(&self) -> bool {
        *self == PDSR::PDS_1
    }
}
#[doc = r" Proxy"]
pub struct _DIDW<'a> {
    w: &'a mut W,
}
impl<'a> _DIDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDS`"]
pub enum PDSW {
    #[doc = "Power Down Domain is OFF"]
    PDS_0,
    #[doc = "Power Down Domain is ON"]
    PDS_1,
}
impl PDSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDSW::PDS_0 => false,
            PDSW::PDS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDSW<'a> {
    w: &'a mut W,
}
impl<'a> _PDSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Power Down Domain is OFF"]
    #[inline]
    pub fn pds_0(self) -> &'a mut W {
        self.variant(PDSW::PDS_0)
    }
    #[doc = "Power Down Domain is ON"]
    #[inline]
    pub fn pds_1(self) -> &'a mut W {
        self.variant(PDSW::PDS_1)
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
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:3 - Domain ID"]
    #[inline]
    pub fn did(&self) -> DIDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DIDR { bits }
    }
    #[doc = "Bit 8 - Power Domain Status"]
    #[inline]
    pub fn pds(&self) -> PDSR {
        PDSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 256 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Domain ID"]
    #[inline]
    pub fn did(&mut self) -> _DIDW {
        _DIDW { w: self }
    }
    #[doc = "Bit 8 - Power Domain Status"]
    #[inline]
    pub fn pds(&mut self) -> _PDSW {
        _PDSW { w: self }
    }
}
