#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLASH1LAYOUT1 {
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
pub struct DATAN_SIZER {
    bits: u16,
}
impl DATAN_SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GF13_0_GF14_1R {
    bits: bool,
}
impl GF13_0_GF14_1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `ECCN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECCNR {
    #[doc = "No ECC to be performed"]
    NONE,
    #[doc = "ECC 2 to be performed"]
    ECC2,
    #[doc = "ECC 4 to be performed"]
    ECC4,
    #[doc = "ECC 60 to be performed"]
    ECC60,
    #[doc = "ECC 62 to be performed"]
    ECC62,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ECCNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ECCNR::NONE => 0,
            ECCNR::ECC2 => 1,
            ECCNR::ECC4 => 2,
            ECCNR::ECC60 => 30,
            ECCNR::ECC62 => 31,
            ECCNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ECCNR {
        match value {
            0 => ECCNR::NONE,
            1 => ECCNR::ECC2,
            2 => ECCNR::ECC4,
            30 => ECCNR::ECC60,
            31 => ECCNR::ECC62,
            i => ECCNR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == ECCNR::NONE
    }
    #[doc = "Checks if the value of the field is `ECC2`"]
    #[inline]
    pub fn is_ecc2(&self) -> bool {
        *self == ECCNR::ECC2
    }
    #[doc = "Checks if the value of the field is `ECC4`"]
    #[inline]
    pub fn is_ecc4(&self) -> bool {
        *self == ECCNR::ECC4
    }
    #[doc = "Checks if the value of the field is `ECC60`"]
    #[inline]
    pub fn is_ecc60(&self) -> bool {
        *self == ECCNR::ECC60
    }
    #[doc = "Checks if the value of the field is `ECC62`"]
    #[inline]
    pub fn is_ecc62(&self) -> bool {
        *self == ECCNR::ECC62
    }
}
#[doc = r" Value of the field"]
pub struct PAGE_SIZER {
    bits: u16,
}
impl PAGE_SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DATAN_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _DATAN_SIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GF13_0_GF14_1W<'a> {
    w: &'a mut W,
}
impl<'a> _GF13_0_GF14_1W<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ECCN`"]
pub enum ECCNW {
    #[doc = "No ECC to be performed"]
    NONE,
    #[doc = "ECC 2 to be performed"]
    ECC2,
    #[doc = "ECC 4 to be performed"]
    ECC4,
    #[doc = "ECC 60 to be performed"]
    ECC60,
    #[doc = "ECC 62 to be performed"]
    ECC62,
}
impl ECCNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ECCNW::NONE => 0,
            ECCNW::ECC2 => 1,
            ECCNW::ECC4 => 2,
            ECCNW::ECC60 => 30,
            ECCNW::ECC62 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECCNW<'a> {
    w: &'a mut W,
}
impl<'a> _ECCNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECCNW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No ECC to be performed"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(ECCNW::NONE)
    }
    #[doc = "ECC 2 to be performed"]
    #[inline]
    pub fn ecc2(self) -> &'a mut W {
        self.variant(ECCNW::ECC2)
    }
    #[doc = "ECC 4 to be performed"]
    #[inline]
    pub fn ecc4(self) -> &'a mut W {
        self.variant(ECCNW::ECC4)
    }
    #[doc = "ECC 60 to be performed"]
    #[inline]
    pub fn ecc60(self) -> &'a mut W {
        self.variant(ECCNW::ECC60)
    }
    #[doc = "ECC 62 to be performed"]
    #[inline]
    pub fn ecc62(self) -> &'a mut W {
        self.variant(ECCNW::ECC62)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PAGE_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _PAGE_SIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:9 - Indicates the size of the subsequent data blocks (in DWORDS / four bytes) to be stored on the flash page"]
    #[inline]
    pub fn datan_size(&self) -> DATAN_SIZER {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DATAN_SIZER { bits }
    }
    #[doc = "Bit 10 - Select GF13 or GF14: 0-GF13; 1-GF14"]
    #[inline]
    pub fn gf13_0_gf14_1(&self) -> GF13_0_GF14_1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GF13_0_GF14_1R { bits }
    }
    #[doc = "Bits 11:15 - Indicates the ECC level for the subsequent blocks on the flash page (blocks 1-n)"]
    #[inline]
    pub fn eccn(&self) -> ECCNR {
        ECCNR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:31 - Indicates the total size of the flash page (in bytes)"]
    #[inline]
    pub fn page_size(&self) -> PAGE_SIZER {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PAGE_SIZER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 282738816 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:9 - Indicates the size of the subsequent data blocks (in DWORDS / four bytes) to be stored on the flash page"]
    #[inline]
    pub fn datan_size(&mut self) -> _DATAN_SIZEW {
        _DATAN_SIZEW { w: self }
    }
    #[doc = "Bit 10 - Select GF13 or GF14: 0-GF13; 1-GF14"]
    #[inline]
    pub fn gf13_0_gf14_1(&mut self) -> _GF13_0_GF14_1W {
        _GF13_0_GF14_1W { w: self }
    }
    #[doc = "Bits 11:15 - Indicates the ECC level for the subsequent blocks on the flash page (blocks 1-n)"]
    #[inline]
    pub fn eccn(&mut self) -> _ECCNW {
        _ECCNW { w: self }
    }
    #[doc = "Bits 16:31 - Indicates the total size of the flash page (in bytes)"]
    #[inline]
    pub fn page_size(&mut self) -> _PAGE_SIZEW {
        _PAGE_SIZEW { w: self }
    }
}
