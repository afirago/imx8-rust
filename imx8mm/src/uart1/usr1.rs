#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USR1 {
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
#[doc = "Possible values of the field `SAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SADR {
    #[doc = "No slave address detected"]
    SAD_0,
    #[doc = "Slave address detected"]
    SAD_1,
}
impl SADR {
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
            SADR::SAD_0 => false,
            SADR::SAD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SADR {
        match value {
            false => SADR::SAD_0,
            true => SADR::SAD_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAD_0`"]
    #[inline]
    pub fn is_sad_0(&self) -> bool {
        *self == SADR::SAD_0
    }
    #[doc = "Checks if the value of the field is `SAD_1`"]
    #[inline]
    pub fn is_sad_1(&self) -> bool {
        *self == SADR::SAD_1
    }
}
#[doc = "Possible values of the field `AWAKE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWAKER {
    #[doc = "No falling edge was detected on the RXD Serial pin"]
    AWAKE_0,
    #[doc = "A falling edge was detected on the RXD Serial pin"]
    AWAKE_1,
}
impl AWAKER {
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
            AWAKER::AWAKE_0 => false,
            AWAKER::AWAKE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AWAKER {
        match value {
            false => AWAKER::AWAKE_0,
            true => AWAKER::AWAKE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AWAKE_0`"]
    #[inline]
    pub fn is_awake_0(&self) -> bool {
        *self == AWAKER::AWAKE_0
    }
    #[doc = "Checks if the value of the field is `AWAKE_1`"]
    #[inline]
    pub fn is_awake_1(&self) -> bool {
        *self == AWAKER::AWAKE_1
    }
}
#[doc = "Possible values of the field `AIRINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIRINTR {
    #[doc = "No pulse was detected on the RXD IrDA pin"]
    AIRINT_0,
    #[doc = "A pulse was detected on the RXD IrDA pin"]
    AIRINT_1,
}
impl AIRINTR {
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
            AIRINTR::AIRINT_0 => false,
            AIRINTR::AIRINT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AIRINTR {
        match value {
            false => AIRINTR::AIRINT_0,
            true => AIRINTR::AIRINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `AIRINT_0`"]
    #[inline]
    pub fn is_airint_0(&self) -> bool {
        *self == AIRINTR::AIRINT_0
    }
    #[doc = "Checks if the value of the field is `AIRINT_1`"]
    #[inline]
    pub fn is_airint_1(&self) -> bool {
        *self == AIRINTR::AIRINT_1
    }
}
#[doc = "Possible values of the field `RXDS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDSR {
    #[doc = "Receive in progress"]
    RXDS_0,
    #[doc = "Receiver is IDLE"]
    RXDS_1,
}
impl RXDSR {
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
            RXDSR::RXDS_0 => false,
            RXDSR::RXDS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXDSR {
        match value {
            false => RXDSR::RXDS_0,
            true => RXDSR::RXDS_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXDS_0`"]
    #[inline]
    pub fn is_rxds_0(&self) -> bool {
        *self == RXDSR::RXDS_0
    }
    #[doc = "Checks if the value of the field is `RXDS_1`"]
    #[inline]
    pub fn is_rxds_1(&self) -> bool {
        *self == RXDSR::RXDS_1
    }
}
#[doc = r" Value of the field"]
pub struct DTRDR {
    bits: bool,
}
impl DTRDR {
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
#[doc = "Possible values of the field `AGTIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AGTIMR {
    #[doc = "AGTIM is not active"]
    AGTIM_0,
    #[doc = "AGTIM is active (write 1 to clear)"]
    AGTIM_1,
}
impl AGTIMR {
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
            AGTIMR::AGTIM_0 => false,
            AGTIMR::AGTIM_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AGTIMR {
        match value {
            false => AGTIMR::AGTIM_0,
            true => AGTIMR::AGTIM_1,
        }
    }
    #[doc = "Checks if the value of the field is `AGTIM_0`"]
    #[inline]
    pub fn is_agtim_0(&self) -> bool {
        *self == AGTIMR::AGTIM_0
    }
    #[doc = "Checks if the value of the field is `AGTIM_1`"]
    #[inline]
    pub fn is_agtim_1(&self) -> bool {
        *self == AGTIMR::AGTIM_1
    }
}
#[doc = "Possible values of the field `RRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRDYR {
    #[doc = "No character ready"]
    RRDY_0,
    #[doc = "Character(s) ready (interrupt posted)"]
    RRDY_1,
}
impl RRDYR {
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
            RRDYR::RRDY_0 => false,
            RRDYR::RRDY_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RRDYR {
        match value {
            false => RRDYR::RRDY_0,
            true => RRDYR::RRDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `RRDY_0`"]
    #[inline]
    pub fn is_rrdy_0(&self) -> bool {
        *self == RRDYR::RRDY_0
    }
    #[doc = "Checks if the value of the field is `RRDY_1`"]
    #[inline]
    pub fn is_rrdy_1(&self) -> bool {
        *self == RRDYR::RRDY_1
    }
}
#[doc = "Possible values of the field `FRAMERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAMERRR {
    #[doc = "No frame error detected"]
    FRAMERR_0,
    #[doc = "Frame error detected (write 1 to clear)"]
    FRAMERR_1,
}
impl FRAMERRR {
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
            FRAMERRR::FRAMERR_0 => false,
            FRAMERRR::FRAMERR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRAMERRR {
        match value {
            false => FRAMERRR::FRAMERR_0,
            true => FRAMERRR::FRAMERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRAMERR_0`"]
    #[inline]
    pub fn is_framerr_0(&self) -> bool {
        *self == FRAMERRR::FRAMERR_0
    }
    #[doc = "Checks if the value of the field is `FRAMERR_1`"]
    #[inline]
    pub fn is_framerr_1(&self) -> bool {
        *self == FRAMERRR::FRAMERR_1
    }
}
#[doc = "Possible values of the field `ESCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESCFR {
    #[doc = "No escape sequence detected"]
    ESCF_0,
    #[doc = "Escape sequence detected (write 1 to clear)."]
    ESCF_1,
}
impl ESCFR {
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
            ESCFR::ESCF_0 => false,
            ESCFR::ESCF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ESCFR {
        match value {
            false => ESCFR::ESCF_0,
            true => ESCFR::ESCF_1,
        }
    }
    #[doc = "Checks if the value of the field is `ESCF_0`"]
    #[inline]
    pub fn is_escf_0(&self) -> bool {
        *self == ESCFR::ESCF_0
    }
    #[doc = "Checks if the value of the field is `ESCF_1`"]
    #[inline]
    pub fn is_escf_1(&self) -> bool {
        *self == ESCFR::ESCF_1
    }
}
#[doc = "Possible values of the field `RTSD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSDR {
    #[doc = "RTS_B pin did not change state since last cleared"]
    RTSD_0,
    #[doc = "RTS_B pin changed state (write 1 to clear)"]
    RTSD_1,
}
impl RTSDR {
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
            RTSDR::RTSD_0 => false,
            RTSDR::RTSD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTSDR {
        match value {
            false => RTSDR::RTSD_0,
            true => RTSDR::RTSD_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTSD_0`"]
    #[inline]
    pub fn is_rtsd_0(&self) -> bool {
        *self == RTSDR::RTSD_0
    }
    #[doc = "Checks if the value of the field is `RTSD_1`"]
    #[inline]
    pub fn is_rtsd_1(&self) -> bool {
        *self == RTSDR::RTSD_1
    }
}
#[doc = "Possible values of the field `TRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRDYR {
    #[doc = "The transmitter does not require data"]
    TRDY_0,
    #[doc = "The transmitter requires data (interrupt posted)"]
    TRDY_1,
}
impl TRDYR {
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
            TRDYR::TRDY_0 => false,
            TRDYR::TRDY_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRDYR {
        match value {
            false => TRDYR::TRDY_0,
            true => TRDYR::TRDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRDY_0`"]
    #[inline]
    pub fn is_trdy_0(&self) -> bool {
        *self == TRDYR::TRDY_0
    }
    #[doc = "Checks if the value of the field is `TRDY_1`"]
    #[inline]
    pub fn is_trdy_1(&self) -> bool {
        *self == TRDYR::TRDY_1
    }
}
#[doc = "Possible values of the field `RTSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSSR {
    #[doc = "The RTS_B module input is high (inactive)"]
    RTSS_0,
    #[doc = "The RTS_B module input is low (active)"]
    RTSS_1,
}
impl RTSSR {
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
            RTSSR::RTSS_0 => false,
            RTSSR::RTSS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTSSR {
        match value {
            false => RTSSR::RTSS_0,
            true => RTSSR::RTSS_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTSS_0`"]
    #[inline]
    pub fn is_rtss_0(&self) -> bool {
        *self == RTSSR::RTSS_0
    }
    #[doc = "Checks if the value of the field is `RTSS_1`"]
    #[inline]
    pub fn is_rtss_1(&self) -> bool {
        *self == RTSSR::RTSS_1
    }
}
#[doc = "Possible values of the field `PARITYERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITYERRR {
    #[doc = "No parity error detected"]
    PARITYERR_0,
    #[doc = "Parity error detected (write 1 to clear)"]
    PARITYERR_1,
}
impl PARITYERRR {
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
            PARITYERRR::PARITYERR_0 => false,
            PARITYERRR::PARITYERR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PARITYERRR {
        match value {
            false => PARITYERRR::PARITYERR_0,
            true => PARITYERRR::PARITYERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `PARITYERR_0`"]
    #[inline]
    pub fn is_parityerr_0(&self) -> bool {
        *self == PARITYERRR::PARITYERR_0
    }
    #[doc = "Checks if the value of the field is `PARITYERR_1`"]
    #[inline]
    pub fn is_parityerr_1(&self) -> bool {
        *self == PARITYERRR::PARITYERR_1
    }
}
#[doc = "Values that can be written to the field `SAD`"]
pub enum SADW {
    #[doc = "No slave address detected"]
    SAD_0,
    #[doc = "Slave address detected"]
    SAD_1,
}
impl SADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SADW::SAD_0 => false,
            SADW::SAD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SADW<'a> {
    w: &'a mut W,
}
impl<'a> _SADW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SADW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No slave address detected"]
    #[inline]
    pub fn sad_0(self) -> &'a mut W {
        self.variant(SADW::SAD_0)
    }
    #[doc = "Slave address detected"]
    #[inline]
    pub fn sad_1(self) -> &'a mut W {
        self.variant(SADW::SAD_1)
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
#[doc = "Values that can be written to the field `AWAKE`"]
pub enum AWAKEW {
    #[doc = "No falling edge was detected on the RXD Serial pin"]
    AWAKE_0,
    #[doc = "A falling edge was detected on the RXD Serial pin"]
    AWAKE_1,
}
impl AWAKEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AWAKEW::AWAKE_0 => false,
            AWAKEW::AWAKE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AWAKEW<'a> {
    w: &'a mut W,
}
impl<'a> _AWAKEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AWAKEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No falling edge was detected on the RXD Serial pin"]
    #[inline]
    pub fn awake_0(self) -> &'a mut W {
        self.variant(AWAKEW::AWAKE_0)
    }
    #[doc = "A falling edge was detected on the RXD Serial pin"]
    #[inline]
    pub fn awake_1(self) -> &'a mut W {
        self.variant(AWAKEW::AWAKE_1)
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
#[doc = "Values that can be written to the field `AIRINT`"]
pub enum AIRINTW {
    #[doc = "No pulse was detected on the RXD IrDA pin"]
    AIRINT_0,
    #[doc = "A pulse was detected on the RXD IrDA pin"]
    AIRINT_1,
}
impl AIRINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AIRINTW::AIRINT_0 => false,
            AIRINTW::AIRINT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AIRINTW<'a> {
    w: &'a mut W,
}
impl<'a> _AIRINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AIRINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No pulse was detected on the RXD IrDA pin"]
    #[inline]
    pub fn airint_0(self) -> &'a mut W {
        self.variant(AIRINTW::AIRINT_0)
    }
    #[doc = "A pulse was detected on the RXD IrDA pin"]
    #[inline]
    pub fn airint_1(self) -> &'a mut W {
        self.variant(AIRINTW::AIRINT_1)
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
#[doc = r" Proxy"]
pub struct _DTRDW<'a> {
    w: &'a mut W,
}
impl<'a> _DTRDW<'a> {
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
#[doc = "Values that can be written to the field `AGTIM`"]
pub enum AGTIMW {
    #[doc = "AGTIM is not active"]
    AGTIM_0,
    #[doc = "AGTIM is active (write 1 to clear)"]
    AGTIM_1,
}
impl AGTIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AGTIMW::AGTIM_0 => false,
            AGTIMW::AGTIM_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AGTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _AGTIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AGTIMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "AGTIM is not active"]
    #[inline]
    pub fn agtim_0(self) -> &'a mut W {
        self.variant(AGTIMW::AGTIM_0)
    }
    #[doc = "AGTIM is active (write 1 to clear)"]
    #[inline]
    pub fn agtim_1(self) -> &'a mut W {
        self.variant(AGTIMW::AGTIM_1)
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
#[doc = "Values that can be written to the field `FRAMERR`"]
pub enum FRAMERRW {
    #[doc = "No frame error detected"]
    FRAMERR_0,
    #[doc = "Frame error detected (write 1 to clear)"]
    FRAMERR_1,
}
impl FRAMERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRAMERRW::FRAMERR_0 => false,
            FRAMERRW::FRAMERR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRAMERRW<'a> {
    w: &'a mut W,
}
impl<'a> _FRAMERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRAMERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No frame error detected"]
    #[inline]
    pub fn framerr_0(self) -> &'a mut W {
        self.variant(FRAMERRW::FRAMERR_0)
    }
    #[doc = "Frame error detected (write 1 to clear)"]
    #[inline]
    pub fn framerr_1(self) -> &'a mut W {
        self.variant(FRAMERRW::FRAMERR_1)
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
#[doc = "Values that can be written to the field `ESCF`"]
pub enum ESCFW {
    #[doc = "No escape sequence detected"]
    ESCF_0,
    #[doc = "Escape sequence detected (write 1 to clear)."]
    ESCF_1,
}
impl ESCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ESCFW::ESCF_0 => false,
            ESCFW::ESCF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ESCFW<'a> {
    w: &'a mut W,
}
impl<'a> _ESCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ESCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No escape sequence detected"]
    #[inline]
    pub fn escf_0(self) -> &'a mut W {
        self.variant(ESCFW::ESCF_0)
    }
    #[doc = "Escape sequence detected (write 1 to clear)."]
    #[inline]
    pub fn escf_1(self) -> &'a mut W {
        self.variant(ESCFW::ESCF_1)
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
#[doc = "Values that can be written to the field `RTSD`"]
pub enum RTSDW {
    #[doc = "RTS_B pin did not change state since last cleared"]
    RTSD_0,
    #[doc = "RTS_B pin changed state (write 1 to clear)"]
    RTSD_1,
}
impl RTSDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTSDW::RTSD_0 => false,
            RTSDW::RTSD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTSDW<'a> {
    w: &'a mut W,
}
impl<'a> _RTSDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTSDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RTS_B pin did not change state since last cleared"]
    #[inline]
    pub fn rtsd_0(self) -> &'a mut W {
        self.variant(RTSDW::RTSD_0)
    }
    #[doc = "RTS_B pin changed state (write 1 to clear)"]
    #[inline]
    pub fn rtsd_1(self) -> &'a mut W {
        self.variant(RTSDW::RTSD_1)
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
#[doc = "Values that can be written to the field `PARITYERR`"]
pub enum PARITYERRW {
    #[doc = "No parity error detected"]
    PARITYERR_0,
    #[doc = "Parity error detected (write 1 to clear)"]
    PARITYERR_1,
}
impl PARITYERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PARITYERRW::PARITYERR_0 => false,
            PARITYERRW::PARITYERR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PARITYERRW<'a> {
    w: &'a mut W,
}
impl<'a> _PARITYERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PARITYERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No parity error detected"]
    #[inline]
    pub fn parityerr_0(self) -> &'a mut W {
        self.variant(PARITYERRW::PARITYERR_0)
    }
    #[doc = "Parity error detected (write 1 to clear)"]
    #[inline]
    pub fn parityerr_1(self) -> &'a mut W {
        self.variant(PARITYERRW::PARITYERR_1)
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
        const OFFSET: u8 = 15;
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
    #[doc = "Bit 3 - RS-485 Slave Address Detected Interrupt Flag"]
    #[inline]
    pub fn sad(&self) -> SADR {
        SADR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Asynchronous WAKE Interrupt Flag"]
    #[inline]
    pub fn awake(&self) -> AWAKER {
        AWAKER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Asynchronous IR WAKE Interrupt Flag"]
    #[inline]
    pub fn airint(&self) -> AIRINTR {
        AIRINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Receiver IDLE Interrupt Flag"]
    #[inline]
    pub fn rxds(&self) -> RXDSR {
        RXDSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - This bit is not used in this chip."]
    #[inline]
    pub fn dtrd(&self) -> DTRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTRDR { bits }
    }
    #[doc = "Bit 8 - Ageing Timer Interrupt Flag"]
    #[inline]
    pub fn agtim(&self) -> AGTIMR {
        AGTIMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Receiver Ready Interrupt / DMA Flag"]
    #[inline]
    pub fn rrdy(&self) -> RRDYR {
        RRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Frame Error Interrupt Flag"]
    #[inline]
    pub fn framerr(&self) -> FRAMERRR {
        FRAMERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Escape Sequence Interrupt Flag"]
    #[inline]
    pub fn escf(&self) -> ESCFR {
        ESCFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - RTS Delta"]
    #[inline]
    pub fn rtsd(&self) -> RTSDR {
        RTSDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Transmitter Ready Interrupt / DMA Flag"]
    #[inline]
    pub fn trdy(&self) -> TRDYR {
        TRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - RTS_B Pin Status"]
    #[inline]
    pub fn rtss(&self) -> RTSSR {
        RTSSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Parity Error Interrupt Flag"]
    #[inline]
    pub fn parityerr(&self) -> PARITYERRR {
        PARITYERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 8256 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 3 - RS-485 Slave Address Detected Interrupt Flag"]
    #[inline]
    pub fn sad(&mut self) -> _SADW {
        _SADW { w: self }
    }
    #[doc = "Bit 4 - Asynchronous WAKE Interrupt Flag"]
    #[inline]
    pub fn awake(&mut self) -> _AWAKEW {
        _AWAKEW { w: self }
    }
    #[doc = "Bit 5 - Asynchronous IR WAKE Interrupt Flag"]
    #[inline]
    pub fn airint(&mut self) -> _AIRINTW {
        _AIRINTW { w: self }
    }
    #[doc = "Bit 7 - This bit is not used in this chip."]
    #[inline]
    pub fn dtrd(&mut self) -> _DTRDW {
        _DTRDW { w: self }
    }
    #[doc = "Bit 8 - Ageing Timer Interrupt Flag"]
    #[inline]
    pub fn agtim(&mut self) -> _AGTIMW {
        _AGTIMW { w: self }
    }
    #[doc = "Bit 10 - Frame Error Interrupt Flag"]
    #[inline]
    pub fn framerr(&mut self) -> _FRAMERRW {
        _FRAMERRW { w: self }
    }
    #[doc = "Bit 11 - Escape Sequence Interrupt Flag"]
    #[inline]
    pub fn escf(&mut self) -> _ESCFW {
        _ESCFW { w: self }
    }
    #[doc = "Bit 12 - RTS Delta"]
    #[inline]
    pub fn rtsd(&mut self) -> _RTSDW {
        _RTSDW { w: self }
    }
    #[doc = "Bit 15 - Parity Error Interrupt Flag"]
    #[inline]
    pub fn parityerr(&mut self) -> _PARITYERRW {
        _PARITYERRW { w: self }
    }
}
