#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::UCR2 {
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
#[doc = "Possible values of the field `SRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRSTR {
    #[doc = "Reset the transmit and receive state machines, all FIFOs and register USR1, USR2, UBIR, UBMR, UBRC , URXD, UTXD and UTS\\[6-3\\]."]
    SRST_0,
    #[doc = "No reset"]
    SRST_1,
}
impl SRSTR {
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
            SRSTR::SRST_0 => false,
            SRSTR::SRST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRSTR {
        match value {
            false => SRSTR::SRST_0,
            true => SRSTR::SRST_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRST_0`"]
    #[inline]
    pub fn is_srst_0(&self) -> bool {
        *self == SRSTR::SRST_0
    }
    #[doc = "Checks if the value of the field is `SRST_1`"]
    #[inline]
    pub fn is_srst_1(&self) -> bool {
        *self == SRSTR::SRST_1
    }
}
#[doc = "Possible values of the field `RXEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXENR {
    #[doc = "Disable the receiver"]
    RXEN_0,
    #[doc = "Enable the receiver"]
    RXEN_1,
}
impl RXENR {
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
            RXENR::RXEN_0 => false,
            RXENR::RXEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXENR {
        match value {
            false => RXENR::RXEN_0,
            true => RXENR::RXEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXEN_0`"]
    #[inline]
    pub fn is_rxen_0(&self) -> bool {
        *self == RXENR::RXEN_0
    }
    #[doc = "Checks if the value of the field is `RXEN_1`"]
    #[inline]
    pub fn is_rxen_1(&self) -> bool {
        *self == RXENR::RXEN_1
    }
}
#[doc = "Possible values of the field `TXEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXENR {
    #[doc = "Disable the transmitter"]
    TXEN_0,
    #[doc = "Enable the transmitter"]
    TXEN_1,
}
impl TXENR {
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
            TXENR::TXEN_0 => false,
            TXENR::TXEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXENR {
        match value {
            false => TXENR::TXEN_0,
            true => TXENR::TXEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXEN_0`"]
    #[inline]
    pub fn is_txen_0(&self) -> bool {
        *self == TXENR::TXEN_0
    }
    #[doc = "Checks if the value of the field is `TXEN_1`"]
    #[inline]
    pub fn is_txen_1(&self) -> bool {
        *self == TXENR::TXEN_1
    }
}
#[doc = "Possible values of the field `ATEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATENR {
    #[doc = "AGTIM interrupt disabled"]
    ATEN_0,
    #[doc = "AGTIM interrupt enabled"]
    ATEN_1,
}
impl ATENR {
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
            ATENR::ATEN_0 => false,
            ATENR::ATEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ATENR {
        match value {
            false => ATENR::ATEN_0,
            true => ATENR::ATEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ATEN_0`"]
    #[inline]
    pub fn is_aten_0(&self) -> bool {
        *self == ATENR::ATEN_0
    }
    #[doc = "Checks if the value of the field is `ATEN_1`"]
    #[inline]
    pub fn is_aten_1(&self) -> bool {
        *self == ATENR::ATEN_1
    }
}
#[doc = "Possible values of the field `RTSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSENR {
    #[doc = "Disable request to send interrupt"]
    RTSEN_0,
    #[doc = "Enable request to send interrupt"]
    RTSEN_1,
}
impl RTSENR {
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
            RTSENR::RTSEN_0 => false,
            RTSENR::RTSEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTSENR {
        match value {
            false => RTSENR::RTSEN_0,
            true => RTSENR::RTSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTSEN_0`"]
    #[inline]
    pub fn is_rtsen_0(&self) -> bool {
        *self == RTSENR::RTSEN_0
    }
    #[doc = "Checks if the value of the field is `RTSEN_1`"]
    #[inline]
    pub fn is_rtsen_1(&self) -> bool {
        *self == RTSENR::RTSEN_1
    }
}
#[doc = "Possible values of the field `WS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WSR {
    #[doc = "7-bit transmit and receive character length (not including START, STOP or PARITY bits)"]
    WS_0,
    #[doc = "8-bit transmit and receive character length (not including START, STOP or PARITY bits)"]
    WS_1,
}
impl WSR {
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
            WSR::WS_0 => false,
            WSR::WS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WSR {
        match value {
            false => WSR::WS_0,
            true => WSR::WS_1,
        }
    }
    #[doc = "Checks if the value of the field is `WS_0`"]
    #[inline]
    pub fn is_ws_0(&self) -> bool {
        *self == WSR::WS_0
    }
    #[doc = "Checks if the value of the field is `WS_1`"]
    #[inline]
    pub fn is_ws_1(&self) -> bool {
        *self == WSR::WS_1
    }
}
#[doc = "Possible values of the field `STPB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPBR {
    #[doc = "The transmitter sends 1 stop bit. The receiver expects 1 or more stop bits."]
    STPB_0,
    #[doc = "The transmitter sends 2 stop bits. The receiver expects 2 or more stop bits."]
    STPB_1,
}
impl STPBR {
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
            STPBR::STPB_0 => false,
            STPBR::STPB_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STPBR {
        match value {
            false => STPBR::STPB_0,
            true => STPBR::STPB_1,
        }
    }
    #[doc = "Checks if the value of the field is `STPB_0`"]
    #[inline]
    pub fn is_stpb_0(&self) -> bool {
        *self == STPBR::STPB_0
    }
    #[doc = "Checks if the value of the field is `STPB_1`"]
    #[inline]
    pub fn is_stpb_1(&self) -> bool {
        *self == STPBR::STPB_1
    }
}
#[doc = "Possible values of the field `PROE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROER {
    #[doc = "Even parity"]
    PROE_0,
    #[doc = "Odd parity"]
    PROE_1,
}
impl PROER {
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
            PROER::PROE_0 => false,
            PROER::PROE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PROER {
        match value {
            false => PROER::PROE_0,
            true => PROER::PROE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PROE_0`"]
    #[inline]
    pub fn is_proe_0(&self) -> bool {
        *self == PROER::PROE_0
    }
    #[doc = "Checks if the value of the field is `PROE_1`"]
    #[inline]
    pub fn is_proe_1(&self) -> bool {
        *self == PROER::PROE_1
    }
}
#[doc = "Possible values of the field `PREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRENR {
    #[doc = "Disable parity generator and checker"]
    PREN_0,
    #[doc = "Enable parity generator and checker"]
    PREN_1,
}
impl PRENR {
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
            PRENR::PREN_0 => false,
            PRENR::PREN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRENR {
        match value {
            false => PRENR::PREN_0,
            true => PRENR::PREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `PREN_0`"]
    #[inline]
    pub fn is_pren_0(&self) -> bool {
        *self == PRENR::PREN_0
    }
    #[doc = "Checks if the value of the field is `PREN_1`"]
    #[inline]
    pub fn is_pren_1(&self) -> bool {
        *self == PRENR::PREN_1
    }
}
#[doc = "Possible values of the field `RTEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTECR {
    #[doc = "Trigger interrupt on a rising edge"]
    RTEC_0,
    #[doc = "Trigger interrupt on a falling edge"]
    RTEC_1,
    #[doc = "Trigger interrupt on any edge"]
    RTEC_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RTECR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RTECR::RTEC_0 => 0,
            RTECR::RTEC_1 => 1,
            RTECR::RTEC_2 => 2,
            RTECR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RTECR {
        match value {
            0 => RTECR::RTEC_0,
            1 => RTECR::RTEC_1,
            2 => RTECR::RTEC_2,
            i => RTECR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RTEC_0`"]
    #[inline]
    pub fn is_rtec_0(&self) -> bool {
        *self == RTECR::RTEC_0
    }
    #[doc = "Checks if the value of the field is `RTEC_1`"]
    #[inline]
    pub fn is_rtec_1(&self) -> bool {
        *self == RTECR::RTEC_1
    }
    #[doc = "Checks if the value of the field is `RTEC_2`"]
    #[inline]
    pub fn is_rtec_2(&self) -> bool {
        *self == RTECR::RTEC_2
    }
}
#[doc = "Possible values of the field `ESCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESCENR {
    #[doc = "Disable escape sequence detection"]
    ESCEN_0,
    #[doc = "Enable escape sequence detection"]
    ESCEN_1,
}
impl ESCENR {
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
            ESCENR::ESCEN_0 => false,
            ESCENR::ESCEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ESCENR {
        match value {
            false => ESCENR::ESCEN_0,
            true => ESCENR::ESCEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ESCEN_0`"]
    #[inline]
    pub fn is_escen_0(&self) -> bool {
        *self == ESCENR::ESCEN_0
    }
    #[doc = "Checks if the value of the field is `ESCEN_1`"]
    #[inline]
    pub fn is_escen_1(&self) -> bool {
        *self == ESCENR::ESCEN_1
    }
}
#[doc = "Possible values of the field `CTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSR {
    #[doc = "The CTS_B pin is high (inactive)"]
    CTS_0,
    #[doc = "The CTS_B pin is low (active)"]
    CTS_1,
}
impl CTSR {
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
            CTSR::CTS_0 => false,
            CTSR::CTS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTSR {
        match value {
            false => CTSR::CTS_0,
            true => CTSR::CTS_1,
        }
    }
    #[doc = "Checks if the value of the field is `CTS_0`"]
    #[inline]
    pub fn is_cts_0(&self) -> bool {
        *self == CTSR::CTS_0
    }
    #[doc = "Checks if the value of the field is `CTS_1`"]
    #[inline]
    pub fn is_cts_1(&self) -> bool {
        *self == CTSR::CTS_1
    }
}
#[doc = "Possible values of the field `CTSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSCR {
    #[doc = "The CTS_B pin is controlled by the CTS bit"]
    CTSC_0,
    #[doc = "The CTS_B pin is controlled by the receiver"]
    CTSC_1,
}
impl CTSCR {
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
            CTSCR::CTSC_0 => false,
            CTSCR::CTSC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTSCR {
        match value {
            false => CTSCR::CTSC_0,
            true => CTSCR::CTSC_1,
        }
    }
    #[doc = "Checks if the value of the field is `CTSC_0`"]
    #[inline]
    pub fn is_ctsc_0(&self) -> bool {
        *self == CTSCR::CTSC_0
    }
    #[doc = "Checks if the value of the field is `CTSC_1`"]
    #[inline]
    pub fn is_ctsc_1(&self) -> bool {
        *self == CTSCR::CTSC_1
    }
}
#[doc = "Possible values of the field `IRTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRTSR {
    #[doc = "Transmit only when the RTS pin is asserted"]
    IRTS_0,
    #[doc = "Ignore the RTS pin"]
    IRTS_1,
}
impl IRTSR {
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
            IRTSR::IRTS_0 => false,
            IRTSR::IRTS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRTSR {
        match value {
            false => IRTSR::IRTS_0,
            true => IRTSR::IRTS_1,
        }
    }
    #[doc = "Checks if the value of the field is `IRTS_0`"]
    #[inline]
    pub fn is_irts_0(&self) -> bool {
        *self == IRTSR::IRTS_0
    }
    #[doc = "Checks if the value of the field is `IRTS_1`"]
    #[inline]
    pub fn is_irts_1(&self) -> bool {
        *self == IRTSR::IRTS_1
    }
}
#[doc = "Possible values of the field `ESCI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESCIR {
    #[doc = "Disable the escape sequence interrupt"]
    ESCI_0,
    #[doc = "Enable the escape sequence interrupt"]
    ESCI_1,
}
impl ESCIR {
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
            ESCIR::ESCI_0 => false,
            ESCIR::ESCI_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ESCIR {
        match value {
            false => ESCIR::ESCI_0,
            true => ESCIR::ESCI_1,
        }
    }
    #[doc = "Checks if the value of the field is `ESCI_0`"]
    #[inline]
    pub fn is_esci_0(&self) -> bool {
        *self == ESCIR::ESCI_0
    }
    #[doc = "Checks if the value of the field is `ESCI_1`"]
    #[inline]
    pub fn is_esci_1(&self) -> bool {
        *self == ESCIR::ESCI_1
    }
}
#[doc = "Values that can be written to the field `SRST`"]
pub enum SRSTW {
    #[doc = "Reset the transmit and receive state machines, all FIFOs and register USR1, USR2, UBIR, UBMR, UBRC , URXD, UTXD and UTS\\[6-3\\]."]
    SRST_0,
    #[doc = "No reset"]
    SRST_1,
}
impl SRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRSTW::SRST_0 => false,
            SRSTW::SRST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the transmit and receive state machines, all FIFOs and register USR1, USR2, UBIR, UBMR, UBRC , URXD, UTXD and UTS\\[6-3\\]."]
    #[inline]
    pub fn srst_0(self) -> &'a mut W {
        self.variant(SRSTW::SRST_0)
    }
    #[doc = "No reset"]
    #[inline]
    pub fn srst_1(self) -> &'a mut W {
        self.variant(SRSTW::SRST_1)
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
#[doc = "Values that can be written to the field `RXEN`"]
pub enum RXENW {
    #[doc = "Disable the receiver"]
    RXEN_0,
    #[doc = "Enable the receiver"]
    RXEN_1,
}
impl RXENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXENW::RXEN_0 => false,
            RXENW::RXEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the receiver"]
    #[inline]
    pub fn rxen_0(self) -> &'a mut W {
        self.variant(RXENW::RXEN_0)
    }
    #[doc = "Enable the receiver"]
    #[inline]
    pub fn rxen_1(self) -> &'a mut W {
        self.variant(RXENW::RXEN_1)
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
#[doc = "Values that can be written to the field `TXEN`"]
pub enum TXENW {
    #[doc = "Disable the transmitter"]
    TXEN_0,
    #[doc = "Enable the transmitter"]
    TXEN_1,
}
impl TXENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXENW::TXEN_0 => false,
            TXENW::TXEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the transmitter"]
    #[inline]
    pub fn txen_0(self) -> &'a mut W {
        self.variant(TXENW::TXEN_0)
    }
    #[doc = "Enable the transmitter"]
    #[inline]
    pub fn txen_1(self) -> &'a mut W {
        self.variant(TXENW::TXEN_1)
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
#[doc = "Values that can be written to the field `ATEN`"]
pub enum ATENW {
    #[doc = "AGTIM interrupt disabled"]
    ATEN_0,
    #[doc = "AGTIM interrupt enabled"]
    ATEN_1,
}
impl ATENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ATENW::ATEN_0 => false,
            ATENW::ATEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ATENW<'a> {
    w: &'a mut W,
}
impl<'a> _ATENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ATENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "AGTIM interrupt disabled"]
    #[inline]
    pub fn aten_0(self) -> &'a mut W {
        self.variant(ATENW::ATEN_0)
    }
    #[doc = "AGTIM interrupt enabled"]
    #[inline]
    pub fn aten_1(self) -> &'a mut W {
        self.variant(ATENW::ATEN_1)
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
#[doc = "Values that can be written to the field `RTSEN`"]
pub enum RTSENW {
    #[doc = "Disable request to send interrupt"]
    RTSEN_0,
    #[doc = "Enable request to send interrupt"]
    RTSEN_1,
}
impl RTSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTSENW::RTSEN_0 => false,
            RTSENW::RTSEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTSENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable request to send interrupt"]
    #[inline]
    pub fn rtsen_0(self) -> &'a mut W {
        self.variant(RTSENW::RTSEN_0)
    }
    #[doc = "Enable request to send interrupt"]
    #[inline]
    pub fn rtsen_1(self) -> &'a mut W {
        self.variant(RTSENW::RTSEN_1)
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
#[doc = "Values that can be written to the field `WS`"]
pub enum WSW {
    #[doc = "7-bit transmit and receive character length (not including START, STOP or PARITY bits)"]
    WS_0,
    #[doc = "8-bit transmit and receive character length (not including START, STOP or PARITY bits)"]
    WS_1,
}
impl WSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WSW::WS_0 => false,
            WSW::WS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WSW<'a> {
    w: &'a mut W,
}
impl<'a> _WSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "7-bit transmit and receive character length (not including START, STOP or PARITY bits)"]
    #[inline]
    pub fn ws_0(self) -> &'a mut W {
        self.variant(WSW::WS_0)
    }
    #[doc = "8-bit transmit and receive character length (not including START, STOP or PARITY bits)"]
    #[inline]
    pub fn ws_1(self) -> &'a mut W {
        self.variant(WSW::WS_1)
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
#[doc = "Values that can be written to the field `STPB`"]
pub enum STPBW {
    #[doc = "The transmitter sends 1 stop bit. The receiver expects 1 or more stop bits."]
    STPB_0,
    #[doc = "The transmitter sends 2 stop bits. The receiver expects 2 or more stop bits."]
    STPB_1,
}
impl STPBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STPBW::STPB_0 => false,
            STPBW::STPB_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STPBW<'a> {
    w: &'a mut W,
}
impl<'a> _STPBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STPBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The transmitter sends 1 stop bit. The receiver expects 1 or more stop bits."]
    #[inline]
    pub fn stpb_0(self) -> &'a mut W {
        self.variant(STPBW::STPB_0)
    }
    #[doc = "The transmitter sends 2 stop bits. The receiver expects 2 or more stop bits."]
    #[inline]
    pub fn stpb_1(self) -> &'a mut W {
        self.variant(STPBW::STPB_1)
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
#[doc = "Values that can be written to the field `PROE`"]
pub enum PROEW {
    #[doc = "Even parity"]
    PROE_0,
    #[doc = "Odd parity"]
    PROE_1,
}
impl PROEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PROEW::PROE_0 => false,
            PROEW::PROE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PROEW<'a> {
    w: &'a mut W,
}
impl<'a> _PROEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PROEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Even parity"]
    #[inline]
    pub fn proe_0(self) -> &'a mut W {
        self.variant(PROEW::PROE_0)
    }
    #[doc = "Odd parity"]
    #[inline]
    pub fn proe_1(self) -> &'a mut W {
        self.variant(PROEW::PROE_1)
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
#[doc = "Values that can be written to the field `PREN`"]
pub enum PRENW {
    #[doc = "Disable parity generator and checker"]
    PREN_0,
    #[doc = "Enable parity generator and checker"]
    PREN_1,
}
impl PRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRENW::PREN_0 => false,
            PRENW::PREN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRENW<'a> {
    w: &'a mut W,
}
impl<'a> _PRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable parity generator and checker"]
    #[inline]
    pub fn pren_0(self) -> &'a mut W {
        self.variant(PRENW::PREN_0)
    }
    #[doc = "Enable parity generator and checker"]
    #[inline]
    pub fn pren_1(self) -> &'a mut W {
        self.variant(PRENW::PREN_1)
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
#[doc = "Values that can be written to the field `RTEC`"]
pub enum RTECW {
    #[doc = "Trigger interrupt on a rising edge"]
    RTEC_0,
    #[doc = "Trigger interrupt on a falling edge"]
    RTEC_1,
    #[doc = "Trigger interrupt on any edge"]
    RTEC_2,
}
impl RTECW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RTECW::RTEC_0 => 0,
            RTECW::RTEC_1 => 1,
            RTECW::RTEC_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTECW<'a> {
    w: &'a mut W,
}
impl<'a> _RTECW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTECW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Trigger interrupt on a rising edge"]
    #[inline]
    pub fn rtec_0(self) -> &'a mut W {
        self.variant(RTECW::RTEC_0)
    }
    #[doc = "Trigger interrupt on a falling edge"]
    #[inline]
    pub fn rtec_1(self) -> &'a mut W {
        self.variant(RTECW::RTEC_1)
    }
    #[doc = "Trigger interrupt on any edge"]
    #[inline]
    pub fn rtec_2(self) -> &'a mut W {
        self.variant(RTECW::RTEC_2)
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
#[doc = "Values that can be written to the field `ESCEN`"]
pub enum ESCENW {
    #[doc = "Disable escape sequence detection"]
    ESCEN_0,
    #[doc = "Enable escape sequence detection"]
    ESCEN_1,
}
impl ESCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ESCENW::ESCEN_0 => false,
            ESCENW::ESCEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ESCENW<'a> {
    w: &'a mut W,
}
impl<'a> _ESCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ESCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable escape sequence detection"]
    #[inline]
    pub fn escen_0(self) -> &'a mut W {
        self.variant(ESCENW::ESCEN_0)
    }
    #[doc = "Enable escape sequence detection"]
    #[inline]
    pub fn escen_1(self) -> &'a mut W {
        self.variant(ESCENW::ESCEN_1)
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
#[doc = "Values that can be written to the field `CTS`"]
pub enum CTSW {
    #[doc = "The CTS_B pin is high (inactive)"]
    CTS_0,
    #[doc = "The CTS_B pin is low (active)"]
    CTS_1,
}
impl CTSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTSW::CTS_0 => false,
            CTSW::CTS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTSW<'a> {
    w: &'a mut W,
}
impl<'a> _CTSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The CTS_B pin is high (inactive)"]
    #[inline]
    pub fn cts_0(self) -> &'a mut W {
        self.variant(CTSW::CTS_0)
    }
    #[doc = "The CTS_B pin is low (active)"]
    #[inline]
    pub fn cts_1(self) -> &'a mut W {
        self.variant(CTSW::CTS_1)
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
#[doc = "Values that can be written to the field `CTSC`"]
pub enum CTSCW {
    #[doc = "The CTS_B pin is controlled by the CTS bit"]
    CTSC_0,
    #[doc = "The CTS_B pin is controlled by the receiver"]
    CTSC_1,
}
impl CTSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTSCW::CTSC_0 => false,
            CTSCW::CTSC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTSCW<'a> {
    w: &'a mut W,
}
impl<'a> _CTSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTSCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The CTS_B pin is controlled by the CTS bit"]
    #[inline]
    pub fn ctsc_0(self) -> &'a mut W {
        self.variant(CTSCW::CTSC_0)
    }
    #[doc = "The CTS_B pin is controlled by the receiver"]
    #[inline]
    pub fn ctsc_1(self) -> &'a mut W {
        self.variant(CTSCW::CTSC_1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IRTS`"]
pub enum IRTSW {
    #[doc = "Transmit only when the RTS pin is asserted"]
    IRTS_0,
    #[doc = "Ignore the RTS pin"]
    IRTS_1,
}
impl IRTSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRTSW::IRTS_0 => false,
            IRTSW::IRTS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRTSW<'a> {
    w: &'a mut W,
}
impl<'a> _IRTSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRTSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit only when the RTS pin is asserted"]
    #[inline]
    pub fn irts_0(self) -> &'a mut W {
        self.variant(IRTSW::IRTS_0)
    }
    #[doc = "Ignore the RTS pin"]
    #[inline]
    pub fn irts_1(self) -> &'a mut W {
        self.variant(IRTSW::IRTS_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ESCI`"]
pub enum ESCIW {
    #[doc = "Disable the escape sequence interrupt"]
    ESCI_0,
    #[doc = "Enable the escape sequence interrupt"]
    ESCI_1,
}
impl ESCIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ESCIW::ESCI_0 => false,
            ESCIW::ESCI_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ESCIW<'a> {
    w: &'a mut W,
}
impl<'a> _ESCIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ESCIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the escape sequence interrupt"]
    #[inline]
    pub fn esci_0(self) -> &'a mut W {
        self.variant(ESCIW::ESCI_0)
    }
    #[doc = "Enable the escape sequence interrupt"]
    #[inline]
    pub fn esci_1(self) -> &'a mut W {
        self.variant(ESCIW::ESCI_1)
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
    #[doc = "Bit 0 - Software Reset"]
    #[inline]
    pub fn srst(&self) -> SRSTR {
        SRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Receiver Enable"]
    #[inline]
    pub fn rxen(&self) -> RXENR {
        RXENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Transmitter Enable"]
    #[inline]
    pub fn txen(&self) -> TXENR {
        TXENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Aging Timer Enable. This bit is used to enable the aging timer interrupt (triggered with AGTIM)"]
    #[inline]
    pub fn aten(&self) -> ATENR {
        ATENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Request to Send Interrupt Enable"]
    #[inline]
    pub fn rtsen(&self) -> RTSENR {
        RTSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Word Size"]
    #[inline]
    pub fn ws(&self) -> WSR {
        WSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Stop"]
    #[inline]
    pub fn stpb(&self) -> STPBR {
        STPBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Parity Odd/Even"]
    #[inline]
    pub fn proe(&self) -> PROER {
        PROER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Parity Enable"]
    #[inline]
    pub fn pren(&self) -> PRENR {
        PRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 9:10 - Request to Send Edge Control"]
    #[inline]
    pub fn rtec(&self) -> RTECR {
        RTECR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Escape Enable. Enables/Disables the escape sequence detection logic."]
    #[inline]
    pub fn escen(&self) -> ESCENR {
        ESCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Clear to Send"]
    #[inline]
    pub fn cts(&self) -> CTSR {
        CTSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - CTS Pin Control"]
    #[inline]
    pub fn ctsc(&self) -> CTSCR {
        CTSCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Ignore RTS Pin"]
    #[inline]
    pub fn irts(&self) -> IRTSR {
        IRTSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Escape Sequence Interrupt Enable. Enables/Disables the ESCF bit to generate an interrupt."]
    #[inline]
    pub fn esci(&self) -> ESCIR {
        ESCIR::_from({
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
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Software Reset"]
    #[inline]
    pub fn srst(&mut self) -> _SRSTW {
        _SRSTW { w: self }
    }
    #[doc = "Bit 1 - Receiver Enable"]
    #[inline]
    pub fn rxen(&mut self) -> _RXENW {
        _RXENW { w: self }
    }
    #[doc = "Bit 2 - Transmitter Enable"]
    #[inline]
    pub fn txen(&mut self) -> _TXENW {
        _TXENW { w: self }
    }
    #[doc = "Bit 3 - Aging Timer Enable. This bit is used to enable the aging timer interrupt (triggered with AGTIM)"]
    #[inline]
    pub fn aten(&mut self) -> _ATENW {
        _ATENW { w: self }
    }
    #[doc = "Bit 4 - Request to Send Interrupt Enable"]
    #[inline]
    pub fn rtsen(&mut self) -> _RTSENW {
        _RTSENW { w: self }
    }
    #[doc = "Bit 5 - Word Size"]
    #[inline]
    pub fn ws(&mut self) -> _WSW {
        _WSW { w: self }
    }
    #[doc = "Bit 6 - Stop"]
    #[inline]
    pub fn stpb(&mut self) -> _STPBW {
        _STPBW { w: self }
    }
    #[doc = "Bit 7 - Parity Odd/Even"]
    #[inline]
    pub fn proe(&mut self) -> _PROEW {
        _PROEW { w: self }
    }
    #[doc = "Bit 8 - Parity Enable"]
    #[inline]
    pub fn pren(&mut self) -> _PRENW {
        _PRENW { w: self }
    }
    #[doc = "Bits 9:10 - Request to Send Edge Control"]
    #[inline]
    pub fn rtec(&mut self) -> _RTECW {
        _RTECW { w: self }
    }
    #[doc = "Bit 11 - Escape Enable. Enables/Disables the escape sequence detection logic."]
    #[inline]
    pub fn escen(&mut self) -> _ESCENW {
        _ESCENW { w: self }
    }
    #[doc = "Bit 12 - Clear to Send"]
    #[inline]
    pub fn cts(&mut self) -> _CTSW {
        _CTSW { w: self }
    }
    #[doc = "Bit 13 - CTS Pin Control"]
    #[inline]
    pub fn ctsc(&mut self) -> _CTSCW {
        _CTSCW { w: self }
    }
    #[doc = "Bit 14 - Ignore RTS Pin"]
    #[inline]
    pub fn irts(&mut self) -> _IRTSW {
        _IRTSW { w: self }
    }
    #[doc = "Bit 15 - Escape Sequence Interrupt Enable. Enables/Disables the ESCF bit to generate an interrupt."]
    #[inline]
    pub fn esci(&mut self) -> _ESCIW {
        _ESCIW { w: self }
    }
}
