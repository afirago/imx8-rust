#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IMR1_M4 {
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
#[doc = "Possible values of the field `IMR1_M4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMR1_M4R {
    #[doc = "IRQ not masked"]
    IMR1_M4_0,
    #[doc = "IRQ masked"]
    IMR1_M4_1,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl IMR1_M4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            IMR1_M4R::IMR1_M4_0 => 0,
            IMR1_M4R::IMR1_M4_1 => 1,
            IMR1_M4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> IMR1_M4R {
        match value {
            0 => IMR1_M4R::IMR1_M4_0,
            1 => IMR1_M4R::IMR1_M4_1,
            i => IMR1_M4R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMR1_M4_0`"]
    #[inline]
    pub fn is_imr1_m4_0(&self) -> bool {
        *self == IMR1_M4R::IMR1_M4_0
    }
    #[doc = "Checks if the value of the field is `IMR1_M4_1`"]
    #[inline]
    pub fn is_imr1_m4_1(&self) -> bool {
        *self == IMR1_M4R::IMR1_M4_1
    }
}
#[doc = "Values that can be written to the field `IMR1_M4`"]
pub enum IMR1_M4W {
    #[doc = "IRQ not masked"]
    IMR1_M4_0,
    #[doc = "IRQ masked"]
    IMR1_M4_1,
}
impl IMR1_M4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            IMR1_M4W::IMR1_M4_0 => 0,
            IMR1_M4W::IMR1_M4_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IMR1_M4W<'a> {
    w: &'a mut W,
}
impl<'a> _IMR1_M4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IMR1_M4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "IRQ not masked"]
    #[inline]
    pub fn imr1_m4_0(self) -> &'a mut W {
        self.variant(IMR1_M4W::IMR1_M4_0)
    }
    #[doc = "IRQ masked"]
    #[inline]
    pub fn imr1_m4_1(self) -> &'a mut W {
        self.variant(IMR1_M4W::IMR1_M4_1)
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
    #[doc = "Bits 0:31 - M4 IRQ\\[31:0\\] masking bits:"]
    #[inline]
    pub fn imr1_m4(&self) -> IMR1_M4R {
        IMR1_M4R::_from({
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
    #[doc = "Bits 0:31 - M4 IRQ\\[31:0\\] masking bits:"]
    #[inline]
    pub fn imr1_m4(&mut self) -> _IMR1_M4W {
        _IMR1_M4W { w: self }
    }
}
