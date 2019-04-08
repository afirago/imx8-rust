#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHANNEL_CTRL_SET {
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
#[doc = "Possible values of the field `FREEZE_CHANNEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREEZE_CHANNELR {
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
impl FREEZE_CHANNELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            FREEZE_CHANNELR::NAND0 => 1,
            FREEZE_CHANNELR::NAND1 => 2,
            FREEZE_CHANNELR::NAND2 => 4,
            FREEZE_CHANNELR::NAND3 => 8,
            FREEZE_CHANNELR::NAND4 => 16,
            FREEZE_CHANNELR::NAND5 => 32,
            FREEZE_CHANNELR::NAND6 => 64,
            FREEZE_CHANNELR::NAND7 => 128,
            FREEZE_CHANNELR::SSP => 256,
            FREEZE_CHANNELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> FREEZE_CHANNELR {
        match value {
            1 => FREEZE_CHANNELR::NAND0,
            2 => FREEZE_CHANNELR::NAND1,
            4 => FREEZE_CHANNELR::NAND2,
            8 => FREEZE_CHANNELR::NAND3,
            16 => FREEZE_CHANNELR::NAND4,
            32 => FREEZE_CHANNELR::NAND5,
            64 => FREEZE_CHANNELR::NAND6,
            128 => FREEZE_CHANNELR::NAND7,
            256 => FREEZE_CHANNELR::SSP,
            i => FREEZE_CHANNELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NAND0`"]
    #[inline]
    pub fn is_nand0(&self) -> bool {
        *self == FREEZE_CHANNELR::NAND0
    }
    #[doc = "Checks if the value of the field is `NAND1`"]
    #[inline]
    pub fn is_nand1(&self) -> bool {
        *self == FREEZE_CHANNELR::NAND1
    }
    #[doc = "Checks if the value of the field is `NAND2`"]
    #[inline]
    pub fn is_nand2(&self) -> bool {
        *self == FREEZE_CHANNELR::NAND2
    }
    #[doc = "Checks if the value of the field is `NAND3`"]
    #[inline]
    pub fn is_nand3(&self) -> bool {
        *self == FREEZE_CHANNELR::NAND3
    }
    #[doc = "Checks if the value of the field is `NAND4`"]
    #[inline]
    pub fn is_nand4(&self) -> bool {
        *self == FREEZE_CHANNELR::NAND4
    }
    #[doc = "Checks if the value of the field is `NAND5`"]
    #[inline]
    pub fn is_nand5(&self) -> bool {
        *self == FREEZE_CHANNELR::NAND5
    }
    #[doc = "Checks if the value of the field is `NAND6`"]
    #[inline]
    pub fn is_nand6(&self) -> bool {
        *self == FREEZE_CHANNELR::NAND6
    }
    #[doc = "Checks if the value of the field is `NAND7`"]
    #[inline]
    pub fn is_nand7(&self) -> bool {
        *self == FREEZE_CHANNELR::NAND7
    }
    #[doc = "Checks if the value of the field is `SSP`"]
    #[inline]
    pub fn is_ssp(&self) -> bool {
        *self == FREEZE_CHANNELR::SSP
    }
}
#[doc = "Possible values of the field `RESET_CHANNEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_CHANNELR {
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
impl RESET_CHANNELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            RESET_CHANNELR::NAND0 => 1,
            RESET_CHANNELR::NAND1 => 2,
            RESET_CHANNELR::NAND2 => 4,
            RESET_CHANNELR::NAND3 => 8,
            RESET_CHANNELR::NAND4 => 16,
            RESET_CHANNELR::NAND5 => 32,
            RESET_CHANNELR::NAND6 => 64,
            RESET_CHANNELR::NAND7 => 128,
            RESET_CHANNELR::SSP => 256,
            RESET_CHANNELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> RESET_CHANNELR {
        match value {
            1 => RESET_CHANNELR::NAND0,
            2 => RESET_CHANNELR::NAND1,
            4 => RESET_CHANNELR::NAND2,
            8 => RESET_CHANNELR::NAND3,
            16 => RESET_CHANNELR::NAND4,
            32 => RESET_CHANNELR::NAND5,
            64 => RESET_CHANNELR::NAND6,
            128 => RESET_CHANNELR::NAND7,
            256 => RESET_CHANNELR::SSP,
            i => RESET_CHANNELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NAND0`"]
    #[inline]
    pub fn is_nand0(&self) -> bool {
        *self == RESET_CHANNELR::NAND0
    }
    #[doc = "Checks if the value of the field is `NAND1`"]
    #[inline]
    pub fn is_nand1(&self) -> bool {
        *self == RESET_CHANNELR::NAND1
    }
    #[doc = "Checks if the value of the field is `NAND2`"]
    #[inline]
    pub fn is_nand2(&self) -> bool {
        *self == RESET_CHANNELR::NAND2
    }
    #[doc = "Checks if the value of the field is `NAND3`"]
    #[inline]
    pub fn is_nand3(&self) -> bool {
        *self == RESET_CHANNELR::NAND3
    }
    #[doc = "Checks if the value of the field is `NAND4`"]
    #[inline]
    pub fn is_nand4(&self) -> bool {
        *self == RESET_CHANNELR::NAND4
    }
    #[doc = "Checks if the value of the field is `NAND5`"]
    #[inline]
    pub fn is_nand5(&self) -> bool {
        *self == RESET_CHANNELR::NAND5
    }
    #[doc = "Checks if the value of the field is `NAND6`"]
    #[inline]
    pub fn is_nand6(&self) -> bool {
        *self == RESET_CHANNELR::NAND6
    }
    #[doc = "Checks if the value of the field is `NAND7`"]
    #[inline]
    pub fn is_nand7(&self) -> bool {
        *self == RESET_CHANNELR::NAND7
    }
    #[doc = "Checks if the value of the field is `SSP`"]
    #[inline]
    pub fn is_ssp(&self) -> bool {
        *self == RESET_CHANNELR::SSP
    }
}
#[doc = "Values that can be written to the field `FREEZE_CHANNEL`"]
pub enum FREEZE_CHANNELW {
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
impl FREEZE_CHANNELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            FREEZE_CHANNELW::NAND0 => 1,
            FREEZE_CHANNELW::NAND1 => 2,
            FREEZE_CHANNELW::NAND2 => 4,
            FREEZE_CHANNELW::NAND3 => 8,
            FREEZE_CHANNELW::NAND4 => 16,
            FREEZE_CHANNELW::NAND5 => 32,
            FREEZE_CHANNELW::NAND6 => 64,
            FREEZE_CHANNELW::NAND7 => 128,
            FREEZE_CHANNELW::SSP => 256,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FREEZE_CHANNELW<'a> {
    w: &'a mut W,
}
impl<'a> _FREEZE_CHANNELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FREEZE_CHANNELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "NAND0"]
    #[inline]
    pub fn nand0(self) -> &'a mut W {
        self.variant(FREEZE_CHANNELW::NAND0)
    }
    #[doc = "NAND1"]
    #[inline]
    pub fn nand1(self) -> &'a mut W {
        self.variant(FREEZE_CHANNELW::NAND1)
    }
    #[doc = "NAND2"]
    #[inline]
    pub fn nand2(self) -> &'a mut W {
        self.variant(FREEZE_CHANNELW::NAND2)
    }
    #[doc = "NAND3"]
    #[inline]
    pub fn nand3(self) -> &'a mut W {
        self.variant(FREEZE_CHANNELW::NAND3)
    }
    #[doc = "NAND4"]
    #[inline]
    pub fn nand4(self) -> &'a mut W {
        self.variant(FREEZE_CHANNELW::NAND4)
    }
    #[doc = "NAND5"]
    #[inline]
    pub fn nand5(self) -> &'a mut W {
        self.variant(FREEZE_CHANNELW::NAND5)
    }
    #[doc = "NAND6"]
    #[inline]
    pub fn nand6(self) -> &'a mut W {
        self.variant(FREEZE_CHANNELW::NAND6)
    }
    #[doc = "NAND7"]
    #[inline]
    pub fn nand7(self) -> &'a mut W {
        self.variant(FREEZE_CHANNELW::NAND7)
    }
    #[doc = "SSP"]
    #[inline]
    pub fn ssp(self) -> &'a mut W {
        self.variant(FREEZE_CHANNELW::SSP)
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
#[doc = "Values that can be written to the field `RESET_CHANNEL`"]
pub enum RESET_CHANNELW {
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
impl RESET_CHANNELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            RESET_CHANNELW::NAND0 => 1,
            RESET_CHANNELW::NAND1 => 2,
            RESET_CHANNELW::NAND2 => 4,
            RESET_CHANNELW::NAND3 => 8,
            RESET_CHANNELW::NAND4 => 16,
            RESET_CHANNELW::NAND5 => 32,
            RESET_CHANNELW::NAND6 => 64,
            RESET_CHANNELW::NAND7 => 128,
            RESET_CHANNELW::SSP => 256,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESET_CHANNELW<'a> {
    w: &'a mut W,
}
impl<'a> _RESET_CHANNELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESET_CHANNELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "NAND0"]
    #[inline]
    pub fn nand0(self) -> &'a mut W {
        self.variant(RESET_CHANNELW::NAND0)
    }
    #[doc = "NAND1"]
    #[inline]
    pub fn nand1(self) -> &'a mut W {
        self.variant(RESET_CHANNELW::NAND1)
    }
    #[doc = "NAND2"]
    #[inline]
    pub fn nand2(self) -> &'a mut W {
        self.variant(RESET_CHANNELW::NAND2)
    }
    #[doc = "NAND3"]
    #[inline]
    pub fn nand3(self) -> &'a mut W {
        self.variant(RESET_CHANNELW::NAND3)
    }
    #[doc = "NAND4"]
    #[inline]
    pub fn nand4(self) -> &'a mut W {
        self.variant(RESET_CHANNELW::NAND4)
    }
    #[doc = "NAND5"]
    #[inline]
    pub fn nand5(self) -> &'a mut W {
        self.variant(RESET_CHANNELW::NAND5)
    }
    #[doc = "NAND6"]
    #[inline]
    pub fn nand6(self) -> &'a mut W {
        self.variant(RESET_CHANNELW::NAND6)
    }
    #[doc = "NAND7"]
    #[inline]
    pub fn nand7(self) -> &'a mut W {
        self.variant(RESET_CHANNELW::NAND7)
    }
    #[doc = "SSP"]
    #[inline]
    pub fn ssp(self) -> &'a mut W {
        self.variant(RESET_CHANNELW::SSP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:15 - Setting a bit in this field will freeze the DMA channel associated with it"]
    #[inline]
    pub fn freeze_channel(&self) -> FREEZE_CHANNELR {
        FREEZE_CHANNELR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 16:31 - Setting a bit in this field causes the DMA controller to take the corresponding channel through its reset state"]
    #[inline]
    pub fn reset_channel(&self) -> RESET_CHANNELR {
        RESET_CHANNELR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
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
    #[doc = "Bits 0:15 - Setting a bit in this field will freeze the DMA channel associated with it"]
    #[inline]
    pub fn freeze_channel(&mut self) -> _FREEZE_CHANNELW {
        _FREEZE_CHANNELW { w: self }
    }
    #[doc = "Bits 16:31 - Setting a bit in this field causes the DMA controller to take the corresponding channel through its reset state"]
    #[inline]
    pub fn reset_channel(&mut self) -> _RESET_CHANNELW {
        _RESET_CHANNELW { w: self }
    }
}
