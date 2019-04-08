#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MRC {
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
#[doc = "Possible values of the field `D0W`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D0WR {
    #[doc = "Processing Domain 0 does not have Write access to the memory region"]
    D0W_0,
    #[doc = "Processing Domain 0 has Write access to the memory region"]
    D0W_1,
}
impl D0WR {
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
            D0WR::D0W_0 => false,
            D0WR::D0W_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> D0WR {
        match value {
            false => D0WR::D0W_0,
            true => D0WR::D0W_1,
        }
    }
    #[doc = "Checks if the value of the field is `D0W_0`"]
    #[inline]
    pub fn is_d0w_0(&self) -> bool {
        *self == D0WR::D0W_0
    }
    #[doc = "Checks if the value of the field is `D0W_1`"]
    #[inline]
    pub fn is_d0w_1(&self) -> bool {
        *self == D0WR::D0W_1
    }
}
#[doc = "Possible values of the field `D0R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D0RR {
    #[doc = "Processing Domain 0 does not have Read access to the memory region"]
    D0R_0,
    #[doc = "Processing Domain 0 has Read access to the memory region"]
    D0R_1,
}
impl D0RR {
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
            D0RR::D0R_0 => false,
            D0RR::D0R_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> D0RR {
        match value {
            false => D0RR::D0R_0,
            true => D0RR::D0R_1,
        }
    }
    #[doc = "Checks if the value of the field is `D0R_0`"]
    #[inline]
    pub fn is_d0r_0(&self) -> bool {
        *self == D0RR::D0R_0
    }
    #[doc = "Checks if the value of the field is `D0R_1`"]
    #[inline]
    pub fn is_d0r_1(&self) -> bool {
        *self == D0RR::D0R_1
    }
}
#[doc = "Possible values of the field `D1W`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D1WR {
    #[doc = "Processing Domain 1 does not have Write access to the memory region"]
    D1W_0,
    #[doc = "Processing Domain 1 has Write access to the memory region"]
    D1W_1,
}
impl D1WR {
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
            D1WR::D1W_0 => false,
            D1WR::D1W_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> D1WR {
        match value {
            false => D1WR::D1W_0,
            true => D1WR::D1W_1,
        }
    }
    #[doc = "Checks if the value of the field is `D1W_0`"]
    #[inline]
    pub fn is_d1w_0(&self) -> bool {
        *self == D1WR::D1W_0
    }
    #[doc = "Checks if the value of the field is `D1W_1`"]
    #[inline]
    pub fn is_d1w_1(&self) -> bool {
        *self == D1WR::D1W_1
    }
}
#[doc = "Possible values of the field `D1R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D1RR {
    #[doc = "Processing Domain 1 does not have Read access to the memory region"]
    D1R_0,
    #[doc = "Processing Domain 1 has Read access to the memory region"]
    D1R_1,
}
impl D1RR {
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
            D1RR::D1R_0 => false,
            D1RR::D1R_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> D1RR {
        match value {
            false => D1RR::D1R_0,
            true => D1RR::D1R_1,
        }
    }
    #[doc = "Checks if the value of the field is `D1R_0`"]
    #[inline]
    pub fn is_d1r_0(&self) -> bool {
        *self == D1RR::D1R_0
    }
    #[doc = "Checks if the value of the field is `D1R_1`"]
    #[inline]
    pub fn is_d1r_1(&self) -> bool {
        *self == D1RR::D1R_1
    }
}
#[doc = "Possible values of the field `D2W`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D2WR {
    #[doc = "Processing Domain 2 does not have Write access to the memory region"]
    D2W_0,
    #[doc = "Processing Domain 2 has Write access to the memory region"]
    D2W_1,
}
impl D2WR {
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
            D2WR::D2W_0 => false,
            D2WR::D2W_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> D2WR {
        match value {
            false => D2WR::D2W_0,
            true => D2WR::D2W_1,
        }
    }
    #[doc = "Checks if the value of the field is `D2W_0`"]
    #[inline]
    pub fn is_d2w_0(&self) -> bool {
        *self == D2WR::D2W_0
    }
    #[doc = "Checks if the value of the field is `D2W_1`"]
    #[inline]
    pub fn is_d2w_1(&self) -> bool {
        *self == D2WR::D2W_1
    }
}
#[doc = "Possible values of the field `D2R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D2RR {
    #[doc = "Processing Domain 2 does not have Read access to the memory region"]
    D2R_0,
    #[doc = "Processing Domain 2 has Read access to the memory region"]
    D2R_1,
}
impl D2RR {
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
            D2RR::D2R_0 => false,
            D2RR::D2R_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> D2RR {
        match value {
            false => D2RR::D2R_0,
            true => D2RR::D2R_1,
        }
    }
    #[doc = "Checks if the value of the field is `D2R_0`"]
    #[inline]
    pub fn is_d2r_0(&self) -> bool {
        *self == D2RR::D2R_0
    }
    #[doc = "Checks if the value of the field is `D2R_1`"]
    #[inline]
    pub fn is_d2r_1(&self) -> bool {
        *self == D2RR::D2R_1
    }
}
#[doc = "Possible values of the field `D3W`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D3WR {
    #[doc = "Processing Domain 3 does not have Write access to the memory region"]
    D3W_0,
    #[doc = "Processing Domain 3 has Read access to the memory region"]
    D3W_1,
}
impl D3WR {
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
            D3WR::D3W_0 => false,
            D3WR::D3W_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> D3WR {
        match value {
            false => D3WR::D3W_0,
            true => D3WR::D3W_1,
        }
    }
    #[doc = "Checks if the value of the field is `D3W_0`"]
    #[inline]
    pub fn is_d3w_0(&self) -> bool {
        *self == D3WR::D3W_0
    }
    #[doc = "Checks if the value of the field is `D3W_1`"]
    #[inline]
    pub fn is_d3w_1(&self) -> bool {
        *self == D3WR::D3W_1
    }
}
#[doc = "Possible values of the field `D3R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D3RR {
    #[doc = "Processing Domain 3 does not have Read access to the memory region"]
    D3R_0,
    #[doc = "Processing Domain 3 has Read access to the memory region"]
    D3R_1,
}
impl D3RR {
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
            D3RR::D3R_0 => false,
            D3RR::D3R_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> D3RR {
        match value {
            false => D3RR::D3R_0,
            true => D3RR::D3R_1,
        }
    }
    #[doc = "Checks if the value of the field is `D3R_0`"]
    #[inline]
    pub fn is_d3r_0(&self) -> bool {
        *self == D3RR::D3R_0
    }
    #[doc = "Checks if the value of the field is `D3R_1`"]
    #[inline]
    pub fn is_d3r_1(&self) -> bool {
        *self == D3RR::D3R_1
    }
}
#[doc = "Possible values of the field `ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENAR {
    #[doc = "Memory region is not defined or restricted."]
    ENA_0,
    #[doc = "Memory boundaries, domain permissions and controls are in effect."]
    ENA_1,
}
impl ENAR {
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
            ENAR::ENA_0 => false,
            ENAR::ENA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENAR {
        match value {
            false => ENAR::ENA_0,
            true => ENAR::ENA_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENA_0`"]
    #[inline]
    pub fn is_ena_0(&self) -> bool {
        *self == ENAR::ENA_0
    }
    #[doc = "Checks if the value of the field is `ENA_1`"]
    #[inline]
    pub fn is_ena_1(&self) -> bool {
        *self == ENAR::ENA_1
    }
}
#[doc = "Possible values of the field `LCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCKR {
    #[doc = "No Lock. All fields in this register may be modified."]
    LCK_0,
    #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
    LCK_1,
}
impl LCKR {
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
            LCKR::LCK_0 => false,
            LCKR::LCK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LCKR {
        match value {
            false => LCKR::LCK_0,
            true => LCKR::LCK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCK_0`"]
    #[inline]
    pub fn is_lck_0(&self) -> bool {
        *self == LCKR::LCK_0
    }
    #[doc = "Checks if the value of the field is `LCK_1`"]
    #[inline]
    pub fn is_lck_1(&self) -> bool {
        *self == LCKR::LCK_1
    }
}
#[doc = "Values that can be written to the field `D0W`"]
pub enum D0WW {
    #[doc = "Processing Domain 0 does not have Write access to the memory region"]
    D0W_0,
    #[doc = "Processing Domain 0 has Write access to the memory region"]
    D0W_1,
}
impl D0WW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            D0WW::D0W_0 => false,
            D0WW::D0W_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _D0WW<'a> {
    w: &'a mut W,
}
impl<'a> _D0WW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: D0WW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Processing Domain 0 does not have Write access to the memory region"]
    #[inline]
    pub fn d0w_0(self) -> &'a mut W {
        self.variant(D0WW::D0W_0)
    }
    #[doc = "Processing Domain 0 has Write access to the memory region"]
    #[inline]
    pub fn d0w_1(self) -> &'a mut W {
        self.variant(D0WW::D0W_1)
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
#[doc = "Values that can be written to the field `D0R`"]
pub enum D0RW {
    #[doc = "Processing Domain 0 does not have Read access to the memory region"]
    D0R_0,
    #[doc = "Processing Domain 0 has Read access to the memory region"]
    D0R_1,
}
impl D0RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            D0RW::D0R_0 => false,
            D0RW::D0R_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _D0RW<'a> {
    w: &'a mut W,
}
impl<'a> _D0RW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: D0RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Processing Domain 0 does not have Read access to the memory region"]
    #[inline]
    pub fn d0r_0(self) -> &'a mut W {
        self.variant(D0RW::D0R_0)
    }
    #[doc = "Processing Domain 0 has Read access to the memory region"]
    #[inline]
    pub fn d0r_1(self) -> &'a mut W {
        self.variant(D0RW::D0R_1)
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
#[doc = "Values that can be written to the field `D1W`"]
pub enum D1WW {
    #[doc = "Processing Domain 1 does not have Write access to the memory region"]
    D1W_0,
    #[doc = "Processing Domain 1 has Write access to the memory region"]
    D1W_1,
}
impl D1WW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            D1WW::D1W_0 => false,
            D1WW::D1W_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _D1WW<'a> {
    w: &'a mut W,
}
impl<'a> _D1WW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: D1WW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Processing Domain 1 does not have Write access to the memory region"]
    #[inline]
    pub fn d1w_0(self) -> &'a mut W {
        self.variant(D1WW::D1W_0)
    }
    #[doc = "Processing Domain 1 has Write access to the memory region"]
    #[inline]
    pub fn d1w_1(self) -> &'a mut W {
        self.variant(D1WW::D1W_1)
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
#[doc = "Values that can be written to the field `D1R`"]
pub enum D1RW {
    #[doc = "Processing Domain 1 does not have Read access to the memory region"]
    D1R_0,
    #[doc = "Processing Domain 1 has Read access to the memory region"]
    D1R_1,
}
impl D1RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            D1RW::D1R_0 => false,
            D1RW::D1R_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _D1RW<'a> {
    w: &'a mut W,
}
impl<'a> _D1RW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: D1RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Processing Domain 1 does not have Read access to the memory region"]
    #[inline]
    pub fn d1r_0(self) -> &'a mut W {
        self.variant(D1RW::D1R_0)
    }
    #[doc = "Processing Domain 1 has Read access to the memory region"]
    #[inline]
    pub fn d1r_1(self) -> &'a mut W {
        self.variant(D1RW::D1R_1)
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
#[doc = "Values that can be written to the field `D2W`"]
pub enum D2WW {
    #[doc = "Processing Domain 2 does not have Write access to the memory region"]
    D2W_0,
    #[doc = "Processing Domain 2 has Write access to the memory region"]
    D2W_1,
}
impl D2WW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            D2WW::D2W_0 => false,
            D2WW::D2W_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _D2WW<'a> {
    w: &'a mut W,
}
impl<'a> _D2WW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: D2WW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Processing Domain 2 does not have Write access to the memory region"]
    #[inline]
    pub fn d2w_0(self) -> &'a mut W {
        self.variant(D2WW::D2W_0)
    }
    #[doc = "Processing Domain 2 has Write access to the memory region"]
    #[inline]
    pub fn d2w_1(self) -> &'a mut W {
        self.variant(D2WW::D2W_1)
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
#[doc = "Values that can be written to the field `D2R`"]
pub enum D2RW {
    #[doc = "Processing Domain 2 does not have Read access to the memory region"]
    D2R_0,
    #[doc = "Processing Domain 2 has Read access to the memory region"]
    D2R_1,
}
impl D2RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            D2RW::D2R_0 => false,
            D2RW::D2R_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _D2RW<'a> {
    w: &'a mut W,
}
impl<'a> _D2RW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: D2RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Processing Domain 2 does not have Read access to the memory region"]
    #[inline]
    pub fn d2r_0(self) -> &'a mut W {
        self.variant(D2RW::D2R_0)
    }
    #[doc = "Processing Domain 2 has Read access to the memory region"]
    #[inline]
    pub fn d2r_1(self) -> &'a mut W {
        self.variant(D2RW::D2R_1)
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
#[doc = "Values that can be written to the field `D3W`"]
pub enum D3WW {
    #[doc = "Processing Domain 3 does not have Write access to the memory region"]
    D3W_0,
    #[doc = "Processing Domain 3 has Read access to the memory region"]
    D3W_1,
}
impl D3WW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            D3WW::D3W_0 => false,
            D3WW::D3W_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _D3WW<'a> {
    w: &'a mut W,
}
impl<'a> _D3WW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: D3WW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Processing Domain 3 does not have Write access to the memory region"]
    #[inline]
    pub fn d3w_0(self) -> &'a mut W {
        self.variant(D3WW::D3W_0)
    }
    #[doc = "Processing Domain 3 has Read access to the memory region"]
    #[inline]
    pub fn d3w_1(self) -> &'a mut W {
        self.variant(D3WW::D3W_1)
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
#[doc = "Values that can be written to the field `D3R`"]
pub enum D3RW {
    #[doc = "Processing Domain 3 does not have Read access to the memory region"]
    D3R_0,
    #[doc = "Processing Domain 3 has Read access to the memory region"]
    D3R_1,
}
impl D3RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            D3RW::D3R_0 => false,
            D3RW::D3R_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _D3RW<'a> {
    w: &'a mut W,
}
impl<'a> _D3RW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: D3RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Processing Domain 3 does not have Read access to the memory region"]
    #[inline]
    pub fn d3r_0(self) -> &'a mut W {
        self.variant(D3RW::D3R_0)
    }
    #[doc = "Processing Domain 3 has Read access to the memory region"]
    #[inline]
    pub fn d3r_1(self) -> &'a mut W {
        self.variant(D3RW::D3R_1)
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
#[doc = "Values that can be written to the field `ENA`"]
pub enum ENAW {
    #[doc = "Memory region is not defined or restricted."]
    ENA_0,
    #[doc = "Memory boundaries, domain permissions and controls are in effect."]
    ENA_1,
}
impl ENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENAW::ENA_0 => false,
            ENAW::ENA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _ENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Memory region is not defined or restricted."]
    #[inline]
    pub fn ena_0(self) -> &'a mut W {
        self.variant(ENAW::ENA_0)
    }
    #[doc = "Memory boundaries, domain permissions and controls are in effect."]
    #[inline]
    pub fn ena_1(self) -> &'a mut W {
        self.variant(ENAW::ENA_1)
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
#[doc = "Values that can be written to the field `LCK`"]
pub enum LCKW {
    #[doc = "No Lock. All fields in this register may be modified."]
    LCK_0,
    #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
    LCK_1,
}
impl LCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LCKW::LCK_0 => false,
            LCKW::LCK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LCKW<'a> {
    w: &'a mut W,
}
impl<'a> _LCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LCKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Lock. All fields in this register may be modified."]
    #[inline]
    pub fn lck_0(self) -> &'a mut W {
        self.variant(LCKW::LCK_0)
    }
    #[doc = "Locked. No fields in this register may be modified except ENA, which may be set but not cleared."]
    #[inline]
    pub fn lck_1(self) -> &'a mut W {
        self.variant(LCKW::LCK_1)
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
    #[doc = "Bit 0 - Domain 0 Write Access to Region"]
    #[inline]
    pub fn d0w(&self) -> D0WR {
        D0WR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Domain 0 Read Access to Region"]
    #[inline]
    pub fn d0r(&self) -> D0RR {
        D0RR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Domain 1 Write Access to Region"]
    #[inline]
    pub fn d1w(&self) -> D1WR {
        D1WR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Domain 1 Read Access to Region"]
    #[inline]
    pub fn d1r(&self) -> D1RR {
        D1RR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Domain 2 Write Access to Region"]
    #[inline]
    pub fn d2w(&self) -> D2WR {
        D2WR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Domain 2 Read Access to Region"]
    #[inline]
    pub fn d2r(&self) -> D2RR {
        D2RR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Domain 3 Write Access to Region"]
    #[inline]
    pub fn d3w(&self) -> D3WR {
        D3WR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Domain 3 Read Access to Region"]
    #[inline]
    pub fn d3r(&self) -> D3RR {
        D3RR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Region Enable"]
    #[inline]
    pub fn ena(&self) -> ENAR {
        ENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Region Lock"]
    #[inline]
    pub fn lck(&self) -> LCKR {
        LCKR::_from({
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
        W { bits: 255 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Domain 0 Write Access to Region"]
    #[inline]
    pub fn d0w(&mut self) -> _D0WW {
        _D0WW { w: self }
    }
    #[doc = "Bit 1 - Domain 0 Read Access to Region"]
    #[inline]
    pub fn d0r(&mut self) -> _D0RW {
        _D0RW { w: self }
    }
    #[doc = "Bit 2 - Domain 1 Write Access to Region"]
    #[inline]
    pub fn d1w(&mut self) -> _D1WW {
        _D1WW { w: self }
    }
    #[doc = "Bit 3 - Domain 1 Read Access to Region"]
    #[inline]
    pub fn d1r(&mut self) -> _D1RW {
        _D1RW { w: self }
    }
    #[doc = "Bit 4 - Domain 2 Write Access to Region"]
    #[inline]
    pub fn d2w(&mut self) -> _D2WW {
        _D2WW { w: self }
    }
    #[doc = "Bit 5 - Domain 2 Read Access to Region"]
    #[inline]
    pub fn d2r(&mut self) -> _D2RW {
        _D2RW { w: self }
    }
    #[doc = "Bit 6 - Domain 3 Write Access to Region"]
    #[inline]
    pub fn d3w(&mut self) -> _D3WW {
        _D3WW { w: self }
    }
    #[doc = "Bit 7 - Domain 3 Read Access to Region"]
    #[inline]
    pub fn d3r(&mut self) -> _D3RW {
        _D3RW { w: self }
    }
    #[doc = "Bit 30 - Region Enable"]
    #[inline]
    pub fn ena(&mut self) -> _ENAW {
        _ENAW { w: self }
    }
    #[doc = "Bit 31 - Region Lock"]
    #[inline]
    pub fn lck(&mut self) -> _LCKW {
        _LCKW { w: self }
    }
}
