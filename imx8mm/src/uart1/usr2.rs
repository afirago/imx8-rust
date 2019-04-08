#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USR2 {
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
#[doc = "Possible values of the field `RDR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDRR {
    #[doc = "No receive data ready"]
    RDR_0,
    #[doc = "Receive data ready"]
    RDR_1,
}
impl RDRR {
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
            RDRR::RDR_0 => false,
            RDRR::RDR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDRR {
        match value {
            false => RDRR::RDR_0,
            true => RDRR::RDR_1,
        }
    }
    #[doc = "Checks if the value of the field is `RDR_0`"]
    #[inline]
    pub fn is_rdr_0(&self) -> bool {
        *self == RDRR::RDR_0
    }
    #[doc = "Checks if the value of the field is `RDR_1`"]
    #[inline]
    pub fn is_rdr_1(&self) -> bool {
        *self == RDRR::RDR_1
    }
}
#[doc = "Possible values of the field `ORE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ORER {
    #[doc = "No overrun error"]
    ORE_0,
    #[doc = "Overrun error (write 1 to clear)"]
    ORE_1,
}
impl ORER {
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
            ORER::ORE_0 => false,
            ORER::ORE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ORER {
        match value {
            false => ORER::ORE_0,
            true => ORER::ORE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ORE_0`"]
    #[inline]
    pub fn is_ore_0(&self) -> bool {
        *self == ORER::ORE_0
    }
    #[doc = "Checks if the value of the field is `ORE_1`"]
    #[inline]
    pub fn is_ore_1(&self) -> bool {
        *self == ORER::ORE_1
    }
}
#[doc = "Possible values of the field `BRCD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRCDR {
    #[doc = "No BREAK condition was detected"]
    BRCD_0,
    #[doc = "A BREAK condition was detected (write 1 to clear)"]
    BRCD_1,
}
impl BRCDR {
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
            BRCDR::BRCD_0 => false,
            BRCDR::BRCD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BRCDR {
        match value {
            false => BRCDR::BRCD_0,
            true => BRCDR::BRCD_1,
        }
    }
    #[doc = "Checks if the value of the field is `BRCD_0`"]
    #[inline]
    pub fn is_brcd_0(&self) -> bool {
        *self == BRCDR::BRCD_0
    }
    #[doc = "Checks if the value of the field is `BRCD_1`"]
    #[inline]
    pub fn is_brcd_1(&self) -> bool {
        *self == BRCDR::BRCD_1
    }
}
#[doc = "Possible values of the field `TXDC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDCR {
    #[doc = "Transmit is incomplete"]
    TXDC_0,
    #[doc = "Transmit is complete"]
    TXDC_1,
}
impl TXDCR {
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
            TXDCR::TXDC_0 => false,
            TXDCR::TXDC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXDCR {
        match value {
            false => TXDCR::TXDC_0,
            true => TXDCR::TXDC_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXDC_0`"]
    #[inline]
    pub fn is_txdc_0(&self) -> bool {
        *self == TXDCR::TXDC_0
    }
    #[doc = "Checks if the value of the field is `TXDC_1`"]
    #[inline]
    pub fn is_txdc_1(&self) -> bool {
        *self == TXDCR::TXDC_1
    }
}
#[doc = "Possible values of the field `RTSF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSFR {
    #[doc = "Programmed edge not detected on RTS_B"]
    RTSF_0,
    #[doc = "Programmed edge detected on RTS_B (write 1 to clear)"]
    RTSF_1,
}
impl RTSFR {
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
            RTSFR::RTSF_0 => false,
            RTSFR::RTSF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTSFR {
        match value {
            false => RTSFR::RTSF_0,
            true => RTSFR::RTSF_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTSF_0`"]
    #[inline]
    pub fn is_rtsf_0(&self) -> bool {
        *self == RTSFR::RTSF_0
    }
    #[doc = "Checks if the value of the field is `RTSF_1`"]
    #[inline]
    pub fn is_rtsf_1(&self) -> bool {
        *self == RTSFR::RTSF_1
    }
}
#[doc = r" Value of the field"]
pub struct DCDINR {
    bits: bool,
}
impl DCDINR {
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
pub struct DCDDELTR {
    bits: bool,
}
impl DCDDELTR {
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
#[doc = "Possible values of the field `WAKE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKER {
    #[doc = "start bit not detected"]
    WAKE_0,
    #[doc = "start bit detected (write 1 to clear)"]
    WAKE_1,
}
impl WAKER {
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
            WAKER::WAKE_0 => false,
            WAKER::WAKE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKER {
        match value {
            false => WAKER::WAKE_0,
            true => WAKER::WAKE_1,
        }
    }
    #[doc = "Checks if the value of the field is `WAKE_0`"]
    #[inline]
    pub fn is_wake_0(&self) -> bool {
        *self == WAKER::WAKE_0
    }
    #[doc = "Checks if the value of the field is `WAKE_1`"]
    #[inline]
    pub fn is_wake_1(&self) -> bool {
        *self == WAKER::WAKE_1
    }
}
#[doc = "Possible values of the field `IRINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRINTR {
    #[doc = "no edge detected"]
    IRINT_0,
    #[doc = "valid edge detected (write 1 to clear)"]
    IRINT_1,
}
impl IRINTR {
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
            IRINTR::IRINT_0 => false,
            IRINTR::IRINT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRINTR {
        match value {
            false => IRINTR::IRINT_0,
            true => IRINTR::IRINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `IRINT_0`"]
    #[inline]
    pub fn is_irint_0(&self) -> bool {
        *self == IRINTR::IRINT_0
    }
    #[doc = "Checks if the value of the field is `IRINT_1`"]
    #[inline]
    pub fn is_irint_1(&self) -> bool {
        *self == IRINTR::IRINT_1
    }
}
#[doc = r" Value of the field"]
pub struct RIINR {
    bits: bool,
}
impl RIINR {
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
pub struct RIDELTR {
    bits: bool,
}
impl RIDELTR {
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
#[doc = "Possible values of the field `ACST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACSTR {
    #[doc = "Measurement of bit length not finished (in autobaud)"]
    ACST_0,
    #[doc = "Measurement of bit length finished (in autobaud). (write 1 to clear)"]
    ACST_1,
}
impl ACSTR {
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
            ACSTR::ACST_0 => false,
            ACSTR::ACST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACSTR {
        match value {
            false => ACSTR::ACST_0,
            true => ACSTR::ACST_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACST_0`"]
    #[inline]
    pub fn is_acst_0(&self) -> bool {
        *self == ACSTR::ACST_0
    }
    #[doc = "Checks if the value of the field is `ACST_1`"]
    #[inline]
    pub fn is_acst_1(&self) -> bool {
        *self == ACSTR::ACST_1
    }
}
#[doc = "Possible values of the field `IDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLER {
    #[doc = "No idle condition detected"]
    IDLE_0,
    #[doc = "Idle condition detected (write 1 to clear)"]
    IDLE_1,
}
impl IDLER {
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
            IDLER::IDLE_0 => false,
            IDLER::IDLE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDLER {
        match value {
            false => IDLER::IDLE_0,
            true => IDLER::IDLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_0`"]
    #[inline]
    pub fn is_idle_0(&self) -> bool {
        *self == IDLER::IDLE_0
    }
    #[doc = "Checks if the value of the field is `IDLE_1`"]
    #[inline]
    pub fn is_idle_1(&self) -> bool {
        *self == IDLER::IDLE_1
    }
}
#[doc = r" Value of the field"]
pub struct DTRFR {
    bits: bool,
}
impl DTRFR {
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
#[doc = "Possible values of the field `TXFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFER {
    #[doc = "The transmit buffer (TxFIFO) is not empty"]
    TXFE_0,
    #[doc = "The transmit buffer (TxFIFO) is empty"]
    TXFE_1,
}
impl TXFER {
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
            TXFER::TXFE_0 => false,
            TXFER::TXFE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXFER {
        match value {
            false => TXFER::TXFE_0,
            true => TXFER::TXFE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXFE_0`"]
    #[inline]
    pub fn is_txfe_0(&self) -> bool {
        *self == TXFER::TXFE_0
    }
    #[doc = "Checks if the value of the field is `TXFE_1`"]
    #[inline]
    pub fn is_txfe_1(&self) -> bool {
        *self == TXFER::TXFE_1
    }
}
#[doc = "Possible values of the field `ADET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADETR {
    #[doc = "ASCII \"A\" or \"a\" was not received"]
    ADET_0,
    #[doc = "ASCII \"A\" or \"a\" was received (write 1 to clear)"]
    ADET_1,
}
impl ADETR {
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
            ADETR::ADET_0 => false,
            ADETR::ADET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADETR {
        match value {
            false => ADETR::ADET_0,
            true => ADETR::ADET_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADET_0`"]
    #[inline]
    pub fn is_adet_0(&self) -> bool {
        *self == ADETR::ADET_0
    }
    #[doc = "Checks if the value of the field is `ADET_1`"]
    #[inline]
    pub fn is_adet_1(&self) -> bool {
        *self == ADETR::ADET_1
    }
}
#[doc = "Values that can be written to the field `ORE`"]
pub enum OREW {
    #[doc = "No overrun error"]
    ORE_0,
    #[doc = "Overrun error (write 1 to clear)"]
    ORE_1,
}
impl OREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OREW::ORE_0 => false,
            OREW::ORE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OREW<'a> {
    w: &'a mut W,
}
impl<'a> _OREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No overrun error"]
    #[inline]
    pub fn ore_0(self) -> &'a mut W {
        self.variant(OREW::ORE_0)
    }
    #[doc = "Overrun error (write 1 to clear)"]
    #[inline]
    pub fn ore_1(self) -> &'a mut W {
        self.variant(OREW::ORE_1)
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
#[doc = "Values that can be written to the field `BRCD`"]
pub enum BRCDW {
    #[doc = "No BREAK condition was detected"]
    BRCD_0,
    #[doc = "A BREAK condition was detected (write 1 to clear)"]
    BRCD_1,
}
impl BRCDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BRCDW::BRCD_0 => false,
            BRCDW::BRCD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BRCDW<'a> {
    w: &'a mut W,
}
impl<'a> _BRCDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BRCDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No BREAK condition was detected"]
    #[inline]
    pub fn brcd_0(self) -> &'a mut W {
        self.variant(BRCDW::BRCD_0)
    }
    #[doc = "A BREAK condition was detected (write 1 to clear)"]
    #[inline]
    pub fn brcd_1(self) -> &'a mut W {
        self.variant(BRCDW::BRCD_1)
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
#[doc = "Values that can be written to the field `RTSF`"]
pub enum RTSFW {
    #[doc = "Programmed edge not detected on RTS_B"]
    RTSF_0,
    #[doc = "Programmed edge detected on RTS_B (write 1 to clear)"]
    RTSF_1,
}
impl RTSFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTSFW::RTSF_0 => false,
            RTSFW::RTSF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTSFW<'a> {
    w: &'a mut W,
}
impl<'a> _RTSFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTSFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Programmed edge not detected on RTS_B"]
    #[inline]
    pub fn rtsf_0(self) -> &'a mut W {
        self.variant(RTSFW::RTSF_0)
    }
    #[doc = "Programmed edge detected on RTS_B (write 1 to clear)"]
    #[inline]
    pub fn rtsf_1(self) -> &'a mut W {
        self.variant(RTSFW::RTSF_1)
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
#[doc = r" Proxy"]
pub struct _DCDDELTW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDDELTW<'a> {
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
#[doc = "Values that can be written to the field `WAKE`"]
pub enum WAKEW {
    #[doc = "start bit not detected"]
    WAKE_0,
    #[doc = "start bit detected (write 1 to clear)"]
    WAKE_1,
}
impl WAKEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEW::WAKE_0 => false,
            WAKEW::WAKE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "start bit not detected"]
    #[inline]
    pub fn wake_0(self) -> &'a mut W {
        self.variant(WAKEW::WAKE_0)
    }
    #[doc = "start bit detected (write 1 to clear)"]
    #[inline]
    pub fn wake_1(self) -> &'a mut W {
        self.variant(WAKEW::WAKE_1)
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
#[doc = "Values that can be written to the field `IRINT`"]
pub enum IRINTW {
    #[doc = "no edge detected"]
    IRINT_0,
    #[doc = "valid edge detected (write 1 to clear)"]
    IRINT_1,
}
impl IRINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRINTW::IRINT_0 => false,
            IRINTW::IRINT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRINTW<'a> {
    w: &'a mut W,
}
impl<'a> _IRINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no edge detected"]
    #[inline]
    pub fn irint_0(self) -> &'a mut W {
        self.variant(IRINTW::IRINT_0)
    }
    #[doc = "valid edge detected (write 1 to clear)"]
    #[inline]
    pub fn irint_1(self) -> &'a mut W {
        self.variant(IRINTW::IRINT_1)
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
#[doc = r" Proxy"]
pub struct _RIDELTW<'a> {
    w: &'a mut W,
}
impl<'a> _RIDELTW<'a> {
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
#[doc = "Values that can be written to the field `ACST`"]
pub enum ACSTW {
    #[doc = "Measurement of bit length not finished (in autobaud)"]
    ACST_0,
    #[doc = "Measurement of bit length finished (in autobaud). (write 1 to clear)"]
    ACST_1,
}
impl ACSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACSTW::ACST_0 => false,
            ACSTW::ACST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACSTW<'a> {
    w: &'a mut W,
}
impl<'a> _ACSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Measurement of bit length not finished (in autobaud)"]
    #[inline]
    pub fn acst_0(self) -> &'a mut W {
        self.variant(ACSTW::ACST_0)
    }
    #[doc = "Measurement of bit length finished (in autobaud). (write 1 to clear)"]
    #[inline]
    pub fn acst_1(self) -> &'a mut W {
        self.variant(ACSTW::ACST_1)
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
#[doc = "Values that can be written to the field `IDLE`"]
pub enum IDLEW {
    #[doc = "No idle condition detected"]
    IDLE_0,
    #[doc = "Idle condition detected (write 1 to clear)"]
    IDLE_1,
}
impl IDLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IDLEW::IDLE_0 => false,
            IDLEW::IDLE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDLEW<'a> {
    w: &'a mut W,
}
impl<'a> _IDLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No idle condition detected"]
    #[inline]
    pub fn idle_0(self) -> &'a mut W {
        self.variant(IDLEW::IDLE_0)
    }
    #[doc = "Idle condition detected (write 1 to clear)"]
    #[inline]
    pub fn idle_1(self) -> &'a mut W {
        self.variant(IDLEW::IDLE_1)
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
pub struct _DTRFW<'a> {
    w: &'a mut W,
}
impl<'a> _DTRFW<'a> {
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
#[doc = "Values that can be written to the field `ADET`"]
pub enum ADETW {
    #[doc = "ASCII \"A\" or \"a\" was not received"]
    ADET_0,
    #[doc = "ASCII \"A\" or \"a\" was received (write 1 to clear)"]
    ADET_1,
}
impl ADETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADETW::ADET_0 => false,
            ADETW::ADET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADETW<'a> {
    w: &'a mut W,
}
impl<'a> _ADETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ASCII \"A\" or \"a\" was not received"]
    #[inline]
    pub fn adet_0(self) -> &'a mut W {
        self.variant(ADETW::ADET_0)
    }
    #[doc = "ASCII \"A\" or \"a\" was received (write 1 to clear)"]
    #[inline]
    pub fn adet_1(self) -> &'a mut W {
        self.variant(ADETW::ADET_1)
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
    #[doc = "Bit 0 - Receive Data Ready-Indicates that at least 1 character is received and written to the RxFIFO"]
    #[inline]
    pub fn rdr(&self) -> RDRR {
        RDRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Overrun Error"]
    #[inline]
    pub fn ore(&self) -> ORER {
        ORER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - BREAK Condition Detected"]
    #[inline]
    pub fn brcd(&self) -> BRCDR {
        BRCDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Transmitter Complete"]
    #[inline]
    pub fn txdc(&self) -> TXDCR {
        TXDCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - RTS Edge Triggered Interrupt Flag"]
    #[inline]
    pub fn rtsf(&self) -> RTSFR {
        RTSFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - This bit is not used in this chip."]
    #[inline]
    pub fn dcdin(&self) -> DCDINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDINR { bits }
    }
    #[doc = "Bit 6 - This bit is not used in this chip."]
    #[inline]
    pub fn dcddelt(&self) -> DCDDELTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDDELTR { bits }
    }
    #[doc = "Bit 7 - Wake"]
    #[inline]
    pub fn wake(&self) -> WAKER {
        WAKER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Serial Infrared Interrupt Flag"]
    #[inline]
    pub fn irint(&self) -> IRINTR {
        IRINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - This bit is not used in this chip."]
    #[inline]
    pub fn riin(&self) -> RIINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RIINR { bits }
    }
    #[doc = "Bit 10 - This bit is not used in this chip."]
    #[inline]
    pub fn ridelt(&self) -> RIDELTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RIDELTR { bits }
    }
    #[doc = "Bit 11 - Autobaud Counter Stopped"]
    #[inline]
    pub fn acst(&self) -> ACSTR {
        ACSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Idle Condition"]
    #[inline]
    pub fn idle(&self) -> IDLER {
        IDLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - This bit is not used in this chip."]
    #[inline]
    pub fn dtrf(&self) -> DTRFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTRFR { bits }
    }
    #[doc = "Bit 14 - Transmit Buffer FIFO Empty"]
    #[inline]
    pub fn txfe(&self) -> TXFER {
        TXFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Automatic Baud Rate Detect Complete"]
    #[inline]
    pub fn adet(&self) -> ADETR {
        ADETR::_from({
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
        W { bits: 16424 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Overrun Error"]
    #[inline]
    pub fn ore(&mut self) -> _OREW {
        _OREW { w: self }
    }
    #[doc = "Bit 2 - BREAK Condition Detected"]
    #[inline]
    pub fn brcd(&mut self) -> _BRCDW {
        _BRCDW { w: self }
    }
    #[doc = "Bit 4 - RTS Edge Triggered Interrupt Flag"]
    #[inline]
    pub fn rtsf(&mut self) -> _RTSFW {
        _RTSFW { w: self }
    }
    #[doc = "Bit 6 - This bit is not used in this chip."]
    #[inline]
    pub fn dcddelt(&mut self) -> _DCDDELTW {
        _DCDDELTW { w: self }
    }
    #[doc = "Bit 7 - Wake"]
    #[inline]
    pub fn wake(&mut self) -> _WAKEW {
        _WAKEW { w: self }
    }
    #[doc = "Bit 8 - Serial Infrared Interrupt Flag"]
    #[inline]
    pub fn irint(&mut self) -> _IRINTW {
        _IRINTW { w: self }
    }
    #[doc = "Bit 10 - This bit is not used in this chip."]
    #[inline]
    pub fn ridelt(&mut self) -> _RIDELTW {
        _RIDELTW { w: self }
    }
    #[doc = "Bit 11 - Autobaud Counter Stopped"]
    #[inline]
    pub fn acst(&mut self) -> _ACSTW {
        _ACSTW { w: self }
    }
    #[doc = "Bit 12 - Idle Condition"]
    #[inline]
    pub fn idle(&mut self) -> _IDLEW {
        _IDLEW { w: self }
    }
    #[doc = "Bit 13 - This bit is not used in this chip."]
    #[inline]
    pub fn dtrf(&mut self) -> _DTRFW {
        _DTRFW { w: self }
    }
    #[doc = "Bit 15 - Automatic Baud Rate Detect Complete"]
    #[inline]
    pub fn adet(&mut self) -> _ADETW {
        _ADETW { w: self }
    }
}
