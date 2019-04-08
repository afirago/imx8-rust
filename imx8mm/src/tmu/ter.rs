#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TER {
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
#[doc = "Possible values of the field `ALPF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALPFR {
    #[doc = "1.0"]
    ALPF_0,
    #[doc = "0.5"]
    ALPF_1,
    #[doc = "0.25"]
    ALPF_2,
    #[doc = "0.125"]
    ALPF_3,
}
impl ALPFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ALPFR::ALPF_0 => 0,
            ALPFR::ALPF_1 => 1,
            ALPFR::ALPF_2 => 2,
            ALPFR::ALPF_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ALPFR {
        match value {
            0 => ALPFR::ALPF_0,
            1 => ALPFR::ALPF_1,
            2 => ALPFR::ALPF_2,
            3 => ALPFR::ALPF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ALPF_0`"]
    #[inline]
    pub fn is_alpf_0(&self) -> bool {
        *self == ALPFR::ALPF_0
    }
    #[doc = "Checks if the value of the field is `ALPF_1`"]
    #[inline]
    pub fn is_alpf_1(&self) -> bool {
        *self == ALPFR::ALPF_1
    }
    #[doc = "Checks if the value of the field is `ALPF_2`"]
    #[inline]
    pub fn is_alpf_2(&self) -> bool {
        *self == ALPFR::ALPF_2
    }
    #[doc = "Checks if the value of the field is `ALPF_3`"]
    #[inline]
    pub fn is_alpf_3(&self) -> bool {
        *self == ALPFR::ALPF_3
    }
}
#[doc = "Possible values of the field `EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENR {
    #[doc = "No monitoring"]
    EN_0,
    #[doc = "Enable monitoring"]
    EN_1,
}
impl ENR {
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
            ENR::EN_0 => false,
            ENR::EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENR {
        match value {
            false => ENR::EN_0,
            true => ENR::EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_0`"]
    #[inline]
    pub fn is_en_0(&self) -> bool {
        *self == ENR::EN_0
    }
    #[doc = "Checks if the value of the field is `EN_1`"]
    #[inline]
    pub fn is_en_1(&self) -> bool {
        *self == ENR::EN_1
    }
}
#[doc = "Values that can be written to the field `ALPF`"]
pub enum ALPFW {
    #[doc = "1.0"]
    ALPF_0,
    #[doc = "0.5"]
    ALPF_1,
    #[doc = "0.25"]
    ALPF_2,
    #[doc = "0.125"]
    ALPF_3,
}
impl ALPFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ALPFW::ALPF_0 => 0,
            ALPFW::ALPF_1 => 1,
            ALPFW::ALPF_2 => 2,
            ALPFW::ALPF_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALPFW<'a> {
    w: &'a mut W,
}
impl<'a> _ALPFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALPFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1.0"]
    #[inline]
    pub fn alpf_0(self) -> &'a mut W {
        self.variant(ALPFW::ALPF_0)
    }
    #[doc = "0.5"]
    #[inline]
    pub fn alpf_1(self) -> &'a mut W {
        self.variant(ALPFW::ALPF_1)
    }
    #[doc = "0.25"]
    #[inline]
    pub fn alpf_2(self) -> &'a mut W {
        self.variant(ALPFW::ALPF_2)
    }
    #[doc = "0.125"]
    #[inline]
    pub fn alpf_3(self) -> &'a mut W {
        self.variant(ALPFW::ALPF_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EN`"]
pub enum ENW {
    #[doc = "No monitoring"]
    EN_0,
    #[doc = "Enable monitoring"]
    EN_1,
}
impl ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENW::EN_0 => false,
            ENW::EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No monitoring"]
    #[inline]
    pub fn en_0(self) -> &'a mut W {
        self.variant(ENW::EN_0)
    }
    #[doc = "Enable monitoring"]
    #[inline]
    pub fn en_1(self) -> &'a mut W {
        self.variant(ENW::EN_1)
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
    #[doc = "Bits 0:1 - Average low pass filter setting"]
    #[inline]
    pub fn alpf(&self) -> ALPFR {
        ALPFR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 31 - Enable monitoring the temperature sensor"]
    #[inline]
    pub fn en(&self) -> ENR {
        ENR::_from({
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
    #[doc = "Bits 0:1 - Average low pass filter setting"]
    #[inline]
    pub fn alpf(&mut self) -> _ALPFW {
        _ALPFW { w: self }
    }
    #[doc = "Bit 31 - Enable monitoring the temperature sensor"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
}
