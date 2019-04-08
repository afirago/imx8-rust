#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONFIGREG {
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
#[doc = "Possible values of the field `SCLK_PHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLK_PHAR {
    #[doc = "Phase 0 operation."]
    SCLK_PHA_0,
    #[doc = "Phase 1 operation."]
    SCLK_PHA_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SCLK_PHAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SCLK_PHAR::SCLK_PHA_0 => 0,
            SCLK_PHAR::SCLK_PHA_1 => 1,
            SCLK_PHAR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SCLK_PHAR {
        match value {
            0 => SCLK_PHAR::SCLK_PHA_0,
            1 => SCLK_PHAR::SCLK_PHA_1,
            i => SCLK_PHAR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SCLK_PHA_0`"]
    #[inline]
    pub fn is_sclk_pha_0(&self) -> bool {
        *self == SCLK_PHAR::SCLK_PHA_0
    }
    #[doc = "Checks if the value of the field is `SCLK_PHA_1`"]
    #[inline]
    pub fn is_sclk_pha_1(&self) -> bool {
        *self == SCLK_PHAR::SCLK_PHA_1
    }
}
#[doc = "Possible values of the field `SCLK_POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLK_POLR {
    #[doc = "Active high polarity (0 = Idle)."]
    SCLK_POL_0,
    #[doc = "Active low polarity (1 = Idle)."]
    SCLK_POL_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SCLK_POLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SCLK_POLR::SCLK_POL_0 => 0,
            SCLK_POLR::SCLK_POL_1 => 1,
            SCLK_POLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SCLK_POLR {
        match value {
            0 => SCLK_POLR::SCLK_POL_0,
            1 => SCLK_POLR::SCLK_POL_1,
            i => SCLK_POLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SCLK_POL_0`"]
    #[inline]
    pub fn is_sclk_pol_0(&self) -> bool {
        *self == SCLK_POLR::SCLK_POL_0
    }
    #[doc = "Checks if the value of the field is `SCLK_POL_1`"]
    #[inline]
    pub fn is_sclk_pol_1(&self) -> bool {
        *self == SCLK_POLR::SCLK_POL_1
    }
}
#[doc = "Possible values of the field `SS_CTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS_CTLR {
    #[doc = "In master mode - only one SPI burst will be transmitted."]
    SS_CTL_0,
    #[doc = "In master mode - Negate Chip Select (SS) signal between SPI bursts. Multiple SPI bursts will be transmitted. The SPI transfer will automatically stop when the TXFIFO is empty."]
    SS_CTL_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SS_CTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SS_CTLR::SS_CTL_0 => 0,
            SS_CTLR::SS_CTL_1 => 1,
            SS_CTLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SS_CTLR {
        match value {
            0 => SS_CTLR::SS_CTL_0,
            1 => SS_CTLR::SS_CTL_1,
            i => SS_CTLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SS_CTL_0`"]
    #[inline]
    pub fn is_ss_ctl_0(&self) -> bool {
        *self == SS_CTLR::SS_CTL_0
    }
    #[doc = "Checks if the value of the field is `SS_CTL_1`"]
    #[inline]
    pub fn is_ss_ctl_1(&self) -> bool {
        *self == SS_CTLR::SS_CTL_1
    }
}
#[doc = "Possible values of the field `SS_POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS_POLR {
    #[doc = "Active low."]
    SS_POL_0,
    #[doc = "Active high."]
    SS_POL_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SS_POLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SS_POLR::SS_POL_0 => 0,
            SS_POLR::SS_POL_1 => 1,
            SS_POLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SS_POLR {
        match value {
            0 => SS_POLR::SS_POL_0,
            1 => SS_POLR::SS_POL_1,
            i => SS_POLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SS_POL_0`"]
    #[inline]
    pub fn is_ss_pol_0(&self) -> bool {
        *self == SS_POLR::SS_POL_0
    }
    #[doc = "Checks if the value of the field is `SS_POL_1`"]
    #[inline]
    pub fn is_ss_pol_1(&self) -> bool {
        *self == SS_POLR::SS_POL_1
    }
}
#[doc = "Possible values of the field `DATA_CTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_CTLR {
    #[doc = "Stay high."]
    DATA_CTL_0,
    #[doc = "Stay low."]
    DATA_CTL_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DATA_CTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DATA_CTLR::DATA_CTL_0 => 0,
            DATA_CTLR::DATA_CTL_1 => 1,
            DATA_CTLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DATA_CTLR {
        match value {
            0 => DATA_CTLR::DATA_CTL_0,
            1 => DATA_CTLR::DATA_CTL_1,
            i => DATA_CTLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DATA_CTL_0`"]
    #[inline]
    pub fn is_data_ctl_0(&self) -> bool {
        *self == DATA_CTLR::DATA_CTL_0
    }
    #[doc = "Checks if the value of the field is `DATA_CTL_1`"]
    #[inline]
    pub fn is_data_ctl_1(&self) -> bool {
        *self == DATA_CTLR::DATA_CTL_1
    }
}
#[doc = "Possible values of the field `SCLK_CTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLK_CTLR {
    #[doc = "Stay low."]
    SCLK_CTL_0,
    #[doc = "Stay high."]
    SCLK_CTL_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SCLK_CTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SCLK_CTLR::SCLK_CTL_0 => 0,
            SCLK_CTLR::SCLK_CTL_1 => 1,
            SCLK_CTLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SCLK_CTLR {
        match value {
            0 => SCLK_CTLR::SCLK_CTL_0,
            1 => SCLK_CTLR::SCLK_CTL_1,
            i => SCLK_CTLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SCLK_CTL_0`"]
    #[inline]
    pub fn is_sclk_ctl_0(&self) -> bool {
        *self == SCLK_CTLR::SCLK_CTL_0
    }
    #[doc = "Checks if the value of the field is `SCLK_CTL_1`"]
    #[inline]
    pub fn is_sclk_ctl_1(&self) -> bool {
        *self == SCLK_CTLR::SCLK_CTL_1
    }
}
#[doc = r" Value of the field"]
pub struct HT_LENGTHR {
    bits: u8,
}
impl HT_LENGTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `SCLK_PHA`"]
pub enum SCLK_PHAW {
    #[doc = "Phase 0 operation."]
    SCLK_PHA_0,
    #[doc = "Phase 1 operation."]
    SCLK_PHA_1,
}
impl SCLK_PHAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SCLK_PHAW::SCLK_PHA_0 => 0,
            SCLK_PHAW::SCLK_PHA_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCLK_PHAW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLK_PHAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCLK_PHAW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Phase 0 operation."]
    #[inline]
    pub fn sclk_pha_0(self) -> &'a mut W {
        self.variant(SCLK_PHAW::SCLK_PHA_0)
    }
    #[doc = "Phase 1 operation."]
    #[inline]
    pub fn sclk_pha_1(self) -> &'a mut W {
        self.variant(SCLK_PHAW::SCLK_PHA_1)
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
#[doc = "Values that can be written to the field `SCLK_POL`"]
pub enum SCLK_POLW {
    #[doc = "Active high polarity (0 = Idle)."]
    SCLK_POL_0,
    #[doc = "Active low polarity (1 = Idle)."]
    SCLK_POL_1,
}
impl SCLK_POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SCLK_POLW::SCLK_POL_0 => 0,
            SCLK_POLW::SCLK_POL_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCLK_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLK_POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCLK_POLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Active high polarity (0 = Idle)."]
    #[inline]
    pub fn sclk_pol_0(self) -> &'a mut W {
        self.variant(SCLK_POLW::SCLK_POL_0)
    }
    #[doc = "Active low polarity (1 = Idle)."]
    #[inline]
    pub fn sclk_pol_1(self) -> &'a mut W {
        self.variant(SCLK_POLW::SCLK_POL_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SS_CTL`"]
pub enum SS_CTLW {
    #[doc = "In master mode - only one SPI burst will be transmitted."]
    SS_CTL_0,
    #[doc = "In master mode - Negate Chip Select (SS) signal between SPI bursts. Multiple SPI bursts will be transmitted. The SPI transfer will automatically stop when the TXFIFO is empty."]
    SS_CTL_1,
}
impl SS_CTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SS_CTLW::SS_CTL_0 => 0,
            SS_CTLW::SS_CTL_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SS_CTLW<'a> {
    w: &'a mut W,
}
impl<'a> _SS_CTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SS_CTLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "In master mode - only one SPI burst will be transmitted."]
    #[inline]
    pub fn ss_ctl_0(self) -> &'a mut W {
        self.variant(SS_CTLW::SS_CTL_0)
    }
    #[doc = "In master mode - Negate Chip Select (SS) signal between SPI bursts. Multiple SPI bursts will be transmitted. The SPI transfer will automatically stop when the TXFIFO is empty."]
    #[inline]
    pub fn ss_ctl_1(self) -> &'a mut W {
        self.variant(SS_CTLW::SS_CTL_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SS_POL`"]
pub enum SS_POLW {
    #[doc = "Active low."]
    SS_POL_0,
    #[doc = "Active high."]
    SS_POL_1,
}
impl SS_POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SS_POLW::SS_POL_0 => 0,
            SS_POLW::SS_POL_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SS_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _SS_POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SS_POLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Active low."]
    #[inline]
    pub fn ss_pol_0(self) -> &'a mut W {
        self.variant(SS_POLW::SS_POL_0)
    }
    #[doc = "Active high."]
    #[inline]
    pub fn ss_pol_1(self) -> &'a mut W {
        self.variant(SS_POLW::SS_POL_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DATA_CTL`"]
pub enum DATA_CTLW {
    #[doc = "Stay high."]
    DATA_CTL_0,
    #[doc = "Stay low."]
    DATA_CTL_1,
}
impl DATA_CTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DATA_CTLW::DATA_CTL_0 => 0,
            DATA_CTLW::DATA_CTL_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA_CTLW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_CTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA_CTLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Stay high."]
    #[inline]
    pub fn data_ctl_0(self) -> &'a mut W {
        self.variant(DATA_CTLW::DATA_CTL_0)
    }
    #[doc = "Stay low."]
    #[inline]
    pub fn data_ctl_1(self) -> &'a mut W {
        self.variant(DATA_CTLW::DATA_CTL_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SCLK_CTL`"]
pub enum SCLK_CTLW {
    #[doc = "Stay low."]
    SCLK_CTL_0,
    #[doc = "Stay high."]
    SCLK_CTL_1,
}
impl SCLK_CTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SCLK_CTLW::SCLK_CTL_0 => 0,
            SCLK_CTLW::SCLK_CTL_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCLK_CTLW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLK_CTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCLK_CTLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Stay low."]
    #[inline]
    pub fn sclk_ctl_0(self) -> &'a mut W {
        self.variant(SCLK_CTLW::SCLK_CTL_0)
    }
    #[doc = "Stay high."]
    #[inline]
    pub fn sclk_ctl_1(self) -> &'a mut W {
        self.variant(SCLK_CTLW::SCLK_CTL_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HT_LENGTHW<'a> {
    w: &'a mut W,
}
impl<'a> _HT_LENGTHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:3 - SPI Clock/Data Phase Control"]
    #[inline]
    pub fn sclk_pha(&self) -> SCLK_PHAR {
        SCLK_PHAR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - SPI Clock Polarity Control"]
    #[inline]
    pub fn sclk_pol(&self) -> SCLK_POLR {
        SCLK_POLR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - SPI SS Wave Form Select"]
    #[inline]
    pub fn ss_ctl(&self) -> SS_CTLR {
        SS_CTLR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - SPI SS Polarity Select"]
    #[inline]
    pub fn ss_pol(&self) -> SS_POLR {
        SS_POLR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - DATA CTL"]
    #[inline]
    pub fn data_ctl(&self) -> DATA_CTLR {
        DATA_CTLR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - SCLK CTL"]
    #[inline]
    pub fn sclk_ctl(&self) -> SCLK_CTLR {
        SCLK_CTLR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:28 - HT LENGTH"]
    #[inline]
    pub fn ht_length(&self) -> HT_LENGTHR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HT_LENGTHR { bits }
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
    #[doc = "Bits 0:3 - SPI Clock/Data Phase Control"]
    #[inline]
    pub fn sclk_pha(&mut self) -> _SCLK_PHAW {
        _SCLK_PHAW { w: self }
    }
    #[doc = "Bits 4:7 - SPI Clock Polarity Control"]
    #[inline]
    pub fn sclk_pol(&mut self) -> _SCLK_POLW {
        _SCLK_POLW { w: self }
    }
    #[doc = "Bits 8:11 - SPI SS Wave Form Select"]
    #[inline]
    pub fn ss_ctl(&mut self) -> _SS_CTLW {
        _SS_CTLW { w: self }
    }
    #[doc = "Bits 12:15 - SPI SS Polarity Select"]
    #[inline]
    pub fn ss_pol(&mut self) -> _SS_POLW {
        _SS_POLW { w: self }
    }
    #[doc = "Bits 16:19 - DATA CTL"]
    #[inline]
    pub fn data_ctl(&mut self) -> _DATA_CTLW {
        _DATA_CTLW { w: self }
    }
    #[doc = "Bits 20:23 - SCLK CTL"]
    #[inline]
    pub fn sclk_ctl(&mut self) -> _SCLK_CTLW {
        _SCLK_CTLW { w: self }
    }
    #[doc = "Bits 24:28 - HT LENGTH"]
    #[inline]
    pub fn ht_length(&mut self) -> _HT_LENGTHW {
        _HT_LENGTHW { w: self }
    }
}
