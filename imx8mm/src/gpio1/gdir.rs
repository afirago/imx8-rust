#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GDIR {
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
#[doc = "Possible values of the field `GDIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GDIRR {
    #[doc = "GPIO is configured as input."]
    INPUT,
    #[doc = "GPIO is configured as output."]
    OUTPUT,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl GDIRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            GDIRR::INPUT => 0,
            GDIRR::OUTPUT => 1,
            GDIRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> GDIRR {
        match value {
            0 => GDIRR::INPUT,
            1 => GDIRR::OUTPUT,
            i => GDIRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == GDIRR::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == GDIRR::OUTPUT
    }
}
#[doc = "Values that can be written to the field `GDIR`"]
pub enum GDIRW {
    #[doc = "GPIO is configured as input."]
    INPUT,
    #[doc = "GPIO is configured as output."]
    OUTPUT,
}
impl GDIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            GDIRW::INPUT => 0,
            GDIRW::OUTPUT => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GDIRW<'a> {
    w: &'a mut W,
}
impl<'a> _GDIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GDIRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "GPIO is configured as input."]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(GDIRW::INPUT)
    }
    #[doc = "GPIO is configured as output."]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(GDIRW::OUTPUT)
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
    #[doc = "Bits 0:31 - GPIO direction bits"]
    #[inline]
    pub fn gdir(&self) -> GDIRR {
        GDIRR::_from({
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
    #[doc = "Bits 0:31 - GPIO direction bits"]
    #[inline]
    pub fn gdir(&mut self) -> _GDIRW {
        _GDIRW { w: self }
    }
}
