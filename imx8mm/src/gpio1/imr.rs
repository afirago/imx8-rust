#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IMR {
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
#[doc = "Possible values of the field `IMR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMRR {
    #[doc = "Interrupt n is disabled."]
    MASKED,
    #[doc = "Interrupt n is enabled."]
    UNMASKED,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl IMRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            IMRR::MASKED => 0,
            IMRR::UNMASKED => 1,
            IMRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> IMRR {
        match value {
            0 => IMRR::MASKED,
            1 => IMRR::UNMASKED,
            i => IMRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline]
    pub fn is_masked(&self) -> bool {
        *self == IMRR::MASKED
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline]
    pub fn is_unmasked(&self) -> bool {
        *self == IMRR::UNMASKED
    }
}
#[doc = "Values that can be written to the field `IMR`"]
pub enum IMRW {
    #[doc = "Interrupt n is disabled."]
    MASKED,
    #[doc = "Interrupt n is enabled."]
    UNMASKED,
}
impl IMRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            IMRW::MASKED => 0,
            IMRW::UNMASKED => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IMRW<'a> {
    w: &'a mut W,
}
impl<'a> _IMRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IMRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Interrupt n is disabled."]
    #[inline]
    pub fn masked(self) -> &'a mut W {
        self.variant(IMRW::MASKED)
    }
    #[doc = "Interrupt n is enabled."]
    #[inline]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IMRW::UNMASKED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
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
    #[doc = "Bits 0:31 - Interrupt Mask bits"]
    #[inline]
    pub fn imr(&self) -> IMRR {
        IMRR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
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
    #[doc = "Bits 0:31 - Interrupt Mask bits"]
    #[inline]
    pub fn imr(&mut self) -> _IMRW {
        _IMRW { w: self }
    }
}
