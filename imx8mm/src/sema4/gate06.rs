#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::GATE06 {
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
#[doc = "Possible values of the field `GTFSM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GTFSMR {
    #[doc = "The gate is unlocked (free)."]
    GTFSM_0,
    #[doc = "The gate has been locked by processor 0."]
    GTFSM_1,
    #[doc = "The gate has been locked by processor 1."]
    GTFSM_2,
    #[doc = "This state encoding is never used and therefore reserved. Attempted writes of 0x03 are treated as \"no operation\" and do not affect the gate state machine."]
    GTFSM_3,
}
impl GTFSMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GTFSMR::GTFSM_0 => 0,
            GTFSMR::GTFSM_1 => 1,
            GTFSMR::GTFSM_2 => 2,
            GTFSMR::GTFSM_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GTFSMR {
        match value {
            0 => GTFSMR::GTFSM_0,
            1 => GTFSMR::GTFSM_1,
            2 => GTFSMR::GTFSM_2,
            3 => GTFSMR::GTFSM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GTFSM_0`"]
    #[inline]
    pub fn is_gtfsm_0(&self) -> bool {
        *self == GTFSMR::GTFSM_0
    }
    #[doc = "Checks if the value of the field is `GTFSM_1`"]
    #[inline]
    pub fn is_gtfsm_1(&self) -> bool {
        *self == GTFSMR::GTFSM_1
    }
    #[doc = "Checks if the value of the field is `GTFSM_2`"]
    #[inline]
    pub fn is_gtfsm_2(&self) -> bool {
        *self == GTFSMR::GTFSM_2
    }
    #[doc = "Checks if the value of the field is `GTFSM_3`"]
    #[inline]
    pub fn is_gtfsm_3(&self) -> bool {
        *self == GTFSMR::GTFSM_3
    }
}
#[doc = "Values that can be written to the field `GTFSM`"]
pub enum GTFSMW {
    #[doc = "The gate is unlocked (free)."]
    GTFSM_0,
    #[doc = "The gate has been locked by processor 0."]
    GTFSM_1,
    #[doc = "The gate has been locked by processor 1."]
    GTFSM_2,
    #[doc = "This state encoding is never used and therefore reserved. Attempted writes of 0x03 are treated as \"no operation\" and do not affect the gate state machine."]
    GTFSM_3,
}
impl GTFSMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GTFSMW::GTFSM_0 => 0,
            GTFSMW::GTFSM_1 => 1,
            GTFSMW::GTFSM_2 => 2,
            GTFSMW::GTFSM_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GTFSMW<'a> {
    w: &'a mut W,
}
impl<'a> _GTFSMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GTFSMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The gate is unlocked (free)."]
    #[inline]
    pub fn gtfsm_0(self) -> &'a mut W {
        self.variant(GTFSMW::GTFSM_0)
    }
    #[doc = "The gate has been locked by processor 0."]
    #[inline]
    pub fn gtfsm_1(self) -> &'a mut W {
        self.variant(GTFSMW::GTFSM_1)
    }
    #[doc = "The gate has been locked by processor 1."]
    #[inline]
    pub fn gtfsm_2(self) -> &'a mut W {
        self.variant(GTFSMW::GTFSM_2)
    }
    #[doc = "This state encoding is never used and therefore reserved. Attempted writes of 0x03 are treated as \"no operation\" and do not affect the gate state machine."]
    #[inline]
    pub fn gtfsm_3(self) -> &'a mut W {
        self.variant(GTFSMW::GTFSM_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:1 - Gate Finite State Machine."]
    #[inline]
    pub fn gtfsm(&self) -> GTFSMR {
        GTFSMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Gate Finite State Machine."]
    #[inline]
    pub fn gtfsm(&mut self) -> _GTFSMW {
        _GTFSMW { w: self }
    }
}
