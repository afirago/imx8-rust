#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL1 {
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
#[doc = "Possible values of the field `GPMI_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPMI_MODER {
    #[doc = "NAND mode."]
    GPMI_MODE_0,
    #[doc = "ATA mode."]
    GPMI_MODE_1,
}
impl GPMI_MODER {
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
            GPMI_MODER::GPMI_MODE_0 => false,
            GPMI_MODER::GPMI_MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPMI_MODER {
        match value {
            false => GPMI_MODER::GPMI_MODE_0,
            true => GPMI_MODER::GPMI_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPMI_MODE_0`"]
    #[inline]
    pub fn is_gpmi_mode_0(&self) -> bool {
        *self == GPMI_MODER::GPMI_MODE_0
    }
    #[doc = "Checks if the value of the field is `GPMI_MODE_1`"]
    #[inline]
    pub fn is_gpmi_mode_1(&self) -> bool {
        *self == GPMI_MODER::GPMI_MODE_1
    }
}
#[doc = r" Value of the field"]
pub struct CAMERA_MODER {
    bits: bool,
}
impl CAMERA_MODER {
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
#[doc = "Possible values of the field `ATA_IRQRDY_POLARITY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATA_IRQRDY_POLARITYR {
    #[doc = "External RDY_BUSY\\[1\\] and RDY_BUSY\\[0\\] pins are ready when low and busy when high."]
    ATA_IRQRDY_POLARITY_0,
    #[doc = "External RDY_BUSY\\[1\\] and RDY_BUSY\\[0\\] pins are ready when high and busy when low."]
    ATA_IRQRDY_POLARITY_1,
}
impl ATA_IRQRDY_POLARITYR {
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
            ATA_IRQRDY_POLARITYR::ATA_IRQRDY_POLARITY_0 => false,
            ATA_IRQRDY_POLARITYR::ATA_IRQRDY_POLARITY_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ATA_IRQRDY_POLARITYR {
        match value {
            false => ATA_IRQRDY_POLARITYR::ATA_IRQRDY_POLARITY_0,
            true => ATA_IRQRDY_POLARITYR::ATA_IRQRDY_POLARITY_1,
        }
    }
    #[doc = "Checks if the value of the field is `ATA_IRQRDY_POLARITY_0`"]
    #[inline]
    pub fn is_ata_irqrdy_polarity_0(&self) -> bool {
        *self == ATA_IRQRDY_POLARITYR::ATA_IRQRDY_POLARITY_0
    }
    #[doc = "Checks if the value of the field is `ATA_IRQRDY_POLARITY_1`"]
    #[inline]
    pub fn is_ata_irqrdy_polarity_1(&self) -> bool {
        *self == ATA_IRQRDY_POLARITYR::ATA_IRQRDY_POLARITY_1
    }
}
#[doc = "Possible values of the field `DEV_RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEV_RESETR {
    #[doc = "NANDF_WP_B pin is held low (asserted)."]
    DEV_RESET_0,
    #[doc = "NANDF_WP_B pin is held high (de-asserted)."]
    DEV_RESET_1,
}
impl DEV_RESETR {
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
            DEV_RESETR::DEV_RESET_0 => false,
            DEV_RESETR::DEV_RESET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEV_RESETR {
        match value {
            false => DEV_RESETR::DEV_RESET_0,
            true => DEV_RESETR::DEV_RESET_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEV_RESET_0`"]
    #[inline]
    pub fn is_dev_reset_0(&self) -> bool {
        *self == DEV_RESETR::DEV_RESET_0
    }
    #[doc = "Checks if the value of the field is `DEV_RESET_1`"]
    #[inline]
    pub fn is_dev_reset_1(&self) -> bool {
        *self == DEV_RESETR::DEV_RESET_1
    }
}
#[doc = r" Value of the field"]
pub struct ABORT_WAIT_FOR_READY_CHANNELR {
    bits: u8,
}
impl ABORT_WAIT_FOR_READY_CHANNELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ABORT_WAIT_REQUESTR {
    bits: bool,
}
impl ABORT_WAIT_REQUESTR {
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
pub struct BURST_ENR {
    bits: bool,
}
impl BURST_ENR {
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
pub struct TIMEOUT_IRQR {
    bits: bool,
}
impl TIMEOUT_IRQR {
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
pub struct DEV_IRQR {
    bits: bool,
}
impl DEV_IRQR {
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
pub struct DMA2ECC_MODER {
    bits: bool,
}
impl DMA2ECC_MODER {
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
pub struct RDN_DELAYR {
    bits: u8,
}
impl RDN_DELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HALF_PERIODR {
    bits: bool,
}
impl HALF_PERIODR {
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
pub struct DLL_ENABLER {
    bits: bool,
}
impl DLL_ENABLER {
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
pub struct BCH_MODER {
    bits: bool,
}
impl BCH_MODER {
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
pub struct GANGED_RDYBUSYR {
    bits: bool,
}
impl GANGED_RDYBUSYR {
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
pub struct TIMEOUT_IRQ_ENR {
    bits: bool,
}
impl TIMEOUT_IRQ_ENR {
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
#[doc = "Possible values of the field `TEST_TRIGGER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEST_TRIGGERR {
    #[doc = "Disable"]
    TEST_TRIGGER_0,
    #[doc = "Enable"]
    TEST_TRIGGER_1,
}
impl TEST_TRIGGERR {
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
            TEST_TRIGGERR::TEST_TRIGGER_0 => false,
            TEST_TRIGGERR::TEST_TRIGGER_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEST_TRIGGERR {
        match value {
            false => TEST_TRIGGERR::TEST_TRIGGER_0,
            true => TEST_TRIGGERR::TEST_TRIGGER_1,
        }
    }
    #[doc = "Checks if the value of the field is `TEST_TRIGGER_0`"]
    #[inline]
    pub fn is_test_trigger_0(&self) -> bool {
        *self == TEST_TRIGGERR::TEST_TRIGGER_0
    }
    #[doc = "Checks if the value of the field is `TEST_TRIGGER_1`"]
    #[inline]
    pub fn is_test_trigger_1(&self) -> bool {
        *self == TEST_TRIGGERR::TEST_TRIGGER_1
    }
}
#[doc = r" Value of the field"]
pub struct WRN_DLY_SELR {
    bits: u8,
}
impl WRN_DLY_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DECOUPLE_CSR {
    bits: bool,
}
impl DECOUPLE_CSR {
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
pub struct SSYNCMODER {
    bits: bool,
}
impl SSYNCMODER {
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
pub struct UPDATE_CSR {
    bits: bool,
}
impl UPDATE_CSR {
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
#[doc = "Possible values of the field `GPMI_CLK_DIV2_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPMI_CLK_DIV2_ENR {
    #[doc = "internal factor-2 clock divider is disabled"]
    GPMI_CLK_DIV2_EN_0,
    #[doc = "internal factor-2 clock divider is enabled."]
    GPMI_CLK_DIV2_EN_1,
}
impl GPMI_CLK_DIV2_ENR {
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
            GPMI_CLK_DIV2_ENR::GPMI_CLK_DIV2_EN_0 => false,
            GPMI_CLK_DIV2_ENR::GPMI_CLK_DIV2_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPMI_CLK_DIV2_ENR {
        match value {
            false => GPMI_CLK_DIV2_ENR::GPMI_CLK_DIV2_EN_0,
            true => GPMI_CLK_DIV2_ENR::GPMI_CLK_DIV2_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPMI_CLK_DIV2_EN_0`"]
    #[inline]
    pub fn is_gpmi_clk_div2_en_0(&self) -> bool {
        *self == GPMI_CLK_DIV2_ENR::GPMI_CLK_DIV2_EN_0
    }
    #[doc = "Checks if the value of the field is `GPMI_CLK_DIV2_EN_1`"]
    #[inline]
    pub fn is_gpmi_clk_div2_en_1(&self) -> bool {
        *self == GPMI_CLK_DIV2_ENR::GPMI_CLK_DIV2_EN_1
    }
}
#[doc = r" Value of the field"]
pub struct TOGGLE_MODER {
    bits: bool,
}
impl TOGGLE_MODER {
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
pub struct WRITE_CLK_STOPR {
    bits: bool,
}
impl WRITE_CLK_STOPR {
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
pub struct SSYNC_CLK_STOPR {
    bits: bool,
}
impl SSYNC_CLK_STOPR {
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
pub struct DEV_CLK_STOPR {
    bits: bool,
}
impl DEV_CLK_STOPR {
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
#[doc = "Values that can be written to the field `GPMI_MODE`"]
pub enum GPMI_MODEW {
    #[doc = "NAND mode."]
    GPMI_MODE_0,
    #[doc = "ATA mode."]
    GPMI_MODE_1,
}
impl GPMI_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPMI_MODEW::GPMI_MODE_0 => false,
            GPMI_MODEW::GPMI_MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPMI_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _GPMI_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPMI_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "NAND mode."]
    #[inline]
    pub fn gpmi_mode_0(self) -> &'a mut W {
        self.variant(GPMI_MODEW::GPMI_MODE_0)
    }
    #[doc = "ATA mode."]
    #[inline]
    pub fn gpmi_mode_1(self) -> &'a mut W {
        self.variant(GPMI_MODEW::GPMI_MODE_1)
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
#[doc = r" Proxy"]
pub struct _CAMERA_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CAMERA_MODEW<'a> {
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
#[doc = "Values that can be written to the field `ATA_IRQRDY_POLARITY`"]
pub enum ATA_IRQRDY_POLARITYW {
    #[doc = "External RDY_BUSY\\[1\\] and RDY_BUSY\\[0\\] pins are ready when low and busy when high."]
    ATA_IRQRDY_POLARITY_0,
    #[doc = "External RDY_BUSY\\[1\\] and RDY_BUSY\\[0\\] pins are ready when high and busy when low."]
    ATA_IRQRDY_POLARITY_1,
}
impl ATA_IRQRDY_POLARITYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ATA_IRQRDY_POLARITYW::ATA_IRQRDY_POLARITY_0 => false,
            ATA_IRQRDY_POLARITYW::ATA_IRQRDY_POLARITY_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ATA_IRQRDY_POLARITYW<'a> {
    w: &'a mut W,
}
impl<'a> _ATA_IRQRDY_POLARITYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ATA_IRQRDY_POLARITYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External RDY_BUSY\\[1\\] and RDY_BUSY\\[0\\] pins are ready when low and busy when high."]
    #[inline]
    pub fn ata_irqrdy_polarity_0(self) -> &'a mut W {
        self.variant(ATA_IRQRDY_POLARITYW::ATA_IRQRDY_POLARITY_0)
    }
    #[doc = "External RDY_BUSY\\[1\\] and RDY_BUSY\\[0\\] pins are ready when high and busy when low."]
    #[inline]
    pub fn ata_irqrdy_polarity_1(self) -> &'a mut W {
        self.variant(ATA_IRQRDY_POLARITYW::ATA_IRQRDY_POLARITY_1)
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
#[doc = "Values that can be written to the field `DEV_RESET`"]
pub enum DEV_RESETW {
    #[doc = "NANDF_WP_B pin is held low (asserted)."]
    DEV_RESET_0,
    #[doc = "NANDF_WP_B pin is held high (de-asserted)."]
    DEV_RESET_1,
}
impl DEV_RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DEV_RESETW::DEV_RESET_0 => false,
            DEV_RESETW::DEV_RESET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEV_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _DEV_RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEV_RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "NANDF_WP_B pin is held low (asserted)."]
    #[inline]
    pub fn dev_reset_0(self) -> &'a mut W {
        self.variant(DEV_RESETW::DEV_RESET_0)
    }
    #[doc = "NANDF_WP_B pin is held high (de-asserted)."]
    #[inline]
    pub fn dev_reset_1(self) -> &'a mut W {
        self.variant(DEV_RESETW::DEV_RESET_1)
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
#[doc = r" Proxy"]
pub struct _ABORT_WAIT_FOR_READY_CHANNELW<'a> {
    w: &'a mut W,
}
impl<'a> _ABORT_WAIT_FOR_READY_CHANNELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ABORT_WAIT_REQUESTW<'a> {
    w: &'a mut W,
}
impl<'a> _ABORT_WAIT_REQUESTW<'a> {
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
pub struct _BURST_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BURST_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _TIMEOUT_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMEOUT_IRQW<'a> {
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
#[doc = r" Proxy"]
pub struct _DEV_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _DEV_IRQW<'a> {
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
pub struct _DMA2ECC_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA2ECC_MODEW<'a> {
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
#[doc = r" Proxy"]
pub struct _RDN_DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _RDN_DELAYW<'a> {
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
#[doc = r" Proxy"]
pub struct _HALF_PERIODW<'a> {
    w: &'a mut W,
}
impl<'a> _HALF_PERIODW<'a> {
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
#[doc = r" Proxy"]
pub struct _DLL_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _DLL_ENABLEW<'a> {
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
#[doc = r" Proxy"]
pub struct _BCH_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _BCH_MODEW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GANGED_RDYBUSYW<'a> {
    w: &'a mut W,
}
impl<'a> _GANGED_RDYBUSYW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TIMEOUT_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMEOUT_IRQ_ENW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TEST_TRIGGER`"]
pub enum TEST_TRIGGERW {
    #[doc = "Disable"]
    TEST_TRIGGER_0,
    #[doc = "Enable"]
    TEST_TRIGGER_1,
}
impl TEST_TRIGGERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TEST_TRIGGERW::TEST_TRIGGER_0 => false,
            TEST_TRIGGERW::TEST_TRIGGER_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TEST_TRIGGERW<'a> {
    w: &'a mut W,
}
impl<'a> _TEST_TRIGGERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEST_TRIGGERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn test_trigger_0(self) -> &'a mut W {
        self.variant(TEST_TRIGGERW::TEST_TRIGGER_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn test_trigger_1(self) -> &'a mut W {
        self.variant(TEST_TRIGGERW::TEST_TRIGGER_1)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WRN_DLY_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _WRN_DLY_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DECOUPLE_CSW<'a> {
    w: &'a mut W,
}
impl<'a> _DECOUPLE_CSW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SSYNCMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SSYNCMODEW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _UPDATE_CSW<'a> {
    w: &'a mut W,
}
impl<'a> _UPDATE_CSW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPMI_CLK_DIV2_EN`"]
pub enum GPMI_CLK_DIV2_ENW {
    #[doc = "internal factor-2 clock divider is disabled"]
    GPMI_CLK_DIV2_EN_0,
    #[doc = "internal factor-2 clock divider is enabled."]
    GPMI_CLK_DIV2_EN_1,
}
impl GPMI_CLK_DIV2_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPMI_CLK_DIV2_ENW::GPMI_CLK_DIV2_EN_0 => false,
            GPMI_CLK_DIV2_ENW::GPMI_CLK_DIV2_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPMI_CLK_DIV2_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _GPMI_CLK_DIV2_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPMI_CLK_DIV2_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "internal factor-2 clock divider is disabled"]
    #[inline]
    pub fn gpmi_clk_div2_en_0(self) -> &'a mut W {
        self.variant(GPMI_CLK_DIV2_ENW::GPMI_CLK_DIV2_EN_0)
    }
    #[doc = "internal factor-2 clock divider is enabled."]
    #[inline]
    pub fn gpmi_clk_div2_en_1(self) -> &'a mut W {
        self.variant(GPMI_CLK_DIV2_ENW::GPMI_CLK_DIV2_EN_1)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TOGGLE_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _TOGGLE_MODEW<'a> {
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
pub struct _WRITE_CLK_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _WRITE_CLK_STOPW<'a> {
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
pub struct _SSYNC_CLK_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _SSYNC_CLK_STOPW<'a> {
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
pub struct _DEV_CLK_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _DEV_CLK_STOPW<'a> {
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
    #[doc = "Bit 0 - ATA mode is only supported on channel zero"]
    #[inline]
    pub fn gpmi_mode(&self) -> GPMI_MODER {
        GPMI_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - When set to 1 and ATA UDMA is enabled the UDMA interface becomes a camera interface."]
    #[inline]
    pub fn camera_mode(&self) -> CAMERA_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAMERA_MODER { bits }
    }
    #[doc = "Bit 2 - For ATA MODE: Note NAND_RDY_BUSY\\[3:2\\] are not affected by this bit"]
    #[inline]
    pub fn ata_irqrdy_polarity(&self) -> ATA_IRQRDY_POLARITYR {
        ATA_IRQRDY_POLARITYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - ENABLED = 0x0 NANDF_WP_B(WPN) pin is held low (asserted)"]
    #[inline]
    pub fn dev_reset(&self) -> DEV_RESETR {
        DEV_RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:6 - Abort a wait for ready command on selected channel"]
    #[inline]
    pub fn abort_wait_for_ready_channel(&self) -> ABORT_WAIT_FOR_READY_CHANNELR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ABORT_WAIT_FOR_READY_CHANNELR { bits }
    }
    #[doc = "Bit 7 - Request to abort \"wait for ready\" command on channel indicated by ABORT_WAIT_FOR_READY_CHANNEL"]
    #[inline]
    pub fn abort_wait_request(&self) -> ABORT_WAIT_REQUESTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ABORT_WAIT_REQUESTR { bits }
    }
    #[doc = "Bit 8 - When set to 1 each DMA request will generate a 4-transfer burst on the APB bus."]
    #[inline]
    pub fn burst_en(&self) -> BURST_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BURST_ENR { bits }
    }
    #[doc = "Bit 9 - This bit is set when a timeout occurs using the Device_Busy_Timeout value. Write 0 to clear."]
    #[inline]
    pub fn timeout_irq(&self) -> TIMEOUT_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TIMEOUT_IRQR { bits }
    }
    #[doc = "Bit 10 - This bit is set when an Interrupt is received from the ATA device. Write 0 to clear."]
    #[inline]
    pub fn dev_irq(&self) -> DEV_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DEV_IRQR { bits }
    }
    #[doc = "Bit 11 - This is mainly for testing HWECC without involving the Nand device"]
    #[inline]
    pub fn dma2ecc_mode(&self) -> DMA2ECC_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA2ECC_MODER { bits }
    }
    #[doc = "Bits 12:15 - This variable is a factor in the calculated delay to apply to the internal read strobe for correct read data sampling"]
    #[inline]
    pub fn rdn_delay(&self) -> RDN_DELAYR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RDN_DELAYR { bits }
    }
    #[doc = "Bit 16 - Set this bit to 1 if the GPMI clock period is greater than 16ns for proper DLL operation"]
    #[inline]
    pub fn half_period(&self) -> HALF_PERIODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HALF_PERIODR { bits }
    }
    #[doc = "Bit 17 - Set this bit to 1 to enable the GPMI DLL"]
    #[inline]
    pub fn dll_enable(&self) -> DLL_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DLL_ENABLER { bits }
    }
    #[doc = "Bit 18 - This bit selects which error correction unit will access GPMI"]
    #[inline]
    pub fn bch_mode(&self) -> BCH_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BCH_MODER { bits }
    }
    #[doc = "Bit 19 - Set this bit to 1 will force all Nand RDY_BUSY inputs to be sourced from (tied to) RDY_BUSY0"]
    #[inline]
    pub fn ganged_rdybusy(&self) -> GANGED_RDYBUSYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GANGED_RDYBUSYR { bits }
    }
    #[doc = "Bit 20 - Setting this bit to '1' will enable timeout IRQ for transfers in ATA mode only, and for WAIT_FOR_READY commands in both ATA and Nand mode"]
    #[inline]
    pub fn timeout_irq_en(&self) -> TIMEOUT_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TIMEOUT_IRQ_ENR { bits }
    }
    #[doc = "Bit 21 - Test Trigger Enable"]
    #[inline]
    pub fn test_trigger(&self) -> TEST_TRIGGERR {
        TEST_TRIGGERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 22:23 - Since the GPMI write strobe (WRN) is a fast clock pin, the delay on this signal can be programmed to match the load on this pin"]
    #[inline]
    pub fn wrn_dly_sel(&self) -> WRN_DLY_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WRN_DLY_SELR { bits }
    }
    #[doc = "Bit 24 - Decouple Chip Select from DMA Channel"]
    #[inline]
    pub fn decouple_cs(&self) -> DECOUPLE_CSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DECOUPLE_CSR { bits }
    }
    #[doc = "Bit 25 - source synchronouse mode 1 or asynchrous mode 0"]
    #[inline]
    pub fn ssyncmode(&self) -> SSYNCMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SSYNCMODER { bits }
    }
    #[doc = "Bit 26 - force the CS value is be updated to external chip select pin, even GPMI is idle."]
    #[inline]
    pub fn update_cs(&self) -> UPDATE_CSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UPDATE_CSR { bits }
    }
    #[doc = "Bit 27 - This bit should be reset to 0 in asynchronous mode"]
    #[inline]
    pub fn gpmi_clk_div2_en(&self) -> GPMI_CLK_DIV2_ENR {
        GPMI_CLK_DIV2_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - enable samsung toggle mode."]
    #[inline]
    pub fn toggle_mode(&self) -> TOGGLE_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TOGGLE_MODER { bits }
    }
    #[doc = "Bit 29 - In onfi source synchronous mode, host may save power during the data write cycles by holding the CLK signal high (i"]
    #[inline]
    pub fn write_clk_stop(&self) -> WRITE_CLK_STOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRITE_CLK_STOPR { bits }
    }
    #[doc = "Bit 30 - set this bit to 1 will stop the source synchronous mode clk."]
    #[inline]
    pub fn ssync_clk_stop(&self) -> SSYNC_CLK_STOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SSYNC_CLK_STOPR { bits }
    }
    #[doc = "Bit 31 - set this bit to 1 will stop gpmi io working clk."]
    #[inline]
    pub fn dev_clk_stop(&self) -> DEV_CLK_STOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DEV_CLK_STOPR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 262148 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - ATA mode is only supported on channel zero"]
    #[inline]
    pub fn gpmi_mode(&mut self) -> _GPMI_MODEW {
        _GPMI_MODEW { w: self }
    }
    #[doc = "Bit 1 - When set to 1 and ATA UDMA is enabled the UDMA interface becomes a camera interface."]
    #[inline]
    pub fn camera_mode(&mut self) -> _CAMERA_MODEW {
        _CAMERA_MODEW { w: self }
    }
    #[doc = "Bit 2 - For ATA MODE: Note NAND_RDY_BUSY\\[3:2\\] are not affected by this bit"]
    #[inline]
    pub fn ata_irqrdy_polarity(&mut self) -> _ATA_IRQRDY_POLARITYW {
        _ATA_IRQRDY_POLARITYW { w: self }
    }
    #[doc = "Bit 3 - ENABLED = 0x0 NANDF_WP_B(WPN) pin is held low (asserted)"]
    #[inline]
    pub fn dev_reset(&mut self) -> _DEV_RESETW {
        _DEV_RESETW { w: self }
    }
    #[doc = "Bits 4:6 - Abort a wait for ready command on selected channel"]
    #[inline]
    pub fn abort_wait_for_ready_channel(&mut self) -> _ABORT_WAIT_FOR_READY_CHANNELW {
        _ABORT_WAIT_FOR_READY_CHANNELW { w: self }
    }
    #[doc = "Bit 7 - Request to abort \"wait for ready\" command on channel indicated by ABORT_WAIT_FOR_READY_CHANNEL"]
    #[inline]
    pub fn abort_wait_request(&mut self) -> _ABORT_WAIT_REQUESTW {
        _ABORT_WAIT_REQUESTW { w: self }
    }
    #[doc = "Bit 8 - When set to 1 each DMA request will generate a 4-transfer burst on the APB bus."]
    #[inline]
    pub fn burst_en(&mut self) -> _BURST_ENW {
        _BURST_ENW { w: self }
    }
    #[doc = "Bit 9 - This bit is set when a timeout occurs using the Device_Busy_Timeout value. Write 0 to clear."]
    #[inline]
    pub fn timeout_irq(&mut self) -> _TIMEOUT_IRQW {
        _TIMEOUT_IRQW { w: self }
    }
    #[doc = "Bit 10 - This bit is set when an Interrupt is received from the ATA device. Write 0 to clear."]
    #[inline]
    pub fn dev_irq(&mut self) -> _DEV_IRQW {
        _DEV_IRQW { w: self }
    }
    #[doc = "Bit 11 - This is mainly for testing HWECC without involving the Nand device"]
    #[inline]
    pub fn dma2ecc_mode(&mut self) -> _DMA2ECC_MODEW {
        _DMA2ECC_MODEW { w: self }
    }
    #[doc = "Bits 12:15 - This variable is a factor in the calculated delay to apply to the internal read strobe for correct read data sampling"]
    #[inline]
    pub fn rdn_delay(&mut self) -> _RDN_DELAYW {
        _RDN_DELAYW { w: self }
    }
    #[doc = "Bit 16 - Set this bit to 1 if the GPMI clock period is greater than 16ns for proper DLL operation"]
    #[inline]
    pub fn half_period(&mut self) -> _HALF_PERIODW {
        _HALF_PERIODW { w: self }
    }
    #[doc = "Bit 17 - Set this bit to 1 to enable the GPMI DLL"]
    #[inline]
    pub fn dll_enable(&mut self) -> _DLL_ENABLEW {
        _DLL_ENABLEW { w: self }
    }
    #[doc = "Bit 18 - This bit selects which error correction unit will access GPMI"]
    #[inline]
    pub fn bch_mode(&mut self) -> _BCH_MODEW {
        _BCH_MODEW { w: self }
    }
    #[doc = "Bit 19 - Set this bit to 1 will force all Nand RDY_BUSY inputs to be sourced from (tied to) RDY_BUSY0"]
    #[inline]
    pub fn ganged_rdybusy(&mut self) -> _GANGED_RDYBUSYW {
        _GANGED_RDYBUSYW { w: self }
    }
    #[doc = "Bit 20 - Setting this bit to '1' will enable timeout IRQ for transfers in ATA mode only, and for WAIT_FOR_READY commands in both ATA and Nand mode"]
    #[inline]
    pub fn timeout_irq_en(&mut self) -> _TIMEOUT_IRQ_ENW {
        _TIMEOUT_IRQ_ENW { w: self }
    }
    #[doc = "Bit 21 - Test Trigger Enable"]
    #[inline]
    pub fn test_trigger(&mut self) -> _TEST_TRIGGERW {
        _TEST_TRIGGERW { w: self }
    }
    #[doc = "Bits 22:23 - Since the GPMI write strobe (WRN) is a fast clock pin, the delay on this signal can be programmed to match the load on this pin"]
    #[inline]
    pub fn wrn_dly_sel(&mut self) -> _WRN_DLY_SELW {
        _WRN_DLY_SELW { w: self }
    }
    #[doc = "Bit 24 - Decouple Chip Select from DMA Channel"]
    #[inline]
    pub fn decouple_cs(&mut self) -> _DECOUPLE_CSW {
        _DECOUPLE_CSW { w: self }
    }
    #[doc = "Bit 25 - source synchronouse mode 1 or asynchrous mode 0"]
    #[inline]
    pub fn ssyncmode(&mut self) -> _SSYNCMODEW {
        _SSYNCMODEW { w: self }
    }
    #[doc = "Bit 26 - force the CS value is be updated to external chip select pin, even GPMI is idle."]
    #[inline]
    pub fn update_cs(&mut self) -> _UPDATE_CSW {
        _UPDATE_CSW { w: self }
    }
    #[doc = "Bit 27 - This bit should be reset to 0 in asynchronous mode"]
    #[inline]
    pub fn gpmi_clk_div2_en(&mut self) -> _GPMI_CLK_DIV2_ENW {
        _GPMI_CLK_DIV2_ENW { w: self }
    }
    #[doc = "Bit 28 - enable samsung toggle mode."]
    #[inline]
    pub fn toggle_mode(&mut self) -> _TOGGLE_MODEW {
        _TOGGLE_MODEW { w: self }
    }
    #[doc = "Bit 29 - In onfi source synchronous mode, host may save power during the data write cycles by holding the CLK signal high (i"]
    #[inline]
    pub fn write_clk_stop(&mut self) -> _WRITE_CLK_STOPW {
        _WRITE_CLK_STOPW { w: self }
    }
    #[doc = "Bit 30 - set this bit to 1 will stop the source synchronous mode clk."]
    #[inline]
    pub fn ssync_clk_stop(&mut self) -> _SSYNC_CLK_STOPW {
        _SSYNC_CLK_STOPW { w: self }
    }
    #[doc = "Bit 31 - set this bit to 1 will stop gpmi io working clk."]
    #[inline]
    pub fn dev_clk_stop(&mut self) -> _DEV_CLK_STOPW {
        _DEV_CLK_STOPW { w: self }
    }
}
