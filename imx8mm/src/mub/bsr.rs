#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BSR {
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
#[doc = "Possible values of the field `Fn`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FNR {
    #[doc = "ABFn bit in ACR register is written 0 (default)."]
    FN_0,
    #[doc = "ABFn bit in ACR register is written 1."]
    FN_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FNR::FN_0 => 0,
            FNR::FN_1 => 1,
            FNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FNR {
        match value {
            0 => FNR::FN_0,
            1 => FNR::FN_1,
            i => FNR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FN_0`"]
    #[inline]
    pub fn is_fn_0(&self) -> bool {
        *self == FNR::FN_0
    }
    #[doc = "Checks if the value of the field is `FN_1`"]
    #[inline]
    pub fn is_fn_1(&self) -> bool {
        *self == FNR::FN_1
    }
}
#[doc = "Possible values of the field `EP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPR {
    #[doc = "The Processor B-side event is not pending (default)."]
    EP_0,
    #[doc = "The Processor B-side event is pending."]
    EP_1,
}
impl EPR {
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
            EPR::EP_0 => false,
            EPR::EP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPR {
        match value {
            false => EPR::EP_0,
            true => EPR::EP_1,
        }
    }
    #[doc = "Checks if the value of the field is `EP_0`"]
    #[inline]
    pub fn is_ep_0(&self) -> bool {
        *self == EPR::EP_0
    }
    #[doc = "Checks if the value of the field is `EP_1`"]
    #[inline]
    pub fn is_ep_1(&self) -> bool {
        *self == EPR::EP_1
    }
}
#[doc = "Possible values of the field `APM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APMR {
    #[doc = "The System is in Run Mode."]
    APM_0,
    #[doc = "The System is in WAIT Mode."]
    APM_1,
    #[doc = "The System is in STOP Mode."]
    APM_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl APMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            APMR::APM_0 => 0,
            APMR::APM_1 => 1,
            APMR::APM_3 => 3,
            APMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> APMR {
        match value {
            0 => APMR::APM_0,
            1 => APMR::APM_1,
            3 => APMR::APM_3,
            i => APMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `APM_0`"]
    #[inline]
    pub fn is_apm_0(&self) -> bool {
        *self == APMR::APM_0
    }
    #[doc = "Checks if the value of the field is `APM_1`"]
    #[inline]
    pub fn is_apm_1(&self) -> bool {
        *self == APMR::APM_1
    }
    #[doc = "Checks if the value of the field is `APM_3`"]
    #[inline]
    pub fn is_apm_3(&self) -> bool {
        *self == APMR::APM_3
    }
}
#[doc = "Possible values of the field `ARS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARSR {
    #[doc = "The Processor A or the Processor A-side of the MU is not in reset."]
    ARS_0,
    #[doc = "The Processor A or the Processor A-side of the MU is in reset."]
    ARS_1,
}
impl ARSR {
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
            ARSR::ARS_0 => false,
            ARSR::ARS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARSR {
        match value {
            false => ARSR::ARS_0,
            true => ARSR::ARS_1,
        }
    }
    #[doc = "Checks if the value of the field is `ARS_0`"]
    #[inline]
    pub fn is_ars_0(&self) -> bool {
        *self == ARSR::ARS_0
    }
    #[doc = "Checks if the value of the field is `ARS_1`"]
    #[inline]
    pub fn is_ars_1(&self) -> bool {
        *self == ARSR::ARS_1
    }
}
#[doc = "Possible values of the field `FUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUPR {
    #[doc = "No flags updated, initiated by the Processor B, in progress (default)"]
    FUP_0,
    #[doc = "Processor B initiated flags update, processing"]
    FUP_1,
}
impl FUPR {
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
            FUPR::FUP_0 => false,
            FUPR::FUP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FUPR {
        match value {
            false => FUPR::FUP_0,
            true => FUPR::FUP_1,
        }
    }
    #[doc = "Checks if the value of the field is `FUP_0`"]
    #[inline]
    pub fn is_fup_0(&self) -> bool {
        *self == FUPR::FUP_0
    }
    #[doc = "Checks if the value of the field is `FUP_1`"]
    #[inline]
    pub fn is_fup_1(&self) -> bool {
        *self == FUPR::FUP_1
    }
}
#[doc = "Possible values of the field `TEn`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TENR {
    #[doc = "BTRn register is not empty."]
    TEN_0,
    #[doc = "BTRn register is empty (default)."]
    TEN_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TENR::TEN_0 => 0,
            TENR::TEN_1 => 1,
            TENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TENR {
        match value {
            0 => TENR::TEN_0,
            1 => TENR::TEN_1,
            i => TENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TEN_0`"]
    #[inline]
    pub fn is_ten_0(&self) -> bool {
        *self == TENR::TEN_0
    }
    #[doc = "Checks if the value of the field is `TEN_1`"]
    #[inline]
    pub fn is_ten_1(&self) -> bool {
        *self == TENR::TEN_1
    }
}
#[doc = "Possible values of the field `RFn`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFNR {
    #[doc = "BRRn register is not full (default)."]
    RFN_0,
    #[doc = "BRRn register has received data from ATRn register and is ready to be read by the Processor B."]
    RFN_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RFNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RFNR::RFN_0 => 0,
            RFNR::RFN_1 => 1,
            RFNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RFNR {
        match value {
            0 => RFNR::RFN_0,
            1 => RFNR::RFN_1,
            i => RFNR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RFN_0`"]
    #[inline]
    pub fn is_rfn_0(&self) -> bool {
        *self == RFNR::RFN_0
    }
    #[doc = "Checks if the value of the field is `RFN_1`"]
    #[inline]
    pub fn is_rfn_1(&self) -> bool {
        *self == RFNR::RFN_1
    }
}
#[doc = "Possible values of the field `GIPn`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIPNR {
    #[doc = "Processor B general purpose interrupt n is not pending. (default)"]
    GIPN_0,
    #[doc = "Processor B general purpose interrupt n is pending."]
    GIPN_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GIPNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GIPNR::GIPN_0 => 0,
            GIPNR::GIPN_1 => 1,
            GIPNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GIPNR {
        match value {
            0 => GIPNR::GIPN_0,
            1 => GIPNR::GIPN_1,
            i => GIPNR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GIPN_0`"]
    #[inline]
    pub fn is_gipn_0(&self) -> bool {
        *self == GIPNR::GIPN_0
    }
    #[doc = "Checks if the value of the field is `GIPN_1`"]
    #[inline]
    pub fn is_gipn_1(&self) -> bool {
        *self == GIPNR::GIPN_1
    }
}
#[doc = "Values that can be written to the field `Fn`"]
pub enum FNW {
    #[doc = "ABFn bit in ACR register is written 0 (default)."]
    FN_0,
    #[doc = "ABFn bit in ACR register is written 1."]
    FN_1,
}
impl FNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FNW::FN_0 => 0,
            FNW::FN_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FNW<'a> {
    w: &'a mut W,
}
impl<'a> _FNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FNW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ABFn bit in ACR register is written 0 (default)."]
    #[inline]
    pub fn fn_0(self) -> &'a mut W {
        self.variant(FNW::FN_0)
    }
    #[doc = "ABFn bit in ACR register is written 1."]
    #[inline]
    pub fn fn_1(self) -> &'a mut W {
        self.variant(FNW::FN_1)
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
#[doc = "Values that can be written to the field `EP`"]
pub enum EPW {
    #[doc = "The Processor B-side event is not pending (default)."]
    EP_0,
    #[doc = "The Processor B-side event is pending."]
    EP_1,
}
impl EPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPW::EP_0 => false,
            EPW::EP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPW<'a> {
    w: &'a mut W,
}
impl<'a> _EPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The Processor B-side event is not pending (default)."]
    #[inline]
    pub fn ep_0(self) -> &'a mut W {
        self.variant(EPW::EP_0)
    }
    #[doc = "The Processor B-side event is pending."]
    #[inline]
    pub fn ep_1(self) -> &'a mut W {
        self.variant(EPW::EP_1)
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
#[doc = "Values that can be written to the field `APM`"]
pub enum APMW {
    #[doc = "The System is in Run Mode."]
    APM_0,
    #[doc = "The System is in WAIT Mode."]
    APM_1,
    #[doc = "The System is in STOP Mode."]
    APM_3,
}
impl APMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            APMW::APM_0 => 0,
            APMW::APM_1 => 1,
            APMW::APM_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _APMW<'a> {
    w: &'a mut W,
}
impl<'a> _APMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: APMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The System is in Run Mode."]
    #[inline]
    pub fn apm_0(self) -> &'a mut W {
        self.variant(APMW::APM_0)
    }
    #[doc = "The System is in WAIT Mode."]
    #[inline]
    pub fn apm_1(self) -> &'a mut W {
        self.variant(APMW::APM_1)
    }
    #[doc = "The System is in STOP Mode."]
    #[inline]
    pub fn apm_3(self) -> &'a mut W {
        self.variant(APMW::APM_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ARS`"]
pub enum ARSW {
    #[doc = "The Processor A or the Processor A-side of the MU is not in reset."]
    ARS_0,
    #[doc = "The Processor A or the Processor A-side of the MU is in reset."]
    ARS_1,
}
impl ARSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ARSW::ARS_0 => false,
            ARSW::ARS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARSW<'a> {
    w: &'a mut W,
}
impl<'a> _ARSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The Processor A or the Processor A-side of the MU is not in reset."]
    #[inline]
    pub fn ars_0(self) -> &'a mut W {
        self.variant(ARSW::ARS_0)
    }
    #[doc = "The Processor A or the Processor A-side of the MU is in reset."]
    #[inline]
    pub fn ars_1(self) -> &'a mut W {
        self.variant(ARSW::ARS_1)
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
#[doc = "Values that can be written to the field `FUP`"]
pub enum FUPW {
    #[doc = "No flags updated, initiated by the Processor B, in progress (default)"]
    FUP_0,
    #[doc = "Processor B initiated flags update, processing"]
    FUP_1,
}
impl FUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FUPW::FUP_0 => false,
            FUPW::FUP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FUPW<'a> {
    w: &'a mut W,
}
impl<'a> _FUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No flags updated, initiated by the Processor B, in progress (default)"]
    #[inline]
    pub fn fup_0(self) -> &'a mut W {
        self.variant(FUPW::FUP_0)
    }
    #[doc = "Processor B initiated flags update, processing"]
    #[inline]
    pub fn fup_1(self) -> &'a mut W {
        self.variant(FUPW::FUP_1)
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
#[doc = "Values that can be written to the field `TEn`"]
pub enum TENW {
    #[doc = "BTRn register is not empty."]
    TEN_0,
    #[doc = "BTRn register is empty (default)."]
    TEN_1,
}
impl TENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TENW::TEN_0 => 0,
            TENW::TEN_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TENW<'a> {
    w: &'a mut W,
}
impl<'a> _TENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "BTRn register is not empty."]
    #[inline]
    pub fn ten_0(self) -> &'a mut W {
        self.variant(TENW::TEN_0)
    }
    #[doc = "BTRn register is empty (default)."]
    #[inline]
    pub fn ten_1(self) -> &'a mut W {
        self.variant(TENW::TEN_1)
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
#[doc = "Values that can be written to the field `RFn`"]
pub enum RFNW {
    #[doc = "BRRn register is not full (default)."]
    RFN_0,
    #[doc = "BRRn register has received data from ATRn register and is ready to be read by the Processor B."]
    RFN_1,
}
impl RFNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RFNW::RFN_0 => 0,
            RFNW::RFN_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RFNW<'a> {
    w: &'a mut W,
}
impl<'a> _RFNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RFNW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "BRRn register is not full (default)."]
    #[inline]
    pub fn rfn_0(self) -> &'a mut W {
        self.variant(RFNW::RFN_0)
    }
    #[doc = "BRRn register has received data from ATRn register and is ready to be read by the Processor B."]
    #[inline]
    pub fn rfn_1(self) -> &'a mut W {
        self.variant(RFNW::RFN_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GIPn`"]
pub enum GIPNW {
    #[doc = "Processor B general purpose interrupt n is not pending. (default)"]
    GIPN_0,
    #[doc = "Processor B general purpose interrupt n is pending."]
    GIPN_1,
}
impl GIPNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GIPNW::GIPN_0 => 0,
            GIPNW::GIPN_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GIPNW<'a> {
    w: &'a mut W,
}
impl<'a> _GIPNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GIPNW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Processor B general purpose interrupt n is not pending. (default)"]
    #[inline]
    pub fn gipn_0(self) -> &'a mut W {
        self.variant(GIPNW::GIPN_0)
    }
    #[doc = "Processor B general purpose interrupt n is pending."]
    #[inline]
    pub fn gipn_1(self) -> &'a mut W {
        self.variant(GIPNW::GIPN_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:2 - For n = {0, 1, 2} Processor B-Side Flag n"]
    #[inline]
    pub fn fn_(&self) -> FNR {
        FNR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Processor B-Side Event Pending"]
    #[inline]
    pub fn ep(&self) -> EPR {
        EPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:6 - Processor A Power Mode. (Read-only) APM\\[1:0\\] bits indicate the Processor A power mode."]
    #[inline]
    pub fn apm(&self) -> APMR {
        APMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Processor A Reset State"]
    #[inline]
    pub fn ars(&self) -> ARSR {
        ARSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Processor B Flags Update Pending"]
    #[inline]
    pub fn fup(&self) -> FUPR {
        FUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 20:23 - For n = {0, 1, 2, 3} Processor B Transmit Register n Empty"]
    #[inline]
    pub fn ten(&self) -> TENR {
        TENR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - For n = {0, 1, 2, 3} Processor B Receive Register n Full"]
    #[inline]
    pub fn rfn(&self) -> RFNR {
        RFNR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - For n = {0, 1, 2, 3} Processor B General Interrupt Request n Pending"]
    #[inline]
    pub fn gipn(&self) -> GIPNR {
        GIPNR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 15728768 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - For n = {0, 1, 2} Processor B-Side Flag n"]
    #[inline]
    pub fn fn_(&mut self) -> _FNW {
        _FNW { w: self }
    }
    #[doc = "Bit 4 - Processor B-Side Event Pending"]
    #[inline]
    pub fn ep(&mut self) -> _EPW {
        _EPW { w: self }
    }
    #[doc = "Bits 5:6 - Processor A Power Mode. (Read-only) APM\\[1:0\\] bits indicate the Processor A power mode."]
    #[inline]
    pub fn apm(&mut self) -> _APMW {
        _APMW { w: self }
    }
    #[doc = "Bit 7 - Processor A Reset State"]
    #[inline]
    pub fn ars(&mut self) -> _ARSW {
        _ARSW { w: self }
    }
    #[doc = "Bit 8 - Processor B Flags Update Pending"]
    #[inline]
    pub fn fup(&mut self) -> _FUPW {
        _FUPW { w: self }
    }
    #[doc = "Bits 20:23 - For n = {0, 1, 2, 3} Processor B Transmit Register n Empty"]
    #[inline]
    pub fn ten(&mut self) -> _TENW {
        _TENW { w: self }
    }
    #[doc = "Bits 24:27 - For n = {0, 1, 2, 3} Processor B Receive Register n Full"]
    #[inline]
    pub fn rfn(&mut self) -> _RFNW {
        _RFNW { w: self }
    }
    #[doc = "Bits 28:31 - For n = {0, 1, 2, 3} Processor B General Interrupt Request n Pending"]
    #[inline]
    pub fn gipn(&mut self) -> _GIPNW {
        _GIPNW { w: self }
    }
}
