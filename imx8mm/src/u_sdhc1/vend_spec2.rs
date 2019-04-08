#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::VEND_SPEC2 {
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
#[doc = "Possible values of the field `SDR104_TIMING_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDR104_TIMING_DISR {
    #[doc = "The timeout counter for Ncr changes to 80, Ncrc changes to 21."]
    SDR104_TIMING_DIS_0,
    #[doc = "The timeout counter for Ncr changes to 72, Ncrc changes to 15."]
    SDR104_TIMING_DIS_1,
}
impl SDR104_TIMING_DISR {
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
            SDR104_TIMING_DISR::SDR104_TIMING_DIS_0 => false,
            SDR104_TIMING_DISR::SDR104_TIMING_DIS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDR104_TIMING_DISR {
        match value {
            false => SDR104_TIMING_DISR::SDR104_TIMING_DIS_0,
            true => SDR104_TIMING_DISR::SDR104_TIMING_DIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `SDR104_TIMING_DIS_0`"]
    #[inline]
    pub fn is_sdr104_timing_dis_0(&self) -> bool {
        *self == SDR104_TIMING_DISR::SDR104_TIMING_DIS_0
    }
    #[doc = "Checks if the value of the field is `SDR104_TIMING_DIS_1`"]
    #[inline]
    pub fn is_sdr104_timing_dis_1(&self) -> bool {
        *self == SDR104_TIMING_DISR::SDR104_TIMING_DIS_1
    }
}
#[doc = "Possible values of the field `SDR104_OE_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDR104_OE_DISR {
    #[doc = "Drive the CMD_OE / DATA_OE for one more clock cycle after the end bit."]
    SDR104_OE_DIS_0,
    #[doc = "Stop to drive the CMD_OE / DATA_OE at once after driving the end bit."]
    SDR104_OE_DIS_1,
}
impl SDR104_OE_DISR {
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
            SDR104_OE_DISR::SDR104_OE_DIS_0 => false,
            SDR104_OE_DISR::SDR104_OE_DIS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDR104_OE_DISR {
        match value {
            false => SDR104_OE_DISR::SDR104_OE_DIS_0,
            true => SDR104_OE_DISR::SDR104_OE_DIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `SDR104_OE_DIS_0`"]
    #[inline]
    pub fn is_sdr104_oe_dis_0(&self) -> bool {
        *self == SDR104_OE_DISR::SDR104_OE_DIS_0
    }
    #[doc = "Checks if the value of the field is `SDR104_OE_DIS_1`"]
    #[inline]
    pub fn is_sdr104_oe_dis_1(&self) -> bool {
        *self == SDR104_OE_DISR::SDR104_OE_DIS_1
    }
}
#[doc = "Possible values of the field `SDR104_NSD_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDR104_NSD_DISR {
    #[doc = "Enable the interrupt window 9 cycles later after the end of the I/O abort command (or CMD12) is sent."]
    SDR104_NSD_DIS_0,
    #[doc = "Enable the interrupt window 5 cycles later after the end of the I/O abort command (or CMD12) is sent."]
    SDR104_NSD_DIS_1,
}
impl SDR104_NSD_DISR {
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
            SDR104_NSD_DISR::SDR104_NSD_DIS_0 => false,
            SDR104_NSD_DISR::SDR104_NSD_DIS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDR104_NSD_DISR {
        match value {
            false => SDR104_NSD_DISR::SDR104_NSD_DIS_0,
            true => SDR104_NSD_DISR::SDR104_NSD_DIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `SDR104_NSD_DIS_0`"]
    #[inline]
    pub fn is_sdr104_nsd_dis_0(&self) -> bool {
        *self == SDR104_NSD_DISR::SDR104_NSD_DIS_0
    }
    #[doc = "Checks if the value of the field is `SDR104_NSD_DIS_1`"]
    #[inline]
    pub fn is_sdr104_nsd_dis_1(&self) -> bool {
        *self == SDR104_NSD_DISR::SDR104_NSD_DIS_1
    }
}
#[doc = "Possible values of the field `CARD_INT_D3_TEST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_INT_D3_TESTR {
    #[doc = "Check the card interrupt only when DATA3 is high."]
    CARD_INT_D3_TEST_0,
    #[doc = "Check the card interrupt by ignoring the status of DATA3."]
    CARD_INT_D3_TEST_1,
}
impl CARD_INT_D3_TESTR {
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
            CARD_INT_D3_TESTR::CARD_INT_D3_TEST_0 => false,
            CARD_INT_D3_TESTR::CARD_INT_D3_TEST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CARD_INT_D3_TESTR {
        match value {
            false => CARD_INT_D3_TESTR::CARD_INT_D3_TEST_0,
            true => CARD_INT_D3_TESTR::CARD_INT_D3_TEST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CARD_INT_D3_TEST_0`"]
    #[inline]
    pub fn is_card_int_d3_test_0(&self) -> bool {
        *self == CARD_INT_D3_TESTR::CARD_INT_D3_TEST_0
    }
    #[doc = "Checks if the value of the field is `CARD_INT_D3_TEST_1`"]
    #[inline]
    pub fn is_card_int_d3_test_1(&self) -> bool {
        *self == CARD_INT_D3_TESTR::CARD_INT_D3_TEST_1
    }
}
#[doc = "Possible values of the field `TUNING_8bit_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TUNING_8BIT_ENR {
    #[doc = "Tuning circuit only checks the DATA\\[3:0\\]."]
    TUNING_8BIT_EN_0,
    #[doc = "Tuning circuit only checks the DATA0."]
    TUNING_8BIT_EN_1,
}
impl TUNING_8BIT_ENR {
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
            TUNING_8BIT_ENR::TUNING_8BIT_EN_0 => false,
            TUNING_8BIT_ENR::TUNING_8BIT_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TUNING_8BIT_ENR {
        match value {
            false => TUNING_8BIT_ENR::TUNING_8BIT_EN_0,
            true => TUNING_8BIT_ENR::TUNING_8BIT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TUNING_8BIT_EN_0`"]
    #[inline]
    pub fn is_tuning_8bit_en_0(&self) -> bool {
        *self == TUNING_8BIT_ENR::TUNING_8BIT_EN_0
    }
    #[doc = "Checks if the value of the field is `TUNING_8BIT_EN_1`"]
    #[inline]
    pub fn is_tuning_8bit_en_1(&self) -> bool {
        *self == TUNING_8BIT_ENR::TUNING_8BIT_EN_1
    }
}
#[doc = r" Value of the field"]
pub struct TUNING_1BIT_ENR {
    bits: bool,
}
impl TUNING_1BIT_ENR {
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
#[doc = "Possible values of the field `TUNING_CMD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TUNING_CMD_ENR {
    #[doc = "Auto tuning circuit does not check the CMD line."]
    TUNING_CMD_EN_0,
    #[doc = "Auto tuning circuit checks the CMD line."]
    TUNING_CMD_EN_1,
}
impl TUNING_CMD_ENR {
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
            TUNING_CMD_ENR::TUNING_CMD_EN_0 => false,
            TUNING_CMD_ENR::TUNING_CMD_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TUNING_CMD_ENR {
        match value {
            false => TUNING_CMD_ENR::TUNING_CMD_EN_0,
            true => TUNING_CMD_ENR::TUNING_CMD_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TUNING_CMD_EN_0`"]
    #[inline]
    pub fn is_tuning_cmd_en_0(&self) -> bool {
        *self == TUNING_CMD_ENR::TUNING_CMD_EN_0
    }
    #[doc = "Checks if the value of the field is `TUNING_CMD_EN_1`"]
    #[inline]
    pub fn is_tuning_cmd_en_1(&self) -> bool {
        *self == TUNING_CMD_ENR::TUNING_CMD_EN_1
    }
}
#[doc = "Possible values of the field `CARD_INT_AUTO_CLR_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_INT_AUTO_CLR_DISR {
    #[doc = "Card interrupt status bit (CINT) can be cleared when Card Interrupt status enable bit is 0."]
    CARD_INT_AUTO_CLR_DIS_0,
    #[doc = "Card interrupt status bit (CINT) can only be cleared by writting a 1 to CINT bit."]
    CARD_INT_AUTO_CLR_DIS_1,
}
impl CARD_INT_AUTO_CLR_DISR {
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
            CARD_INT_AUTO_CLR_DISR::CARD_INT_AUTO_CLR_DIS_0 => false,
            CARD_INT_AUTO_CLR_DISR::CARD_INT_AUTO_CLR_DIS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CARD_INT_AUTO_CLR_DISR {
        match value {
            false => CARD_INT_AUTO_CLR_DISR::CARD_INT_AUTO_CLR_DIS_0,
            true => CARD_INT_AUTO_CLR_DISR::CARD_INT_AUTO_CLR_DIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `CARD_INT_AUTO_CLR_DIS_0`"]
    #[inline]
    pub fn is_card_int_auto_clr_dis_0(&self) -> bool {
        *self == CARD_INT_AUTO_CLR_DISR::CARD_INT_AUTO_CLR_DIS_0
    }
    #[doc = "Checks if the value of the field is `CARD_INT_AUTO_CLR_DIS_1`"]
    #[inline]
    pub fn is_card_int_auto_clr_dis_1(&self) -> bool {
        *self == CARD_INT_AUTO_CLR_DISR::CARD_INT_AUTO_CLR_DIS_1
    }
}
#[doc = r" Value of the field"]
pub struct HS400_WR_CLK_STOP_ENR {
    bits: bool,
}
impl HS400_WR_CLK_STOP_ENR {
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
pub struct HS400_RD_CLK_STOP_ENR {
    bits: bool,
}
impl HS400_RD_CLK_STOP_ENR {
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
#[doc = "Values that can be written to the field `SDR104_TIMING_DIS`"]
pub enum SDR104_TIMING_DISW {
    #[doc = "The timeout counter for Ncr changes to 80, Ncrc changes to 21."]
    SDR104_TIMING_DIS_0,
    #[doc = "The timeout counter for Ncr changes to 72, Ncrc changes to 15."]
    SDR104_TIMING_DIS_1,
}
impl SDR104_TIMING_DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDR104_TIMING_DISW::SDR104_TIMING_DIS_0 => false,
            SDR104_TIMING_DISW::SDR104_TIMING_DIS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDR104_TIMING_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _SDR104_TIMING_DISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDR104_TIMING_DISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The timeout counter for Ncr changes to 80, Ncrc changes to 21."]
    #[inline]
    pub fn sdr104_timing_dis_0(self) -> &'a mut W {
        self.variant(SDR104_TIMING_DISW::SDR104_TIMING_DIS_0)
    }
    #[doc = "The timeout counter for Ncr changes to 72, Ncrc changes to 15."]
    #[inline]
    pub fn sdr104_timing_dis_1(self) -> &'a mut W {
        self.variant(SDR104_TIMING_DISW::SDR104_TIMING_DIS_1)
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
#[doc = "Values that can be written to the field `SDR104_OE_DIS`"]
pub enum SDR104_OE_DISW {
    #[doc = "Drive the CMD_OE / DATA_OE for one more clock cycle after the end bit."]
    SDR104_OE_DIS_0,
    #[doc = "Stop to drive the CMD_OE / DATA_OE at once after driving the end bit."]
    SDR104_OE_DIS_1,
}
impl SDR104_OE_DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDR104_OE_DISW::SDR104_OE_DIS_0 => false,
            SDR104_OE_DISW::SDR104_OE_DIS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDR104_OE_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _SDR104_OE_DISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDR104_OE_DISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Drive the CMD_OE / DATA_OE for one more clock cycle after the end bit."]
    #[inline]
    pub fn sdr104_oe_dis_0(self) -> &'a mut W {
        self.variant(SDR104_OE_DISW::SDR104_OE_DIS_0)
    }
    #[doc = "Stop to drive the CMD_OE / DATA_OE at once after driving the end bit."]
    #[inline]
    pub fn sdr104_oe_dis_1(self) -> &'a mut W {
        self.variant(SDR104_OE_DISW::SDR104_OE_DIS_1)
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
#[doc = "Values that can be written to the field `SDR104_NSD_DIS`"]
pub enum SDR104_NSD_DISW {
    #[doc = "Enable the interrupt window 9 cycles later after the end of the I/O abort command (or CMD12) is sent."]
    SDR104_NSD_DIS_0,
    #[doc = "Enable the interrupt window 5 cycles later after the end of the I/O abort command (or CMD12) is sent."]
    SDR104_NSD_DIS_1,
}
impl SDR104_NSD_DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDR104_NSD_DISW::SDR104_NSD_DIS_0 => false,
            SDR104_NSD_DISW::SDR104_NSD_DIS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDR104_NSD_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _SDR104_NSD_DISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDR104_NSD_DISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable the interrupt window 9 cycles later after the end of the I/O abort command (or CMD12) is sent."]
    #[inline]
    pub fn sdr104_nsd_dis_0(self) -> &'a mut W {
        self.variant(SDR104_NSD_DISW::SDR104_NSD_DIS_0)
    }
    #[doc = "Enable the interrupt window 5 cycles later after the end of the I/O abort command (or CMD12) is sent."]
    #[inline]
    pub fn sdr104_nsd_dis_1(self) -> &'a mut W {
        self.variant(SDR104_NSD_DISW::SDR104_NSD_DIS_1)
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
#[doc = "Values that can be written to the field `CARD_INT_D3_TEST`"]
pub enum CARD_INT_D3_TESTW {
    #[doc = "Check the card interrupt only when DATA3 is high."]
    CARD_INT_D3_TEST_0,
    #[doc = "Check the card interrupt by ignoring the status of DATA3."]
    CARD_INT_D3_TEST_1,
}
impl CARD_INT_D3_TESTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CARD_INT_D3_TESTW::CARD_INT_D3_TEST_0 => false,
            CARD_INT_D3_TESTW::CARD_INT_D3_TEST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CARD_INT_D3_TESTW<'a> {
    w: &'a mut W,
}
impl<'a> _CARD_INT_D3_TESTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CARD_INT_D3_TESTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Check the card interrupt only when DATA3 is high."]
    #[inline]
    pub fn card_int_d3_test_0(self) -> &'a mut W {
        self.variant(CARD_INT_D3_TESTW::CARD_INT_D3_TEST_0)
    }
    #[doc = "Check the card interrupt by ignoring the status of DATA3."]
    #[inline]
    pub fn card_int_d3_test_1(self) -> &'a mut W {
        self.variant(CARD_INT_D3_TESTW::CARD_INT_D3_TEST_1)
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
#[doc = "Values that can be written to the field `TUNING_8bit_EN`"]
pub enum TUNING_8BIT_ENW {
    #[doc = "Tuning circuit only checks the DATA\\[3:0\\]."]
    TUNING_8BIT_EN_0,
    #[doc = "Tuning circuit only checks the DATA0."]
    TUNING_8BIT_EN_1,
}
impl TUNING_8BIT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TUNING_8BIT_ENW::TUNING_8BIT_EN_0 => false,
            TUNING_8BIT_ENW::TUNING_8BIT_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TUNING_8BIT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TUNING_8BIT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TUNING_8BIT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Tuning circuit only checks the DATA\\[3:0\\]."]
    #[inline]
    pub fn tuning_8bit_en_0(self) -> &'a mut W {
        self.variant(TUNING_8BIT_ENW::TUNING_8BIT_EN_0)
    }
    #[doc = "Tuning circuit only checks the DATA0."]
    #[inline]
    pub fn tuning_8bit_en_1(self) -> &'a mut W {
        self.variant(TUNING_8BIT_ENW::TUNING_8BIT_EN_1)
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
pub struct _TUNING_1BIT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TUNING_1BIT_ENW<'a> {
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
#[doc = "Values that can be written to the field `TUNING_CMD_EN`"]
pub enum TUNING_CMD_ENW {
    #[doc = "Auto tuning circuit does not check the CMD line."]
    TUNING_CMD_EN_0,
    #[doc = "Auto tuning circuit checks the CMD line."]
    TUNING_CMD_EN_1,
}
impl TUNING_CMD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TUNING_CMD_ENW::TUNING_CMD_EN_0 => false,
            TUNING_CMD_ENW::TUNING_CMD_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TUNING_CMD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TUNING_CMD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TUNING_CMD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Auto tuning circuit does not check the CMD line."]
    #[inline]
    pub fn tuning_cmd_en_0(self) -> &'a mut W {
        self.variant(TUNING_CMD_ENW::TUNING_CMD_EN_0)
    }
    #[doc = "Auto tuning circuit checks the CMD line."]
    #[inline]
    pub fn tuning_cmd_en_1(self) -> &'a mut W {
        self.variant(TUNING_CMD_ENW::TUNING_CMD_EN_1)
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
#[doc = "Values that can be written to the field `CARD_INT_AUTO_CLR_DIS`"]
pub enum CARD_INT_AUTO_CLR_DISW {
    #[doc = "Card interrupt status bit (CINT) can be cleared when Card Interrupt status enable bit is 0."]
    CARD_INT_AUTO_CLR_DIS_0,
    #[doc = "Card interrupt status bit (CINT) can only be cleared by writting a 1 to CINT bit."]
    CARD_INT_AUTO_CLR_DIS_1,
}
impl CARD_INT_AUTO_CLR_DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CARD_INT_AUTO_CLR_DISW::CARD_INT_AUTO_CLR_DIS_0 => false,
            CARD_INT_AUTO_CLR_DISW::CARD_INT_AUTO_CLR_DIS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CARD_INT_AUTO_CLR_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _CARD_INT_AUTO_CLR_DISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CARD_INT_AUTO_CLR_DISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Card interrupt status bit (CINT) can be cleared when Card Interrupt status enable bit is 0."]
    #[inline]
    pub fn card_int_auto_clr_dis_0(self) -> &'a mut W {
        self.variant(CARD_INT_AUTO_CLR_DISW::CARD_INT_AUTO_CLR_DIS_0)
    }
    #[doc = "Card interrupt status bit (CINT) can only be cleared by writting a 1 to CINT bit."]
    #[inline]
    pub fn card_int_auto_clr_dis_1(self) -> &'a mut W {
        self.variant(CARD_INT_AUTO_CLR_DISW::CARD_INT_AUTO_CLR_DIS_1)
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
pub struct _HS400_WR_CLK_STOP_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _HS400_WR_CLK_STOP_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _HS400_RD_CLK_STOP_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _HS400_RD_CLK_STOP_ENW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Timeout counter test. This bit only uses for debugging."]
    #[inline]
    pub fn sdr104_timing_dis(&self) -> SDR104_TIMING_DISR {
        SDR104_TIMING_DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - CMD_OE / DATA_OE logic generation test. This bit only uses for debugging."]
    #[inline]
    pub fn sdr104_oe_dis(&self) -> SDR104_OE_DISR {
        SDR104_OE_DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Interrupt window after abort command is sent. This bit only uses for debugging."]
    #[inline]
    pub fn sdr104_nsd_dis(&self) -> SDR104_NSD_DISR {
        SDR104_NSD_DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Card Interrupt Detection Test"]
    #[inline]
    pub fn card_int_d3_test(&self) -> CARD_INT_D3_TESTR {
        CARD_INT_D3_TESTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enable the auto tuning circuit to check the DATA\\[7:0\\]"]
    #[inline]
    pub fn tuning_8bit_en(&self) -> TUNING_8BIT_ENR {
        TUNING_8BIT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Enable the auto tuning circuit to check the DATA0 only"]
    #[inline]
    pub fn tuning_1bit_en(&self) -> TUNING_1BIT_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TUNING_1BIT_ENR { bits }
    }
    #[doc = "Bit 6 - Enable the auto tuning circuit to check the CMD line."]
    #[inline]
    pub fn tuning_cmd_en(&self) -> TUNING_CMD_ENR {
        TUNING_CMD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Disable the feature to clear the Card interrupt status bit when Card Interrupt status enable bit is cleared"]
    #[inline]
    pub fn card_int_auto_clr_dis(&self) -> CARD_INT_AUTO_CLR_DISR {
        CARD_INT_AUTO_CLR_DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - HS400 Write Clock Stop Enable"]
    #[inline]
    pub fn hs400_wr_clk_stop_en(&self) -> HS400_WR_CLK_STOP_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HS400_WR_CLK_STOP_ENR { bits }
    }
    #[doc = "Bit 11 - HS400 Read Clock Stop Enable"]
    #[inline]
    pub fn hs400_rd_clk_stop_en(&self) -> HS400_RD_CLK_STOP_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HS400_RD_CLK_STOP_ENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 6 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Timeout counter test. This bit only uses for debugging."]
    #[inline]
    pub fn sdr104_timing_dis(&mut self) -> _SDR104_TIMING_DISW {
        _SDR104_TIMING_DISW { w: self }
    }
    #[doc = "Bit 1 - CMD_OE / DATA_OE logic generation test. This bit only uses for debugging."]
    #[inline]
    pub fn sdr104_oe_dis(&mut self) -> _SDR104_OE_DISW {
        _SDR104_OE_DISW { w: self }
    }
    #[doc = "Bit 2 - Interrupt window after abort command is sent. This bit only uses for debugging."]
    #[inline]
    pub fn sdr104_nsd_dis(&mut self) -> _SDR104_NSD_DISW {
        _SDR104_NSD_DISW { w: self }
    }
    #[doc = "Bit 3 - Card Interrupt Detection Test"]
    #[inline]
    pub fn card_int_d3_test(&mut self) -> _CARD_INT_D3_TESTW {
        _CARD_INT_D3_TESTW { w: self }
    }
    #[doc = "Bit 4 - Enable the auto tuning circuit to check the DATA\\[7:0\\]"]
    #[inline]
    pub fn tuning_8bit_en(&mut self) -> _TUNING_8BIT_ENW {
        _TUNING_8BIT_ENW { w: self }
    }
    #[doc = "Bit 5 - Enable the auto tuning circuit to check the DATA0 only"]
    #[inline]
    pub fn tuning_1bit_en(&mut self) -> _TUNING_1BIT_ENW {
        _TUNING_1BIT_ENW { w: self }
    }
    #[doc = "Bit 6 - Enable the auto tuning circuit to check the CMD line."]
    #[inline]
    pub fn tuning_cmd_en(&mut self) -> _TUNING_CMD_ENW {
        _TUNING_CMD_ENW { w: self }
    }
    #[doc = "Bit 7 - Disable the feature to clear the Card interrupt status bit when Card Interrupt status enable bit is cleared"]
    #[inline]
    pub fn card_int_auto_clr_dis(&mut self) -> _CARD_INT_AUTO_CLR_DISW {
        _CARD_INT_AUTO_CLR_DISW { w: self }
    }
    #[doc = "Bit 10 - HS400 Write Clock Stop Enable"]
    #[inline]
    pub fn hs400_wr_clk_stop_en(&mut self) -> _HS400_WR_CLK_STOP_ENW {
        _HS400_WR_CLK_STOP_ENW { w: self }
    }
    #[doc = "Bit 11 - HS400 Read Clock Stop Enable"]
    #[inline]
    pub fn hs400_rd_clk_stop_en(&mut self) -> _HS400_RD_CLK_STOP_ENW {
        _HS400_RD_CLK_STOP_ENW { w: self }
    }
}
