#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TSR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `TB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBR {
    #[doc = "TMU idle."]
    TB_0,
    #[doc = "TMU busy. In monitoring mode this indicates a temperature measurement is pending. In calibration mode, sensor result has not yet been determined based on last given ambient temperature."]
    TB_1,
}
impl TBR {
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
            TBR::TB_0 => false,
            TBR::TB_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBR {
        match value {
            false => TBR::TB_0,
            true => TBR::TB_1,
        }
    }
    #[doc = "Checks if the value of the field is `TB_0`"]
    #[inline]
    pub fn is_tb_0(&self) -> bool {
        *self == TBR::TB_0
    }
    #[doc = "Checks if the value of the field is `TB_1`"]
    #[inline]
    pub fn is_tb_1(&self) -> bool {
        *self == TBR::TB_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 31 - TMU busy."]
    #[inline]
    pub fn tb(&self) -> TBR {
        TBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
