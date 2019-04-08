#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::PLAMC {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `AMC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AMCR {
    #[doc = "A bus master connection to AXBS input port n is absent"]
    AMC_0,
    #[doc = "A bus master connection to AXBS input port n is present"]
    AMC_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AMCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AMCR::AMC_0 => 0,
            AMCR::AMC_1 => 1,
            AMCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AMCR {
        match value {
            0 => AMCR::AMC_0,
            1 => AMCR::AMC_1,
            i => AMCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AMC_0`"]
    #[inline]
    pub fn is_amc_0(&self) -> bool {
        *self == AMCR::AMC_0
    }
    #[doc = "Checks if the value of the field is `AMC_1`"]
    #[inline]
    pub fn is_amc_1(&self) -> bool {
        *self == AMCR::AMC_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:7 - Each bit in the AMC field indicates whether there is a corresponding connection to the AXBS master input port."]
    #[inline]
    pub fn amc(&self) -> AMCR {
        AMCR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
}
