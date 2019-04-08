#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::URXD {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RX_DATAR {
    bits: u8,
}
impl RX_DATAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PRERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRERRR {
    #[doc = "= No parity error was detected for data in the RX_DATA field"]
    PRERR_0,
    #[doc = "= A parity error was detected for data in the RX_DATA field"]
    PRERR_1,
}
impl PRERRR {
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
            PRERRR::PRERR_0 => false,
            PRERRR::PRERR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRERRR {
        match value {
            false => PRERRR::PRERR_0,
            true => PRERRR::PRERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `PRERR_0`"]
    #[inline]
    pub fn is_prerr_0(&self) -> bool {
        *self == PRERRR::PRERR_0
    }
    #[doc = "Checks if the value of the field is `PRERR_1`"]
    #[inline]
    pub fn is_prerr_1(&self) -> bool {
        *self == PRERRR::PRERR_1
    }
}
#[doc = "Possible values of the field `BRK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRKR {
    #[doc = "The current character is not a BREAK character"]
    BRK_0,
    #[doc = "The current character is a BREAK character"]
    BRK_1,
}
impl BRKR {
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
            BRKR::BRK_0 => false,
            BRKR::BRK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BRKR {
        match value {
            false => BRKR::BRK_0,
            true => BRKR::BRK_1,
        }
    }
    #[doc = "Checks if the value of the field is `BRK_0`"]
    #[inline]
    pub fn is_brk_0(&self) -> bool {
        *self == BRKR::BRK_0
    }
    #[doc = "Checks if the value of the field is `BRK_1`"]
    #[inline]
    pub fn is_brk_1(&self) -> bool {
        *self == BRKR::BRK_1
    }
}
#[doc = "Possible values of the field `FRMERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRMERRR {
    #[doc = "The current character has no framing error"]
    FRMERR_0,
    #[doc = "The current character has a framing error"]
    FRMERR_1,
}
impl FRMERRR {
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
            FRMERRR::FRMERR_0 => false,
            FRMERRR::FRMERR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRMERRR {
        match value {
            false => FRMERRR::FRMERR_0,
            true => FRMERRR::FRMERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRMERR_0`"]
    #[inline]
    pub fn is_frmerr_0(&self) -> bool {
        *self == FRMERRR::FRMERR_0
    }
    #[doc = "Checks if the value of the field is `FRMERR_1`"]
    #[inline]
    pub fn is_frmerr_1(&self) -> bool {
        *self == FRMERRR::FRMERR_1
    }
}
#[doc = "Possible values of the field `OVRRUN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRRUNR {
    #[doc = "No RxFIFO overrun was detected"]
    OVRRUN_0,
    #[doc = "A RxFIFO overrun was detected"]
    OVRRUN_1,
}
impl OVRRUNR {
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
            OVRRUNR::OVRRUN_0 => false,
            OVRRUNR::OVRRUN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVRRUNR {
        match value {
            false => OVRRUNR::OVRRUN_0,
            true => OVRRUNR::OVRRUN_1,
        }
    }
    #[doc = "Checks if the value of the field is `OVRRUN_0`"]
    #[inline]
    pub fn is_ovrrun_0(&self) -> bool {
        *self == OVRRUNR::OVRRUN_0
    }
    #[doc = "Checks if the value of the field is `OVRRUN_1`"]
    #[inline]
    pub fn is_ovrrun_1(&self) -> bool {
        *self == OVRRUNR::OVRRUN_1
    }
}
#[doc = "Possible values of the field `ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRR {
    #[doc = "No error status was detected"]
    ERR_0,
    #[doc = "An error status was detected"]
    ERR_1,
}
impl ERRR {
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
            ERRR::ERR_0 => false,
            ERRR::ERR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRR {
        match value {
            false => ERRR::ERR_0,
            true => ERRR::ERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERR_0`"]
    #[inline]
    pub fn is_err_0(&self) -> bool {
        *self == ERRR::ERR_0
    }
    #[doc = "Checks if the value of the field is `ERR_1`"]
    #[inline]
    pub fn is_err_1(&self) -> bool {
        *self == ERRR::ERR_1
    }
}
#[doc = "Possible values of the field `CHARRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHARRDYR {
    #[doc = "Character in RX_DATA field and associated flags are invalid."]
    CHARRDY_0,
    #[doc = "Character in RX_DATA field and associated flags valid and ready for reading."]
    CHARRDY_1,
}
impl CHARRDYR {
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
            CHARRDYR::CHARRDY_0 => false,
            CHARRDYR::CHARRDY_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHARRDYR {
        match value {
            false => CHARRDYR::CHARRDY_0,
            true => CHARRDYR::CHARRDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `CHARRDY_0`"]
    #[inline]
    pub fn is_charrdy_0(&self) -> bool {
        *self == CHARRDYR::CHARRDY_0
    }
    #[doc = "Checks if the value of the field is `CHARRDY_1`"]
    #[inline]
    pub fn is_charrdy_1(&self) -> bool {
        *self == CHARRDYR::CHARRDY_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Received Data"]
    #[inline]
    pub fn rx_data(&self) -> RX_DATAR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RX_DATAR { bits }
    }
    #[doc = "Bit 10 - In RS-485 mode, it holds the ninth data bit (bit \\[8\\]) of received 9-bit RS-485 data In RS232/IrDA mode, it is the Parity Error flag"]
    #[inline]
    pub fn prerr(&self) -> PRERRR {
        PRERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - BREAK Detect"]
    #[inline]
    pub fn brk(&self) -> BRKR {
        BRKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Frame Error"]
    #[inline]
    pub fn frmerr(&self) -> FRMERRR {
        FRMERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Receiver Overrun"]
    #[inline]
    pub fn ovrrun(&self) -> OVRRUNR {
        OVRRUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Error Detect"]
    #[inline]
    pub fn err(&self) -> ERRR {
        ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Character Ready"]
    #[inline]
    pub fn charrdy(&self) -> CHARRDYR {
        CHARRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
