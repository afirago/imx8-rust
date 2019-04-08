#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SW_PAD_CTL_PAD_GPIO1_IO03 {
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
#[doc = "Possible values of the field `DSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSER {
    #[doc = "Drive strength X1"]
    X1,
    #[doc = "Drive strength X2"]
    X2,
    #[doc = "Drive strength X6"]
    X6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DSER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DSER::X1 => 0,
            DSER::X2 => 4,
            DSER::X6 => 6,
            DSER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DSER {
        match value {
            0 => DSER::X1,
            4 => DSER::X2,
            6 => DSER::X6,
            i => DSER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `X1`"]
    #[inline]
    pub fn is_x1(&self) -> bool {
        *self == DSER::X1
    }
    #[doc = "Checks if the value of the field is `X2`"]
    #[inline]
    pub fn is_x2(&self) -> bool {
        *self == DSER::X2
    }
    #[doc = "Checks if the value of the field is `X6`"]
    #[inline]
    pub fn is_x6(&self) -> bool {
        *self == DSER::X6
    }
}
#[doc = "Possible values of the field `FSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSELR {
    #[doc = "Select slow slew rate (SR=1)"]
    SLOW,
    #[doc = "Select fast slew rate (SR=0)"]
    FAST,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FSELR::SLOW => 0,
            FSELR::FAST => 2,
            FSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FSELR {
        match value {
            0 => FSELR::SLOW,
            2 => FSELR::FAST,
            i => FSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SLOW`"]
    #[inline]
    pub fn is_slow(&self) -> bool {
        *self == FSELR::SLOW
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline]
    pub fn is_fast(&self) -> bool {
        *self == FSELR::FAST
    }
}
#[doc = "Possible values of the field `ODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ODER {
    #[doc = "Disable open-drain mode"]
    DISABLED,
    #[doc = "Enable open-drain mode"]
    ENABLED,
}
impl ODER {
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
            ODER::DISABLED => false,
            ODER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ODER {
        match value {
            false => ODER::DISABLED,
            true => ODER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ODER::ENABLED
    }
}
#[doc = "Possible values of the field `PUE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUER {
    #[doc = "Select pull-down resistors"]
    DISABLED,
    #[doc = "Select pull-up resistors"]
    ENABLED,
}
impl PUER {
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
            PUER::DISABLED => false,
            PUER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PUER {
        match value {
            false => PUER::DISABLED,
            true => PUER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PUER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PUER::ENABLED
    }
}
#[doc = "Possible values of the field `HYS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYSR {
    #[doc = "Select CMOS input"]
    DISABLED,
    #[doc = "Select Schmitt input"]
    ENABLED,
}
impl HYSR {
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
            HYSR::DISABLED => false,
            HYSR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HYSR {
        match value {
            false => HYSR::DISABLED,
            true => HYSR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == HYSR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == HYSR::ENABLED
    }
}
#[doc = "Possible values of the field `PE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER {
    #[doc = "Disable pull resistors"]
    DISABLED,
    #[doc = "Enable pull resistors"]
    ENABLED,
}
impl PER {
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
            PER::DISABLED => false,
            PER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PER {
        match value {
            false => PER::DISABLED,
            true => PER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PER::ENABLED
    }
}
#[doc = "Values that can be written to the field `DSE`"]
pub enum DSEW {
    #[doc = "Drive strength X1"]
    X1,
    #[doc = "Drive strength X2"]
    X2,
    #[doc = "Drive strength X6"]
    X6,
}
impl DSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DSEW::X1 => 0,
            DSEW::X2 => 4,
            DSEW::X6 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSEW<'a> {
    w: &'a mut W,
}
impl<'a> _DSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Drive strength X1"]
    #[inline]
    pub fn x1(self) -> &'a mut W {
        self.variant(DSEW::X1)
    }
    #[doc = "Drive strength X2"]
    #[inline]
    pub fn x2(self) -> &'a mut W {
        self.variant(DSEW::X2)
    }
    #[doc = "Drive strength X6"]
    #[inline]
    pub fn x6(self) -> &'a mut W {
        self.variant(DSEW::X6)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FSEL`"]
pub enum FSELW {
    #[doc = "Select slow slew rate (SR=1)"]
    SLOW,
    #[doc = "Select fast slew rate (SR=0)"]
    FAST,
}
impl FSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FSELW::SLOW => 0,
            FSELW::FAST => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select slow slew rate (SR=1)"]
    #[inline]
    pub fn slow(self) -> &'a mut W {
        self.variant(FSELW::SLOW)
    }
    #[doc = "Select fast slew rate (SR=0)"]
    #[inline]
    pub fn fast(self) -> &'a mut W {
        self.variant(FSELW::FAST)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ODE`"]
pub enum ODEW {
    #[doc = "Disable open-drain mode"]
    DISABLED,
    #[doc = "Enable open-drain mode"]
    ENABLED,
}
impl ODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ODEW::DISABLED => false,
            ODEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable open-drain mode"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ODEW::DISABLED)
    }
    #[doc = "Enable open-drain mode"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ODEW::ENABLED)
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
#[doc = "Values that can be written to the field `PUE`"]
pub enum PUEW {
    #[doc = "Select pull-down resistors"]
    DISABLED,
    #[doc = "Select pull-up resistors"]
    ENABLED,
}
impl PUEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PUEW::DISABLED => false,
            PUEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PUEW<'a> {
    w: &'a mut W,
}
impl<'a> _PUEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Select pull-down resistors"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PUEW::DISABLED)
    }
    #[doc = "Select pull-up resistors"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PUEW::ENABLED)
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
#[doc = "Values that can be written to the field `HYS`"]
pub enum HYSW {
    #[doc = "Select CMOS input"]
    DISABLED,
    #[doc = "Select Schmitt input"]
    ENABLED,
}
impl HYSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HYSW::DISABLED => false,
            HYSW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HYSW<'a> {
    w: &'a mut W,
}
impl<'a> _HYSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HYSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Select CMOS input"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HYSW::DISABLED)
    }
    #[doc = "Select Schmitt input"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HYSW::ENABLED)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PE`"]
