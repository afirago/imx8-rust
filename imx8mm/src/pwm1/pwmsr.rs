#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWMSR {
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
#[doc = "Possible values of the field `FIFOAV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOAVR {
    #[doc = "No data available"]
    FIFOAV_0,
    #[doc = "1 word of data in FIFO"]
    FIFOAV_1,
    #[doc = "2 words of data in FIFO"]
    FIFOAV_2,
    #[doc = "3 words of data in FIFO"]
    FIFOAV_3,
    #[doc = "4 words of data in FIFO"]
    FIFOAV_4,
    #[doc = "unused"]
    FIFOAV_5,
    #[doc = "unused"]
    FIFOAV_6,
    #[doc = "unused"]
    FIFOAV_7,
}
impl FIFOAVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FIFOAVR::FIFOAV_0 => 0,
            FIFOAVR::FIFOAV_1 => 1,
            FIFOAVR::FIFOAV_2 => 2,
            FIFOAVR::FIFOAV_3 => 3,
            FIFOAVR::FIFOAV_4 => 4,
            FIFOAVR::FIFOAV_5 => 5,
            FIFOAVR::FIFOAV_6 => 6,
            FIFOAVR::FIFOAV_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FIFOAVR {
        match value {
            0 => FIFOAVR::FIFOAV_0,
            1 => FIFOAVR::FIFOAV_1,
            2 => FIFOAVR::FIFOAV_2,
            3 => FIFOAVR::FIFOAV_3,
            4 => FIFOAVR::FIFOAV_4,
            5 => FIFOAVR::FIFOAV_5,
            6 => FIFOAVR::FIFOAV_6,
            7 => FIFOAVR::FIFOAV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FIFOAV_0`"]
    #[inline]
    pub fn is_fifoav_0(&self) -> bool {
        *self == FIFOAVR::FIFOAV_0
    }
    #[doc = "Checks if the value of the field is `FIFOAV_1`"]
    #[inline]
    pub fn is_fifoav_1(&self) -> bool {
        *self == FIFOAVR::FIFOAV_1
    }
    #[doc = "Checks if the value of the field is `FIFOAV_2`"]
    #[inline]
    pub fn is_fifoav_2(&self) -> bool {
        *self == FIFOAVR::FIFOAV_2
    }
    #[doc = "Checks if the value of the field is `FIFOAV_3`"]
    #[inline]
    pub fn is_fifoav_3(&self) -> bool {
        *self == FIFOAVR::FIFOAV_3
    }
    #[doc = "Checks if the value of the field is `FIFOAV_4`"]
    #[inline]
    pub fn is_fifoav_4(&self) -> bool {
        *self == FIFOAVR::FIFOAV_4
    }
    #[doc = "Checks if the value of the field is `FIFOAV_5`"]
    #[inline]
    pub fn is_fifoav_5(&self) -> bool {
        *self == FIFOAVR::FIFOAV_5
    }
    #[doc = "Checks if the value of the field is `FIFOAV_6`"]
    #[inline]
    pub fn is_fifoav_6(&self) -> bool {
        *self == FIFOAVR::FIFOAV_6
    }
    #[doc = "Checks if the value of the field is `FIFOAV_7`"]
    #[inline]
    pub fn is_fifoav_7(&self) -> bool {
        *self == FIFOAVR::FIFOAV_7
    }
}
#[doc = "Possible values of the field `FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FER {
    #[doc = "Data level is above water mark"]
    FE_0,
    #[doc = "When the data level falls below the mark set by FWM field"]
    FE_1,
}
impl FER {
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
            FER::FE_0 => false,
            FER::FE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FER {
        match value {
            false => FER::FE_0,
            true => FER::FE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FE_0`"]
    #[inline]
    pub fn is_fe_0(&self) -> bool {
        *self == FER::FE_0
    }
    #[doc = "Checks if the value of the field is `FE_1`"]
    #[inline]
    pub fn is_fe_1(&self) -> bool {
        *self == FER::FE_1
    }
}
#[doc = "Possible values of the field `ROV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROVR {
    #[doc = "Roll-over event not occurred"]
    ROV_0,
    #[doc = "Roll-over event occurred"]
    ROV_1,
}
impl ROVR {
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
            ROVR::ROV_0 => false,
            ROVR::ROV_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ROVR {
        match value {
            false => ROVR::ROV_0,
            true => ROVR::ROV_1,
        }
    }
    #[doc = "Checks if the value of the field is `ROV_0`"]
    #[inline]
    pub fn is_rov_0(&self) -> bool {
        *self == ROVR::ROV_0
    }
    #[doc = "Checks if the value of the field is `ROV_1`"]
    #[inline]
    pub fn is_rov_1(&self) -> bool {
        *self == ROVR::ROV_1
    }
}
#[doc = "Possible values of the field `CMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPR {
    #[doc = "Compare event not occurred"]
    CMP_0,
    #[doc = "Compare event occurred"]
    CMP_1,
}
impl CMPR {
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
            CMPR::CMP_0 => false,
            CMPR::CMP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMPR {
        match value {
            false => CMPR::CMP_0,
            true => CMPR::CMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `CMP_0`"]
    #[inline]
    pub fn is_cmp_0(&self) -> bool {
        *self == CMPR::CMP_0
    }
    #[doc = "Checks if the value of the field is `CMP_1`"]
    #[inline]
    pub fn is_cmp_1(&self) -> bool {
        *self == CMPR::CMP_1
    }
}
#[doc = "Possible values of the field `FWE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWER {
    #[doc = "FIFO write error not occurred"]
    FWE_0,
    #[doc = "FIFO write error occurred"]
    FWE_1,
}
impl FWER {
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
            FWER::FWE_0 => false,
            FWER::FWE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FWER {
        match value {
            false => FWER::FWE_0,
            true => FWER::FWE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FWE_0`"]
    #[inline]
    pub fn is_fwe_0(&self) -> bool {
        *self == FWER::FWE_0
    }
    #[doc = "Checks if the value of the field is `FWE_1`"]
    #[inline]
    pub fn is_fwe_1(&self) -> bool {
        *self == FWER::FWE_1
    }
}
#[doc = "Values that can be written to the field `FE`"]
pub enum FEW {
    #[doc = "Data level is above water mark"]
    FE_0,
    #[doc = "When the data level falls below the mark set by FWM field"]
    FE_1,
}
impl FEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FEW::FE_0 => false,
            FEW::FE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FEW<'a> {
    w: &'a mut W,
}
impl<'a> _FEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data level is above water mark"]
    #[inline]
    pub fn fe_0(self) -> &'a mut W {
        self.variant(FEW::FE_0)
    }
    #[doc = "When the data level falls below the mark set by FWM field"]
    #[inline]
    pub fn fe_1(self) -> &'a mut W {
        self.variant(FEW::FE_1)
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
#[doc = "Values that can be written to the field `ROV`"]
pub enum ROVW {
    #[doc = "Roll-over event not occurred"]
    ROV_0,
    #[doc = "Roll-over event occurred"]
    ROV_1,
}
impl ROVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ROVW::ROV_0 => false,
            ROVW::ROV_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ROVW<'a> {
    w: &'a mut W,
}
impl<'a> _ROVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ROVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Roll-over event not occurred"]
    #[inline]
    pub fn rov_0(self) -> &'a mut W {
        self.variant(ROVW::ROV_0)
    }
    #[doc = "Roll-over event occurred"]
    #[inline]
    pub fn rov_1(self) -> &'a mut W {
        self.variant(ROVW::ROV_1)
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
#[doc = "Values that can be written to the field `CMP`"]
pub enum CMPW {
    #[doc = "Compare event not occurred"]
    CMP_0,
    #[doc = "Compare event occurred"]
    CMP_1,
}
impl CMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMPW::CMP_0 => false,
            CMPW::CMP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMPW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Compare event not occurred"]
    #[inline]
    pub fn cmp_0(self) -> &'a mut W {
        self.variant(CMPW::CMP_0)
    }
    #[doc = "Compare event occurred"]
    #[inline]
    pub fn cmp_1(self) -> &'a mut W {
        self.variant(CMPW::CMP_1)
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
#[doc = "Values that can be written to the field `FWE`"]
pub enum FWEW {
    #[doc = "FIFO write error not occurred"]
    FWE_0,
    #[doc = "FIFO write error occurred"]
    FWE_1,
}
impl FWEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FWEW::FWE_0 => false,
            FWEW::FWE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FWEW<'a> {
    w: &'a mut W,
}
impl<'a> _FWEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FWEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FIFO write error not occurred"]
    #[inline]
    pub fn fwe_0(self) -> &'a mut W {
        self.variant(FWEW::FWE_0)
    }
    #[doc = "FIFO write error occurred"]
    #[inline]
    pub fn fwe_1(self) -> &'a mut W {
        self.variant(FWEW::FWE_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - FIFO Available"]
    #[inline]
    pub fn fifoav(&self) -> FIFOAVR {
        FIFOAVR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - FIFO Empty Status Bit"]
    #[inline]
    pub fn fe(&self) -> FER {
        FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Roll-over Status. This bit shows that a roll-over event has occurred."]
    #[inline]
    pub fn rov(&self) -> ROVR {
        ROVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Compare Status. This bit shows that a compare event has occurred."]
    #[inline]
    pub fn cmp(&self) -> CMPR {
        CMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - FIFO Write Error Status"]
    #[inline]
    pub fn fwe(&self) -> FWER {
        FWER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 8 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 3 - FIFO Empty Status Bit"]
    #[inline]
    pub fn fe(&mut self) -> _FEW {
        _FEW { w: self }
    }
    #[doc = "Bit 4 - Roll-over Status. This bit shows that a roll-over event has occurred."]
    #[inline]
    pub fn rov(&mut self) -> _ROVW {
        _ROVW { w: self }
    }
    #[doc = "Bit 5 - Compare Status. This bit shows that a compare event has occurred."]
    #[inline]
    pub fn cmp(&mut self) -> _CMPW {
        _CMPW { w: self }
    }
    #[doc = "Bit 6 - FIFO Write Error Status"]
    #[inline]
    pub fn fwe(&mut self) -> _FWEW {
        _FWEW { w: self }
    }
}
