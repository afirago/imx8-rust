#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STATREG {
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
#[doc = "Possible values of the field `TE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TER {
    #[doc = "TXFIFO contains one or more words."]
    TE_0,
    #[doc = "TXFIFO is empty."]
    TE_1,
}
impl TER {
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
            TER::TE_0 => false,
            TER::TE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TER {
        match value {
            false => TER::TE_0,
            true => TER::TE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TE_0`"]
    #[inline]
    pub fn is_te_0(&self) -> bool {
        *self == TER::TE_0
    }
    #[doc = "Checks if the value of the field is `TE_1`"]
    #[inline]
    pub fn is_te_1(&self) -> bool {
        *self == TER::TE_1
    }
}
#[doc = "Possible values of the field `TDR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDRR {
    #[doc = "Number of valid data slots in TXFIFO is greater than TX_THRESHOLD."]
    TDR_0,
    #[doc = "Number of valid data slots in TXFIFO is not greater than TX_THRESHOLD."]
    TDR_1,
}
impl TDRR {
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
            TDRR::TDR_0 => false,
            TDRR::TDR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDRR {
        match value {
            false => TDRR::TDR_0,
            true => TDRR::TDR_1,
        }
    }
    #[doc = "Checks if the value of the field is `TDR_0`"]
    #[inline]
    pub fn is_tdr_0(&self) -> bool {
        *self == TDRR::TDR_0
    }
    #[doc = "Checks if the value of the field is `TDR_1`"]
    #[inline]
    pub fn is_tdr_1(&self) -> bool {
        *self == TDRR::TDR_1
    }
}
#[doc = "Possible values of the field `TF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFR {
    #[doc = "TXFIFO is not Full."]
    TF_0,
    #[doc = "TXFIFO is Full."]
    TF_1,
}
impl TFR {
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
            TFR::TF_0 => false,
            TFR::TF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TFR {
        match value {
            false => TFR::TF_0,
            true => TFR::TF_1,
        }
    }
    #[doc = "Checks if the value of the field is `TF_0`"]
    #[inline]
    pub fn is_tf_0(&self) -> bool {
        *self == TFR::TF_0
    }
    #[doc = "Checks if the value of the field is `TF_1`"]
    #[inline]
    pub fn is_tf_1(&self) -> bool {
        *self == TFR::TF_1
    }
}
#[doc = "Possible values of the field `RR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRR {
    #[doc = "No valid data in RXFIFO."]
    RR_0,
    #[doc = "More than 1 word in RXFIFO."]
    RR_1,
}
impl RRR {
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
            RRR::RR_0 => false,
            RRR::RR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RRR {
        match value {
            false => RRR::RR_0,
            true => RRR::RR_1,
        }
    }
    #[doc = "Checks if the value of the field is `RR_0`"]
    #[inline]
    pub fn is_rr_0(&self) -> bool {
        *self == RRR::RR_0
    }
    #[doc = "Checks if the value of the field is `RR_1`"]
    #[inline]
    pub fn is_rr_1(&self) -> bool {
        *self == RRR::RR_1
    }
}
#[doc = "Possible values of the field `RDR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDRR {
    #[doc = "When RXTDE is set - Number of data entries in the RXFIFO is not greater than RX_THRESHOLD."]
    RDR_0,
    #[doc = "When RXTDE is set - Number of data entries in the RXFIFO is greater than RX_THRESHOLD or a DMA TAIL DMA condition exists."]
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
#[doc = "Possible values of the field `RF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFR {
    #[doc = "Not Full."]
    RF_0,
    #[doc = "Full."]
    RF_1,
}
impl RFR {
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
            RFR::RF_0 => false,
            RFR::RF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RFR {
        match value {
            false => RFR::RF_0,
            true => RFR::RF_1,
        }
    }
    #[doc = "Checks if the value of the field is `RF_0`"]
    #[inline]
    pub fn is_rf_0(&self) -> bool {
        *self == RFR::RF_0
    }
    #[doc = "Checks if the value of the field is `RF_1`"]
    #[inline]
    pub fn is_rf_1(&self) -> bool {
        *self == RFR::RF_1
    }
}
#[doc = "Possible values of the field `RO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROR {
    #[doc = "RXFIFO has no overflow."]
    RO_0,
    #[doc = "RXFIFO has overflowed."]
    RO_1,
}
impl ROR {
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
            ROR::RO_0 => false,
            ROR::RO_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ROR {
        match value {
            false => ROR::RO_0,
            true => ROR::RO_1,
        }
    }
    #[doc = "Checks if the value of the field is `RO_0`"]
    #[inline]
    pub fn is_ro_0(&self) -> bool {
        *self == ROR::RO_0
    }
    #[doc = "Checks if the value of the field is `RO_1`"]
    #[inline]
    pub fn is_ro_1(&self) -> bool {
        *self == ROR::RO_1
    }
}
#[doc = "Possible values of the field `TC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCR {
    #[doc = "Transfer in progress."]
    TC_0,
    #[doc = "Transfer completed."]
    TC_1,
}
impl TCR {
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
            TCR::TC_0 => false,
            TCR::TC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCR {
        match value {
            false => TCR::TC_0,
            true => TCR::TC_1,
        }
    }
    #[doc = "Checks if the value of the field is `TC_0`"]
    #[inline]
    pub fn is_tc_0(&self) -> bool {
        *self == TCR::TC_0
    }
    #[doc = "Checks if the value of the field is `TC_1`"]
    #[inline]
    pub fn is_tc_1(&self) -> bool {
        *self == TCR::TC_1
    }
}
#[doc = "Values that can be written to the field `RO`"]
pub enum ROW {
    #[doc = "RXFIFO has no overflow."]
    RO_0,
    #[doc = "RXFIFO has overflowed."]
    RO_1,
}
impl ROW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ROW::RO_0 => false,
            ROW::RO_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ROW<'a> {
    w: &'a mut W,
}
impl<'a> _ROW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ROW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RXFIFO has no overflow."]
    #[inline]
    pub fn ro_0(self) -> &'a mut W {
        self.variant(ROW::RO_0)
    }
    #[doc = "RXFIFO has overflowed."]
    #[inline]
    pub fn ro_1(self) -> &'a mut W {
        self.variant(ROW::RO_1)
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
#[doc = "Values that can be written to the field `TC`"]
pub enum TCW {
    #[doc = "Transfer in progress."]
    TC_0,
    #[doc = "Transfer completed."]
    TC_1,
}
impl TCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCW::TC_0 => false,
            TCW::TC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCW<'a> {
    w: &'a mut W,
}
impl<'a> _TCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transfer in progress."]
    #[inline]
    pub fn tc_0(self) -> &'a mut W {
        self.variant(TCW::TC_0)
    }
    #[doc = "Transfer completed."]
    #[inline]
    pub fn tc_1(self) -> &'a mut W {
        self.variant(TCW::TC_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - TXFIFO Empty. This bit is set if the TXFIFO is empty."]
    #[inline]
    pub fn te(&self) -> TER {
        TER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - TXFIFO Data Request."]
    #[inline]
    pub fn tdr(&self) -> TDRR {
        TDRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - TXFIFO Full. This bit is set when if the TXFIFO is full."]
    #[inline]
    pub fn tf(&self) -> TFR {
        TFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - RXFIFO Ready. This bit is set when one or more words are stored in the RXFIFO."]
    #[inline]
    pub fn rr(&self) -> RRR {
        RRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - RXFIFO Data Request."]
    #[inline]
    pub fn rdr(&self) -> RDRR {
        RDRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - RXFIFO Full. This bit is set when the RXFIFO is full."]
    #[inline]
    pub fn rf(&self) -> RFR {
        RFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - RXFIFO Overflow"]
    #[inline]
    pub fn ro(&self) -> ROR {
        ROR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Transfer Completed Status bit"]
    #[inline]
    pub fn tc(&self) -> TCR {
        TCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 6 - RXFIFO Overflow"]
    #[inline]
    pub fn ro(&mut self) -> _ROW {
        _ROW { w: self }
    }
    #[doc = "Bit 7 - Transfer Completed Status bit"]
    #[inline]
    pub fn tc(&mut self) -> _TCW {
        _TCW { w: self }
    }
}
