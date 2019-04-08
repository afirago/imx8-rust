#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LPCR_A53_BSC2 {
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
#[doc = "Possible values of the field `LPM2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPM2R {
    #[doc = "Remain in RUN mode"]
    LPM2_0,
    #[doc = "Transfer to WAIT mode"]
    LPM2_1,
    #[doc = "Transfer to STOP mode"]
    LPM2_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LPM2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPM2R::LPM2_0 => 0,
            LPM2R::LPM2_1 => 1,
            LPM2R::LPM2_2 => 2,
            LPM2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPM2R {
        match value {
            0 => LPM2R::LPM2_0,
            1 => LPM2R::LPM2_1,
            2 => LPM2R::LPM2_2,
            i => LPM2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LPM2_0`"]
    #[inline]
    pub fn is_lpm2_0(&self) -> bool {
        *self == LPM2R::LPM2_0
    }
    #[doc = "Checks if the value of the field is `LPM2_1`"]
    #[inline]
    pub fn is_lpm2_1(&self) -> bool {
        *self == LPM2R::LPM2_1
    }
    #[doc = "Checks if the value of the field is `LPM2_2`"]
    #[inline]
    pub fn is_lpm2_2(&self) -> bool {
        *self == LPM2R::LPM2_2
    }
}
#[doc = "Possible values of the field `LPM3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPM3R {
    #[doc = "Remain in RUN mode"]
    LPM3_0,
    #[doc = "Transfer to WAIT mode"]
    LPM3_1,
    #[doc = "Transfer to STOP mode"]
    LPM3_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LPM3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPM3R::LPM3_0 => 0,
            LPM3R::LPM3_1 => 1,
            LPM3R::LPM3_2 => 2,
            LPM3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPM3R {
        match value {
            0 => LPM3R::LPM3_0,
            1 => LPM3R::LPM3_1,
            2 => LPM3R::LPM3_2,
            i => LPM3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LPM3_0`"]
    #[inline]
    pub fn is_lpm3_0(&self) -> bool {
        *self == LPM3R::LPM3_0
    }
    #[doc = "Checks if the value of the field is `LPM3_1`"]
    #[inline]
    pub fn is_lpm3_1(&self) -> bool {
        *self == LPM3R::LPM3_1
    }
    #[doc = "Checks if the value of the field is `LPM3_2`"]
    #[inline]
    pub fn is_lpm3_2(&self) -> bool {
        *self == LPM3R::LPM3_2
    }
}
#[doc = "Values that can be written to the field `LPM2`"]
pub enum LPM2W {
    #[doc = "Remain in RUN mode"]
    LPM2_0,
    #[doc = "Transfer to WAIT mode"]
    LPM2_1,
    #[doc = "Transfer to STOP mode"]
    LPM2_2,
}
impl LPM2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPM2W::LPM2_0 => 0,
            LPM2W::LPM2_1 => 1,
            LPM2W::LPM2_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPM2W<'a> {
    w: &'a mut W,
}
impl<'a> _LPM2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPM2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Remain in RUN mode"]
    #[inline]
    pub fn lpm2_0(self) -> &'a mut W {
        self.variant(LPM2W::LPM2_0)
    }
    #[doc = "Transfer to WAIT mode"]
    #[inline]
    pub fn lpm2_1(self) -> &'a mut W {
        self.variant(LPM2W::LPM2_1)
    }
    #[doc = "Transfer to STOP mode"]
    #[inline]
    pub fn lpm2_2(self) -> &'a mut W {
        self.variant(LPM2W::LPM2_2)
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
#[doc = "Values that can be written to the field `LPM3`"]
pub enum LPM3W {
    #[doc = "Remain in RUN mode"]
    LPM3_0,
    #[doc = "Transfer to WAIT mode"]
    LPM3_1,
    #[doc = "Transfer to STOP mode"]
    LPM3_2,
}
impl LPM3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPM3W::LPM3_0 => 0,
            LPM3W::LPM3_1 => 1,
            LPM3W::LPM3_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPM3W<'a> {
    w: &'a mut W,
}
impl<'a> _LPM3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPM3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Remain in RUN mode"]
    #[inline]
    pub fn lpm3_0(self) -> &'a mut W {
        self.variant(LPM3W::LPM3_0)
    }
    #[doc = "Transfer to WAIT mode"]
    #[inline]
    pub fn lpm3_1(self) -> &'a mut W {
        self.variant(LPM3W::LPM3_1)
    }
    #[doc = "Transfer to STOP mode"]
    #[inline]
    pub fn lpm3_2(self) -> &'a mut W {
        self.variant(LPM3W::LPM3_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
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
    #[doc = "Bits 0:1 - CORE2 Setting the low power mode that system will enter on next assertion of dsm_request signal."]
    #[inline]
    pub fn lpm2(&self) -> LPM2R {
        LPM2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - CORE3 Setting the low power mode that system will enter on next assertion of dsm_request signal."]
    #[inline]
    pub fn lpm3(&self) -> LPM3R {
        LPM3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:1 - CORE2 Setting the low power mode that system will enter on next assertion of dsm_request signal."]
    #[inline]
    pub fn lpm2(&mut self) -> _LPM2W {
        _LPM2W { w: self }
    }
    #[doc = "Bits 2:3 - CORE3 Setting the low power mode that system will enter on next assertion of dsm_request signal."]
    #[inline]
    pub fn lpm3(&mut self) -> _LPM3W {
        _LPM3W { w: self }
    }
}
