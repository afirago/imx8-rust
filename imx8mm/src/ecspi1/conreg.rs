#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONREG {
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
#[doc = "Possible values of the field `EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENR {
    #[doc = "Disable the block."]
    EN_0,
    #[doc = "Enable the block."]
    EN_1,
}
impl ENR {
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
            ENR::EN_0 => false,
            ENR::EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENR {
        match value {
            false => ENR::EN_0,
            true => ENR::EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_0`"]
    #[inline]
    pub fn is_en_0(&self) -> bool {
        *self == ENR::EN_0
    }
    #[doc = "Checks if the value of the field is `EN_1`"]
    #[inline]
    pub fn is_en_1(&self) -> bool {
        *self == ENR::EN_1
    }
}
#[doc = "Possible values of the field `HT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTR {
    #[doc = "Disable HT mode."]
    HT_0,
    #[doc = "Enable HT mode."]
    HT_1,
}
impl HTR {
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
            HTR::HT_0 => false,
            HTR::HT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HTR {
        match value {
            false => HTR::HT_0,
            true => HTR::HT_1,
        }
    }
    #[doc = "Checks if the value of the field is `HT_0`"]
    #[inline]
    pub fn is_ht_0(&self) -> bool {
        *self == HTR::HT_0
    }
    #[doc = "Checks if the value of the field is `HT_1`"]
    #[inline]
    pub fn is_ht_1(&self) -> bool {
        *self == HTR::HT_1
    }
}
#[doc = "Possible values of the field `XCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XCHR {
    #[doc = "Idle."]
    XCH_0,
    #[doc = "Initiates exchange (write) or busy (read)."]
    XCH_1,
}
impl XCHR {
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
            XCHR::XCH_0 => false,
            XCHR::XCH_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XCHR {
        match value {
            false => XCHR::XCH_0,
            true => XCHR::XCH_1,
        }
    }
    #[doc = "Checks if the value of the field is `XCH_0`"]
    #[inline]
    pub fn is_xch_0(&self) -> bool {
        *self == XCHR::XCH_0
    }
    #[doc = "Checks if the value of the field is `XCH_1`"]
    #[inline]
    pub fn is_xch_1(&self) -> bool {
        *self == XCHR::XCH_1
    }
}
#[doc = "Possible values of the field `SMC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMCR {
    #[doc = "SPI Exchange Bit (XCH) controls when a SPI burst can start. Setting the XCH bit will start a SPI burst or multiple bursts. This is controlled by the SPI SS Wave Form Select (SS_CTL). Refer to XCH and SS_CTL descriptions."]
    SMC_0,
    #[doc = "Immediately starts a SPI burst when data is written in TXFIFO."]
    SMC_1,
}
impl SMCR {
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
            SMCR::SMC_0 => false,
            SMCR::SMC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMCR {
        match value {
            false => SMCR::SMC_0,
            true => SMCR::SMC_1,
        }
    }
    #[doc = "Checks if the value of the field is `SMC_0`"]
    #[inline]
    pub fn is_smc_0(&self) -> bool {
        *self == SMCR::SMC_0
    }
    #[doc = "Checks if the value of the field is `SMC_1`"]
    #[inline]
    pub fn is_smc_1(&self) -> bool {
        *self == SMCR::SMC_1
    }
}
#[doc = "Possible values of the field `CHANNEL_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL_MODER {
    #[doc = "Slave mode."]
    CHANNEL_MODE_0,
    #[doc = "Master mode."]
    CHANNEL_MODE_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CHANNEL_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CHANNEL_MODER::CHANNEL_MODE_0 => 0,
            CHANNEL_MODER::CHANNEL_MODE_1 => 1,
            CHANNEL_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CHANNEL_MODER {
        match value {
            0 => CHANNEL_MODER::CHANNEL_MODE_0,
            1 => CHANNEL_MODER::CHANNEL_MODE_1,
            i => CHANNEL_MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL_MODE_0`"]
    #[inline]
    pub fn is_channel_mode_0(&self) -> bool {
        *self == CHANNEL_MODER::CHANNEL_MODE_0
    }
    #[doc = "Checks if the value of the field is `CHANNEL_MODE_1`"]
    #[inline]
    pub fn is_channel_mode_1(&self) -> bool {
        *self == CHANNEL_MODER::CHANNEL_MODE_1
    }
}
#[doc = "Possible values of the field `POST_DIVIDER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POST_DIVIDERR {
    #[doc = "Divide by 1."]
    POST_DIVIDER_0,
    #[doc = "Divide by 2."]
    POST_DIVIDER_1,
    #[doc = "Divide by 4."]
    POST_DIVIDER_2,
    #[doc = "Divide by 2 14 ."]
    POST_DIVIDER_14,
    #[doc = "Divide by 2 15 ."]
    POST_DIVIDER_15,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl POST_DIVIDERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            POST_DIVIDERR::POST_DIVIDER_0 => 0,
            POST_DIVIDERR::POST_DIVIDER_1 => 1,
            POST_DIVIDERR::POST_DIVIDER_2 => 2,
            POST_DIVIDERR::POST_DIVIDER_14 => 14,
            POST_DIVIDERR::POST_DIVIDER_15 => 15,
            POST_DIVIDERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> POST_DIVIDERR {
        match value {
            0 => POST_DIVIDERR::POST_DIVIDER_0,
            1 => POST_DIVIDERR::POST_DIVIDER_1,
            2 => POST_DIVIDERR::POST_DIVIDER_2,
            14 => POST_DIVIDERR::POST_DIVIDER_14,
            15 => POST_DIVIDERR::POST_DIVIDER_15,
            i => POST_DIVIDERR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `POST_DIVIDER_0`"]
    #[inline]
    pub fn is_post_divider_0(&self) -> bool {
        *self == POST_DIVIDERR::POST_DIVIDER_0
    }
    #[doc = "Checks if the value of the field is `POST_DIVIDER_1`"]
    #[inline]
    pub fn is_post_divider_1(&self) -> bool {
        *self == POST_DIVIDERR::POST_DIVIDER_1
    }
    #[doc = "Checks if the value of the field is `POST_DIVIDER_2`"]
    #[inline]
    pub fn is_post_divider_2(&self) -> bool {
        *self == POST_DIVIDERR::POST_DIVIDER_2
    }
    #[doc = "Checks if the value of the field is `POST_DIVIDER_14`"]
    #[inline]
    pub fn is_post_divider_14(&self) -> bool {
        *self == POST_DIVIDERR::POST_DIVIDER_14
    }
    #[doc = "Checks if the value of the field is `POST_DIVIDER_15`"]
    #[inline]
    pub fn is_post_divider_15(&self) -> bool {
        *self == POST_DIVIDERR::POST_DIVIDER_15
    }
}
#[doc = "Possible values of the field `PRE_DIVIDER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRE_DIVIDERR {
    #[doc = "Divide by 1."]
    PRE_DIVIDER_0,
    #[doc = "Divide by 2."]
    PRE_DIVIDER_1,
    #[doc = "Divide by 3."]
    PRE_DIVIDER_2,
    #[doc = "Divide by 14."]
    PRE_DIVIDER_13,
    #[doc = "Divide by 15."]
    PRE_DIVIDER_14,
    #[doc = "Divide by 16."]
    PRE_DIVIDER_15,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRE_DIVIDERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRE_DIVIDERR::PRE_DIVIDER_0 => 0,
            PRE_DIVIDERR::PRE_DIVIDER_1 => 1,
            PRE_DIVIDERR::PRE_DIVIDER_2 => 2,
            PRE_DIVIDERR::PRE_DIVIDER_13 => 13,
            PRE_DIVIDERR::PRE_DIVIDER_14 => 14,
            PRE_DIVIDERR::PRE_DIVIDER_15 => 15,
            PRE_DIVIDERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRE_DIVIDERR {
        match value {
            0 => PRE_DIVIDERR::PRE_DIVIDER_0,
            1 => PRE_DIVIDERR::PRE_DIVIDER_1,
            2 => PRE_DIVIDERR::PRE_DIVIDER_2,
            13 => PRE_DIVIDERR::PRE_DIVIDER_13,
            14 => PRE_DIVIDERR::PRE_DIVIDER_14,
            15 => PRE_DIVIDERR::PRE_DIVIDER_15,
            i => PRE_DIVIDERR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRE_DIVIDER_0`"]
    #[inline]
    pub fn is_pre_divider_0(&self) -> bool {
        *self == PRE_DIVIDERR::PRE_DIVIDER_0
    }
    #[doc = "Checks if the value of the field is `PRE_DIVIDER_1`"]
    #[inline]
    pub fn is_pre_divider_1(&self) -> bool {
        *self == PRE_DIVIDERR::PRE_DIVIDER_1
    }
    #[doc = "Checks if the value of the field is `PRE_DIVIDER_2`"]
    #[inline]
    pub fn is_pre_divider_2(&self) -> bool {
        *self == PRE_DIVIDERR::PRE_DIVIDER_2
    }
    #[doc = "Checks if the value of the field is `PRE_DIVIDER_13`"]
    #[inline]
    pub fn is_pre_divider_13(&self) -> bool {
        *self == PRE_DIVIDERR::PRE_DIVIDER_13
    }
    #[doc = "Checks if the value of the field is `PRE_DIVIDER_14`"]
    #[inline]
    pub fn is_pre_divider_14(&self) -> bool {
        *self == PRE_DIVIDERR::PRE_DIVIDER_14
    }
    #[doc = "Checks if the value of the field is `PRE_DIVIDER_15`"]
    #[inline]
    pub fn is_pre_divider_15(&self) -> bool {
        *self == PRE_DIVIDERR::PRE_DIVIDER_15
    }
}
#[doc = "Possible values of the field `DRCTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRCTLR {
    #[doc = "The SPI_RDY signal is a don't care."]
    DRCTL_0,
    #[doc = "Burst will be triggered by the falling edge of the SPI_RDY signal (edge-triggered)."]
    DRCTL_1,
    #[doc = "Burst will be triggered by a low level of the SPI_RDY signal (level-triggered)."]
    DRCTL_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DRCTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DRCTLR::DRCTL_0 => 0,
            DRCTLR::DRCTL_1 => 1,
            DRCTLR::DRCTL_2 => 2,
            DRCTLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DRCTLR {
        match value {
            0 => DRCTLR::DRCTL_0,
            1 => DRCTLR::DRCTL_1,
            2 => DRCTLR::DRCTL_2,
            i => DRCTLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DRCTL_0`"]
    #[inline]
    pub fn is_drctl_0(&self) -> bool {
        *self == DRCTLR::DRCTL_0
    }
    #[doc = "Checks if the value of the field is `DRCTL_1`"]
    #[inline]
    pub fn is_drctl_1(&self) -> bool {
        *self == DRCTLR::DRCTL_1
    }
    #[doc = "Checks if the value of the field is `DRCTL_2`"]
    #[inline]
    pub fn is_drctl_2(&self) -> bool {
        *self == DRCTLR::DRCTL_2
    }
}
#[doc = "Possible values of the field `CHANNEL_SELECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL_SELECTR {
    #[doc = "Channel 0 is selected. Chip Select 0 (SS0) will be asserted."]
    CHANNEL_SELECT_0,
    #[doc = "Channel 1 is selected. Chip Select 1 (SS1) will be asserted."]
    CHANNEL_SELECT_1,
    #[doc = "Channel 2 is selected. Chip Select 2 (SS2) will be asserted."]
    CHANNEL_SELECT_2,
    #[doc = "Channel 3 is selected. Chip Select 3 (SS3) will be asserted."]
    CHANNEL_SELECT_3,
}
impl CHANNEL_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CHANNEL_SELECTR::CHANNEL_SELECT_0 => 0,
            CHANNEL_SELECTR::CHANNEL_SELECT_1 => 1,
            CHANNEL_SELECTR::CHANNEL_SELECT_2 => 2,
            CHANNEL_SELECTR::CHANNEL_SELECT_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CHANNEL_SELECTR {
        match value {
            0 => CHANNEL_SELECTR::CHANNEL_SELECT_0,
            1 => CHANNEL_SELECTR::CHANNEL_SELECT_1,
            2 => CHANNEL_SELECTR::CHANNEL_SELECT_2,
            3 => CHANNEL_SELECTR::CHANNEL_SELECT_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL_SELECT_0`"]
    #[inline]
    pub fn is_channel_select_0(&self) -> bool {
        *self == CHANNEL_SELECTR::CHANNEL_SELECT_0
    }
    #[doc = "Checks if the value of the field is `CHANNEL_SELECT_1`"]
    #[inline]
    pub fn is_channel_select_1(&self) -> bool {
        *self == CHANNEL_SELECTR::CHANNEL_SELECT_1
    }
    #[doc = "Checks if the value of the field is `CHANNEL_SELECT_2`"]
    #[inline]
    pub fn is_channel_select_2(&self) -> bool {
        *self == CHANNEL_SELECTR::CHANNEL_SELECT_2
    }
    #[doc = "Checks if the value of the field is `CHANNEL_SELECT_3`"]
    #[inline]
    pub fn is_channel_select_3(&self) -> bool {
        *self == CHANNEL_SELECTR::CHANNEL_SELECT_3
    }
}
#[doc = "Possible values of the field `BURST_LENGTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURST_LENGTHR {
    #[doc = "A SPI burst contains the 1 LSB in a word."]
    BURST_LENGTH_0,
    #[doc = "A SPI burst contains the 2 LSB in a word."]
    BURST_LENGTH_1,
    #[doc = "A SPI burst contains the 3 LSB in a word."]
    BURST_LENGTH_2,
    #[doc = "A SPI burst contains all 32 bits in a word."]
    BURST_LENGTH_31,
    #[doc = "A SPI burst contains the 1 LSB in first word and all 32 bits in second word."]
    BURST_LENGTH_32,
    #[doc = "A SPI burst contains the 2 LSB in first word and all 32 bits in second word."]
    BURST_LENGTH_33,
    #[doc = "A SPI burst contains the 31 LSB in first word and 2^7 -1 words."]
    BURST_LENGTH_4094,
    #[doc = "A SPI burst contains 2^7 words."]
    BURST_LENGTH_4095,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl BURST_LENGTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            BURST_LENGTHR::BURST_LENGTH_0 => 0,
            BURST_LENGTHR::BURST_LENGTH_1 => 1,
            BURST_LENGTHR::BURST_LENGTH_2 => 2,
            BURST_LENGTHR::BURST_LENGTH_31 => 31,
            BURST_LENGTHR::BURST_LENGTH_32 => 32,
            BURST_LENGTHR::BURST_LENGTH_33 => 33,
            BURST_LENGTHR::BURST_LENGTH_4094 => 4094,
            BURST_LENGTHR::BURST_LENGTH_4095 => 4095,
            BURST_LENGTHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> BURST_LENGTHR {
        match value {
            0 => BURST_LENGTHR::BURST_LENGTH_0,
            1 => BURST_LENGTHR::BURST_LENGTH_1,
            2 => BURST_LENGTHR::BURST_LENGTH_2,
            31 => BURST_LENGTHR::BURST_LENGTH_31,
            32 => BURST_LENGTHR::BURST_LENGTH_32,
            33 => BURST_LENGTHR::BURST_LENGTH_33,
            4094 => BURST_LENGTHR::BURST_LENGTH_4094,
            4095 => BURST_LENGTHR::BURST_LENGTH_4095,
            i => BURST_LENGTHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BURST_LENGTH_0`"]
    #[inline]
    pub fn is_burst_length_0(&self) -> bool {
        *self == BURST_LENGTHR::BURST_LENGTH_0
    }
    #[doc = "Checks if the value of the field is `BURST_LENGTH_1`"]
    #[inline]
    pub fn is_burst_length_1(&self) -> bool {
        *self == BURST_LENGTHR::BURST_LENGTH_1
    }
    #[doc = "Checks if the value of the field is `BURST_LENGTH_2`"]
    #[inline]
    pub fn is_burst_length_2(&self) -> bool {
        *self == BURST_LENGTHR::BURST_LENGTH_2
    }
    #[doc = "Checks if the value of the field is `BURST_LENGTH_31`"]
    #[inline]
    pub fn is_burst_length_31(&self) -> bool {
        *self == BURST_LENGTHR::BURST_LENGTH_31
    }
    #[doc = "Checks if the value of the field is `BURST_LENGTH_32`"]
    #[inline]
    pub fn is_burst_length_32(&self) -> bool {
        *self == BURST_LENGTHR::BURST_LENGTH_32
    }
    #[doc = "Checks if the value of the field is `BURST_LENGTH_33`"]
    #[inline]
    pub fn is_burst_length_33(&self) -> bool {
        *self == BURST_LENGTHR::BURST_LENGTH_33
    }
    #[doc = "Checks if the value of the field is `BURST_LENGTH_4094`"]
    #[inline]
    pub fn is_burst_length_4094(&self) -> bool {
        *self == BURST_LENGTHR::BURST_LENGTH_4094
    }
    #[doc = "Checks if the value of the field is `BURST_LENGTH_4095`"]
    #[inline]
    pub fn is_burst_length_4095(&self) -> bool {
        *self == BURST_LENGTHR::BURST_LENGTH_4095
    }
}
#[doc = "Values that can be written to the field `EN`"]
pub enum ENW {
    #[doc = "Disable the block."]
    EN_0,
    #[doc = "Enable the block."]
    EN_1,
}
impl ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENW::EN_0 => false,
            ENW::EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the block."]
    #[inline]
    pub fn en_0(self) -> &'a mut W {
        self.variant(ENW::EN_0)
    }
    #[doc = "Enable the block."]
    #[inline]
    pub fn en_1(self) -> &'a mut W {
        self.variant(ENW::EN_1)
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
#[doc = "Values that can be written to the field `HT`"]
pub enum HTW {
    #[doc = "Disable HT mode."]
    HT_0,
    #[doc = "Enable HT mode."]
    HT_1,
}
impl HTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HTW::HT_0 => false,
            HTW::HT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HTW<'a> {
    w: &'a mut W,
}
impl<'a> _HTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable HT mode."]
    #[inline]
    pub fn ht_0(self) -> &'a mut W {
        self.variant(HTW::HT_0)
    }
    #[doc = "Enable HT mode."]
    #[inline]
    pub fn ht_1(self) -> &'a mut W {
        self.variant(HTW::HT_1)
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
#[doc = "Values that can be written to the field `XCH`"]
pub enum XCHW {
    #[doc = "Idle."]
    XCH_0,
    #[doc = "Initiates exchange (write) or busy (read)."]
    XCH_1,
}
impl XCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XCHW::XCH_0 => false,
            XCHW::XCH_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XCHW<'a> {
    w: &'a mut W,
}
impl<'a> _XCHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XCHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Idle."]
    #[inline]
    pub fn xch_0(self) -> &'a mut W {
        self.variant(XCHW::XCH_0)
    }
    #[doc = "Initiates exchange (write) or busy (read)."]
    #[inline]
    pub fn xch_1(self) -> &'a mut W {
        self.variant(XCHW::XCH_1)
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
#[doc = "Values that can be written to the field `SMC`"]
pub enum SMCW {
    #[doc = "SPI Exchange Bit (XCH) controls when a SPI burst can start. Setting the XCH bit will start a SPI burst or multiple bursts. This is controlled by the SPI SS Wave Form Select (SS_CTL). Refer to XCH and SS_CTL descriptions."]
    SMC_0,
    #[doc = "Immediately starts a SPI burst when data is written in TXFIFO."]
    SMC_1,
}
impl SMCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMCW::SMC_0 => false,
            SMCW::SMC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMCW<'a> {
    w: &'a mut W,
}
impl<'a> _SMCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SPI Exchange Bit (XCH) controls when a SPI burst can start. Setting the XCH bit will start a SPI burst or multiple bursts. This is controlled by the SPI SS Wave Form Select (SS_CTL). Refer to XCH and SS_CTL descriptions."]
    #[inline]
    pub fn smc_0(self) -> &'a mut W {
        self.variant(SMCW::SMC_0)
    }
    #[doc = "Immediately starts a SPI burst when data is written in TXFIFO."]
    #[inline]
    pub fn smc_1(self) -> &'a mut W {
        self.variant(SMCW::SMC_1)
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
#[doc = "Values that can be written to the field `CHANNEL_MODE`"]
pub enum CHANNEL_MODEW {
    #[doc = "Slave mode."]
    CHANNEL_MODE_0,
    #[doc = "Master mode."]
    CHANNEL_MODE_1,
}
impl CHANNEL_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CHANNEL_MODEW::CHANNEL_MODE_0 => 0,
            CHANNEL_MODEW::CHANNEL_MODE_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHANNEL_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNEL_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHANNEL_MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Slave mode."]
    #[inline]
    pub fn channel_mode_0(self) -> &'a mut W {
        self.variant(CHANNEL_MODEW::CHANNEL_MODE_0)
    }
    #[doc = "Master mode."]
    #[inline]
    pub fn channel_mode_1(self) -> &'a mut W {
        self.variant(CHANNEL_MODEW::CHANNEL_MODE_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `POST_DIVIDER`"]
pub enum POST_DIVIDERW {
    #[doc = "Divide by 1."]
    POST_DIVIDER_0,
    #[doc = "Divide by 2."]
    POST_DIVIDER_1,
    #[doc = "Divide by 4."]
    POST_DIVIDER_2,
    #[doc = "Divide by 2 14 ."]
    POST_DIVIDER_14,
    #[doc = "Divide by 2 15 ."]
    POST_DIVIDER_15,
}
impl POST_DIVIDERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            POST_DIVIDERW::POST_DIVIDER_0 => 0,
            POST_DIVIDERW::POST_DIVIDER_1 => 1,
            POST_DIVIDERW::POST_DIVIDER_2 => 2,
            POST_DIVIDERW::POST_DIVIDER_14 => 14,
            POST_DIVIDERW::POST_DIVIDER_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POST_DIVIDERW<'a> {
    w: &'a mut W,
}
impl<'a> _POST_DIVIDERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POST_DIVIDERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Divide by 1."]
    #[inline]
    pub fn post_divider_0(self) -> &'a mut W {
        self.variant(POST_DIVIDERW::POST_DIVIDER_0)
    }
    #[doc = "Divide by 2."]
    #[inline]
    pub fn post_divider_1(self) -> &'a mut W {
        self.variant(POST_DIVIDERW::POST_DIVIDER_1)
    }
    #[doc = "Divide by 4."]
    #[inline]
    pub fn post_divider_2(self) -> &'a mut W {
        self.variant(POST_DIVIDERW::POST_DIVIDER_2)
    }
    #[doc = "Divide by 2 14 ."]
    #[inline]
    pub fn post_divider_14(self) -> &'a mut W {
        self.variant(POST_DIVIDERW::POST_DIVIDER_14)
    }
    #[doc = "Divide by 2 15 ."]
    #[inline]
    pub fn post_divider_15(self) -> &'a mut W {
        self.variant(POST_DIVIDERW::POST_DIVIDER_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRE_DIVIDER`"]
pub enum PRE_DIVIDERW {
    #[doc = "Divide by 1."]
    PRE_DIVIDER_0,
    #[doc = "Divide by 2."]
    PRE_DIVIDER_1,
    #[doc = "Divide by 3."]
    PRE_DIVIDER_2,
    #[doc = "Divide by 14."]
    PRE_DIVIDER_13,
    #[doc = "Divide by 15."]
    PRE_DIVIDER_14,
    #[doc = "Divide by 16."]
    PRE_DIVIDER_15,
}
impl PRE_DIVIDERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRE_DIVIDERW::PRE_DIVIDER_0 => 0,
            PRE_DIVIDERW::PRE_DIVIDER_1 => 1,
            PRE_DIVIDERW::PRE_DIVIDER_2 => 2,
            PRE_DIVIDERW::PRE_DIVIDER_13 => 13,
            PRE_DIVIDERW::PRE_DIVIDER_14 => 14,
            PRE_DIVIDERW::PRE_DIVIDER_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRE_DIVIDERW<'a> {
    w: &'a mut W,
}
impl<'a> _PRE_DIVIDERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRE_DIVIDERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Divide by 1."]
    #[inline]
    pub fn pre_divider_0(self) -> &'a mut W {
        self.variant(PRE_DIVIDERW::PRE_DIVIDER_0)
    }
    #[doc = "Divide by 2."]
    #[inline]
    pub fn pre_divider_1(self) -> &'a mut W {
        self.variant(PRE_DIVIDERW::PRE_DIVIDER_1)
    }
    #[doc = "Divide by 3."]
    #[inline]
    pub fn pre_divider_2(self) -> &'a mut W {
        self.variant(PRE_DIVIDERW::PRE_DIVIDER_2)
    }
    #[doc = "Divide by 14."]
    #[inline]
    pub fn pre_divider_13(self) -> &'a mut W {
        self.variant(PRE_DIVIDERW::PRE_DIVIDER_13)
    }
    #[doc = "Divide by 15."]
    #[inline]
    pub fn pre_divider_14(self) -> &'a mut W {
        self.variant(PRE_DIVIDERW::PRE_DIVIDER_14)
    }
    #[doc = "Divide by 16."]
    #[inline]
    pub fn pre_divider_15(self) -> &'a mut W {
        self.variant(PRE_DIVIDERW::PRE_DIVIDER_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DRCTL`"]
pub enum DRCTLW {
    #[doc = "The SPI_RDY signal is a don't care."]
    DRCTL_0,
    #[doc = "Burst will be triggered by the falling edge of the SPI_RDY signal (edge-triggered)."]
    DRCTL_1,
    #[doc = "Burst will be triggered by a low level of the SPI_RDY signal (level-triggered)."]
    DRCTL_2,
}
impl DRCTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DRCTLW::DRCTL_0 => 0,
            DRCTLW::DRCTL_1 => 1,
            DRCTLW::DRCTL_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DRCTLW<'a> {
    w: &'a mut W,
}
impl<'a> _DRCTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DRCTLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The SPI_RDY signal is a don't care."]
    #[inline]
    pub fn drctl_0(self) -> &'a mut W {
        self.variant(DRCTLW::DRCTL_0)
    }
    #[doc = "Burst will be triggered by the falling edge of the SPI_RDY signal (edge-triggered)."]
    #[inline]
    pub fn drctl_1(self) -> &'a mut W {
        self.variant(DRCTLW::DRCTL_1)
    }
    #[doc = "Burst will be triggered by a low level of the SPI_RDY signal (level-triggered)."]
    #[inline]
    pub fn drctl_2(self) -> &'a mut W {
        self.variant(DRCTLW::DRCTL_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHANNEL_SELECT`"]
pub enum CHANNEL_SELECTW {
    #[doc = "Channel 0 is selected. Chip Select 0 (SS0) will be asserted."]
    CHANNEL_SELECT_0,
    #[doc = "Channel 1 is selected. Chip Select 1 (SS1) will be asserted."]
    CHANNEL_SELECT_1,
    #[doc = "Channel 2 is selected. Chip Select 2 (SS2) will be asserted."]
    CHANNEL_SELECT_2,
    #[doc = "Channel 3 is selected. Chip Select 3 (SS3) will be asserted."]
    CHANNEL_SELECT_3,
}
impl CHANNEL_SELECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CHANNEL_SELECTW::CHANNEL_SELECT_0 => 0,
            CHANNEL_SELECTW::CHANNEL_SELECT_1 => 1,
            CHANNEL_SELECTW::CHANNEL_SELECT_2 => 2,
            CHANNEL_SELECTW::CHANNEL_SELECT_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHANNEL_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNEL_SELECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHANNEL_SELECTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Channel 0 is selected. Chip Select 0 (SS0) will be asserted."]
    #[inline]
    pub fn channel_select_0(self) -> &'a mut W {
        self.variant(CHANNEL_SELECTW::CHANNEL_SELECT_0)
    }
    #[doc = "Channel 1 is selected. Chip Select 1 (SS1) will be asserted."]
    #[inline]
    pub fn channel_select_1(self) -> &'a mut W {
        self.variant(CHANNEL_SELECTW::CHANNEL_SELECT_1)
    }
    #[doc = "Channel 2 is selected. Chip Select 2 (SS2) will be asserted."]
    #[inline]
    pub fn channel_select_2(self) -> &'a mut W {
        self.variant(CHANNEL_SELECTW::CHANNEL_SELECT_2)
    }
    #[doc = "Channel 3 is selected. Chip Select 3 (SS3) will be asserted."]
    #[inline]
    pub fn channel_select_3(self) -> &'a mut W {
        self.variant(CHANNEL_SELECTW::CHANNEL_SELECT_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BURST_LENGTH`"]
pub enum BURST_LENGTHW {
    #[doc = "A SPI burst contains the 1 LSB in a word."]
    BURST_LENGTH_0,
    #[doc = "A SPI burst contains the 2 LSB in a word."]
    BURST_LENGTH_1,
    #[doc = "A SPI burst contains the 3 LSB in a word."]
    BURST_LENGTH_2,
    #[doc = "A SPI burst contains all 32 bits in a word."]
    BURST_LENGTH_31,
    #[doc = "A SPI burst contains the 1 LSB in first word and all 32 bits in second word."]
    BURST_LENGTH_32,
    #[doc = "A SPI burst contains the 2 LSB in first word and all 32 bits in second word."]
    BURST_LENGTH_33,
    #[doc = "A SPI burst contains the 31 LSB in first word and 2^7 -1 words."]
    BURST_LENGTH_4094,
    #[doc = "A SPI burst contains 2^7 words."]
    BURST_LENGTH_4095,
}
impl BURST_LENGTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            BURST_LENGTHW::BURST_LENGTH_0 => 0,
            BURST_LENGTHW::BURST_LENGTH_1 => 1,
            BURST_LENGTHW::BURST_LENGTH_2 => 2,
            BURST_LENGTHW::BURST_LENGTH_31 => 31,
            BURST_LENGTHW::BURST_LENGTH_32 => 32,
            BURST_LENGTHW::BURST_LENGTH_33 => 33,
            BURST_LENGTHW::BURST_LENGTH_4094 => 4094,
            BURST_LENGTHW::BURST_LENGTH_4095 => 4095,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BURST_LENGTHW<'a> {
    w: &'a mut W,
}
impl<'a> _BURST_LENGTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BURST_LENGTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "A SPI burst contains the 1 LSB in a word."]
    #[inline]
    pub fn burst_length_0(self) -> &'a mut W {
        self.variant(BURST_LENGTHW::BURST_LENGTH_0)
    }
    #[doc = "A SPI burst contains the 2 LSB in a word."]
    #[inline]
    pub fn burst_length_1(self) -> &'a mut W {
        self.variant(BURST_LENGTHW::BURST_LENGTH_1)
    }
    #[doc = "A SPI burst contains the 3 LSB in a word."]
    #[inline]
    pub fn burst_length_2(self) -> &'a mut W {
        self.variant(BURST_LENGTHW::BURST_LENGTH_2)
    }
    #[doc = "A SPI burst contains all 32 bits in a word."]
    #[inline]
    pub fn burst_length_31(self) -> &'a mut W {
        self.variant(BURST_LENGTHW::BURST_LENGTH_31)
    }
    #[doc = "A SPI burst contains the 1 LSB in first word and all 32 bits in second word."]
    #[inline]
    pub fn burst_length_32(self) -> &'a mut W {
        self.variant(BURST_LENGTHW::BURST_LENGTH_32)
    }
    #[doc = "A SPI burst contains the 2 LSB in first word and all 32 bits in second word."]
    #[inline]
    pub fn burst_length_33(self) -> &'a mut W {
        self.variant(BURST_LENGTHW::BURST_LENGTH_33)
    }
    #[doc = "A SPI burst contains the 31 LSB in first word and 2^7 -1 words."]
    #[inline]
    pub fn burst_length_4094(self) -> &'a mut W {
        self.variant(BURST_LENGTHW::BURST_LENGTH_4094)
    }
    #[doc = "A SPI burst contains 2^7 words."]
    #[inline]
    pub fn burst_length_4095(self) -> &'a mut W {
        self.variant(BURST_LENGTHW::BURST_LENGTH_4095)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 20;
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
    #[doc = "Bit 0 - SPI Block Enable Control"]
    #[inline]
    pub fn en(&self) -> ENR {
        ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Hardware Trigger Enable"]
    #[inline]
    pub fn ht(&self) -> HTR {
        HTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - SPI Exchange Bit"]
    #[inline]
    pub fn xch(&self) -> XCHR {
        XCHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Start Mode Control"]
    #[inline]
    pub fn smc(&self) -> SMCR {
        SMCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:7 - SPI CHANNEL MODE selects the mode for each SPI channel"]
    #[inline]
    pub fn channel_mode(&self) -> CHANNEL_MODER {
        CHANNEL_MODER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - SPI Post Divider"]
    #[inline]
    pub fn post_divider(&self) -> POST_DIVIDERR {
        POST_DIVIDERR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - SPI Pre Divider"]
    #[inline]
    pub fn pre_divider(&self) -> PRE_DIVIDERR {
        PRE_DIVIDERR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - SPI Data Ready Control"]
    #[inline]
    pub fn drctl(&self) -> DRCTLR {
        DRCTLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - SPI CHANNEL SELECT bits"]
    #[inline]
    pub fn channel_select(&self) -> CHANNEL_SELECTR {
        CHANNEL_SELECTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:31 - Burst Length"]
    #[inline]
    pub fn burst_length(&self) -> BURST_LENGTHR {
        BURST_LENGTHR::_from({
            const MASK: u16 = 4095;
            const OFFSET: u8 = 20;
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
    #[doc = "Bit 0 - SPI Block Enable Control"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bit 1 - Hardware Trigger Enable"]
    #[inline]
    pub fn ht(&mut self) -> _HTW {
        _HTW { w: self }
    }
    #[doc = "Bit 2 - SPI Exchange Bit"]
    #[inline]
    pub fn xch(&mut self) -> _XCHW {
        _XCHW { w: self }
    }
    #[doc = "Bit 3 - Start Mode Control"]
    #[inline]
    pub fn smc(&mut self) -> _SMCW {
        _SMCW { w: self }
    }
    #[doc = "Bits 4:7 - SPI CHANNEL MODE selects the mode for each SPI channel"]
    #[inline]
    pub fn channel_mode(&mut self) -> _CHANNEL_MODEW {
        _CHANNEL_MODEW { w: self }
    }
    #[doc = "Bits 8:11 - SPI Post Divider"]
    #[inline]
    pub fn post_divider(&mut self) -> _POST_DIVIDERW {
        _POST_DIVIDERW { w: self }
    }
    #[doc = "Bits 12:15 - SPI Pre Divider"]
    #[inline]
    pub fn pre_divider(&mut self) -> _PRE_DIVIDERW {
        _PRE_DIVIDERW { w: self }
    }
    #[doc = "Bits 16:17 - SPI Data Ready Control"]
    #[inline]
    pub fn drctl(&mut self) -> _DRCTLW {
        _DRCTLW { w: self }
    }
    #[doc = "Bits 18:19 - SPI CHANNEL SELECT bits"]
    #[inline]
    pub fn channel_select(&mut self) -> _CHANNEL_SELECTW {
        _CHANNEL_SELECTW { w: self }
    }
    #[doc = "Bits 20:31 - Burst Length"]
    #[inline]
    pub fn burst_length(&mut self) -> _BURST_LENGTHW {
        _BURST_LENGTHW { w: self }
    }
}
