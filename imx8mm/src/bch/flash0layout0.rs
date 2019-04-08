#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLASH0LAYOUT0 {
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
pub struct DATA0_SIZER {
    bits: u16,
}
impl DATA0_SIZER {
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
#[doc = "Possible values of the field `ECC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECC0R {
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
impl ECC0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ECC0R::NONE => 0,
            ECC0R::ECC2 => 1,
            ECC0R::ECC4 => 2,
            ECC0R::ECC60 => 30,
            ECC0R::ECC62 => 31,
            ECC0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ECC0R {
        match value {
            0 => ECC0R::NONE,
            1 => ECC0R::ECC2,
            2 => ECC0R::ECC4,
            30 => ECC0R::ECC60,
            31 => ECC0R::ECC62,
            i => ECC0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == ECC0R::NONE
    }
    #[doc = "Checks if the value of the field is `ECC2`"]
    #[inline]
    pub fn is_ecc2(&self) -> bool {
        *self == ECC0R::ECC2
    }
    #[doc = "Checks if the value of the field is `ECC4`"]
    #[inline]
    pub fn is_ecc4(&self) -> bool {
        *self == ECC0R::ECC4
    }
    #[doc = "Checks if the value of the field is `ECC60`"]
    #[inline]
    pub fn is_ecc60(&self) -> bool {
        *self == ECC0R::ECC60
    }
    #[doc = "Checks if the value of the field is `ECC62`"]
    #[inline]
    pub fn is_ecc62(&self) -> bool {
        *self == ECC0R::ECC62
    }
}
#[doc = r" Value of the field"]
pub struct META_SIZER {
    bits: u8,
}
impl META_SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NBLOCKSR {
    bits: u8,
}
impl NBLOCKSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DATA0_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA0_SIZEW<'a> {
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
#[doc = "Values that can be written to the field `ECC0`"]
pub enum ECC0W {
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
impl ECC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ECC0W::NONE => 0,
            ECC0W::ECC2 => 1,
            ECC0W::ECC4 => 2,
            ECC0W::ECC60 => 30,
            ECC0W::ECC62 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECC0W<'a> {
    w: &'a mut W,
}
impl<'a> _ECC0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECC0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No ECC to be performed"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(ECC0W::NONE)
    }
    #[doc = "ECC 2 to be performed"]
    #[inline]
    pub fn ecc2(self) -> &'a mut W {
        self.variant(ECC0W::ECC2)
    }
    #[doc = "ECC 4 to be performed"]
    #[inline]
    pub fn ecc4(self) -> &'a mut W {
        self.variant(ECC0W::ECC4)
    }
    #[doc = "ECC 60 to be performed"]
    #[inline]
    pub fn ecc60(self) -> &'a mut W {
        self.variant(ECC0W::ECC60)
    }
    #[doc = "ECC 62 to be performed"]
    #[inline]
    pub fn ecc62(self) -> &'a mut W {
        self.variant(ECC0W::ECC62)
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
pub struct _META_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _META_SIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NBLOCKSW<'a> {
    w: &'a mut W,
}
impl<'a> _NBLOCKSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:9 - Indicates the size of the data 0 block (in DWORDS / four bytes) to be stored on the flash page"]
    #[inline]
    pub fn data0_size(&self) -> DATA0_SIZER {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DATA0_SIZER { bits }
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
    #[doc = "Bits 11:15 - Indicates the ECC level for the first block on the flash page"]
    #[inline]
    pub fn ecc0(&self) -> ECC0R {
        ECC0R::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - Indicates the size of the metadata (in bytes) to be stored on a flash page"]
    #[inline]
    pub fn meta_size(&self) -> META_SIZER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        META_SIZER { bits }
    }
    #[doc = "Bits 24:31 - Number of subsequent blocks on the flash page (excluding the data0 block)"]
    #[inline]
    pub fn nblocks(&self) -> NBLOCKSR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NBLOCKSR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 118112384 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:9 - Indicates the size of the data 0 block (in DWORDS / four bytes) to be stored on the flash page"]
    #[inline]
    pub fn data0_size(&mut self) -> _DATA0_SIZEW {
        _DATA0_SIZEW { w: self }
    }
    #[doc = "Bit 10 - Select GF13 or GF14: 0-GF13; 1-GF14"]
    #[inline]
    pub fn gf13_0_gf14_1(&mut self) -> _GF13_0_GF14_1W {
        _GF13_0_GF14_1W { w: self }
    }
    #[doc = "Bits 11:15 - Indicates the ECC level for the first block on the flash page"]
    #[inline]
    pub fn ecc0(&mut self) -> _ECC0W {
        _ECC0W { w: self }
    }
    #[doc = "Bits 16:23 - Indicates the size of the metadata (in bytes) to be stored on a flash page"]
    #[inline]
    pub fn meta_size(&mut self) -> _META_SIZEW {
        _META_SIZEW { w: self }
    }
    #[doc = "Bits 24:31 - Number of subsequent blocks on the flash page (excluding the data0 block)"]
    #[inline]
    pub fn nblocks(&mut self) -> _NBLOCKSW {
        _NBLOCKSW { w: self }
    }
}
