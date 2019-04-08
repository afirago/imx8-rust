#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BCR {
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
#[doc = "Possible values of the field `BAFn`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BAFNR {
    #[doc = "Clears the Fn bit in the ASR register."]
    BAFN_0,
    #[doc = "Sets the Fn bit in the ASR register."]
    BAFN_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BAFNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BAFNR::BAFN_0 => 0,
            BAFNR::BAFN_1 => 1,
            BAFNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BAFNR {
        match value {
            0 => BAFNR::BAFN_0,
            1 => BAFNR::BAFN_1,
            i => BAFNR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BAFN_0`"]
    #[inline]
    pub fn is_bafn_0(&self) -> bool {
        *self == BAFNR::BAFN_0
    }
    #[doc = "Checks if the value of the field is `BAFN_1`"]
    #[inline]
    pub fn is_bafn_1(&self) -> bool {
        *self == BAFNR::BAFN_1
    }
}
#[doc = "Possible values of the field `HRM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRMR {
    #[doc = "BHR bit in ACR is not masked, enables the hardware reset to the Processor B (default after hardware reset)."]
    HRM_0,
    #[doc = "BHR bit in ACR is masked, disables the hardware reset request to the Processor B."]
    HRM_1,
}
impl HRMR {
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
            HRMR::HRM_0 => false,
            HRMR::HRM_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRMR {
        match value {
            false => HRMR::HRM_0,
            true => HRMR::HRM_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRM_0`"]
    #[inline]
    pub fn is_hrm_0(&self) -> bool {
        *self == HRMR::HRM_0
    }
    #[doc = "Checks if the value of the field is `HRM_1`"]
    #[inline]
    pub fn is_hrm_1(&self) -> bool {
        *self == HRMR::HRM_1
    }
}
#[doc = "Possible values of the field `GIRn`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIRNR {
    #[doc = "Processor B General Interrupt n is not requested to the Processor A (default)."]
    GIRN_0,
    #[doc = "Processor B General Interrupt n is requested to the Processor A."]
    GIRN_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GIRNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GIRNR::GIRN_0 => 0,
            GIRNR::GIRN_1 => 1,
            GIRNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GIRNR {
        match value {
            0 => GIRNR::GIRN_0,
            1 => GIRNR::GIRN_1,
            i => GIRNR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GIRN_0`"]
    #[inline]
    pub fn is_girn_0(&self) -> bool {
        *self == GIRNR::GIRN_0
    }
    #[doc = "Checks if the value of the field is `GIRN_1`"]
    #[inline]
    pub fn is_girn_1(&self) -> bool {
        *self == GIRNR::GIRN_1
    }
}
#[doc = "Possible values of the field `TIEn`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIENR {
    #[doc = "Disables Processor B Transmit Interrupt n. (default)"]
    TIEN_0,
    #[doc = "Enables Processor B Transmit Interrupt n."]
    TIEN_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TIENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIENR::TIEN_0 => 0,
            TIENR::TIEN_1 => 1,
            TIENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIENR {
        match value {
            0 => TIENR::TIEN_0,
            1 => TIENR::TIEN_1,
            i => TIENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIEN_0`"]
    #[inline]
    pub fn is_tien_0(&self) -> bool {
        *self == TIENR::TIEN_0
    }
    #[doc = "Checks if the value of the field is `TIEN_1`"]
    #[inline]
    pub fn is_tien_1(&self) -> bool {
        *self == TIENR::TIEN_1
    }
}
#[doc = "Possible values of the field `RIEn`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIENR {
    #[doc = "Disables Processor B Receive Interrupt n. (default)"]
    RIEN_0,
    #[doc = "Enables Processor B Receive Interrupt n."]
    RIEN_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RIENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RIENR::RIEN_0 => 0,
            RIENR::RIEN_1 => 1,
            RIENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RIENR {
        match value {
            0 => RIENR::RIEN_0,
            1 => RIENR::RIEN_1,
            i => RIENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RIEN_0`"]
    #[inline]
    pub fn is_rien_0(&self) -> bool {
        *self == RIENR::RIEN_0
    }
    #[doc = "Checks if the value of the field is `RIEN_1`"]
    #[inline]
    pub fn is_rien_1(&self) -> bool {
        *self == RIENR::RIEN_1
    }
}
#[doc = "Possible values of the field `GIEn`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIENR {
    #[doc = "Disables Processor B General Interrupt n. (default)"]
    GIEN_0,
    #[doc = "Enables Processor B General Interrupt n."]
    GIEN_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GIENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GIENR::GIEN_0 => 0,
            GIENR::GIEN_1 => 1,
            GIENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GIENR {
        match value {
            0 => GIENR::GIEN_0,
            1 => GIENR::GIEN_1,
            i => GIENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GIEN_0`"]
    #[inline]
    pub fn is_gien_0(&self) -> bool {
        *self == GIENR::GIEN_0
    }
    #[doc = "Checks if the value of the field is `GIEN_1`"]
    #[inline]
    pub fn is_gien_1(&self) -> bool {
        *self == GIENR::GIEN_1
    }
}
#[doc = "Values that can be written to the field `BAFn`"]
pub enum BAFNW {
    #[doc = "Clears the Fn bit in the ASR register."]
    BAFN_0,
    #[doc = "Sets the Fn bit in the ASR register."]
    BAFN_1,
}
impl BAFNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BAFNW::BAFN_0 => 0,
            BAFNW::BAFN_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BAFNW<'a> {
    w: &'a mut W,
}
impl<'a> _BAFNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BAFNW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Clears the Fn bit in the ASR register."]
    #[inline]
    pub fn bafn_0(self) -> &'a mut W {
        self.variant(BAFNW::BAFN_0)
    }
    #[doc = "Sets the Fn bit in the ASR register."]
    #[inline]
    pub fn bafn_1(self) -> &'a mut W {
        self.variant(BAFNW::BAFN_1)
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
#[doc = "Values that can be written to the field `HRM`"]
pub enum HRMW {
    #[doc = "BHR bit in ACR is not masked, enables the hardware reset to the Processor B (default after hardware reset)."]
    HRM_0,
    #[doc = "BHR bit in ACR is masked, disables the hardware reset request to the Processor B."]
    HRM_1,
}
impl HRMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRMW::HRM_0 => false,
            HRMW::HRM_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRMW<'a> {
    w: &'a mut W,
}
impl<'a> _HRMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "BHR bit in ACR is not masked, enables the hardware reset to the Processor B (default after hardware reset)."]
    #[inline]
    pub fn hrm_0(self) -> &'a mut W {
        self.variant(HRMW::HRM_0)
    }
    #[doc = "BHR bit in ACR is masked, disables the hardware reset request to the Processor B."]
    #[inline]
    pub fn hrm_1(self) -> &'a mut W {
        self.variant(HRMW::HRM_1)
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
#[doc = "Values that can be written to the field `GIRn`"]
pub enum GIRNW {
    #[doc = "Processor B General Interrupt n is not requested to the Processor A (default)."]
    GIRN_0,
    #[doc = "Processor B General Interrupt n is requested to the Processor A."]
    GIRN_1,
}
impl GIRNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GIRNW::GIRN_0 => 0,
            GIRNW::GIRN_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GIRNW<'a> {
    w: &'a mut W,
}
impl<'a> _GIRNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GIRNW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Processor B General Interrupt n is not requested to the Processor A (default)."]
    #[inline]
    pub fn girn_0(self) -> &'a mut W {
        self.variant(GIRNW::GIRN_0)
    }
    #[doc = "Processor B General Interrupt n is requested to the Processor A."]
    #[inline]
    pub fn girn_1(self) -> &'a mut W {
        self.variant(GIRNW::GIRN_1)
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
#[doc = "Values that can be written to the field `TIEn`"]
pub enum TIENW {
    #[doc = "Disables Processor B Transmit Interrupt n. (default)"]
    TIEN_0,
    #[doc = "Enables Processor B Transmit Interrupt n."]
    TIEN_1,
}
impl TIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIENW::TIEN_0 => 0,
            TIENW::TIEN_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disables Processor B Transmit Interrupt n. (default)"]
    #[inline]
    pub fn tien_0(self) -> &'a mut W {
        self.variant(TIENW::TIEN_0)
    }
    #[doc = "Enables Processor B Transmit Interrupt n."]
    #[inline]
    pub fn tien_1(self) -> &'a mut W {
        self.variant(TIENW::TIEN_1)
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
#[doc = "Values that can be written to the field `RIEn`"]
pub enum RIENW {
    #[doc = "Disables Processor B Receive Interrupt n. (default)"]
    RIEN_0,
    #[doc = "Enables Processor B Receive Interrupt n."]
    RIEN_1,
}
impl RIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RIENW::RIEN_0 => 0,
            RIENW::RIEN_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RIENW<'a> {
    w: &'a mut W,
}
impl<'a> _RIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RIENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disables Processor B Receive Interrupt n. (default)"]
    #[inline]
    pub fn rien_0(self) -> &'a mut W {
        self.variant(RIENW::RIEN_0)
    }
    #[doc = "Enables Processor B Receive Interrupt n."]
    #[inline]
    pub fn rien_1(self) -> &'a mut W {
        self.variant(RIENW::RIEN_1)
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
#[doc = "Values that can be written to the field `GIEn`"]
pub enum GIENW {
    #[doc = "Disables Processor B General Interrupt n. (default)"]
    GIEN_0,
    #[doc = "Enables Processor B General Interrupt n."]
    GIEN_1,
}
impl GIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GIENW::GIEN_0 => 0,
            GIENW::GIEN_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GIENW<'a> {
    w: &'a mut W,
}
impl<'a> _GIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GIENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disables Processor B General Interrupt n. (default)"]
    #[inline]
    pub fn gien_0(self) -> &'a mut W {
        self.variant(GIENW::GIEN_0)
    }
    #[doc = "Enables Processor B General Interrupt n."]
    #[inline]
    pub fn gien_1(self) -> &'a mut W {
        self.variant(GIENW::GIEN_1)
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
    #[doc = "Bits 0:2 - For n = {0, 1, 2} Processor B to Processor A Flag n"]
    #[inline]
    pub fn bafn(&self) -> BAFNR {
        BAFNR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Processor B Hardware Reset Mask"]
    #[inline]
    pub fn hrm(&self) -> HRMR {
        HRMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:19 - For n = {0, 1, 2, 3} Processor B General Purpose Interrupt Request n"]
    #[inline]
    pub fn girn(&self) -> GIRNR {
        GIRNR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - For n = {0, 1, 2, 3} Processor B Transmit Interrupt Enable n"]
    #[inline]
    pub fn tien(&self) -> TIENR {
        TIENR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - For n = {0, 1, 2, 3} Processor B Receive Interrupt Enable n"]
    #[inline]
    pub fn rien(&self) -> RIENR {
        RIENR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - For n = {0, 1, 2, 3} Processor B General Purpose Interrupt Enable n"]
    #[inline]
    pub fn gien(&self) -> GIENR {
        GIENR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - For n = {0, 1, 2} Processor B to Processor A Flag n"]
    #[inline]
    pub fn bafn(&mut self) -> _BAFNW {
        _BAFNW { w: self }
    }
    #[doc = "Bit 4 - Processor B Hardware Reset Mask"]
    #[inline]
    pub fn hrm(&mut self) -> _HRMW {
        _HRMW { w: self }
    }
    #[doc = "Bits 16:19 - For n = {0, 1, 2, 3} Processor B General Purpose Interrupt Request n"]
    #[inline]
    pub fn girn(&mut self) -> _GIRNW {
        _GIRNW { w: self }
    }
    #[doc = "Bits 20:23 - For n = {0, 1, 2, 3} Processor B Transmit Interrupt Enable n"]
    #[inline]
    pub fn tien(&mut self) -> _TIENW {
        _TIENW { w: self }
    }
    #[doc = "Bits 24:27 - For n = {0, 1, 2, 3} Processor B Receive Interrupt Enable n"]
    #[inline]
    pub fn rien(&mut self) -> _RIENW {
        _RIENW { w: self }
    }
    #[doc = "Bits 28:31 - For n = {0, 1, 2, 3} Processor B General Purpose Interrupt Enable n"]
    #[inline]
    pub fn gien(&mut self) -> _GIENW {
        _GIENW { w: self }
    }
}
