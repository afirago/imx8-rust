#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PLL_CTRL_TOG {
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
#[doc = "Possible values of the field `SETTING0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETTING0R {
    #[doc = "Domain clocks not needed"]
    SETTING0_0,
    #[doc = "Domain clocks needed when in RUN"]
    SETTING0_1,
    #[doc = "Domain clocks needed when in RUN and WAIT"]
    SETTING0_2,
    #[doc = "Domain clocks needed all the time"]
    SETTING0_3,
}
impl SETTING0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETTING0R::SETTING0_0 => 0,
            SETTING0R::SETTING0_1 => 1,
            SETTING0R::SETTING0_2 => 2,
            SETTING0R::SETTING0_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETTING0R {
        match value {
            0 => SETTING0R::SETTING0_0,
            1 => SETTING0R::SETTING0_1,
            2 => SETTING0R::SETTING0_2,
            3 => SETTING0R::SETTING0_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SETTING0_0`"]
    #[inline]
    pub fn is_setting0_0(&self) -> bool {
        *self == SETTING0R::SETTING0_0
    }
    #[doc = "Checks if the value of the field is `SETTING0_1`"]
    #[inline]
    pub fn is_setting0_1(&self) -> bool {
        *self == SETTING0R::SETTING0_1
    }
    #[doc = "Checks if the value of the field is `SETTING0_2`"]
    #[inline]
    pub fn is_setting0_2(&self) -> bool {
        *self == SETTING0R::SETTING0_2
    }
    #[doc = "Checks if the value of the field is `SETTING0_3`"]
    #[inline]
    pub fn is_setting0_3(&self) -> bool {
        *self == SETTING0R::SETTING0_3
    }
}
#[doc = "Possible values of the field `SETTING1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETTING1R {
    #[doc = "Domain clocks not needed"]
    SETTING1_0,
    #[doc = "Domain clocks needed when in RUN"]
    SETTING1_1,
    #[doc = "Domain clocks needed when in RUN and WAIT"]
    SETTING1_2,
    #[doc = "Domain clocks needed all the time"]
    SETTING1_3,
}
impl SETTING1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETTING1R::SETTING1_0 => 0,
            SETTING1R::SETTING1_1 => 1,
            SETTING1R::SETTING1_2 => 2,
            SETTING1R::SETTING1_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETTING1R {
        match value {
            0 => SETTING1R::SETTING1_0,
            1 => SETTING1R::SETTING1_1,
            2 => SETTING1R::SETTING1_2,
            3 => SETTING1R::SETTING1_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SETTING1_0`"]
    #[inline]
    pub fn is_setting1_0(&self) -> bool {
        *self == SETTING1R::SETTING1_0
    }
    #[doc = "Checks if the value of the field is `SETTING1_1`"]
    #[inline]
    pub fn is_setting1_1(&self) -> bool {
        *self == SETTING1R::SETTING1_1
    }
    #[doc = "Checks if the value of the field is `SETTING1_2`"]
    #[inline]
    pub fn is_setting1_2(&self) -> bool {
        *self == SETTING1R::SETTING1_2
    }
    #[doc = "Checks if the value of the field is `SETTING1_3`"]
    #[inline]
    pub fn is_setting1_3(&self) -> bool {
        *self == SETTING1R::SETTING1_3
    }
}
#[doc = "Possible values of the field `SETTING2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETTING2R {
    #[doc = "Domain clocks not needed"]
    SETTING2_0,
    #[doc = "Domain clocks needed when in RUN"]
    SETTING2_1,
    #[doc = "Domain clocks needed when in RUN and WAIT"]
    SETTING2_2,
    #[doc = "Domain clocks needed all the time"]
    SETTING2_3,
}
impl SETTING2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETTING2R::SETTING2_0 => 0,
            SETTING2R::SETTING2_1 => 1,
            SETTING2R::SETTING2_2 => 2,
            SETTING2R::SETTING2_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETTING2R {
        match value {
            0 => SETTING2R::SETTING2_0,
            1 => SETTING2R::SETTING2_1,
            2 => SETTING2R::SETTING2_2,
            3 => SETTING2R::SETTING2_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SETTING2_0`"]
    #[inline]
    pub fn is_setting2_0(&self) -> bool {
        *self == SETTING2R::SETTING2_0
    }
    #[doc = "Checks if the value of the field is `SETTING2_1`"]
    #[inline]
    pub fn is_setting2_1(&self) -> bool {
        *self == SETTING2R::SETTING2_1
    }
    #[doc = "Checks if the value of the field is `SETTING2_2`"]
    #[inline]
    pub fn is_setting2_2(&self) -> bool {
        *self == SETTING2R::SETTING2_2
    }
    #[doc = "Checks if the value of the field is `SETTING2_3`"]
    #[inline]
    pub fn is_setting2_3(&self) -> bool {
        *self == SETTING2R::SETTING2_3
    }
}
#[doc = "Possible values of the field `SETTING3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETTING3R {
    #[doc = "Domain clocks not needed"]
    SETTING3_0,
    #[doc = "Domain clocks needed when in RUN"]
    SETTING3_1,
    #[doc = "Domain clocks needed when in RUN and WAIT"]
    SETTING3_2,
    #[doc = "Domain clocks needed all the time"]
    SETTING3_3,
}
impl SETTING3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETTING3R::SETTING3_0 => 0,
            SETTING3R::SETTING3_1 => 1,
            SETTING3R::SETTING3_2 => 2,
            SETTING3R::SETTING3_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETTING3R {
        match value {
            0 => SETTING3R::SETTING3_0,
            1 => SETTING3R::SETTING3_1,
            2 => SETTING3R::SETTING3_2,
            3 => SETTING3R::SETTING3_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SETTING3_0`"]
    #[inline]
    pub fn is_setting3_0(&self) -> bool {
        *self == SETTING3R::SETTING3_0
    }
    #[doc = "Checks if the value of the field is `SETTING3_1`"]
    #[inline]
    pub fn is_setting3_1(&self) -> bool {
        *self == SETTING3R::SETTING3_1
    }
    #[doc = "Checks if the value of the field is `SETTING3_2`"]
    #[inline]
    pub fn is_setting3_2(&self) -> bool {
        *self == SETTING3R::SETTING3_2
    }
    #[doc = "Checks if the value of the field is `SETTING3_3`"]
    #[inline]
    pub fn is_setting3_3(&self) -> bool {
        *self == SETTING3R::SETTING3_3
    }
}
#[doc = "Values that can be written to the field `SETTING0`"]
pub enum SETTING0W {
    #[doc = "Domain clocks not needed"]
    SETTING0_0,
    #[doc = "Domain clocks needed when in RUN"]
    SETTING0_1,
    #[doc = "Domain clocks needed when in RUN and WAIT"]
    SETTING0_2,
    #[doc = "Domain clocks needed all the time"]
    SETTING0_3,
}
impl SETTING0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETTING0W::SETTING0_0 => 0,
            SETTING0W::SETTING0_1 => 1,
            SETTING0W::SETTING0_2 => 2,
            SETTING0W::SETTING0_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETTING0W<'a> {
    w: &'a mut W,
}
impl<'a> _SETTING0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETTING0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Domain clocks not needed"]
    #[inline]
    pub fn setting0_0(self) -> &'a mut W {
        self.variant(SETTING0W::SETTING0_0)
    }
    #[doc = "Domain clocks needed when in RUN"]
    #[inline]
    pub fn setting0_1(self) -> &'a mut W {
        self.variant(SETTING0W::SETTING0_1)
    }
    #[doc = "Domain clocks needed when in RUN and WAIT"]
    #[inline]
    pub fn setting0_2(self) -> &'a mut W {
        self.variant(SETTING0W::SETTING0_2)
    }
    #[doc = "Domain clocks needed all the time"]
    #[inline]
    pub fn setting0_3(self) -> &'a mut W {
        self.variant(SETTING0W::SETTING0_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETTING1`"]
pub enum SETTING1W {
    #[doc = "Domain clocks not needed"]
    SETTING1_0,
    #[doc = "Domain clocks needed when in RUN"]
    SETTING1_1,
    #[doc = "Domain clocks needed when in RUN and WAIT"]
    SETTING1_2,
    #[doc = "Domain clocks needed all the time"]
    SETTING1_3,
}
impl SETTING1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETTING1W::SETTING1_0 => 0,
            SETTING1W::SETTING1_1 => 1,
            SETTING1W::SETTING1_2 => 2,
            SETTING1W::SETTING1_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETTING1W<'a> {
    w: &'a mut W,
}
impl<'a> _SETTING1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETTING1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Domain clocks not needed"]
    #[inline]
    pub fn setting1_0(self) -> &'a mut W {
        self.variant(SETTING1W::SETTING1_0)
    }
    #[doc = "Domain clocks needed when in RUN"]
    #[inline]
    pub fn setting1_1(self) -> &'a mut W {
        self.variant(SETTING1W::SETTING1_1)
    }
    #[doc = "Domain clocks needed when in RUN and WAIT"]
    #[inline]
    pub fn setting1_2(self) -> &'a mut W {
        self.variant(SETTING1W::SETTING1_2)
    }
    #[doc = "Domain clocks needed all the time"]
    #[inline]
    pub fn setting1_3(self) -> &'a mut W {
        self.variant(SETTING1W::SETTING1_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETTING2`"]
pub enum SETTING2W {
    #[doc = "Domain clocks not needed"]
    SETTING2_0,
    #[doc = "Domain clocks needed when in RUN"]
    SETTING2_1,
    #[doc = "Domain clocks needed when in RUN and WAIT"]
    SETTING2_2,
    #[doc = "Domain clocks needed all the time"]
    SETTING2_3,
}
impl SETTING2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETTING2W::SETTING2_0 => 0,
            SETTING2W::SETTING2_1 => 1,
            SETTING2W::SETTING2_2 => 2,
            SETTING2W::SETTING2_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETTING2W<'a> {
    w: &'a mut W,
}
impl<'a> _SETTING2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETTING2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Domain clocks not needed"]
    #[inline]
    pub fn setting2_0(self) -> &'a mut W {
        self.variant(SETTING2W::SETTING2_0)
    }
    #[doc = "Domain clocks needed when in RUN"]
    #[inline]
    pub fn setting2_1(self) -> &'a mut W {
        self.variant(SETTING2W::SETTING2_1)
    }
    #[doc = "Domain clocks needed when in RUN and WAIT"]
    #[inline]
    pub fn setting2_2(self) -> &'a mut W {
        self.variant(SETTING2W::SETTING2_2)
    }
    #[doc = "Domain clocks needed all the time"]
    #[inline]
    pub fn setting2_3(self) -> &'a mut W {
        self.variant(SETTING2W::SETTING2_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETTING3`"]
pub enum SETTING3W {
    #[doc = "Domain clocks not needed"]
    SETTING3_0,
    #[doc = "Domain clocks needed when in RUN"]
    SETTING3_1,
    #[doc = "Domain clocks needed when in RUN and WAIT"]
    SETTING3_2,
    #[doc = "Domain clocks needed all the time"]
    SETTING3_3,
}
impl SETTING3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETTING3W::SETTING3_0 => 0,
            SETTING3W::SETTING3_1 => 1,
            SETTING3W::SETTING3_2 => 2,
            SETTING3W::SETTING3_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETTING3W<'a> {
    w: &'a mut W,
}
impl<'a> _SETTING3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETTING3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Domain clocks not needed"]
    #[inline]
    pub fn setting3_0(self) -> &'a mut W {
        self.variant(SETTING3W::SETTING3_0)
    }
    #[doc = "Domain clocks needed when in RUN"]
    #[inline]
    pub fn setting3_1(self) -> &'a mut W {
        self.variant(SETTING3W::SETTING3_1)
    }
    #[doc = "Domain clocks needed when in RUN and WAIT"]
    #[inline]
    pub fn setting3_2(self) -> &'a mut W {
        self.variant(SETTING3W::SETTING3_2)
    }
    #[doc = "Domain clocks needed all the time"]
    #[inline]
    pub fn setting3_3(self) -> &'a mut W {
        self.variant(SETTING3W::SETTING3_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:1 - Clock gate control setting for domain 0. This field can only be written by domain 0."]
    #[inline]
    pub fn setting0(&self) -> SETTING0R {
        SETTING0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Clock gate control setting for domain 1. This field can only be written by domain 1."]
    #[inline]
    pub fn setting1(&self) -> SETTING1R {
        SETTING1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Clock gate control setting for domain 2. This field can only be written by domain 2"]
    #[inline]
    pub fn setting2(&self) -> SETTING2R {
        SETTING2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Clock gate control setting for domain 3. This field can only be written by domain 3"]
    #[inline]
    pub fn setting3(&self) -> SETTING3R {
        SETTING3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Clock gate control setting for domain 0. This field can only be written by domain 0."]
    #[inline]
    pub fn setting0(&mut self) -> _SETTING0W {
        _SETTING0W { w: self }
    }
    #[doc = "Bits 4:5 - Clock gate control setting for domain 1. This field can only be written by domain 1."]
    #[inline]
    pub fn setting1(&mut self) -> _SETTING1W {
        _SETTING1W { w: self }
    }
    #[doc = "Bits 8:9 - Clock gate control setting for domain 2. This field can only be written by domain 2"]
    #[inline]
    pub fn setting2(&mut self) -> _SETTING2W {
        _SETTING2W { w: self }
    }
    #[doc = "Bits 12:13 - Clock gate control setting for domain 3. This field can only be written by domain 3"]
    #[inline]
    pub fn setting3(&mut self) -> _SETTING3W {
        _SETTING3W { w: self }
    }
}
