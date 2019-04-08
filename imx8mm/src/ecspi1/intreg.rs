#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTREG {
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
#[doc = "Possible values of the field `TEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEENR {
    #[doc = "Disable"]
    TEEN_0,
    #[doc = "Enable"]
    TEEN_1,
}
impl TEENR {
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
            TEENR::TEEN_0 => false,
            TEENR::TEEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEENR {
        match value {
            false => TEENR::TEEN_0,
            true => TEENR::TEEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TEEN_0`"]
    #[inline]
    pub fn is_teen_0(&self) -> bool {
        *self == TEENR::TEEN_0
    }
    #[doc = "Checks if the value of the field is `TEEN_1`"]
    #[inline]
    pub fn is_teen_1(&self) -> bool {
        *self == TEENR::TEEN_1
    }
}
#[doc = "Possible values of the field `TDREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDRENR {
    #[doc = "Disable"]
    TDREN_0,
    #[doc = "Enable"]
    TDREN_1,
}
impl TDRENR {
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
            TDRENR::TDREN_0 => false,
            TDRENR::TDREN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDRENR {
        match value {
            false => TDRENR::TDREN_0,
            true => TDRENR::TDREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TDREN_0`"]
    #[inline]
    pub fn is_tdren_0(&self) -> bool {
        *self == TDRENR::TDREN_0
    }
    #[doc = "Checks if the value of the field is `TDREN_1`"]
    #[inline]
    pub fn is_tdren_1(&self) -> bool {
        *self == TDRENR::TDREN_1
    }
}
#[doc = "Possible values of the field `TFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFENR {
    #[doc = "Disable"]
    TFEN_0,
    #[doc = "Enable"]
    TFEN_1,
}
impl TFENR {
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
            TFENR::TFEN_0 => false,
            TFENR::TFEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TFENR {
        match value {
            false => TFENR::TFEN_0,
            true => TFENR::TFEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TFEN_0`"]
    #[inline]
    pub fn is_tfen_0(&self) -> bool {
        *self == TFENR::TFEN_0
    }
    #[doc = "Checks if the value of the field is `TFEN_1`"]
    #[inline]
    pub fn is_tfen_1(&self) -> bool {
        *self == TFENR::TFEN_1
    }
}
#[doc = "Possible values of the field `RREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRENR {
    #[doc = "Disable"]
    RREN_0,
    #[doc = "Enable"]
    RREN_1,
}
impl RRENR {
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
            RRENR::RREN_0 => false,
            RRENR::RREN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RRENR {
        match value {
            false => RRENR::RREN_0,
            true => RRENR::RREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RREN_0`"]
    #[inline]
    pub fn is_rren_0(&self) -> bool {
        *self == RRENR::RREN_0
    }
    #[doc = "Checks if the value of the field is `RREN_1`"]
    #[inline]
    pub fn is_rren_1(&self) -> bool {
        *self == RRENR::RREN_1
    }
}
#[doc = "Possible values of the field `RDREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDRENR {
    #[doc = "Disable"]
    RDREN_0,
    #[doc = "Enable"]
    RDREN_1,
}
impl RDRENR {
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
            RDRENR::RDREN_0 => false,
            RDRENR::RDREN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDRENR {
        match value {
            false => RDRENR::RDREN_0,
            true => RDRENR::RDREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RDREN_0`"]
    #[inline]
    pub fn is_rdren_0(&self) -> bool {
        *self == RDRENR::RDREN_0
    }
    #[doc = "Checks if the value of the field is `RDREN_1`"]
    #[inline]
    pub fn is_rdren_1(&self) -> bool {
        *self == RDRENR::RDREN_1
    }
}
#[doc = "Possible values of the field `RFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFENR {
    #[doc = "Disable"]
    RFEN_0,
    #[doc = "Enable"]
    RFEN_1,
}
impl RFENR {
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
            RFENR::RFEN_0 => false,
            RFENR::RFEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RFENR {
        match value {
            false => RFENR::RFEN_0,
            true => RFENR::RFEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RFEN_0`"]
    #[inline]
    pub fn is_rfen_0(&self) -> bool {
        *self == RFENR::RFEN_0
    }
    #[doc = "Checks if the value of the field is `RFEN_1`"]
    #[inline]
    pub fn is_rfen_1(&self) -> bool {
        *self == RFENR::RFEN_1
    }
}
#[doc = "Possible values of the field `ROEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROENR {
    #[doc = "Disable"]
    ROEN_0,
    #[doc = "Enable"]
    ROEN_1,
}
impl ROENR {
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
            ROENR::ROEN_0 => false,
            ROENR::ROEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ROENR {
        match value {
            false => ROENR::ROEN_0,
            true => ROENR::ROEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ROEN_0`"]
    #[inline]
    pub fn is_roen_0(&self) -> bool {
        *self == ROENR::ROEN_0
    }
    #[doc = "Checks if the value of the field is `ROEN_1`"]
    #[inline]
    pub fn is_roen_1(&self) -> bool {
        *self == ROENR::ROEN_1
    }
}
#[doc = "Possible values of the field `TCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCENR {
    #[doc = "Disable"]
    TCEN_0,
    #[doc = "Enable"]
    TCEN_1,
}
impl TCENR {
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
            TCENR::TCEN_0 => false,
            TCENR::TCEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCENR {
        match value {
            false => TCENR::TCEN_0,
            true => TCENR::TCEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCEN_0`"]
    #[inline]
    pub fn is_tcen_0(&self) -> bool {
        *self == TCENR::TCEN_0
    }
    #[doc = "Checks if the value of the field is `TCEN_1`"]
    #[inline]
    pub fn is_tcen_1(&self) -> bool {
        *self == TCENR::TCEN_1
    }
}
#[doc = "Values that can be written to the field `TEEN`"]
pub enum TEENW {
    #[doc = "Disable"]
    TEEN_0,
    #[doc = "Enable"]
    TEEN_1,
}
impl TEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TEENW::TEEN_0 => false,
            TEENW::TEEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TEENW<'a> {
    w: &'a mut W,
}
impl<'a> _TEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn teen_0(self) -> &'a mut W {
        self.variant(TEENW::TEEN_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn teen_1(self) -> &'a mut W {
        self.variant(TEENW::TEEN_1)
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
#[doc = "Values that can be written to the field `TDREN`"]
pub enum TDRENW {
    #[doc = "Disable"]
    TDREN_0,
    #[doc = "Enable"]
    TDREN_1,
}
impl TDRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TDRENW::TDREN_0 => false,
            TDRENW::TDREN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDRENW<'a> {
    w: &'a mut W,
}
impl<'a> _TDRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn tdren_0(self) -> &'a mut W {
        self.variant(TDRENW::TDREN_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn tdren_1(self) -> &'a mut W {
        self.variant(TDRENW::TDREN_1)
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
#[doc = "Values that can be written to the field `TFEN`"]
pub enum TFENW {
    #[doc = "Disable"]
    TFEN_0,
    #[doc = "Enable"]
    TFEN_1,
}
impl TFENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TFENW::TFEN_0 => false,
            TFENW::TFEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TFENW<'a> {
    w: &'a mut W,
}
impl<'a> _TFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn tfen_0(self) -> &'a mut W {
        self.variant(TFENW::TFEN_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn tfen_1(self) -> &'a mut W {
        self.variant(TFENW::TFEN_1)
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
#[doc = "Values that can be written to the field `RREN`"]
pub enum RRENW {
    #[doc = "Disable"]
    RREN_0,
    #[doc = "Enable"]
    RREN_1,
}
impl RRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RRENW::RREN_0 => false,
            RRENW::RREN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RRENW<'a> {
    w: &'a mut W,
}
impl<'a> _RRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn rren_0(self) -> &'a mut W {
        self.variant(RRENW::RREN_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn rren_1(self) -> &'a mut W {
        self.variant(RRENW::RREN_1)
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
#[doc = "Values that can be written to the field `RDREN`"]
pub enum RDRENW {
    #[doc = "Disable"]
    RDREN_0,
    #[doc = "Enable"]
    RDREN_1,
}
impl RDRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RDRENW::RDREN_0 => false,
            RDRENW::RDREN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDRENW<'a> {
    w: &'a mut W,
}
impl<'a> _RDRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn rdren_0(self) -> &'a mut W {
        self.variant(RDRENW::RDREN_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn rdren_1(self) -> &'a mut W {
        self.variant(RDRENW::RDREN_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RFEN`"]
pub enum RFENW {
    #[doc = "Disable"]
    RFEN_0,
    #[doc = "Enable"]
    RFEN_1,
}
impl RFENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RFENW::RFEN_0 => false,
            RFENW::RFEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RFENW<'a> {
    w: &'a mut W,
}
impl<'a> _RFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn rfen_0(self) -> &'a mut W {
        self.variant(RFENW::RFEN_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn rfen_1(self) -> &'a mut W {
        self.variant(RFENW::RFEN_1)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ROEN`"]
pub enum ROENW {
    #[doc = "Disable"]
    ROEN_0,
    #[doc = "Enable"]
    ROEN_1,
}
impl ROENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ROENW::ROEN_0 => false,
            ROENW::ROEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ROENW<'a> {
    w: &'a mut W,
}
impl<'a> _ROENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ROENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn roen_0(self) -> &'a mut W {
        self.variant(ROENW::ROEN_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn roen_1(self) -> &'a mut W {
        self.variant(ROENW::ROEN_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TCEN`"]
pub enum TCENW {
    #[doc = "Disable"]
    TCEN_0,
    #[doc = "Enable"]
    TCEN_1,
}
impl TCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCENW::TCEN_0 => false,
            TCENW::TCEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCENW<'a> {
    w: &'a mut W,
}
impl<'a> _TCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn tcen_0(self) -> &'a mut W {
        self.variant(TCENW::TCEN_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn tcen_1(self) -> &'a mut W {
        self.variant(TCENW::TCEN_1)
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
        const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - TXFIFO Empty Interrupt enable. This bit enables the TXFIFO Empty Interrupt."]
    #[inline]
    pub fn teen(&self) -> TEENR {
        TEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - TXFIFO Data Request Interrupt enable"]
    #[inline]
    pub fn tdren(&self) -> TDRENR {
        TDRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - TXFIFO Full Interrupt enable. This bit enables the TXFIFO Full Interrupt."]
    #[inline]
    pub fn tfen(&self) -> TFENR {
        TFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - RXFIFO Ready Interrupt enable. This bit enables the RXFIFO Ready Interrupt."]
    #[inline]
    pub fn rren(&self) -> RRENR {
        RRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - RXFIFO Data Request Interrupt enable"]
    #[inline]
    pub fn rdren(&self) -> RDRENR {
        RDRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - RXFIFO Full Interrupt enable. This bit enables the RXFIFO Full Interrupt."]
    #[inline]
    pub fn rfen(&self) -> RFENR {
        RFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - RXFIFO Overflow Interrupt enable. This bit enables the RXFIFO Overflow Interrupt."]
    #[inline]
    pub fn roen(&self) -> ROENR {
        ROENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Transfer Completed Interrupt enable. This bit enables the Transfer Completed Interrupt."]
    #[inline]
    pub fn tcen(&self) -> TCENR {
        TCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - TXFIFO Empty Interrupt enable. This bit enables the TXFIFO Empty Interrupt."]
    #[inline]
    pub fn teen(&mut self) -> _TEENW {
        _TEENW { w: self }
    }
    #[doc = "Bit 1 - TXFIFO Data Request Interrupt enable"]
    #[inline]
    pub fn tdren(&mut self) -> _TDRENW {
        _TDRENW { w: self }
    }
    #[doc = "Bit 2 - TXFIFO Full Interrupt enable. This bit enables the TXFIFO Full Interrupt."]
    #[inline]
    pub fn tfen(&mut self) -> _TFENW {
        _TFENW { w: self }
    }
    #[doc = "Bit 3 - RXFIFO Ready Interrupt enable. This bit enables the RXFIFO Ready Interrupt."]
    #[inline]
    pub fn rren(&mut self) -> _RRENW {
        _RRENW { w: self }
    }
    #[doc = "Bit 4 - RXFIFO Data Request Interrupt enable"]
    #[inline]
    pub fn rdren(&mut self) -> _RDRENW {
        _RDRENW { w: self }
    }
    #[doc = "Bit 5 - RXFIFO Full Interrupt enable. This bit enables the RXFIFO Full Interrupt."]
    #[inline]
    pub fn rfen(&mut self) -> _RFENW {
        _RFENW { w: self }
    }
    #[doc = "Bit 6 - RXFIFO Overflow Interrupt enable. This bit enables the RXFIFO Overflow Interrupt."]
    #[inline]
    pub fn roen(&mut self) -> _ROENW {
        _ROENW { w: self }
    }
    #[doc = "Bit 7 - Transfer Completed Interrupt enable. This bit enables the Transfer Completed Interrupt."]
    #[inline]
    pub fn tcen(&mut self) -> _TCENW {
        _TCENW { w: self }
    }
}
