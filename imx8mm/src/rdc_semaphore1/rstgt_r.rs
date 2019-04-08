#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::RSTGT_R {
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
#[doc = r" Value of the field"]
pub struct RSTGMSR {
    bits: u8,
}
impl RSTGMSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RSTGSM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTGSMR {
    #[doc = "Idle, waiting for the first data pattern write."]
    RSTGSM_0,
    #[doc = "Waiting for the second data pattern write."]
    RSTGSM_1,
    #[doc = "The 2-write sequence has completed. Generate the specified gate reset(s). After the reset is performed, this machine returns to the idle (waiting for first data pattern write) state. The \"01\" state persists for only one clock cycle. Software will never be able to observe this state."]
    RSTGSM_2,
    #[doc = "This state encoding is never used and therefore reserved."]
    RSTGSM_3,
}
impl RSTGSMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RSTGSMR::RSTGSM_0 => 0,
            RSTGSMR::RSTGSM_1 => 1,
            RSTGSMR::RSTGSM_2 => 2,
            RSTGSMR::RSTGSM_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RSTGSMR {
        match value {
            0 => RSTGSMR::RSTGSM_0,
            1 => RSTGSMR::RSTGSM_1,
            2 => RSTGSMR::RSTGSM_2,
            3 => RSTGSMR::RSTGSM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RSTGSM_0`"]
    #[inline]
    pub fn is_rstgsm_0(&self) -> bool {
        *self == RSTGSMR::RSTGSM_0
    }
    #[doc = "Checks if the value of the field is `RSTGSM_1`"]
    #[inline]
    pub fn is_rstgsm_1(&self) -> bool {
        *self == RSTGSMR::RSTGSM_1
    }
    #[doc = "Checks if the value of the field is `RSTGSM_2`"]
    #[inline]
    pub fn is_rstgsm_2(&self) -> bool {
        *self == RSTGSMR::RSTGSM_2
    }
    #[doc = "Checks if the value of the field is `RSTGSM_3`"]
    #[inline]
    pub fn is_rstgsm_3(&self) -> bool {
        *self == RSTGSMR::RSTGSM_3
    }
}
#[doc = r" Value of the field"]
pub struct RSTGTNR {
    bits: u8,
}
impl RSTGTNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RSTGTNW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTGTNW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:3 - Reset Gate Bus Master"]
    #[inline]
    pub fn rstgms(&self) -> RSTGMSR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        RSTGMSR { bits }
    }
    #[doc = "Bits 4:5 - Reset Gate Finite State Machine"]
    #[inline]
    pub fn rstgsm(&self) -> RSTGSMR {
        RSTGSMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 8:15 - Reset Gate Number"]
    #[inline]
    pub fn rstgtn(&self) -> RSTGTNR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        RSTGTNR { bits }
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 8:15 - Reset Gate Number"]
    #[inline]
    pub fn rstgtn(&mut self) -> _RSTGTNW {
        _RSTGTNW { w: self }
    }
}
