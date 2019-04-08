#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::UCR1 {
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
#[doc = "Possible values of the field `UARTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UARTENR {
    #[doc = "Disable the UART"]
    UARTEN_0,
    #[doc = "Enable the UART"]
    UARTEN_1,
}
impl UARTENR {
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
            UARTENR::UARTEN_0 => false,
            UARTENR::UARTEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UARTENR {
        match value {
            false => UARTENR::UARTEN_0,
            true => UARTENR::UARTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `UARTEN_0`"]
    #[inline]
    pub fn is_uarten_0(&self) -> bool {
        *self == UARTENR::UARTEN_0
    }
    #[doc = "Checks if the value of the field is `UARTEN_1`"]
    #[inline]
    pub fn is_uarten_1(&self) -> bool {
        *self == UARTENR::UARTEN_1
    }
}
#[doc = "Possible values of the field `DOZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZER {
    #[doc = "The UART is enabled when in DOZE state"]
    DOZE_0,
    #[doc = "The UART is disabled when in DOZE state"]
    DOZE_1,
}
impl DOZER {
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
            DOZER::DOZE_0 => false,
            DOZER::DOZE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOZER {
        match value {
            false => DOZER::DOZE_0,
            true => DOZER::DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOZE_0`"]
    #[inline]
    pub fn is_doze_0(&self) -> bool {
        *self == DOZER::DOZE_0
    }
    #[doc = "Checks if the value of the field is `DOZE_1`"]
    #[inline]
    pub fn is_doze_1(&self) -> bool {
        *self == DOZER::DOZE_1
    }
}
#[doc = "Possible values of the field `ATDMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATDMAENR {
    #[doc = "Disable AGTIM DMA request"]
    ATDMAEN_0,
    #[doc = "Enable AGTIM DMA request"]
    ATDMAEN_1,
}
impl ATDMAENR {
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
            ATDMAENR::ATDMAEN_0 => false,
            ATDMAENR::ATDMAEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ATDMAENR {
        match value {
            false => ATDMAENR::ATDMAEN_0,
            true => ATDMAENR::ATDMAEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ATDMAEN_0`"]
    #[inline]
    pub fn is_atdmaen_0(&self) -> bool {
        *self == ATDMAENR::ATDMAEN_0
    }
    #[doc = "Checks if the value of the field is `ATDMAEN_1`"]
    #[inline]
    pub fn is_atdmaen_1(&self) -> bool {
        *self == ATDMAENR::ATDMAEN_1
    }
}
#[doc = "Possible values of the field `TXDMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDMAENR {
    #[doc = "Disable transmit DMA request"]
    TXDMAEN_0,
    #[doc = "Enable transmit DMA request"]
    TXDMAEN_1,
}
impl TXDMAENR {
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
            TXDMAENR::TXDMAEN_0 => false,
            TXDMAENR::TXDMAEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXDMAENR {
        match value {
            false => TXDMAENR::TXDMAEN_0,
            true => TXDMAENR::TXDMAEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXDMAEN_0`"]
    #[inline]
    pub fn is_txdmaen_0(&self) -> bool {
        *self == TXDMAENR::TXDMAEN_0
    }
    #[doc = "Checks if the value of the field is `TXDMAEN_1`"]
    #[inline]
    pub fn is_txdmaen_1(&self) -> bool {
        *self == TXDMAENR::TXDMAEN_1
    }
}
#[doc = "Possible values of the field `SNDBRK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SNDBRKR {
    #[doc = "Do not send a BREAK character"]
    SNDBRK_0,
    #[doc = "Send a BREAK character (continuous 0s)"]
    SNDBRK_1,
}
impl SNDBRKR {
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
            SNDBRKR::SNDBRK_0 => false,
            SNDBRKR::SNDBRK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SNDBRKR {
        match value {
            false => SNDBRKR::SNDBRK_0,
            true => SNDBRKR::SNDBRK_1,
        }
    }
    #[doc = "Checks if the value of the field is `SNDBRK_0`"]
    #[inline]
    pub fn is_sndbrk_0(&self) -> bool {
        *self == SNDBRKR::SNDBRK_0
    }
    #[doc = "Checks if the value of the field is `SNDBRK_1`"]
    #[inline]
    pub fn is_sndbrk_1(&self) -> bool {
        *self == SNDBRKR::SNDBRK_1
    }
}
#[doc = "Possible values of the field `RTSDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSDENR {
    #[doc = "Disable RTSD interrupt"]
    RTSDEN_0,
    #[doc = "Enable RTSD interrupt"]
    RTSDEN_1,
}
impl RTSDENR {
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
            RTSDENR::RTSDEN_0 => false,
            RTSDENR::RTSDEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTSDENR {
        match value {
            false => RTSDENR::RTSDEN_0,
            true => RTSDENR::RTSDEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTSDEN_0`"]
    #[inline]
    pub fn is_rtsden_0(&self) -> bool {
        *self == RTSDENR::RTSDEN_0
    }
    #[doc = "Checks if the value of the field is `RTSDEN_1`"]
    #[inline]
    pub fn is_rtsden_1(&self) -> bool {
        *self == RTSDENR::RTSDEN_1
    }
}
#[doc = "Possible values of the field `TXMPTYEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXMPTYENR {
    #[doc = "Disable the transmitter FIFO empty interrupt"]
    TXMPTYEN_0,
    #[doc = "Enable the transmitter FIFO empty interrupt"]
    TXMPTYEN_1,
}
impl TXMPTYENR {
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
            TXMPTYENR::TXMPTYEN_0 => false,
            TXMPTYENR::TXMPTYEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXMPTYENR {
        match value {
            false => TXMPTYENR::TXMPTYEN_0,
            true => TXMPTYENR::TXMPTYEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXMPTYEN_0`"]
    #[inline]
    pub fn is_txmptyen_0(&self) -> bool {
        *self == TXMPTYENR::TXMPTYEN_0
    }
    #[doc = "Checks if the value of the field is `TXMPTYEN_1`"]
    #[inline]
    pub fn is_txmptyen_1(&self) -> bool {
        *self == TXMPTYENR::TXMPTYEN_1
    }
}
#[doc = "Possible values of the field `IREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRENR {
    #[doc = "Disable the IR interface"]
    IREN_0,
    #[doc = "Enable the IR interface"]
    IREN_1,
}
impl IRENR {
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
            IRENR::IREN_0 => false,
            IRENR::IREN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRENR {
        match value {
            false => IRENR::IREN_0,
            true => IRENR::IREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `IREN_0`"]
    #[inline]
    pub fn is_iren_0(&self) -> bool {
        *self == IRENR::IREN_0
    }
    #[doc = "Checks if the value of the field is `IREN_1`"]
    #[inline]
    pub fn is_iren_1(&self) -> bool {
        *self == IRENR::IREN_1
    }
}
#[doc = "Possible values of the field `RXDMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMAENR {
    #[doc = "Disable DMA request"]
    RXDMAEN_0,
    #[doc = "Enable DMA request"]
    RXDMAEN_1,
}
impl RXDMAENR {
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
            RXDMAENR::RXDMAEN_0 => false,
            RXDMAENR::RXDMAEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXDMAENR {
        match value {
            false => RXDMAENR::RXDMAEN_0,
            true => RXDMAENR::RXDMAEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXDMAEN_0`"]
    #[inline]
    pub fn is_rxdmaen_0(&self) -> bool {
        *self == RXDMAENR::RXDMAEN_0
    }
    #[doc = "Checks if the value of the field is `RXDMAEN_1`"]
    #[inline]
    pub fn is_rxdmaen_1(&self) -> bool {
        *self == RXDMAENR::RXDMAEN_1
    }
}
#[doc = "Possible values of the field `RRDYEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRDYENR {
    #[doc = "Disables the RRDY interrupt"]
    RRDYEN_0,
    #[doc = "Enables the RRDY interrupt"]
    RRDYEN_1,
}
impl RRDYENR {
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
            RRDYENR::RRDYEN_0 => false,
            RRDYENR::RRDYEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RRDYENR {
        match value {
            false => RRDYENR::RRDYEN_0,
            true => RRDYENR::RRDYEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RRDYEN_0`"]
    #[inline]
    pub fn is_rrdyen_0(&self) -> bool {
        *self == RRDYENR::RRDYEN_0
    }
    #[doc = "Checks if the value of the field is `RRDYEN_1`"]
    #[inline]
    pub fn is_rrdyen_1(&self) -> bool {
        *self == RRDYENR::RRDYEN_1
    }
}
#[doc = "Possible values of the field `ICD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICDR {
    #[doc = "Idle for more than 4 frames"]
    ICD_0,
    #[doc = "Idle for more than 8 frames"]
    ICD_1,
    #[doc = "Idle for more than 16 frames"]
    ICD_2,
    #[doc = "Idle for more than 32 frames"]
    ICD_3,
}
impl ICDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICDR::ICD_0 => 0,
            ICDR::ICD_1 => 1,
            ICDR::ICD_2 => 2,
            ICDR::ICD_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICDR {
        match value {
            0 => ICDR::ICD_0,
            1 => ICDR::ICD_1,
            2 => ICDR::ICD_2,
            3 => ICDR::ICD_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ICD_0`"]
    #[inline]
    pub fn is_icd_0(&self) -> bool {
        *self == ICDR::ICD_0
    }
    #[doc = "Checks if the value of the field is `ICD_1`"]
    #[inline]
    pub fn is_icd_1(&self) -> bool {
        *self == ICDR::ICD_1
    }
    #[doc = "Checks if the value of the field is `ICD_2`"]
    #[inline]
    pub fn is_icd_2(&self) -> bool {
        *self == ICDR::ICD_2
    }
    #[doc = "Checks if the value of the field is `ICD_3`"]
    #[inline]
    pub fn is_icd_3(&self) -> bool {
        *self == ICDR::ICD_3
    }
}
#[doc = "Possible values of the field `IDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDENR {
    #[doc = "Disable the IDLE interrupt"]
    IDEN_0,
    #[doc = "Enable the IDLE interrupt"]
    IDEN_1,
}
impl IDENR {
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
            IDENR::IDEN_0 => false,
            IDENR::IDEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDENR {
        match value {
            false => IDENR::IDEN_0,
            true => IDENR::IDEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `IDEN_0`"]
    #[inline]
    pub fn is_iden_0(&self) -> bool {
        *self == IDENR::IDEN_0
    }
    #[doc = "Checks if the value of the field is `IDEN_1`"]
    #[inline]
    pub fn is_iden_1(&self) -> bool {
        *self == IDENR::IDEN_1
    }
}
#[doc = "Possible values of the field `TRDYEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRDYENR {
    #[doc = "Disable the transmitter ready interrupt"]
    TRDYEN_0,
    #[doc = "Enable the transmitter ready interrupt"]
    TRDYEN_1,
}
impl TRDYENR {
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
            TRDYENR::TRDYEN_0 => false,
            TRDYENR::TRDYEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRDYENR {
        match value {
            false => TRDYENR::TRDYEN_0,
            true => TRDYENR::TRDYEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRDYEN_0`"]
    #[inline]
    pub fn is_trdyen_0(&self) -> bool {
        *self == TRDYENR::TRDYEN_0
    }
    #[doc = "Checks if the value of the field is `TRDYEN_1`"]
    #[inline]
    pub fn is_trdyen_1(&self) -> bool {
        *self == TRDYENR::TRDYEN_1
    }
}
#[doc = "Possible values of the field `ADBR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADBRR {
    #[doc = "Disable automatic detection of baud rate"]
    ADBR_0,
    #[doc = "Enable automatic detection of baud rate"]
    ADBR_1,
}
impl ADBRR {
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
            ADBRR::ADBR_0 => false,
            ADBRR::ADBR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADBRR {
        match value {
            false => ADBRR::ADBR_0,
            true => ADBRR::ADBR_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADBR_0`"]
    #[inline]
    pub fn is_adbr_0(&self) -> bool {
        *self == ADBRR::ADBR_0
    }
    #[doc = "Checks if the value of the field is `ADBR_1`"]
    #[inline]
    pub fn is_adbr_1(&self) -> bool {
        *self == ADBRR::ADBR_1
    }
}
#[doc = "Possible values of the field `ADEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADENR {
    #[doc = "Disable the automatic baud rate detection interrupt"]
    ADEN_0,
    #[doc = "Enable the automatic baud rate detection interrupt"]
    ADEN_1,
}
impl ADENR {
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
            ADENR::ADEN_0 => false,
            ADENR::ADEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADENR {
        match value {
            false => ADENR::ADEN_0,
            true => ADENR::ADEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADEN_0`"]
    #[inline]
    pub fn is_aden_0(&self) -> bool {
        *self == ADENR::ADEN_0
    }
    #[doc = "Checks if the value of the field is `ADEN_1`"]
    #[inline]
    pub fn is_aden_1(&self) -> bool {
        *self == ADENR::ADEN_1
    }
}
#[doc = "Values that can be written to the field `UARTEN`"]
pub enum UARTENW {
    #[doc = "Disable the UART"]
    UARTEN_0,
    #[doc = "Enable the UART"]
    UARTEN_1,
}
impl UARTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UARTENW::UARTEN_0 => false,
            UARTENW::UARTEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UARTENW<'a> {
    w: &'a mut W,
}
impl<'a> _UARTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UARTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the UART"]
    #[inline]
    pub fn uarten_0(self) -> &'a mut W {
        self.variant(UARTENW::UARTEN_0)
    }
    #[doc = "Enable the UART"]
    #[inline]
    pub fn uarten_1(self) -> &'a mut W {
        self.variant(UARTENW::UARTEN_1)
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
#[doc = "Values that can be written to the field `DOZE`"]
pub enum DOZEW {
    #[doc = "The UART is enabled when in DOZE state"]
    DOZE_0,
    #[doc = "The UART is disabled when in DOZE state"]
    DOZE_1,
}
impl DOZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOZEW::DOZE_0 => false,
            DOZEW::DOZE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOZEW<'a> {
    w: &'a mut W,
}
impl<'a> _DOZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The UART is enabled when in DOZE state"]
    #[inline]
    pub fn doze_0(self) -> &'a mut W {
        self.variant(DOZEW::DOZE_0)
    }
    #[doc = "The UART is disabled when in DOZE state"]
    #[inline]
    pub fn doze_1(self) -> &'a mut W {
        self.variant(DOZEW::DOZE_1)
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
#[doc = "Values that can be written to the field `ATDMAEN`"]
pub enum ATDMAENW {
    #[doc = "Disable AGTIM DMA request"]
    ATDMAEN_0,
    #[doc = "Enable AGTIM DMA request"]
    ATDMAEN_1,
}
impl ATDMAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ATDMAENW::ATDMAEN_0 => false,
            ATDMAENW::ATDMAEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ATDMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _ATDMAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ATDMAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable AGTIM DMA request"]
    #[inline]
    pub fn atdmaen_0(self) -> &'a mut W {
        self.variant(ATDMAENW::ATDMAEN_0)
    }
    #[doc = "Enable AGTIM DMA request"]
    #[inline]
    pub fn atdmaen_1(self) -> &'a mut W {
        self.variant(ATDMAENW::ATDMAEN_1)
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
#[doc = "Values that can be written to the field `TXDMAEN`"]
pub enum TXDMAENW {
    #[doc = "Disable transmit DMA request"]
    TXDMAEN_0,
    #[doc = "Enable transmit DMA request"]
    TXDMAEN_1,
}
impl TXDMAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXDMAENW::TXDMAEN_0 => false,
            TXDMAENW::TXDMAEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXDMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDMAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXDMAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable transmit DMA request"]
    #[inline]
    pub fn txdmaen_0(self) -> &'a mut W {
        self.variant(TXDMAENW::TXDMAEN_0)
    }
    #[doc = "Enable transmit DMA request"]
    #[inline]
    pub fn txdmaen_1(self) -> &'a mut W {
        self.variant(TXDMAENW::TXDMAEN_1)
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
#[doc = "Values that can be written to the field `SNDBRK`"]
pub enum SNDBRKW {
    #[doc = "Do not send a BREAK character"]
    SNDBRK_0,
    #[doc = "Send a BREAK character (continuous 0s)"]
    SNDBRK_1,
}
impl SNDBRKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SNDBRKW::SNDBRK_0 => false,
            SNDBRKW::SNDBRK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SNDBRKW<'a> {
    w: &'a mut W,
}
impl<'a> _SNDBRKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SNDBRKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not send a BREAK character"]
    #[inline]
    pub fn sndbrk_0(self) -> &'a mut W {
        self.variant(SNDBRKW::SNDBRK_0)
    }
    #[doc = "Send a BREAK character (continuous 0s)"]
    #[inline]
    pub fn sndbrk_1(self) -> &'a mut W {
        self.variant(SNDBRKW::SNDBRK_1)
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
#[doc = "Values that can be written to the field `RTSDEN`"]
pub enum RTSDENW {
    #[doc = "Disable RTSD interrupt"]
    RTSDEN_0,
    #[doc = "Enable RTSD interrupt"]
    RTSDEN_1,
}
impl RTSDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTSDENW::RTSDEN_0 => false,
            RTSDENW::RTSDEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTSDENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTSDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTSDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable RTSD interrupt"]
    #[inline]
    pub fn rtsden_0(self) -> &'a mut W {
        self.variant(RTSDENW::RTSDEN_0)
    }
    #[doc = "Enable RTSD interrupt"]
    #[inline]
    pub fn rtsden_1(self) -> &'a mut W {
        self.variant(RTSDENW::RTSDEN_1)
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
#[doc = "Values that can be written to the field `TXMPTYEN`"]
pub enum TXMPTYENW {
    #[doc = "Disable the transmitter FIFO empty interrupt"]
    TXMPTYEN_0,
    #[doc = "Enable the transmitter FIFO empty interrupt"]
    TXMPTYEN_1,
}
impl TXMPTYENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXMPTYENW::TXMPTYEN_0 => false,
            TXMPTYENW::TXMPTYEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXMPTYENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXMPTYENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXMPTYENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the transmitter FIFO empty interrupt"]
    #[inline]
    pub fn txmptyen_0(self) -> &'a mut W {
        self.variant(TXMPTYENW::TXMPTYEN_0)
    }
    #[doc = "Enable the transmitter FIFO empty interrupt"]
    #[inline]
    pub fn txmptyen_1(self) -> &'a mut W {
        self.variant(TXMPTYENW::TXMPTYEN_1)
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
#[doc = "Values that can be written to the field `IREN`"]
pub enum IRENW {
    #[doc = "Disable the IR interface"]
    IREN_0,
    #[doc = "Enable the IR interface"]
    IREN_1,
}
impl IRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRENW::IREN_0 => false,
            IRENW::IREN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRENW<'a> {
    w: &'a mut W,
}
impl<'a> _IRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the IR interface"]
    #[inline]
    pub fn iren_0(self) -> &'a mut W {
        self.variant(IRENW::IREN_0)
    }
    #[doc = "Enable the IR interface"]
    #[inline]
    pub fn iren_1(self) -> &'a mut W {
        self.variant(IRENW::IREN_1)
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
#[doc = "Values that can be written to the field `RXDMAEN`"]
pub enum RXDMAENW {
    #[doc = "Disable DMA request"]
    RXDMAEN_0,
    #[doc = "Enable DMA request"]
    RXDMAEN_1,
}
impl RXDMAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXDMAENW::RXDMAEN_0 => false,
            RXDMAENW::RXDMAEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXDMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDMAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXDMAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable DMA request"]
    #[inline]
    pub fn rxdmaen_0(self) -> &'a mut W {
        self.variant(RXDMAENW::RXDMAEN_0)
    }
    #[doc = "Enable DMA request"]
    #[inline]
    pub fn rxdmaen_1(self) -> &'a mut W {
        self.variant(RXDMAENW::RXDMAEN_1)
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
#[doc = "Values that can be written to the field `RRDYEN`"]
pub enum RRDYENW {
    #[doc = "Disables the RRDY interrupt"]
    RRDYEN_0,
    #[doc = "Enables the RRDY interrupt"]
    RRDYEN_1,
}
impl RRDYENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RRDYENW::RRDYEN_0 => false,
            RRDYENW::RRDYEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RRDYENW<'a> {
    w: &'a mut W,
}
impl<'a> _RRDYENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RRDYENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the RRDY interrupt"]
    #[inline]
    pub fn rrdyen_0(self) -> &'a mut W {
        self.variant(RRDYENW::RRDYEN_0)
    }
    #[doc = "Enables the RRDY interrupt"]
    #[inline]
    pub fn rrdyen_1(self) -> &'a mut W {
        self.variant(RRDYENW::RRDYEN_1)
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
#[doc = "Values that can be written to the field `ICD`"]
pub enum ICDW {
    #[doc = "Idle for more than 4 frames"]
    ICD_0,
    #[doc = "Idle for more than 8 frames"]
    ICD_1,
    #[doc = "Idle for more than 16 frames"]
    ICD_2,
    #[doc = "Idle for more than 32 frames"]
    ICD_3,
}
impl ICDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICDW::ICD_0 => 0,
            ICDW::ICD_1 => 1,
            ICDW::ICD_2 => 2,
            ICDW::ICD_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICDW<'a> {
    w: &'a mut W,
}
impl<'a> _ICDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Idle for more than 4 frames"]
    #[inline]
    pub fn icd_0(self) -> &'a mut W {
        self.variant(ICDW::ICD_0)
    }
    #[doc = "Idle for more than 8 frames"]
    #[inline]
    pub fn icd_1(self) -> &'a mut W {
        self.variant(ICDW::ICD_1)
    }
    #[doc = "Idle for more than 16 frames"]
    #[inline]
    pub fn icd_2(self) -> &'a mut W {
        self.variant(ICDW::ICD_2)
    }
    #[doc = "Idle for more than 32 frames"]
    #[inline]
    pub fn icd_3(self) -> &'a mut W {
        self.variant(ICDW::ICD_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IDEN`"]
pub enum IDENW {
    #[doc = "Disable the IDLE interrupt"]
    IDEN_0,
    #[doc = "Enable the IDLE interrupt"]
    IDEN_1,
}
impl IDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IDENW::IDEN_0 => false,
            IDENW::IDEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDENW<'a> {
    w: &'a mut W,
}
impl<'a> _IDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the IDLE interrupt"]
    #[inline]
    pub fn iden_0(self) -> &'a mut W {
        self.variant(IDENW::IDEN_0)
    }
    #[doc = "Enable the IDLE interrupt"]
    #[inline]
    pub fn iden_1(self) -> &'a mut W {
        self.variant(IDENW::IDEN_1)
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
#[doc = "Values that can be written to the field `TRDYEN`"]
pub enum TRDYENW {
    #[doc = "Disable the transmitter ready interrupt"]
    TRDYEN_0,
    #[doc = "Enable the transmitter ready interrupt"]
    TRDYEN_1,
}
impl TRDYENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRDYENW::TRDYEN_0 => false,
            TRDYENW::TRDYEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRDYENW<'a> {
    w: &'a mut W,
}
impl<'a> _TRDYENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRDYENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the transmitter ready interrupt"]
    #[inline]
    pub fn trdyen_0(self) -> &'a mut W {
        self.variant(TRDYENW::TRDYEN_0)
    }
    #[doc = "Enable the transmitter ready interrupt"]
    #[inline]
    pub fn trdyen_1(self) -> &'a mut W {
        self.variant(TRDYENW::TRDYEN_1)
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
#[doc = "Values that can be written to the field `ADBR`"]
pub enum ADBRW {
    #[doc = "Disable automatic detection of baud rate"]
    ADBR_0,
    #[doc = "Enable automatic detection of baud rate"]
    ADBR_1,
}
impl ADBRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADBRW::ADBR_0 => false,
            ADBRW::ADBR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADBRW<'a> {
    w: &'a mut W,
}
impl<'a> _ADBRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADBRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable automatic detection of baud rate"]
    #[inline]
    pub fn adbr_0(self) -> &'a mut W {
        self.variant(ADBRW::ADBR_0)
    }
    #[doc = "Enable automatic detection of baud rate"]
    #[inline]
    pub fn adbr_1(self) -> &'a mut W {
        self.variant(ADBRW::ADBR_1)
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
#[doc = "Values that can be written to the field `ADEN`"]
pub enum ADENW {
    #[doc = "Disable the automatic baud rate detection interrupt"]
    ADEN_0,
    #[doc = "Enable the automatic baud rate detection interrupt"]
    ADEN_1,
}
impl ADENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADENW::ADEN_0 => false,
            ADENW::ADEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the automatic baud rate detection interrupt"]
    #[inline]
    pub fn aden_0(self) -> &'a mut W {
        self.variant(ADENW::ADEN_0)
    }
    #[doc = "Enable the automatic baud rate detection interrupt"]
    #[inline]
    pub fn aden_1(self) -> &'a mut W {
        self.variant(ADENW::ADEN_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - UART Enable"]
    #[inline]
    pub fn uarten(&self) -> UARTENR {
        UARTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - DOZE"]
    #[inline]
    pub fn doze(&self) -> DOZER {
        DOZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Aging DMA Timer Enable"]
    #[inline]
    pub fn atdmaen(&self) -> ATDMAENR {
        ATDMAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Transmitter Ready DMA Enable"]
    #[inline]
    pub fn txdmaen(&self) -> TXDMAENR {
        TXDMAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Send BREAK"]
    #[inline]
    pub fn sndbrk(&self) -> SNDBRKR {
        SNDBRKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - RTS Delta Interrupt Enable"]
    #[inline]
    pub fn rtsden(&self) -> RTSDENR {
        RTSDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Transmitter Empty Interrupt Enable"]
    #[inline]
    pub fn txmptyen(&self) -> TXMPTYENR {
        TXMPTYENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Infrared Interface Enable"]
    #[inline]
    pub fn iren(&self) -> IRENR {
        IRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Receive Ready DMA Enable"]
    #[inline]
    pub fn rxdmaen(&self) -> RXDMAENR {
        RXDMAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Receiver Ready Interrupt Enable"]
    #[inline]
    pub fn rrdyen(&self) -> RRDYENR {
        RRDYENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 10:11 - Idle Condition Detect"]
    #[inline]
    pub fn icd(&self) -> ICDR {
        ICDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Idle Condition Detected Interrupt Enable"]
    #[inline]
    pub fn iden(&self) -> IDENR {
        IDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Transmitter Ready Interrupt Enable"]
    #[inline]
    pub fn trdyen(&self) -> TRDYENR {
        TRDYENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Automatic Detection of Baud Rate"]
    #[inline]
    pub fn adbr(&self) -> ADBRR {
        ADBRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Automatic Baud Rate Detection Interrupt Enable"]
    #[inline]
    pub fn aden(&self) -> ADENR {
        ADENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
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
    #[doc = "Bit 0 - UART Enable"]
    #[inline]
    pub fn uarten(&mut self) -> _UARTENW {
        _UARTENW { w: self }
    }
    #[doc = "Bit 1 - DOZE"]
    #[inline]
    pub fn doze(&mut self) -> _DOZEW {
        _DOZEW { w: self }
    }
    #[doc = "Bit 2 - Aging DMA Timer Enable"]
    #[inline]
    pub fn atdmaen(&mut self) -> _ATDMAENW {
        _ATDMAENW { w: self }
    }
    #[doc = "Bit 3 - Transmitter Ready DMA Enable"]
    #[inline]
    pub fn txdmaen(&mut self) -> _TXDMAENW {
        _TXDMAENW { w: self }
    }
    #[doc = "Bit 4 - Send BREAK"]
    #[inline]
    pub fn sndbrk(&mut self) -> _SNDBRKW {
        _SNDBRKW { w: self }
    }
    #[doc = "Bit 5 - RTS Delta Interrupt Enable"]
    #[inline]
    pub fn rtsden(&mut self) -> _RTSDENW {
        _RTSDENW { w: self }
    }
    #[doc = "Bit 6 - Transmitter Empty Interrupt Enable"]
    #[inline]
    pub fn txmptyen(&mut self) -> _TXMPTYENW {
        _TXMPTYENW { w: self }
    }
    #[doc = "Bit 7 - Infrared Interface Enable"]
    #[inline]
    pub fn iren(&mut self) -> _IRENW {
        _IRENW { w: self }
    }
    #[doc = "Bit 8 - Receive Ready DMA Enable"]
    #[inline]
    pub fn rxdmaen(&mut self) -> _RXDMAENW {
        _RXDMAENW { w: self }
    }
    #[doc = "Bit 9 - Receiver Ready Interrupt Enable"]
    #[inline]
    pub fn rrdyen(&mut self) -> _RRDYENW {
        _RRDYENW { w: self }
    }
    #[doc = "Bits 10:11 - Idle Condition Detect"]
    #[inline]
    pub fn icd(&mut self) -> _ICDW {
        _ICDW { w: self }
    }
    #[doc = "Bit 12 - Idle Condition Detected Interrupt Enable"]
    #[inline]
    pub fn iden(&mut self) -> _IDENW {
        _IDENW { w: self }
    }
    #[doc = "Bit 13 - Transmitter Ready Interrupt Enable"]
    #[inline]
    pub fn trdyen(&mut self) -> _TRDYENW {
        _TRDYENW { w: self }
    }
    #[doc = "Bit 14 - Automatic Detection of Baud Rate"]
    #[inline]
    pub fn adbr(&mut self) -> _ADBRW {
        _ADBRW { w: self }
    }
    #[doc = "Bit 15 - Automatic Baud Rate Detection Interrupt Enable"]
    #[inline]
    pub fn aden(&mut self) -> _ADENW {
        _ADENW { w: self }
    }
}
