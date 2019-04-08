#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::A53RCR0 {
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
#[doc = "Possible values of the field `A53_CORE_POR_RESET0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A53_CORE_POR_RESET0R {
    #[doc = "do not assert core0 reset"]
    A53_CORE_POR_RESET0_0,
    #[doc = "assert core0 reset"]
    A53_CORE_POR_RESET0_1,
}
impl A53_CORE_POR_RESET0R {
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
            A53_CORE_POR_RESET0R::A53_CORE_POR_RESET0_0 => false,
            A53_CORE_POR_RESET0R::A53_CORE_POR_RESET0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> A53_CORE_POR_RESET0R {
        match value {
            false => A53_CORE_POR_RESET0R::A53_CORE_POR_RESET0_0,
            true => A53_CORE_POR_RESET0R::A53_CORE_POR_RESET0_1,
        }
    }
    #[doc = "Checks if the value of the field is `A53_CORE_POR_RESET0_0`"]
    #[inline]
    pub fn is_a53_core_por_reset0_0(&self) -> bool {
        *self == A53_CORE_POR_RESET0R::A53_CORE_POR_RESET0_0
    }
    #[doc = "Checks if the value of the field is `A53_CORE_POR_RESET0_1`"]
    #[inline]
    pub fn is_a53_core_por_reset0_1(&self) -> bool {
        *self == A53_CORE_POR_RESET0R::A53_CORE_POR_RESET0_1
    }
}
#[doc = "Possible values of the field `A53_CORE_POR_RESET1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A53_CORE_POR_RESET1R {
    #[doc = "do not assert core1 reset"]
    A53_CORE_POR_RESET1_0,
    #[doc = "assert core1 reset"]
    A53_CORE_POR_RESET1_1,
}
impl A53_CORE_POR_RESET1R {
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
            A53_CORE_POR_RESET1R::A53_CORE_POR_RESET1_0 => false,
            A53_CORE_POR_RESET1R::A53_CORE_POR_RESET1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> A53_CORE_POR_RESET1R {
        match value {
            false => A53_CORE_POR_RESET1R::A53_CORE_POR_RESET1_0,
            true => A53_CORE_POR_RESET1R::A53_CORE_POR_RESET1_1,
        }
    }
    #[doc = "Checks if the value of the field is `A53_CORE_POR_RESET1_0`"]
    #[inline]
    pub fn is_a53_core_por_reset1_0(&self) -> bool {
        *self == A53_CORE_POR_RESET1R::A53_CORE_POR_RESET1_0
    }
    #[doc = "Checks if the value of the field is `A53_CORE_POR_RESET1_1`"]
    #[inline]
    pub fn is_a53_core_por_reset1_1(&self) -> bool {
        *self == A53_CORE_POR_RESET1R::A53_CORE_POR_RESET1_1
    }
}
#[doc = "Possible values of the field `A53_CORE_POR_RESET2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A53_CORE_POR_RESET2R {
    #[doc = "do not assert core2 reset"]
    A53_CORE_POR_RESET2_0,
    #[doc = "assert core2 reset"]
    A53_CORE_POR_RESET2_1,
}
impl A53_CORE_POR_RESET2R {
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
            A53_CORE_POR_RESET2R::A53_CORE_POR_RESET2_0 => false,
            A53_CORE_POR_RESET2R::A53_CORE_POR_RESET2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> A53_CORE_POR_RESET2R {
        match value {
            false => A53_CORE_POR_RESET2R::A53_CORE_POR_RESET2_0,
            true => A53_CORE_POR_RESET2R::A53_CORE_POR_RESET2_1,
        }
    }
    #[doc = "Checks if the value of the field is `A53_CORE_POR_RESET2_0`"]
    #[inline]
    pub fn is_a53_core_por_reset2_0(&self) -> bool {
        *self == A53_CORE_POR_RESET2R::A53_CORE_POR_RESET2_0
    }
    #[doc = "Checks if the value of the field is `A53_CORE_POR_RESET2_1`"]
    #[inline]
    pub fn is_a53_core_por_reset2_1(&self) -> bool {
        *self == A53_CORE_POR_RESET2R::A53_CORE_POR_RESET2_1
    }
}
#[doc = "Possible values of the field `A53_CORE_POR_RESET3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A53_CORE_POR_RESET3R {
    #[doc = "do not assert core3 reset"]
    A53_CORE_POR_RESET3_0,
    #[doc = "assert core3 reset"]
    A53_CORE_POR_RESET3_1,
}
impl A53_CORE_POR_RESET3R {
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
            A53_CORE_POR_RESET3R::A53_CORE_POR_RESET3_0 => false,
            A53_CORE_POR_RESET3R::A53_CORE_POR_RESET3_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> A53_CORE_POR_RESET3R {
        match value {
            false => A53_CORE_POR_RESET3R::A53_CORE_POR_RESET3_0,
            true => A53_CORE_POR_RESET3R::A53_CORE_POR_RESET3_1,
        }
    }
    #[doc = "Checks if the value of the field is `A53_CORE_POR_RESET3_0`"]
    #[inline]
    pub fn is_a53_core_por_reset3_0(&self) -> bool {
        *self == A53_CORE_POR_RESET3R::A53_CORE_POR_RESET3_0
    }
    #[doc = "Checks if the value of the field is `A53_CORE_POR_RESET3_1`"]
    #[inline]
    pub fn is_a53_core_por_reset3_1(&self) -> bool {
        *self == A53_CORE_POR_RESET3R::A53_CORE_POR_RESET3_1
    }
}
#[doc = "Possible values of the field `A53_CORE_RESET0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A53_CORE_RESET0R {
    #[doc = "do not assert core0 reset"]
    A53_CORE_RESET0_0,
    #[doc = "assert core0 reset"]
    A53_CORE_RESET0_1,
}
impl A53_CORE_RESET0R {
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
            A53_CORE_RESET0R::A53_CORE_RESET0_0 => false,
            A53_CORE_RESET0R::A53_CORE_RESET0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> A53_CORE_RESET0R {
        match value {
            false => A53_CORE_RESET0R::A53_CORE_RESET0_0,
            true => A53_CORE_RESET0R::A53_CORE_RESET0_1,
        }
    }
    #[doc = "Checks if the value of the field is `A53_CORE_RESET0_0`"]
    #[inline]
    pub fn is_a53_core_reset0_0(&self) -> bool {
        *self == A53_CORE_RESET0R::A53_CORE_RESET0_0
    }
    #[doc = "Checks if the value of the field is `A53_CORE_RESET0_1`"]
    #[inline]
    pub fn is_a53_core_reset0_1(&self) -> bool {
        *self == A53_CORE_RESET0R::A53_CORE_RESET0_1
    }
}
#[doc = "Possible values of the field `A53_CORE_RESET1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A53_CORE_RESET1R {
    #[doc = "do not assert core1 reset"]
    A53_CORE_RESET1_0,
    #[doc = "assert core1 reset"]
    A53_CORE_RESET1_1,
}
impl A53_CORE_RESET1R {
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
            A53_CORE_RESET1R::A53_CORE_RESET1_0 => false,
            A53_CORE_RESET1R::A53_CORE_RESET1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> A53_CORE_RESET1R {
        match value {
            false => A53_CORE_RESET1R::A53_CORE_RESET1_0,
            true => A53_CORE_RESET1R::A53_CORE_RESET1_1,
        }
    }
    #[doc = "Checks if the value of the field is `A53_CORE_RESET1_0`"]
    #[inline]
    pub fn is_a53_core_reset1_0(&self) -> bool {
        *self == A53_CORE_RESET1R::A53_CORE_RESET1_0
    }
    #[doc = "Checks if the value of the field is `A53_CORE_RESET1_1`"]
    #[inline]
    pub fn is_a53_core_reset1_1(&self) -> bool {
        *self == A53_CORE_RESET1R::A53_CORE_RESET1_1
    }
}
#[doc = "Possible values of the field `A53_CORE_RESET2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A53_CORE_RESET2R {
    #[doc = "do not assert core2 reset"]
    A53_CORE_RESET2_0,
    #[doc = "assert core2 reset"]
    A53_CORE_RESET2_1,
}
impl A53_CORE_RESET2R {
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
            A53_CORE_RESET2R::A53_CORE_RESET2_0 => false,
            A53_CORE_RESET2R::A53_CORE_RESET2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> A53_CORE_RESET2R {
        match value {
            false => A53_CORE_RESET2R::A53_CORE_RESET2_0,
            true => A53_CORE_RESET2R::A53_CORE_RESET2_1,
        }
    }
    #[doc = "Checks if the value of the field is `A53_CORE_RESET2_0`"]
    #[inline]
    pub fn is_a53_core_reset2_0(&self) -> bool {
        *self == A53_CORE_RESET2R::A53_CORE_RESET2_0
    }
    #[doc = "Checks if the value of the field is `A53_CORE_RESET2_1`"]
    #[inline]
    pub fn is_a53_core_reset2_1(&self) -> bool {
        *self == A53_CORE_RESET2R::A53_CORE_RESET2_1
    }
}
#[doc = "Possible values of the field `A53_CORE_RESET3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A53_CORE_RESET3R {
    #[doc = "do not assert core3 reset"]
    A53_CORE_RESET3_0,
    #[doc = "assert core3 reset"]
    A53_CORE_RESET3_1,
}
impl A53_CORE_RESET3R {
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
            A53_CORE_RESET3R::A53_CORE_RESET3_0 => false,
            A53_CORE_RESET3R::A53_CORE_RESET3_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> A53_CORE_RESET3R {
        match value {
            false => A53_CORE_RESET3R::A53_CORE_RESET3_0,
            true => A53_CORE_RESET3R::A53_CORE_RESET3_1,
        }
    }
    #[doc = "Checks if the value of the field is `A53_CORE_RESET3_0`"]
    #[inline]
    pub fn is_a53_core_reset3_0(&self) -> bool {
        *self == A53_CORE_RESET3R::A53_CORE_RESET3_0
    }
    #[doc = "Checks if the value of the field is `A53_CORE_RESET3_1`"]
    #[inline]
    pub fn is_a53_core_reset3_1(&self) -> bool {
        *self == A53_CORE_RESET3R::A53_CORE_RESET3_1
    }
}
#[doc = "Possible values of the field `A53_DBG_RESET0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A53_DBG_RESET0R {
    #[doc = "do not assert core0 debug reset"]
    A53_DBG_RESET0_0,
    #[doc = "assert core0 debug reset"]
    A53_DBG_RESET0_1,
}
impl A53_DBG_RESET0R {
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
            A53_DBG_RESET0R::A53_DBG_RESET0_0 => false,
            A53_DBG_RESET0R::A53_DBG_RESET0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> A53_DBG_RESET0R {
        match value {
            false => A53_DBG_RESET0R::A53_DBG_RESET0_0,
            true => A53_DBG_RESET0R::A53_DBG_RESET0_1,
        }
    }
    #[doc = "Checks if the value of the field is `A53_DBG_RESET0_0`"]
    #[inline]
    pub fn is_a53_dbg_reset0_0(&self) -> bool {
        *self == A53_DBG_RESET0R::A53_DBG_RESET0_0
    }
    #[doc = "Checks if the value of the field is `A53_DBG_RESET0_1`"]
    #[inline]
    pub fn is_a53_dbg_reset0_1(&self) -> bool {
        *self == A53_DBG_RESET0R::A53_DBG_RESET0_1
    }
}
#[doc = "Possible values of the field `A53_DBG_RESET1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A53_DBG_RESET1R {
    #[doc = "do not assert core1 debug reset"]
    A53_DBG_RESET1_0,
    #[doc = "assert core1 debug reset"]
    A53_DBG_RESET1_1,
}
impl A53_DBG_RESET1R {
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
            A53_DBG_RESET1R::A53_DBG_RESET1_0 => false,
            A53_DBG_RESET1R::A53_DBG_RESET1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> A53_DBG_RESET1R {
        match value {
            false => A53_DBG_RESET1R::A53_DBG_RESET1_0,
            true => A53_DBG_RESET1R::A53_DBG_RESET1_1,
        }
    }
    #[doc = "Checks if the value of the field is `A53_DBG_RESET1_0`"]
    #[inline]
    pub fn is_a53_dbg_reset1_0(&self) -> bool {
        *self == A53_DBG_RESET1R::A53_DBG_RESET1_0
    }
    #[doc = "Checks if the value of the field is `A53_DBG_RESET1_1`"]
    #[inline]
    pub fn is_a53_dbg_reset1_1(&self) -> bool {
        *self == A53_DBG_RESET1R::A53_DBG_RESET1_1
    }
}
#[doc = "Possible values of the field `A53_DBG_RESET2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A53_DBG_RESET2R {
    #[doc = "do not assert core2 debug reset"]
    A53_DBG_RESET2_0,
    #[doc = "assert core2 debug reset"]
    A53_DBG_RESET2_1,
}
impl A53_DBG_RESET2R {
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
            A53_DBG_RESET2R::A53_DBG_RESET2_0 => false,
            A53_DBG_RESET2R::A53_DBG_RESET2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> A53_DBG_RESET2R {
        match value {
            false => A53_DBG_RESET2R::A53_DBG_RESET2_0,
            true => A53_DBG_RESET2R::A53_DBG_RESET2_1,
        }
    }
    #[doc = "Checks if the value of the field is `A53_DBG_RESET2_0`"]
    #[inline]
    pub fn is_a53_dbg_reset2_0(&self) -> bool {
        *self == A53_DBG_RESET2R::A53_DBG_RESET2_0
    }
    #[doc = "Checks if the value of the field is `A53_DBG_RESET2_1`"]
    #[inline]
    pub fn is_a53_dbg_reset2_1(&self) -> bool {
        *self == A53_DBG_RESET2R::A53_DBG_RESET2_1
    }
}
#[doc = "Possible values of the field `A53_DBG_RESET3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A53_DBG_RESET3R {
    #[doc = "do not assert core3 debug reset"]
    A53_DBG_RESET3_0,
    #[doc = "assert core3 debug reset"]
    A53_DBG_RESET3_1,
}
impl A53_DBG_RESET3R {
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
            A53_DBG_RESET3R::A53_DBG_RESET3_0 => false,
            A53_DBG_RESET3R::A53_DBG_RESET3_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> A53_DBG_RESET3R {
        match value {
            false => A53_DBG_RESET3R::A53_DBG_RESET3_0,
            true => A53_DBG_RESET3R::A53_DBG_RESET3_1,
        }
    }
    #[doc = "Checks if the value of the field is `A53_DBG_RESET3_0`"]
    #[inline]
    pub fn is_a53_dbg_reset3_0(&self) -> bool {
        *self == A53_DBG_RESET3R::A53_DBG_RESET3_0
    }
    #[doc = "Checks if the value of the field is `A53_DBG_RESET3_1`"]
    #[inline]
    pub fn is_a53_dbg_reset3_1(&self) -> bool {
        *self == A53_DBG_RESET3R::A53_DBG_RESET3_1
    }
}
#[doc = "Possible values of the field `A53_ETM_RESET0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A53_ETM_RESET0R {
    #[doc = "do not assert core0 ETM reset"]
    A53_ETM_RESET0_0,
    #[doc = "assert core0 ETM reset"]
    A53_ETM_RESET0_1,
}
impl A53_ETM_RESET0R {
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
            A53_ETM_RESET0R::A53_ETM_RESET0_0 => false,
            A53_ETM_RESET0R::A53_ETM_RESET0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> A53_ETM_RESET0R {
        match value {
            false => A53_ETM_RESET0R::A53_ETM_RESET0_0,
            true => A53_ETM_RESET0R::A53_ETM_RESET0_1,
        }
    }
    #[doc = "Checks if the value of the field is `A53_ETM_RESET0_0`"]
    #[inline]
    pub fn is_a53_etm_reset0_0(&self) -> bool {
        *self == A53_ETM_RESET0R::A53_ETM_RESET0_0
    }
    #[doc = "Checks if the value of the field is `A53_ETM_RESET0_1`"]
    #[inline]
    pub fn is_a53_etm_reset0_1(&self) -> bool {
        *self == A53_ETM_RESET0R::A53_ETM_RESET0_1
    }
}
#[doc = "Possible values of the field `A53_ETM_RESET1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A53_ETM_RESET1R {
    #[doc = "do not assert core1 ETM reset"]
    A53_ETM_RESET1_0,
    #[doc = "assert core1 ETM reset"]
    A53_ETM_RESET1_1,
}
impl A53_ETM_RESET1R {
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
            A53_ETM_RESET1R::A53_ETM_RESET1_0 => false,
            A53_ETM_RESET1R::A53_ETM_RESET1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> A53_ETM_RESET1R {
        match value {
            false => A53_ETM_RESET1R::A53_ETM_RESET1_0,
            true => A53_ETM_RESET1R::A53_ETM_RESET1_1,
        }
    }
    #[doc = "Checks if the value of the field is `A53_ETM_RESET1_0`"]
    #[inline]
    pub fn is_a53_etm_reset1_0(&self) -> bool {
        *self == A53_ETM_RESET1R::A53_ETM_RESET1_0
    }
    #[doc = "Checks if the value of the field is `A53_ETM_RESET1_1`"]
    #[inline]
    pub fn is_a53_etm_reset1_1(&self) -> bool {
        *self == A53_ETM_RESET1R::A53_ETM_RESET1_1
    }
}
#[doc = "Possible values of the field `A53_ETM_RESET2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A53_ETM_RESET2R {
    #[doc = "do not assert core2 ETM reset"]
    A53_ETM_RESET2_0,
    #[doc = "assert core2 ETM reset"]
    A53_ETM_RESET2_1,
}
impl A53_ETM_RESET2R {
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
            A53_ETM_RESET2R::A53_ETM_RESET2_0 => false,
            A53_ETM_RESET2R::A53_ETM_RESET2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> A53_ETM_RESET2R {
        match value {
            false => A53_ETM_RESET2R::A53_ETM_RESET2_0,
            true => A53_ETM_RESET2R::A53_ETM_RESET2_1,
        }
    }
    #[doc = "Checks if the value of the field is `A53_ETM_RESET2_0`"]
    #[inline]
    pub fn is_a53_etm_reset2_0(&self) -> bool {
        *self == A53_ETM_RESET2R::A53_ETM_RESET2_0
    }
    #[doc = "Checks if the value of the field is `A53_ETM_RESET2_1`"]
    #[inline]
    pub fn is_a53_etm_reset2_1(&self) -> bool {
        *self == A53_ETM_RESET2R::A53_ETM_RESET2_1
    }
}
#[doc = "Possible values of the field `A53_ETM_RESET3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A53_ETM_RESET3R {
    #[doc = "do not assert core3 ETM reset"]
    A53_ETM_RESET3_0,
    #[doc = "assert core3 ETM reset"]
    A53_ETM_RESET3_1,
}
impl A53_ETM_RESET3R {
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
            A53_ETM_RESET3R::A53_ETM_RESET3_0 => false,
            A53_ETM_RESET3R::A53_ETM_RESET3_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> A53_ETM_RESET3R {
        match value {
            false => A53_ETM_RESET3R::A53_ETM_RESET3_0,
            true => A53_ETM_RESET3R::A53_ETM_RESET3_1,
        }
    }
    #[doc = "Checks if the value of the field is `A53_ETM_RESET3_0`"]
    #[inline]
    pub fn is_a53_etm_reset3_0(&self) -> bool {
        *self == A53_ETM_RESET3R::A53_ETM_RESET3_0
    }
    #[doc = "Checks if the value of the field is `A53_ETM_RESET3_1`"]
    #[inline]
    pub fn is_a53_etm_reset3_1(&self) -> bool {
        *self == A53_ETM_RESET3R::A53_ETM_RESET3_1
    }
}
#[doc = "Possible values of the field `MASK_WDOG1_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_WDOG1_RSTR {
    #[doc = "wdog1_rst_b is masked"]
    MASK_WDOG1_RST_5,
    #[doc = "wdog1_rst_b is not masked"]
    MASK_WDOG1_RST_10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MASK_WDOG1_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MASK_WDOG1_RSTR::MASK_WDOG1_RST_5 => 5,
            MASK_WDOG1_RSTR::MASK_WDOG1_RST_10 => 10,
            MASK_WDOG1_RSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MASK_WDOG1_RSTR {
        match value {
            5 => MASK_WDOG1_RSTR::MASK_WDOG1_RST_5,
            10 => MASK_WDOG1_RSTR::MASK_WDOG1_RST_10,
            i => MASK_WDOG1_RSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASK_WDOG1_RST_5`"]
    #[inline]
    pub fn is_mask_wdog1_rst_5(&self) -> bool {
        *self == MASK_WDOG1_RSTR::MASK_WDOG1_RST_5
    }
    #[doc = "Checks if the value of the field is `MASK_WDOG1_RST_10`"]
    #[inline]
    pub fn is_mask_wdog1_rst_10(&self) -> bool {
        *self == MASK_WDOG1_RSTR::MASK_WDOG1_RST_10
    }
}
#[doc = "Possible values of the field `A53_SOC_DBG_RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A53_SOC_DBG_RESETR {
    #[doc = "do not assert system level debug reset"]
    A53_SOC_DBG_RESET_0,
    #[doc = "assert system level debug reset"]
    A53_SOC_DBG_RESET_1,
}
impl A53_SOC_DBG_RESETR {
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
            A53_SOC_DBG_RESETR::A53_SOC_DBG_RESET_0 => false,
            A53_SOC_DBG_RESETR::A53_SOC_DBG_RESET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> A53_SOC_DBG_RESETR {
        match value {
            false => A53_SOC_DBG_RESETR::A53_SOC_DBG_RESET_0,
            true => A53_SOC_DBG_RESETR::A53_SOC_DBG_RESET_1,
        }
    }
    #[doc = "Checks if the value of the field is `A53_SOC_DBG_RESET_0`"]
    #[inline]
    pub fn is_a53_soc_dbg_reset_0(&self) -> bool {
        *self == A53_SOC_DBG_RESETR::A53_SOC_DBG_RESET_0
    }
    #[doc = "Checks if the value of the field is `A53_SOC_DBG_RESET_1`"]
    #[inline]
    pub fn is_a53_soc_dbg_reset_1(&self) -> bool {
        *self == A53_SOC_DBG_RESETR::A53_SOC_DBG_RESET_1
    }
}
#[doc = "Possible values of the field `A53_L2RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A53_L2RESETR {
    #[doc = "do not assert SCU reset"]
    A53_L2RESET_0,
    #[doc = "assert SCU reset"]
    A53_L2RESET_1,
}
impl A53_L2RESETR {
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
            A53_L2RESETR::A53_L2RESET_0 => false,
            A53_L2RESETR::A53_L2RESET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> A53_L2RESETR {
        match value {
            false => A53_L2RESETR::A53_L2RESET_0,
            true => A53_L2RESETR::A53_L2RESET_1,
        }
    }
    #[doc = "Checks if the value of the field is `A53_L2RESET_0`"]
    #[inline]
    pub fn is_a53_l2reset_0(&self) -> bool {
        *self == A53_L2RESETR::A53_L2RESET_0
    }
    #[doc = "Checks if the value of the field is `A53_L2RESET_1`"]
    #[inline]
    pub fn is_a53_l2reset_1(&self) -> bool {
        *self == A53_L2RESETR::A53_L2RESET_1
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
#[doc = "Values that can be written to the field `A53_CORE_POR_RESET0`"]
pub enum A53_CORE_POR_RESET0W {
    #[doc = "do not assert core0 reset"]
    A53_CORE_POR_RESET0_0,
    #[doc = "assert core0 reset"]
    A53_CORE_POR_RESET0_1,
}
impl A53_CORE_POR_RESET0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            A53_CORE_POR_RESET0W::A53_CORE_POR_RESET0_0 => false,
            A53_CORE_POR_RESET0W::A53_CORE_POR_RESET0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _A53_CORE_POR_RESET0W<'a> {
    w: &'a mut W,
}
impl<'a> _A53_CORE_POR_RESET0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: A53_CORE_POR_RESET0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not assert core0 reset"]
    #[inline]
    pub fn a53_core_por_reset0_0(self) -> &'a mut W {
        self.variant(A53_CORE_POR_RESET0W::A53_CORE_POR_RESET0_0)
    }
    #[doc = "assert core0 reset"]
    #[inline]
    pub fn a53_core_por_reset0_1(self) -> &'a mut W {
        self.variant(A53_CORE_POR_RESET0W::A53_CORE_POR_RESET0_1)
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
#[doc = "Values that can be written to the field `A53_CORE_POR_RESET1`"]
pub enum A53_CORE_POR_RESET1W {
    #[doc = "do not assert core1 reset"]
    A53_CORE_POR_RESET1_0,
    #[doc = "assert core1 reset"]
    A53_CORE_POR_RESET1_1,
}
impl A53_CORE_POR_RESET1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            A53_CORE_POR_RESET1W::A53_CORE_POR_RESET1_0 => false,
            A53_CORE_POR_RESET1W::A53_CORE_POR_RESET1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _A53_CORE_POR_RESET1W<'a> {
    w: &'a mut W,
}
impl<'a> _A53_CORE_POR_RESET1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: A53_CORE_POR_RESET1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not assert core1 reset"]
    #[inline]
    pub fn a53_core_por_reset1_0(self) -> &'a mut W {
        self.variant(A53_CORE_POR_RESET1W::A53_CORE_POR_RESET1_0)
    }
    #[doc = "assert core1 reset"]
    #[inline]
    pub fn a53_core_por_reset1_1(self) -> &'a mut W {
        self.variant(A53_CORE_POR_RESET1W::A53_CORE_POR_RESET1_1)
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
#[doc = "Values that can be written to the field `A53_CORE_POR_RESET2`"]
pub enum A53_CORE_POR_RESET2W {
    #[doc = "do not assert core2 reset"]
    A53_CORE_POR_RESET2_0,
    #[doc = "assert core2 reset"]
    A53_CORE_POR_RESET2_1,
}
impl A53_CORE_POR_RESET2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            A53_CORE_POR_RESET2W::A53_CORE_POR_RESET2_0 => false,
            A53_CORE_POR_RESET2W::A53_CORE_POR_RESET2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _A53_CORE_POR_RESET2W<'a> {
    w: &'a mut W,
}
impl<'a> _A53_CORE_POR_RESET2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: A53_CORE_POR_RESET2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not assert core2 reset"]
    #[inline]
    pub fn a53_core_por_reset2_0(self) -> &'a mut W {
        self.variant(A53_CORE_POR_RESET2W::A53_CORE_POR_RESET2_0)
    }
    #[doc = "assert core2 reset"]
    #[inline]
    pub fn a53_core_por_reset2_1(self) -> &'a mut W {
        self.variant(A53_CORE_POR_RESET2W::A53_CORE_POR_RESET2_1)
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
#[doc = "Values that can be written to the field `A53_CORE_POR_RESET3`"]
pub enum A53_CORE_POR_RESET3W {
    #[doc = "do not assert core3 reset"]
    A53_CORE_POR_RESET3_0,
    #[doc = "assert core3 reset"]
    A53_CORE_POR_RESET3_1,
}
impl A53_CORE_POR_RESET3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            A53_CORE_POR_RESET3W::A53_CORE_POR_RESET3_0 => false,
            A53_CORE_POR_RESET3W::A53_CORE_POR_RESET3_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _A53_CORE_POR_RESET3W<'a> {
    w: &'a mut W,
}
impl<'a> _A53_CORE_POR_RESET3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: A53_CORE_POR_RESET3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not assert core3 reset"]
    #[inline]
    pub fn a53_core_por_reset3_0(self) -> &'a mut W {
        self.variant(A53_CORE_POR_RESET3W::A53_CORE_POR_RESET3_0)
    }
    #[doc = "assert core3 reset"]
    #[inline]
    pub fn a53_core_por_reset3_1(self) -> &'a mut W {
        self.variant(A53_CORE_POR_RESET3W::A53_CORE_POR_RESET3_1)
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
#[doc = "Values that can be written to the field `A53_CORE_RESET0`"]
pub enum A53_CORE_RESET0W {
    #[doc = "do not assert core0 reset"]
    A53_CORE_RESET0_0,
    #[doc = "assert core0 reset"]
    A53_CORE_RESET0_1,
}
impl A53_CORE_RESET0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            A53_CORE_RESET0W::A53_CORE_RESET0_0 => false,
            A53_CORE_RESET0W::A53_CORE_RESET0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _A53_CORE_RESET0W<'a> {
    w: &'a mut W,
}
impl<'a> _A53_CORE_RESET0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: A53_CORE_RESET0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not assert core0 reset"]
    #[inline]
    pub fn a53_core_reset0_0(self) -> &'a mut W {
        self.variant(A53_CORE_RESET0W::A53_CORE_RESET0_0)
    }
    #[doc = "assert core0 reset"]
    #[inline]
    pub fn a53_core_reset0_1(self) -> &'a mut W {
        self.variant(A53_CORE_RESET0W::A53_CORE_RESET0_1)
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
#[doc = "Values that can be written to the field `A53_CORE_RESET1`"]
pub enum A53_CORE_RESET1W {
    #[doc = "do not assert core1 reset"]
    A53_CORE_RESET1_0,
    #[doc = "assert core1 reset"]
    A53_CORE_RESET1_1,
}
impl A53_CORE_RESET1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            A53_CORE_RESET1W::A53_CORE_RESET1_0 => false,
            A53_CORE_RESET1W::A53_CORE_RESET1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _A53_CORE_RESET1W<'a> {
    w: &'a mut W,
}
impl<'a> _A53_CORE_RESET1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: A53_CORE_RESET1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not assert core1 reset"]
    #[inline]
    pub fn a53_core_reset1_0(self) -> &'a mut W {
        self.variant(A53_CORE_RESET1W::A53_CORE_RESET1_0)
    }
    #[doc = "assert core1 reset"]
    #[inline]
    pub fn a53_core_reset1_1(self) -> &'a mut W {
        self.variant(A53_CORE_RESET1W::A53_CORE_RESET1_1)
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
#[doc = "Values that can be written to the field `A53_CORE_RESET2`"]
pub enum A53_CORE_RESET2W {
    #[doc = "do not assert core2 reset"]
    A53_CORE_RESET2_0,
    #[doc = "assert core2 reset"]
    A53_CORE_RESET2_1,
}
impl A53_CORE_RESET2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            A53_CORE_RESET2W::A53_CORE_RESET2_0 => false,
            A53_CORE_RESET2W::A53_CORE_RESET2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _A53_CORE_RESET2W<'a> {
    w: &'a mut W,
}
impl<'a> _A53_CORE_RESET2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: A53_CORE_RESET2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not assert core2 reset"]
    #[inline]
    pub fn a53_core_reset2_0(self) -> &'a mut W {
        self.variant(A53_CORE_RESET2W::A53_CORE_RESET2_0)
    }
    #[doc = "assert core2 reset"]
    #[inline]
    pub fn a53_core_reset2_1(self) -> &'a mut W {
        self.variant(A53_CORE_RESET2W::A53_CORE_RESET2_1)
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
#[doc = "Values that can be written to the field `A53_CORE_RESET3`"]
pub enum A53_CORE_RESET3W {
    #[doc = "do not assert core3 reset"]
    A53_CORE_RESET3_0,
    #[doc = "assert core3 reset"]
    A53_CORE_RESET3_1,
}
impl A53_CORE_RESET3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            A53_CORE_RESET3W::A53_CORE_RESET3_0 => false,
            A53_CORE_RESET3W::A53_CORE_RESET3_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _A53_CORE_RESET3W<'a> {
    w: &'a mut W,
}
impl<'a> _A53_CORE_RESET3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: A53_CORE_RESET3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not assert core3 reset"]
    #[inline]
    pub fn a53_core_reset3_0(self) -> &'a mut W {
        self.variant(A53_CORE_RESET3W::A53_CORE_RESET3_0)
    }
    #[doc = "assert core3 reset"]
    #[inline]
    pub fn a53_core_reset3_1(self) -> &'a mut W {
        self.variant(A53_CORE_RESET3W::A53_CORE_RESET3_1)
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
#[doc = "Values that can be written to the field `A53_DBG_RESET0`"]
pub enum A53_DBG_RESET0W {
    #[doc = "do not assert core0 debug reset"]
    A53_DBG_RESET0_0,
    #[doc = "assert core0 debug reset"]
    A53_DBG_RESET0_1,
}
impl A53_DBG_RESET0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            A53_DBG_RESET0W::A53_DBG_RESET0_0 => false,
            A53_DBG_RESET0W::A53_DBG_RESET0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _A53_DBG_RESET0W<'a> {
    w: &'a mut W,
}
impl<'a> _A53_DBG_RESET0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: A53_DBG_RESET0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not assert core0 debug reset"]
    #[inline]
    pub fn a53_dbg_reset0_0(self) -> &'a mut W {
        self.variant(A53_DBG_RESET0W::A53_DBG_RESET0_0)
    }
    #[doc = "assert core0 debug reset"]
    #[inline]
    pub fn a53_dbg_reset0_1(self) -> &'a mut W {
        self.variant(A53_DBG_RESET0W::A53_DBG_RESET0_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `A53_DBG_RESET1`"]
pub enum A53_DBG_RESET1W {
    #[doc = "do not assert core1 debug reset"]
    A53_DBG_RESET1_0,
    #[doc = "assert core1 debug reset"]
    A53_DBG_RESET1_1,
}
impl A53_DBG_RESET1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            A53_DBG_RESET1W::A53_DBG_RESET1_0 => false,
            A53_DBG_RESET1W::A53_DBG_RESET1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _A53_DBG_RESET1W<'a> {
    w: &'a mut W,
}
impl<'a> _A53_DBG_RESET1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: A53_DBG_RESET1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not assert core1 debug reset"]
    #[inline]
    pub fn a53_dbg_reset1_0(self) -> &'a mut W {
        self.variant(A53_DBG_RESET1W::A53_DBG_RESET1_0)
    }
    #[doc = "assert core1 debug reset"]
    #[inline]
    pub fn a53_dbg_reset1_1(self) -> &'a mut W {
        self.variant(A53_DBG_RESET1W::A53_DBG_RESET1_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `A53_DBG_RESET2`"]
pub enum A53_DBG_RESET2W {
    #[doc = "do not assert core2 debug reset"]
    A53_DBG_RESET2_0,
    #[doc = "assert core2 debug reset"]
    A53_DBG_RESET2_1,
}
impl A53_DBG_RESET2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            A53_DBG_RESET2W::A53_DBG_RESET2_0 => false,
            A53_DBG_RESET2W::A53_DBG_RESET2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _A53_DBG_RESET2W<'a> {
    w: &'a mut W,
}
impl<'a> _A53_DBG_RESET2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: A53_DBG_RESET2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not assert core2 debug reset"]
    #[inline]
    pub fn a53_dbg_reset2_0(self) -> &'a mut W {
        self.variant(A53_DBG_RESET2W::A53_DBG_RESET2_0)
    }
    #[doc = "assert core2 debug reset"]
    #[inline]
    pub fn a53_dbg_reset2_1(self) -> &'a mut W {
        self.variant(A53_DBG_RESET2W::A53_DBG_RESET2_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `A53_DBG_RESET3`"]
pub enum A53_DBG_RESET3W {
    #[doc = "do not assert core3 debug reset"]
    A53_DBG_RESET3_0,
    #[doc = "assert core3 debug reset"]
    A53_DBG_RESET3_1,
}
impl A53_DBG_RESET3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            A53_DBG_RESET3W::A53_DBG_RESET3_0 => false,
            A53_DBG_RESET3W::A53_DBG_RESET3_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _A53_DBG_RESET3W<'a> {
    w: &'a mut W,
}
impl<'a> _A53_DBG_RESET3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: A53_DBG_RESET3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not assert core3 debug reset"]
    #[inline]
    pub fn a53_dbg_reset3_0(self) -> &'a mut W {
        self.variant(A53_DBG_RESET3W::A53_DBG_RESET3_0)
    }
    #[doc = "assert core3 debug reset"]
    #[inline]
    pub fn a53_dbg_reset3_1(self) -> &'a mut W {
        self.variant(A53_DBG_RESET3W::A53_DBG_RESET3_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `A53_ETM_RESET0`"]
pub enum A53_ETM_RESET0W {
    #[doc = "do not assert core0 ETM reset"]
    A53_ETM_RESET0_0,
    #[doc = "assert core0 ETM reset"]
    A53_ETM_RESET0_1,
}
impl A53_ETM_RESET0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            A53_ETM_RESET0W::A53_ETM_RESET0_0 => false,
            A53_ETM_RESET0W::A53_ETM_RESET0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _A53_ETM_RESET0W<'a> {
    w: &'a mut W,
}
impl<'a> _A53_ETM_RESET0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: A53_ETM_RESET0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not assert core0 ETM reset"]
    #[inline]
    pub fn a53_etm_reset0_0(self) -> &'a mut W {
        self.variant(A53_ETM_RESET0W::A53_ETM_RESET0_0)
    }
    #[doc = "assert core0 ETM reset"]
    #[inline]
    pub fn a53_etm_reset0_1(self) -> &'a mut W {
        self.variant(A53_ETM_RESET0W::A53_ETM_RESET0_1)
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
#[doc = "Values that can be written to the field `A53_ETM_RESET1`"]
pub enum A53_ETM_RESET1W {
    #[doc = "do not assert core1 ETM reset"]
    A53_ETM_RESET1_0,
    #[doc = "assert core1 ETM reset"]
    A53_ETM_RESET1_1,
}
impl A53_ETM_RESET1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            A53_ETM_RESET1W::A53_ETM_RESET1_0 => false,
            A53_ETM_RESET1W::A53_ETM_RESET1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _A53_ETM_RESET1W<'a> {
    w: &'a mut W,
}
impl<'a> _A53_ETM_RESET1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: A53_ETM_RESET1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not assert core1 ETM reset"]
    #[inline]
    pub fn a53_etm_reset1_0(self) -> &'a mut W {
        self.variant(A53_ETM_RESET1W::A53_ETM_RESET1_0)
    }
    #[doc = "assert core1 ETM reset"]
    #[inline]
    pub fn a53_etm_reset1_1(self) -> &'a mut W {
        self.variant(A53_ETM_RESET1W::A53_ETM_RESET1_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `A53_ETM_RESET2`"]
pub enum A53_ETM_RESET2W {
    #[doc = "do not assert core2 ETM reset"]
    A53_ETM_RESET2_0,
    #[doc = "assert core2 ETM reset"]
    A53_ETM_RESET2_1,
}
impl A53_ETM_RESET2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            A53_ETM_RESET2W::A53_ETM_RESET2_0 => false,
            A53_ETM_RESET2W::A53_ETM_RESET2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _A53_ETM_RESET2W<'a> {
    w: &'a mut W,
}
impl<'a> _A53_ETM_RESET2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: A53_ETM_RESET2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not assert core2 ETM reset"]
    #[inline]
    pub fn a53_etm_reset2_0(self) -> &'a mut W {
        self.variant(A53_ETM_RESET2W::A53_ETM_RESET2_0)
    }
    #[doc = "assert core2 ETM reset"]
    #[inline]
    pub fn a53_etm_reset2_1(self) -> &'a mut W {
        self.variant(A53_ETM_RESET2W::A53_ETM_RESET2_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `A53_ETM_RESET3`"]
pub enum A53_ETM_RESET3W {
    #[doc = "do not assert core3 ETM reset"]
    A53_ETM_RESET3_0,
    #[doc = "assert core3 ETM reset"]
    A53_ETM_RESET3_1,
}
impl A53_ETM_RESET3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            A53_ETM_RESET3W::A53_ETM_RESET3_0 => false,
            A53_ETM_RESET3W::A53_ETM_RESET3_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _A53_ETM_RESET3W<'a> {
    w: &'a mut W,
}
impl<'a> _A53_ETM_RESET3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: A53_ETM_RESET3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not assert core3 ETM reset"]
    #[inline]
    pub fn a53_etm_reset3_0(self) -> &'a mut W {
        self.variant(A53_ETM_RESET3W::A53_ETM_RESET3_0)
    }
    #[doc = "assert core3 ETM reset"]
    #[inline]
    pub fn a53_etm_reset3_1(self) -> &'a mut W {
        self.variant(A53_ETM_RESET3W::A53_ETM_RESET3_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MASK_WDOG1_RST`"]
pub enum MASK_WDOG1_RSTW {
    #[doc = "wdog1_rst_b is masked"]
    MASK_WDOG1_RST_5,
    #[doc = "wdog1_rst_b is not masked"]
    MASK_WDOG1_RST_10,
}
impl MASK_WDOG1_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MASK_WDOG1_RSTW::MASK_WDOG1_RST_5 => 5,
            MASK_WDOG1_RSTW::MASK_WDOG1_RST_10 => 10,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_WDOG1_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_WDOG1_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_WDOG1_RSTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "wdog1_rst_b is masked"]
    #[inline]
    pub fn mask_wdog1_rst_5(self) -> &'a mut W {
        self.variant(MASK_WDOG1_RSTW::MASK_WDOG1_RST_5)
    }
    #[doc = "wdog1_rst_b is not masked"]
    #[inline]
    pub fn mask_wdog1_rst_10(self) -> &'a mut W {
        self.variant(MASK_WDOG1_RSTW::MASK_WDOG1_RST_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `A53_SOC_DBG_RESET`"]
pub enum A53_SOC_DBG_RESETW {
    #[doc = "do not assert system level debug reset"]
    A53_SOC_DBG_RESET_0,
    #[doc = "assert system level debug reset"]
    A53_SOC_DBG_RESET_1,
}
impl A53_SOC_DBG_RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            A53_SOC_DBG_RESETW::A53_SOC_DBG_RESET_0 => false,
            A53_SOC_DBG_RESETW::A53_SOC_DBG_RESET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _A53_SOC_DBG_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _A53_SOC_DBG_RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: A53_SOC_DBG_RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not assert system level debug reset"]
    #[inline]
    pub fn a53_soc_dbg_reset_0(self) -> &'a mut W {
        self.variant(A53_SOC_DBG_RESETW::A53_SOC_DBG_RESET_0)
    }
    #[doc = "assert system level debug reset"]
    #[inline]
    pub fn a53_soc_dbg_reset_1(self) -> &'a mut W {
        self.variant(A53_SOC_DBG_RESETW::A53_SOC_DBG_RESET_1)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `A53_L2RESET`"]
pub enum A53_L2RESETW {
    #[doc = "do not assert SCU reset"]
    A53_L2RESET_0,
    #[doc = "assert SCU reset"]
    A53_L2RESET_1,
}
impl A53_L2RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            A53_L2RESETW::A53_L2RESET_0 => false,
            A53_L2RESETW::A53_L2RESET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _A53_L2RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _A53_L2RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: A53_L2RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not assert SCU reset"]
    #[inline]
    pub fn a53_l2reset_0(self) -> &'a mut W {
        self.variant(A53_L2RESETW::A53_L2RESET_0)
    }
    #[doc = "assert SCU reset"]
    #[inline]
    pub fn a53_l2reset_1(self) -> &'a mut W {
        self.variant(A53_L2RESETW::A53_L2RESET_1)
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
        const OFFSET: u8 = 21;
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
    #[doc = "Bit 0 - POR reset for A53 core0 only"]
    #[inline]
    pub fn a53_core_por_reset0(&self) -> A53_CORE_POR_RESET0R {
        A53_CORE_POR_RESET0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - POR reset for A53 core1 only"]
    #[inline]
    pub fn a53_core_por_reset1(&self) -> A53_CORE_POR_RESET1R {
        A53_CORE_POR_RESET1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - POR reset for A53 core2 only"]
    #[inline]
    pub fn a53_core_por_reset2(&self) -> A53_CORE_POR_RESET2R {
        A53_CORE_POR_RESET2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - POR reset for A53 core3 only"]
    #[inline]
    pub fn a53_core_por_reset3(&self) -> A53_CORE_POR_RESET3R {
        A53_CORE_POR_RESET3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Software reset for core0 only"]
    #[inline]
    pub fn a53_core_reset0(&self) -> A53_CORE_RESET0R {
        A53_CORE_RESET0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Software reset for core1 only"]
    #[inline]
    pub fn a53_core_reset1(&self) -> A53_CORE_RESET1R {
        A53_CORE_RESET1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Software reset for core2 only"]
    #[inline]
    pub fn a53_core_reset2(&self) -> A53_CORE_RESET2R {
        A53_CORE_RESET2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Software reset for core3 only"]
    #[inline]
    pub fn a53_core_reset3(&self) -> A53_CORE_RESET3R {
        A53_CORE_RESET3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Software reset for core0 debug only"]
    #[inline]
    pub fn a53_dbg_reset0(&self) -> A53_DBG_RESET0R {
        A53_DBG_RESET0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Software reset for core1 debug only"]
    #[inline]
    pub fn a53_dbg_reset1(&self) -> A53_DBG_RESET1R {
        A53_DBG_RESET1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Software reset for core2 debug only"]
    #[inline]
    pub fn a53_dbg_reset2(&self) -> A53_DBG_RESET2R {
        A53_DBG_RESET2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Software reset for core3 debug only"]
    #[inline]
    pub fn a53_dbg_reset3(&self) -> A53_DBG_RESET3R {
        A53_DBG_RESET3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Software reset for core0 ETM only"]
    #[inline]
    pub fn a53_etm_reset0(&self) -> A53_ETM_RESET0R {
        A53_ETM_RESET0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Software reset for core1 ETM only"]
    #[inline]
    pub fn a53_etm_reset1(&self) -> A53_ETM_RESET1R {
        A53_ETM_RESET1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Software reset for core2 ETM only"]
    #[inline]
    pub fn a53_etm_reset2(&self) -> A53_ETM_RESET2R {
        A53_ETM_RESET2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Software reset for core3 ETM only"]
    #[inline]
    pub fn a53_etm_reset3(&self) -> A53_ETM_RESET3R {
        A53_ETM_RESET3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:19 - Mask wdog1_rst_b source"]
    #[inline]
    pub fn mask_wdog1_rst(&self) -> MASK_WDOG1_RSTR {
        MASK_WDOG1_RSTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Software reset for system level debug reset"]
    #[inline]
    pub fn a53_soc_dbg_reset(&self) -> A53_SOC_DBG_RESETR {
        A53_SOC_DBG_RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Software reset for A53 Snoop Control Unit (SCU)"]
    #[inline]
    pub fn a53_l2reset(&self) -> A53_L2RESETR {
        A53_L2RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
        W { bits: 655360 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - POR reset for A53 core0 only"]
    #[inline]
    pub fn a53_core_por_reset0(&mut self) -> _A53_CORE_POR_RESET0W {
        _A53_CORE_POR_RESET0W { w: self }
    }
    #[doc = "Bit 1 - POR reset for A53 core1 only"]
    #[inline]
    pub fn a53_core_por_reset1(&mut self) -> _A53_CORE_POR_RESET1W {
        _A53_CORE_POR_RESET1W { w: self }
    }
    #[doc = "Bit 2 - POR reset for A53 core2 only"]
    #[inline]
    pub fn a53_core_por_reset2(&mut self) -> _A53_CORE_POR_RESET2W {
        _A53_CORE_POR_RESET2W { w: self }
    }
    #[doc = "Bit 3 - POR reset for A53 core3 only"]
    #[inline]
    pub fn a53_core_por_reset3(&mut self) -> _A53_CORE_POR_RESET3W {
        _A53_CORE_POR_RESET3W { w: self }
    }
    #[doc = "Bit 4 - Software reset for core0 only"]
    #[inline]
    pub fn a53_core_reset0(&mut self) -> _A53_CORE_RESET0W {
        _A53_CORE_RESET0W { w: self }
    }
    #[doc = "Bit 5 - Software reset for core1 only"]
    #[inline]
    pub fn a53_core_reset1(&mut self) -> _A53_CORE_RESET1W {
        _A53_CORE_RESET1W { w: self }
    }
    #[doc = "Bit 6 - Software reset for core2 only"]
    #[inline]
    pub fn a53_core_reset2(&mut self) -> _A53_CORE_RESET2W {
        _A53_CORE_RESET2W { w: self }
    }
    #[doc = "Bit 7 - Software reset for core3 only"]
    #[inline]
    pub fn a53_core_reset3(&mut self) -> _A53_CORE_RESET3W {
        _A53_CORE_RESET3W { w: self }
    }
    #[doc = "Bit 8 - Software reset for core0 debug only"]
    #[inline]
    pub fn a53_dbg_reset0(&mut self) -> _A53_DBG_RESET0W {
        _A53_DBG_RESET0W { w: self }
    }
    #[doc = "Bit 9 - Software reset for core1 debug only"]
    #[inline]
    pub fn a53_dbg_reset1(&mut self) -> _A53_DBG_RESET1W {
        _A53_DBG_RESET1W { w: self }
    }
    #[doc = "Bit 10 - Software reset for core2 debug only"]
    #[inline]
    pub fn a53_dbg_reset2(&mut self) -> _A53_DBG_RESET2W {
        _A53_DBG_RESET2W { w: self }
    }
    #[doc = "Bit 11 - Software reset for core3 debug only"]
    #[inline]
    pub fn a53_dbg_reset3(&mut self) -> _A53_DBG_RESET3W {
        _A53_DBG_RESET3W { w: self }
    }
    #[doc = "Bit 12 - Software reset for core0 ETM only"]
    #[inline]
    pub fn a53_etm_reset0(&mut self) -> _A53_ETM_RESET0W {
        _A53_ETM_RESET0W { w: self }
    }
    #[doc = "Bit 13 - Software reset for core1 ETM only"]
    #[inline]
    pub fn a53_etm_reset1(&mut self) -> _A53_ETM_RESET1W {
        _A53_ETM_RESET1W { w: self }
    }
    #[doc = "Bit 14 - Software reset for core2 ETM only"]
    #[inline]
    pub fn a53_etm_reset2(&mut self) -> _A53_ETM_RESET2W {
        _A53_ETM_RESET2W { w: self }
    }
    #[doc = "Bit 15 - Software reset for core3 ETM only"]
    #[inline]
    pub fn a53_etm_reset3(&mut self) -> _A53_ETM_RESET3W {
        _A53_ETM_RESET3W { w: self }
    }
    #[doc = "Bits 16:19 - Mask wdog1_rst_b source"]
    #[inline]
    pub fn mask_wdog1_rst(&mut self) -> _MASK_WDOG1_RSTW {
        _MASK_WDOG1_RSTW { w: self }
    }
    #[doc = "Bit 20 - Software reset for system level debug reset"]
    #[inline]
    pub fn a53_soc_dbg_reset(&mut self) -> _A53_SOC_DBG_RESETW {
        _A53_SOC_DBG_RESETW { w: self }
    }
    #[doc = "Bit 21 - Software reset for A53 Snoop Control Unit (SCU)"]
    #[inline]
    pub fn a53_l2reset(&mut self) -> _A53_L2RESETW {
        _A53_L2RESETW { w: self }
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
