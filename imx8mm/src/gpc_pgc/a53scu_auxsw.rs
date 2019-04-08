#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::A53SCU_AUXSW {
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
pub struct DFTRAM_TRC1_TMC_TMR_TCD2R {
    bits: u16,
}
impl DFTRAM_TRC1_TMC_TMR_TCD2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct L2RETN_RTC1_TMC_TMRR {
    bits: u16,
}
impl L2RETN_RTC1_TMC_TMRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MEMPWR_TRC1_TMCR {
    bits: u16,
}
impl MEMPWR_TRC1_TMCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DFTRAM_TRC1_TMC_TMR_TCD2W<'a> {
    w: &'a mut W,
}
impl<'a> _DFTRAM_TRC1_TMC_TMR_TCD2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _L2RETN_RTC1_TMC_TMRW<'a> {
    w: &'a mut W,
}
impl<'a> _L2RETN_RTC1_TMC_TMRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MEMPWR_TRC1_TMCW<'a> {
    w: &'a mut W,
}
impl<'a> _MEMPWR_TRC1_TMCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 20;
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
    #[doc = "Bits 0:9 - After scu starts pup reset, count this value to assert a53 l2 memory dftram to 1'b0."]
    #[inline]
    pub fn dftram_trc1_tmc_tmr_tcd2(&self) -> DFTRAM_TRC1_TMC_TMR_TCD2R {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DFTRAM_TRC1_TMC_TMR_TCD2R { bits }
    }
    #[doc = "Bits 10:19 - After scu starts pup reset, count this value to assert a53 l2 memory retention to 1'b1."]
    #[inline]
    pub fn l2retn_rtc1_tmc_tmr(&self) -> L2RETN_RTC1_TMC_TMRR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        L2RETN_RTC1_TMC_TMRR { bits }
    }
    #[doc = "Bits 20:29 - After scu starts pup reset, count this value to assert a53 l2 memory switch to 1'b0."]
    #[inline]
    pub fn mempwr_trc1_tmc(&self) -> MEMPWR_TRC1_TMCR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MEMPWR_TRC1_TMCR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 305 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:9 - After scu starts pup reset, count this value to assert a53 l2 memory dftram to 1'b0."]
    #[inline]
    pub fn dftram_trc1_tmc_tmr_tcd2(&mut self) -> _DFTRAM_TRC1_TMC_TMR_TCD2W {
        _DFTRAM_TRC1_TMC_TMR_TCD2W { w: self }
    }
    #[doc = "Bits 10:19 - After scu starts pup reset, count this value to assert a53 l2 memory retention to 1'b1."]
    #[inline]
    pub fn l2retn_rtc1_tmc_tmr(&mut self) -> _L2RETN_RTC1_TMC_TMRW {
        _L2RETN_RTC1_TMC_TMRW { w: self }
    }
    #[doc = "Bits 20:29 - After scu starts pup reset, count this value to assert a53 l2 memory switch to 1'b0."]
    #[inline]
    pub fn mempwr_trc1_tmc(&mut self) -> _MEMPWR_TRC1_TMCW {
        _MEMPWR_TRC1_TMCW { w: self }
    }
}
