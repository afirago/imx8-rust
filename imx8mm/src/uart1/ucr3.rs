#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::UCR3 {
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
#[doc = "Possible values of the field `ACIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACIENR {
    #[doc = "ACST interrupt disabled"]
    ACIEN_0,
    #[doc = "ACST interrupt enabled"]
    ACIEN_1,
}
impl ACIENR {
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
            ACIENR::ACIEN_0 => false,
            ACIENR::ACIEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACIENR {
        match value {
            false => ACIENR::ACIEN_0,
            true => ACIENR::ACIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACIEN_0`"]
    #[inline]
    pub fn is_acien_0(&self) -> bool {
        *self == ACIENR::ACIEN_0
    }
    #[doc = "Checks if the value of the field is `ACIEN_1`"]
    #[inline]
    pub fn is_acien_1(&self) -> bool {
        *self == ACIENR::ACIEN_1
    }
}
#[doc = "Possible values of the field `INVT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVTR {
    #[doc = "TXD is not inverted"]
    INVT_0,
    #[doc = "TXD is inverted"]
    INVT_1,
}
impl INVTR {
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
            INVTR::INVT_0 => false,
            INVTR::INVT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVTR {
        match value {
            false => INVTR::INVT_0,
            true => INVTR::INVT_1,
        }
    }
    #[doc = "Checks if the value of the field is `INVT_0`"]
    #[inline]
    pub fn is_invt_0(&self) -> bool {
        *self == INVTR::INVT_0
    }
    #[doc = "Checks if the value of the field is `INVT_1`"]
    #[inline]
    pub fn is_invt_1(&self) -> bool {
        *self == INVTR::INVT_1
    }
}
#[doc = r" Value of the field"]
pub struct RXDMUXSELR {
    bits: bool,
}
impl RXDMUXSELR {
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
pub struct DTRDENR {
    bits: bool,
}
impl DTRDENR {
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
#[doc = "Possible values of the field `AWAKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWAKENR {
    #[doc = "Disable the AWAKE interrupt"]
    AWAKEN_0,
    #[doc = "Enable the AWAKE interrupt"]
    AWAKEN_1,
}
impl AWAKENR {
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
            AWAKENR::AWAKEN_0 => false,
            AWAKENR::AWAKEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AWAKENR {
        match value {
            false => AWAKENR::AWAKEN_0,
            true => AWAKENR::AWAKEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AWAKEN_0`"]
    #[inline]
    pub fn is_awaken_0(&self) -> bool {
        *self == AWAKENR::AWAKEN_0
    }
    #[doc = "Checks if the value of the field is `AWAKEN_1`"]
    #[inline]
    pub fn is_awaken_1(&self) -> bool {
        *self == AWAKENR::AWAKEN_1
    }
}
#[doc = "Possible values of the field `AIRINTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIRINTENR {
    #[doc = "Disable the AIRINT interrupt"]
    AIRINTEN_0,
    #[doc = "Enable the AIRINT interrupt"]
    AIRINTEN_1,
}
impl AIRINTENR {
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
            AIRINTENR::AIRINTEN_0 => false,
            AIRINTENR::AIRINTEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AIRINTENR {
        match value {
            false => AIRINTENR::AIRINTEN_0,
            true => AIRINTENR::AIRINTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AIRINTEN_0`"]
    #[inline]
    pub fn is_airinten_0(&self) -> bool {
        *self == AIRINTENR::AIRINTEN_0
    }
    #[doc = "Checks if the value of the field is `AIRINTEN_1`"]
    #[inline]
    pub fn is_airinten_1(&self) -> bool {
        *self == AIRINTENR::AIRINTEN_1
    }
}
#[doc = "Possible values of the field `RXDSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDSENR {
    #[doc = "Disable the RXDS interrupt"]
    RXDSEN_0,
    #[doc = "Enable the RXDS interrupt"]
    RXDSEN_1,
}
impl RXDSENR {
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
            RXDSENR::RXDSEN_0 => false,
            RXDSENR::RXDSEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXDSENR {
        match value {
            false => RXDSENR::RXDSEN_0,
            true => RXDSENR::RXDSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXDSEN_0`"]
    #[inline]
    pub fn is_rxdsen_0(&self) -> bool {
        *self == RXDSENR::RXDSEN_0
    }
    #[doc = "Checks if the value of the field is `RXDSEN_1`"]
    #[inline]
    pub fn is_rxdsen_1(&self) -> bool {
        *self == RXDSENR::RXDSEN_1
    }
}
#[doc = "Possible values of the field `ADNIMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADNIMPR {
    #[doc = "Autobaud detection new features selected"]
    ADNIMP_0,
    #[doc = "Keep old autobaud detection mechanism"]
    ADNIMP_1,
}
impl ADNIMPR {
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
            ADNIMPR::ADNIMP_0 => false,
            ADNIMPR::ADNIMP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADNIMPR {
        match value {
            false => ADNIMPR::ADNIMP_0,
            true => ADNIMPR::ADNIMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADNIMP_0`"]
    #[inline]
    pub fn is_adnimp_0(&self) -> bool {
        *self == ADNIMPR::ADNIMP_0
    }
    #[doc = "Checks if the value of the field is `ADNIMP_1`"]
    #[inline]
    pub fn is_adnimp_1(&self) -> bool {
        *self == ADNIMPR::ADNIMP_1
    }
}
#[doc = r" Value of the field"]
pub struct RIR {
    bits: bool,
}
impl RIR {
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
pub struct DCDR {
    bits: bool,
}
impl DCDR {
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
pub struct DSRR {
    bits: bool,
}
impl DSRR {
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
#[doc = "Possible values of the field `FRAERREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAERRENR {
    #[doc = "Disable the frame error interrupt"]
    FRAERREN_0,
    #[doc = "Enable the frame error interrupt"]
    FRAERREN_1,
}
impl FRAERRENR {
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
            FRAERRENR::FRAERREN_0 => false,
            FRAERRENR::FRAERREN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRAERRENR {
        match value {
            false => FRAERRENR::FRAERREN_0,
            true => FRAERRENR::FRAERREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRAERREN_0`"]
    #[inline]
    pub fn is_fraerren_0(&self) -> bool {
        *self == FRAERRENR::FRAERREN_0
    }
    #[doc = "Checks if the value of the field is `FRAERREN_1`"]
    #[inline]
    pub fn is_fraerren_1(&self) -> bool {
        *self == FRAERRENR::FRAERREN_1
    }
}
#[doc = "Possible values of the field `PARERREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARERRENR {
    #[doc = "Disable the parity error interrupt"]
    PARERREN_0,
    #[doc = "Enable the parity error interrupt"]
    PARERREN_1,
}
impl PARERRENR {
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
            PARERRENR::PARERREN_0 => false,
            PARERRENR::PARERREN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PARERRENR {
        match value {
            false => PARERRENR::PARERREN_0,
            true => PARERRENR::PARERREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `PARERREN_0`"]
    #[inline]
    pub fn is_parerren_0(&self) -> bool {
        *self == PARERRENR::PARERREN_0
    }
    #[doc = "Checks if the value of the field is `PARERREN_1`"]
    #[inline]
    pub fn is_parerren_1(&self) -> bool {
        *self == PARERRENR::PARERREN_1
    }
}
#[doc = r" Value of the field"]
pub struct DTRENR {
    bits: bool,
}
impl DTRENR {
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
pub struct DPECR {
    bits: u8,
}
impl DPECR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `ACIEN`"]
pub enum ACIENW {
    #[doc = "ACST interrupt disabled"]
    ACIEN_0,
    #[doc = "ACST interrupt enabled"]
    ACIEN_1,
}
impl ACIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACIENW::ACIEN_0 => false,
            ACIENW::ACIEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACIENW<'a> {
    w: &'a mut W,
}
impl<'a> _ACIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ACST interrupt disabled"]
    #[inline]
    pub fn acien_0(self) -> &'a mut W {
        self.variant(ACIENW::ACIEN_0)
    }
    #[doc = "ACST interrupt enabled"]
    #[inline]
    pub fn acien_1(self) -> &'a mut W {
        self.variant(ACIENW::ACIEN_1)
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
#[doc = "Values that can be written to the field `INVT`"]
pub enum INVTW {
    #[doc = "TXD is not inverted"]
    INVT_0,
    #[doc = "TXD is inverted"]
    INVT_1,
}
impl INVTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVTW::INVT_0 => false,
            INVTW::INVT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVTW<'a> {
    w: &'a mut W,
}
impl<'a> _INVTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TXD is not inverted"]
    #[inline]
    pub fn invt_0(self) -> &'a mut W {
        self.variant(INVTW::INVT_0)
    }
    #[doc = "TXD is inverted"]
    #[inline]
    pub fn invt_1(self) -> &'a mut W {
        self.variant(INVTW::INVT_1)
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
#[doc = r" Proxy"]
pub struct _RXDMUXSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDMUXSELW<'a> {
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
#[doc = r" Proxy"]
pub struct _DTRDENW<'a> {
    w: &'a mut W,
}
impl<'a> _DTRDENW<'a> {
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
#[doc = "Values that can be written to the field `AWAKEN`"]
pub enum AWAKENW {
    #[doc = "Disable the AWAKE interrupt"]
    AWAKEN_0,
    #[doc = "Enable the AWAKE interrupt"]
    AWAKEN_1,
}
impl AWAKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AWAKENW::AWAKEN_0 => false,
            AWAKENW::AWAKEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AWAKENW<'a> {
    w: &'a mut W,
}
impl<'a> _AWAKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AWAKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the AWAKE interrupt"]
    #[inline]
    pub fn awaken_0(self) -> &'a mut W {
        self.variant(AWAKENW::AWAKEN_0)
    }
    #[doc = "Enable the AWAKE interrupt"]
    #[inline]
    pub fn awaken_1(self) -> &'a mut W {
        self.variant(AWAKENW::AWAKEN_1)
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
#[doc = "Values that can be written to the field `AIRINTEN`"]
pub enum AIRINTENW {
    #[doc = "Disable the AIRINT interrupt"]
    AIRINTEN_0,
    #[doc = "Enable the AIRINT interrupt"]
    AIRINTEN_1,
}
impl AIRINTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AIRINTENW::AIRINTEN_0 => false,
            AIRINTENW::AIRINTEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AIRINTENW<'a> {
    w: &'a mut W,
}
impl<'a> _AIRINTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AIRINTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the AIRINT interrupt"]
    #[inline]
    pub fn airinten_0(self) -> &'a mut W {
        self.variant(AIRINTENW::AIRINTEN_0)
    }
    #[doc = "Enable the AIRINT interrupt"]
    #[inline]
    pub fn airinten_1(self) -> &'a mut W {
        self.variant(AIRINTENW::AIRINTEN_1)
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
#[doc = "Values that can be written to the field `RXDSEN`"]
pub enum RXDSENW {
    #[doc = "Disable the RXDS interrupt"]
    RXDSEN_0,
    #[doc = "Enable the RXDS interrupt"]
    RXDSEN_1,
}
impl RXDSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXDSENW::RXDSEN_0 => false,
            RXDSENW::RXDSEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXDSENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXDSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the RXDS interrupt"]
    #[inline]
    pub fn rxdsen_0(self) -> &'a mut W {
        self.variant(RXDSENW::RXDSEN_0)
    }
    #[doc = "Enable the RXDS interrupt"]
    #[inline]
    pub fn rxdsen_1(self) -> &'a mut W {
        self.variant(RXDSENW::RXDSEN_1)
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
#[doc = "Values that can be written to the field `ADNIMP`"]
pub enum ADNIMPW {
    #[doc = "Autobaud detection new features selected"]
    ADNIMP_0,
    #[doc = "Keep old autobaud detection mechanism"]
    ADNIMP_1,
}
impl ADNIMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADNIMPW::ADNIMP_0 => false,
            ADNIMPW::ADNIMP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADNIMPW<'a> {
    w: &'a mut W,
}
impl<'a> _ADNIMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADNIMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Autobaud detection new features selected"]
    #[inline]
    pub fn adnimp_0(self) -> &'a mut W {
        self.variant(ADNIMPW::ADNIMP_0)
    }
    #[doc = "Keep old autobaud detection mechanism"]
    #[inline]
    pub fn adnimp_1(self) -> &'a mut W {
        self.variant(ADNIMPW::ADNIMP_1)
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
#[doc = r" Proxy"]
pub struct _RIW<'a> {
    w: &'a mut W,
}
impl<'a> _RIW<'a> {
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
pub struct _DCDW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDW<'a> {
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
pub struct _DSRW<'a> {
    w: &'a mut W,
}
impl<'a> _DSRW<'a> {
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
#[doc = "Values that can be written to the field `FRAERREN`"]
pub enum FRAERRENW {
    #[doc = "Disable the frame error interrupt"]
    FRAERREN_0,
    #[doc = "Enable the frame error interrupt"]
    FRAERREN_1,
}
impl FRAERRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRAERRENW::FRAERREN_0 => false,
            FRAERRENW::FRAERREN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRAERRENW<'a> {
    w: &'a mut W,
}
impl<'a> _FRAERRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRAERRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the frame error interrupt"]
    #[inline]
    pub fn fraerren_0(self) -> &'a mut W {
        self.variant(FRAERRENW::FRAERREN_0)
    }
    #[doc = "Enable the frame error interrupt"]
    #[inline]
    pub fn fraerren_1(self) -> &'a mut W {
        self.variant(FRAERRENW::FRAERREN_1)
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
#[doc = "Values that can be written to the field `PARERREN`"]
pub enum PARERRENW {
    #[doc = "Disable the parity error interrupt"]
    PARERREN_0,
    #[doc = "Enable the parity error interrupt"]
    PARERREN_1,
}
impl PARERRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PARERRENW::PARERREN_0 => false,
            PARERRENW::PARERREN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PARERRENW<'a> {
    w: &'a mut W,
}
impl<'a> _PARERRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PARERRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the parity error interrupt"]
    #[inline]
    pub fn parerren_0(self) -> &'a mut W {
        self.variant(PARERRENW::PARERREN_0)
    }
    #[doc = "Enable the parity error interrupt"]
    #[inline]
    pub fn parerren_1(self) -> &'a mut W {
        self.variant(PARERRENW::PARERREN_1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DTRENW<'a> {
    w: &'a mut W,
}
impl<'a> _DTRENW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DPECW<'a> {
    w: &'a mut W,
}
impl<'a> _DPECW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
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
    #[doc = "Bit 0 - Autobaud Counter Interrupt Enable"]
    #[inline]
    pub fn acien(&self) -> ACIENR {
        ACIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Invert TXD output in RS-232/RS-485 mode, set TXD active level in IrDA mode"]
    #[inline]
    pub fn invt(&self) -> INVTR {
        INVTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - RXD Muxed Input Selected"]
    #[inline]
    pub fn rxdmuxsel(&self) -> RXDMUXSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXDMUXSELR { bits }
    }
    #[doc = "Bit 3 - This bit is not used in this chip."]
    #[inline]
    pub fn dtrden(&self) -> DTRDENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTRDENR { bits }
    }
    #[doc = "Bit 4 - Asynchronous WAKE Interrupt Enable"]
    #[inline]
    pub fn awaken(&self) -> AWAKENR {
        AWAKENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Asynchronous IR WAKE Interrupt Enable"]
    #[inline]
    pub fn airinten(&self) -> AIRINTENR {
        AIRINTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Receive Status Interrupt Enable"]
    #[inline]
    pub fn rxdsen(&self) -> RXDSENR {
        RXDSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Autobaud Detection Not Improved-"]
    #[inline]
    pub fn adnimp(&self) -> ADNIMPR {
        ADNIMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - This bit is not used in this chip."]
    #[inline]
    pub fn ri(&self) -> RIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RIR { bits }
    }
    #[doc = "Bit 9 - This bit is not used in this chip."]
    #[inline]
    pub fn dcd(&self) -> DCDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDR { bits }
    }
    #[doc = "Bit 10 - This bit is not used in this chip."]
    #[inline]
    pub fn dsr(&self) -> DSRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DSRR { bits }
    }
    #[doc = "Bit 11 - Frame Error Interrupt Enable"]
    #[inline]
    pub fn fraerren(&self) -> FRAERRENR {
        FRAERRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Parity Error Interrupt Enable"]
    #[inline]
    pub fn parerren(&self) -> PARERRENR {
        PARERRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - This bit is not used in this chip."]
    #[inline]
    pub fn dtren(&self) -> DTRENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTRENR { bits }
    }
    #[doc = "Bits 14:15 - This bit is not used in this chip."]
    #[inline]
    pub fn dpec(&self) -> DPECR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DPECR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1792 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Autobaud Counter Interrupt Enable"]
    #[inline]
    pub fn acien(&mut self) -> _ACIENW {
        _ACIENW { w: self }
    }
    #[doc = "Bit 1 - Invert TXD output in RS-232/RS-485 mode, set TXD active level in IrDA mode"]
    #[inline]
    pub fn invt(&mut self) -> _INVTW {
        _INVTW { w: self }
    }
    #[doc = "Bit 2 - RXD Muxed Input Selected"]
    #[inline]
    pub fn rxdmuxsel(&mut self) -> _RXDMUXSELW {
        _RXDMUXSELW { w: self }
    }
    #[doc = "Bit 3 - This bit is not used in this chip."]
    #[inline]
    pub fn dtrden(&mut self) -> _DTRDENW {
        _DTRDENW { w: self }
    }
    #[doc = "Bit 4 - Asynchronous WAKE Interrupt Enable"]
    #[inline]
    pub fn awaken(&mut self) -> _AWAKENW {
        _AWAKENW { w: self }
    }
    #[doc = "Bit 5 - Asynchronous IR WAKE Interrupt Enable"]
    #[inline]
    pub fn airinten(&mut self) -> _AIRINTENW {
        _AIRINTENW { w: self }
    }
    #[doc = "Bit 6 - Receive Status Interrupt Enable"]
    #[inline]
    pub fn rxdsen(&mut self) -> _RXDSENW {
        _RXDSENW { w: self }
    }
    #[doc = "Bit 7 - Autobaud Detection Not Improved-"]
    #[inline]
    pub fn adnimp(&mut self) -> _ADNIMPW {
        _ADNIMPW { w: self }
    }
    #[doc = "Bit 8 - This bit is not used in this chip."]
    #[inline]
    pub fn ri(&mut self) -> _RIW {
        _RIW { w: self }
    }
    #[doc = "Bit 9 - This bit is not used in this chip."]
    #[inline]
    pub fn dcd(&mut self) -> _DCDW {
        _DCDW { w: self }
    }
    #[doc = "Bit 10 - This bit is not used in this chip."]
    #[inline]
    pub fn dsr(&mut self) -> _DSRW {
        _DSRW { w: self }
    }
    #[doc = "Bit 11 - Frame Error Interrupt Enable"]
    #[inline]
    pub fn fraerren(&mut self) -> _FRAERRENW {
        _FRAERRENW { w: self }
    }
    #[doc = "Bit 12 - Parity Error Interrupt Enable"]
    #[inline]
    pub fn parerren(&mut self) -> _PARERRENW {
        _PARERRENW { w: self }
    }
    #[doc = "Bit 13 - This bit is not used in this chip."]
    #[inline]
    pub fn dtren(&mut self) -> _DTRENW {
        _DTRENW { w: self }
    }
    #[doc = "Bits 14:15 - This bit is not used in this chip."]
    #[inline]
    pub fn dpec(&mut self) -> _DPECW {
        _DPECW { w: self }
    }
}
