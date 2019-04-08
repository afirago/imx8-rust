#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMACFG {
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
pub struct IDLE_SLOPER {
    bits: u16,
}
impl IDLE_SLOPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `DMA_CLASS_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_CLASS_ENR {
    #[doc = "The DMA controller's channel for the class is not used. Disabling the DMA controller of a class also requires disabling the class match comparator for the class (see registers RCMRn). When class 1 and class 2 queues are disabled then their frames will be placed in queue 0."]
    DMA_CLASS_EN_0,
    #[doc = "Enable the DMA controller to support the corresponding descriptor ring for this class of traffic."]
    DMA_CLASS_EN_1,
}
impl DMA_CLASS_ENR {
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
            DMA_CLASS_ENR::DMA_CLASS_EN_0 => false,
            DMA_CLASS_ENR::DMA_CLASS_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_CLASS_ENR {
        match value {
            false => DMA_CLASS_ENR::DMA_CLASS_EN_0,
            true => DMA_CLASS_ENR::DMA_CLASS_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_CLASS_EN_0`"]
    #[inline]
    pub fn is_dma_class_en_0(&self) -> bool {
        *self == DMA_CLASS_ENR::DMA_CLASS_EN_0
    }
    #[doc = "Checks if the value of the field is `DMA_CLASS_EN_1`"]
    #[inline]
    pub fn is_dma_class_en_1(&self) -> bool {
        *self == DMA_CLASS_ENR::DMA_CLASS_EN_1
    }
}
#[doc = "Possible values of the field `CALC_NOIPG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALC_NOIPGR {
    #[doc = "The traffic shaper function should consider 12 octets of IPG in addition to the frame data transferred for a frame when doing bandwidth calculations. This is the default."]
    CALC_NOIPG_0,
    #[doc = "Addition of 12 bytes for the IPG should be omitted when calculating the bandwidth (for traffic shaping, when writing a frame into the transmit FIFO, the shaper will usually consider 12 bytes of IPG for every frame as part of the bandwidth allocated by the frame. This addition can be suppressed, meaning short frames will become more bandwidth than large frames due to the relation of data to IPG overhead)."]
    CALC_NOIPG_1,
}
impl CALC_NOIPGR {
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
            CALC_NOIPGR::CALC_NOIPG_0 => false,
            CALC_NOIPGR::CALC_NOIPG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CALC_NOIPGR {
        match value {
            false => CALC_NOIPGR::CALC_NOIPG_0,
            true => CALC_NOIPGR::CALC_NOIPG_1,
        }
    }
    #[doc = "Checks if the value of the field is `CALC_NOIPG_0`"]
    #[inline]
    pub fn is_calc_noipg_0(&self) -> bool {
        *self == CALC_NOIPGR::CALC_NOIPG_0
    }
    #[doc = "Checks if the value of the field is `CALC_NOIPG_1`"]
    #[inline]
    pub fn is_calc_noipg_1(&self) -> bool {
        *self == CALC_NOIPGR::CALC_NOIPG_1
    }
}
#[doc = r" Proxy"]
pub struct _IDLE_SLOPEW<'a> {
    w: &'a mut W,
}
impl<'a> _IDLE_SLOPEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA_CLASS_EN`"]
pub enum DMA_CLASS_ENW {
    #[doc = "The DMA controller's channel for the class is not used. Disabling the DMA controller of a class also requires disabling the class match comparator for the class (see registers RCMRn). When class 1 and class 2 queues are disabled then their frames will be placed in queue 0."]
    DMA_CLASS_EN_0,
    #[doc = "Enable the DMA controller to support the corresponding descriptor ring for this class of traffic."]
    DMA_CLASS_EN_1,
}
impl DMA_CLASS_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA_CLASS_ENW::DMA_CLASS_EN_0 => false,
            DMA_CLASS_ENW::DMA_CLASS_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA_CLASS_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_CLASS_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_CLASS_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA controller's channel for the class is not used. Disabling the DMA controller of a class also requires disabling the class match comparator for the class (see registers RCMRn). When class 1 and class 2 queues are disabled then their frames will be placed in queue 0."]
    #[inline]
    pub fn dma_class_en_0(self) -> &'a mut W {
        self.variant(DMA_CLASS_ENW::DMA_CLASS_EN_0)
    }
    #[doc = "Enable the DMA controller to support the corresponding descriptor ring for this class of traffic."]
    #[inline]
    pub fn dma_class_en_1(self) -> &'a mut W {
        self.variant(DMA_CLASS_ENW::DMA_CLASS_EN_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CALC_NOIPG`"]
pub enum CALC_NOIPGW {
    #[doc = "The traffic shaper function should consider 12 octets of IPG in addition to the frame data transferred for a frame when doing bandwidth calculations. This is the default."]
    CALC_NOIPG_0,
    #[doc = "Addition of 12 bytes for the IPG should be omitted when calculating the bandwidth (for traffic shaping, when writing a frame into the transmit FIFO, the shaper will usually consider 12 bytes of IPG for every frame as part of the bandwidth allocated by the frame. This addition can be suppressed, meaning short frames will become more bandwidth than large frames due to the relation of data to IPG overhead)."]
    CALC_NOIPG_1,
}
impl CALC_NOIPGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CALC_NOIPGW::CALC_NOIPG_0 => false,
            CALC_NOIPGW::CALC_NOIPG_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CALC_NOIPGW<'a> {
    w: &'a mut W,
}
impl<'a> _CALC_NOIPGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CALC_NOIPGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The traffic shaper function should consider 12 octets of IPG in addition to the frame data transferred for a frame when doing bandwidth calculations. This is the default."]
    #[inline]
    pub fn calc_noipg_0(self) -> &'a mut W {
        self.variant(CALC_NOIPGW::CALC_NOIPG_0)
    }
    #[doc = "Addition of 12 bytes for the IPG should be omitted when calculating the bandwidth (for traffic shaping, when writing a frame into the transmit FIFO, the shaper will usually consider 12 bytes of IPG for every frame as part of the bandwidth allocated by the frame. This addition can be suppressed, meaning short frames will become more bandwidth than large frames due to the relation of data to IPG overhead)."]
    #[inline]
    pub fn calc_noipg_1(self) -> &'a mut W {
        self.variant(CALC_NOIPGW::CALC_NOIPG_1)
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
        const OFFSET: u8 = 17;
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
    #[doc = "Bits 0:15 - Idle slope"]
    #[inline]
    pub fn idle_slope(&self) -> IDLE_SLOPER {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        IDLE_SLOPER { bits }
    }
    #[doc = "Bit 16 - DMA class enable"]
    #[inline]
    pub fn dma_class_en(&self) -> DMA_CLASS_ENR {
        DMA_CLASS_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Calculate no IPG"]
    #[inline]
    pub fn calc_noipg(&self) -> CALC_NOIPGR {
        CALC_NOIPGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
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
    #[doc = "Bits 0:15 - Idle slope"]
    #[inline]
    pub fn idle_slope(&mut self) -> _IDLE_SLOPEW {
        _IDLE_SLOPEW { w: self }
    }
    #[doc = "Bit 16 - DMA class enable"]
    #[inline]
    pub fn dma_class_en(&mut self) -> _DMA_CLASS_ENW {
        _DMA_CLASS_ENW { w: self }
    }
    #[doc = "Bit 17 - Calculate no IPG"]
    #[inline]
    pub fn calc_noipg(&mut self) -> _CALC_NOIPGW {
        _CALC_NOIPGW { w: self }
    }
}
