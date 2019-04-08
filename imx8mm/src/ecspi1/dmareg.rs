#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMAREG {
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
pub struct TX_THRESHOLDR {
    bits: u8,
}
impl TX_THRESHOLDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TEDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEDENR {
    #[doc = "Disable"]
    TEDEN_0,
    #[doc = "Enable"]
    TEDEN_1,
}
impl TEDENR {
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
            TEDENR::TEDEN_0 => false,
            TEDENR::TEDEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEDENR {
        match value {
            false => TEDENR::TEDEN_0,
            true => TEDENR::TEDEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TEDEN_0`"]
    #[inline]
    pub fn is_teden_0(&self) -> bool {
        *self == TEDENR::TEDEN_0
    }
    #[doc = "Checks if the value of the field is `TEDEN_1`"]
    #[inline]
    pub fn is_teden_1(&self) -> bool {
        *self == TEDENR::TEDEN_1
    }
}
#[doc = r" Value of the field"]
pub struct RX_THRESHOLDR {
    bits: u8,
}
impl RX_THRESHOLDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RXDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDENR {
    #[doc = "Disable"]
    RXDEN_0,
    #[doc = "Enable"]
    RXDEN_1,
}
impl RXDENR {
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
            RXDENR::RXDEN_0 => false,
            RXDENR::RXDEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXDENR {
        match value {
            false => RXDENR::RXDEN_0,
            true => RXDENR::RXDEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXDEN_0`"]
    #[inline]
    pub fn is_rxden_0(&self) -> bool {
        *self == RXDENR::RXDEN_0
    }
    #[doc = "Checks if the value of the field is `RXDEN_1`"]
    #[inline]
    pub fn is_rxden_1(&self) -> bool {
        *self == RXDENR::RXDEN_1
    }
}
#[doc = r" Value of the field"]
pub struct RX_DMA_LENGTHR {
    bits: u8,
}
impl RX_DMA_LENGTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RXTDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTDENR {
    #[doc = "Disable"]
    RXTDEN_0,
    #[doc = "Enable"]
    RXTDEN_1,
}
impl RXTDENR {
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
            RXTDENR::RXTDEN_0 => false,
            RXTDENR::RXTDEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXTDENR {
        match value {
            false => RXTDENR::RXTDEN_0,
            true => RXTDENR::RXTDEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXTDEN_0`"]
    #[inline]
    pub fn is_rxtden_0(&self) -> bool {
        *self == RXTDENR::RXTDEN_0
    }
    #[doc = "Checks if the value of the field is `RXTDEN_1`"]
    #[inline]
    pub fn is_rxtden_1(&self) -> bool {
        *self == RXTDENR::RXTDEN_1
    }
}
#[doc = r" Proxy"]
pub struct _TX_THRESHOLDW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_THRESHOLDW<'a> {
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
#[doc = "Values that can be written to the field `TEDEN`"]
pub enum TEDENW {
    #[doc = "Disable"]
    TEDEN_0,
    #[doc = "Enable"]
    TEDEN_1,
}
impl TEDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TEDENW::TEDEN_0 => false,
            TEDENW::TEDEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TEDENW<'a> {
    w: &'a mut W,
}
impl<'a> _TEDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn teden_0(self) -> &'a mut W {
        self.variant(TEDENW::TEDEN_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn teden_1(self) -> &'a mut W {
        self.variant(TEDENW::TEDEN_1)
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
pub struct _RX_THRESHOLDW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_THRESHOLDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXDEN`"]
pub enum RXDENW {
    #[doc = "Disable"]
    RXDEN_0,
    #[doc = "Enable"]
    RXDEN_1,
}
impl RXDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXDENW::RXDEN_0 => false,
            RXDENW::RXDEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXDENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn rxden_0(self) -> &'a mut W {
        self.variant(RXDENW::RXDEN_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn rxden_1(self) -> &'a mut W {
        self.variant(RXDENW::RXDEN_1)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RX_DMA_LENGTHW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_DMA_LENGTHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXTDEN`"]
pub enum RXTDENW {
    #[doc = "Disable"]
    RXTDEN_0,
    #[doc = "Enable"]
    RXTDEN_1,
}
impl RXTDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXTDENW::RXTDEN_0 => false,
            RXTDENW::RXTDEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXTDENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXTDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn rxtden_0(self) -> &'a mut W {
        self.variant(RXTDENW::RXTDEN_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn rxtden_1(self) -> &'a mut W {
        self.variant(RXTDENW::RXTDEN_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:5 - TX THRESHOLD"]
    #[inline]
    pub fn tx_threshold(&self) -> TX_THRESHOLDR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TX_THRESHOLDR { bits }
    }
    #[doc = "Bit 7 - TXFIFO Empty DMA Request Enable. This bit enables/disables the TXFIFO Empty DMA Request."]
    #[inline]
    pub fn teden(&self) -> TEDENR {
        TEDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:21 - RX THRESHOLD"]
    #[inline]
    pub fn rx_threshold(&self) -> RX_THRESHOLDR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RX_THRESHOLDR { bits }
    }
    #[doc = "Bit 23 - RXFIFO DMA Request Enable. This bit enables/disables the RXFIFO DMA Request."]
    #[inline]
    pub fn rxden(&self) -> RXDENR {
        RXDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:29 - RX DMA LENGTH"]
    #[inline]
    pub fn rx_dma_length(&self) -> RX_DMA_LENGTHR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RX_DMA_LENGTHR { bits }
    }
    #[doc = "Bit 31 - RXFIFO TAIL DMA Request/Interrupt Enable"]
    #[inline]
    pub fn rxtden(&self) -> RXTDENR {
        RXTDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bits 0:5 - TX THRESHOLD"]
    #[inline]
    pub fn tx_threshold(&mut self) -> _TX_THRESHOLDW {
        _TX_THRESHOLDW { w: self }
    }
    #[doc = "Bit 7 - TXFIFO Empty DMA Request Enable. This bit enables/disables the TXFIFO Empty DMA Request."]
    #[inline]
    pub fn teden(&mut self) -> _TEDENW {
        _TEDENW { w: self }
    }
    #[doc = "Bits 16:21 - RX THRESHOLD"]
    #[inline]
    pub fn rx_threshold(&mut self) -> _RX_THRESHOLDW {
        _RX_THRESHOLDW { w: self }
    }
    #[doc = "Bit 23 - RXFIFO DMA Request Enable. This bit enables/disables the RXFIFO DMA Request."]
    #[inline]
    pub fn rxden(&mut self) -> _RXDENW {
        _RXDENW { w: self }
    }
    #[doc = "Bits 24:29 - RX DMA LENGTH"]
    #[inline]
    pub fn rx_dma_length(&mut self) -> _RX_DMA_LENGTHW {
        _RX_DMA_LENGTHW { w: self }
    }
    #[doc = "Bit 31 - RXFIFO TAIL DMA Request/Interrupt Enable"]
    #[inline]
    pub fn rxtden(&mut self) -> _RXTDENW {
        _RXTDENW { w: self }
    }
}
