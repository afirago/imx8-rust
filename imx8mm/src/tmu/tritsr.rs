#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TRITSR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct TEMPR {
    bits: u8,
}
impl TEMPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `V`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VR {
    #[doc = "Not valid. Temperature out of sensor range or first measurement still pending."]
    V_0,
    #[doc = "Valid."]
    V_1,
}
impl VR {
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
            VR::V_0 => false,
            VR::V_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VR {
        match value {
            false => VR::V_0,
            true => VR::V_1,
        }
    }
    #[doc = "Checks if the value of the field is `V_0`"]
    #[inline]
    pub fn is_v_0(&self) -> bool {
        *self == VR::V_0
    }
    #[doc = "Checks if the value of the field is `V_1`"]
    #[inline]
    pub fn is_v_1(&self) -> bool {
        *self == VR::V_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Last temperature reading at site when V=1."]
    #[inline]
    pub fn temp(&self) -> TEMPR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TEMPR { bits }
    }
    #[doc = "Bit 31 - Valid measured temperature."]
    #[inline]
    pub fn v(&self) -> VR {
        VR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
