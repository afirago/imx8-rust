#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FATR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `BEDA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEDAR {
    #[doc = "Instruction"]
    BEDA_0,
    #[doc = "Data"]
    BEDA_1,
}
impl BEDAR {
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
            BEDAR::BEDA_0 => false,
            BEDAR::BEDA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BEDAR {
        match value {
            false => BEDAR::BEDA_0,
            true => BEDAR::BEDA_1,
        }
    }
    #[doc = "Checks if the value of the field is `BEDA_0`"]
    #[inline]
    pub fn is_beda_0(&self) -> bool {
        *self == BEDAR::BEDA_0
    }
    #[doc = "Checks if the value of the field is `BEDA_1`"]
    #[inline]
    pub fn is_beda_1(&self) -> bool {
        *self == BEDAR::BEDA_1
    }
}
#[doc = "Possible values of the field `BEMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEMDR {
    #[doc = "User mode"]
    BEMD_0,
    #[doc = "Supervisor/privileged mode"]
    BEMD_1,
}
impl BEMDR {
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
            BEMDR::BEMD_0 => false,
            BEMDR::BEMD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BEMDR {
        match value {
            false => BEMDR::BEMD_0,
            true => BEMDR::BEMD_1,
        }
    }
    #[doc = "Checks if the value of the field is `BEMD_0`"]
    #[inline]
    pub fn is_bemd_0(&self) -> bool {
        *self == BEMDR::BEMD_0
    }
    #[doc = "Checks if the value of the field is `BEMD_1`"]
    #[inline]
    pub fn is_bemd_1(&self) -> bool {
        *self == BEMDR::BEMD_1
    }
}
#[doc = "Possible values of the field `BESZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BESZR {
    #[doc = "8-bit access"]
    BESZ_0,
    #[doc = "16-bit access"]
    BESZ_1,
    #[doc = "32-bit access"]
    BESZ_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BESZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BESZR::BESZ_0 => 0,
            BESZR::BESZ_1 => 1,
            BESZR::BESZ_2 => 2,
            BESZR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BESZR {
        match value {
            0 => BESZR::BESZ_0,
            1 => BESZR::BESZ_1,
            2 => BESZR::BESZ_2,
            i => BESZR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BESZ_0`"]
    #[inline]
    pub fn is_besz_0(&self) -> bool {
        *self == BESZR::BESZ_0
    }
    #[doc = "Checks if the value of the field is `BESZ_1`"]
    #[inline]
    pub fn is_besz_1(&self) -> bool {
        *self == BESZR::BESZ_1
    }
    #[doc = "Checks if the value of the field is `BESZ_2`"]
    #[inline]
    pub fn is_besz_2(&self) -> bool {
        *self == BESZR::BESZ_2
    }
}
#[doc = "Possible values of the field `BEWT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEWTR {
    #[doc = "Read access"]
    BEWT_0,
    #[doc = "Write access"]
    BEWT_1,
}
impl BEWTR {
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
            BEWTR::BEWT_0 => false,
            BEWTR::BEWT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BEWTR {
        match value {
            false => BEWTR::BEWT_0,
            true => BEWTR::BEWT_1,
        }
    }
    #[doc = "Checks if the value of the field is `BEWT_0`"]
    #[inline]
    pub fn is_bewt_0(&self) -> bool {
        *self == BEWTR::BEWT_0
    }
    #[doc = "Checks if the value of the field is `BEWT_1`"]
    #[inline]
    pub fn is_bewt_1(&self) -> bool {
        *self == BEWTR::BEWT_1
    }
}
#[doc = r" Value of the field"]
pub struct BEMNR {
    bits: u8,
}
impl BEMNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BEOVR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEOVRR {
    #[doc = "No bus error overrun"]
    BEOVR_0,
    #[doc = "Bus error overrun occurred. The FADR and FDR registers and the other FATR bits are not updated to reflect this new bus error."]
    BEOVR_1,
}
impl BEOVRR {
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
            BEOVRR::BEOVR_0 => false,
            BEOVRR::BEOVR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BEOVRR {
        match value {
            false => BEOVRR::BEOVR_0,
            true => BEOVRR::BEOVR_1,
        }
    }
    #[doc = "Checks if the value of the field is `BEOVR_0`"]
    #[inline]
    pub fn is_beovr_0(&self) -> bool {
        *self == BEOVRR::BEOVR_0
    }
    #[doc = "Checks if the value of the field is `BEOVR_1`"]
    #[inline]
    pub fn is_beovr_1(&self) -> bool {
        *self == BEOVRR::BEOVR_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Bus error access type"]
    #[inline]
    pub fn beda(&self) -> BEDAR {
        BEDAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Bus error privilege level"]
    #[inline]
    pub fn bemd(&self) -> BEMDR {
        BEMDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - Bus error size"]
    #[inline]
    pub fn besz(&self) -> BESZR {
        BESZR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Bus error write"]
    #[inline]
    pub fn bewt(&self) -> BEWTR {
        BEWTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - Bus error master number"]
    #[inline]
    pub fn bemn(&self) -> BEMNR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BEMNR { bits }
    }
    #[doc = "Bit 31 - Bus error overrun"]
    #[inline]
    pub fn beovr(&self) -> BEOVRR {
        BEOVRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
