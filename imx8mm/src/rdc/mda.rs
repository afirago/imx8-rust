#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MDA {
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
#[doc = "Possible values of the field `DID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIDR {
    #[doc = "Master assigned to Processing Domain 0"]
    DID_0,
    #[doc = "Master assigned to Processing Domain 1"]
    DID_1,
    #[doc = "Master assigned to Processing Domain 2"]
    DID_2,
    #[doc = "Master assigned to Processing Domain 3"]
    DID_3,
}
impl DIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DIDR::DID_0 => 0,
            DIDR::DID_1 => 1,
            DIDR::DID_2 => 2,
            DIDR::DID_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DIDR {
        match value {
            0 => DIDR::DID_0,
            1 => DIDR::DID_1,
            2 => DIDR::DID_2,
            3 => DIDR::DID_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DID_0`"]
    #[inline]
    pub fn is_did_0(&self) -> bool {
        *self == DIDR::DID_0
    }
    #[doc = "Checks if the value of the field is `DID_1`"]
    #[inline]
    pub fn is_did_1(&self) -> bool {
        *self == DIDR::DID_1
    }
    #[doc = "Checks if the value of the field is `DID_2`"]
    #[inline]
    pub fn is_did_2(&self) -> bool {
        *self == DIDR::DID_2
    }
    #[doc = "Checks if the value of the field is `DID_3`"]
    #[inline]
    pub fn is_did_3(&self) -> bool {
        *self == DIDR::DID_3
    }
}
#[doc = "Possible values of the field `LCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCKR {
    #[doc = "Not Locked"]
    LCK_0,
    #[doc = "Locked"]
    LCK_1,
}
impl LCKR {
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
            LCKR::LCK_0 => false,
            LCKR::LCK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LCKR {
        match value {
            false => LCKR::LCK_0,
            true => LCKR::LCK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCK_0`"]
    #[inline]
    pub fn is_lck_0(&self) -> bool {
        *self == LCKR::LCK_0
    }
    #[doc = "Checks if the value of the field is `LCK_1`"]
    #[inline]
    pub fn is_lck_1(&self) -> bool {
        *self == LCKR::LCK_1
    }
}
#[doc = "Values that can be written to the field `DID`"]
pub enum DIDW {
    #[doc = "Master assigned to Processing Domain 0"]
    DID_0,
    #[doc = "Master assigned to Processing Domain 1"]
    DID_1,
    #[doc = "Master assigned to Processing Domain 2"]
    DID_2,
    #[doc = "Master assigned to Processing Domain 3"]
    DID_3,
}
impl DIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DIDW::DID_0 => 0,
            DIDW::DID_1 => 1,
            DIDW::DID_2 => 2,
            DIDW::DID_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIDW<'a> {
    w: &'a mut W,
}
impl<'a> _DIDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Master assigned to Processing Domain 0"]
    #[inline]
    pub fn did_0(self) -> &'a mut W {
        self.variant(DIDW::DID_0)
    }
    #[doc = "Master assigned to Processing Domain 1"]
    #[inline]
    pub fn did_1(self) -> &'a mut W {
        self.variant(DIDW::DID_1)
    }
    #[doc = "Master assigned to Processing Domain 2"]
    #[inline]
    pub fn did_2(self) -> &'a mut W {
        self.variant(DIDW::DID_2)
    }
    #[doc = "Master assigned to Processing Domain 3"]
    #[inline]
    pub fn did_3(self) -> &'a mut W {
        self.variant(DIDW::DID_3)
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
#[doc = "Values that can be written to the field `LCK`"]
pub enum LCKW {
    #[doc = "Not Locked"]
    LCK_0,
    #[doc = "Locked"]
    LCK_1,
}
impl LCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LCKW::LCK_0 => false,
            LCKW::LCK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LCKW<'a> {
    w: &'a mut W,
}
impl<'a> _LCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LCKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not Locked"]
    #[inline]
    pub fn lck_0(self) -> &'a mut W {
        self.variant(LCKW::LCK_0)
    }
    #[doc = "Locked"]
    #[inline]
    pub fn lck_1(self) -> &'a mut W {
        self.variant(LCKW::LCK_1)
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
    #[doc = "Bits 0:1 - Domain ID"]
    #[inline]
    pub fn did(&self) -> DIDR {
        DIDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 31 - no description available"]
    #[inline]
    pub fn lck(&self) -> LCKR {
        LCKR::_from({
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
    #[doc = "Bits 0:1 - Domain ID"]
    #[inline]
    pub fn did(&mut self) -> _DIDW {
        _DIDW { w: self }
    }
    #[doc = "Bit 31 - no description available"]
    #[inline]
    pub fn lck(&mut self) -> _LCKW {
        _LCKW { w: self }
    }
}
