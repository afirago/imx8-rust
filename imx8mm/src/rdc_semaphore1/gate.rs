#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::GATE {
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
    #[doc = "The gate has been locked by processor with master_index = 0."]
    GTFSM_1,
    #[doc = "The gate has been locked by processor with master_index = 1."]
    GTFSM_2,
    #[doc = "The gate has been locked by processor with master_index = 2."]
    GTFSM_3,
    #[doc = "The gate has been locked by processor with master_index = 3."]
    GTFSM_4,
    #[doc = "The gate has been locked by processor with master_index = 4."]
    GTFSM_5,
    #[doc = "The gate has been locked by processor with master_index = 5."]
    GTFSM_6,
    #[doc = "The gate has been locked by processor with master_index = 6."]
    GTFSM_7,
    #[doc = "The gate has been locked by processor with master_index = 7."]
    GTFSM_8,
    #[doc = "The gate has been locked by processor with master_index = 8."]
    GTFSM_9,
    #[doc = "The gate has been locked by processor with master_index = 9."]
    GTFSM_10,
    #[doc = "The gate has been locked by processor with master_index = 10."]
    GTFSM_11,
    #[doc = "The gate has been locked by processor with master_index = 11."]
    GTFSM_12,
    #[doc = "The gate has been locked by processor with master_index = 12."]
    GTFSM_13,
    #[doc = "The gate has been locked by processor with master_index = 13."]
    GTFSM_14,
    #[doc = "The gate has been locked by processor with master_index = 14."]
    GTFSM_15,
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
            GTFSMR::GTFSM_4 => 4,
            GTFSMR::GTFSM_5 => 5,
            GTFSMR::GTFSM_6 => 6,
            GTFSMR::GTFSM_7 => 7,
            GTFSMR::GTFSM_8 => 8,
            GTFSMR::GTFSM_9 => 9,
            GTFSMR::GTFSM_10 => 10,
            GTFSMR::GTFSM_11 => 11,
            GTFSMR::GTFSM_12 => 12,
            GTFSMR::GTFSM_13 => 13,
            GTFSMR::GTFSM_14 => 14,
            GTFSMR::GTFSM_15 => 15,
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
            4 => GTFSMR::GTFSM_4,
            5 => GTFSMR::GTFSM_5,
            6 => GTFSMR::GTFSM_6,
            7 => GTFSMR::GTFSM_7,
            8 => GTFSMR::GTFSM_8,
            9 => GTFSMR::GTFSM_9,
            10 => GTFSMR::GTFSM_10,
            11 => GTFSMR::GTFSM_11,
            12 => GTFSMR::GTFSM_12,
            13 => GTFSMR::GTFSM_13,
            14 => GTFSMR::GTFSM_14,
            15 => GTFSMR::GTFSM_15,
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
    #[doc = "Checks if the value of the field is `GTFSM_4`"]
    #[inline]
    pub fn is_gtfsm_4(&self) -> bool {
        *self == GTFSMR::GTFSM_4
    }
    #[doc = "Checks if the value of the field is `GTFSM_5`"]
    #[inline]
    pub fn is_gtfsm_5(&self) -> bool {
        *self == GTFSMR::GTFSM_5
    }
    #[doc = "Checks if the value of the field is `GTFSM_6`"]
    #[inline]
    pub fn is_gtfsm_6(&self) -> bool {
        *self == GTFSMR::GTFSM_6
    }
    #[doc = "Checks if the value of the field is `GTFSM_7`"]
    #[inline]
    pub fn is_gtfsm_7(&self) -> bool {
        *self == GTFSMR::GTFSM_7
    }
    #[doc = "Checks if the value of the field is `GTFSM_8`"]
    #[inline]
    pub fn is_gtfsm_8(&self) -> bool {
        *self == GTFSMR::GTFSM_8
    }
    #[doc = "Checks if the value of the field is `GTFSM_9`"]
    #[inline]
    pub fn is_gtfsm_9(&self) -> bool {
        *self == GTFSMR::GTFSM_9
    }
    #[doc = "Checks if the value of the field is `GTFSM_10`"]
    #[inline]
    pub fn is_gtfsm_10(&self) -> bool {
        *self == GTFSMR::GTFSM_10
    }
    #[doc = "Checks if the value of the field is `GTFSM_11`"]
    #[inline]
    pub fn is_gtfsm_11(&self) -> bool {
        *self == GTFSMR::GTFSM_11
    }
    #[doc = "Checks if the value of the field is `GTFSM_12`"]
    #[inline]
    pub fn is_gtfsm_12(&self) -> bool {
        *self == GTFSMR::GTFSM_12
    }
    #[doc = "Checks if the value of the field is `GTFSM_13`"]
    #[inline]
    pub fn is_gtfsm_13(&self) -> bool {
        *self == GTFSMR::GTFSM_13
    }
    #[doc = "Checks if the value of the field is `GTFSM_14`"]
    #[inline]
    pub fn is_gtfsm_14(&self) -> bool {
        *self == GTFSMR::GTFSM_14
    }
    #[doc = "Checks if the value of the field is `GTFSM_15`"]
    #[inline]
    pub fn is_gtfsm_15(&self) -> bool {
        *self == GTFSMR::GTFSM_15
    }
}
#[doc = "Possible values of the field `LDOM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDOMR {
    #[doc = "The gate is locked by domain 0. (True if bits \\[3:0\\] do not equal 0000.)"]
    LDOM_0,
    #[doc = "The gate has been locked by domain 1."]
    LDOM_1,
    #[doc = "The gate has been locked by domain 2."]
    LDOM_2,
    #[doc = "The gate has been locked by domain 3."]
    LDOM_3,
}
impl LDOMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LDOMR::LDOM_0 => 0,
            LDOMR::LDOM_1 => 1,
            LDOMR::LDOM_2 => 2,
            LDOMR::LDOM_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LDOMR {
        match value {
            0 => LDOMR::LDOM_0,
            1 => LDOMR::LDOM_1,
            2 => LDOMR::LDOM_2,
            3 => LDOMR::LDOM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LDOM_0`"]
    #[inline]
    pub fn is_ldom_0(&self) -> bool {
        *self == LDOMR::LDOM_0
    }
    #[doc = "Checks if the value of the field is `LDOM_1`"]
    #[inline]
    pub fn is_ldom_1(&self) -> bool {
        *self == LDOMR::LDOM_1
    }
    #[doc = "Checks if the value of the field is `LDOM_2`"]
    #[inline]
    pub fn is_ldom_2(&self) -> bool {
        *self == LDOMR::LDOM_2
    }
    #[doc = "Checks if the value of the field is `LDOM_3`"]
    #[inline]
    pub fn is_ldom_3(&self) -> bool {
        *self == LDOMR::LDOM_3
    }
}
#[doc = "Values that can be written to the field `GTFSM`"]
pub enum GTFSMW {
    #[doc = "The gate is unlocked (free)."]
    GTFSM_0,
    #[doc = "The gate has been locked by processor with master_index = 0."]
    GTFSM_1,
    #[doc = "The gate has been locked by processor with master_index = 1."]
    GTFSM_2,
    #[doc = "The gate has been locked by processor with master_index = 2."]
    GTFSM_3,
    #[doc = "The gate has been locked by processor with master_index = 3."]
    GTFSM_4,
    #[doc = "The gate has been locked by processor with master_index = 4."]
    GTFSM_5,
    #[doc = "The gate has been locked by processor with master_index = 5."]
    GTFSM_6,
    #[doc = "The gate has been locked by processor with master_index = 6."]
    GTFSM_7,
    #[doc = "The gate has been locked by processor with master_index = 7."]
    GTFSM_8,
    #[doc = "The gate has been locked by processor with master_index = 8."]
    GTFSM_9,
    #[doc = "The gate has been locked by processor with master_index = 9."]
    GTFSM_10,
    #[doc = "The gate has been locked by processor with master_index = 10."]
    GTFSM_11,
    #[doc = "The gate has been locked by processor with master_index = 11."]
    GTFSM_12,
    #[doc = "The gate has been locked by processor with master_index = 12."]
    GTFSM_13,
    #[doc = "The gate has been locked by processor with master_index = 13."]
    GTFSM_14,
    #[doc = "The gate has been locked by processor with master_index = 14."]
    GTFSM_15,
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
            GTFSMW::GTFSM_4 => 4,
            GTFSMW::GTFSM_5 => 5,
            GTFSMW::GTFSM_6 => 6,
            GTFSMW::GTFSM_7 => 7,
            GTFSMW::GTFSM_8 => 8,
            GTFSMW::GTFSM_9 => 9,
            GTFSMW::GTFSM_10 => 10,
            GTFSMW::GTFSM_11 => 11,
            GTFSMW::GTFSM_12 => 12,
            GTFSMW::GTFSM_13 => 13,
            GTFSMW::GTFSM_14 => 14,
            GTFSMW::GTFSM_15 => 15,
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
    #[doc = "The gate has been locked by processor with master_index = 0."]
    #[inline]
    pub fn gtfsm_1(self) -> &'a mut W {
        self.variant(GTFSMW::GTFSM_1)
    }
    #[doc = "The gate has been locked by processor with master_index = 1."]
    #[inline]
    pub fn gtfsm_2(self) -> &'a mut W {
        self.variant(GTFSMW::GTFSM_2)
    }
    #[doc = "The gate has been locked by processor with master_index = 2."]
    #[inline]
    pub fn gtfsm_3(self) -> &'a mut W {
        self.variant(GTFSMW::GTFSM_3)
    }
    #[doc = "The gate has been locked by processor with master_index = 3."]
    #[inline]
    pub fn gtfsm_4(self) -> &'a mut W {
        self.variant(GTFSMW::GTFSM_4)
    }
    #[doc = "The gate has been locked by processor with master_index = 4."]
    #[inline]
    pub fn gtfsm_5(self) -> &'a mut W {
        self.variant(GTFSMW::GTFSM_5)
    }
    #[doc = "The gate has been locked by processor with master_index = 5."]
    #[inline]
    pub fn gtfsm_6(self) -> &'a mut W {
        self.variant(GTFSMW::GTFSM_6)
    }
    #[doc = "The gate has been locked by processor with master_index = 6."]
    #[inline]
    pub fn gtfsm_7(self) -> &'a mut W {
        self.variant(GTFSMW::GTFSM_7)
    }
    #[doc = "The gate has been locked by processor with master_index = 7."]
    #[inline]
    pub fn gtfsm_8(self) -> &'a mut W {
        self.variant(GTFSMW::GTFSM_8)
    }
    #[doc = "The gate has been locked by processor with master_index = 8."]
    #[inline]
    pub fn gtfsm_9(self) -> &'a mut W {
        self.variant(GTFSMW::GTFSM_9)
    }
    #[doc = "The gate has been locked by processor with master_index = 9."]
    #[inline]
    pub fn gtfsm_10(self) -> &'a mut W {
        self.variant(GTFSMW::GTFSM_10)
    }
    #[doc = "The gate has been locked by processor with master_index = 10."]
    #[inline]
    pub fn gtfsm_11(self) -> &'a mut W {
        self.variant(GTFSMW::GTFSM_11)
    }
    #[doc = "The gate has been locked by processor with master_index = 11."]
    #[inline]
    pub fn gtfsm_12(self) -> &'a mut W {
        self.variant(GTFSMW::GTFSM_12)
    }
    #[doc = "The gate has been locked by processor with master_index = 12."]
    #[inline]
    pub fn gtfsm_13(self) -> &'a mut W {
        self.variant(GTFSMW::GTFSM_13)
    }
    #[doc = "The gate has been locked by processor with master_index = 13."]
    #[inline]
    pub fn gtfsm_14(self) -> &'a mut W {
        self.variant(GTFSMW::GTFSM_14)
    }
    #[doc = "The gate has been locked by processor with master_index = 14."]
    #[inline]
    pub fn gtfsm_15(self) -> &'a mut W {
        self.variant(GTFSMW::GTFSM_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - Gate Finite State Machine."]
    #[inline]
    pub fn gtfsm(&self) -> GTFSMR {
        GTFSMR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 4:5 - Read-only bits. They indicate which domain had currently locked the gate."]
    #[inline]
    pub fn ldom(&self) -> LDOMR {
        LDOMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:3 - Gate Finite State Machine."]
    #[inline]
    pub fn gtfsm(&mut self) -> _GTFSMW {
        _GTFSMW { w: self }
    }
}