pub enum PEW {
    #[doc = "Disable pull resistors"]
    DISABLED,
    #[doc = "Enable pull resistors"]
    ENABLED,
}
impl PEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEW::DISABLED => false,
            PEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEW<'a> {
    w: &'a mut W,
}
impl<'a> _PEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable pull resistors"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PEW::DISABLED)
    }
    #[doc = "Enable pull resistors"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PEW::ENABLED)
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
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:2 - Drive Strength Field"]
    #[inline]
    pub fn dse(&self) -> DSER {
        DSER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:4 - Slew Rate Field"]
    #[inline]
    pub fn fsel(&self) -> FSELR {
        FSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Open Drain Enable Field"]
    #[inline]
    pub fn ode(&self) -> ODER {
        ODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Control IO ports PS:"]
    #[inline]
    pub fn pue(&self) -> PUER {
        PUER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Hysteresis Enable Field"]
    #[inline]
    pub fn hys(&self) -> HYSR {
        HYSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Pull Resistors Enable Field"]
    #[inline]
    pub fn pe(&self) -> PER {
        PER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 278 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Drive Strength Field"]
    #[inline]
    pub fn dse(&mut self) -> _DSEW {
        _DSEW { w: self }
    }
    #[doc = "Bits 3:4 - Slew Rate Field"]
    #[inline]
    pub fn fsel(&mut self) -> _FSELW {
        _FSELW { w: self }
    }
    #[doc = "Bit 5 - Open Drain Enable Field"]
    #[inline]
    pub fn ode(&mut self) -> _ODEW {
        _ODEW { w: self }
    }
    #[doc = "Bit 6 - Control IO ports PS:"]
    #[inline]
    pub fn pue(&mut self) -> _PUEW {
        _PUEW { w: self }
    }
    #[doc = "Bit 7 - Hysteresis Enable Field"]
    #[inline]
    pub fn hys(&mut self) -> _HYSW {
        _HYSW { w: self }
    }
    #[doc = "Bit 8 - Pull Resistors Enable Field"]
    #[inline]
    pub fn pe(&mut self) -> _PEW {
        _PEW { w: self }
    }
}
