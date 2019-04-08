#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PERIODREG {
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
#[doc = "Possible values of the field `SAMPLE_PERIOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLE_PERIODR {
    #[doc = "0 wait states inserted"]
    SAMPLE_PERIOD_0,
    #[doc = "1 wait state inserted"]
    SAMPLE_PERIOD_1,
    #[doc = "32766 wait states inserted"]
    SAMPLE_PERIOD_32766,
    #[doc = "32767 wait states inserted"]
    SAMPLE_PERIOD_32767,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl SAMPLE_PERIODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            SAMPLE_PERIODR::SAMPLE_PERIOD_0 => 0,
            SAMPLE_PERIODR::SAMPLE_PERIOD_1 => 1,
            SAMPLE_PERIODR::SAMPLE_PERIOD_32766 => 32766,
            SAMPLE_PERIODR::SAMPLE_PERIOD_32767 => 32767,
            SAMPLE_PERIODR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> SAMPLE_PERIODR {
        match value {
            0 => SAMPLE_PERIODR::SAMPLE_PERIOD_0,
            1 => SAMPLE_PERIODR::SAMPLE_PERIOD_1,
            32766 => SAMPLE_PERIODR::SAMPLE_PERIOD_32766,
            32767 => SAMPLE_PERIODR::SAMPLE_PERIOD_32767,
            i => SAMPLE_PERIODR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SAMPLE_PERIOD_0`"]
    #[inline]
    pub fn is_sample_period_0(&self) -> bool {
        *self == SAMPLE_PERIODR::SAMPLE_PERIOD_0
    }
    #[doc = "Checks if the value of the field is `SAMPLE_PERIOD_1`"]
    #[inline]
    pub fn is_sample_period_1(&self) -> bool {
        *self == SAMPLE_PERIODR::SAMPLE_PERIOD_1
    }
    #[doc = "Checks if the value of the field is `SAMPLE_PERIOD_32766`"]
    #[inline]
    pub fn is_sample_period_32766(&self) -> bool {
        *self == SAMPLE_PERIODR::SAMPLE_PERIOD_32766
    }
    #[doc = "Checks if the value of the field is `SAMPLE_PERIOD_32767`"]
    #[inline]
    pub fn is_sample_period_32767(&self) -> bool {
        *self == SAMPLE_PERIODR::SAMPLE_PERIOD_32767
    }
}
#[doc = "Possible values of the field `CSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSRCR {
    #[doc = "SPI Clock (SCLK)"]
    CSRC_0,
    #[doc = "Low-Frequency Reference Clock (32.768 KHz)"]
    CSRC_1,
}
impl CSRCR {
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
            CSRCR::CSRC_0 => false,
            CSRCR::CSRC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSRCR {
        match value {
            false => CSRCR::CSRC_0,
            true => CSRCR::CSRC_1,
        }
    }
    #[doc = "Checks if the value of the field is `CSRC_0`"]
    #[inline]
    pub fn is_csrc_0(&self) -> bool {
        *self == CSRCR::CSRC_0
    }
    #[doc = "Checks if the value of the field is `CSRC_1`"]
    #[inline]
    pub fn is_csrc_1(&self) -> bool {
        *self == CSRCR::CSRC_1
    }
}
#[doc = r" Value of the field"]
pub struct CSD_CTLR {
    bits: u8,
}
impl CSD_CTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `SAMPLE_PERIOD`"]
pub enum SAMPLE_PERIODW {
    #[doc = "0 wait states inserted"]
    SAMPLE_PERIOD_0,
    #[doc = "1 wait state inserted"]
    SAMPLE_PERIOD_1,
    #[doc = "32766 wait states inserted"]
    SAMPLE_PERIOD_32766,
    #[doc = "32767 wait states inserted"]
    SAMPLE_PERIOD_32767,
}
impl SAMPLE_PERIODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            SAMPLE_PERIODW::SAMPLE_PERIOD_0 => 0,
            SAMPLE_PERIODW::SAMPLE_PERIOD_1 => 1,
            SAMPLE_PERIODW::SAMPLE_PERIOD_32766 => 32766,
            SAMPLE_PERIODW::SAMPLE_PERIOD_32767 => 32767,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAMPLE_PERIODW<'a> {
    w: &'a mut W,
}
impl<'a> _SAMPLE_PERIODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAMPLE_PERIODW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "0 wait states inserted"]
    #[inline]
    pub fn sample_period_0(self) -> &'a mut W {
        self.variant(SAMPLE_PERIODW::SAMPLE_PERIOD_0)
    }
    #[doc = "1 wait state inserted"]
    #[inline]
    pub fn sample_period_1(self) -> &'a mut W {
        self.variant(SAMPLE_PERIODW::SAMPLE_PERIOD_1)
    }
    #[doc = "32766 wait states inserted"]
    #[inline]
    pub fn sample_period_32766(self) -> &'a mut W {
        self.variant(SAMPLE_PERIODW::SAMPLE_PERIOD_32766)
    }
    #[doc = "32767 wait states inserted"]
    #[inline]
    pub fn sample_period_32767(self) -> &'a mut W {
        self.variant(SAMPLE_PERIODW::SAMPLE_PERIOD_32767)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 32767;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CSRC`"]
pub enum CSRCW {
    #[doc = "SPI Clock (SCLK)"]
    CSRC_0,
    #[doc = "Low-Frequency Reference Clock (32.768 KHz)"]
    CSRC_1,
}
impl CSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSRCW::CSRC_0 => false,
            CSRCW::CSRC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SPI Clock (SCLK)"]
    #[inline]
    pub fn csrc_0(self) -> &'a mut W {
        self.variant(CSRCW::CSRC_0)
    }
    #[doc = "Low-Frequency Reference Clock (32.768 KHz)"]
    #[inline]
    pub fn csrc_1(self) -> &'a mut W {
        self.variant(CSRCW::CSRC_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CSD_CTLW<'a> {
    w: &'a mut W,
}
impl<'a> _CSD_CTLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:14 - Sample Period Control"]
    #[inline]
    pub fn sample_period(&self) -> SAMPLE_PERIODR {
        SAMPLE_PERIODR::_from({
            const MASK: u16 = 32767;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bit 15 - Clock Source Control. This bit selects the clock source for the sample period counter."]
    #[inline]
    pub fn csrc(&self) -> CSRCR {
        CSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:21 - Chip Select Delay Control bits"]
    #[inline]
    pub fn csd_ctl(&self) -> CSD_CTLR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CSD_CTLR { bits }
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
    #[doc = "Bits 0:14 - Sample Period Control"]
    #[inline]
    pub fn sample_period(&mut self) -> _SAMPLE_PERIODW {
        _SAMPLE_PERIODW { w: self }
    }
    #[doc = "Bit 15 - Clock Source Control. This bit selects the clock source for the sample period counter."]
    #[inline]
    pub fn csrc(&mut self) -> _CSRCW {
        _CSRCW { w: self }
    }
    #[doc = "Bits 16:21 - Chip Select Delay Control bits"]
    #[inline]
    pub fn csd_ctl(&mut self) -> _CSD_CTLW {
        _CSD_CTLW { w: self }
    }
}
