#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::A53RCR1 {
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
pub struct A53_CORE0_ENABLER {
    bits: bool,
}
impl A53_CORE0_ENABLER {
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
#[doc = "Possible values of the field `A53_CORE1_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A53_CORE1_ENABLER {
    #[doc = "core1 is disabled"]
    A53_CORE1_ENABLE_0,
    #[doc = "core1 is enabled"]
    A53_CORE1_ENABLE_1,
}
impl A53_CORE1_ENABLER {
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
            A53_CORE1_ENABLER::A53_CORE1_ENABLE_0 => false,
            A53_CORE1_ENABLER::A53_CORE1_ENABLE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> A53_CORE1_ENABLER {
        match value {
            false => A53_CORE1_ENABLER::A53_CORE1_ENABLE_0,
            true => A53_CORE1_ENABLER::A53_CORE1_ENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `A53_CORE1_ENABLE_0`"]
    #[inline]
    pub fn is_a53_core1_enable_0(&self) -> bool {
        *self == A53_CORE1_ENABLER::A53_CORE1_ENABLE_0
    }
    #[doc = "Checks if the value of the field is `A53_CORE1_ENABLE_1`"]
    #[inline]
    pub fn is_a53_core1_enable_1(&self) -> bool {
        *self == A53_CORE1_ENABLER::A53_CORE1_ENABLE_1
    }
}
#[doc = "Possible values of the field `A53_CORE2_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A53_CORE2_ENABLER {
    #[doc = "core2 is disabled"]
    A53_CORE2_ENABLE_0,
    #[doc = "core2 is enabled"]
    A53_CORE2_ENABLE_1,
}
impl A53_CORE2_ENABLER {
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
            A53_CORE2_ENABLER::A53_CORE2_ENABLE_0 => false,
            A53_CORE2_ENABLER::A53_CORE2_ENABLE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> A53_CORE2_ENABLER {
        match value {
            false => A53_CORE2_ENABLER::A53_CORE2_ENABLE_0,
            true => A53_CORE2_ENABLER::A53_CORE2_ENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `A53_CORE2_ENABLE_0`"]
    #[inline]
    pub fn is_a53_core2_enable_0(&self) -> bool {
        *self == A53_CORE2_ENABLER::A53_CORE2_ENABLE_0
    }
    #[doc = "Checks if the value of the field is `A53_CORE2_ENABLE_1`"]
    #[inline]
    pub fn is_a53_core2_enable_1(&self) -> bool {
        *self == A53_CORE2_ENABLER::A53_CORE2_ENABLE_1
    }
}
#[doc = "Possible values of the field `A53_CORE3_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A53_CORE3_ENABLER {
    #[doc = "core3 is disabled"]
    A53_CORE3_ENABLE_0,
    #[doc = "core3 is enabled"]
    A53_CORE3_ENABLE_1,
}
impl A53_CORE3_ENABLER {
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
            A53_CORE3_ENABLER::A53_CORE3_ENABLE_0 => false,
            A53_CORE3_ENABLER::A53_CORE3_ENABLE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> A53_CORE3_ENABLER {
        match value {
            false => A53_CORE3_ENABLER::A53_CORE3_ENABLE_0,
            true => A53_CORE3_ENABLER::A53_CORE3_ENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `A53_CORE3_ENABLE_0`"]
    #[inline]
    pub fn is_a53_core3_enable_0(&self) -> bool {
        *self == A53_CORE3_ENABLER::A53_CORE3_ENABLE_0
    }
    #[doc = "Checks if the value of the field is `A53_CORE3_ENABLE_1`"]
    #[inline]
    pub fn is_a53_core3_enable_1(&self) -> bool {
        *self == A53_CORE3_ENABLER::A53_CORE3_ENABLE_1
    }
}
#[doc = r" Value of the field"]
pub struct A53_RST_SLOWR {
    bits: u8,
}
impl A53_RST_SLOWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DOMAIN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOMAIN0R {
    #[doc = "This register is not assigned to domain0. The master from domain3 cannot write to this register."]
    DOMAIN0_0,
    #[doc = "This register is assigned to domain0. The master from domain3 can write to this register"]
    DOMAIN0_1,
}
impl DOMAIN0R {
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
            DOMAIN0R::DOMAIN0_0 => false,
            DOMAIN0R::DOMAIN0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOMAIN0R {
        match value {
            false => DOMAIN0R::DOMAIN0_0,
            true => DOMAIN0R::DOMAIN0_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOMAIN0_0`"]
    #[inline]
    pub fn is_domain0_0(&self) -> bool {
        *self == DOMAIN0R::DOMAIN0_0
    }
    #[doc = "Checks if the value of the field is `DOMAIN0_1`"]
    #[inline]
    pub fn is_domain0_1(&self) -> bool {
        *self == DOMAIN0R::DOMAIN0_1
    }
}
#[doc = "Possible values of the field `DOMAIN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOMAIN1R {
    #[doc = "This register is not assigned to domain1. The master from domain1 cannot write to this register."]
    DOMAIN1_0,
    #[doc = "This register is assigned to domain1. The master from domain1 can write to this register"]
    DOMAIN1_1,
}
impl DOMAIN1R {
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
            DOMAIN1R::DOMAIN1_0 => false,
            DOMAIN1R::DOMAIN1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOMAIN1R {
        match value {
            false => DOMAIN1R::DOMAIN1_0,
            true => DOMAIN1R::DOMAIN1_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOMAIN1_0`"]
    #[inline]
    pub fn is_domain1_0(&self) -> bool {
        *self == DOMAIN1R::DOMAIN1_0
    }
    #[doc = "Checks if the value of the field is `DOMAIN1_1`"]
    #[inline]
    pub fn is_domain1_1(&self) -> bool {
        *self == DOMAIN1R::DOMAIN1_1
    }
}
#[doc = "Possible values of the field `DOMAIN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOMAIN2R {
    #[doc = "This register is not assigned to domain2. The master from domain2 cannot write to this register."]
    DOMAIN2_0,
    #[doc = "This register is assigned to domain2. The master from domain2 can write to this register"]
    DOMAIN2_1,
}
impl DOMAIN2R {
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
            DOMAIN2R::DOMAIN2_0 => false,
            DOMAIN2R::DOMAIN2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOMAIN2R {
        match value {
            false => DOMAIN2R::DOMAIN2_0,
            true => DOMAIN2R::DOMAIN2_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOMAIN2_0`"]
    #[inline]
    pub fn is_domain2_0(&self) -> bool {
        *self == DOMAIN2R::DOMAIN2_0
    }
    #[doc = "Checks if the value of the field is `DOMAIN2_1`"]
    #[inline]
    pub fn is_domain2_1(&self) -> bool {
        *self == DOMAIN2R::DOMAIN2_1
    }
}
#[doc = "Possible values of the field `DOMAIN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOMAIN3R {
    #[doc = "This register is not assigned to domain3. The master from domain3 cannot write to this register."]
    DOMAIN3_0,
    #[doc = "This register is assigned to domain3. The master from domain3 can write to this register"]
    DOMAIN3_1,
}
impl DOMAIN3R {
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
            DOMAIN3R::DOMAIN3_0 => false,
            DOMAIN3R::DOMAIN3_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOMAIN3R {
        match value {
            false => DOMAIN3R::DOMAIN3_0,
            true => DOMAIN3R::DOMAIN3_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOMAIN3_0`"]
    #[inline]
    pub fn is_domain3_0(&self) -> bool {
        *self == DOMAIN3R::DOMAIN3_0
    }
    #[doc = "Checks if the value of the field is `DOMAIN3_1`"]
    #[inline]
    pub fn is_domain3_1(&self) -> bool {
        *self == DOMAIN3R::DOMAIN3_1
    }
}
#[doc = "Possible values of the field `LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKR {
    #[doc = "31\\] and \\[27:24\\] bits can be modified"]
    LOCK_0,
    #[doc = "31\\] and \\[27:24\\] bits cannot be modified"]
    LOCK_1,
}
impl LOCKR {
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
            LOCKR::LOCK_0 => false,
            LOCKR::LOCK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCKR {
        match value {
            false => LOCKR::LOCK_0,
            true => LOCKR::LOCK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_0`"]
    #[inline]
    pub fn is_lock_0(&self) -> bool {
        *self == LOCKR::LOCK_0
    }
    #[doc = "Checks if the value of the field is `LOCK_1`"]
    #[inline]
    pub fn is_lock_1(&self) -> bool {
        *self == LOCKR::LOCK_1
    }
}
#[doc = "Possible values of the field `DOM_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOM_ENR {
    #[doc = "Disables domain control. All of this register's bits except \\[31:30\\] and \\[27:24\\] can be modified by any masters"]
    DOM_EN_0,
    #[doc = "Enables domain control. All of this register's bits except \\[31:30\\] and \\[27:24\\] can only be modified by the masters from the domains specified in \\[27:24\\] area."]
    DOM_EN_1,
}
impl DOM_ENR {
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
            DOM_ENR::DOM_EN_0 => false,
            DOM_ENR::DOM_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOM_ENR {
        match value {
            false => DOM_ENR::DOM_EN_0,
            true => DOM_ENR::DOM_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOM_EN_0`"]
    #[inline]
    pub fn is_dom_en_0(&self) -> bool {
        *self == DOM_ENR::DOM_EN_0
    }
    #[doc = "Checks if the value of the field is `DOM_EN_1`"]
    #[inline]
    pub fn is_dom_en_1(&self) -> bool {
        *self == DOM_ENR::DOM_EN_1
    }
}
#[doc = "Values that can be written to the field `A53_CORE1_ENABLE`"]
pub enum A53_CORE1_ENABLEW {
    #[doc = "core1 is disabled"]
    A53_CORE1_ENABLE_0,
    #[doc = "core1 is enabled"]
    A53_CORE1_ENABLE_1,
}
impl A53_CORE1_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            A53_CORE1_ENABLEW::A53_CORE1_ENABLE_0 => false,
            A53_CORE1_ENABLEW::A53_CORE1_ENABLE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _A53_CORE1_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _A53_CORE1_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: A53_CORE1_ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "core1 is disabled"]
    #[inline]
    pub fn a53_core1_enable_0(self) -> &'a mut W {
        self.variant(A53_CORE1_ENABLEW::A53_CORE1_ENABLE_0)
    }
    #[doc = "core1 is enabled"]
    #[inline]
    pub fn a53_core1_enable_1(self) -> &'a mut W {
        self.variant(A53_CORE1_ENABLEW::A53_CORE1_ENABLE_1)
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
#[doc = "Values that can be written to the field `A53_CORE2_ENABLE`"]
pub enum A53_CORE2_ENABLEW {
    #[doc = "core2 is disabled"]
    A53_CORE2_ENABLE_0,
    #[doc = "core2 is enabled"]
    A53_CORE2_ENABLE_1,
}
impl A53_CORE2_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            A53_CORE2_ENABLEW::A53_CORE2_ENABLE_0 => false,
            A53_CORE2_ENABLEW::A53_CORE2_ENABLE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _A53_CORE2_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _A53_CORE2_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: A53_CORE2_ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "core2 is disabled"]
    #[inline]
    pub fn a53_core2_enable_0(self) -> &'a mut W {
        self.variant(A53_CORE2_ENABLEW::A53_CORE2_ENABLE_0)
    }
    #[doc = "core2 is enabled"]
    #[inline]
    pub fn a53_core2_enable_1(self) -> &'a mut W {
        self.variant(A53_CORE2_ENABLEW::A53_CORE2_ENABLE_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `A53_CORE3_ENABLE`"]
pub enum A53_CORE3_ENABLEW {
    #[doc = "core3 is disabled"]
    A53_CORE3_ENABLE_0,
    #[doc = "core3 is enabled"]
    A53_CORE3_ENABLE_1,
}
impl A53_CORE3_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            A53_CORE3_ENABLEW::A53_CORE3_ENABLE_0 => false,
            A53_CORE3_ENABLEW::A53_CORE3_ENABLE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _A53_CORE3_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _A53_CORE3_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: A53_CORE3_ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "core3 is disabled"]
    #[inline]
    pub fn a53_core3_enable_0(self) -> &'a mut W {
        self.variant(A53_CORE3_ENABLEW::A53_CORE3_ENABLE_0)
    }
    #[doc = "core3 is enabled"]
    #[inline]
    pub fn a53_core3_enable_1(self) -> &'a mut W {
        self.variant(A53_CORE3_ENABLEW::A53_CORE3_ENABLE_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _A53_RST_SLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _A53_RST_SLOWW<'a> {
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
#[doc = "Values that can be written to the field `DOMAIN0`"]
pub enum DOMAIN0W {
    #[doc = "This register is not assigned to domain0. The master from domain3 cannot write to this register."]
    DOMAIN0_0,
    #[doc = "This register is assigned to domain0. The master from domain3 can write to this register"]
    DOMAIN0_1,
}
impl DOMAIN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOMAIN0W::DOMAIN0_0 => false,
            DOMAIN0W::DOMAIN0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOMAIN0W<'a> {
    w: &'a mut W,
}
impl<'a> _DOMAIN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOMAIN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This register is not assigned to domain0. The master from domain3 cannot write to this register."]
    #[inline]
    pub fn domain0_0(self) -> &'a mut W {
        self.variant(DOMAIN0W::DOMAIN0_0)
    }
    #[doc = "This register is assigned to domain0. The master from domain3 can write to this register"]
    #[inline]
    pub fn domain0_1(self) -> &'a mut W {
        self.variant(DOMAIN0W::DOMAIN0_1)
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
#[doc = "Values that can be written to the field `DOMAIN1`"]
pub enum DOMAIN1W {
    #[doc = "This register is not assigned to domain1. The master from domain1 cannot write to this register."]
    DOMAIN1_0,
    #[doc = "This register is assigned to domain1. The master from domain1 can write to this register"]
    DOMAIN1_1,
}
impl DOMAIN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOMAIN1W::DOMAIN1_0 => false,
            DOMAIN1W::DOMAIN1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOMAIN1W<'a> {
    w: &'a mut W,
}
impl<'a> _DOMAIN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOMAIN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This register is not assigned to domain1. The master from domain1 cannot write to this register."]
    #[inline]
    pub fn domain1_0(self) -> &'a mut W {
        self.variant(DOMAIN1W::DOMAIN1_0)
    }
    #[doc = "This register is assigned to domain1. The master from domain1 can write to this register"]
    #[inline]
    pub fn domain1_1(self) -> &'a mut W {
        self.variant(DOMAIN1W::DOMAIN1_1)
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
#[doc = "Values that can be written to the field `DOMAIN2`"]
pub enum DOMAIN2W {
    #[doc = "This register is not assigned to domain2. The master from domain2 cannot write to this register."]
    DOMAIN2_0,
    #[doc = "This register is assigned to domain2. The master from domain2 can write to this register"]
    DOMAIN2_1,
}
impl DOMAIN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOMAIN2W::DOMAIN2_0 => false,
            DOMAIN2W::DOMAIN2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOMAIN2W<'a> {
    w: &'a mut W,
}
impl<'a> _DOMAIN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOMAIN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This register is not assigned to domain2. The master from domain2 cannot write to this register."]
    #[inline]
    pub fn domain2_0(self) -> &'a mut W {
        self.variant(DOMAIN2W::DOMAIN2_0)
    }
    #[doc = "This register is assigned to domain2. The master from domain2 can write to this register"]
    #[inline]
    pub fn domain2_1(self) -> &'a mut W {
        self.variant(DOMAIN2W::DOMAIN2_1)
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
#[doc = "Values that can be written to the field `DOMAIN3`"]
pub enum DOMAIN3W {
    #[doc = "This register is not assigned to domain3. The master from domain3 cannot write to this register."]
    DOMAIN3_0,
    #[doc = "This register is assigned to domain3. The master from domain3 can write to this register"]
    DOMAIN3_1,
}
impl DOMAIN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOMAIN3W::DOMAIN3_0 => false,
            DOMAIN3W::DOMAIN3_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOMAIN3W<'a> {
    w: &'a mut W,
}
impl<'a> _DOMAIN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOMAIN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This register is not assigned to domain3. The master from domain3 cannot write to this register."]
    #[inline]
    pub fn domain3_0(self) -> &'a mut W {
        self.variant(DOMAIN3W::DOMAIN3_0)
    }
    #[doc = "This register is assigned to domain3. The master from domain3 can write to this register"]
    #[inline]
    pub fn domain3_1(self) -> &'a mut W {
        self.variant(DOMAIN3W::DOMAIN3_1)
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
#[doc = "Values that can be written to the field `LOCK`"]
pub enum LOCKW {
    #[doc = "31\\] and \\[27:24\\] bits can be modified"]
    LOCK_0,
    #[doc = "31\\] and \\[27:24\\] bits cannot be modified"]
    LOCK_1,
}
impl LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCKW::LOCK_0 => false,
            LOCKW::LOCK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "31\\] and \\[27:24\\] bits can be modified"]
    #[inline]
    pub fn lock_0(self) -> &'a mut W {
        self.variant(LOCKW::LOCK_0)
    }
    #[doc = "31\\] and \\[27:24\\] bits cannot be modified"]
    #[inline]
    pub fn lock_1(self) -> &'a mut W {
        self.variant(LOCKW::LOCK_1)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DOM_EN`"]
pub enum DOM_ENW {
    #[doc = "Disables domain control. All of this register's bits except \\[31:30\\] and \\[27:24\\] can be modified by any masters"]
    DOM_EN_0,
    #[doc = "Enables domain control. All of this register's bits except \\[31:30\\] and \\[27:24\\] can only be modified by the masters from the domains specified in \\[27:24\\] area."]
    DOM_EN_1,
}
impl DOM_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOM_ENW::DOM_EN_0 => false,
            DOM_ENW::DOM_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOM_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DOM_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOM_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables domain control. All of this register's bits except \\[31:30\\] and \\[27:24\\] can be modified by any masters"]
    #[inline]
    pub fn dom_en_0(self) -> &'a mut W {
        self.variant(DOM_ENW::DOM_EN_0)
    }
    #[doc = "Enables domain control. All of this register's bits except \\[31:30\\] and \\[27:24\\] can only be modified by the masters from the domains specified in \\[27:24\\] area."]
    #[inline]
    pub fn dom_en_1(self) -> &'a mut W {
        self.variant(DOM_ENW::DOM_EN_1)
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
    #[doc = "Bit 0 - Always 1, can't be changed."]
    #[inline]
    pub fn a53_core0_enable(&self) -> A53_CORE0_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_CORE0_ENABLER { bits }
    }
    #[doc = "Bit 1 - core 1 enable"]
    #[inline]
    pub fn a53_core1_enable(&self) -> A53_CORE1_ENABLER {
        A53_CORE1_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - core 2 enable"]
    #[inline]
    pub fn a53_core2_enable(&self) -> A53_CORE2_ENABLER {
        A53_CORE2_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - core 3 enable"]
    #[inline]
    pub fn a53_core3_enable(&self) -> A53_CORE3_ENABLER {
        A53_CORE3_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:6 - A53_RST_SLOW"]
    #[inline]
    pub fn a53_rst_slow(&self) -> A53_RST_SLOWR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        A53_RST_SLOWR { bits }
    }
    #[doc = "Bit 24 - Domain0 assignment control. Effective when dom_en is set to 1."]
    #[inline]
    pub fn domain0(&self) -> DOMAIN0R {
        DOMAIN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Domain1 assignment control. Effective when dom_en is set to 1."]
    #[inline]
    pub fn domain1(&self) -> DOMAIN1R {
        DOMAIN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Domain2 assignment control. Effective when dom_en is set to 1."]
    #[inline]
    pub fn domain2(&self) -> DOMAIN2R {
        DOMAIN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Domain3 assignment control. Effective when dom_en is set to 1."]
    #[inline]
    pub fn domain3(&self) -> DOMAIN3R {
        DOMAIN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Domain control bits lock Lock bit is a write-once register, once it is set to 1, it can't be write to 0"]
    #[inline]
    pub fn lock(&self) -> LOCKR {
        LOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Domain Control enable for this register"]
    #[inline]
    pub fn dom_en(&self) -> DOM_ENR {
        DOM_ENR::_from({
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
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - core 1 enable"]
    #[inline]
    pub fn a53_core1_enable(&mut self) -> _A53_CORE1_ENABLEW {
        _A53_CORE1_ENABLEW { w: self }
    }
    #[doc = "Bit 2 - core 2 enable"]
    #[inline]
    pub fn a53_core2_enable(&mut self) -> _A53_CORE2_ENABLEW {
        _A53_CORE2_ENABLEW { w: self }
    }
    #[doc = "Bit 3 - core 3 enable"]
    #[inline]
    pub fn a53_core3_enable(&mut self) -> _A53_CORE3_ENABLEW {
        _A53_CORE3_ENABLEW { w: self }
    }
    #[doc = "Bits 4:6 - A53_RST_SLOW"]
    #[inline]
    pub fn a53_rst_slow(&mut self) -> _A53_RST_SLOWW {
        _A53_RST_SLOWW { w: self }
    }
    #[doc = "Bit 24 - Domain0 assignment control. Effective when dom_en is set to 1."]
    #[inline]
    pub fn domain0(&mut self) -> _DOMAIN0W {
        _DOMAIN0W { w: self }
    }
    #[doc = "Bit 25 - Domain1 assignment control. Effective when dom_en is set to 1."]
    #[inline]
    pub fn domain1(&mut self) -> _DOMAIN1W {
        _DOMAIN1W { w: self }
    }
    #[doc = "Bit 26 - Domain2 assignment control. Effective when dom_en is set to 1."]
    #[inline]
    pub fn domain2(&mut self) -> _DOMAIN2W {
        _DOMAIN2W { w: self }
    }
    #[doc = "Bit 27 - Domain3 assignment control. Effective when dom_en is set to 1."]
    #[inline]
    pub fn domain3(&mut self) -> _DOMAIN3W {
        _DOMAIN3W { w: self }
    }
    #[doc = "Bit 30 - Domain control bits lock Lock bit is a write-once register, once it is set to 1, it can't be write to 0"]
    #[inline]
    pub fn lock(&mut self) -> _LOCKW {
        _LOCKW { w: self }
    }
    #[doc = "Bit 31 - Domain Control enable for this register"]
    #[inline]
    pub fn dom_en(&mut self) -> _DOM_ENW {
        _DOM_ENW { w: self }
    }
}
