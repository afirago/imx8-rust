#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::POST_ROOT124_SET {
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
#[doc = "Possible values of the field `POST_PODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POST_PODFR {
    #[doc = "Divide by 1"]
    POST_PODF_0,
    #[doc = "Divide by 2"]
    POST_PODF_1,
    #[doc = "Divide by 3"]
    POST_PODF_2,
    #[doc = "Divide by 4"]
    POST_PODF_3,
    #[doc = "Divide by 5"]
    POST_PODF_4,
    #[doc = "Divide by 6"]
    POST_PODF_5,
    #[doc = "Divide by 64"]
    POST_PODF_63,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl POST_PODFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            POST_PODFR::POST_PODF_0 => 0,
            POST_PODFR::POST_PODF_1 => 1,
            POST_PODFR::POST_PODF_2 => 2,
            POST_PODFR::POST_PODF_3 => 3,
            POST_PODFR::POST_PODF_4 => 4,
            POST_PODFR::POST_PODF_5 => 5,
            POST_PODFR::POST_PODF_63 => 63,
            POST_PODFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> POST_PODFR {
        match value {
            0 => POST_PODFR::POST_PODF_0,
            1 => POST_PODFR::POST_PODF_1,
            2 => POST_PODFR::POST_PODF_2,
            3 => POST_PODFR::POST_PODF_3,
            4 => POST_PODFR::POST_PODF_4,
            5 => POST_PODFR::POST_PODF_5,
            63 => POST_PODFR::POST_PODF_63,
            i => POST_PODFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `POST_PODF_0`"]
    #[inline]
    pub fn is_post_podf_0(&self) -> bool {
        *self == POST_PODFR::POST_PODF_0
    }
    #[doc = "Checks if the value of the field is `POST_PODF_1`"]
    #[inline]
    pub fn is_post_podf_1(&self) -> bool {
        *self == POST_PODFR::POST_PODF_1
    }
    #[doc = "Checks if the value of the field is `POST_PODF_2`"]
    #[inline]
    pub fn is_post_podf_2(&self) -> bool {
        *self == POST_PODFR::POST_PODF_2
    }
    #[doc = "Checks if the value of the field is `POST_PODF_3`"]
    #[inline]
    pub fn is_post_podf_3(&self) -> bool {
        *self == POST_PODFR::POST_PODF_3
    }
    #[doc = "Checks if the value of the field is `POST_PODF_4`"]
    #[inline]
    pub fn is_post_podf_4(&self) -> bool {
        *self == POST_PODFR::POST_PODF_4
    }
    #[doc = "Checks if the value of the field is `POST_PODF_5`"]
    #[inline]
    pub fn is_post_podf_5(&self) -> bool {
        *self == POST_PODFR::POST_PODF_5
    }
    #[doc = "Checks if the value of the field is `POST_PODF_63`"]
    #[inline]
    pub fn is_post_podf_63(&self) -> bool {
        *self == POST_PODFR::POST_PODF_63
    }
}
#[doc = r" Value of the field"]
pub struct BUSY1R {
    bits: bool,
}
impl BUSY1R {
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
#[doc = "Possible values of the field `SELECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELECTR {
    #[doc = "select branch A"]
    SELECT_0,
    #[doc = "select branch B"]
    SELECT_1,
}
impl SELECTR {
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
            SELECTR::SELECT_0 => false,
            SELECTR::SELECT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELECTR {
        match value {
            false => SELECTR::SELECT_0,
            true => SELECTR::SELECT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SELECT_0`"]
    #[inline]
    pub fn is_select_0(&self) -> bool {
        *self == SELECTR::SELECT_0
    }
    #[doc = "Checks if the value of the field is `SELECT_1`"]
    #[inline]
    pub fn is_select_1(&self) -> bool {
        *self == SELECTR::SELECT_1
    }
}
#[doc = r" Value of the field"]
pub struct BUSY2R {
    bits: bool,
}
impl BUSY2R {
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
#[doc = "Values that can be written to the field `POST_PODF`"]
pub enum POST_PODFW {
    #[doc = "Divide by 1"]
    POST_PODF_0,
    #[doc = "Divide by 2"]
    POST_PODF_1,
    #[doc = "Divide by 3"]
    POST_PODF_2,
    #[doc = "Divide by 4"]
    POST_PODF_3,
    #[doc = "Divide by 5"]
    POST_PODF_4,
    #[doc = "Divide by 6"]
    POST_PODF_5,
    #[doc = "Divide by 64"]
    POST_PODF_63,
}
impl POST_PODFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            POST_PODFW::POST_PODF_0 => 0,
            POST_PODFW::POST_PODF_1 => 1,
            POST_PODFW::POST_PODF_2 => 2,
            POST_PODFW::POST_PODF_3 => 3,
            POST_PODFW::POST_PODF_4 => 4,
            POST_PODFW::POST_PODF_5 => 5,
            POST_PODFW::POST_PODF_63 => 63,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POST_PODFW<'a> {
    w: &'a mut W,
}
impl<'a> _POST_PODFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POST_PODFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn post_podf_0(self) -> &'a mut W {
        self.variant(POST_PODFW::POST_PODF_0)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn post_podf_1(self) -> &'a mut W {
        self.variant(POST_PODFW::POST_PODF_1)
    }
    #[doc = "Divide by 3"]
    #[inline]
    pub fn post_podf_2(self) -> &'a mut W {
        self.variant(POST_PODFW::POST_PODF_2)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn post_podf_3(self) -> &'a mut W {
        self.variant(POST_PODFW::POST_PODF_3)
    }
    #[doc = "Divide by 5"]
    #[inline]
    pub fn post_podf_4(self) -> &'a mut W {
        self.variant(POST_PODFW::POST_PODF_4)
    }
    #[doc = "Divide by 6"]
    #[inline]
    pub fn post_podf_5(self) -> &'a mut W {
        self.variant(POST_PODFW::POST_PODF_5)
    }
    #[doc = "Divide by 64"]
    #[inline]
    pub fn post_podf_63(self) -> &'a mut W {
        self.variant(POST_PODFW::POST_PODF_63)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SELECT`"]
pub enum SELECTW {
    #[doc = "select branch A"]
    SELECT_0,
    #[doc = "select branch B"]
    SELECT_1,
}
impl SELECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELECTW::SELECT_0 => false,
            SELECTW::SELECT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _SELECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELECTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "select branch A"]
    #[inline]
    pub fn select_0(self) -> &'a mut W {
        self.variant(SELECTW::SELECT_0)
    }
    #[doc = "select branch B"]
    #[inline]
    pub fn select_1(self) -> &'a mut W {
        self.variant(SELECTW::SELECT_1)
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
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:5 - Post divider divide number Divider value is n + 1 For CORE, this field is 3 bit long"]
    #[inline]
    pub fn post_podf(&self) -> POST_PODFR {
        POST_PODFR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Post divider is applying new set value"]
    #[inline]
    pub fn busy1(&self) -> BUSY1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BUSY1R { bits }
    }
    #[doc = "Bit 28 - Selection of pre clock branches This field is not applied in IP"]
    #[inline]
    pub fn select(&self) -> SELECTR {
        SELECTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Clock switching multiplexer is applying new setting"]
    #[inline]
    pub fn busy2(&self) -> BUSY2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BUSY2R { bits }
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
    #[doc = "Bits 0:5 - Post divider divide number Divider value is n + 1 For CORE, this field is 3 bit long"]
    #[inline]
    pub fn post_podf(&mut self) -> _POST_PODFW {
        _POST_PODFW { w: self }
    }
    #[doc = "Bit 28 - Selection of pre clock branches This field is not applied in IP"]
    #[inline]
    pub fn select(&mut self) -> _SELECTW {
        _SELECTW { w: self }
    }
}
