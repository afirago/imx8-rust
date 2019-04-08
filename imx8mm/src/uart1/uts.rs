#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::UTS {
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
#[doc = "Possible values of the field `SOFTRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFTRSTR {
    #[doc = "Software reset inactive"]
    SOFTRST_0,
    #[doc = "Software reset active"]
    SOFTRST_1,
}
impl SOFTRSTR {
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
            SOFTRSTR::SOFTRST_0 => false,
            SOFTRSTR::SOFTRST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOFTRSTR {
        match value {
            false => SOFTRSTR::SOFTRST_0,
            true => SOFTRSTR::SOFTRST_1,
        }
    }
    #[doc = "Checks if the value of the field is `SOFTRST_0`"]
    #[inline]
    pub fn is_softrst_0(&self) -> bool {
        *self == SOFTRSTR::SOFTRST_0
    }
    #[doc = "Checks if the value of the field is `SOFTRST_1`"]
    #[inline]
    pub fn is_softrst_1(&self) -> bool {
        *self == SOFTRSTR::SOFTRST_1
    }
}
#[doc = "Possible values of the field `RXFULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFULLR {
    #[doc = "The RxFIFO is not full"]
    RXFULL_0,
    #[doc = "The RxFIFO is full"]
    RXFULL_1,
}
impl RXFULLR {
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
            RXFULLR::RXFULL_0 => false,
            RXFULLR::RXFULL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXFULLR {
        match value {
            false => RXFULLR::RXFULL_0,
            true => RXFULLR::RXFULL_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXFULL_0`"]
    #[inline]
    pub fn is_rxfull_0(&self) -> bool {
        *self == RXFULLR::RXFULL_0
    }
    #[doc = "Checks if the value of the field is `RXFULL_1`"]
    #[inline]
    pub fn is_rxfull_1(&self) -> bool {
        *self == RXFULLR::RXFULL_1
    }
}
#[doc = "Possible values of the field `TXFULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFULLR {
    #[doc = "The TxFIFO is not full"]
    TXFULL_0,
    #[doc = "The TxFIFO is full"]
    TXFULL_1,
}
impl TXFULLR {
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
            TXFULLR::TXFULL_0 => false,
            TXFULLR::TXFULL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXFULLR {
        match value {
            false => TXFULLR::TXFULL_0,
            true => TXFULLR::TXFULL_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXFULL_0`"]
    #[inline]
    pub fn is_txfull_0(&self) -> bool {
        *self == TXFULLR::TXFULL_0
    }
    #[doc = "Checks if the value of the field is `TXFULL_1`"]
    #[inline]
    pub fn is_txfull_1(&self) -> bool {
        *self == TXFULLR::TXFULL_1
    }
}
#[doc = "Possible values of the field `RXEMPTY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEMPTYR {
    #[doc = "The RxFIFO is not empty"]
    RXEMPTY_0,
    #[doc = "The RxFIFO is empty"]
    RXEMPTY_1,
}
impl RXEMPTYR {
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
            RXEMPTYR::RXEMPTY_0 => false,
            RXEMPTYR::RXEMPTY_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXEMPTYR {
        match value {
            false => RXEMPTYR::RXEMPTY_0,
            true => RXEMPTYR::RXEMPTY_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXEMPTY_0`"]
    #[inline]
    pub fn is_rxempty_0(&self) -> bool {
        *self == RXEMPTYR::RXEMPTY_0
    }
    #[doc = "Checks if the value of the field is `RXEMPTY_1`"]
    #[inline]
    pub fn is_rxempty_1(&self) -> bool {
        *self == RXEMPTYR::RXEMPTY_1
    }
}
#[doc = "Possible values of the field `TXEMPTY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEMPTYR {
    #[doc = "The TxFIFO is not empty"]
    TXEMPTY_0,
    #[doc = "The TxFIFO is empty"]
    TXEMPTY_1,
}
impl TXEMPTYR {
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
            TXEMPTYR::TXEMPTY_0 => false,
            TXEMPTYR::TXEMPTY_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXEMPTYR {
        match value {
            false => TXEMPTYR::TXEMPTY_0,
            true => TXEMPTYR::TXEMPTY_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXEMPTY_0`"]
    #[inline]
    pub fn is_txempty_0(&self) -> bool {
        *self == TXEMPTYR::TXEMPTY_0
    }
    #[doc = "Checks if the value of the field is `TXEMPTY_1`"]
    #[inline]
    pub fn is_txempty_1(&self) -> bool {
        *self == TXEMPTYR::TXEMPTY_1
    }
}
#[doc = "Possible values of the field `RXDBG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDBGR {
    #[doc = "rx fifo read pointer does not increment"]
    RXDBG_0,
    #[doc = "rx_fifo read pointer increments as normal"]
    RXDBG_1,
}
impl RXDBGR {
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
            RXDBGR::RXDBG_0 => false,
            RXDBGR::RXDBG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXDBGR {
        match value {
            false => RXDBGR::RXDBG_0,
            true => RXDBGR::RXDBG_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXDBG_0`"]
    #[inline]
    pub fn is_rxdbg_0(&self) -> bool {
        *self == RXDBGR::RXDBG_0
    }
    #[doc = "Checks if the value of the field is `RXDBG_1`"]
    #[inline]
    pub fn is_rxdbg_1(&self) -> bool {
        *self == RXDBGR::RXDBG_1
    }
}
#[doc = "Possible values of the field `LOOPIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPIRR {
    #[doc = "No IR loop"]
    LOOPIR_0,
    #[doc = "Connect IR transmitter to IR receiver"]
    LOOPIR_1,
}
impl LOOPIRR {
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
            LOOPIRR::LOOPIR_0 => false,
            LOOPIRR::LOOPIR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOOPIRR {
        match value {
            false => LOOPIRR::LOOPIR_0,
            true => LOOPIRR::LOOPIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOOPIR_0`"]
    #[inline]
    pub fn is_loopir_0(&self) -> bool {
        *self == LOOPIRR::LOOPIR_0
    }
    #[doc = "Checks if the value of the field is `LOOPIR_1`"]
    #[inline]
    pub fn is_loopir_1(&self) -> bool {
        *self == LOOPIRR::LOOPIR_1
    }
}
#[doc = "Possible values of the field `DBGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGENR {
    #[doc = "UART will go into debug mode when debug_req is HIGH"]
    DBGEN_0,
    #[doc = "UART will not go into debug mode even if debug_req is HIGH"]
    DBGEN_1,
}
impl DBGENR {
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
            DBGENR::DBGEN_0 => false,
            DBGENR::DBGEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBGENR {
        match value {
            false => DBGENR::DBGEN_0,
            true => DBGENR::DBGEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBGEN_0`"]
    #[inline]
    pub fn is_dbgen_0(&self) -> bool {
        *self == DBGENR::DBGEN_0
    }
    #[doc = "Checks if the value of the field is `DBGEN_1`"]
    #[inline]
    pub fn is_dbgen_1(&self) -> bool {
        *self == DBGENR::DBGEN_1
    }
}
#[doc = "Possible values of the field `LOOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPR {
    #[doc = "Normal receiver operation"]
    LOOP_0,
    #[doc = "Internally connect the transmitter output to the receiver input"]
    LOOP_1,
}
impl LOOPR {
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
            LOOPR::LOOP_0 => false,
            LOOPR::LOOP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOOPR {
        match value {
            false => LOOPR::LOOP_0,
            true => LOOPR::LOOP_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOOP_0`"]
    #[inline]
    pub fn is_loop_0(&self) -> bool {
        *self == LOOPR::LOOP_0
    }
    #[doc = "Checks if the value of the field is `LOOP_1`"]
    #[inline]
    pub fn is_loop_1(&self) -> bool {
        *self == LOOPR::LOOP_1
    }
}
#[doc = "Possible values of the field `FRCPERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRCPERRR {
    #[doc = "Generate normal parity"]
    FRCPERR_0,
    #[doc = "Generate inverted parity (error)"]
    FRCPERR_1,
}
impl FRCPERRR {
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
            FRCPERRR::FRCPERR_0 => false,
            FRCPERRR::FRCPERR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRCPERRR {
        match value {
            false => FRCPERRR::FRCPERR_0,
            true => FRCPERRR::FRCPERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRCPERR_0`"]
    #[inline]
    pub fn is_frcperr_0(&self) -> bool {
        *self == FRCPERRR::FRCPERR_0
    }
    #[doc = "Checks if the value of the field is `FRCPERR_1`"]
    #[inline]
    pub fn is_frcperr_1(&self) -> bool {
        *self == FRCPERRR::FRCPERR_1
    }
}
#[doc = "Values that can be written to the field `SOFTRST`"]
pub enum SOFTRSTW {
    #[doc = "Software reset inactive"]
    SOFTRST_0,
    #[doc = "Software reset active"]
    SOFTRST_1,
}
impl SOFTRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SOFTRSTW::SOFTRST_0 => false,
            SOFTRSTW::SOFTRST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOFTRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOFTRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Software reset inactive"]
    #[inline]
    pub fn softrst_0(self) -> &'a mut W {
        self.variant(SOFTRSTW::SOFTRST_0)
    }
    #[doc = "Software reset active"]
    #[inline]
    pub fn softrst_1(self) -> &'a mut W {
        self.variant(SOFTRSTW::SOFTRST_1)
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
#[doc = "Values that can be written to the field `RXFULL`"]
pub enum RXFULLW {
    #[doc = "The RxFIFO is not full"]
    RXFULL_0,
    #[doc = "The RxFIFO is full"]
    RXFULL_1,
}
impl RXFULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXFULLW::RXFULL_0 => false,
            RXFULLW::RXFULL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXFULLW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXFULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The RxFIFO is not full"]
    #[inline]
    pub fn rxfull_0(self) -> &'a mut W {
        self.variant(RXFULLW::RXFULL_0)
    }
    #[doc = "The RxFIFO is full"]
    #[inline]
    pub fn rxfull_1(self) -> &'a mut W {
        self.variant(RXFULLW::RXFULL_1)
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
#[doc = "Values that can be written to the field `TXFULL`"]
pub enum TXFULLW {
    #[doc = "The TxFIFO is not full"]
    TXFULL_0,
    #[doc = "The TxFIFO is full"]
    TXFULL_1,
}
impl TXFULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXFULLW::TXFULL_0 => false,
            TXFULLW::TXFULL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXFULLW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXFULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The TxFIFO is not full"]
    #[inline]
    pub fn txfull_0(self) -> &'a mut W {
        self.variant(TXFULLW::TXFULL_0)
    }
    #[doc = "The TxFIFO is full"]
    #[inline]
    pub fn txfull_1(self) -> &'a mut W {
        self.variant(TXFULLW::TXFULL_1)
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
#[doc = "Values that can be written to the field `RXEMPTY`"]
pub enum RXEMPTYW {
    #[doc = "The RxFIFO is not empty"]
    RXEMPTY_0,
    #[doc = "The RxFIFO is empty"]
    RXEMPTY_1,
}
impl RXEMPTYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXEMPTYW::RXEMPTY_0 => false,
            RXEMPTYW::RXEMPTY_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXEMPTYW<'a> {
    w: &'a mut W,
}
impl<'a> _RXEMPTYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXEMPTYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The RxFIFO is not empty"]
    #[inline]
    pub fn rxempty_0(self) -> &'a mut W {
        self.variant(RXEMPTYW::RXEMPTY_0)
    }
    #[doc = "The RxFIFO is empty"]
    #[inline]
    pub fn rxempty_1(self) -> &'a mut W {
        self.variant(RXEMPTYW::RXEMPTY_1)
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
#[doc = "Values that can be written to the field `TXEMPTY`"]
pub enum TXEMPTYW {
    #[doc = "The TxFIFO is not empty"]
    TXEMPTY_0,
    #[doc = "The TxFIFO is empty"]
    TXEMPTY_1,
}
impl TXEMPTYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXEMPTYW::TXEMPTY_0 => false,
            TXEMPTYW::TXEMPTY_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXEMPTYW<'a> {
    w: &'a mut W,
}
impl<'a> _TXEMPTYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXEMPTYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The TxFIFO is not empty"]
    #[inline]
    pub fn txempty_0(self) -> &'a mut W {
        self.variant(TXEMPTYW::TXEMPTY_0)
    }
    #[doc = "The TxFIFO is empty"]
    #[inline]
    pub fn txempty_1(self) -> &'a mut W {
        self.variant(TXEMPTYW::TXEMPTY_1)
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
#[doc = "Values that can be written to the field `RXDBG`"]
pub enum RXDBGW {
    #[doc = "rx fifo read pointer does not increment"]
    RXDBG_0,
    #[doc = "rx_fifo read pointer increments as normal"]
    RXDBG_1,
}
impl RXDBGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXDBGW::RXDBG_0 => false,
            RXDBGW::RXDBG_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXDBGW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDBGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXDBGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "rx fifo read pointer does not increment"]
    #[inline]
    pub fn rxdbg_0(self) -> &'a mut W {
        self.variant(RXDBGW::RXDBG_0)
    }
    #[doc = "rx_fifo read pointer increments as normal"]
    #[inline]
    pub fn rxdbg_1(self) -> &'a mut W {
        self.variant(RXDBGW::RXDBG_1)
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
#[doc = "Values that can be written to the field `LOOPIR`"]
pub enum LOOPIRW {
    #[doc = "No IR loop"]
    LOOPIR_0,
    #[doc = "Connect IR transmitter to IR receiver"]
    LOOPIR_1,
}
impl LOOPIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOOPIRW::LOOPIR_0 => false,
            LOOPIRW::LOOPIR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOOPIRW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOOPIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No IR loop"]
    #[inline]
    pub fn loopir_0(self) -> &'a mut W {
        self.variant(LOOPIRW::LOOPIR_0)
    }
    #[doc = "Connect IR transmitter to IR receiver"]
    #[inline]
    pub fn loopir_1(self) -> &'a mut W {
        self.variant(LOOPIRW::LOOPIR_1)
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
#[doc = "Values that can be written to the field `DBGEN`"]
pub enum DBGENW {
    #[doc = "UART will go into debug mode when debug_req is HIGH"]
    DBGEN_0,
    #[doc = "UART will not go into debug mode even if debug_req is HIGH"]
    DBGEN_1,
}
impl DBGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBGENW::DBGEN_0 => false,
            DBGENW::DBGEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBGENW<'a> {
    w: &'a mut W,
}
impl<'a> _DBGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "UART will go into debug mode when debug_req is HIGH"]
    #[inline]
    pub fn dbgen_0(self) -> &'a mut W {
        self.variant(DBGENW::DBGEN_0)
    }
    #[doc = "UART will not go into debug mode even if debug_req is HIGH"]
    #[inline]
    pub fn dbgen_1(self) -> &'a mut W {
        self.variant(DBGENW::DBGEN_1)
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
#[doc = "Values that can be written to the field `LOOP`"]
pub enum LOOPW {
    #[doc = "Normal receiver operation"]
    LOOP_0,
    #[doc = "Internally connect the transmitter output to the receiver input"]
    LOOP_1,
}
impl LOOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOOPW::LOOP_0 => false,
            LOOPW::LOOP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOOPW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal receiver operation"]
    #[inline]
    pub fn loop_0(self) -> &'a mut W {
        self.variant(LOOPW::LOOP_0)
    }
    #[doc = "Internally connect the transmitter output to the receiver input"]
    #[inline]
    pub fn loop_1(self) -> &'a mut W {
        self.variant(LOOPW::LOOP_1)
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
#[doc = "Values that can be written to the field `FRCPERR`"]
pub enum FRCPERRW {
    #[doc = "Generate normal parity"]
    FRCPERR_0,
    #[doc = "Generate inverted parity (error)"]
    FRCPERR_1,
}
impl FRCPERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRCPERRW::FRCPERR_0 => false,
            FRCPERRW::FRCPERR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRCPERRW<'a> {
    w: &'a mut W,
}
impl<'a> _FRCPERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRCPERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Generate normal parity"]
    #[inline]
    pub fn frcperr_0(self) -> &'a mut W {
        self.variant(FRCPERRW::FRCPERR_0)
    }
    #[doc = "Generate inverted parity (error)"]
    #[inline]
    pub fn frcperr_1(self) -> &'a mut W {
        self.variant(FRCPERRW::FRCPERR_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Software Reset. Indicates the status of the software reset (SRST_B bit of UCR2)."]
    #[inline]
    pub fn softrst(&self) -> SOFTRSTR {
        SOFTRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - RxFIFO FULL. Indicates the RxFIFO is full."]
    #[inline]
    pub fn rxfull(&self) -> RXFULLR {
        RXFULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - TxFIFO FULL. Indicates the TxFIFO is full."]
    #[inline]
    pub fn txfull(&self) -> TXFULLR {
        TXFULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - RxFIFO Empty. Indicates the RxFIFO is empty."]
    #[inline]
    pub fn rxempty(&self) -> RXEMPTYR {
        RXEMPTYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - TxFIFO Empty. Indicates that the TxFIFO is empty."]
    #[inline]
    pub fn txempty(&self) -> TXEMPTYR {
        TXEMPTYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - This bit is not used in this chip"]
    #[inline]
    pub fn rxdbg(&self) -> RXDBGR {
        RXDBGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Loop TX and RX for IR Test (LOOPIR)"]
    #[inline]
    pub fn loopir(&self) -> LOOPIRR {
        LOOPIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - This bit is not used in this chip"]
    #[inline]
    pub fn dbgen(&self) -> DBGENR {
        DBGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Loop TX and RX for Test"]
    #[inline]
    pub fn loop_(&self) -> LOOPR {
        LOOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Force Parity Error"]
    #[inline]
    pub fn frcperr(&self) -> FRCPERRR {
        FRCPERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 96 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Software Reset. Indicates the status of the software reset (SRST_B bit of UCR2)."]
    #[inline]
    pub fn softrst(&mut self) -> _SOFTRSTW {
        _SOFTRSTW { w: self }
    }
    #[doc = "Bit 3 - RxFIFO FULL. Indicates the RxFIFO is full."]
    #[inline]
    pub fn rxfull(&mut self) -> _RXFULLW {
        _RXFULLW { w: self }
    }
    #[doc = "Bit 4 - TxFIFO FULL. Indicates the TxFIFO is full."]
    #[inline]
    pub fn txfull(&mut self) -> _TXFULLW {
        _TXFULLW { w: self }
    }
    #[doc = "Bit 5 - RxFIFO Empty. Indicates the RxFIFO is empty."]
    #[inline]
    pub fn rxempty(&mut self) -> _RXEMPTYW {
        _RXEMPTYW { w: self }
    }
    #[doc = "Bit 6 - TxFIFO Empty. Indicates that the TxFIFO is empty."]
    #[inline]
    pub fn txempty(&mut self) -> _TXEMPTYW {
        _TXEMPTYW { w: self }
    }
    #[doc = "Bit 9 - This bit is not used in this chip"]
    #[inline]
    pub fn rxdbg(&mut self) -> _RXDBGW {
        _RXDBGW { w: self }
    }
    #[doc = "Bit 10 - Loop TX and RX for IR Test (LOOPIR)"]
    #[inline]
    pub fn loopir(&mut self) -> _LOOPIRW {
        _LOOPIRW { w: self }
    }
    #[doc = "Bit 11 - This bit is not used in this chip"]
    #[inline]
    pub fn dbgen(&mut self) -> _DBGENW {
        _DBGENW { w: self }
    }
    #[doc = "Bit 12 - Loop TX and RX for Test"]
    #[inline]
    pub fn loop_(&mut self) -> _LOOPW {
        _LOOPW { w: self }
    }
    #[doc = "Bit 13 - Force Parity Error"]
    #[inline]
    pub fn frcperr(&mut self) -> _FRCPERRW {
        _FRCPERRW { w: self }
    }
}
