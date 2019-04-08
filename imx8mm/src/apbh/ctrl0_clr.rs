#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL0_CLR {
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
#[doc = "Possible values of the field `CLKGATE_CHANNEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKGATE_CHANNELR {
    #[doc = "NAND0"]
    NAND0,
    #[doc = "NAND1"]
    NAND1,
    #[doc = "NAND2"]
    NAND2,
    #[doc = "NAND3"]
    NAND3,
    #[doc = "NAND4"]
    NAND4,
    #[doc = "NAND5"]
    NAND5,
    #[doc = "NAND6"]
    NAND6,
    #[doc = "NAND7"]
    NAND7,
    #[doc = "SSP"]
    SSP,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl CLKGATE_CHANNELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            CLKGATE_CHANNELR::NAND0 => 1,
            CLKGATE_CHANNELR::NAND1 => 2,
            CLKGATE_CHANNELR::NAND2 => 4,
            CLKGATE_CHANNELR::NAND3 => 8,
            CLKGATE_CHANNELR::NAND4 => 16,
            CLKGATE_CHANNELR::NAND5 => 32,
            CLKGATE_CHANNELR::NAND6 => 64,
            CLKGATE_CHANNELR::NAND7 => 128,
            CLKGATE_CHANNELR::SSP => 256,
            CLKGATE_CHANNELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> CLKGATE_CHANNELR {
        match value {
            1 => CLKGATE_CHANNELR::NAND0,
            2 => CLKGATE_CHANNELR::NAND1,
            4 => CLKGATE_CHANNELR::NAND2,
            8 => CLKGATE_CHANNELR::NAND3,
            16 => CLKGATE_CHANNELR::NAND4,
            32 => CLKGATE_CHANNELR::NAND5,
            64 => CLKGATE_CHANNELR::NAND6,
            128 => CLKGATE_CHANNELR::NAND7,
            256 => CLKGATE_CHANNELR::SSP,
            i => CLKGATE_CHANNELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NAND0`"]
    #[inline]
    pub fn is_nand0(&self) -> bool {
        *self == CLKGATE_CHANNELR::NAND0
    }
    #[doc = "Checks if the value of the field is `NAND1`"]
    #[inline]
    pub fn is_nand1(&self) -> bool {
        *self == CLKGATE_CHANNELR::NAND1
    }
    #[doc = "Checks if the value of the field is `NAND2`"]
    #[inline]
    pub fn is_nand2(&self) -> bool {
        *self == CLKGATE_CHANNELR::NAND2
    }
    #[doc = "Checks if the value of the field is `NAND3`"]
    #[inline]
    pub fn is_nand3(&self) -> bool {
        *self == CLKGATE_CHANNELR::NAND3
    }
    #[doc = "Checks if the value of the field is `NAND4`"]
    #[inline]
    pub fn is_nand4(&self) -> bool {
        *self == CLKGATE_CHANNELR::NAND4
    }
    #[doc = "Checks if the value of the field is `NAND5`"]
    #[inline]
    pub fn is_nand5(&self) -> bool {
        *self == CLKGATE_CHANNELR::NAND5
    }
    #[doc = "Checks if the value of the field is `NAND6`"]
    #[inline]
    pub fn is_nand6(&self) -> bool {
        *self == CLKGATE_CHANNELR::NAND6
    }
    #[doc = "Checks if the value of the field is `NAND7`"]
    #[inline]
    pub fn is_nand7(&self) -> bool {
        *self == CLKGATE_CHANNELR::NAND7
    }
    #[doc = "Checks if the value of the field is `SSP`"]
    #[inline]
    pub fn is_ssp(&self) -> bool {
        *self == CLKGATE_CHANNELR::SSP
    }
}
#[doc = r" Value of the field"]
pub struct APB_BURST_ENR {
    bits: bool,
}
impl APB_BURST_ENR {
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
pub struct AHB_BURST8_ENR {
    bits: bool,
}
impl AHB_BURST8_ENR {
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
pub struct CLKGATER {
    bits: bool,
}
impl CLKGATER {
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
pub struct SFTRSTR {
    bits: bool,
}
impl SFTRSTR {
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
#[doc = "Values that can be written to the field `CLKGATE_CHANNEL`"]
pub enum CLKGATE_CHANNELW {
    #[doc = "NAND0"]
    NAND0,
    #[doc = "NAND1"]
    NAND1,
    #[doc = "NAND2"]
    NAND2,
    #[doc = "NAND3"]
    NAND3,
    #[doc = "NAND4"]
    NAND4,
    #[doc = "NAND5"]
    NAND5,
    #[doc = "NAND6"]
    NAND6,
    #[doc = "NAND7"]
    NAND7,
    #[doc = "SSP"]
    SSP,
}
impl CLKGATE_CHANNELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            CLKGATE_CHANNELW::NAND0 => 1,
            CLKGATE_CHANNELW::NAND1 => 2,
            CLKGATE_CHANNELW::NAND2 => 4,
            CLKGATE_CHANNELW::NAND3 => 8,
            CLKGATE_CHANNELW::NAND4 => 16,
            CLKGATE_CHANNELW::NAND5 => 32,
            CLKGATE_CHANNELW::NAND6 => 64,
            CLKGATE_CHANNELW::NAND7 => 128,
            CLKGATE_CHANNELW::SSP => 256,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKGATE_CHANNELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKGATE_CHANNELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKGATE_CHANNELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "NAND0"]
    #[inline]
    pub fn nand0(self) -> &'a mut W {
        self.variant(CLKGATE_CHANNELW::NAND0)
    }
    #[doc = "NAND1"]
    #[inline]
    pub fn nand1(self) -> &'a mut W {
        self.variant(CLKGATE_CHANNELW::NAND1)
    }
    #[doc = "NAND2"]
    #[inline]
    pub fn nand2(self) -> &'a mut W {
        self.variant(CLKGATE_CHANNELW::NAND2)
    }
    #[doc = "NAND3"]
    #[inline]
    pub fn nand3(self) -> &'a mut W {
        self.variant(CLKGATE_CHANNELW::NAND3)
    }
    #[doc = "NAND4"]
    #[inline]
    pub fn nand4(self) -> &'a mut W {
        self.variant(CLKGATE_CHANNELW::NAND4)
    }
    #[doc = "NAND5"]
    #[inline]
    pub fn nand5(self) -> &'a mut W {
        self.variant(CLKGATE_CHANNELW::NAND5)
    }
    #[doc = "NAND6"]
    #[inline]
    pub fn nand6(self) -> &'a mut W {
        self.variant(CLKGATE_CHANNELW::NAND6)
    }
    #[doc = "NAND7"]
    #[inline]
    pub fn nand7(self) -> &'a mut W {
        self.variant(CLKGATE_CHANNELW::NAND7)
    }
    #[doc = "SSP"]
    #[inline]
    pub fn ssp(self) -> &'a mut W {
        self.variant(CLKGATE_CHANNELW::SSP)
    }
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
#[doc = r" Proxy"]
pub struct _APB_BURST_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _APB_BURST_ENW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AHB_BURST8_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _AHB_BURST8_ENW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKGATEW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SFTRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SFTRSTW<'a> {
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
    #[doc = "Bits 0:15 - These bits must be set to zero for normal operation of each channel"]
    #[inline]
    pub fn clkgate_channel(&self) -> CLKGATE_CHANNELR {
        CLKGATE_CHANNELR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bit 28 - Set this bit to one to enable apb master do a continous transfers when a device request a burst dma"]
    #[inline]
    pub fn apb_burst_en(&self) -> APB_BURST_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APB_BURST_ENR { bits }
    }
    #[doc = "Bit 29 - Set this bit to one (default) to enable AHB 8-beat burst"]
    #[inline]
    pub fn ahb_burst8_en(&self) -> AHB_BURST8_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AHB_BURST8_ENR { bits }
    }
    #[doc = "Bit 30 - This bit must be set to zero for normal operation"]
    #[inline]
    pub fn clkgate(&self) -> CLKGATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKGATER { bits }
    }
    #[doc = "Bit 31 - Set this bit to zero to enable normal APBH DMA operation"]
    #[inline]
    pub fn sftrst(&self) -> SFTRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SFTRSTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3758096384 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - These bits must be set to zero for normal operation of each channel"]
    #[inline]
    pub fn clkgate_channel(&mut self) -> _CLKGATE_CHANNELW {
        _CLKGATE_CHANNELW { w: self }
    }
    #[doc = "Bit 28 - Set this bit to one to enable apb master do a continous transfers when a device request a burst dma"]
    #[inline]
    pub fn apb_burst_en(&mut self) -> _APB_BURST_ENW {
        _APB_BURST_ENW { w: self }
    }
    #[doc = "Bit 29 - Set this bit to one (default) to enable AHB 8-beat burst"]
    #[inline]
    pub fn ahb_burst8_en(&mut self) -> _AHB_BURST8_ENW {
        _AHB_BURST8_ENW { w: self }
    }
    #[doc = "Bit 30 - This bit must be set to zero for normal operation"]
    #[inline]
    pub fn clkgate(&mut self) -> _CLKGATEW {
        _CLKGATEW { w: self }
    }
    #[doc = "Bit 31 - Set this bit to zero to enable normal APBH DMA operation"]
    #[inline]
    pub fn sftrst(&mut self) -> _SFTRSTW {
        _SFTRSTW { w: self }
    }
}
