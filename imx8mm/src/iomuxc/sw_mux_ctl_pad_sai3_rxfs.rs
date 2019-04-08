#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SW_MUX_CTL_PAD_SAI3_RXFS {
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
#[doc = "Possible values of the field `MUX_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUX_MODER {
    #[doc = "Select signal SAI3_RX_SYNC"]
    ALT0,
    #[doc = "Select signal GPT1_CAPTURE1"]
    ALT1,
    #[doc = "Select signal SAI5_RX_SYNC- Configure register IOMUXC_SAI5_RX_SYNC_SELECT_INPUTSelect Input Register for mode ALT2."]
    ALT2,
    #[doc = "Select signal SAI3_RX_DATA\\[1\\]"]
    ALT3,
    #[doc = "Select signal GPIO4_IO\\[28\\]"]
    ALT5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MUX_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MUX_MODER::ALT0 => 0,
            MUX_MODER::ALT1 => 1,
            MUX_MODER::ALT2 => 2,
            MUX_MODER::ALT3 => 3,
            MUX_MODER::ALT5 => 5,
            MUX_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MUX_MODER {
        match value {
            0 => MUX_MODER::ALT0,
            1 => MUX_MODER::ALT1,
            2 => MUX_MODER::ALT2,
            3 => MUX_MODER::ALT3,
            5 => MUX_MODER::ALT5,
            i => MUX_MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALT0`"]
    #[inline]
    pub fn is_alt0(&self) -> bool {
        *self == MUX_MODER::ALT0
    }
    #[doc = "Checks if the value of the field is `ALT1`"]
    #[inline]
    pub fn is_alt1(&self) -> bool {
        *self == MUX_MODER::ALT1
    }
    #[doc = "Checks if the value of the field is `ALT2`"]
    #[inline]
    pub fn is_alt2(&self) -> bool {
        *self == MUX_MODER::ALT2
    }
    #[doc = "Checks if the value of the field is `ALT3`"]
    #[inline]
    pub fn is_alt3(&self) -> bool {
        *self == MUX_MODER::ALT3
    }
    #[doc = "Checks if the value of the field is `ALT5`"]
    #[inline]
    pub fn is_alt5(&self) -> bool {
        *self == MUX_MODER::ALT5
    }
}
#[doc = "Possible values of the field `SION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIONR {
    #[doc = "Input Path is determined by functionality of the selected mux mode (regular)."]
    DISABLED,
    #[doc = "Force input path of pad SAI3_RXFS"]
    ENABLED,
}
impl SIONR {
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
            SIONR::DISABLED => false,
            SIONR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SIONR {
        match value {
            false => SIONR::DISABLED,
            true => SIONR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SIONR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SIONR::ENABLED
    }
}
#[doc = "Values that can be written to the field `MUX_MODE`"]
pub enum MUX_MODEW {
    #[doc = "Select signal SAI3_RX_SYNC"]
    ALT0,
    #[doc = "Select signal GPT1_CAPTURE1"]
    ALT1,
    #[doc = "Select signal SAI5_RX_SYNC- Configure register IOMUXC_SAI5_RX_SYNC_SELECT_INPUTSelect Input Register for mode ALT2."]
    ALT2,
    #[doc = "Select signal SAI3_RX_DATA\\[1\\]"]
    ALT3,
    #[doc = "Select signal GPIO4_IO\\[28\\]"]
    ALT5,
}
impl MUX_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MUX_MODEW::ALT0 => 0,
            MUX_MODEW::ALT1 => 1,
            MUX_MODEW::ALT2 => 2,
            MUX_MODEW::ALT3 => 3,
            MUX_MODEW::ALT5 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MUX_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MUX_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MUX_MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select signal SAI3_RX_SYNC"]
    #[inline]
    pub fn alt0(self) -> &'a mut W {
        self.variant(MUX_MODEW::ALT0)
    }
    #[doc = "Select signal GPT1_CAPTURE1"]
    #[inline]
    pub fn alt1(self) -> &'a mut W {
        self.variant(MUX_MODEW::ALT1)
    }
    #[doc = "Select signal SAI5_RX_SYNC- Configure register IOMUXC_SAI5_RX_SYNC_SELECT_INPUTSelect Input Register for mode ALT2."]
    #[inline]
    pub fn alt2(self) -> &'a mut W {
        self.variant(MUX_MODEW::ALT2)
    }
    #[doc = "Select signal SAI3_RX_DATA\\[1\\]"]
    #[inline]
    pub fn alt3(self) -> &'a mut W {
        self.variant(MUX_MODEW::ALT3)
    }
    #[doc = "Select signal GPIO4_IO\\[28\\]"]
    #[inline]
    pub fn alt5(self) -> &'a mut W {
        self.variant(MUX_MODEW::ALT5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SION`"]
pub enum SIONW {
    #[doc = "Input Path is determined by functionality of the selected mux mode (regular)."]
    DISABLED,
    #[doc = "Force input path of pad SAI3_RXFS"]
    ENABLED,
}
impl SIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SIONW::DISABLED => false,
            SIONW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIONW<'a> {
    w: &'a mut W,
}
impl<'a> _SIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input Path is determined by functionality of the selected mux mode (regular)."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SIONW::DISABLED)
    }
    #[doc = "Force input path of pad SAI3_RXFS"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SIONW::ENABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - MUX Mode Select Field"]
    #[inline]
    pub fn mux_mode(&self) -> MUX_MODER {
        MUX_MODER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Software Input On Field"]
    #[inline]
    pub fn sion(&self) -> SIONR {
        SIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 5 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - MUX Mode Select Field"]
    #[inline]
    pub fn mux_mode(&mut self) -> _MUX_MODEW {
        _MUX_MODEW { w: self }
    }
    #[doc = "Bit 4 - Software Input On Field"]
    #[inline]
    pub fn sion(&mut self) -> _SIONW {
        _SIONW { w: self }
    }
}
