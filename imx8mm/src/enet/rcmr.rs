#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RCMR {
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
pub struct CMP0R {
    bits: u8,
}
impl CMP0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CMP1R {
    bits: u8,
}
impl CMP1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CMP2R {
    bits: u8,
}
impl CMP2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CMP3R {
    bits: u8,
}
impl CMP3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `MATCHEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MATCHENR {
    #[doc = "Disabled (default): no compares will occur and the classification indicator for this class will never assert."]
    MATCHEN_0,
    #[doc = "The register contents are valid and a comparison with all compare values is done when a VLAN frame is received."]
    MATCHEN_1,
}
impl MATCHENR {
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
            MATCHENR::MATCHEN_0 => false,
            MATCHENR::MATCHEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MATCHENR {
        match value {
            false => MATCHENR::MATCHEN_0,
            true => MATCHENR::MATCHEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MATCHEN_0`"]
    #[inline]
    pub fn is_matchen_0(&self) -> bool {
        *self == MATCHENR::MATCHEN_0
    }
    #[doc = "Checks if the value of the field is `MATCHEN_1`"]
    #[inline]
    pub fn is_matchen_1(&self) -> bool {
        *self == MATCHENR::MATCHEN_1
    }
}
#[doc = r" Proxy"]
pub struct _CMP0W<'a> {
    w: &'a mut W,
}
impl<'a> _CMP0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CMP1W<'a> {
    w: &'a mut W,
}
impl<'a> _CMP1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CMP2W<'a> {
    w: &'a mut W,
}
impl<'a> _CMP2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CMP3W<'a> {
    w: &'a mut W,
}
impl<'a> _CMP3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MATCHEN`"]
pub enum MATCHENW {
    #[doc = "Disabled (default): no compares will occur and the classification indicator for this class will never assert."]
    MATCHEN_0,
    #[doc = "The register contents are valid and a comparison with all compare values is done when a VLAN frame is received."]
    MATCHEN_1,
}
impl MATCHENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MATCHENW::MATCHEN_0 => false,
            MATCHENW::MATCHEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MATCHENW<'a> {
    w: &'a mut W,
}
impl<'a> _MATCHENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MATCHENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled (default): no compares will occur and the classification indicator for this class will never assert."]
    #[inline]
    pub fn matchen_0(self) -> &'a mut W {
        self.variant(MATCHENW::MATCHEN_0)
    }
    #[doc = "The register contents are valid and a comparison with all compare values is done when a VLAN frame is received."]
    #[inline]
    pub fn matchen_1(self) -> &'a mut W {
        self.variant(MATCHENW::MATCHEN_1)
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
    #[doc = "Bits 0:2 - Compare 0"]
    #[inline]
    pub fn cmp0(&self) -> CMP0R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMP0R { bits }
    }
    #[doc = "Bits 4:6 - Compare 1"]
    #[inline]
    pub fn cmp1(&self) -> CMP1R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMP1R { bits }
    }
    #[doc = "Bits 8:10 - Compare 2"]
    #[inline]
    pub fn cmp2(&self) -> CMP2R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMP2R { bits }
    }
    #[doc = "Bits 12:14 - Compare 3"]
    #[inline]
    pub fn cmp3(&self) -> CMP3R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMP3R { bits }
    }
    #[doc = "Bit 16 - Match Enable"]
    #[inline]
    pub fn matchen(&self) -> MATCHENR {
        MATCHENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:2 - Compare 0"]
    #[inline]
    pub fn cmp0(&mut self) -> _CMP0W {
        _CMP0W { w: self }
    }
    #[doc = "Bits 4:6 - Compare 1"]
    #[inline]
    pub fn cmp1(&mut self) -> _CMP1W {
        _CMP1W { w: self }
    }
    #[doc = "Bits 8:10 - Compare 2"]
    #[inline]
    pub fn cmp2(&mut self) -> _CMP2W {
        _CMP2W { w: self }
    }
    #[doc = "Bits 12:14 - Compare 3"]
    #[inline]
    pub fn cmp3(&mut self) -> _CMP3W {
        _CMP3W { w: self }
    }
    #[doc = "Bit 16 - Match Enable"]
    #[inline]
    pub fn matchen(&mut self) -> _MATCHENW {
        _MATCHENW { w: self }
    }
}
