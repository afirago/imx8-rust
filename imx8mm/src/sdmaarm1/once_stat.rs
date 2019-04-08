#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ONCE_STAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `ECDR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECDRR {
    #[doc = "1 matched addra_cond"]
    ECDR_0,
    #[doc = "1 matched addrb_cond"]
    ECDR_1,
    #[doc = "1 matched data_cond"]
    ECDR_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ECDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ECDRR::ECDR_0 => 0,
            ECDRR::ECDR_1 => 1,
            ECDRR::ECDR_2 => 2,
            ECDRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ECDRR {
        match value {
            0 => ECDRR::ECDR_0,
            1 => ECDRR::ECDR_1,
            2 => ECDRR::ECDR_2,
            i => ECDRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ECDR_0`"]
    #[inline]
    pub fn is_ecdr_0(&self) -> bool {
        *self == ECDRR::ECDR_0
    }
    #[doc = "Checks if the value of the field is `ECDR_1`"]
    #[inline]
    pub fn is_ecdr_1(&self) -> bool {
        *self == ECDRR::ECDR_1
    }
    #[doc = "Checks if the value of the field is `ECDR_2`"]
    #[inline]
    pub fn is_ecdr_2(&self) -> bool {
        *self == ECDRR::ECDR_2
    }
}
#[doc = "Possible values of the field `MST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTR {
    #[doc = "The JTAG interface controls the OnCE."]
    MST_0,
    #[doc = "The Arm platform peripheral interface controls the OnCE."]
    MST_1,
}
impl MSTR {
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
            MSTR::MST_0 => false,
            MSTR::MST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTR {
        match value {
            false => MSTR::MST_0,
            true => MSTR::MST_1,
        }
    }
    #[doc = "Checks if the value of the field is `MST_0`"]
    #[inline]
    pub fn is_mst_0(&self) -> bool {
        *self == MSTR::MST_0
    }
    #[doc = "Checks if the value of the field is `MST_1`"]
    #[inline]
    pub fn is_mst_1(&self) -> bool {
        *self == MSTR::MST_1
    }
}
#[doc = r" Value of the field"]
pub struct SWBR {
    bits: bool,
}
impl SWBR {
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
pub struct ODRR {
    bits: bool,
}
impl ODRR {
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
pub struct EDRR {
    bits: bool,
}
impl EDRR {
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
pub struct RCVR {
    bits: bool,
}
impl RCVR {
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
#[doc = "Possible values of the field `PST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSTR {
    #[doc = "Program"]
    PST_0,
    #[doc = "Data"]
    PST_1,
    #[doc = "Change of Flow"]
    PST_2,
    #[doc = "Change of Flow in Loop"]
    PST_3,
    #[doc = "Debug"]
    PST_4,
    #[doc = "Functional Unit"]
    PST_5,
    #[doc = "Sleep"]
    PST_6,
    #[doc = "Save"]
    PST_7,
    #[doc = "Program in Sleep"]
    PST_8,
    #[doc = "Data in Sleep"]
    PST_9,
    #[doc = "Debug in Sleep"]
    PST_12,
    #[doc = "Functional Unit in Sleep"]
    PST_13,
    #[doc = "Sleep after Reset"]
    PST_14,
    #[doc = "Restore"]
    PST_15,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSTR::PST_0 => 0,
            PSTR::PST_1 => 1,
            PSTR::PST_2 => 2,
            PSTR::PST_3 => 3,
            PSTR::PST_4 => 4,
            PSTR::PST_5 => 5,
            PSTR::PST_6 => 6,
            PSTR::PST_7 => 7,
            PSTR::PST_8 => 8,
            PSTR::PST_9 => 9,
            PSTR::PST_12 => 12,
            PSTR::PST_13 => 13,
            PSTR::PST_14 => 14,
            PSTR::PST_15 => 15,
            PSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSTR {
        match value {
            0 => PSTR::PST_0,
            1 => PSTR::PST_1,
            2 => PSTR::PST_2,
            3 => PSTR::PST_3,
            4 => PSTR::PST_4,
            5 => PSTR::PST_5,
            6 => PSTR::PST_6,
            7 => PSTR::PST_7,
            8 => PSTR::PST_8,
            9 => PSTR::PST_9,
            12 => PSTR::PST_12,
            13 => PSTR::PST_13,
            14 => PSTR::PST_14,
            15 => PSTR::PST_15,
            i => PSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PST_0`"]
    #[inline]
    pub fn is_pst_0(&self) -> bool {
        *self == PSTR::PST_0
    }
    #[doc = "Checks if the value of the field is `PST_1`"]
    #[inline]
    pub fn is_pst_1(&self) -> bool {
        *self == PSTR::PST_1
    }
    #[doc = "Checks if the value of the field is `PST_2`"]
    #[inline]
    pub fn is_pst_2(&self) -> bool {
        *self == PSTR::PST_2
    }
    #[doc = "Checks if the value of the field is `PST_3`"]
    #[inline]
    pub fn is_pst_3(&self) -> bool {
        *self == PSTR::PST_3
    }
    #[doc = "Checks if the value of the field is `PST_4`"]
    #[inline]
    pub fn is_pst_4(&self) -> bool {
        *self == PSTR::PST_4
    }
    #[doc = "Checks if the value of the field is `PST_5`"]
    #[inline]
    pub fn is_pst_5(&self) -> bool {
        *self == PSTR::PST_5
    }
    #[doc = "Checks if the value of the field is `PST_6`"]
    #[inline]
    pub fn is_pst_6(&self) -> bool {
        *self == PSTR::PST_6
    }
    #[doc = "Checks if the value of the field is `PST_7`"]
    #[inline]
    pub fn is_pst_7(&self) -> bool {
        *self == PSTR::PST_7
    }
    #[doc = "Checks if the value of the field is `PST_8`"]
    #[inline]
    pub fn is_pst_8(&self) -> bool {
        *self == PSTR::PST_8
    }
    #[doc = "Checks if the value of the field is `PST_9`"]
    #[inline]
    pub fn is_pst_9(&self) -> bool {
        *self == PSTR::PST_9
    }
    #[doc = "Checks if the value of the field is `PST_12`"]
    #[inline]
    pub fn is_pst_12(&self) -> bool {
        *self == PSTR::PST_12
    }
    #[doc = "Checks if the value of the field is `PST_13`"]
    #[inline]
    pub fn is_pst_13(&self) -> bool {
        *self == PSTR::PST_13
    }
    #[doc = "Checks if the value of the field is `PST_14`"]
    #[inline]
    pub fn is_pst_14(&self) -> bool {
        *self == PSTR::PST_14
    }
    #[doc = "Checks if the value of the field is `PST_15`"]
    #[inline]
    pub fn is_pst_15(&self) -> bool {
        *self == PSTR::PST_15
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Event Cell Debug Request"]
    #[inline]
    pub fn ecdr(&self) -> ECDRR {
        ECDRR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - This flag is raised when the OnCE is controlled from the Arm platform peripheral interface."]
    #[inline]
    pub fn mst(&self) -> MSTR {
        MSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - This flag is raised when the SDMA has entered debug mode after a software breakpoint."]
    #[inline]
    pub fn swb(&self) -> SWBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWBR { bits }
    }
    #[doc = "Bit 9 - This flag is raised when the SDMA has entered debug mode after a OnCE debug request."]
    #[inline]
    pub fn odr(&self) -> ODRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ODRR { bits }
    }
    #[doc = "Bit 10 - This flag is raised when the SDMA has entered debug mode after an external debug request."]
    #[inline]
    pub fn edr(&self) -> EDRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EDRR { bits }
    }
    #[doc = "Bit 11 - After each write access to the real time buffer (RTB), the RCV bit is set"]
    #[inline]
    pub fn rcv(&self) -> RCVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RCVR { bits }
    }
    #[doc = "Bits 12:15 - The Processor Status bits reflect the state of the SDMA RISC engine"]
    #[inline]
    pub fn pst(&self) -> PSTR {
        PSTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
