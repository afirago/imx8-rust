#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CPINE {
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
#[doc = "Possible values of the field `INE7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INE7R {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE7_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE7_1,
}
impl INE7R {
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
            INE7R::INE7_0 => false,
            INE7R::INE7_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INE7R {
        match value {
            false => INE7R::INE7_0,
            true => INE7R::INE7_1,
        }
    }
    #[doc = "Checks if the value of the field is `INE7_0`"]
    #[inline]
    pub fn is_ine7_0(&self) -> bool {
        *self == INE7R::INE7_0
    }
    #[doc = "Checks if the value of the field is `INE7_1`"]
    #[inline]
    pub fn is_ine7_1(&self) -> bool {
        *self == INE7R::INE7_1
    }
}
#[doc = "Possible values of the field `INE6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INE6R {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE6_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE6_1,
}
impl INE6R {
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
            INE6R::INE6_0 => false,
            INE6R::INE6_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INE6R {
        match value {
            false => INE6R::INE6_0,
            true => INE6R::INE6_1,
        }
    }
    #[doc = "Checks if the value of the field is `INE6_0`"]
    #[inline]
    pub fn is_ine6_0(&self) -> bool {
        *self == INE6R::INE6_0
    }
    #[doc = "Checks if the value of the field is `INE6_1`"]
    #[inline]
    pub fn is_ine6_1(&self) -> bool {
        *self == INE6R::INE6_1
    }
}
#[doc = "Possible values of the field `INE5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INE5R {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE5_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE5_1,
}
impl INE5R {
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
            INE5R::INE5_0 => false,
            INE5R::INE5_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INE5R {
        match value {
            false => INE5R::INE5_0,
            true => INE5R::INE5_1,
        }
    }
    #[doc = "Checks if the value of the field is `INE5_0`"]
    #[inline]
    pub fn is_ine5_0(&self) -> bool {
        *self == INE5R::INE5_0
    }
    #[doc = "Checks if the value of the field is `INE5_1`"]
    #[inline]
    pub fn is_ine5_1(&self) -> bool {
        *self == INE5R::INE5_1
    }
}
#[doc = "Possible values of the field `INE4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INE4R {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE4_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE4_1,
}
impl INE4R {
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
            INE4R::INE4_0 => false,
            INE4R::INE4_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INE4R {
        match value {
            false => INE4R::INE4_0,
            true => INE4R::INE4_1,
        }
    }
    #[doc = "Checks if the value of the field is `INE4_0`"]
    #[inline]
    pub fn is_ine4_0(&self) -> bool {
        *self == INE4R::INE4_0
    }
    #[doc = "Checks if the value of the field is `INE4_1`"]
    #[inline]
    pub fn is_ine4_1(&self) -> bool {
        *self == INE4R::INE4_1
    }
}
#[doc = "Possible values of the field `INE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INE3R {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE3_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE3_1,
}
impl INE3R {
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
            INE3R::INE3_0 => false,
            INE3R::INE3_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INE3R {
        match value {
            false => INE3R::INE3_0,
            true => INE3R::INE3_1,
        }
    }
    #[doc = "Checks if the value of the field is `INE3_0`"]
    #[inline]
    pub fn is_ine3_0(&self) -> bool {
        *self == INE3R::INE3_0
    }
    #[doc = "Checks if the value of the field is `INE3_1`"]
    #[inline]
    pub fn is_ine3_1(&self) -> bool {
        *self == INE3R::INE3_1
    }
}
#[doc = "Possible values of the field `INE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INE2R {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE2_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE2_1,
}
impl INE2R {
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
            INE2R::INE2_0 => false,
            INE2R::INE2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INE2R {
        match value {
            false => INE2R::INE2_0,
            true => INE2R::INE2_1,
        }
    }
    #[doc = "Checks if the value of the field is `INE2_0`"]
    #[inline]
    pub fn is_ine2_0(&self) -> bool {
        *self == INE2R::INE2_0
    }
    #[doc = "Checks if the value of the field is `INE2_1`"]
    #[inline]
    pub fn is_ine2_1(&self) -> bool {
        *self == INE2R::INE2_1
    }
}
#[doc = "Possible values of the field `INE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INE1R {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE1_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE1_1,
}
impl INE1R {
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
            INE1R::INE1_0 => false,
            INE1R::INE1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INE1R {
        match value {
            false => INE1R::INE1_0,
            true => INE1R::INE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `INE1_0`"]
    #[inline]
    pub fn is_ine1_0(&self) -> bool {
        *self == INE1R::INE1_0
    }
    #[doc = "Checks if the value of the field is `INE1_1`"]
    #[inline]
    pub fn is_ine1_1(&self) -> bool {
        *self == INE1R::INE1_1
    }
}
#[doc = "Possible values of the field `INE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INE0R {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE0_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE0_1,
}
impl INE0R {
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
            INE0R::INE0_0 => false,
            INE0R::INE0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INE0R {
        match value {
            false => INE0R::INE0_0,
            true => INE0R::INE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `INE0_0`"]
    #[inline]
    pub fn is_ine0_0(&self) -> bool {
        *self == INE0R::INE0_0
    }
    #[doc = "Checks if the value of the field is `INE0_1`"]
    #[inline]
    pub fn is_ine0_1(&self) -> bool {
        *self == INE0R::INE0_1
    }
}
#[doc = "Possible values of the field `INE15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INE15R {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE15_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE15_1,
}
impl INE15R {
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
            INE15R::INE15_0 => false,
            INE15R::INE15_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INE15R {
        match value {
            false => INE15R::INE15_0,
            true => INE15R::INE15_1,
        }
    }
    #[doc = "Checks if the value of the field is `INE15_0`"]
    #[inline]
    pub fn is_ine15_0(&self) -> bool {
        *self == INE15R::INE15_0
    }
    #[doc = "Checks if the value of the field is `INE15_1`"]
    #[inline]
    pub fn is_ine15_1(&self) -> bool {
        *self == INE15R::INE15_1
    }
}
#[doc = "Possible values of the field `INE14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INE14R {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE14_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE14_1,
}
impl INE14R {
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
            INE14R::INE14_0 => false,
            INE14R::INE14_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INE14R {
        match value {
            false => INE14R::INE14_0,
            true => INE14R::INE14_1,
        }
    }
    #[doc = "Checks if the value of the field is `INE14_0`"]
    #[inline]
    pub fn is_ine14_0(&self) -> bool {
        *self == INE14R::INE14_0
    }
    #[doc = "Checks if the value of the field is `INE14_1`"]
    #[inline]
    pub fn is_ine14_1(&self) -> bool {
        *self == INE14R::INE14_1
    }
}
#[doc = "Possible values of the field `INE13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INE13R {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE13_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE13_1,
}
impl INE13R {
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
            INE13R::INE13_0 => false,
            INE13R::INE13_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INE13R {
        match value {
            false => INE13R::INE13_0,
            true => INE13R::INE13_1,
        }
    }
    #[doc = "Checks if the value of the field is `INE13_0`"]
    #[inline]
    pub fn is_ine13_0(&self) -> bool {
        *self == INE13R::INE13_0
    }
    #[doc = "Checks if the value of the field is `INE13_1`"]
    #[inline]
    pub fn is_ine13_1(&self) -> bool {
        *self == INE13R::INE13_1
    }
}
#[doc = "Possible values of the field `INE12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INE12R {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE12_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE12_1,
}
impl INE12R {
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
            INE12R::INE12_0 => false,
            INE12R::INE12_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INE12R {
        match value {
            false => INE12R::INE12_0,
            true => INE12R::INE12_1,
        }
    }
    #[doc = "Checks if the value of the field is `INE12_0`"]
    #[inline]
    pub fn is_ine12_0(&self) -> bool {
        *self == INE12R::INE12_0
    }
    #[doc = "Checks if the value of the field is `INE12_1`"]
    #[inline]
    pub fn is_ine12_1(&self) -> bool {
        *self == INE12R::INE12_1
    }
}
#[doc = "Possible values of the field `INE11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INE11R {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE11_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE11_1,
}
impl INE11R {
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
            INE11R::INE11_0 => false,
            INE11R::INE11_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INE11R {
        match value {
            false => INE11R::INE11_0,
            true => INE11R::INE11_1,
        }
    }
    #[doc = "Checks if the value of the field is `INE11_0`"]
    #[inline]
    pub fn is_ine11_0(&self) -> bool {
        *self == INE11R::INE11_0
    }
    #[doc = "Checks if the value of the field is `INE11_1`"]
    #[inline]
    pub fn is_ine11_1(&self) -> bool {
        *self == INE11R::INE11_1
    }
}
#[doc = "Possible values of the field `INE10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INE10R {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE10_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE10_1,
}
impl INE10R {
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
            INE10R::INE10_0 => false,
            INE10R::INE10_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INE10R {
        match value {
            false => INE10R::INE10_0,
            true => INE10R::INE10_1,
        }
    }
    #[doc = "Checks if the value of the field is `INE10_0`"]
    #[inline]
    pub fn is_ine10_0(&self) -> bool {
        *self == INE10R::INE10_0
    }
    #[doc = "Checks if the value of the field is `INE10_1`"]
    #[inline]
    pub fn is_ine10_1(&self) -> bool {
        *self == INE10R::INE10_1
    }
}
#[doc = "Possible values of the field `INE9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INE9R {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE9_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE9_1,
}
impl INE9R {
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
            INE9R::INE9_0 => false,
            INE9R::INE9_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INE9R {
        match value {
            false => INE9R::INE9_0,
            true => INE9R::INE9_1,
        }
    }
    #[doc = "Checks if the value of the field is `INE9_0`"]
    #[inline]
    pub fn is_ine9_0(&self) -> bool {
        *self == INE9R::INE9_0
    }
    #[doc = "Checks if the value of the field is `INE9_1`"]
    #[inline]
    pub fn is_ine9_1(&self) -> bool {
        *self == INE9R::INE9_1
    }
}
#[doc = "Possible values of the field `INE8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INE8R {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE8_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE8_1,
}
impl INE8R {
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
            INE8R::INE8_0 => false,
            INE8R::INE8_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INE8R {
        match value {
            false => INE8R::INE8_0,
            true => INE8R::INE8_1,
        }
    }
    #[doc = "Checks if the value of the field is `INE8_0`"]
    #[inline]
    pub fn is_ine8_0(&self) -> bool {
        *self == INE8R::INE8_0
    }
    #[doc = "Checks if the value of the field is `INE8_1`"]
    #[inline]
    pub fn is_ine8_1(&self) -> bool {
        *self == INE8R::INE8_1
    }
}
#[doc = "Values that can be written to the field `INE7`"]
pub enum INE7W {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE7_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE7_1,
}
impl INE7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INE7W::INE7_0 => false,
            INE7W::INE7_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INE7W<'a> {
    w: &'a mut W,
}
impl<'a> _INE7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INE7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The generation of the notification interrupt is disabled."]
    #[inline]
    pub fn ine7_0(self) -> &'a mut W {
        self.variant(INE7W::INE7_0)
    }
    #[doc = "The generation of the notification interrupt is enabled."]
    #[inline]
    pub fn ine7_1(self) -> &'a mut W {
        self.variant(INE7W::INE7_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INE6`"]
pub enum INE6W {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE6_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE6_1,
}
impl INE6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INE6W::INE6_0 => false,
            INE6W::INE6_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INE6W<'a> {
    w: &'a mut W,
}
impl<'a> _INE6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INE6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The generation of the notification interrupt is disabled."]
    #[inline]
    pub fn ine6_0(self) -> &'a mut W {
        self.variant(INE6W::INE6_0)
    }
    #[doc = "The generation of the notification interrupt is enabled."]
    #[inline]
    pub fn ine6_1(self) -> &'a mut W {
        self.variant(INE6W::INE6_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INE5`"]
pub enum INE5W {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE5_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE5_1,
}
impl INE5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INE5W::INE5_0 => false,
            INE5W::INE5_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INE5W<'a> {
    w: &'a mut W,
}
impl<'a> _INE5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INE5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The generation of the notification interrupt is disabled."]
    #[inline]
    pub fn ine5_0(self) -> &'a mut W {
        self.variant(INE5W::INE5_0)
    }
    #[doc = "The generation of the notification interrupt is enabled."]
    #[inline]
    pub fn ine5_1(self) -> &'a mut W {
        self.variant(INE5W::INE5_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INE4`"]
pub enum INE4W {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE4_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE4_1,
}
impl INE4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INE4W::INE4_0 => false,
            INE4W::INE4_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INE4W<'a> {
    w: &'a mut W,
}
impl<'a> _INE4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INE4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The generation of the notification interrupt is disabled."]
    #[inline]
    pub fn ine4_0(self) -> &'a mut W {
        self.variant(INE4W::INE4_0)
    }
    #[doc = "The generation of the notification interrupt is enabled."]
    #[inline]
    pub fn ine4_1(self) -> &'a mut W {
        self.variant(INE4W::INE4_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INE3`"]
pub enum INE3W {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE3_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE3_1,
}
impl INE3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INE3W::INE3_0 => false,
            INE3W::INE3_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INE3W<'a> {
    w: &'a mut W,
}
impl<'a> _INE3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INE3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The generation of the notification interrupt is disabled."]
    #[inline]
    pub fn ine3_0(self) -> &'a mut W {
        self.variant(INE3W::INE3_0)
    }
    #[doc = "The generation of the notification interrupt is enabled."]
    #[inline]
    pub fn ine3_1(self) -> &'a mut W {
        self.variant(INE3W::INE3_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INE2`"]
pub enum INE2W {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE2_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE2_1,
}
impl INE2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INE2W::INE2_0 => false,
            INE2W::INE2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INE2W<'a> {
    w: &'a mut W,
}
impl<'a> _INE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INE2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The generation of the notification interrupt is disabled."]
    #[inline]
    pub fn ine2_0(self) -> &'a mut W {
        self.variant(INE2W::INE2_0)
    }
    #[doc = "The generation of the notification interrupt is enabled."]
    #[inline]
    pub fn ine2_1(self) -> &'a mut W {
        self.variant(INE2W::INE2_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INE1`"]
pub enum INE1W {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE1_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE1_1,
}
impl INE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INE1W::INE1_0 => false,
            INE1W::INE1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INE1W<'a> {
    w: &'a mut W,
}
impl<'a> _INE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The generation of the notification interrupt is disabled."]
    #[inline]
    pub fn ine1_0(self) -> &'a mut W {
        self.variant(INE1W::INE1_0)
    }
    #[doc = "The generation of the notification interrupt is enabled."]
    #[inline]
    pub fn ine1_1(self) -> &'a mut W {
        self.variant(INE1W::INE1_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INE0`"]
pub enum INE0W {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE0_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE0_1,
}
impl INE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INE0W::INE0_0 => false,
            INE0W::INE0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INE0W<'a> {
    w: &'a mut W,
}
impl<'a> _INE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The generation of the notification interrupt is disabled."]
    #[inline]
    pub fn ine0_0(self) -> &'a mut W {
        self.variant(INE0W::INE0_0)
    }
    #[doc = "The generation of the notification interrupt is enabled."]
    #[inline]
    pub fn ine0_1(self) -> &'a mut W {
        self.variant(INE0W::INE0_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INE15`"]
pub enum INE15W {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE15_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE15_1,
}
impl INE15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INE15W::INE15_0 => false,
            INE15W::INE15_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INE15W<'a> {
    w: &'a mut W,
}
impl<'a> _INE15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INE15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The generation of the notification interrupt is disabled."]
    #[inline]
    pub fn ine15_0(self) -> &'a mut W {
        self.variant(INE15W::INE15_0)
    }
    #[doc = "The generation of the notification interrupt is enabled."]
    #[inline]
    pub fn ine15_1(self) -> &'a mut W {
        self.variant(INE15W::INE15_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INE14`"]
pub enum INE14W {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE14_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE14_1,
}
impl INE14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INE14W::INE14_0 => false,
            INE14W::INE14_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INE14W<'a> {
    w: &'a mut W,
}
impl<'a> _INE14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INE14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The generation of the notification interrupt is disabled."]
    #[inline]
    pub fn ine14_0(self) -> &'a mut W {
        self.variant(INE14W::INE14_0)
    }
    #[doc = "The generation of the notification interrupt is enabled."]
    #[inline]
    pub fn ine14_1(self) -> &'a mut W {
        self.variant(INE14W::INE14_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INE13`"]
pub enum INE13W {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE13_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE13_1,
}
impl INE13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INE13W::INE13_0 => false,
            INE13W::INE13_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INE13W<'a> {
    w: &'a mut W,
}
impl<'a> _INE13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INE13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The generation of the notification interrupt is disabled."]
    #[inline]
    pub fn ine13_0(self) -> &'a mut W {
        self.variant(INE13W::INE13_0)
    }
    #[doc = "The generation of the notification interrupt is enabled."]
    #[inline]
    pub fn ine13_1(self) -> &'a mut W {
        self.variant(INE13W::INE13_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INE12`"]
pub enum INE12W {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE12_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE12_1,
}
impl INE12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INE12W::INE12_0 => false,
            INE12W::INE12_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INE12W<'a> {
    w: &'a mut W,
}
impl<'a> _INE12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INE12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The generation of the notification interrupt is disabled."]
    #[inline]
    pub fn ine12_0(self) -> &'a mut W {
        self.variant(INE12W::INE12_0)
    }
    #[doc = "The generation of the notification interrupt is enabled."]
    #[inline]
    pub fn ine12_1(self) -> &'a mut W {
        self.variant(INE12W::INE12_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INE11`"]
pub enum INE11W {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE11_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE11_1,
}
impl INE11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INE11W::INE11_0 => false,
            INE11W::INE11_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INE11W<'a> {
    w: &'a mut W,
}
impl<'a> _INE11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INE11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The generation of the notification interrupt is disabled."]
    #[inline]
    pub fn ine11_0(self) -> &'a mut W {
        self.variant(INE11W::INE11_0)
    }
    #[doc = "The generation of the notification interrupt is enabled."]
    #[inline]
    pub fn ine11_1(self) -> &'a mut W {
        self.variant(INE11W::INE11_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INE10`"]
pub enum INE10W {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE10_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE10_1,
}
impl INE10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INE10W::INE10_0 => false,
            INE10W::INE10_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INE10W<'a> {
    w: &'a mut W,
}
impl<'a> _INE10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INE10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The generation of the notification interrupt is disabled."]
    #[inline]
    pub fn ine10_0(self) -> &'a mut W {
        self.variant(INE10W::INE10_0)
    }
    #[doc = "The generation of the notification interrupt is enabled."]
    #[inline]
    pub fn ine10_1(self) -> &'a mut W {
        self.variant(INE10W::INE10_1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INE9`"]
pub enum INE9W {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE9_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE9_1,
}
impl INE9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INE9W::INE9_0 => false,
            INE9W::INE9_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INE9W<'a> {
    w: &'a mut W,
}
impl<'a> _INE9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INE9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The generation of the notification interrupt is disabled."]
    #[inline]
    pub fn ine9_0(self) -> &'a mut W {
        self.variant(INE9W::INE9_0)
    }
    #[doc = "The generation of the notification interrupt is enabled."]
    #[inline]
    pub fn ine9_1(self) -> &'a mut W {
        self.variant(INE9W::INE9_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INE8`"]
pub enum INE8W {
    #[doc = "The generation of the notification interrupt is disabled."]
    INE8_0,
    #[doc = "The generation of the notification interrupt is enabled."]
    INE8_1,
}
impl INE8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INE8W::INE8_0 => false,
            INE8W::INE8_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INE8W<'a> {
    w: &'a mut W,
}
impl<'a> _INE8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INE8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The generation of the notification interrupt is disabled."]
    #[inline]
    pub fn ine8_0(self) -> &'a mut W {
        self.variant(INE8W::INE8_0)
    }
    #[doc = "The generation of the notification interrupt is enabled."]
    #[inline]
    pub fn ine8_1(self) -> &'a mut W {
        self.variant(INE8W::INE8_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Interrupt Request Notification Enable 7. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 7."]
    #[inline]
    pub fn ine7(&self) -> INE7R {
        INE7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Interrupt Request Notification Enable 6. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 6."]
    #[inline]
    pub fn ine6(&self) -> INE6R {
        INE6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - Interrupt Request Notification Enable 5. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 5."]
    #[inline]
    pub fn ine5(&self) -> INE5R {
        INE5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - Interrupt Request Notification Enable 4. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 4."]
    #[inline]
    pub fn ine4(&self) -> INE4R {
        INE4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Interrupt Request Notification Enable 3"]
    #[inline]
    pub fn ine3(&self) -> INE3R {
        INE3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Interrupt Request Notification Enable 2"]
    #[inline]
    pub fn ine2(&self) -> INE2R {
        INE2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - Interrupt Request Notification Enable 1"]
    #[inline]
    pub fn ine1(&self) -> INE1R {
        INE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - Interrupt Request Notification Enable 0"]
    #[inline]
    pub fn ine0(&self) -> INE0R {
        INE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 8 - Interrupt Request Notification Enable 15. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 15."]
    #[inline]
    pub fn ine15(&self) -> INE15R {
        INE15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 9 - Interrupt Request Notification Enable 14. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 14."]
    #[inline]
    pub fn ine14(&self) -> INE14R {
        INE14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 10 - Interrupt Request Notification Enable 13. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 13."]
    #[inline]
    pub fn ine13(&self) -> INE13R {
        INE13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 11 - Interrupt Request Notification Enable 12. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 12."]
    #[inline]
    pub fn ine12(&self) -> INE12R {
        INE12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 12 - Interrupt Request Notification Enable 11. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 11."]
    #[inline]
    pub fn ine11(&self) -> INE11R {
        INE11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 13 - Interrupt Request Notification Enable 10. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 10."]
    #[inline]
    pub fn ine10(&self) -> INE10R {
        INE10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 14 - Interrupt Request Notification Enable 9. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 9."]
    #[inline]
    pub fn ine9(&self) -> INE9R {
        INE9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 15 - Interrupt Request Notification Enable 8. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 8."]
    #[inline]
    pub fn ine8(&self) -> INE8R {
        INE8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u16) != 0
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Interrupt Request Notification Enable 7. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 7."]
    #[inline]
    pub fn ine7(&mut self) -> _INE7W {
        _INE7W { w: self }
    }
    #[doc = "Bit 1 - Interrupt Request Notification Enable 6. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 6."]
    #[inline]
    pub fn ine6(&mut self) -> _INE6W {
        _INE6W { w: self }
    }
    #[doc = "Bit 2 - Interrupt Request Notification Enable 5. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 5."]
    #[inline]
    pub fn ine5(&mut self) -> _INE5W {
        _INE5W { w: self }
    }
    #[doc = "Bit 3 - Interrupt Request Notification Enable 4. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 4."]
    #[inline]
    pub fn ine4(&mut self) -> _INE4W {
        _INE4W { w: self }
    }
    #[doc = "Bit 4 - Interrupt Request Notification Enable 3"]
    #[inline]
    pub fn ine3(&mut self) -> _INE3W {
        _INE3W { w: self }
    }
    #[doc = "Bit 5 - Interrupt Request Notification Enable 2"]
    #[inline]
    pub fn ine2(&mut self) -> _INE2W {
        _INE2W { w: self }
    }
    #[doc = "Bit 6 - Interrupt Request Notification Enable 1"]
    #[inline]
    pub fn ine1(&mut self) -> _INE1W {
        _INE1W { w: self }
    }
    #[doc = "Bit 7 - Interrupt Request Notification Enable 0"]
    #[inline]
    pub fn ine0(&mut self) -> _INE0W {
        _INE0W { w: self }
    }
    #[doc = "Bit 8 - Interrupt Request Notification Enable 15. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 15."]
    #[inline]
    pub fn ine15(&mut self) -> _INE15W {
        _INE15W { w: self }
    }
    #[doc = "Bit 9 - Interrupt Request Notification Enable 14. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 14."]
    #[inline]
    pub fn ine14(&mut self) -> _INE14W {
        _INE14W { w: self }
    }
    #[doc = "Bit 10 - Interrupt Request Notification Enable 13. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 13."]
    #[inline]
    pub fn ine13(&mut self) -> _INE13W {
        _INE13W { w: self }
    }
    #[doc = "Bit 11 - Interrupt Request Notification Enable 12. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 12."]
    #[inline]
    pub fn ine12(&mut self) -> _INE12W {
        _INE12W { w: self }
    }
    #[doc = "Bit 12 - Interrupt Request Notification Enable 11. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 11."]
    #[inline]
    pub fn ine11(&mut self) -> _INE11W {
        _INE11W { w: self }
    }
    #[doc = "Bit 13 - Interrupt Request Notification Enable 10. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 10."]
    #[inline]
    pub fn ine10(&mut self) -> _INE10W {
        _INE10W { w: self }
    }
    #[doc = "Bit 14 - Interrupt Request Notification Enable 9. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 9."]
    #[inline]
    pub fn ine9(&mut self) -> _INE9W {
        _INE9W { w: self }
    }
    #[doc = "Bit 15 - Interrupt Request Notification Enable 8. This field is a bitmap to enable the generation of an interrupt notification from a failed attempt to lock gate 8."]
    #[inline]
    pub fn ine8(&mut self) -> _INE8W {
        _INE8W { w: self }
    }
}
