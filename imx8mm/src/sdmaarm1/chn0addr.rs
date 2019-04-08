#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHN0ADDR {
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
pub struct CHN0ADDRR {
    bits: u16,
}
impl CHN0ADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `SMSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMSZR {
    #[doc = "24 words per context"]
    SMSZ_0,
    #[doc = "32 words per context"]
    SMSZ_1,
}
impl SMSZR {
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
            SMSZR::SMSZ_0 => false,
            SMSZR::SMSZ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMSZR {
        match value {
            false => SMSZR::SMSZ_0,
            true => SMSZR::SMSZ_1,
        }
    }
    #[doc = "Checks if the value of the field is `SMSZ_0`"]
    #[inline]
    pub fn is_smsz_0(&self) -> bool {
        *self == SMSZR::SMSZ_0
    }
    #[doc = "Checks if the value of the field is `SMSZ_1`"]
    #[inline]
    pub fn is_smsz_1(&self) -> bool {
        *self == SMSZR::SMSZ_1
    }
}
#[doc = r" Proxy"]
pub struct _CHN0ADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _CHN0ADDRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 16383;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMSZ`"]
pub enum SMSZW {
    #[doc = "24 words per context"]
    SMSZ_0,
    #[doc = "32 words per context"]
    SMSZ_1,
}
impl SMSZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMSZW::SMSZ_0 => false,
            SMSZW::SMSZ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMSZW<'a> {
    w: &'a mut W,
}
impl<'a> _SMSZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMSZW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "24 words per context"]
    #[inline]
    pub fn smsz_0(self) -> &'a mut W {
        self.variant(SMSZW::SMSZ_0)
    }
    #[doc = "32 words per context"]
    #[inline]
    pub fn smsz_1(self) -> &'a mut W {
        self.variant(SMSZW::SMSZ_1)
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
        const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:13 - This 14-bit register is used by the boot code of the SDMA"]
    #[inline]
    pub fn chn0addr(&self) -> CHN0ADDRR {
        let bits = {
            const MASK: u16 = 16383;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CHN0ADDRR { bits }
    }
    #[doc = "Bit 14 - The bit 14 (Scratch Memory Size) determines if scratch memory must be available after every channel context"]
    #[inline]
    pub fn smsz(&self) -> SMSZR {
        SMSZR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 80 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:13 - This 14-bit register is used by the boot code of the SDMA"]
    #[inline]
    pub fn chn0addr(&mut self) -> _CHN0ADDRW {
        _CHN0ADDRW { w: self }
    }
    #[doc = "Bit 14 - The bit 14 (Scratch Memory Size) determines if scratch memory must be available after every channel context"]
    #[inline]
    pub fn smsz(&mut self) -> _SMSZW {
        _SMSZW { w: self }
    }
}
