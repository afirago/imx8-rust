#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PSCCR {
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
#[doc = "Possible values of the field `ENCACHE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENCACHER {
    #[doc = "Cache disabled"]
    ENCACHE_0,
    #[doc = "Cache enabled"]
    ENCACHE_1,
}
impl ENCACHER {
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
            ENCACHER::ENCACHE_0 => false,
            ENCACHER::ENCACHE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENCACHER {
        match value {
            false => ENCACHER::ENCACHE_0,
            true => ENCACHER::ENCACHE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENCACHE_0`"]
    #[inline]
    pub fn is_encache_0(&self) -> bool {
        *self == ENCACHER::ENCACHE_0
    }
    #[doc = "Checks if the value of the field is `ENCACHE_1`"]
    #[inline]
    pub fn is_encache_1(&self) -> bool {
        *self == ENCACHER::ENCACHE_1
    }
}
#[doc = "Possible values of the field `ENWRBUF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENWRBUFR {
    #[doc = "Write buffer disabled"]
    ENWRBUF_0,
    #[doc = "Write buffer enabled"]
    ENWRBUF_1,
}
impl ENWRBUFR {
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
            ENWRBUFR::ENWRBUF_0 => false,
            ENWRBUFR::ENWRBUF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENWRBUFR {
        match value {
            false => ENWRBUFR::ENWRBUF_0,
            true => ENWRBUFR::ENWRBUF_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENWRBUF_0`"]
    #[inline]
    pub fn is_enwrbuf_0(&self) -> bool {
        *self == ENWRBUFR::ENWRBUF_0
    }
    #[doc = "Checks if the value of the field is `ENWRBUF_1`"]
    #[inline]
    pub fn is_enwrbuf_1(&self) -> bool {
        *self == ENWRBUFR::ENWRBUF_1
    }
}
#[doc = "Possible values of the field `INVW0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVW0R {
    #[doc = "No operation"]
    INVW0_0,
    #[doc = "When setting the GO bit, invalidate all lines in way 0."]
    INVW0_1,
}
impl INVW0R {
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
            INVW0R::INVW0_0 => false,
            INVW0R::INVW0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVW0R {
        match value {
            false => INVW0R::INVW0_0,
            true => INVW0R::INVW0_1,
        }
    }
    #[doc = "Checks if the value of the field is `INVW0_0`"]
    #[inline]
    pub fn is_invw0_0(&self) -> bool {
        *self == INVW0R::INVW0_0
    }
    #[doc = "Checks if the value of the field is `INVW0_1`"]
    #[inline]
    pub fn is_invw0_1(&self) -> bool {
        *self == INVW0R::INVW0_1
    }
}
#[doc = "Possible values of the field `PUSHW0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUSHW0R {
    #[doc = "No operation"]
    PUSHW0_0,
    #[doc = "When setting the GO bit, push all modified lines in way 0"]
    PUSHW0_1,
}
impl PUSHW0R {
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
            PUSHW0R::PUSHW0_0 => false,
            PUSHW0R::PUSHW0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PUSHW0R {
        match value {
            false => PUSHW0R::PUSHW0_0,
            true => PUSHW0R::PUSHW0_1,
        }
    }
    #[doc = "Checks if the value of the field is `PUSHW0_0`"]
    #[inline]
    pub fn is_pushw0_0(&self) -> bool {
        *self == PUSHW0R::PUSHW0_0
    }
    #[doc = "Checks if the value of the field is `PUSHW0_1`"]
    #[inline]
    pub fn is_pushw0_1(&self) -> bool {
        *self == PUSHW0R::PUSHW0_1
    }
}
#[doc = "Possible values of the field `INVW1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVW1R {
    #[doc = "No operation"]
    INVW1_0,
    #[doc = "When setting the GO bit, invalidate all lines in way 1"]
    INVW1_1,
}
impl INVW1R {
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
            INVW1R::INVW1_0 => false,
            INVW1R::INVW1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVW1R {
        match value {
            false => INVW1R::INVW1_0,
            true => INVW1R::INVW1_1,
        }
    }
    #[doc = "Checks if the value of the field is `INVW1_0`"]
    #[inline]
    pub fn is_invw1_0(&self) -> bool {
        *self == INVW1R::INVW1_0
    }
    #[doc = "Checks if the value of the field is `INVW1_1`"]
    #[inline]
    pub fn is_invw1_1(&self) -> bool {
        *self == INVW1R::INVW1_1
    }
}
#[doc = "Possible values of the field `PUSHW1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUSHW1R {
    #[doc = "No operation"]
    PUSHW1_0,
    #[doc = "When setting the GO bit, push all modified lines in way 1"]
    PUSHW1_1,
}
impl PUSHW1R {
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
            PUSHW1R::PUSHW1_0 => false,
            PUSHW1R::PUSHW1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PUSHW1R {
        match value {
            false => PUSHW1R::PUSHW1_0,
            true => PUSHW1R::PUSHW1_1,
        }
    }
    #[doc = "Checks if the value of the field is `PUSHW1_0`"]
    #[inline]
    pub fn is_pushw1_0(&self) -> bool {
        *self == PUSHW1R::PUSHW1_0
    }
    #[doc = "Checks if the value of the field is `PUSHW1_1`"]
    #[inline]
    pub fn is_pushw1_1(&self) -> bool {
        *self == PUSHW1R::PUSHW1_1
    }
}
#[doc = "Possible values of the field `GO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GOR {
    #[doc = "Write: no effect. Read: no cache command active."]
    GO_0,
    #[doc = "Write: initiate command indicated by bits 27-24. Read: cache command active."]
    GO_1,
}
impl GOR {
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
            GOR::GO_0 => false,
            GOR::GO_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GOR {
        match value {
            false => GOR::GO_0,
            true => GOR::GO_1,
        }
    }
    #[doc = "Checks if the value of the field is `GO_0`"]
    #[inline]
    pub fn is_go_0(&self) -> bool {
        *self == GOR::GO_0
    }
    #[doc = "Checks if the value of the field is `GO_1`"]
    #[inline]
    pub fn is_go_1(&self) -> bool {
        *self == GOR::GO_1
    }
}
#[doc = "Values that can be written to the field `ENCACHE`"]
pub enum ENCACHEW {
    #[doc = "Cache disabled"]
    ENCACHE_0,
    #[doc = "Cache enabled"]
    ENCACHE_1,
}
impl ENCACHEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENCACHEW::ENCACHE_0 => false,
            ENCACHEW::ENCACHE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENCACHEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENCACHEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENCACHEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Cache disabled"]
    #[inline]
    pub fn encache_0(self) -> &'a mut W {
        self.variant(ENCACHEW::ENCACHE_0)
    }
    #[doc = "Cache enabled"]
    #[inline]
    pub fn encache_1(self) -> &'a mut W {
        self.variant(ENCACHEW::ENCACHE_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENWRBUF`"]
pub enum ENWRBUFW {
    #[doc = "Write buffer disabled"]
    ENWRBUF_0,
    #[doc = "Write buffer enabled"]
    ENWRBUF_1,
}
impl ENWRBUFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENWRBUFW::ENWRBUF_0 => false,
            ENWRBUFW::ENWRBUF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENWRBUFW<'a> {
    w: &'a mut W,
}
impl<'a> _ENWRBUFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENWRBUFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write buffer disabled"]
    #[inline]
    pub fn enwrbuf_0(self) -> &'a mut W {
        self.variant(ENWRBUFW::ENWRBUF_0)
    }
    #[doc = "Write buffer enabled"]
    #[inline]
    pub fn enwrbuf_1(self) -> &'a mut W {
        self.variant(ENWRBUFW::ENWRBUF_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INVW0`"]
pub enum INVW0W {
    #[doc = "No operation"]
    INVW0_0,
    #[doc = "When setting the GO bit, invalidate all lines in way 0."]
    INVW0_1,
}
impl INVW0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVW0W::INVW0_0 => false,
            INVW0W::INVW0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVW0W<'a> {
    w: &'a mut W,
}
impl<'a> _INVW0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVW0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No operation"]
    #[inline]
    pub fn invw0_0(self) -> &'a mut W {
        self.variant(INVW0W::INVW0_0)
    }
    #[doc = "When setting the GO bit, invalidate all lines in way 0."]
    #[inline]
    pub fn invw0_1(self) -> &'a mut W {
        self.variant(INVW0W::INVW0_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PUSHW0`"]
pub enum PUSHW0W {
    #[doc = "No operation"]
    PUSHW0_0,
    #[doc = "When setting the GO bit, push all modified lines in way 0"]
    PUSHW0_1,
}
impl PUSHW0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PUSHW0W::PUSHW0_0 => false,
            PUSHW0W::PUSHW0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PUSHW0W<'a> {
    w: &'a mut W,
}
impl<'a> _PUSHW0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUSHW0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No operation"]
    #[inline]
    pub fn pushw0_0(self) -> &'a mut W {
        self.variant(PUSHW0W::PUSHW0_0)
    }
    #[doc = "When setting the GO bit, push all modified lines in way 0"]
    #[inline]
    pub fn pushw0_1(self) -> &'a mut W {
        self.variant(PUSHW0W::PUSHW0_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INVW1`"]
pub enum INVW1W {
    #[doc = "No operation"]
    INVW1_0,
    #[doc = "When setting the GO bit, invalidate all lines in way 1"]
    INVW1_1,
}
impl INVW1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVW1W::INVW1_0 => false,
            INVW1W::INVW1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVW1W<'a> {
    w: &'a mut W,
}
impl<'a> _INVW1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVW1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No operation"]
    #[inline]
    pub fn invw1_0(self) -> &'a mut W {
        self.variant(INVW1W::INVW1_0)
    }
    #[doc = "When setting the GO bit, invalidate all lines in way 1"]
    #[inline]
    pub fn invw1_1(self) -> &'a mut W {
        self.variant(INVW1W::INVW1_1)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PUSHW1`"]
pub enum PUSHW1W {
    #[doc = "No operation"]
    PUSHW1_0,
    #[doc = "When setting the GO bit, push all modified lines in way 1"]
    PUSHW1_1,
}
impl PUSHW1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PUSHW1W::PUSHW1_0 => false,
            PUSHW1W::PUSHW1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PUSHW1W<'a> {
    w: &'a mut W,
}
impl<'a> _PUSHW1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUSHW1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No operation"]
    #[inline]
    pub fn pushw1_0(self) -> &'a mut W {
        self.variant(PUSHW1W::PUSHW1_0)
    }
    #[doc = "When setting the GO bit, push all modified lines in way 1"]
    #[inline]
    pub fn pushw1_1(self) -> &'a mut W {
        self.variant(PUSHW1W::PUSHW1_1)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GO`"]
