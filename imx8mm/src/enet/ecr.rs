#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ECR {
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
pub struct RESETR {
    bits: bool,
}
impl RESETR {
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
#[doc = "Possible values of the field `ETHEREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETHERENR {
    #[doc = "Reception immediately stops and transmission stops after a bad CRC is appended to any currently transmitted frame."]
    ETHEREN_0,
    #[doc = "MAC is enabled, and reception and transmission are possible."]
    ETHEREN_1,
}
impl ETHERENR {
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
            ETHERENR::ETHEREN_0 => false,
            ETHERENR::ETHEREN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ETHERENR {
        match value {
            false => ETHERENR::ETHEREN_0,
            true => ETHERENR::ETHEREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ETHEREN_0`"]
    #[inline]
    pub fn is_etheren_0(&self) -> bool {
        *self == ETHERENR::ETHEREN_0
    }
    #[doc = "Checks if the value of the field is `ETHEREN_1`"]
    #[inline]
    pub fn is_etheren_1(&self) -> bool {
        *self == ETHERENR::ETHEREN_1
    }
}
#[doc = "Possible values of the field `MAGICEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAGICENR {
    #[doc = "Magic detection logic disabled."]
    MAGICEN_0,
    #[doc = "The MAC core detects magic packets and asserts EIR\\[WAKEUP\\] when a frame is detected."]
    MAGICEN_1,
}
impl MAGICENR {
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
            MAGICENR::MAGICEN_0 => false,
            MAGICENR::MAGICEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MAGICENR {
        match value {
            false => MAGICENR::MAGICEN_0,
            true => MAGICENR::MAGICEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MAGICEN_0`"]
    #[inline]
    pub fn is_magicen_0(&self) -> bool {
        *self == MAGICENR::MAGICEN_0
    }
    #[doc = "Checks if the value of the field is `MAGICEN_1`"]
    #[inline]
    pub fn is_magicen_1(&self) -> bool {
        *self == MAGICENR::MAGICEN_1
    }
}
#[doc = "Possible values of the field `SLEEP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPR {
    #[doc = "Normal operating mode."]
    SLEEP_0,
    #[doc = "Sleep mode."]
    SLEEP_1,
}
impl SLEEPR {
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
            SLEEPR::SLEEP_0 => false,
            SLEEPR::SLEEP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLEEPR {
        match value {
            false => SLEEPR::SLEEP_0,
            true => SLEEPR::SLEEP_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLEEP_0`"]
    #[inline]
    pub fn is_sleep_0(&self) -> bool {
        *self == SLEEPR::SLEEP_0
    }
    #[doc = "Checks if the value of the field is `SLEEP_1`"]
    #[inline]
    pub fn is_sleep_1(&self) -> bool {
        *self == SLEEPR::SLEEP_1
    }
}
#[doc = "Possible values of the field `EN1588`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN1588R {
    #[doc = "Legacy FEC buffer descriptors and functions enabled."]
    EN1588_0,
    #[doc = "Enhanced frame time-stamping functions enabled."]
    EN1588_1,
}
impl EN1588R {
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
            EN1588R::EN1588_0 => false,
            EN1588R::EN1588_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN1588R {
        match value {
            false => EN1588R::EN1588_0,
            true => EN1588R::EN1588_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN1588_0`"]
    #[inline]
    pub fn is_en1588_0(&self) -> bool {
        *self == EN1588R::EN1588_0
    }
    #[doc = "Checks if the value of the field is `EN1588_1`"]
    #[inline]
    pub fn is_en1588_1(&self) -> bool {
        *self == EN1588R::EN1588_1
    }
}
#[doc = "Possible values of the field `SPEED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPEEDR {
    #[doc = "10/100-Mbit/s mode"]
    SPEED_0,
    #[doc = "1000-Mbit/s mode"]
    SPEED_1,
}
impl SPEEDR {
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
            SPEEDR::SPEED_0 => false,
            SPEEDR::SPEED_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPEEDR {
        match value {
            false => SPEEDR::SPEED_0,
            true => SPEEDR::SPEED_1,
        }
    }
    #[doc = "Checks if the value of the field is `SPEED_0`"]
    #[inline]
    pub fn is_speed_0(&self) -> bool {
        *self == SPEEDR::SPEED_0
    }
    #[doc = "Checks if the value of the field is `SPEED_1`"]
    #[inline]
    pub fn is_speed_1(&self) -> bool {
        *self == SPEEDR::SPEED_1
    }
}
#[doc = "Possible values of the field `DBGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGENR {
    #[doc = "MAC continues operation in debug mode."]
    DBGEN_0,
    #[doc = "MAC enters hardware freeze mode when the processor is in debug mode."]
    DBGEN_1,
}
impl DBGENR {
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
            DBGENR::DBGEN_0 => false,
            DBGENR::DBGEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBGENR {
        match value {
            false => DBGENR::DBGEN_0,
            true => DBGENR::DBGEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBGEN_0`"]
    #[inline]
    pub fn is_dbgen_0(&self) -> bool {
        *self == DBGENR::DBGEN_0
    }
    #[doc = "Checks if the value of the field is `DBGEN_1`"]
    #[inline]
    pub fn is_dbgen_1(&self) -> bool {
        *self == DBGENR::DBGEN_1
    }
}
#[doc = "Possible values of the field `DBSWP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBSWPR {
    #[doc = "The buffer descriptor bytes are not swapped to support big-endian devices."]
    DBSWP_0,
    #[doc = "The buffer descriptor bytes are swapped to support little-endian devices."]
    DBSWP_1,
}
impl DBSWPR {
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
            DBSWPR::DBSWP_0 => false,
            DBSWPR::DBSWP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBSWPR {
        match value {
            false => DBSWPR::DBSWP_0,
            true => DBSWPR::DBSWP_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBSWP_0`"]
    #[inline]
    pub fn is_dbswp_0(&self) -> bool {
        *self == DBSWPR::DBSWP_0
    }
    #[doc = "Checks if the value of the field is `DBSWP_1`"]
    #[inline]
    pub fn is_dbswp_1(&self) -> bool {
        *self == DBSWPR::DBSWP_1
    }
}
#[doc = "Possible values of the field `SVLANEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVLANENR {
    #[doc = "Only the EtherType 0x8100 will be considered for VLAN detection."]
    SVLANEN_0,
    #[doc = "The EtherType 0x88a8 will be considered in addition to 0x8100 (C-VLAN) to identify a VLAN frame in receive. When a VLAN frame is identified, the two bytes following the VLAN type are extracted and used by the classification match comparators, RCMRn."]
    SVLANEN_1,
}
impl SVLANENR {
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
            SVLANENR::SVLANEN_0 => false,
            SVLANENR::SVLANEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SVLANENR {
        match value {
            false => SVLANENR::SVLANEN_0,
            true => SVLANENR::SVLANEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SVLANEN_0`"]
    #[inline]
    pub fn is_svlanen_0(&self) -> bool {
        *self == SVLANENR::SVLANEN_0
    }
    #[doc = "Checks if the value of the field is `SVLANEN_1`"]
    #[inline]
    pub fn is_svlanen_1(&self) -> bool {
        *self == SVLANENR::SVLANEN_1
    }
}
#[doc = "Possible values of the field `VLANUSE2ND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLANUSE2NDR {
    #[doc = "Always extract data from the first VLAN tag if it exists."]
    VLANUSE2ND_0,
    #[doc = "When a double-tagged frame is detected, the data of the second tag is extracted for further processing. A double-tagged frame is defined as: The first tag can be a C-VLAN or a S-VLAN (if SVLAN_ENA = 1) The second tag must be a C-VLAN"]
    VLANUSE2ND_1,
}
impl VLANUSE2NDR {
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
            VLANUSE2NDR::VLANUSE2ND_0 => false,
            VLANUSE2NDR::VLANUSE2ND_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VLANUSE2NDR {
        match value {
            false => VLANUSE2NDR::VLANUSE2ND_0,
            true => VLANUSE2NDR::VLANUSE2ND_1,
        }
    }
    #[doc = "Checks if the value of the field is `VLANUSE2ND_0`"]
    #[inline]
    pub fn is_vlanuse2nd_0(&self) -> bool {
        *self == VLANUSE2NDR::VLANUSE2ND_0
    }
    #[doc = "Checks if the value of the field is `VLANUSE2ND_1`"]
    #[inline]
    pub fn is_vlanuse2nd_1(&self) -> bool {
        *self == VLANUSE2NDR::VLANUSE2ND_1
    }
}
#[doc = r" Value of the field"]
pub struct SVLANDBLR {
    bits: bool,
}
impl SVLANDBLR {
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
#[doc = r" Proxy"]
pub struct _RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _RESETW<'a> {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ETHEREN`"]
pub enum ETHERENW {
    #[doc = "Reception immediately stops and transmission stops after a bad CRC is appended to any currently transmitted frame."]
    ETHEREN_0,
    #[doc = "MAC is enabled, and reception and transmission are possible."]
    ETHEREN_1,
}
impl ETHERENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ETHERENW::ETHEREN_0 => false,
            ETHERENW::ETHEREN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ETHERENW<'a> {
    w: &'a mut W,
}
impl<'a> _ETHERENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETHERENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reception immediately stops and transmission stops after a bad CRC is appended to any currently transmitted frame."]
    #[inline]
    pub fn etheren_0(self) -> &'a mut W {
        self.variant(ETHERENW::ETHEREN_0)
    }
    #[doc = "MAC is enabled, and reception and transmission are possible."]
    #[inline]
    pub fn etheren_1(self) -> &'a mut W {
        self.variant(ETHERENW::ETHEREN_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MAGICEN`"]
pub enum MAGICENW {
    #[doc = "Magic detection logic disabled."]
    MAGICEN_0,
    #[doc = "The MAC core detects magic packets and asserts EIR\\[WAKEUP\\] when a frame is detected."]
    MAGICEN_1,
}
impl MAGICENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MAGICENW::MAGICEN_0 => false,
            MAGICENW::MAGICEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAGICENW<'a> {
    w: &'a mut W,
}
impl<'a> _MAGICENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAGICENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Magic detection logic disabled."]
    #[inline]
    pub fn magicen_0(self) -> &'a mut W {
        self.variant(MAGICENW::MAGICEN_0)
    }
    #[doc = "The MAC core detects magic packets and asserts EIR\\[WAKEUP\\] when a frame is detected."]
    #[inline]
    pub fn magicen_1(self) -> &'a mut W {
        self.variant(MAGICENW::MAGICEN_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SLEEP`"]
pub enum SLEEPW {
    #[doc = "Normal operating mode."]
    SLEEP_0,
    #[doc = "Sleep mode."]
    SLEEP_1,
}
impl SLEEPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLEEPW::SLEEP_0 => false,
            SLEEPW::SLEEP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLEEPW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEEPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLEEPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operating mode."]
    #[inline]
    pub fn sleep_0(self) -> &'a mut W {
        self.variant(SLEEPW::SLEEP_0)
    }
    #[doc = "Sleep mode."]
    #[inline]
    pub fn sleep_1(self) -> &'a mut W {
        self.variant(SLEEPW::SLEEP_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EN1588`"]
pub enum EN1588W {
    #[doc = "Legacy FEC buffer descriptors and functions enabled."]
    EN1588_0,
    #[doc = "Enhanced frame time-stamping functions enabled."]
    EN1588_1,
}
impl EN1588W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN1588W::EN1588_0 => false,
            EN1588W::EN1588_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN1588W<'a> {
    w: &'a mut W,
}
impl<'a> _EN1588W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN1588W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Legacy FEC buffer descriptors and functions enabled."]
    #[inline]
    pub fn en1588_0(self) -> &'a mut W {
        self.variant(EN1588W::EN1588_0)
    }
    #[doc = "Enhanced frame time-stamping functions enabled."]
    #[inline]
    pub fn en1588_1(self) -> &'a mut W {
        self.variant(EN1588W::EN1588_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPEED`"]
pub enum SPEEDW {
    #[doc = "10/100-Mbit/s mode"]
    SPEED_0,
    #[doc = "1000-Mbit/s mode"]
    SPEED_1,
}
impl SPEEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPEEDW::SPEED_0 => false,
            SPEEDW::SPEED_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPEEDW<'a> {
    w: &'a mut W,
}
impl<'a> _SPEEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPEEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "10/100-Mbit/s mode"]
    #[inline]
    pub fn speed_0(self) -> &'a mut W {
        self.variant(SPEEDW::SPEED_0)
    }
    #[doc = "1000-Mbit/s mode"]
    #[inline]
    pub fn speed_1(self) -> &'a mut W {
        self.variant(SPEEDW::SPEED_1)
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
#[doc = "Values that can be written to the field `DBGEN`"]
pub enum DBGENW {
    #[doc = "MAC continues operation in debug mode."]
    DBGEN_0,
    #[doc = "MAC enters hardware freeze mode when the processor is in debug mode."]
    DBGEN_1,
}
impl DBGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBGENW::DBGEN_0 => false,
            DBGENW::DBGEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBGENW<'a> {
    w: &'a mut W,
}
impl<'a> _DBGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MAC continues operation in debug mode."]
    #[inline]
    pub fn dbgen_0(self) -> &'a mut W {
        self.variant(DBGENW::DBGEN_0)
    }
    #[doc = "MAC enters hardware freeze mode when the processor is in debug mode."]
    #[inline]
    pub fn dbgen_1(self) -> &'a mut W {
        self.variant(DBGENW::DBGEN_1)
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
#[doc = "Values that can be written to the field `DBSWP`"]
pub enum DBSWPW {
    #[doc = "The buffer descriptor bytes are not swapped to support big-endian devices."]
    DBSWP_0,
    #[doc = "The buffer descriptor bytes are swapped to support little-endian devices."]
    DBSWP_1,
}
impl DBSWPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBSWPW::DBSWP_0 => false,
            DBSWPW::DBSWP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBSWPW<'a> {
    w: &'a mut W,
}
impl<'a> _DBSWPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBSWPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The buffer descriptor bytes are not swapped to support big-endian devices."]
    #[inline]
    pub fn dbswp_0(self) -> &'a mut W {
        self.variant(DBSWPW::DBSWP_0)
    }
    #[doc = "The buffer descriptor bytes are swapped to support little-endian devices."]
    #[inline]
    pub fn dbswp_1(self) -> &'a mut W {
        self.variant(DBSWPW::DBSWP_1)
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
#[doc = "Values that can be written to the field `SVLANEN`"]
pub enum SVLANENW {
    #[doc = "Only the EtherType 0x8100 will be considered for VLAN detection."]
    SVLANEN_0,
    #[doc = "The EtherType 0x88a8 will be considered in addition to 0x8100 (C-VLAN) to identify a VLAN frame in receive. When a VLAN frame is identified, the two bytes following the VLAN type are extracted and used by the classification match comparators, RCMRn."]
    SVLANEN_1,
}
impl SVLANENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SVLANENW::SVLANEN_0 => false,
            SVLANENW::SVLANEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SVLANENW<'a> {
    w: &'a mut W,
}
impl<'a> _SVLANENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SVLANENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Only the EtherType 0x8100 will be considered for VLAN detection."]
    #[inline]
    pub fn svlanen_0(self) -> &'a mut W {
        self.variant(SVLANENW::SVLANEN_0)
    }
    #[doc = "The EtherType 0x88a8 will be considered in addition to 0x8100 (C-VLAN) to identify a VLAN frame in receive. When a VLAN frame is identified, the two bytes following the VLAN type are extracted and used by the classification match comparators, RCMRn."]
    #[inline]
    pub fn svlanen_1(self) -> &'a mut W {
        self.variant(SVLANENW::SVLANEN_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VLANUSE2ND`"]
pub enum VLANUSE2NDW {
    #[doc = "Always extract data from the first VLAN tag if it exists."]
    VLANUSE2ND_0,
    #[doc = "When a double-tagged frame is detected, the data of the second tag is extracted for further processing. A double-tagged frame is defined as: The first tag can be a C-VLAN or a S-VLAN (if SVLAN_ENA = 1) The second tag must be a C-VLAN"]
    VLANUSE2ND_1,
}
impl VLANUSE2NDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VLANUSE2NDW::VLANUSE2ND_0 => false,
            VLANUSE2NDW::VLANUSE2ND_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VLANUSE2NDW<'a> {
    w: &'a mut W,
}
impl<'a> _VLANUSE2NDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VLANUSE2NDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Always extract data from the first VLAN tag if it exists."]
    #[inline]
    pub fn vlanuse2nd_0(self) -> &'a mut W {
        self.variant(VLANUSE2NDW::VLANUSE2ND_0)
    }
    #[doc = "When a double-tagged frame is detected, the data of the second tag is extracted for further processing. A double-tagged frame is defined as: The first tag can be a C-VLAN or a S-VLAN (if SVLAN_ENA = 1) The second tag must be a C-VLAN"]
    #[inline]
    pub fn vlanuse2nd_1(self) -> &'a mut W {
        self.variant(VLANUSE2NDW::VLANUSE2ND_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SVLANDBLW<'a> {
    w: &'a mut W,
}
impl<'a> _SVLANDBLW<'a> {
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
        const OFFSET: u8 = 11;
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
    #[doc = "Bit 0 - Ethernet MAC Reset"]
    #[inline]
    pub fn reset(&self) -> RESETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESETR { bits }
    }
    #[doc = "Bit 1 - Ethernet Enable"]
    #[inline]
    pub fn etheren(&self) -> ETHERENR {
        ETHERENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Magic Packet Detection Enable"]
    #[inline]
    pub fn magicen(&self) -> MAGICENR {
        MAGICENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Sleep Mode Enable"]
    #[inline]
    pub fn sleep(&self) -> SLEEPR {
        SLEEPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - EN1588 Enable"]
    #[inline]
    pub fn en1588(&self) -> EN1588R {
        EN1588R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Selects between 10/100-Mbit/s and 1000-Mbit/s modes of operation."]
    #[inline]
    pub fn speed(&self) -> SPEEDR {
        SPEEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Debug Enable"]
    #[inline]
    pub fn dbgen(&self) -> DBGENR {
        DBGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Descriptor Byte Swapping Enable"]
    #[inline]
    pub fn dbswp(&self) -> DBSWPR {
        DBSWPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - S-VLAN enable"]
    #[inline]
    pub fn svlanen(&self) -> SVLANENR {
        SVLANENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - VLAN use second tag"]
    #[inline]
    pub fn vlanuse2nd(&self) -> VLANUSE2NDR {
        VLANUSE2NDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - S-VLAN double tag"]
    #[inline]
    pub fn svlandbl(&self) -> SVLANDBLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SVLANDBLR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1879048192 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Ethernet MAC Reset"]
    #[inline]
    pub fn reset(&mut self) -> _RESETW {
        _RESETW { w: self }
    }
    #[doc = "Bit 1 - Ethernet Enable"]
    #[inline]
    pub fn etheren(&mut self) -> _ETHERENW {
        _ETHERENW { w: self }
    }
    #[doc = "Bit 2 - Magic Packet Detection Enable"]
    #[inline]
    pub fn magicen(&mut self) -> _MAGICENW {
        _MAGICENW { w: self }
    }
    #[doc = "Bit 3 - Sleep Mode Enable"]
    #[inline]
    pub fn sleep(&mut self) -> _SLEEPW {
        _SLEEPW { w: self }
    }
    #[doc = "Bit 4 - EN1588 Enable"]
    #[inline]
    pub fn en1588(&mut self) -> _EN1588W {
        _EN1588W { w: self }
    }
    #[doc = "Bit 5 - Selects between 10/100-Mbit/s and 1000-Mbit/s modes of operation."]
    #[inline]
    pub fn speed(&mut self) -> _SPEEDW {
        _SPEEDW { w: self }
    }
    #[doc = "Bit 6 - Debug Enable"]
    #[inline]
    pub fn dbgen(&mut self) -> _DBGENW {
        _DBGENW { w: self }
    }
    #[doc = "Bit 8 - Descriptor Byte Swapping Enable"]
    #[inline]
    pub fn dbswp(&mut self) -> _DBSWPW {
        _DBSWPW { w: self }
    }
    #[doc = "Bit 9 - S-VLAN enable"]
    #[inline]
    pub fn svlanen(&mut self) -> _SVLANENW {
        _SVLANENW { w: self }
    }
    #[doc = "Bit 10 - VLAN use second tag"]
    #[inline]
    pub fn vlanuse2nd(&mut self) -> _VLANUSE2NDW {
        _VLANUSE2NDW { w: self }
    }
    #[doc = "Bit 11 - S-VLAN double tag"]
    #[inline]
    pub fn svlandbl(&mut self) -> _SVLANDBLW {
        _SVLANDBLW { w: self }
    }
}
