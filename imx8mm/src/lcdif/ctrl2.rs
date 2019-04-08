#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL2 {
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
pub struct INITIAL_DUMMY_READR {
    bits: u8,
}
impl INITIAL_DUMMY_READR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct READ_MODE_NUM_PACKED_SUBWORDSR {
    bits: u8,
}
impl READ_MODE_NUM_PACKED_SUBWORDSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct READ_MODE_6_BIT_INPUTR {
    bits: bool,
}
impl READ_MODE_6_BIT_INPUTR {
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
pub struct READ_MODE_OUTPUT_IN_RGB_FORMATR {
    bits: bool,
}
impl READ_MODE_OUTPUT_IN_RGB_FORMATR {
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
pub struct READ_PACK_DIRR {
    bits: bool,
}
impl READ_PACK_DIRR {
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
#[doc = "Possible values of the field `EVEN_LINE_PATTERN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVEN_LINE_PATTERNR {
    #[doc = "RGB"]
    RGB,
    #[doc = "RBG"]
    RBG,
    #[doc = "GBR"]
    GBR,
    #[doc = "GRB"]
    GRB,
    #[doc = "BRG"]
    BRG,
    #[doc = "BGR"]
    BGR,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EVEN_LINE_PATTERNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EVEN_LINE_PATTERNR::RGB => 0,
            EVEN_LINE_PATTERNR::RBG => 1,
            EVEN_LINE_PATTERNR::GBR => 2,
            EVEN_LINE_PATTERNR::GRB => 3,
            EVEN_LINE_PATTERNR::BRG => 4,
            EVEN_LINE_PATTERNR::BGR => 5,
            EVEN_LINE_PATTERNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EVEN_LINE_PATTERNR {
        match value {
            0 => EVEN_LINE_PATTERNR::RGB,
            1 => EVEN_LINE_PATTERNR::RBG,
            2 => EVEN_LINE_PATTERNR::GBR,
            3 => EVEN_LINE_PATTERNR::GRB,
            4 => EVEN_LINE_PATTERNR::BRG,
            5 => EVEN_LINE_PATTERNR::BGR,
            i => EVEN_LINE_PATTERNR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RGB`"]
    #[inline]
    pub fn is_rgb(&self) -> bool {
        *self == EVEN_LINE_PATTERNR::RGB
    }
    #[doc = "Checks if the value of the field is `RBG`"]
    #[inline]
    pub fn is_rbg(&self) -> bool {
        *self == EVEN_LINE_PATTERNR::RBG
    }
    #[doc = "Checks if the value of the field is `GBR`"]
    #[inline]
    pub fn is_gbr(&self) -> bool {
        *self == EVEN_LINE_PATTERNR::GBR
    }
    #[doc = "Checks if the value of the field is `GRB`"]
    #[inline]
    pub fn is_grb(&self) -> bool {
        *self == EVEN_LINE_PATTERNR::GRB
    }
    #[doc = "Checks if the value of the field is `BRG`"]
    #[inline]
    pub fn is_brg(&self) -> bool {
        *self == EVEN_LINE_PATTERNR::BRG
    }
    #[doc = "Checks if the value of the field is `BGR`"]
    #[inline]
    pub fn is_bgr(&self) -> bool {
        *self == EVEN_LINE_PATTERNR::BGR
    }
}
#[doc = "Possible values of the field `ODD_LINE_PATTERN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ODD_LINE_PATTERNR {
    #[doc = "RGB"]
    RGB,
    #[doc = "RBG"]
    RBG,
    #[doc = "GBR"]
    GBR,
    #[doc = "GRB"]
    GRB,
    #[doc = "BRG"]
    BRG,
    #[doc = "BGR"]
    BGR,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ODD_LINE_PATTERNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ODD_LINE_PATTERNR::RGB => 0,
            ODD_LINE_PATTERNR::RBG => 1,
            ODD_LINE_PATTERNR::GBR => 2,
            ODD_LINE_PATTERNR::GRB => 3,
            ODD_LINE_PATTERNR::BRG => 4,
            ODD_LINE_PATTERNR::BGR => 5,
            ODD_LINE_PATTERNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ODD_LINE_PATTERNR {
        match value {
            0 => ODD_LINE_PATTERNR::RGB,
            1 => ODD_LINE_PATTERNR::RBG,
            2 => ODD_LINE_PATTERNR::GBR,
            3 => ODD_LINE_PATTERNR::GRB,
            4 => ODD_LINE_PATTERNR::BRG,
            5 => ODD_LINE_PATTERNR::BGR,
            i => ODD_LINE_PATTERNR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RGB`"]
    #[inline]
    pub fn is_rgb(&self) -> bool {
        *self == ODD_LINE_PATTERNR::RGB
    }
    #[doc = "Checks if the value of the field is `RBG`"]
    #[inline]
    pub fn is_rbg(&self) -> bool {
        *self == ODD_LINE_PATTERNR::RBG
    }
    #[doc = "Checks if the value of the field is `GBR`"]
    #[inline]
    pub fn is_gbr(&self) -> bool {
        *self == ODD_LINE_PATTERNR::GBR
    }
    #[doc = "Checks if the value of the field is `GRB`"]
    #[inline]
    pub fn is_grb(&self) -> bool {
        *self == ODD_LINE_PATTERNR::GRB
    }
    #[doc = "Checks if the value of the field is `BRG`"]
    #[inline]
    pub fn is_brg(&self) -> bool {
        *self == ODD_LINE_PATTERNR::BRG
    }
    #[doc = "Checks if the value of the field is `BGR`"]
    #[inline]
    pub fn is_bgr(&self) -> bool {
        *self == ODD_LINE_PATTERNR::BGR
    }
}
#[doc = r" Value of the field"]
pub struct BURST_LEN_8R {
    bits: bool,
}
impl BURST_LEN_8R {
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
#[doc = "Possible values of the field `OUTSTANDING_REQS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSTANDING_REQSR {
    #[doc = "REQ_1"]
    REQ_1,
    #[doc = "REQ_2"]
    REQ_2,
    #[doc = "REQ_4"]
    REQ_4,
    #[doc = "REQ_8"]
    REQ_8,
    #[doc = "REQ_16"]
    REQ_16,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OUTSTANDING_REQSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OUTSTANDING_REQSR::REQ_1 => 0,
            OUTSTANDING_REQSR::REQ_2 => 1,
            OUTSTANDING_REQSR::REQ_4 => 2,
            OUTSTANDING_REQSR::REQ_8 => 3,
            OUTSTANDING_REQSR::REQ_16 => 4,
            OUTSTANDING_REQSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OUTSTANDING_REQSR {
        match value {
            0 => OUTSTANDING_REQSR::REQ_1,
            1 => OUTSTANDING_REQSR::REQ_2,
            2 => OUTSTANDING_REQSR::REQ_4,
            3 => OUTSTANDING_REQSR::REQ_8,
            4 => OUTSTANDING_REQSR::REQ_16,
            i => OUTSTANDING_REQSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `REQ_1`"]
    #[inline]
    pub fn is_req_1(&self) -> bool {
        *self == OUTSTANDING_REQSR::REQ_1
    }
    #[doc = "Checks if the value of the field is `REQ_2`"]
    #[inline]
    pub fn is_req_2(&self) -> bool {
        *self == OUTSTANDING_REQSR::REQ_2
    }
    #[doc = "Checks if the value of the field is `REQ_4`"]
    #[inline]
    pub fn is_req_4(&self) -> bool {
        *self == OUTSTANDING_REQSR::REQ_4
    }
    #[doc = "Checks if the value of the field is `REQ_8`"]
    #[inline]
    pub fn is_req_8(&self) -> bool {
        *self == OUTSTANDING_REQSR::REQ_8
    }
    #[doc = "Checks if the value of the field is `REQ_16`"]
    #[inline]
    pub fn is_req_16(&self) -> bool {
        *self == OUTSTANDING_REQSR::REQ_16
    }
}
#[doc = r" Proxy"]
pub struct _INITIAL_DUMMY_READW<'a> {
    w: &'a mut W,
}
impl<'a> _INITIAL_DUMMY_READW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _READ_MODE_NUM_PACKED_SUBWORDSW<'a> {
    w: &'a mut W,
}
impl<'a> _READ_MODE_NUM_PACKED_SUBWORDSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _READ_MODE_6_BIT_INPUTW<'a> {
    w: &'a mut W,
}
impl<'a> _READ_MODE_6_BIT_INPUTW<'a> {
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
#[doc = r" Proxy"]
pub struct _READ_MODE_OUTPUT_IN_RGB_FORMATW<'a> {
    w: &'a mut W,
}
impl<'a> _READ_MODE_OUTPUT_IN_RGB_FORMATW<'a> {
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
#[doc = r" Proxy"]
pub struct _READ_PACK_DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _READ_PACK_DIRW<'a> {
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
#[doc = "Values that can be written to the field `EVEN_LINE_PATTERN`"]
pub enum EVEN_LINE_PATTERNW {
    #[doc = "RGB"]
    RGB,
    #[doc = "RBG"]
    RBG,
    #[doc = "GBR"]
    GBR,
    #[doc = "GRB"]
    GRB,
    #[doc = "BRG"]
    BRG,
    #[doc = "BGR"]
    BGR,
}
impl EVEN_LINE_PATTERNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EVEN_LINE_PATTERNW::RGB => 0,
            EVEN_LINE_PATTERNW::RBG => 1,
            EVEN_LINE_PATTERNW::GBR => 2,
            EVEN_LINE_PATTERNW::GRB => 3,
            EVEN_LINE_PATTERNW::BRG => 4,
            EVEN_LINE_PATTERNW::BGR => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EVEN_LINE_PATTERNW<'a> {
    w: &'a mut W,
}
impl<'a> _EVEN_LINE_PATTERNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EVEN_LINE_PATTERNW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "RGB"]
    #[inline]
    pub fn rgb(self) -> &'a mut W {
        self.variant(EVEN_LINE_PATTERNW::RGB)
    }
    #[doc = "RBG"]
    #[inline]
    pub fn rbg(self) -> &'a mut W {
        self.variant(EVEN_LINE_PATTERNW::RBG)
    }
    #[doc = "GBR"]
    #[inline]
    pub fn gbr(self) -> &'a mut W {
        self.variant(EVEN_LINE_PATTERNW::GBR)
    }
    #[doc = "GRB"]
    #[inline]
    pub fn grb(self) -> &'a mut W {
        self.variant(EVEN_LINE_PATTERNW::GRB)
    }
    #[doc = "BRG"]
    #[inline]
    pub fn brg(self) -> &'a mut W {
        self.variant(EVEN_LINE_PATTERNW::BRG)
    }
    #[doc = "BGR"]
    #[inline]
    pub fn bgr(self) -> &'a mut W {
        self.variant(EVEN_LINE_PATTERNW::BGR)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ODD_LINE_PATTERN`"]
pub enum ODD_LINE_PATTERNW {
    #[doc = "RGB"]
    RGB,
    #[doc = "RBG"]
    RBG,
    #[doc = "GBR"]
    GBR,
    #[doc = "GRB"]
    GRB,
    #[doc = "BRG"]
    BRG,
    #[doc = "BGR"]
    BGR,
}
impl ODD_LINE_PATTERNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ODD_LINE_PATTERNW::RGB => 0,
            ODD_LINE_PATTERNW::RBG => 1,
            ODD_LINE_PATTERNW::GBR => 2,
            ODD_LINE_PATTERNW::GRB => 3,
            ODD_LINE_PATTERNW::BRG => 4,
            ODD_LINE_PATTERNW::BGR => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ODD_LINE_PATTERNW<'a> {
    w: &'a mut W,
}
impl<'a> _ODD_LINE_PATTERNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ODD_LINE_PATTERNW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "RGB"]
    #[inline]
    pub fn rgb(self) -> &'a mut W {
        self.variant(ODD_LINE_PATTERNW::RGB)
    }
    #[doc = "RBG"]
    #[inline]
    pub fn rbg(self) -> &'a mut W {
        self.variant(ODD_LINE_PATTERNW::RBG)
    }
    #[doc = "GBR"]
    #[inline]
    pub fn gbr(self) -> &'a mut W {
        self.variant(ODD_LINE_PATTERNW::GBR)
    }
    #[doc = "GRB"]
    #[inline]
    pub fn grb(self) -> &'a mut W {
        self.variant(ODD_LINE_PATTERNW::GRB)
    }
    #[doc = "BRG"]
    #[inline]
    pub fn brg(self) -> &'a mut W {
        self.variant(ODD_LINE_PATTERNW::BRG)
    }
    #[doc = "BGR"]
    #[inline]
    pub fn bgr(self) -> &'a mut W {
        self.variant(ODD_LINE_PATTERNW::BGR)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BURST_LEN_8W<'a> {
    w: &'a mut W,
}
impl<'a> _BURST_LEN_8W<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OUTSTANDING_REQS`"]
pub enum OUTSTANDING_REQSW {
    #[doc = "REQ_1"]
    REQ_1,
    #[doc = "REQ_2"]
    REQ_2,
    #[doc = "REQ_4"]
    REQ_4,
    #[doc = "REQ_8"]
    REQ_8,
    #[doc = "REQ_16"]
    REQ_16,
}
impl OUTSTANDING_REQSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OUTSTANDING_REQSW::REQ_1 => 0,
            OUTSTANDING_REQSW::REQ_2 => 1,
            OUTSTANDING_REQSW::REQ_4 => 2,
            OUTSTANDING_REQSW::REQ_8 => 3,
            OUTSTANDING_REQSW::REQ_16 => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUTSTANDING_REQSW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTSTANDING_REQSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTSTANDING_REQSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "REQ_1"]
    #[inline]
    pub fn req_1(self) -> &'a mut W {
        self.variant(OUTSTANDING_REQSW::REQ_1)
    }
    #[doc = "REQ_2"]
    #[inline]
    pub fn req_2(self) -> &'a mut W {
        self.variant(OUTSTANDING_REQSW::REQ_2)
    }
    #[doc = "REQ_4"]
    #[inline]
    pub fn req_4(self) -> &'a mut W {
        self.variant(OUTSTANDING_REQSW::REQ_4)
    }
    #[doc = "REQ_8"]
    #[inline]
    pub fn req_8(self) -> &'a mut W {
        self.variant(OUTSTANDING_REQSW::REQ_8)
    }
    #[doc = "REQ_16"]
    #[inline]
    pub fn req_16(self) -> &'a mut W {
        self.variant(OUTSTANDING_REQSW::REQ_16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 21;
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
    #[doc = "Bits 1:3 - The value in this field determines the number of dummy 8/16/18/24-bit subwords that have to be read back from the LCD panel/controller"]
    #[inline]
    pub fn initial_dummy_read(&self) -> INITIAL_DUMMY_READR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INITIAL_DUMMY_READR { bits }
    }
    #[doc = "Bits 4:6 - Indicates the number of valid 8/16/18/24-bit subwords that will be packed into the 32-bit word in read mode"]
    #[inline]
    pub fn read_mode_num_packed_subwords(&self) -> READ_MODE_NUM_PACKED_SUBWORDSR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        READ_MODE_NUM_PACKED_SUBWORDSR { bits }
    }
    #[doc = "Bit 8 - Setting this bit to 1 indicates to LCDIF that even though LCD_DATABUS_WIDTH is set to 8 bits, the input data is actually only 6 bits wide and exists on D5-D0"]
    #[inline]
    pub fn read_mode_6_bit_input(&self) -> READ_MODE_6_BIT_INPUTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        READ_MODE_6_BIT_INPUTR { bits }
    }
    #[doc = "Bit 9 - Setting this bit will enable the LCDIF to convert the incoming data to the RGB format given by WORD_LENGTH bitfield"]
    #[inline]
    pub fn read_mode_output_in_rgb_format(&self) -> READ_MODE_OUTPUT_IN_RGB_FORMATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        READ_MODE_OUTPUT_IN_RGB_FORMATR { bits }
    }
    #[doc = "Bit 10 - The default value of 0 indicates data is stored in the little endian format"]
    #[inline]
    pub fn read_pack_dir(&self) -> READ_PACK_DIRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        READ_PACK_DIRR { bits }
    }
    #[doc = "Bits 12:14 - This field determines the order of the RGB components of each pixel in EVEN lines (line numbers 2,4,6,"]
    #[inline]
    pub fn even_line_pattern(&self) -> EVEN_LINE_PATTERNR {
        EVEN_LINE_PATTERNR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - This field determines the order of the RGB components of each pixel in ODD lines (line numbers 1,3,5,"]
    #[inline]
    pub fn odd_line_pattern(&self) -> ODD_LINE_PATTERNR {
        ODD_LINE_PATTERNR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - By default, when the LCDIF is in the bus master mode, it will issue AXI bursts of length 16 (except when in packed 24 bpp mode, it will issue bursts of length 15)"]
    #[inline]
    pub fn burst_len_8(&self) -> BURST_LEN_8R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BURST_LEN_8R { bits }
    }
    #[doc = "Bits 21:23 - This bitfield indicates the maximum number of outstanding transactions that LCDIF should request when it is acting as a bus master"]
    #[inline]
    pub fn outstanding_reqs(&self) -> OUTSTANDING_REQSR {
        OUTSTANDING_REQSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2097152 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 1:3 - The value in this field determines the number of dummy 8/16/18/24-bit subwords that have to be read back from the LCD panel/controller"]
    #[inline]
    pub fn initial_dummy_read(&mut self) -> _INITIAL_DUMMY_READW {
        _INITIAL_DUMMY_READW { w: self }
    }
    #[doc = "Bits 4:6 - Indicates the number of valid 8/16/18/24-bit subwords that will be packed into the 32-bit word in read mode"]
    #[inline]
    pub fn read_mode_num_packed_subwords(&mut self) -> _READ_MODE_NUM_PACKED_SUBWORDSW {
        _READ_MODE_NUM_PACKED_SUBWORDSW { w: self }
    }
    #[doc = "Bit 8 - Setting this bit to 1 indicates to LCDIF that even though LCD_DATABUS_WIDTH is set to 8 bits, the input data is actually only 6 bits wide and exists on D5-D0"]
    #[inline]
    pub fn read_mode_6_bit_input(&mut self) -> _READ_MODE_6_BIT_INPUTW {
        _READ_MODE_6_BIT_INPUTW { w: self }
    }
    #[doc = "Bit 9 - Setting this bit will enable the LCDIF to convert the incoming data to the RGB format given by WORD_LENGTH bitfield"]
    #[inline]
    pub fn read_mode_output_in_rgb_format(&mut self) -> _READ_MODE_OUTPUT_IN_RGB_FORMATW {
        _READ_MODE_OUTPUT_IN_RGB_FORMATW { w: self }
    }
    #[doc = "Bit 10 - The default value of 0 indicates data is stored in the little endian format"]
    #[inline]
    pub fn read_pack_dir(&mut self) -> _READ_PACK_DIRW {
        _READ_PACK_DIRW { w: self }
    }
    #[doc = "Bits 12:14 - This field determines the order of the RGB components of each pixel in EVEN lines (line numbers 2,4,6,"]
    #[inline]
    pub fn even_line_pattern(&mut self) -> _EVEN_LINE_PATTERNW {
        _EVEN_LINE_PATTERNW { w: self }
    }
    #[doc = "Bits 16:18 - This field determines the order of the RGB components of each pixel in ODD lines (line numbers 1,3,5,"]
    #[inline]
    pub fn odd_line_pattern(&mut self) -> _ODD_LINE_PATTERNW {
        _ODD_LINE_PATTERNW { w: self }
    }
    #[doc = "Bit 20 - By default, when the LCDIF is in the bus master mode, it will issue AXI bursts of length 16 (except when in packed 24 bpp mode, it will issue bursts of length 15)"]
    #[inline]
    pub fn burst_len_8(&mut self) -> _BURST_LEN_8W {
        _BURST_LEN_8W { w: self }
    }
    #[doc = "Bits 21:23 - This bitfield indicates the maximum number of outstanding transactions that LCDIF should request when it is acting as a bus master"]
    #[inline]
    pub fn outstanding_reqs(&mut self) -> _OUTSTANDING_REQSW {
        _OUTSTANDING_REQSW { w: self }
    }
}
