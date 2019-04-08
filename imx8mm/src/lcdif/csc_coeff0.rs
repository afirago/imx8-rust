#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSC_COEFF0 {
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
#[doc = "Possible values of the field `CSC_SUBSAMPLE_FILTER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSC_SUBSAMPLE_FILTERR {
    #[doc = "No filtering, simply keep every chroma value for samples numbered 2n and discard chroma values associated with all samples numbered 2n+1."]
    SAMPLE_AND_HOLD,
    #[doc = "Chroma samples numbered 2n and 2n+1 are averaged (weights 1/2, 1/2) and that chroma value replaces the two chroma values at 2n and 2n+1. This chroma now exists horizontally halfway between the two luma samples."]
    INTERSTITIAL,
    #[doc = "Chroma samples numbered 2n-1, 2n, and 2n+1 are averaged (weights 1/4, 1/2, 1/4) and that chroma value exists at the same site as the luma sample numbered 2n and the chroma samples at 2n+1 are discarded."]
    COSITED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CSC_SUBSAMPLE_FILTERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSC_SUBSAMPLE_FILTERR::SAMPLE_AND_HOLD => 0,
            CSC_SUBSAMPLE_FILTERR::INTERSTITIAL => 2,
            CSC_SUBSAMPLE_FILTERR::COSITED => 3,
            CSC_SUBSAMPLE_FILTERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSC_SUBSAMPLE_FILTERR {
        match value {
            0 => CSC_SUBSAMPLE_FILTERR::SAMPLE_AND_HOLD,
            2 => CSC_SUBSAMPLE_FILTERR::INTERSTITIAL,
            3 => CSC_SUBSAMPLE_FILTERR::COSITED,
            i => CSC_SUBSAMPLE_FILTERR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SAMPLE_AND_HOLD`"]
    #[inline]
    pub fn is_sample_and_hold(&self) -> bool {
        *self == CSC_SUBSAMPLE_FILTERR::SAMPLE_AND_HOLD
    }
    #[doc = "Checks if the value of the field is `INTERSTITIAL`"]
    #[inline]
    pub fn is_interstitial(&self) -> bool {
        *self == CSC_SUBSAMPLE_FILTERR::INTERSTITIAL
    }
    #[doc = "Checks if the value of the field is `COSITED`"]
    #[inline]
    pub fn is_cosited(&self) -> bool {
        *self == CSC_SUBSAMPLE_FILTERR::COSITED
    }
}
#[doc = r" Value of the field"]
pub struct C0R {
    bits: u16,
}
impl C0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `CSC_SUBSAMPLE_FILTER`"]
pub enum CSC_SUBSAMPLE_FILTERW {
    #[doc = "No filtering, simply keep every chroma value for samples numbered 2n and discard chroma values associated with all samples numbered 2n+1."]
    SAMPLE_AND_HOLD,
    #[doc = "Chroma samples numbered 2n and 2n+1 are averaged (weights 1/2, 1/2) and that chroma value replaces the two chroma values at 2n and 2n+1. This chroma now exists horizontally halfway between the two luma samples."]
    INTERSTITIAL,
    #[doc = "Chroma samples numbered 2n-1, 2n, and 2n+1 are averaged (weights 1/4, 1/2, 1/4) and that chroma value exists at the same site as the luma sample numbered 2n and the chroma samples at 2n+1 are discarded."]
    COSITED,
}
impl CSC_SUBSAMPLE_FILTERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSC_SUBSAMPLE_FILTERW::SAMPLE_AND_HOLD => 0,
            CSC_SUBSAMPLE_FILTERW::INTERSTITIAL => 2,
            CSC_SUBSAMPLE_FILTERW::COSITED => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSC_SUBSAMPLE_FILTERW<'a> {
    w: &'a mut W,
}
impl<'a> _CSC_SUBSAMPLE_FILTERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSC_SUBSAMPLE_FILTERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No filtering, simply keep every chroma value for samples numbered 2n and discard chroma values associated with all samples numbered 2n+1."]
    #[inline]
    pub fn sample_and_hold(self) -> &'a mut W {
        self.variant(CSC_SUBSAMPLE_FILTERW::SAMPLE_AND_HOLD)
    }
    #[doc = "Chroma samples numbered 2n and 2n+1 are averaged (weights 1/2, 1/2) and that chroma value replaces the two chroma values at 2n and 2n+1. This chroma now exists horizontally halfway between the two luma samples."]
    #[inline]
    pub fn interstitial(self) -> &'a mut W {
        self.variant(CSC_SUBSAMPLE_FILTERW::INTERSTITIAL)
    }
    #[doc = "Chroma samples numbered 2n-1, 2n, and 2n+1 are averaged (weights 1/4, 1/2, 1/4) and that chroma value exists at the same site as the luma sample numbered 2n and the chroma samples at 2n+1 are discarded."]
    #[inline]
    pub fn cosited(self) -> &'a mut W {
        self.variant(CSC_SUBSAMPLE_FILTERW::COSITED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _C0W<'a> {
    w: &'a mut W,
}
impl<'a> _C0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
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
    #[doc = "Bits 0:1 - This register describes the filtering and subsampling scheme to be performed on the chroma components in order to convert from YCbCr 4:4:4 to YCbCr 4:2:2 space"]
    #[inline]
    pub fn csc_subsample_filter(&self) -> CSC_SUBSAMPLE_FILTERR {
        CSC_SUBSAMPLE_FILTERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:25 - Two's complement red multiplier coefficient for Y"]
    #[inline]
    pub fn c0(&self) -> C0R {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        C0R { bits }
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
    #[doc = "Bits 0:1 - This register describes the filtering and subsampling scheme to be performed on the chroma components in order to convert from YCbCr 4:4:4 to YCbCr 4:2:2 space"]
    #[inline]
    pub fn csc_subsample_filter(&mut self) -> _CSC_SUBSAMPLE_FILTERW {
        _CSC_SUBSAMPLE_FILTERW { w: self }
    }
    #[doc = "Bits 16:25 - Two's complement red multiplier coefficient for Y"]
    #[inline]
    pub fn c0(&mut self) -> _C0W {
        _C0W { w: self }
    }
}
