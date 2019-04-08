#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ONCE_CMD {
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
#[doc = "Possible values of the field `CMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDR {
    #[doc = "rstatus"]
    CMD_0,
    #[doc = "dmov"]
    CMD_1,
    #[doc = "exec_once"]
    CMD_2,
    #[doc = "run_core"]
    CMD_3,
    #[doc = "exec_core"]
    CMD_4,
    #[doc = "debug_rqst"]
    CMD_5,
    #[doc = "rbuffer"]
    CMD_6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMDR::CMD_0 => 0,
            CMDR::CMD_1 => 1,
            CMDR::CMD_2 => 2,
            CMDR::CMD_3 => 3,
            CMDR::CMD_4 => 4,
            CMDR::CMD_5 => 5,
            CMDR::CMD_6 => 6,
            CMDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMDR {
        match value {
            0 => CMDR::CMD_0,
            1 => CMDR::CMD_1,
            2 => CMDR::CMD_2,
            3 => CMDR::CMD_3,
            4 => CMDR::CMD_4,
            5 => CMDR::CMD_5,
            6 => CMDR::CMD_6,
            i => CMDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CMD_0`"]
    #[inline]
    pub fn is_cmd_0(&self) -> bool {
        *self == CMDR::CMD_0
    }
    #[doc = "Checks if the value of the field is `CMD_1`"]
    #[inline]
    pub fn is_cmd_1(&self) -> bool {
        *self == CMDR::CMD_1
    }
    #[doc = "Checks if the value of the field is `CMD_2`"]
    #[inline]
    pub fn is_cmd_2(&self) -> bool {
        *self == CMDR::CMD_2
    }
    #[doc = "Checks if the value of the field is `CMD_3`"]
    #[inline]
    pub fn is_cmd_3(&self) -> bool {
        *self == CMDR::CMD_3
    }
    #[doc = "Checks if the value of the field is `CMD_4`"]
    #[inline]
    pub fn is_cmd_4(&self) -> bool {
        *self == CMDR::CMD_4
    }
    #[doc = "Checks if the value of the field is `CMD_5`"]
    #[inline]
    pub fn is_cmd_5(&self) -> bool {
        *self == CMDR::CMD_5
    }
    #[doc = "Checks if the value of the field is `CMD_6`"]
    #[inline]
    pub fn is_cmd_6(&self) -> bool {
        *self == CMDR::CMD_6
    }
}
#[doc = "Values that can be written to the field `CMD`"]
pub enum CMDW {
    #[doc = "rstatus"]
    CMD_0,
    #[doc = "dmov"]
    CMD_1,
    #[doc = "exec_once"]
    CMD_2,
    #[doc = "run_core"]
    CMD_3,
    #[doc = "exec_core"]
    CMD_4,
    #[doc = "debug_rqst"]
    CMD_5,
    #[doc = "rbuffer"]
    CMD_6,
}
impl CMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMDW::CMD_0 => 0,
            CMDW::CMD_1 => 1,
            CMDW::CMD_2 => 2,
            CMDW::CMD_3 => 3,
            CMDW::CMD_4 => 4,
            CMDW::CMD_5 => 5,
            CMDW::CMD_6 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "rstatus"]
    #[inline]
    pub fn cmd_0(self) -> &'a mut W {
        self.variant(CMDW::CMD_0)
    }
    #[doc = "dmov"]
    #[inline]
    pub fn cmd_1(self) -> &'a mut W {
        self.variant(CMDW::CMD_1)
    }
    #[doc = "exec_once"]
    #[inline]
    pub fn cmd_2(self) -> &'a mut W {
        self.variant(CMDW::CMD_2)
    }
    #[doc = "run_core"]
    #[inline]
    pub fn cmd_3(self) -> &'a mut W {
        self.variant(CMDW::CMD_3)
    }
    #[doc = "exec_core"]
    #[inline]
    pub fn cmd_4(self) -> &'a mut W {
        self.variant(CMDW::CMD_4)
    }
    #[doc = "debug_rqst"]
    #[inline]
    pub fn cmd_5(self) -> &'a mut W {
        self.variant(CMDW::CMD_5)
    }
    #[doc = "rbuffer"]
    #[inline]
    pub fn cmd_6(self) -> &'a mut W {
        self.variant(CMDW::CMD_6)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:3 - Writing to this register will cause the OnCE to execute the command that is written"]
    #[inline]
    pub fn cmd(&self) -> CMDR {
        CMDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:3 - Writing to this register will cause the OnCE to execute the command that is written"]
    #[inline]
    pub fn cmd(&mut self) -> _CMDW {
        _CMDW { w: self }
    }
}
