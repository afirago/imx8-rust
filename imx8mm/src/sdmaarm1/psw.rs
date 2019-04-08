#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PSW {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CCRR {
    bits: u8,
}
impl CCRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CCP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCPR {
    #[doc = "No running channel"]
    CCP_0,
    #[doc = "Active channel priority"]
    CCP_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CCPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CCPR::CCP_0 => 0,
            CCPR::CCP_1 => 1,
            CCPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CCPR {
        match value {
            0 => CCPR::CCP_0,
            1 => CCPR::CCP_1,
            i => CCPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CCP_0`"]
    #[inline]
    pub fn is_ccp_0(&self) -> bool {
        *self == CCPR::CCP_0
    }
    #[doc = "Checks if the value of the field is `CCP_1`"]
    #[inline]
    pub fn is_ccp_1(&self) -> bool {
        *self == CCPR::CCP_1
    }
}
#[doc = r" Value of the field"]
pub struct NCRR {
    bits: u8,
}
impl NCRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `NCP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCPR {
    #[doc = "No running channel"]
    NCP_0,
    #[doc = "Active channel priority"]
    NCP_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NCPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NCPR::NCP_0 => 0,
            NCPR::NCP_1 => 1,
            NCPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NCPR {
        match value {
            0 => NCPR::NCP_0,
            1 => NCPR::NCP_1,
            i => NCPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NCP_0`"]
    #[inline]
    pub fn is_ncp_0(&self) -> bool {
        *self == NCPR::NCP_0
    }
    #[doc = "Checks if the value of the field is `NCP_1`"]
    #[inline]
    pub fn is_ncp_1(&self) -> bool {
        *self == NCPR::NCP_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - The Current Channel Register indicates the number of the channel that is being executed by the SDMA"]
    #[inline]
    pub fn ccr(&self) -> CCRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CCRR { bits }
    }
    #[doc = "Bits 4:7 - The Current Channel Priority indicates the priority of the current active channel"]
    #[inline]
    pub fn ccp(&self) -> CCPR {
        CCPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:12 - The Next Channel Register indicates the number of the next scheduled pending channel with the highest priority"]
    #[inline]
    pub fn ncr(&self) -> NCRR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NCRR { bits }
    }
    #[doc = "Bits 13:15 - The Next Channel Priority gives the next pending channel priority"]
    #[inline]
    pub fn ncp(&self) -> NCPR {
        NCPR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
