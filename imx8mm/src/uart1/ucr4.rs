#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::UCR4 {
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
#[doc = "Possible values of the field `DREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRENR {
    #[doc = "Disable RDR interrupt"]
    DREN_0,
    #[doc = "Enable RDR interrupt"]
    DREN_1,
}
impl DRENR {
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
            DRENR::DREN_0 => false,
            DRENR::DREN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DRENR {
        match value {
            false => DRENR::DREN_0,
            true => DRENR::DREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DREN_0`"]
    #[inline]
    pub fn is_dren_0(&self) -> bool {
        *self == DRENR::DREN_0
    }
    #[doc = "Checks if the value of the field is `DREN_1`"]
    #[inline]
    pub fn is_dren_1(&self) -> bool {
        *self == DRENR::DREN_1
    }
}
#[doc = "Possible values of the field `OREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ORENR {
    #[doc = "Disable ORE interrupt"]
    OREN_0,
    #[doc = "Enable ORE interrupt"]
    OREN_1,
}
impl ORENR {
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
            ORENR::OREN_0 => false,
            ORENR::OREN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ORENR {
        match value {
            false => ORENR::OREN_0,
            true => ORENR::OREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `OREN_0`"]
    #[inline]
    pub fn is_oren_0(&self) -> bool {
        *self == ORENR::OREN_0
    }
    #[doc = "Checks if the value of the field is `OREN_1`"]
    #[inline]
    pub fn is_oren_1(&self) -> bool {
        *self == ORENR::OREN_1
    }
}
#[doc = "Possible values of the field `BKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKENR {
    #[doc = "Disable the BRCD interrupt"]
    BKEN_0,
    #[doc = "Enable the BRCD interrupt"]
    BKEN_1,
}
impl BKENR {
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
            BKENR::BKEN_0 => false,
            BKENR::BKEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BKENR {
        match value {
            false => BKENR::BKEN_0,
            true => BKENR::BKEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BKEN_0`"]
    #[inline]
    pub fn is_bken_0(&self) -> bool {
        *self == BKENR::BKEN_0
    }
    #[doc = "Checks if the value of the field is `BKEN_1`"]
    #[inline]
    pub fn is_bken_1(&self) -> bool {
        *self == BKENR::BKEN_1
    }
}
#[doc = "Possible values of the field `TCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCENR {
    #[doc = "Disable TXDC interrupt"]
    TCEN_0,
    #[doc = "Enable TXDC interrupt"]
    TCEN_1,
}
impl TCENR {
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
            TCENR::TCEN_0 => false,
            TCENR::TCEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCENR {
        match value {
            false => TCENR::TCEN_0,
            true => TCENR::TCEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCEN_0`"]
    #[inline]
    pub fn is_tcen_0(&self) -> bool {
        *self == TCENR::TCEN_0
    }
    #[doc = "Checks if the value of the field is `TCEN_1`"]
    #[inline]
    pub fn is_tcen_1(&self) -> bool {
        *self == TCENR::TCEN_1
    }
}
#[doc = "Possible values of the field `LPBYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPBYPR {
    #[doc = "Low power features enabled"]
    LPBYP_0,
    #[doc = "Low power features disabled"]
    LPBYP_1,
}
impl LPBYPR {
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
            LPBYPR::LPBYP_0 => false,
            LPBYPR::LPBYP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPBYPR {
        match value {
            false => LPBYPR::LPBYP_0,
            true => LPBYPR::LPBYP_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPBYP_0`"]
    #[inline]
    pub fn is_lpbyp_0(&self) -> bool {
        *self == LPBYPR::LPBYP_0
    }
    #[doc = "Checks if the value of the field is `LPBYP_1`"]
    #[inline]
    pub fn is_lpbyp_1(&self) -> bool {
        *self == LPBYPR::LPBYP_1
    }
}
#[doc = "Possible values of the field `IRSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRSCR {
    #[doc = "The vote logic uses the sampling clock (16x baud rate) for normal operation"]
    IRSC_0,
    #[doc = "The vote logic uses the UART reference clock"]
    IRSC_1,
}
impl IRSCR {
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
            IRSCR::IRSC_0 => false,
            IRSCR::IRSC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRSCR {
        match value {
            false => IRSCR::IRSC_0,
            true => IRSCR::IRSC_1,
        }
    }
    #[doc = "Checks if the value of the field is `IRSC_0`"]
    #[inline]
    pub fn is_irsc_0(&self) -> bool {
        *self == IRSCR::IRSC_0
    }
    #[doc = "Checks if the value of the field is `IRSC_1`"]
    #[inline]
    pub fn is_irsc_1(&self) -> bool {
        *self == IRSCR::IRSC_1
    }
}
#[doc = "Possible values of the field `IDDMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDDMAENR {
    #[doc = "DMA IDLE interrupt disabled"]
    IDDMAEN_0,
    #[doc = "DMA IDLE interrupt enabled"]
    IDDMAEN_1,
}
impl IDDMAENR {
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
            IDDMAENR::IDDMAEN_0 => false,
            IDDMAENR::IDDMAEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDDMAENR {
        match value {
            false => IDDMAENR::IDDMAEN_0,
            true => IDDMAENR::IDDMAEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `IDDMAEN_0`"]
    #[inline]
    pub fn is_iddmaen_0(&self) -> bool {
        *self == IDDMAENR::IDDMAEN_0
    }
    #[doc = "Checks if the value of the field is `IDDMAEN_1`"]
    #[inline]
    pub fn is_iddmaen_1(&self) -> bool {
        *self == IDDMAENR::IDDMAEN_1
    }
}
#[doc = "Possible values of the field `WKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKENR {
    #[doc = "Disable the WAKE interrupt"]
    WKEN_0,
    #[doc = "Enable the WAKE interrupt"]
    WKEN_1,
}
impl WKENR {
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
            WKENR::WKEN_0 => false,
            WKENR::WKEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKENR {
        match value {
            false => WKENR::WKEN_0,
            true => WKENR::WKEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `WKEN_0`"]
    #[inline]
    pub fn is_wken_0(&self) -> bool {
        *self == WKENR::WKEN_0
    }
    #[doc = "Checks if the value of the field is `WKEN_1`"]
    #[inline]
    pub fn is_wken_1(&self) -> bool {
        *self == WKENR::WKEN_1
    }
}
#[doc = "Possible values of the field `ENIRI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENIRIR {
    #[doc = "Serial infrared Interrupt disabled"]
    ENIRI_0,
    #[doc = "Serial infrared Interrupt enabled"]
    ENIRI_1,
}
impl ENIRIR {
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
            ENIRIR::ENIRI_0 => false,
            ENIRIR::ENIRI_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENIRIR {
        match value {
            false => ENIRIR::ENIRI_0,
            true => ENIRIR::ENIRI_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENIRI_0`"]
    #[inline]
    pub fn is_eniri_0(&self) -> bool {
        *self == ENIRIR::ENIRI_0
    }
    #[doc = "Checks if the value of the field is `ENIRI_1`"]
    #[inline]
    pub fn is_eniri_1(&self) -> bool {
        *self == ENIRIR::ENIRI_1
    }
}
#[doc = "Possible values of the field `INVR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVRR {
    #[doc = "RXD input is not inverted"]
    INVR_0,
    #[doc = "RXD input is inverted"]
    INVR_1,
}
impl INVRR {
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
            INVRR::INVR_0 => false,
            INVRR::INVR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVRR {
        match value {
            false => INVRR::INVR_0,
            true => INVRR::INVR_1,
        }
    }
    #[doc = "Checks if the value of the field is `INVR_0`"]
    #[inline]
    pub fn is_invr_0(&self) -> bool {
        *self == INVRR::INVR_0
    }
    #[doc = "Checks if the value of the field is `INVR_1`"]
    #[inline]
    pub fn is_invr_1(&self) -> bool {
        *self == INVRR::INVR_1
    }
}
#[doc = "Possible values of the field `CTSTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSTLR {
    #[doc = "0 characters received"]
    CTSTL_0,
    #[doc = "1 characters in the RxFIFO"]
    CTSTL_1,
    #[doc = "32 characters in the RxFIFO (maximum)"]
    CTSTL_32,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CTSTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CTSTLR::CTSTL_0 => 0,
            CTSTLR::CTSTL_1 => 1,
            CTSTLR::CTSTL_32 => 32,
            CTSTLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CTSTLR {
        match value {
            0 => CTSTLR::CTSTL_0,
            1 => CTSTLR::CTSTL_1,
            32 => CTSTLR::CTSTL_32,
            i => CTSTLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CTSTL_0`"]
    #[inline]
    pub fn is_ctstl_0(&self) -> bool {
        *self == CTSTLR::CTSTL_0
    }
    #[doc = "Checks if the value of the field is `CTSTL_1`"]
    #[inline]
    pub fn is_ctstl_1(&self) -> bool {
        *self == CTSTLR::CTSTL_1
    }
    #[doc = "Checks if the value of the field is `CTSTL_32`"]
    #[inline]
    pub fn is_ctstl_32(&self) -> bool {
        *self == CTSTLR::CTSTL_32
    }
}
#[doc = "Values that can be written to the field `DREN`"]
pub enum DRENW {
    #[doc = "Disable RDR interrupt"]
    DREN_0,
    #[doc = "Enable RDR interrupt"]
    DREN_1,
}
impl DRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DRENW::DREN_0 => false,
            DRENW::DREN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DRENW<'a> {
    w: &'a mut W,
}
impl<'a> _DRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable RDR interrupt"]
    #[inline]
    pub fn dren_0(self) -> &'a mut W {
        self.variant(DRENW::DREN_0)
    }
    #[doc = "Enable RDR interrupt"]
    #[inline]
    pub fn dren_1(self) -> &'a mut W {
        self.variant(DRENW::DREN_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OREN`"]
pub enum ORENW {
    #[doc = "Disable ORE interrupt"]
    OREN_0,
    #[doc = "Enable ORE interrupt"]
    OREN_1,
}
impl ORENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ORENW::OREN_0 => false,
            ORENW::OREN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ORENW<'a> {
    w: &'a mut W,
}
impl<'a> _ORENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ORENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable ORE interrupt"]
    #[inline]
    pub fn oren_0(self) -> &'a mut W {
        self.variant(ORENW::OREN_0)
    }
    #[doc = "Enable ORE interrupt"]
    #[inline]
    pub fn oren_1(self) -> &'a mut W {
        self.variant(ORENW::OREN_1)
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
#[doc = "Values that can be written to the field `BKEN`"]
pub enum BKENW {
    #[doc = "Disable the BRCD interrupt"]
    BKEN_0,
    #[doc = "Enable the BRCD interrupt"]
    BKEN_1,
}
impl BKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BKENW::BKEN_0 => false,
            BKENW::BKEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BKENW<'a> {
    w: &'a mut W,
}
impl<'a> _BKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the BRCD interrupt"]
    #[inline]
    pub fn bken_0(self) -> &'a mut W {
        self.variant(BKENW::BKEN_0)
    }
    #[doc = "Enable the BRCD interrupt"]
    #[inline]
    pub fn bken_1(self) -> &'a mut W {
        self.variant(BKENW::BKEN_1)
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
#[doc = "Values that can be written to the field `TCEN`"]
pub enum TCENW {
    #[doc = "Disable TXDC interrupt"]
    TCEN_0,
    #[doc = "Enable TXDC interrupt"]
    TCEN_1,
}
impl TCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCENW::TCEN_0 => false,
            TCENW::TCEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCENW<'a> {
    w: &'a mut W,
}
impl<'a> _TCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable TXDC interrupt"]
    #[inline]
    pub fn tcen_0(self) -> &'a mut W {
        self.variant(TCENW::TCEN_0)
    }
    #[doc = "Enable TXDC interrupt"]
    #[inline]
    pub fn tcen_1(self) -> &'a mut W {
        self.variant(TCENW::TCEN_1)
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
#[doc = "Values that can be written to the field `LPBYP`"]
pub enum LPBYPW {
    #[doc = "Low power features enabled"]
    LPBYP_0,
    #[doc = "Low power features disabled"]
    LPBYP_1,
}
impl LPBYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPBYPW::LPBYP_0 => false,
            LPBYPW::LPBYP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPBYPW<'a> {
    w: &'a mut W,
}
impl<'a> _LPBYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPBYPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low power features enabled"]
    #[inline]
    pub fn lpbyp_0(self) -> &'a mut W {
        self.variant(LPBYPW::LPBYP_0)
    }
    #[doc = "Low power features disabled"]
    #[inline]
    pub fn lpbyp_1(self) -> &'a mut W {
        self.variant(LPBYPW::LPBYP_1)
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
#[doc = "Values that can be written to the field `IRSC`"]
pub enum IRSCW {
    #[doc = "The vote logic uses the sampling clock (16x baud rate) for normal operation"]
    IRSC_0,
    #[doc = "The vote logic uses the UART reference clock"]
    IRSC_1,
}
impl IRSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRSCW::IRSC_0 => false,
            IRSCW::IRSC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRSCW<'a> {
    w: &'a mut W,
}
impl<'a> _IRSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRSCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The vote logic uses the sampling clock (16x baud rate) for normal operation"]
    #[inline]
    pub fn irsc_0(self) -> &'a mut W {
        self.variant(IRSCW::IRSC_0)
    }
    #[doc = "The vote logic uses the UART reference clock"]
    #[inline]
    pub fn irsc_1(self) -> &'a mut W {
        self.variant(IRSCW::IRSC_1)
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
#[doc = "Values that can be written to the field `IDDMAEN`"]
pub enum IDDMAENW {
    #[doc = "DMA IDLE interrupt disabled"]
    IDDMAEN_0,
    #[doc = "DMA IDLE interrupt enabled"]
    IDDMAEN_1,
}
impl IDDMAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IDDMAENW::IDDMAEN_0 => false,
            IDDMAENW::IDDMAEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDDMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _IDDMAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDDMAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA IDLE interrupt disabled"]
    #[inline]
    pub fn iddmaen_0(self) -> &'a mut W {
        self.variant(IDDMAENW::IDDMAEN_0)
    }
    #[doc = "DMA IDLE interrupt enabled"]
    #[inline]
    pub fn iddmaen_1(self) -> &'a mut W {
        self.variant(IDDMAENW::IDDMAEN_1)
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
#[doc = "Values that can be written to the field `WKEN`"]
pub enum WKENW {
    #[doc = "Disable the WAKE interrupt"]
    WKEN_0,
    #[doc = "Enable the WAKE interrupt"]
    WKEN_1,
}
impl WKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKENW::WKEN_0 => false,
            WKENW::WKEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKENW<'a> {
    w: &'a mut W,
}
impl<'a> _WKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the WAKE interrupt"]
    #[inline]
    pub fn wken_0(self) -> &'a mut W {
        self.variant(WKENW::WKEN_0)
    }
    #[doc = "Enable the WAKE interrupt"]
    #[inline]
    pub fn wken_1(self) -> &'a mut W {
        self.variant(WKENW::WKEN_1)
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
#[doc = "Values that can be written to the field `ENIRI`"]
pub enum ENIRIW {
    #[doc = "Serial infrared Interrupt disabled"]
    ENIRI_0,
    #[doc = "Serial infrared Interrupt enabled"]
    ENIRI_1,
}
impl ENIRIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENIRIW::ENIRI_0 => false,
            ENIRIW::ENIRI_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENIRIW<'a> {
    w: &'a mut W,
}
impl<'a> _ENIRIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENIRIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Serial infrared Interrupt disabled"]
    #[inline]
    pub fn eniri_0(self) -> &'a mut W {
        self.variant(ENIRIW::ENIRI_0)
    }
    #[doc = "Serial infrared Interrupt enabled"]
    #[inline]
    pub fn eniri_1(self) -> &'a mut W {
        self.variant(ENIRIW::ENIRI_1)
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
#[doc = "Values that can be written to the field `INVR`"]
pub enum INVRW {
    #[doc = "RXD input is not inverted"]
    INVR_0,
    #[doc = "RXD input is inverted"]
    INVR_1,
}
impl INVRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVRW::INVR_0 => false,
            INVRW::INVR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVRW<'a> {
    w: &'a mut W,
}
impl<'a> _INVRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RXD input is not inverted"]
    #[inline]
    pub fn invr_0(self) -> &'a mut W {
        self.variant(INVRW::INVR_0)
    }
    #[doc = "RXD input is inverted"]
    #[inline]
    pub fn invr_1(self) -> &'a mut W {
        self.variant(INVRW::INVR_1)
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
#[doc = "Values that can be written to the field `CTSTL`"]
pub enum CTSTLW {
    #[doc = "0 characters received"]
    CTSTL_0,
    #[doc = "1 characters in the RxFIFO"]
    CTSTL_1,
    #[doc = "32 characters in the RxFIFO (maximum)"]
    CTSTL_32,
}
impl CTSTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CTSTLW::CTSTL_0 => 0,
            CTSTLW::CTSTL_1 => 1,
            CTSTLW::CTSTL_32 => 32,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTSTLW<'a> {
    w: &'a mut W,
}
impl<'a> _CTSTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTSTLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "0 characters received"]
    #[inline]
    pub fn ctstl_0(self) -> &'a mut W {
        self.variant(CTSTLW::CTSTL_0)
    }
    #[doc = "1 characters in the RxFIFO"]
    #[inline]
    pub fn ctstl_1(self) -> &'a mut W {
        self.variant(CTSTLW::CTSTL_1)
    }
    #[doc = "32 characters in the RxFIFO (maximum)"]
    #[inline]
    pub fn ctstl_32(self) -> &'a mut W {
        self.variant(CTSTLW::CTSTL_32)
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
    #[doc = "Bit 0 - Receive Data Ready Interrupt Enable. Enables/Disables the RDR bit to generate an interrupt."]
    #[inline]
    pub fn dren(&self) -> DRENR {
        DRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Receiver Overrun Interrupt Enable. Enables/Disables the ORE bit to generate an interrupt."]
    #[inline]
    pub fn oren(&self) -> ORENR {
        ORENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - BREAK Condition Detected Interrupt Enable. Enables/Disables the BRCD bit to generate an interrupt."]
    #[inline]
    pub fn bken(&self) -> BKENR {
        BKENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Transmit Complete Interrupt Enable"]
    #[inline]
    pub fn tcen(&self) -> TCENR {
        TCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Low Power Bypass. Allows to bypass the low power new features in UART. To use during debug phase."]
    #[inline]
    pub fn lpbyp(&self) -> LPBYPR {
        LPBYPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - IR Special Case"]
    #[inline]
    pub fn irsc(&self) -> IRSCR {
        IRSCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - DMA IDLE Condition Detected Interrupt Enable Enables/Disables the receive DMA request dma_req_rx for the IDLE interrupt (triggered with IDLE flag in USR2\\[12\\])"]
    #[inline]
    pub fn iddmaen(&self) -> IDDMAENR {
        IDDMAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - WAKE Interrupt Enable"]
    #[inline]
    pub fn wken(&self) -> WKENR {
        WKENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Serial Infrared Interrupt Enable. Enables/Disables the serial infrared interrupt."]
    #[inline]
    pub fn eniri(&self) -> ENIRIR {
        ENIRIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Invert RXD input in RS-232/RS-485 Mode, determine RXD input logic level being sampled in In IrDA mode"]
    #[inline]
    pub fn invr(&self) -> INVRR {
        INVRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 10:15 - CTS Trigger Level"]
    #[inline]
    pub fn ctstl(&self) -> CTSTLR {
        CTSTLR::_from({
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
        W { bits: 32768 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Receive Data Ready Interrupt Enable. Enables/Disables the RDR bit to generate an interrupt."]
    #[inline]
    pub fn dren(&mut self) -> _DRENW {
        _DRENW { w: self }
    }
    #[doc = "Bit 1 - Receiver Overrun Interrupt Enable. Enables/Disables the ORE bit to generate an interrupt."]
    #[inline]
    pub fn oren(&mut self) -> _ORENW {
        _ORENW { w: self }
    }
    #[doc = "Bit 2 - BREAK Condition Detected Interrupt Enable. Enables/Disables the BRCD bit to generate an interrupt."]
    #[inline]
    pub fn bken(&mut self) -> _BKENW {
        _BKENW { w: self }
    }
    #[doc = "Bit 3 - Transmit Complete Interrupt Enable"]
    #[inline]
    pub fn tcen(&mut self) -> _TCENW {
        _TCENW { w: self }
    }
    #[doc = "Bit 4 - Low Power Bypass. Allows to bypass the low power new features in UART. To use during debug phase."]
    #[inline]
    pub fn lpbyp(&mut self) -> _LPBYPW {
        _LPBYPW { w: self }
    }
    #[doc = "Bit 5 - IR Special Case"]
    #[inline]
    pub fn irsc(&mut self) -> _IRSCW {
        _IRSCW { w: self }
    }
    #[doc = "Bit 6 - DMA IDLE Condition Detected Interrupt Enable Enables/Disables the receive DMA request dma_req_rx for the IDLE interrupt (triggered with IDLE flag in USR2\\[12\\])"]
    #[inline]
    pub fn iddmaen(&mut self) -> _IDDMAENW {
        _IDDMAENW { w: self }
    }
    #[doc = "Bit 7 - WAKE Interrupt Enable"]
    #[inline]
    pub fn wken(&mut self) -> _WKENW {
        _WKENW { w: self }
    }
    #[doc = "Bit 8 - Serial Infrared Interrupt Enable. Enables/Disables the serial infrared interrupt."]
    #[inline]
    pub fn eniri(&mut self) -> _ENIRIW {
        _ENIRIW { w: self }
    }
    #[doc = "Bit 9 - Invert RXD input in RS-232/RS-485 Mode, determine RXD input logic level being sampled in In IrDA mode"]
    #[inline]
    pub fn invr(&mut self) -> _INVRW {
        _INVRW { w: self }
    }
    #[doc = "Bits 10:15 - CTS Trigger Level"]
    #[inline]
    pub fn ctstl(&mut self) -> _CTSTLW {
        _CTSTLW { w: self }
    }
}
