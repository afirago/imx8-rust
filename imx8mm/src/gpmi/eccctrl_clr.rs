#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ECCCTRL_CLR {
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
pub struct BUFFER_MASKR {
    bits: u16,
}
impl BUFFER_MASKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `RANDOMIZER_TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RANDOMIZER_TYPER {
    #[doc = "Type 0"]
    RANDOMIZER_TYPE_0,
    #[doc = "Type 1"]
    RANDOMIZER_TYPE_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RANDOMIZER_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RANDOMIZER_TYPER::RANDOMIZER_TYPE_0 => 0,
            RANDOMIZER_TYPER::RANDOMIZER_TYPE_1 => 1,
            RANDOMIZER_TYPER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RANDOMIZER_TYPER {
        match value {
            0 => RANDOMIZER_TYPER::RANDOMIZER_TYPE_0,
            1 => RANDOMIZER_TYPER::RANDOMIZER_TYPE_1,
            i => RANDOMIZER_TYPER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RANDOMIZER_TYPE_0`"]
    #[inline]
    pub fn is_randomizer_type_0(&self) -> bool {
        *self == RANDOMIZER_TYPER::RANDOMIZER_TYPE_0
    }
    #[doc = "Checks if the value of the field is `RANDOMIZER_TYPE_1`"]
    #[inline]
    pub fn is_randomizer_type_1(&self) -> bool {
        *self == RANDOMIZER_TYPER::RANDOMIZER_TYPE_1
    }
}
#[doc = "Possible values of the field `RANDOMIZER_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RANDOMIZER_ENABLER {
    #[doc = "disable"]
    RANDOMIZER_ENABLE_0,
    #[doc = "enable"]
    RANDOMIZER_ENABLE_1,
}
impl RANDOMIZER_ENABLER {
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
            RANDOMIZER_ENABLER::RANDOMIZER_ENABLE_0 => false,
            RANDOMIZER_ENABLER::RANDOMIZER_ENABLE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RANDOMIZER_ENABLER {
        match value {
            false => RANDOMIZER_ENABLER::RANDOMIZER_ENABLE_0,
            true => RANDOMIZER_ENABLER::RANDOMIZER_ENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RANDOMIZER_ENABLE_0`"]
    #[inline]
    pub fn is_randomizer_enable_0(&self) -> bool {
        *self == RANDOMIZER_ENABLER::RANDOMIZER_ENABLE_0
    }
    #[doc = "Checks if the value of the field is `RANDOMIZER_ENABLE_1`"]
    #[inline]
    pub fn is_randomizer_enable_1(&self) -> bool {
        *self == RANDOMIZER_ENABLER::RANDOMIZER_ENABLE_1
    }
}
#[doc = r" Value of the field"]
pub struct ENABLE_ECCR {
    bits: bool,
}
impl ENABLE_ECCR {
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
#[doc = r" Value of the field"]
pub struct ECC_CMDR {
    bits: u8,
}
impl ECC_CMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RSVD2R {
    bits: bool,
}
impl RSVD2R {
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
#[doc = r" Value of the field"]
pub struct HANDLER {
    bits: u16,
}
impl HANDLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _BUFFER_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _BUFFER_MASKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RANDOMIZER_TYPE`"]
pub enum RANDOMIZER_TYPEW {
    #[doc = "Type 0"]
    RANDOMIZER_TYPE_0,
    #[doc = "Type 1"]
    RANDOMIZER_TYPE_1,
}
impl RANDOMIZER_TYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RANDOMIZER_TYPEW::RANDOMIZER_TYPE_0 => 0,
            RANDOMIZER_TYPEW::RANDOMIZER_TYPE_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RANDOMIZER_TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _RANDOMIZER_TYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RANDOMIZER_TYPEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Type 0"]
    #[inline]
    pub fn randomizer_type_0(self) -> &'a mut W {
        self.variant(RANDOMIZER_TYPEW::RANDOMIZER_TYPE_0)
    }
    #[doc = "Type 1"]
    #[inline]
    pub fn randomizer_type_1(self) -> &'a mut W {
        self.variant(RANDOMIZER_TYPEW::RANDOMIZER_TYPE_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RANDOMIZER_ENABLE`"]
pub enum RANDOMIZER_ENABLEW {
    #[doc = "disable"]
    RANDOMIZER_ENABLE_0,
    #[doc = "enable"]
    RANDOMIZER_ENABLE_1,
}
impl RANDOMIZER_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RANDOMIZER_ENABLEW::RANDOMIZER_ENABLE_0 => false,
            RANDOMIZER_ENABLEW::RANDOMIZER_ENABLE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RANDOMIZER_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _RANDOMIZER_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RANDOMIZER_ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn randomizer_enable_0(self) -> &'a mut W {
        self.variant(RANDOMIZER_ENABLEW::RANDOMIZER_ENABLE_0)
    }
    #[doc = "enable"]
    #[inline]
    pub fn randomizer_enable_1(self) -> &'a mut W {
        self.variant(RANDOMIZER_ENABLEW::RANDOMIZER_ENABLE_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_ECCW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_ECCW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ECC_CMDW<'a> {
    w: &'a mut W,
}
impl<'a> _ECC_CMDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RSVD2W<'a> {
    w: &'a mut W,
}
impl<'a> _RSVD2W<'a> {
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
pub struct _HANDLEW<'a> {
    w: &'a mut W,
}
impl<'a> _HANDLEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
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
    #[doc = "Bits 0:8 - ECC buffer information"]
    #[inline]
    pub fn buffer_mask(&self) -> BUFFER_MASKR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        BUFFER_MASKR { bits }
    }
    #[doc = "Bits 9:10 - Set randomizer type"]
    #[inline]
    pub fn randomizer_type(&self) -> RANDOMIZER_TYPER {
        RANDOMIZER_TYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Enable randomizer function. If this bit is set to enable, ENABLE_ECC should be also enable."]
    #[inline]
    pub fn randomizer_enable(&self) -> RANDOMIZER_ENABLER {
        RANDOMIZER_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Enable ECC processing of GPMI transfers"]
    #[inline]
    pub fn enable_ecc(&self) -> ENABLE_ECCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_ECCR { bits }
    }
    #[doc = "Bits 13:14 - ECC Command information"]
    #[inline]
    pub fn ecc_cmd(&self) -> ECC_CMDR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ECC_CMDR { bits }
    }
    #[doc = "Bit 15 - Always write zeroes to this bit field."]
    #[inline]
    pub fn rsvd2(&self) -> RSVD2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSVD2R { bits }
    }
    #[doc = "Bits 16:31 - This is a register available to software to attach an identifier to a transaction in progress"]
    #[inline]
    pub fn handle(&self) -> HANDLER {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        HANDLER { bits }
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
    #[doc = "Bits 0:8 - ECC buffer information"]
    #[inline]
    pub fn buffer_mask(&mut self) -> _BUFFER_MASKW {
        _BUFFER_MASKW { w: self }
    }
    #[doc = "Bits 9:10 - Set randomizer type"]
    #[inline]
    pub fn randomizer_type(&mut self) -> _RANDOMIZER_TYPEW {
        _RANDOMIZER_TYPEW { w: self }
    }
    #[doc = "Bit 11 - Enable randomizer function. If this bit is set to enable, ENABLE_ECC should be also enable."]
    #[inline]
    pub fn randomizer_enable(&mut self) -> _RANDOMIZER_ENABLEW {
        _RANDOMIZER_ENABLEW { w: self }
    }
    #[doc = "Bit 12 - Enable ECC processing of GPMI transfers"]
    #[inline]
    pub fn enable_ecc(&mut self) -> _ENABLE_ECCW {
        _ENABLE_ECCW { w: self }
    }
    #[doc = "Bits 13:14 - ECC Command information"]
    #[inline]
    pub fn ecc_cmd(&mut self) -> _ECC_CMDW {
        _ECC_CMDW { w: self }
    }
    #[doc = "Bit 15 - Always write zeroes to this bit field."]
    #[inline]
    pub fn rsvd2(&mut self) -> _RSVD2W {
        _RSVD2W { w: self }
    }
    #[doc = "Bits 16:31 - This is a register available to software to attach an identifier to a transaction in progress"]
    #[inline]
    pub fn handle(&mut self) -> _HANDLEW {
        _HANDLEW { w: self }
    }
}
