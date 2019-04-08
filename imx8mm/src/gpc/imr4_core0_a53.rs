#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IMR4_CORE0_A53 {
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
#[doc = "Possible values of the field `IMR4_CORE0_A53`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMR4_CORE0_A53R {
    #[doc = "IRQ not masked"]
    IMR4_CORE0_A53_0,
    #[doc = "IRQ masked"]
    IMR4_CORE0_A53_1,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl IMR4_CORE0_A53R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            IMR4_CORE0_A53R::IMR4_CORE0_A53_0 => 0,
            IMR4_CORE0_A53R::IMR4_CORE0_A53_1 => 1,
            IMR4_CORE0_A53R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> IMR4_CORE0_A53R {
        match value {
            0 => IMR4_CORE0_A53R::IMR4_CORE0_A53_0,
            1 => IMR4_CORE0_A53R::IMR4_CORE0_A53_1,
            i => IMR4_CORE0_A53R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMR4_CORE0_A53_0`"]
    #[inline]
    pub fn is_imr4_core0_a53_0(&self) -> bool {
        *self == IMR4_CORE0_A53R::IMR4_CORE0_A53_0
    }
    #[doc = "Checks if the value of the field is `IMR4_CORE0_A53_1`"]
    #[inline]
    pub fn is_imr4_core0_a53_1(&self) -> bool {
        *self == IMR4_CORE0_A53R::IMR4_CORE0_A53_1
    }
}
#[doc = "Values that can be written to the field `IMR4_CORE0_A53`"]
pub enum IMR4_CORE0_A53W {
    #[doc = "IRQ not masked"]
    IMR4_CORE0_A53_0,
    #[doc = "IRQ masked"]
    IMR4_CORE0_A53_1,
}
impl IMR4_CORE0_A53W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            IMR4_CORE0_A53W::IMR4_CORE0_A53_0 => 0,
            IMR4_CORE0_A53W::IMR4_CORE0_A53_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IMR4_CORE0_A53W<'a> {
    w: &'a mut W,
}
impl<'a> _IMR4_CORE0_A53W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IMR4_CORE0_A53W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "IRQ not masked"]
    #[inline]
    pub fn imr4_core0_a53_0(self) -> &'a mut W {
        self.variant(IMR4_CORE0_A53W::IMR4_CORE0_A53_0)
    }
    #[doc = "IRQ masked"]
    #[inline]
    pub fn imr4_core0_a53_1(self) -> &'a mut W {
        self.variant(IMR4_CORE0_A53W::IMR4_CORE0_A53_1)
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
    #[doc = "Bits 0:31 - A53 core0 IRQ\\[127:96\\] masking bits:"]
    #[inline]
    pub fn imr4_core0_a53(&self) -> IMR4_CORE0_A53R {
        IMR4_CORE0_A53R::_from({
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
    #[doc = "Bits 0:31 - A53 core0 IRQ\\[127:96\\] masking bits:"]
    #[inline]
    pub fn imr4_core0_a53(&mut self) -> _IMR4_CORE0_A53W {
        _IMR4_CORE0_A53W { w: self }
    }
}
