#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MRVS {
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
#[doc = "Possible values of the field `VDID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDIDR {
    #[doc = "Processing Domain 0"]
    VDID_0,
    #[doc = "Processing Domain 1"]
    VDID_1,
    #[doc = "Processing Domain 2"]
    VDID_2,
    #[doc = "Processing Domain 3"]
    VDID_3,
}
impl VDIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VDIDR::VDID_0 => 0,
            VDIDR::VDID_1 => 1,
            VDIDR::VDID_2 => 2,
            VDIDR::VDID_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VDIDR {
        match value {
            0 => VDIDR::VDID_0,
            1 => VDIDR::VDID_1,
            2 => VDIDR::VDID_2,
            3 => VDIDR::VDID_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VDID_0`"]
    #[inline]
    pub fn is_vdid_0(&self) -> bool {
        *self == VDIDR::VDID_0
    }
    #[doc = "Checks if the value of the field is `VDID_1`"]
    #[inline]
    pub fn is_vdid_1(&self) -> bool {
        *self == VDIDR::VDID_1
    }
    #[doc = "Checks if the value of the field is `VDID_2`"]
    #[inline]
    pub fn is_vdid_2(&self) -> bool {
        *self == VDIDR::VDID_2
    }
    #[doc = "Checks if the value of the field is `VDID_3`"]
    #[inline]
    pub fn is_vdid_3(&self) -> bool {
        *self == VDIDR::VDID_3
    }
}
#[doc = r" Value of the field"]
pub struct ADR {
    bits: bool,
}
impl ADR {
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
pub struct VADRR {
    bits: u32,
}
impl VADRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _ADW<'a> {
    w: &'a mut W,
}
impl<'a> _ADW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Violating Domain ID"]
    #[inline]
    pub fn vdid(&self) -> VDIDR {
        VDIDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Access Denied"]
    #[inline]
    pub fn ad(&self) -> ADR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADR { bits }
    }
    #[doc = "Bits 5:31 - Violating Address"]
    #[inline]
    pub fn vadr(&self) -> VADRR {
        let bits = {
            const MASK: u32 = 134217727;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        VADRR { bits }
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
    #[doc = "Bit 4 - Access Denied"]
    #[inline]
    pub fn ad(&mut self) -> _ADW {
        _ADW { w: self }
    }
}
