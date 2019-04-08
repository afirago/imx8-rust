#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL2_TOG {
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
pub struct CH0_ERROR_IRQR {
    bits: bool,
}
impl CH0_ERROR_IRQR {
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
pub struct CH1_ERROR_IRQR {
    bits: bool,
}
impl CH1_ERROR_IRQR {
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
pub struct CH2_ERROR_IRQR {
    bits: bool,
}
impl CH2_ERROR_IRQR {
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
pub struct CH3_ERROR_IRQR {
    bits: bool,
}
impl CH3_ERROR_IRQR {
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
pub struct CH4_ERROR_IRQR {
    bits: bool,
}
impl CH4_ERROR_IRQR {
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
pub struct CH5_ERROR_IRQR {
    bits: bool,
}
impl CH5_ERROR_IRQR {
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
pub struct CH6_ERROR_IRQR {
    bits: bool,
}
impl CH6_ERROR_IRQR {
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
pub struct CH7_ERROR_IRQR {
    bits: bool,
}
impl CH7_ERROR_IRQR {
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
pub struct CH8_ERROR_IRQR {
    bits: bool,
}
impl CH8_ERROR_IRQR {
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
pub struct CH9_ERROR_IRQR {
    bits: bool,
}
impl CH9_ERROR_IRQR {
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
pub struct CH10_ERROR_IRQR {
    bits: bool,
}
impl CH10_ERROR_IRQR {
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
pub struct CH11_ERROR_IRQR {
    bits: bool,
}
impl CH11_ERROR_IRQR {
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
pub struct CH12_ERROR_IRQR {
    bits: bool,
}
impl CH12_ERROR_IRQR {
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
pub struct CH13_ERROR_IRQR {
    bits: bool,
}
impl CH13_ERROR_IRQR {
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
pub struct CH14_ERROR_IRQR {
    bits: bool,
}
impl CH14_ERROR_IRQR {
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
pub struct CH15_ERROR_IRQR {
    bits: bool,
}
impl CH15_ERROR_IRQR {
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
#[doc = "Possible values of the field `CH0_ERROR_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0_ERROR_STATUSR {
    #[doc = "An early termination from the device causes error IRQ."]
    TERMINATION,
    #[doc = "An AHB bus error causes error IRQ."]
    BUS_ERROR,
}
impl CH0_ERROR_STATUSR {
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
            CH0_ERROR_STATUSR::TERMINATION => false,
            CH0_ERROR_STATUSR::BUS_ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH0_ERROR_STATUSR {
        match value {
            false => CH0_ERROR_STATUSR::TERMINATION,
            true => CH0_ERROR_STATUSR::BUS_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `TERMINATION`"]
    #[inline]
    pub fn is_termination(&self) -> bool {
        *self == CH0_ERROR_STATUSR::TERMINATION
    }
    #[doc = "Checks if the value of the field is `BUS_ERROR`"]
    #[inline]
    pub fn is_bus_error(&self) -> bool {
        *self == CH0_ERROR_STATUSR::BUS_ERROR
    }
}
#[doc = "Possible values of the field `CH1_ERROR_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1_ERROR_STATUSR {
    #[doc = "An early termination from the device causes error IRQ."]
    TERMINATION,
    #[doc = "An AHB bus error causes error IRQ."]
    BUS_ERROR,
}
impl CH1_ERROR_STATUSR {
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
            CH1_ERROR_STATUSR::TERMINATION => false,
            CH1_ERROR_STATUSR::BUS_ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH1_ERROR_STATUSR {
        match value {
            false => CH1_ERROR_STATUSR::TERMINATION,
            true => CH1_ERROR_STATUSR::BUS_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `TERMINATION`"]
    #[inline]
    pub fn is_termination(&self) -> bool {
        *self == CH1_ERROR_STATUSR::TERMINATION
    }
    #[doc = "Checks if the value of the field is `BUS_ERROR`"]
    #[inline]
    pub fn is_bus_error(&self) -> bool {
        *self == CH1_ERROR_STATUSR::BUS_ERROR
    }
}
#[doc = "Possible values of the field `CH2_ERROR_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2_ERROR_STATUSR {
    #[doc = "An early termination from the device causes error IRQ."]
    TERMINATION,
    #[doc = "An AHB bus error causes error IRQ."]
    BUS_ERROR,
}
impl CH2_ERROR_STATUSR {
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
            CH2_ERROR_STATUSR::TERMINATION => false,
            CH2_ERROR_STATUSR::BUS_ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH2_ERROR_STATUSR {
        match value {
            false => CH2_ERROR_STATUSR::TERMINATION,
            true => CH2_ERROR_STATUSR::BUS_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `TERMINATION`"]
    #[inline]
    pub fn is_termination(&self) -> bool {
        *self == CH2_ERROR_STATUSR::TERMINATION
    }
    #[doc = "Checks if the value of the field is `BUS_ERROR`"]
    #[inline]
    pub fn is_bus_error(&self) -> bool {
        *self == CH2_ERROR_STATUSR::BUS_ERROR
    }
}
#[doc = "Possible values of the field `CH3_ERROR_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3_ERROR_STATUSR {
    #[doc = "An early termination from the device causes error IRQ."]
    TERMINATION,
    #[doc = "An AHB bus error causes error IRQ."]
    BUS_ERROR,
}
impl CH3_ERROR_STATUSR {
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
            CH3_ERROR_STATUSR::TERMINATION => false,
            CH3_ERROR_STATUSR::BUS_ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH3_ERROR_STATUSR {
        match value {
            false => CH3_ERROR_STATUSR::TERMINATION,
            true => CH3_ERROR_STATUSR::BUS_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `TERMINATION`"]
    #[inline]
    pub fn is_termination(&self) -> bool {
        *self == CH3_ERROR_STATUSR::TERMINATION
    }
    #[doc = "Checks if the value of the field is `BUS_ERROR`"]
    #[inline]
    pub fn is_bus_error(&self) -> bool {
        *self == CH3_ERROR_STATUSR::BUS_ERROR
    }
}
#[doc = "Possible values of the field `CH4_ERROR_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4_ERROR_STATUSR {
    #[doc = "An early termination from the device causes error IRQ."]
    TERMINATION,
    #[doc = "An AHB bus error causes error IRQ."]
    BUS_ERROR,
}
impl CH4_ERROR_STATUSR {
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
            CH4_ERROR_STATUSR::TERMINATION => false,
            CH4_ERROR_STATUSR::BUS_ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH4_ERROR_STATUSR {
        match value {
            false => CH4_ERROR_STATUSR::TERMINATION,
            true => CH4_ERROR_STATUSR::BUS_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `TERMINATION`"]
    #[inline]
    pub fn is_termination(&self) -> bool {
        *self == CH4_ERROR_STATUSR::TERMINATION
    }
    #[doc = "Checks if the value of the field is `BUS_ERROR`"]
    #[inline]
    pub fn is_bus_error(&self) -> bool {
        *self == CH4_ERROR_STATUSR::BUS_ERROR
    }
}
#[doc = "Possible values of the field `CH5_ERROR_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5_ERROR_STATUSR {
    #[doc = "An early termination from the device causes error IRQ."]
    TERMINATION,
    #[doc = "An AHB bus error causes error IRQ."]
    BUS_ERROR,
}
impl CH5_ERROR_STATUSR {
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
            CH5_ERROR_STATUSR::TERMINATION => false,
            CH5_ERROR_STATUSR::BUS_ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH5_ERROR_STATUSR {
        match value {
            false => CH5_ERROR_STATUSR::TERMINATION,
            true => CH5_ERROR_STATUSR::BUS_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `TERMINATION`"]
    #[inline]
    pub fn is_termination(&self) -> bool {
        *self == CH5_ERROR_STATUSR::TERMINATION
    }
    #[doc = "Checks if the value of the field is `BUS_ERROR`"]
    #[inline]
    pub fn is_bus_error(&self) -> bool {
        *self == CH5_ERROR_STATUSR::BUS_ERROR
    }
}
#[doc = "Possible values of the field `CH6_ERROR_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6_ERROR_STATUSR {
    #[doc = "An early termination from the device causes error IRQ."]
    TERMINATION,
    #[doc = "An AHB bus error causes error IRQ."]
    BUS_ERROR,
}
impl CH6_ERROR_STATUSR {
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
            CH6_ERROR_STATUSR::TERMINATION => false,
            CH6_ERROR_STATUSR::BUS_ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH6_ERROR_STATUSR {
        match value {
            false => CH6_ERROR_STATUSR::TERMINATION,
            true => CH6_ERROR_STATUSR::BUS_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `TERMINATION`"]
    #[inline]
    pub fn is_termination(&self) -> bool {
        *self == CH6_ERROR_STATUSR::TERMINATION
    }
    #[doc = "Checks if the value of the field is `BUS_ERROR`"]
    #[inline]
    pub fn is_bus_error(&self) -> bool {
        *self == CH6_ERROR_STATUSR::BUS_ERROR
    }
}
#[doc = "Possible values of the field `CH7_ERROR_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7_ERROR_STATUSR {
    #[doc = "An early termination from the device causes error IRQ."]
    TERMINATION,
    #[doc = "An AHB bus error causes error IRQ."]
    BUS_ERROR,
}
impl CH7_ERROR_STATUSR {
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
            CH7_ERROR_STATUSR::TERMINATION => false,
            CH7_ERROR_STATUSR::BUS_ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH7_ERROR_STATUSR {
        match value {
            false => CH7_ERROR_STATUSR::TERMINATION,
            true => CH7_ERROR_STATUSR::BUS_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `TERMINATION`"]
    #[inline]
    pub fn is_termination(&self) -> bool {
        *self == CH7_ERROR_STATUSR::TERMINATION
    }
    #[doc = "Checks if the value of the field is `BUS_ERROR`"]
    #[inline]
    pub fn is_bus_error(&self) -> bool {
        *self == CH7_ERROR_STATUSR::BUS_ERROR
    }
}
#[doc = "Possible values of the field `CH8_ERROR_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH8_ERROR_STATUSR {
    #[doc = "An early termination from the device causes error IRQ."]
    TERMINATION,
    #[doc = "An AHB bus error causes error IRQ."]
    BUS_ERROR,
}
impl CH8_ERROR_STATUSR {
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
            CH8_ERROR_STATUSR::TERMINATION => false,
            CH8_ERROR_STATUSR::BUS_ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH8_ERROR_STATUSR {
        match value {
            false => CH8_ERROR_STATUSR::TERMINATION,
            true => CH8_ERROR_STATUSR::BUS_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `TERMINATION`"]
    #[inline]
    pub fn is_termination(&self) -> bool {
        *self == CH8_ERROR_STATUSR::TERMINATION
    }
    #[doc = "Checks if the value of the field is `BUS_ERROR`"]
    #[inline]
    pub fn is_bus_error(&self) -> bool {
        *self == CH8_ERROR_STATUSR::BUS_ERROR
    }
}
#[doc = "Possible values of the field `CH9_ERROR_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH9_ERROR_STATUSR {
    #[doc = "An early termination from the device causes error IRQ."]
    TERMINATION,
    #[doc = "An AHB bus error causes error IRQ."]
    BUS_ERROR,
}
impl CH9_ERROR_STATUSR {
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
            CH9_ERROR_STATUSR::TERMINATION => false,
            CH9_ERROR_STATUSR::BUS_ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH9_ERROR_STATUSR {
        match value {
            false => CH9_ERROR_STATUSR::TERMINATION,
            true => CH9_ERROR_STATUSR::BUS_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `TERMINATION`"]
    #[inline]
    pub fn is_termination(&self) -> bool {
        *self == CH9_ERROR_STATUSR::TERMINATION
    }
    #[doc = "Checks if the value of the field is `BUS_ERROR`"]
    #[inline]
    pub fn is_bus_error(&self) -> bool {
        *self == CH9_ERROR_STATUSR::BUS_ERROR
    }
}
#[doc = "Possible values of the field `CH10_ERROR_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH10_ERROR_STATUSR {
    #[doc = "An early termination from the device causes error IRQ."]
    TERMINATION,
    #[doc = "An AHB bus error causes error IRQ."]
    BUS_ERROR,
}
impl CH10_ERROR_STATUSR {
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
            CH10_ERROR_STATUSR::TERMINATION => false,
            CH10_ERROR_STATUSR::BUS_ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH10_ERROR_STATUSR {
        match value {
            false => CH10_ERROR_STATUSR::TERMINATION,
            true => CH10_ERROR_STATUSR::BUS_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `TERMINATION`"]
    #[inline]
    pub fn is_termination(&self) -> bool {
        *self == CH10_ERROR_STATUSR::TERMINATION
    }
    #[doc = "Checks if the value of the field is `BUS_ERROR`"]
    #[inline]
    pub fn is_bus_error(&self) -> bool {
        *self == CH10_ERROR_STATUSR::BUS_ERROR
    }
}
#[doc = "Possible values of the field `CH11_ERROR_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH11_ERROR_STATUSR {
    #[doc = "An early termination from the device causes error IRQ."]
    TERMINATION,
    #[doc = "An AHB bus error causes error IRQ."]
    BUS_ERROR,
}
impl CH11_ERROR_STATUSR {
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
            CH11_ERROR_STATUSR::TERMINATION => false,
            CH11_ERROR_STATUSR::BUS_ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH11_ERROR_STATUSR {
        match value {
            false => CH11_ERROR_STATUSR::TERMINATION,
            true => CH11_ERROR_STATUSR::BUS_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `TERMINATION`"]
    #[inline]
    pub fn is_termination(&self) -> bool {
        *self == CH11_ERROR_STATUSR::TERMINATION
    }
    #[doc = "Checks if the value of the field is `BUS_ERROR`"]
    #[inline]
    pub fn is_bus_error(&self) -> bool {
        *self == CH11_ERROR_STATUSR::BUS_ERROR
    }
}
#[doc = "Possible values of the field `CH12_ERROR_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH12_ERROR_STATUSR {
    #[doc = "An early termination from the device causes error IRQ."]
    TERMINATION,
    #[doc = "An AHB bus error causes error IRQ."]
    BUS_ERROR,
}
impl CH12_ERROR_STATUSR {
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
            CH12_ERROR_STATUSR::TERMINATION => false,
            CH12_ERROR_STATUSR::BUS_ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH12_ERROR_STATUSR {
        match value {
            false => CH12_ERROR_STATUSR::TERMINATION,
            true => CH12_ERROR_STATUSR::BUS_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `TERMINATION`"]
    #[inline]
    pub fn is_termination(&self) -> bool {
        *self == CH12_ERROR_STATUSR::TERMINATION
    }
    #[doc = "Checks if the value of the field is `BUS_ERROR`"]
    #[inline]
    pub fn is_bus_error(&self) -> bool {
        *self == CH12_ERROR_STATUSR::BUS_ERROR
    }
}
#[doc = "Possible values of the field `CH13_ERROR_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH13_ERROR_STATUSR {
    #[doc = "An early termination from the device causes error IRQ."]
    TERMINATION,
    #[doc = "An AHB bus error causes error IRQ."]
    BUS_ERROR,
}
impl CH13_ERROR_STATUSR {
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
            CH13_ERROR_STATUSR::TERMINATION => false,
            CH13_ERROR_STATUSR::BUS_ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH13_ERROR_STATUSR {
        match value {
            false => CH13_ERROR_STATUSR::TERMINATION,
            true => CH13_ERROR_STATUSR::BUS_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `TERMINATION`"]
    #[inline]
    pub fn is_termination(&self) -> bool {
        *self == CH13_ERROR_STATUSR::TERMINATION
    }
    #[doc = "Checks if the value of the field is `BUS_ERROR`"]
    #[inline]
    pub fn is_bus_error(&self) -> bool {
        *self == CH13_ERROR_STATUSR::BUS_ERROR
    }
}
#[doc = "Possible values of the field `CH14_ERROR_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH14_ERROR_STATUSR {
    #[doc = "An early termination from the device causes error IRQ."]
    TERMINATION,
    #[doc = "An AHB bus error causes error IRQ."]
    BUS_ERROR,
}
impl CH14_ERROR_STATUSR {
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
            CH14_ERROR_STATUSR::TERMINATION => false,
            CH14_ERROR_STATUSR::BUS_ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH14_ERROR_STATUSR {
        match value {
            false => CH14_ERROR_STATUSR::TERMINATION,
            true => CH14_ERROR_STATUSR::BUS_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `TERMINATION`"]
    #[inline]
    pub fn is_termination(&self) -> bool {
        *self == CH14_ERROR_STATUSR::TERMINATION
    }
    #[doc = "Checks if the value of the field is `BUS_ERROR`"]
    #[inline]
    pub fn is_bus_error(&self) -> bool {
        *self == CH14_ERROR_STATUSR::BUS_ERROR
    }
}
#[doc = "Possible values of the field `CH15_ERROR_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH15_ERROR_STATUSR {
    #[doc = "An early termination from the device causes error IRQ."]
    TERMINATION,
    #[doc = "An AHB bus error causes error IRQ."]
    BUS_ERROR,
}
impl CH15_ERROR_STATUSR {
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
            CH15_ERROR_STATUSR::TERMINATION => false,
            CH15_ERROR_STATUSR::BUS_ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH15_ERROR_STATUSR {
        match value {
            false => CH15_ERROR_STATUSR::TERMINATION,
            true => CH15_ERROR_STATUSR::BUS_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `TERMINATION`"]
    #[inline]
    pub fn is_termination(&self) -> bool {
        *self == CH15_ERROR_STATUSR::TERMINATION
    }
    #[doc = "Checks if the value of the field is `BUS_ERROR`"]
    #[inline]
    pub fn is_bus_error(&self) -> bool {
        *self == CH15_ERROR_STATUSR::BUS_ERROR
    }
}
#[doc = r" Proxy"]
pub struct _CH0_ERROR_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0_ERROR_IRQW<'a> {
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
#[doc = r" Proxy"]
pub struct _CH1_ERROR_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1_ERROR_IRQW<'a> {
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
#[doc = r" Proxy"]
pub struct _CH2_ERROR_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2_ERROR_IRQW<'a> {
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
#[doc = r" Proxy"]
pub struct _CH3_ERROR_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3_ERROR_IRQW<'a> {
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
pub struct _CH4_ERROR_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4_ERROR_IRQW<'a> {
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
#[doc = r" Proxy"]
pub struct _CH5_ERROR_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5_ERROR_IRQW<'a> {
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
#[doc = r" Proxy"]
pub struct _CH6_ERROR_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6_ERROR_IRQW<'a> {
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
#[doc = r" Proxy"]
pub struct _CH7_ERROR_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH7_ERROR_IRQW<'a> {
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
#[doc = r" Proxy"]
pub struct _CH8_ERROR_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH8_ERROR_IRQW<'a> {
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
#[doc = r" Proxy"]
pub struct _CH9_ERROR_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH9_ERROR_IRQW<'a> {
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
#[doc = r" Proxy"]
pub struct _CH10_ERROR_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH10_ERROR_IRQW<'a> {
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
#[doc = r" Proxy"]
pub struct _CH11_ERROR_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH11_ERROR_IRQW<'a> {
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
#[doc = r" Proxy"]
pub struct _CH12_ERROR_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH12_ERROR_IRQW<'a> {
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
#[doc = r" Proxy"]
pub struct _CH13_ERROR_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH13_ERROR_IRQW<'a> {
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
#[doc = r" Proxy"]
pub struct _CH14_ERROR_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH14_ERROR_IRQW<'a> {
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
#[doc = r" Proxy"]
pub struct _CH15_ERROR_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH15_ERROR_IRQW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Error interrupt status bit for APBX DMA Channel 0"]
    #[inline]
    pub fn ch0_error_irq(&self) -> CH0_ERROR_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH0_ERROR_IRQR { bits }
    }
    #[doc = "Bit 1 - Error interrupt status bit for APBX DMA Channel 1"]
    #[inline]
    pub fn ch1_error_irq(&self) -> CH1_ERROR_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH1_ERROR_IRQR { bits }
    }
    #[doc = "Bit 2 - Error interrupt status bit for APBX DMA Channel 2"]
    #[inline]
    pub fn ch2_error_irq(&self) -> CH2_ERROR_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH2_ERROR_IRQR { bits }
    }
    #[doc = "Bit 3 - Error interrupt status bit for APBX DMA Channel 3"]
    #[inline]
    pub fn ch3_error_irq(&self) -> CH3_ERROR_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH3_ERROR_IRQR { bits }
    }
    #[doc = "Bit 4 - Error interrupt status bit for APBX DMA Channel 4"]
    #[inline]
    pub fn ch4_error_irq(&self) -> CH4_ERROR_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH4_ERROR_IRQR { bits }
    }
    #[doc = "Bit 5 - Error interrupt status bit for APBX DMA Channel 5"]
    #[inline]
    pub fn ch5_error_irq(&self) -> CH5_ERROR_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH5_ERROR_IRQR { bits }
    }
    #[doc = "Bit 6 - Error interrupt status bit for APBX DMA Channel 6"]
    #[inline]
    pub fn ch6_error_irq(&self) -> CH6_ERROR_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH6_ERROR_IRQR { bits }
    }
    #[doc = "Bit 7 - Error interrupt status bit for APBX DMA Channel 7"]
    #[inline]
    pub fn ch7_error_irq(&self) -> CH7_ERROR_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH7_ERROR_IRQR { bits }
    }
    #[doc = "Bit 8 - Error interrupt status bit for APBH DMA Channel 8"]
    #[inline]
    pub fn ch8_error_irq(&self) -> CH8_ERROR_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH8_ERROR_IRQR { bits }
    }
    #[doc = "Bit 9 - Error interrupt status bit for APBH DMA Channel 9"]
    #[inline]
    pub fn ch9_error_irq(&self) -> CH9_ERROR_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH9_ERROR_IRQR { bits }
    }
    #[doc = "Bit 10 - Error interrupt status bit for APBH DMA Channel 10"]
    #[inline]
    pub fn ch10_error_irq(&self) -> CH10_ERROR_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH10_ERROR_IRQR { bits }
    }
    #[doc = "Bit 11 - Error interrupt status bit for APBH DMA Channel 11"]
    #[inline]
    pub fn ch11_error_irq(&self) -> CH11_ERROR_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH11_ERROR_IRQR { bits }
    }
    #[doc = "Bit 12 - Error interrupt status bit for APBH DMA Channel 12"]
    #[inline]
    pub fn ch12_error_irq(&self) -> CH12_ERROR_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH12_ERROR_IRQR { bits }
    }
    #[doc = "Bit 13 - Error interrupt status bit for APBH DMA Channel 13"]
    #[inline]
    pub fn ch13_error_irq(&self) -> CH13_ERROR_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH13_ERROR_IRQR { bits }
    }
    #[doc = "Bit 14 - Error interrupt status bit for APBH DMA Channel 14"]
    #[inline]
    pub fn ch14_error_irq(&self) -> CH14_ERROR_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH14_ERROR_IRQR { bits }
    }
    #[doc = "Bit 15 - Error interrupt status bit for APBH DMA Channel 15"]
    #[inline]
    pub fn ch15_error_irq(&self) -> CH15_ERROR_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH15_ERROR_IRQR { bits }
    }
    #[doc = "Bit 16 - Error status bit for APBX DMA Channel 0"]
    #[inline]
    pub fn ch0_error_status(&self) -> CH0_ERROR_STATUSR {
        CH0_ERROR_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Error status bit for APBX DMA Channel 1"]
    #[inline]
    pub fn ch1_error_status(&self) -> CH1_ERROR_STATUSR {
        CH1_ERROR_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Error status bit for APBX DMA Channel 2"]
    #[inline]
    pub fn ch2_error_status(&self) -> CH2_ERROR_STATUSR {
        CH2_ERROR_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Error status bit for APBX DMA Channel 3"]
    #[inline]
    pub fn ch3_error_status(&self) -> CH3_ERROR_STATUSR {
        CH3_ERROR_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Error status bit for APBX DMA Channel 4"]
    #[inline]
    pub fn ch4_error_status(&self) -> CH4_ERROR_STATUSR {
        CH4_ERROR_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Error status bit for APBX DMA Channel 5"]
    #[inline]
    pub fn ch5_error_status(&self) -> CH5_ERROR_STATUSR {
        CH5_ERROR_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Error status bit for APBX DMA Channel 6"]
    #[inline]
    pub fn ch6_error_status(&self) -> CH6_ERROR_STATUSR {
        CH6_ERROR_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Error status bit for APBX DMA Channel 7"]
    #[inline]
    pub fn ch7_error_status(&self) -> CH7_ERROR_STATUSR {
        CH7_ERROR_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Error status bit for APBH DMA Channel 8"]
    #[inline]
    pub fn ch8_error_status(&self) -> CH8_ERROR_STATUSR {
        CH8_ERROR_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Error status bit for APBH DMA Channel 9"]
    #[inline]
    pub fn ch9_error_status(&self) -> CH9_ERROR_STATUSR {
        CH9_ERROR_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Error status bit for APBH DMA Channel 10"]
    #[inline]
    pub fn ch10_error_status(&self) -> CH10_ERROR_STATUSR {
        CH10_ERROR_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Error status bit for APBH DMA Channel 11"]
    #[inline]
    pub fn ch11_error_status(&self) -> CH11_ERROR_STATUSR {
        CH11_ERROR_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Error status bit for APBH DMA Channel 12"]
    #[inline]
    pub fn ch12_error_status(&self) -> CH12_ERROR_STATUSR {
        CH12_ERROR_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Error status bit for APBH DMA Channel 13"]
    #[inline]
    pub fn ch13_error_status(&self) -> CH13_ERROR_STATUSR {
        CH13_ERROR_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Error status bit for APBH DMA Channel 14"]
    #[inline]
    pub fn ch14_error_status(&self) -> CH14_ERROR_STATUSR {
        CH14_ERROR_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Error status bit for APBH DMA Channel 15"]
    #[inline]
    pub fn ch15_error_status(&self) -> CH15_ERROR_STATUSR {
        CH15_ERROR_STATUSR::_from({
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
    #[doc = "Bit 0 - Error interrupt status bit for APBX DMA Channel 0"]
    #[inline]
    pub fn ch0_error_irq(&mut self) -> _CH0_ERROR_IRQW {
        _CH0_ERROR_IRQW { w: self }
    }
    #[doc = "Bit 1 - Error interrupt status bit for APBX DMA Channel 1"]
    #[inline]
    pub fn ch1_error_irq(&mut self) -> _CH1_ERROR_IRQW {
        _CH1_ERROR_IRQW { w: self }
    }
    #[doc = "Bit 2 - Error interrupt status bit for APBX DMA Channel 2"]
    #[inline]
    pub fn ch2_error_irq(&mut self) -> _CH2_ERROR_IRQW {
        _CH2_ERROR_IRQW { w: self }
    }
    #[doc = "Bit 3 - Error interrupt status bit for APBX DMA Channel 3"]
    #[inline]
    pub fn ch3_error_irq(&mut self) -> _CH3_ERROR_IRQW {
        _CH3_ERROR_IRQW { w: self }
    }
    #[doc = "Bit 4 - Error interrupt status bit for APBX DMA Channel 4"]
    #[inline]
    pub fn ch4_error_irq(&mut self) -> _CH4_ERROR_IRQW {
        _CH4_ERROR_IRQW { w: self }
    }
    #[doc = "Bit 5 - Error interrupt status bit for APBX DMA Channel 5"]
    #[inline]
    pub fn ch5_error_irq(&mut self) -> _CH5_ERROR_IRQW {
        _CH5_ERROR_IRQW { w: self }
    }
    #[doc = "Bit 6 - Error interrupt status bit for APBX DMA Channel 6"]
    #[inline]
    pub fn ch6_error_irq(&mut self) -> _CH6_ERROR_IRQW {
        _CH6_ERROR_IRQW { w: self }
    }
    #[doc = "Bit 7 - Error interrupt status bit for APBX DMA Channel 7"]
    #[inline]
    pub fn ch7_error_irq(&mut self) -> _CH7_ERROR_IRQW {
        _CH7_ERROR_IRQW { w: self }
    }
    #[doc = "Bit 8 - Error interrupt status bit for APBH DMA Channel 8"]
    #[inline]
    pub fn ch8_error_irq(&mut self) -> _CH8_ERROR_IRQW {
        _CH8_ERROR_IRQW { w: self }
    }
    #[doc = "Bit 9 - Error interrupt status bit for APBH DMA Channel 9"]
    #[inline]
    pub fn ch9_error_irq(&mut self) -> _CH9_ERROR_IRQW {
        _CH9_ERROR_IRQW { w: self }
    }
    #[doc = "Bit 10 - Error interrupt status bit for APBH DMA Channel 10"]
    #[inline]
    pub fn ch10_error_irq(&mut self) -> _CH10_ERROR_IRQW {
        _CH10_ERROR_IRQW { w: self }
    }
    #[doc = "Bit 11 - Error interrupt status bit for APBH DMA Channel 11"]
    #[inline]
    pub fn ch11_error_irq(&mut self) -> _CH11_ERROR_IRQW {
        _CH11_ERROR_IRQW { w: self }
    }
    #[doc = "Bit 12 - Error interrupt status bit for APBH DMA Channel 12"]
    #[inline]
    pub fn ch12_error_irq(&mut self) -> _CH12_ERROR_IRQW {
        _CH12_ERROR_IRQW { w: self }
    }
    #[doc = "Bit 13 - Error interrupt status bit for APBH DMA Channel 13"]
    #[inline]
    pub fn ch13_error_irq(&mut self) -> _CH13_ERROR_IRQW {
        _CH13_ERROR_IRQW { w: self }
    }
    #[doc = "Bit 14 - Error interrupt status bit for APBH DMA Channel 14"]
    #[inline]
    pub fn ch14_error_irq(&mut self) -> _CH14_ERROR_IRQW {
        _CH14_ERROR_IRQW { w: self }
    }
    #[doc = "Bit 15 - Error interrupt status bit for APBH DMA Channel 15"]
    #[inline]
    pub fn ch15_error_irq(&mut self) -> _CH15_ERROR_IRQW {
        _CH15_ERROR_IRQW { w: self }
    }
}
