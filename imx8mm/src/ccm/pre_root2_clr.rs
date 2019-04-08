#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRE_ROOT2_CLR {
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
#[doc = "Possible values of the field `PRE_PODF_B`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRE_PODF_BR {
    #[doc = "Divide by 1"]
    PRE_PODF_B_0,
    #[doc = "Divide by 2"]
    PRE_PODF_B_1,
    #[doc = "Divide by 3"]
    PRE_PODF_B_2,
    #[doc = "Divide by 4"]
    PRE_PODF_B_3,
    #[doc = "Divide by 5"]
    PRE_PODF_B_4,
    #[doc = "Divide by 6"]
    PRE_PODF_B_5,
    #[doc = "Divide by 7"]
    PRE_PODF_B_6,
    #[doc = "Divide by 8"]
    PRE_PODF_B_7,
}
impl PRE_PODF_BR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRE_PODF_BR::PRE_PODF_B_0 => 0,
            PRE_PODF_BR::PRE_PODF_B_1 => 1,
            PRE_PODF_BR::PRE_PODF_B_2 => 2,
            PRE_PODF_BR::PRE_PODF_B_3 => 3,
            PRE_PODF_BR::PRE_PODF_B_4 => 4,
            PRE_PODF_BR::PRE_PODF_B_5 => 5,
            PRE_PODF_BR::PRE_PODF_B_6 => 6,
            PRE_PODF_BR::PRE_PODF_B_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRE_PODF_BR {
        match value {
            0 => PRE_PODF_BR::PRE_PODF_B_0,
            1 => PRE_PODF_BR::PRE_PODF_B_1,
            2 => PRE_PODF_BR::PRE_PODF_B_2,
            3 => PRE_PODF_BR::PRE_PODF_B_3,
            4 => PRE_PODF_BR::PRE_PODF_B_4,
            5 => PRE_PODF_BR::PRE_PODF_B_5,
            6 => PRE_PODF_BR::PRE_PODF_B_6,
            7 => PRE_PODF_BR::PRE_PODF_B_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRE_PODF_B_0`"]
    #[inline]
    pub fn is_pre_podf_b_0(&self) -> bool {
        *self == PRE_PODF_BR::PRE_PODF_B_0
    }
    #[doc = "Checks if the value of the field is `PRE_PODF_B_1`"]
    #[inline]
    pub fn is_pre_podf_b_1(&self) -> bool {
        *self == PRE_PODF_BR::PRE_PODF_B_1
    }
    #[doc = "Checks if the value of the field is `PRE_PODF_B_2`"]
    #[inline]
    pub fn is_pre_podf_b_2(&self) -> bool {
        *self == PRE_PODF_BR::PRE_PODF_B_2
    }
    #[doc = "Checks if the value of the field is `PRE_PODF_B_3`"]
    #[inline]
    pub fn is_pre_podf_b_3(&self) -> bool {
        *self == PRE_PODF_BR::PRE_PODF_B_3
    }
    #[doc = "Checks if the value of the field is `PRE_PODF_B_4`"]
    #[inline]
    pub fn is_pre_podf_b_4(&self) -> bool {
        *self == PRE_PODF_BR::PRE_PODF_B_4
    }
    #[doc = "Checks if the value of the field is `PRE_PODF_B_5`"]
    #[inline]
    pub fn is_pre_podf_b_5(&self) -> bool {
        *self == PRE_PODF_BR::PRE_PODF_B_5
    }
    #[doc = "Checks if the value of the field is `PRE_PODF_B_6`"]
    #[inline]
    pub fn is_pre_podf_b_6(&self) -> bool {
        *self == PRE_PODF_BR::PRE_PODF_B_6
    }
    #[doc = "Checks if the value of the field is `PRE_PODF_B_7`"]
    #[inline]
    pub fn is_pre_podf_b_7(&self) -> bool {
        *self == PRE_PODF_BR::PRE_PODF_B_7
    }
}
#[doc = r" Value of the field"]
pub struct BUSY0R {
    bits: bool,
}
impl BUSY0R {
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
#[doc = r" Value of the field"]
pub struct MUX_BR {
    bits: u8,
}
impl MUX_BR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `EN_B`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_BR {
    #[doc = "Clock shutdown"]
    EN_B_0,
    #[doc = "Clock ON"]
    EN_B_1,
}
impl EN_BR {
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
            EN_BR::EN_B_0 => false,
            EN_BR::EN_B_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_BR {
        match value {
            false => EN_BR::EN_B_0,
            true => EN_BR::EN_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_B_0`"]
    #[inline]
    pub fn is_en_b_0(&self) -> bool {
        *self == EN_BR::EN_B_0
    }
    #[doc = "Checks if the value of the field is `EN_B_1`"]
    #[inline]
    pub fn is_en_b_1(&self) -> bool {
        *self == EN_BR::EN_B_1
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
#[doc = "Possible values of the field `PRE_PODF_A`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRE_PODF_AR {
    #[doc = "Divide by 1"]
    PRE_PODF_A_0,
    #[doc = "Divide by 2"]
    PRE_PODF_A_1,
    #[doc = "Divide by 3"]
    PRE_PODF_A_2,
    #[doc = "Divide by 4"]
    PRE_PODF_A_3,
    #[doc = "Divide by 5"]
    PRE_PODF_A_4,
    #[doc = "Divide by 6"]
    PRE_PODF_A_5,
    #[doc = "Divide by 7"]
    PRE_PODF_A_6,
    #[doc = "Divide by 8"]
    PRE_PODF_A_7,
}
impl PRE_PODF_AR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRE_PODF_AR::PRE_PODF_A_0 => 0,
            PRE_PODF_AR::PRE_PODF_A_1 => 1,
            PRE_PODF_AR::PRE_PODF_A_2 => 2,
            PRE_PODF_AR::PRE_PODF_A_3 => 3,
            PRE_PODF_AR::PRE_PODF_A_4 => 4,
            PRE_PODF_AR::PRE_PODF_A_5 => 5,
            PRE_PODF_AR::PRE_PODF_A_6 => 6,
            PRE_PODF_AR::PRE_PODF_A_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRE_PODF_AR {
        match value {
            0 => PRE_PODF_AR::PRE_PODF_A_0,
            1 => PRE_PODF_AR::PRE_PODF_A_1,
            2 => PRE_PODF_AR::PRE_PODF_A_2,
            3 => PRE_PODF_AR::PRE_PODF_A_3,
            4 => PRE_PODF_AR::PRE_PODF_A_4,
            5 => PRE_PODF_AR::PRE_PODF_A_5,
            6 => PRE_PODF_AR::PRE_PODF_A_6,
            7 => PRE_PODF_AR::PRE_PODF_A_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRE_PODF_A_0`"]
    #[inline]
    pub fn is_pre_podf_a_0(&self) -> bool {
        *self == PRE_PODF_AR::PRE_PODF_A_0
    }
    #[doc = "Checks if the value of the field is `PRE_PODF_A_1`"]
    #[inline]
    pub fn is_pre_podf_a_1(&self) -> bool {
        *self == PRE_PODF_AR::PRE_PODF_A_1
    }
    #[doc = "Checks if the value of the field is `PRE_PODF_A_2`"]
    #[inline]
    pub fn is_pre_podf_a_2(&self) -> bool {
        *self == PRE_PODF_AR::PRE_PODF_A_2
    }
    #[doc = "Checks if the value of the field is `PRE_PODF_A_3`"]
    #[inline]
    pub fn is_pre_podf_a_3(&self) -> bool {
        *self == PRE_PODF_AR::PRE_PODF_A_3
    }
    #[doc = "Checks if the value of the field is `PRE_PODF_A_4`"]
    #[inline]
    pub fn is_pre_podf_a_4(&self) -> bool {
        *self == PRE_PODF_AR::PRE_PODF_A_4
    }
    #[doc = "Checks if the value of the field is `PRE_PODF_A_5`"]
    #[inline]
    pub fn is_pre_podf_a_5(&self) -> bool {
        *self == PRE_PODF_AR::PRE_PODF_A_5
    }
    #[doc = "Checks if the value of the field is `PRE_PODF_A_6`"]
    #[inline]
    pub fn is_pre_podf_a_6(&self) -> bool {
        *self == PRE_PODF_AR::PRE_PODF_A_6
    }
    #[doc = "Checks if the value of the field is `PRE_PODF_A_7`"]
    #[inline]
    pub fn is_pre_podf_a_7(&self) -> bool {
        *self == PRE_PODF_AR::PRE_PODF_A_7
    }
}
#[doc = r" Value of the field"]
pub struct BUSY3R {
    bits: bool,
}
impl BUSY3R {
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
#[doc = r" Value of the field"]
pub struct MUX_AR {
    bits: u8,
}
impl MUX_AR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `EN_A`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_AR {
    #[doc = "Clock shutdown"]
    EN_A_0,
    #[doc = "clock ON"]
    EN_A_1,
}
impl EN_AR {
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
            EN_AR::EN_A_0 => false,
            EN_AR::EN_A_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_AR {
        match value {
            false => EN_AR::EN_A_0,
            true => EN_AR::EN_A_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_A_0`"]
    #[inline]
    pub fn is_en_a_0(&self) -> bool {
        *self == EN_AR::EN_A_0
    }
    #[doc = "Checks if the value of the field is `EN_A_1`"]
    #[inline]
    pub fn is_en_a_1(&self) -> bool {
        *self == EN_AR::EN_A_1
    }
}
#[doc = r" Value of the field"]
pub struct BUSY4R {
    bits: bool,
}
impl BUSY4R {
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
#[doc = "Values that can be written to the field `PRE_PODF_B`"]
pub enum PRE_PODF_BW {
    #[doc = "Divide by 1"]
    PRE_PODF_B_0,
    #[doc = "Divide by 2"]
    PRE_PODF_B_1,
    #[doc = "Divide by 3"]
    PRE_PODF_B_2,
    #[doc = "Divide by 4"]
    PRE_PODF_B_3,
    #[doc = "Divide by 5"]
    PRE_PODF_B_4,
    #[doc = "Divide by 6"]
    PRE_PODF_B_5,
    #[doc = "Divide by 7"]
    PRE_PODF_B_6,
    #[doc = "Divide by 8"]
    PRE_PODF_B_7,
}
impl PRE_PODF_BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRE_PODF_BW::PRE_PODF_B_0 => 0,
            PRE_PODF_BW::PRE_PODF_B_1 => 1,
            PRE_PODF_BW::PRE_PODF_B_2 => 2,
            PRE_PODF_BW::PRE_PODF_B_3 => 3,
            PRE_PODF_BW::PRE_PODF_B_4 => 4,
            PRE_PODF_BW::PRE_PODF_B_5 => 5,
            PRE_PODF_BW::PRE_PODF_B_6 => 6,
            PRE_PODF_BW::PRE_PODF_B_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRE_PODF_BW<'a> {
    w: &'a mut W,
}
impl<'a> _PRE_PODF_BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRE_PODF_BW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn pre_podf_b_0(self) -> &'a mut W {
        self.variant(PRE_PODF_BW::PRE_PODF_B_0)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn pre_podf_b_1(self) -> &'a mut W {
        self.variant(PRE_PODF_BW::PRE_PODF_B_1)
    }
    #[doc = "Divide by 3"]
    #[inline]
    pub fn pre_podf_b_2(self) -> &'a mut W {
        self.variant(PRE_PODF_BW::PRE_PODF_B_2)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn pre_podf_b_3(self) -> &'a mut W {
        self.variant(PRE_PODF_BW::PRE_PODF_B_3)
    }
    #[doc = "Divide by 5"]
    #[inline]
    pub fn pre_podf_b_4(self) -> &'a mut W {
        self.variant(PRE_PODF_BW::PRE_PODF_B_4)
    }
    #[doc = "Divide by 6"]
    #[inline]
    pub fn pre_podf_b_5(self) -> &'a mut W {
        self.variant(PRE_PODF_BW::PRE_PODF_B_5)
    }
    #[doc = "Divide by 7"]
    #[inline]
    pub fn pre_podf_b_6(self) -> &'a mut W {
        self.variant(PRE_PODF_BW::PRE_PODF_B_6)
    }
    #[doc = "Divide by 8"]
    #[inline]
    pub fn pre_podf_b_7(self) -> &'a mut W {
        self.variant(PRE_PODF_BW::PRE_PODF_B_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MUX_BW<'a> {
    w: &'a mut W,
}
impl<'a> _MUX_BW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EN_B`"]
pub enum EN_BW {
    #[doc = "Clock shutdown"]
    EN_B_0,
    #[doc = "Clock ON"]
    EN_B_1,
}
impl EN_BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_BW::EN_B_0 => false,
            EN_BW::EN_B_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_BW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_BW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock shutdown"]
    #[inline]
    pub fn en_b_0(self) -> &'a mut W {
        self.variant(EN_BW::EN_B_0)
    }
    #[doc = "Clock ON"]
    #[inline]
    pub fn en_b_1(self) -> &'a mut W {
        self.variant(EN_BW::EN_B_1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRE_PODF_A`"]
pub enum PRE_PODF_AW {
    #[doc = "Divide by 1"]
    PRE_PODF_A_0,
    #[doc = "Divide by 2"]
    PRE_PODF_A_1,
    #[doc = "Divide by 3"]
    PRE_PODF_A_2,
    #[doc = "Divide by 4"]
    PRE_PODF_A_3,
    #[doc = "Divide by 5"]
    PRE_PODF_A_4,
    #[doc = "Divide by 6"]
    PRE_PODF_A_5,
    #[doc = "Divide by 7"]
    PRE_PODF_A_6,
    #[doc = "Divide by 8"]
    PRE_PODF_A_7,
}
impl PRE_PODF_AW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRE_PODF_AW::PRE_PODF_A_0 => 0,
            PRE_PODF_AW::PRE_PODF_A_1 => 1,
            PRE_PODF_AW::PRE_PODF_A_2 => 2,
            PRE_PODF_AW::PRE_PODF_A_3 => 3,
            PRE_PODF_AW::PRE_PODF_A_4 => 4,
            PRE_PODF_AW::PRE_PODF_A_5 => 5,
            PRE_PODF_AW::PRE_PODF_A_6 => 6,
            PRE_PODF_AW::PRE_PODF_A_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRE_PODF_AW<'a> {
    w: &'a mut W,
}
impl<'a> _PRE_PODF_AW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRE_PODF_AW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn pre_podf_a_0(self) -> &'a mut W {
        self.variant(PRE_PODF_AW::PRE_PODF_A_0)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn pre_podf_a_1(self) -> &'a mut W {
        self.variant(PRE_PODF_AW::PRE_PODF_A_1)
    }
    #[doc = "Divide by 3"]
    #[inline]
    pub fn pre_podf_a_2(self) -> &'a mut W {
        self.variant(PRE_PODF_AW::PRE_PODF_A_2)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn pre_podf_a_3(self) -> &'a mut W {
        self.variant(PRE_PODF_AW::PRE_PODF_A_3)
    }
    #[doc = "Divide by 5"]
    #[inline]
    pub fn pre_podf_a_4(self) -> &'a mut W {
        self.variant(PRE_PODF_AW::PRE_PODF_A_4)
    }
    #[doc = "Divide by 6"]
    #[inline]
    pub fn pre_podf_a_5(self) -> &'a mut W {
        self.variant(PRE_PODF_AW::PRE_PODF_A_5)
    }
    #[doc = "Divide by 7"]
    #[inline]
    pub fn pre_podf_a_6(self) -> &'a mut W {
        self.variant(PRE_PODF_AW::PRE_PODF_A_6)
    }
    #[doc = "Divide by 8"]
    #[inline]
    pub fn pre_podf_a_7(self) -> &'a mut W {
        self.variant(PRE_PODF_AW::PRE_PODF_A_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MUX_AW<'a> {
    w: &'a mut W,
}
impl<'a> _MUX_AW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EN_A`"]
pub enum EN_AW {
    #[doc = "Clock shutdown"]
    EN_A_0,
    #[doc = "clock ON"]
    EN_A_1,
}
impl EN_AW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_AW::EN_A_0 => false,
            EN_AW::EN_A_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_AW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_AW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_AW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock shutdown"]
    #[inline]
    pub fn en_a_0(self) -> &'a mut W {
        self.variant(EN_AW::EN_A_0)
    }
    #[doc = "clock ON"]
    #[inline]
    pub fn en_a_1(self) -> &'a mut W {
        self.variant(EN_AW::EN_A_1)
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
    #[doc = "Bits 0:2 - Pre divider divide number for branch B Divider value is n + 1"]
    #[inline]
    pub fn pre_podf_b(&self) -> PRE_PODF_BR {
        PRE_PODF_BR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Pre divider value for branch A is applied This field does not apply for CORE, IP,DRAM, DRAM_PHYM"]
    #[inline]
    pub fn busy0(&self) -> BUSY0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BUSY0R { bits }
    }
    #[doc = "Bits 8:10 - Selection control of multiplexer of branch B This field does not apply for CORE, IP,DRAM, DRAM_PHYM"]
    #[inline]
    pub fn mux_b(&self) -> MUX_BR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MUX_BR { bits }
    }
    #[doc = "Bit 12 - Branch B clock gate control This field does not apply for CORE, IP,DRAM, DRAM_PHYM"]
    #[inline]
    pub fn en_b(&self) -> EN_BR {
        EN_BR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - EN_B is applied to field This field does not apply for CORE, IP,DRAM, DRAM_PHYM"]
    #[inline]
    pub fn busy1(&self) -> BUSY1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BUSY1R { bits }
    }
    #[doc = "Bits 16:18 - Pre divider divide number for branch A Divider value is n + 1"]
    #[inline]
    pub fn pre_podf_a(&self) -> PRE_PODF_AR {
        PRE_PODF_AR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 19 - Pre divider value for branch A is applied This field applies to DRAM and DRAM_PHYM"]
    #[inline]
    pub fn busy3(&self) -> BUSY3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BUSY3R { bits }
    }
    #[doc = "Bits 24:26 - Selection control of multiplexer of branch A This field applies to DRAM and DRAM_PHYM"]
    #[inline]
    pub fn mux_a(&self) -> MUX_AR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MUX_AR { bits }
    }
    #[doc = "Bit 28 - Branch A clock gate control This field applies to DRAM and DRAM_PHYM"]
    #[inline]
    pub fn en_a(&self) -> EN_AR {
        EN_AR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - EN_A field is applied to field This field applies to DRAM and DRAM_PHYM"]
    #[inline]
    pub fn busy4(&self) -> BUSY4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BUSY4R { bits }
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
    #[doc = "Bits 0:2 - Pre divider divide number for branch B Divider value is n + 1"]
    #[inline]
    pub fn pre_podf_b(&mut self) -> _PRE_PODF_BW {
        _PRE_PODF_BW { w: self }
    }
    #[doc = "Bits 8:10 - Selection control of multiplexer of branch B This field does not apply for CORE, IP,DRAM, DRAM_PHYM"]
    #[inline]
    pub fn mux_b(&mut self) -> _MUX_BW {
        _MUX_BW { w: self }
    }
    #[doc = "Bit 12 - Branch B clock gate control This field does not apply for CORE, IP,DRAM, DRAM_PHYM"]
    #[inline]
    pub fn en_b(&mut self) -> _EN_BW {
        _EN_BW { w: self }
    }
    #[doc = "Bits 16:18 - Pre divider divide number for branch A Divider value is n + 1"]
    #[inline]
    pub fn pre_podf_a(&mut self) -> _PRE_PODF_AW {
        _PRE_PODF_AW { w: self }
    }
    #[doc = "Bits 24:26 - Selection control of multiplexer of branch A This field applies to DRAM and DRAM_PHYM"]
    #[inline]
    pub fn mux_a(&mut self) -> _MUX_AW {
        _MUX_AW { w: self }
    }
    #[doc = "Bit 28 - Branch A clock gate control This field applies to DRAM and DRAM_PHYM"]
    #[inline]
    pub fn en_a(&mut self) -> _EN_AW {
        _EN_AW { w: self }
    }
}