pub enum GOW {
    #[doc = "Write: no effect. Read: no cache command active."]
    GO_0,
    #[doc = "Write: initiate command indicated by bits 27-24. Read: cache command active."]
    GO_1,
}
impl GOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GOW::GO_0 => false,
            GOW::GO_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GOW<'a> {
    w: &'a mut W,
}
impl<'a> _GOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: no effect. Read: no cache command active."]
    #[inline]
    pub fn go_0(self) -> &'a mut W {
        self.variant(GOW::GO_0)
    }
    #[doc = "Write: initiate command indicated by bits 27-24. Read: cache command active."]
    #[inline]
    pub fn go_1(self) -> &'a mut W {
        self.variant(GOW::GO_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Cache enable"]
    #[inline]
    pub fn encache(&self) -> ENCACHER {
        ENCACHER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable Write Buffer"]
    #[inline]
    pub fn enwrbuf(&self) -> ENWRBUFR {
        ENWRBUFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Invalidate Way 0"]
    #[inline]
    pub fn invw0(&self) -> INVW0R {
        INVW0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Push Way 0"]
    #[inline]
    pub fn pushw0(&self) -> PUSHW0R {
        PUSHW0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Invalidate Way 1"]
    #[inline]
    pub fn invw1(&self) -> INVW1R {
        INVW1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Push Way 1"]
    #[inline]
    pub fn pushw1(&self) -> PUSHW1R {
        PUSHW1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Initiate Cache Command"]
    #[inline]
    pub fn go(&self) -> GOR {
        GOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Cache enable"]
    #[inline]
    pub fn encache(&mut self) -> _ENCACHEW {
        _ENCACHEW { w: self }
    }
    #[doc = "Bit 1 - Enable Write Buffer"]
    #[inline]
    pub fn enwrbuf(&mut self) -> _ENWRBUFW {
        _ENWRBUFW { w: self }
    }
    #[doc = "Bit 24 - Invalidate Way 0"]
    #[inline]
    pub fn invw0(&mut self) -> _INVW0W {
        _INVW0W { w: self }
    }
    #[doc = "Bit 25 - Push Way 0"]
    #[inline]
    pub fn pushw0(&mut self) -> _PUSHW0W {
        _PUSHW0W { w: self }
    }
    #[doc = "Bit 26 - Invalidate Way 1"]
    #[inline]
    pub fn invw1(&mut self) -> _INVW1W {
        _INVW1W { w: self }
    }
    #[doc = "Bit 27 - Push Way 1"]
    #[inline]
    pub fn pushw1(&mut self) -> _PUSHW1W {
        _PUSHW1W { w: self }
    }
    #[doc = "Bit 31 - Initiate Cache Command"]
    #[inline]
    pub fn go(&mut self) -> _GOW {
        _GOW { w: self }
    }
}
