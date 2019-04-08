#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::UFCR {
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
#[doc = "Possible values of the field `RXTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTLR {
    #[doc = "0 characters received"]
    RXTL_0,
    #[doc = "RxFIFO has 1 character"]
    RXTL_1,
    #[doc = "RxFIFO has 31 characters"]
    RXTL_31,
    #[doc = "RxFIFO has 32 characters (maximum)"]
    RXTL_32,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RXTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXTLR::RXTL_0 => 0,
            RXTLR::RXTL_1 => 1,
            RXTLR::RXTL_31 => 31,
            RXTLR::RXTL_32 => 32,
            RXTLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXTLR {
        match value {
            0 => RXTLR::RXTL_0,
            1 => RXTLR::RXTL_1,
            31 => RXTLR::RXTL_31,
            32 => RXTLR::RXTL_32,
            i => RXTLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RXTL_0`"]
    #[inline]
    pub fn is_rxtl_0(&self) -> bool {
        *self == RXTLR::RXTL_0
    }
    #[doc = "Checks if the value of the field is `RXTL_1`"]
    #[inline]
    pub fn is_rxtl_1(&self) -> bool {
        *self == RXTLR::RXTL_1
    }
    #[doc = "Checks if the value of the field is `RXTL_31`"]
    #[inline]
    pub fn is_rxtl_31(&self) -> bool {
        *self == RXTLR::RXTL_31
    }
    #[doc = "Checks if the value of the field is `RXTL_32`"]
    #[inline]
    pub fn is_rxtl_32(&self) -> bool {
        *self == RXTLR::RXTL_32
    }
}
#[doc = "Possible values of the field `DCEDTE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCEDTER {
    #[doc = "DCE mode selected"]
    DCEDTE_0,
    #[doc = "DTE mode selected"]
    DCEDTE_1,
}
impl DCEDTER {
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
            DCEDTER::DCEDTE_0 => false,
            DCEDTER::DCEDTE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCEDTER {
        match value {
            false => DCEDTER::DCEDTE_0,
            true => DCEDTER::DCEDTE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCEDTE_0`"]
    #[inline]
    pub fn is_dcedte_0(&self) -> bool {
        *self == DCEDTER::DCEDTE_0
    }
    #[doc = "Checks if the value of the field is `DCEDTE_1`"]
    #[inline]
    pub fn is_dcedte_1(&self) -> bool {
        *self == DCEDTER::DCEDTE_1
    }
}
#[doc = "Possible values of the field `RFDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFDIVR {
    #[doc = "Divide input clock by 6"]
    RFDIV_0,
    #[doc = "Divide input clock by 5"]
    RFDIV_1,
    #[doc = "Divide input clock by 4"]
    RFDIV_2,
    #[doc = "Divide input clock by 3"]
    RFDIV_3,
    #[doc = "Divide input clock by 2"]
    RFDIV_4,
    #[doc = "Divide input clock by 1"]
    RFDIV_5,
    #[doc = "Divide input clock by 7"]
    RFDIV_6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RFDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RFDIVR::RFDIV_0 => 0,
            RFDIVR::RFDIV_1 => 1,
            RFDIVR::RFDIV_2 => 2,
            RFDIVR::RFDIV_3 => 3,
            RFDIVR::RFDIV_4 => 4,
            RFDIVR::RFDIV_5 => 5,
            RFDIVR::RFDIV_6 => 6,
            RFDIVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RFDIVR {
        match value {
            0 => RFDIVR::RFDIV_0,
            1 => RFDIVR::RFDIV_1,
            2 => RFDIVR::RFDIV_2,
            3 => RFDIVR::RFDIV_3,
            4 => RFDIVR::RFDIV_4,
            5 => RFDIVR::RFDIV_5,
            6 => RFDIVR::RFDIV_6,
            i => RFDIVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RFDIV_0`"]
    #[inline]
    pub fn is_rfdiv_0(&self) -> bool {
        *self == RFDIVR::RFDIV_0
    }
    #[doc = "Checks if the value of the field is `RFDIV_1`"]
    #[inline]
    pub fn is_rfdiv_1(&self) -> bool {
        *self == RFDIVR::RFDIV_1
    }
    #[doc = "Checks if the value of the field is `RFDIV_2`"]
    #[inline]
    pub fn is_rfdiv_2(&self) -> bool {
        *self == RFDIVR::RFDIV_2
    }
    #[doc = "Checks if the value of the field is `RFDIV_3`"]
    #[inline]
    pub fn is_rfdiv_3(&self) -> bool {
        *self == RFDIVR::RFDIV_3
    }
    #[doc = "Checks if the value of the field is `RFDIV_4`"]
    #[inline]
    pub fn is_rfdiv_4(&self) -> bool {
        *self == RFDIVR::RFDIV_4
    }
    #[doc = "Checks if the value of the field is `RFDIV_5`"]
    #[inline]
    pub fn is_rfdiv_5(&self) -> bool {
        *self == RFDIVR::RFDIV_5
    }
    #[doc = "Checks if the value of the field is `RFDIV_6`"]
    #[inline]
    pub fn is_rfdiv_6(&self) -> bool {
        *self == RFDIVR::RFDIV_6
    }
}
#[doc = "Possible values of the field `TXTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXTLR {
    #[doc = "TxFIFO has 2 or fewer characters"]
    TXTL_2,
    #[doc = "TxFIFO has 31 or fewer characters"]
    TXTL_31,
    #[doc = "TxFIFO has 32 characters (maximum)"]
    TXTL_32,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TXTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXTLR::TXTL_2 => 2,
            TXTLR::TXTL_31 => 31,
            TXTLR::TXTL_32 => 32,
            TXTLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXTLR {
        match value {
            2 => TXTLR::TXTL_2,
            31 => TXTLR::TXTL_31,
            32 => TXTLR::TXTL_32,
            i => TXTLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TXTL_2`"]
    #[inline]
    pub fn is_txtl_2(&self) -> bool {
        *self == TXTLR::TXTL_2
    }
    #[doc = "Checks if the value of the field is `TXTL_31`"]
    #[inline]
    pub fn is_txtl_31(&self) -> bool {
        *self == TXTLR::TXTL_31
    }
    #[doc = "Checks if the value of the field is `TXTL_32`"]
    #[inline]
    pub fn is_txtl_32(&self) -> bool {
        *self == TXTLR::TXTL_32
    }
}
#[doc = "Values that can be written to the field `RXTL`"]
pub enum RXTLW {
    #[doc = "0 characters received"]
    RXTL_0,
    #[doc = "RxFIFO has 1 character"]
    RXTL_1,
    #[doc = "RxFIFO has 31 characters"]
    RXTL_31,
    #[doc = "RxFIFO has 32 characters (maximum)"]
    RXTL_32,
}
impl RXTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXTLW::RXTL_0 => 0,
            RXTLW::RXTL_1 => 1,
            RXTLW::RXTL_31 => 31,
            RXTLW::RXTL_32 => 32,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXTLW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXTLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "0 characters received"]
    #[inline]
    pub fn rxtl_0(self) -> &'a mut W {
        self.variant(RXTLW::RXTL_0)
    }
    #[doc = "RxFIFO has 1 character"]
    #[inline]
    pub fn rxtl_1(self) -> &'a mut W {
        self.variant(RXTLW::RXTL_1)
    }
    #[doc = "RxFIFO has 31 characters"]
    #[inline]
    pub fn rxtl_31(self) -> &'a mut W {
        self.variant(RXTLW::RXTL_31)
    }
    #[doc = "RxFIFO has 32 characters (maximum)"]
    #[inline]
    pub fn rxtl_32(self) -> &'a mut W {
        self.variant(RXTLW::RXTL_32)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCEDTE`"]
pub enum DCEDTEW {
    #[doc = "DCE mode selected"]
    DCEDTE_0,
    #[doc = "DTE mode selected"]
    DCEDTE_1,
}
impl DCEDTEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCEDTEW::DCEDTE_0 => false,
            DCEDTEW::DCEDTE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCEDTEW<'a> {
    w: &'a mut W,
}
impl<'a> _DCEDTEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCEDTEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DCE mode selected"]
    #[inline]
    pub fn dcedte_0(self) -> &'a mut W {
        self.variant(DCEDTEW::DCEDTE_0)
    }
    #[doc = "DTE mode selected"]
    #[inline]
    pub fn dcedte_1(self) -> &'a mut W {
        self.variant(DCEDTEW::DCEDTE_1)
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
#[doc = "Values that can be written to the field `RFDIV`"]
pub enum RFDIVW {
    #[doc = "Divide input clock by 6"]
    RFDIV_0,
    #[doc = "Divide input clock by 5"]
    RFDIV_1,
    #[doc = "Divide input clock by 4"]
    RFDIV_2,
    #[doc = "Divide input clock by 3"]
    RFDIV_3,
    #[doc = "Divide input clock by 2"]
    RFDIV_4,
    #[doc = "Divide input clock by 1"]
    RFDIV_5,
    #[doc = "Divide input clock by 7"]
    RFDIV_6,
}
impl RFDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RFDIVW::RFDIV_0 => 0,
            RFDIVW::RFDIV_1 => 1,
            RFDIVW::RFDIV_2 => 2,
            RFDIVW::RFDIV_3 => 3,
            RFDIVW::RFDIV_4 => 4,
            RFDIVW::RFDIV_5 => 5,
            RFDIVW::RFDIV_6 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RFDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _RFDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RFDIVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Divide input clock by 6"]
    #[inline]
    pub fn rfdiv_0(self) -> &'a mut W {
        self.variant(RFDIVW::RFDIV_0)
    }
    #[doc = "Divide input clock by 5"]
    #[inline]
    pub fn rfdiv_1(self) -> &'a mut W {
        self.variant(RFDIVW::RFDIV_1)
    }
    #[doc = "Divide input clock by 4"]
    #[inline]
    pub fn rfdiv_2(self) -> &'a mut W {
        self.variant(RFDIVW::RFDIV_2)
    }
    #[doc = "Divide input clock by 3"]
    #[inline]
    pub fn rfdiv_3(self) -> &'a mut W {
        self.variant(RFDIVW::RFDIV_3)
    }
    #[doc = "Divide input clock by 2"]
    #[inline]
    pub fn rfdiv_4(self) -> &'a mut W {
        self.variant(RFDIVW::RFDIV_4)
    }
    #[doc = "Divide input clock by 1"]
    #[inline]
    pub fn rfdiv_5(self) -> &'a mut W {
        self.variant(RFDIVW::RFDIV_5)
    }
    #[doc = "Divide input clock by 7"]
    #[inline]
    pub fn rfdiv_6(self) -> &'a mut W {
        self.variant(RFDIVW::RFDIV_6)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXTL`"]
pub enum TXTLW {
    #[doc = "TxFIFO has 2 or fewer characters"]
    TXTL_2,
    #[doc = "TxFIFO has 31 or fewer characters"]
    TXTL_31,
    #[doc = "TxFIFO has 32 characters (maximum)"]
    TXTL_32,
}
impl TXTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXTLW::TXTL_2 => 2,
            TXTLW::TXTL_31 => 31,
            TXTLW::TXTL_32 => 32,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXTLW<'a> {
    w: &'a mut W,
}
impl<'a> _TXTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXTLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "TxFIFO has 2 or fewer characters"]
    #[inline]
    pub fn txtl_2(self) -> &'a mut W {
        self.variant(TXTLW::TXTL_2)
    }
    #[doc = "TxFIFO has 31 or fewer characters"]
    #[inline]
    pub fn txtl_31(self) -> &'a mut W {
        self.variant(TXTLW::TXTL_31)
    }
    #[doc = "TxFIFO has 32 characters (maximum)"]
    #[inline]
    pub fn txtl_32(self) -> &'a mut W {
        self.variant(TXTLW::TXTL_32)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 10;
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
    #[doc = "Bits 0:5 - Receiver Trigger Level"]
    #[inline]
    pub fn rxtl(&self) -> RXTLR {
        RXTLR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - DCE/DTE mode select"]
    #[inline]
    pub fn dcedte(&self) -> DCEDTER {
        DCEDTER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 7:9 - Reference Frequency Divider"]
    #[inline]
    pub fn rfdiv(&self) -> RFDIVR {
        RFDIVR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:15 - Transmitter Trigger Level"]
    #[inline]
    pub fn txtl(&self) -> TXTLR {
        TXTLR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2049 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - Receiver Trigger Level"]
    #[inline]
    pub fn rxtl(&mut self) -> _RXTLW {
        _RXTLW { w: self }
    }
    #[doc = "Bit 6 - DCE/DTE mode select"]
    #[inline]
    pub fn dcedte(&mut self) -> _DCEDTEW {
        _DCEDTEW { w: self }
    }
    #[doc = "Bits 7:9 - Reference Frequency Divider"]
    #[inline]
    pub fn rfdiv(&mut self) -> _RFDIVW {
        _RFDIVW { w: self }
    }
    #[doc = "Bits 10:15 - Transmitter Trigger Level"]
    #[inline]
    pub fn txtl(&mut self) -> _TXTLW {
        _TXTLW { w: self }
    }
}
