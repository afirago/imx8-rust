#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::VEND_SPEC {
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
#[doc = "Possible values of the field `EXT_DMA_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXT_DMA_ENR {
    #[doc = "In any scenario, uSDHC does not send out external DMA request."]
    EXT_DMA_EN_0,
    #[doc = "When internal DMA is not active, the external DMA request will be sent out."]
    EXT_DMA_EN_1,
}
impl EXT_DMA_ENR {
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
            EXT_DMA_ENR::EXT_DMA_EN_0 => false,
            EXT_DMA_ENR::EXT_DMA_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXT_DMA_ENR {
        match value {
            false => EXT_DMA_ENR::EXT_DMA_EN_0,
            true => EXT_DMA_ENR::EXT_DMA_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EXT_DMA_EN_0`"]
    #[inline]
    pub fn is_ext_dma_en_0(&self) -> bool {
        *self == EXT_DMA_ENR::EXT_DMA_EN_0
    }
    #[doc = "Checks if the value of the field is `EXT_DMA_EN_1`"]
    #[inline]
    pub fn is_ext_dma_en_1(&self) -> bool {
        *self == EXT_DMA_ENR::EXT_DMA_EN_1
    }
}
#[doc = "Possible values of the field `VSELECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VSELECTR {
    #[doc = "Change the voltage to high voltage range, around 3.0 V"]
    VSELECT_0,
    #[doc = "Change the voltage to low voltage range, around 1.8 V"]
    VSELECT_1,
}
impl VSELECTR {
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
            VSELECTR::VSELECT_0 => false,
            VSELECTR::VSELECT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VSELECTR {
        match value {
            false => VSELECTR::VSELECT_0,
            true => VSELECTR::VSELECT_1,
        }
    }
    #[doc = "Checks if the value of the field is `VSELECT_0`"]
    #[inline]
    pub fn is_vselect_0(&self) -> bool {
        *self == VSELECTR::VSELECT_0
    }
    #[doc = "Checks if the value of the field is `VSELECT_1`"]
    #[inline]
    pub fn is_vselect_1(&self) -> bool {
        *self == VSELECTR::VSELECT_1
    }
}
#[doc = "Possible values of the field `CONFLICT_CHK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONFLICT_CHK_ENR {
    #[doc = "Conflict check disable"]
    CONFLICT_CHK_EN_0,
    #[doc = "Conflict check enable"]
    CONFLICT_CHK_EN_1,
}
impl CONFLICT_CHK_ENR {
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
            CONFLICT_CHK_ENR::CONFLICT_CHK_EN_0 => false,
            CONFLICT_CHK_ENR::CONFLICT_CHK_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CONFLICT_CHK_ENR {
        match value {
            false => CONFLICT_CHK_ENR::CONFLICT_CHK_EN_0,
            true => CONFLICT_CHK_ENR::CONFLICT_CHK_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONFLICT_CHK_EN_0`"]
    #[inline]
    pub fn is_conflict_chk_en_0(&self) -> bool {
        *self == CONFLICT_CHK_ENR::CONFLICT_CHK_EN_0
    }
    #[doc = "Checks if the value of the field is `CONFLICT_CHK_EN_1`"]
    #[inline]
    pub fn is_conflict_chk_en_1(&self) -> bool {
        *self == CONFLICT_CHK_ENR::CONFLICT_CHK_EN_1
    }
}
#[doc = "Possible values of the field `AC12_WR_CHKBUSY_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12_WR_CHKBUSY_ENR {
    #[doc = "Do not check busy after auto CMD12 for write data packet"]
    AC12_WR_CHKBUSY_EN_0,
    #[doc = "Check busy after auto CMD12 for write data packet"]
    AC12_WR_CHKBUSY_EN_1,
}
impl AC12_WR_CHKBUSY_ENR {
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
            AC12_WR_CHKBUSY_ENR::AC12_WR_CHKBUSY_EN_0 => false,
            AC12_WR_CHKBUSY_ENR::AC12_WR_CHKBUSY_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AC12_WR_CHKBUSY_ENR {
        match value {
            false => AC12_WR_CHKBUSY_ENR::AC12_WR_CHKBUSY_EN_0,
            true => AC12_WR_CHKBUSY_ENR::AC12_WR_CHKBUSY_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AC12_WR_CHKBUSY_EN_0`"]
    #[inline]
    pub fn is_ac12_wr_chkbusy_en_0(&self) -> bool {
        *self == AC12_WR_CHKBUSY_ENR::AC12_WR_CHKBUSY_EN_0
    }
    #[doc = "Checks if the value of the field is `AC12_WR_CHKBUSY_EN_1`"]
    #[inline]
    pub fn is_ac12_wr_chkbusy_en_1(&self) -> bool {
        *self == AC12_WR_CHKBUSY_ENR::AC12_WR_CHKBUSY_EN_1
    }
}
#[doc = "Possible values of the field `DAT3_CD_POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAT3_CD_POLR {
    #[doc = "Card detected when DATA3 is high."]
    DAT3_CD_POL_0,
    #[doc = "Card detected when DATA3 is low."]
    DAT3_CD_POL_1,
}
impl DAT3_CD_POLR {
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
            DAT3_CD_POLR::DAT3_CD_POL_0 => false,
            DAT3_CD_POLR::DAT3_CD_POL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DAT3_CD_POLR {
        match value {
            false => DAT3_CD_POLR::DAT3_CD_POL_0,
            true => DAT3_CD_POLR::DAT3_CD_POL_1,
        }
    }
    #[doc = "Checks if the value of the field is `DAT3_CD_POL_0`"]
    #[inline]
    pub fn is_dat3_cd_pol_0(&self) -> bool {
        *self == DAT3_CD_POLR::DAT3_CD_POL_0
    }
    #[doc = "Checks if the value of the field is `DAT3_CD_POL_1`"]
    #[inline]
    pub fn is_dat3_cd_pol_1(&self) -> bool {
        *self == DAT3_CD_POLR::DAT3_CD_POL_1
    }
}
#[doc = "Possible values of the field `CD_POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CD_POLR {
    #[doc = "CD_B pin is low active."]
    CD_POL_0,
    #[doc = "CD_B pin is high active."]
    CD_POL_1,
}
impl CD_POLR {
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
            CD_POLR::CD_POL_0 => false,
            CD_POLR::CD_POL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CD_POLR {
        match value {
            false => CD_POLR::CD_POL_0,
            true => CD_POLR::CD_POL_1,
        }
    }
    #[doc = "Checks if the value of the field is `CD_POL_0`"]
    #[inline]
    pub fn is_cd_pol_0(&self) -> bool {
        *self == CD_POLR::CD_POL_0
    }
    #[doc = "Checks if the value of the field is `CD_POL_1`"]
    #[inline]
    pub fn is_cd_pol_1(&self) -> bool {
        *self == CD_POLR::CD_POL_1
    }
}
#[doc = "Possible values of the field `WP_POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP_POLR {
    #[doc = "WP pin is high active."]
    WP_POL_0,
    #[doc = "WP pin is low active."]
    WP_POL_1,
}
impl WP_POLR {
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
            WP_POLR::WP_POL_0 => false,
            WP_POLR::WP_POL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WP_POLR {
        match value {
            false => WP_POLR::WP_POL_0,
            true => WP_POLR::WP_POL_1,
        }
    }
    #[doc = "Checks if the value of the field is `WP_POL_0`"]
    #[inline]
    pub fn is_wp_pol_0(&self) -> bool {
        *self == WP_POLR::WP_POL_0
    }
    #[doc = "Checks if the value of the field is `WP_POL_1`"]
    #[inline]
    pub fn is_wp_pol_1(&self) -> bool {
        *self == WP_POLR::WP_POL_1
    }
}
#[doc = "Possible values of the field `CLKONJ_IN_ABORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKONJ_IN_ABORTR {
    #[doc = "The CLK output is active when sending abort command while data is transmitting even if the internal FIFO is full (for read) or empty (for write)."]
    CLKONJ_IN_ABORT_0,
    #[doc = "The CLK output is inactive when sending abort command while data is transmitting if the internal FIFO is full (for read) or empty (for write)."]
    CLKONJ_IN_ABORT_1,
}
impl CLKONJ_IN_ABORTR {
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
            CLKONJ_IN_ABORTR::CLKONJ_IN_ABORT_0 => false,
            CLKONJ_IN_ABORTR::CLKONJ_IN_ABORT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKONJ_IN_ABORTR {
        match value {
            false => CLKONJ_IN_ABORTR::CLKONJ_IN_ABORT_0,
            true => CLKONJ_IN_ABORTR::CLKONJ_IN_ABORT_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLKONJ_IN_ABORT_0`"]
    #[inline]
    pub fn is_clkonj_in_abort_0(&self) -> bool {
        *self == CLKONJ_IN_ABORTR::CLKONJ_IN_ABORT_0
    }
    #[doc = "Checks if the value of the field is `CLKONJ_IN_ABORT_1`"]
    #[inline]
    pub fn is_clkonj_in_abort_1(&self) -> bool {
        *self == CLKONJ_IN_ABORTR::CLKONJ_IN_ABORT_1
    }
}
#[doc = "Possible values of the field `FRC_SDCLK_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRC_SDCLK_ONR {
    #[doc = "CLK active or inactive is fully controlled by the hardware."]
    FRC_SDCLK_ON_0,
    #[doc = "Force CLK active."]
    FRC_SDCLK_ON_1,
}
impl FRC_SDCLK_ONR {
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
            FRC_SDCLK_ONR::FRC_SDCLK_ON_0 => false,
            FRC_SDCLK_ONR::FRC_SDCLK_ON_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRC_SDCLK_ONR {
        match value {
            false => FRC_SDCLK_ONR::FRC_SDCLK_ON_0,
            true => FRC_SDCLK_ONR::FRC_SDCLK_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRC_SDCLK_ON_0`"]
    #[inline]
    pub fn is_frc_sdclk_on_0(&self) -> bool {
        *self == FRC_SDCLK_ONR::FRC_SDCLK_ON_0
    }
    #[doc = "Checks if the value of the field is `FRC_SDCLK_ON_1`"]
    #[inline]
    pub fn is_frc_sdclk_on_1(&self) -> bool {
        *self == FRC_SDCLK_ONR::FRC_SDCLK_ON_1
    }
}
#[doc = "Possible values of the field `IPG_CLK_SOFT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPG_CLK_SOFT_ENR {
    #[doc = "Gate off the IPG_CLK"]
    IPG_CLK_SOFT_EN_0,
    #[doc = "Enable the IPG_CLK"]
    IPG_CLK_SOFT_EN_1,
}
impl IPG_CLK_SOFT_ENR {
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
            IPG_CLK_SOFT_ENR::IPG_CLK_SOFT_EN_0 => false,
            IPG_CLK_SOFT_ENR::IPG_CLK_SOFT_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IPG_CLK_SOFT_ENR {
        match value {
            false => IPG_CLK_SOFT_ENR::IPG_CLK_SOFT_EN_0,
            true => IPG_CLK_SOFT_ENR::IPG_CLK_SOFT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `IPG_CLK_SOFT_EN_0`"]
    #[inline]
    pub fn is_ipg_clk_soft_en_0(&self) -> bool {
        *self == IPG_CLK_SOFT_ENR::IPG_CLK_SOFT_EN_0
    }
    #[doc = "Checks if the value of the field is `IPG_CLK_SOFT_EN_1`"]
    #[inline]
    pub fn is_ipg_clk_soft_en_1(&self) -> bool {
        *self == IPG_CLK_SOFT_ENR::IPG_CLK_SOFT_EN_1
    }
}
#[doc = "Possible values of the field `HCLK_SOFT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HCLK_SOFT_ENR {
    #[doc = "Gate off the AHB clock."]
    HCLK_SOFT_EN_0,
    #[doc = "Enable the AHB clock."]
    HCLK_SOFT_EN_1,
}
impl HCLK_SOFT_ENR {
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
            HCLK_SOFT_ENR::HCLK_SOFT_EN_0 => false,
            HCLK_SOFT_ENR::HCLK_SOFT_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HCLK_SOFT_ENR {
        match value {
            false => HCLK_SOFT_ENR::HCLK_SOFT_EN_0,
            true => HCLK_SOFT_ENR::HCLK_SOFT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `HCLK_SOFT_EN_0`"]
    #[inline]
    pub fn is_hclk_soft_en_0(&self) -> bool {
        *self == HCLK_SOFT_ENR::HCLK_SOFT_EN_0
    }
    #[doc = "Checks if the value of the field is `HCLK_SOFT_EN_1`"]
    #[inline]
    pub fn is_hclk_soft_en_1(&self) -> bool {
        *self == HCLK_SOFT_ENR::HCLK_SOFT_EN_1
    }
}
#[doc = "Possible values of the field `IPG_PERCLK_SOFT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPG_PERCLK_SOFT_ENR {
    #[doc = "Gate off the IPG_PERCLK"]
    IPG_PERCLK_SOFT_EN_0,
    #[doc = "Enable the IPG_PERCLK"]
    IPG_PERCLK_SOFT_EN_1,
}
impl IPG_PERCLK_SOFT_ENR {
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
            IPG_PERCLK_SOFT_ENR::IPG_PERCLK_SOFT_EN_0 => false,
            IPG_PERCLK_SOFT_ENR::IPG_PERCLK_SOFT_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IPG_PERCLK_SOFT_ENR {
        match value {
            false => IPG_PERCLK_SOFT_ENR::IPG_PERCLK_SOFT_EN_0,
            true => IPG_PERCLK_SOFT_ENR::IPG_PERCLK_SOFT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `IPG_PERCLK_SOFT_EN_0`"]
    #[inline]
    pub fn is_ipg_perclk_soft_en_0(&self) -> bool {
        *self == IPG_PERCLK_SOFT_ENR::IPG_PERCLK_SOFT_EN_0
    }
    #[doc = "Checks if the value of the field is `IPG_PERCLK_SOFT_EN_1`"]
    #[inline]
    pub fn is_ipg_perclk_soft_en_1(&self) -> bool {
        *self == IPG_PERCLK_SOFT_ENR::IPG_PERCLK_SOFT_EN_1
    }
}
#[doc = "Possible values of the field `CARD_CLK_SOFT_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_CLK_SOFT_ENR {
    #[doc = "Gate off the sd_clk"]
    CARD_CLK_SOFT_EN_0,
    #[doc = "Enable the sd_clk"]
    CARD_CLK_SOFT_EN_1,
}
impl CARD_CLK_SOFT_ENR {
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
            CARD_CLK_SOFT_ENR::CARD_CLK_SOFT_EN_0 => false,
            CARD_CLK_SOFT_ENR::CARD_CLK_SOFT_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CARD_CLK_SOFT_ENR {
        match value {
            false => CARD_CLK_SOFT_ENR::CARD_CLK_SOFT_EN_0,
            true => CARD_CLK_SOFT_ENR::CARD_CLK_SOFT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CARD_CLK_SOFT_EN_0`"]
    #[inline]
    pub fn is_card_clk_soft_en_0(&self) -> bool {
        *self == CARD_CLK_SOFT_ENR::CARD_CLK_SOFT_EN_0
    }
    #[doc = "Checks if the value of the field is `CARD_CLK_SOFT_EN_1`"]
    #[inline]
    pub fn is_card_clk_soft_en_1(&self) -> bool {
        *self == CARD_CLK_SOFT_ENR::CARD_CLK_SOFT_EN_1
    }
}
#[doc = "Possible values of the field `CRC_CHK_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_CHK_DISR {
    #[doc = "Check CRC16 for every read data packet and check CRC bits for every write data packet"]
    CRC_CHK_DIS_0,
    #[doc = "Ignore CRC16 check for every read data packet and ignore CRC bits check for every write data packet"]
    CRC_CHK_DIS_1,
}
impl CRC_CHK_DISR {
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
            CRC_CHK_DISR::CRC_CHK_DIS_0 => false,
            CRC_CHK_DISR::CRC_CHK_DIS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRC_CHK_DISR {
        match value {
            false => CRC_CHK_DISR::CRC_CHK_DIS_0,
            true => CRC_CHK_DISR::CRC_CHK_DIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `CRC_CHK_DIS_0`"]
    #[inline]
    pub fn is_crc_chk_dis_0(&self) -> bool {
        *self == CRC_CHK_DISR::CRC_CHK_DIS_0
    }
    #[doc = "Checks if the value of the field is `CRC_CHK_DIS_1`"]
    #[inline]
    pub fn is_crc_chk_dis_1(&self) -> bool {
        *self == CRC_CHK_DISR::CRC_CHK_DIS_1
    }
}
#[doc = r" Value of the field"]
pub struct INT_ST_VALR {
    bits: u8,
}
impl INT_ST_VALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CMD_BYTE_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_BYTE_ENR {
    #[doc = "Disable"]
    CMD_BYTE_EN_0,
    #[doc = "Enable"]
    CMD_BYTE_EN_1,
}
impl CMD_BYTE_ENR {
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
            CMD_BYTE_ENR::CMD_BYTE_EN_0 => false,
            CMD_BYTE_ENR::CMD_BYTE_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMD_BYTE_ENR {
        match value {
            false => CMD_BYTE_ENR::CMD_BYTE_EN_0,
            true => CMD_BYTE_ENR::CMD_BYTE_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CMD_BYTE_EN_0`"]
    #[inline]
    pub fn is_cmd_byte_en_0(&self) -> bool {
        *self == CMD_BYTE_ENR::CMD_BYTE_EN_0
    }
    #[doc = "Checks if the value of the field is `CMD_BYTE_EN_1`"]
    #[inline]
    pub fn is_cmd_byte_en_1(&self) -> bool {
        *self == CMD_BYTE_ENR::CMD_BYTE_EN_1
    }
}
#[doc = "Values that can be written to the field `EXT_DMA_EN`"]
pub enum EXT_DMA_ENW {
    #[doc = "In any scenario, uSDHC does not send out external DMA request."]
    EXT_DMA_EN_0,
    #[doc = "When internal DMA is not active, the external DMA request will be sent out."]
    EXT_DMA_EN_1,
}
impl EXT_DMA_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EXT_DMA_ENW::EXT_DMA_EN_0 => false,
            EXT_DMA_ENW::EXT_DMA_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXT_DMA_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _EXT_DMA_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXT_DMA_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "In any scenario, uSDHC does not send out external DMA request."]
    #[inline]
    pub fn ext_dma_en_0(self) -> &'a mut W {
        self.variant(EXT_DMA_ENW::EXT_DMA_EN_0)
    }
    #[doc = "When internal DMA is not active, the external DMA request will be sent out."]
    #[inline]
    pub fn ext_dma_en_1(self) -> &'a mut W {
        self.variant(EXT_DMA_ENW::EXT_DMA_EN_1)
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
#[doc = "Values that can be written to the field `VSELECT`"]
pub enum VSELECTW {
    #[doc = "Change the voltage to high voltage range, around 3.0 V"]
    VSELECT_0,
    #[doc = "Change the voltage to low voltage range, around 1.8 V"]
    VSELECT_1,
}
impl VSELECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VSELECTW::VSELECT_0 => false,
            VSELECTW::VSELECT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VSELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _VSELECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VSELECTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Change the voltage to high voltage range, around 3.0 V"]
    #[inline]
    pub fn vselect_0(self) -> &'a mut W {
        self.variant(VSELECTW::VSELECT_0)
    }
    #[doc = "Change the voltage to low voltage range, around 1.8 V"]
    #[inline]
    pub fn vselect_1(self) -> &'a mut W {
        self.variant(VSELECTW::VSELECT_1)
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
#[doc = "Values that can be written to the field `CONFLICT_CHK_EN`"]
pub enum CONFLICT_CHK_ENW {
    #[doc = "Conflict check disable"]
    CONFLICT_CHK_EN_0,
    #[doc = "Conflict check enable"]
    CONFLICT_CHK_EN_1,
}
impl CONFLICT_CHK_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CONFLICT_CHK_ENW::CONFLICT_CHK_EN_0 => false,
            CONFLICT_CHK_ENW::CONFLICT_CHK_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CONFLICT_CHK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CONFLICT_CHK_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CONFLICT_CHK_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Conflict check disable"]
    #[inline]
    pub fn conflict_chk_en_0(self) -> &'a mut W {
        self.variant(CONFLICT_CHK_ENW::CONFLICT_CHK_EN_0)
    }
    #[doc = "Conflict check enable"]
    #[inline]
    pub fn conflict_chk_en_1(self) -> &'a mut W {
        self.variant(CONFLICT_CHK_ENW::CONFLICT_CHK_EN_1)
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
#[doc = "Values that can be written to the field `AC12_WR_CHKBUSY_EN`"]
pub enum AC12_WR_CHKBUSY_ENW {
    #[doc = "Do not check busy after auto CMD12 for write data packet"]
    AC12_WR_CHKBUSY_EN_0,
    #[doc = "Check busy after auto CMD12 for write data packet"]
    AC12_WR_CHKBUSY_EN_1,
}
impl AC12_WR_CHKBUSY_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AC12_WR_CHKBUSY_ENW::AC12_WR_CHKBUSY_EN_0 => false,
            AC12_WR_CHKBUSY_ENW::AC12_WR_CHKBUSY_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AC12_WR_CHKBUSY_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _AC12_WR_CHKBUSY_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AC12_WR_CHKBUSY_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not check busy after auto CMD12 for write data packet"]
    #[inline]
    pub fn ac12_wr_chkbusy_en_0(self) -> &'a mut W {
        self.variant(AC12_WR_CHKBUSY_ENW::AC12_WR_CHKBUSY_EN_0)
    }
    #[doc = "Check busy after auto CMD12 for write data packet"]
    #[inline]
    pub fn ac12_wr_chkbusy_en_1(self) -> &'a mut W {
        self.variant(AC12_WR_CHKBUSY_ENW::AC12_WR_CHKBUSY_EN_1)
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
#[doc = "Values that can be written to the field `DAT3_CD_POL`"]
pub enum DAT3_CD_POLW {
    #[doc = "Card detected when DATA3 is high."]
    DAT3_CD_POL_0,
    #[doc = "Card detected when DATA3 is low."]
    DAT3_CD_POL_1,
}
impl DAT3_CD_POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DAT3_CD_POLW::DAT3_CD_POL_0 => false,
            DAT3_CD_POLW::DAT3_CD_POL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DAT3_CD_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _DAT3_CD_POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DAT3_CD_POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Card detected when DATA3 is high."]
    #[inline]
    pub fn dat3_cd_pol_0(self) -> &'a mut W {
        self.variant(DAT3_CD_POLW::DAT3_CD_POL_0)
    }
    #[doc = "Card detected when DATA3 is low."]
    #[inline]
    pub fn dat3_cd_pol_1(self) -> &'a mut W {
        self.variant(DAT3_CD_POLW::DAT3_CD_POL_1)
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
#[doc = "Values that can be written to the field `CD_POL`"]
pub enum CD_POLW {
    #[doc = "CD_B pin is low active."]
    CD_POL_0,
    #[doc = "CD_B pin is high active."]
    CD_POL_1,
}
impl CD_POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CD_POLW::CD_POL_0 => false,
            CD_POLW::CD_POL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CD_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _CD_POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CD_POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CD_B pin is low active."]
    #[inline]
    pub fn cd_pol_0(self) -> &'a mut W {
        self.variant(CD_POLW::CD_POL_0)
    }
    #[doc = "CD_B pin is high active."]
    #[inline]
    pub fn cd_pol_1(self) -> &'a mut W {
        self.variant(CD_POLW::CD_POL_1)
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
#[doc = "Values that can be written to the field `WP_POL`"]
pub enum WP_POLW {
    #[doc = "WP pin is high active."]
    WP_POL_0,
    #[doc = "WP pin is low active."]
    WP_POL_1,
}
impl WP_POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WP_POLW::WP_POL_0 => false,
            WP_POLW::WP_POL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WP_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _WP_POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WP_POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "WP pin is high active."]
    #[inline]
    pub fn wp_pol_0(self) -> &'a mut W {
        self.variant(WP_POLW::WP_POL_0)
    }
    #[doc = "WP pin is low active."]
    #[inline]
    pub fn wp_pol_1(self) -> &'a mut W {
        self.variant(WP_POLW::WP_POL_1)
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
#[doc = "Values that can be written to the field `CLKONJ_IN_ABORT`"]
pub enum CLKONJ_IN_ABORTW {
    #[doc = "The CLK output is active when sending abort command while data is transmitting even if the internal FIFO is full (for read) or empty (for write)."]
    CLKONJ_IN_ABORT_0,
    #[doc = "The CLK output is inactive when sending abort command while data is transmitting if the internal FIFO is full (for read) or empty (for write)."]
    CLKONJ_IN_ABORT_1,
}
impl CLKONJ_IN_ABORTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKONJ_IN_ABORTW::CLKONJ_IN_ABORT_0 => false,
            CLKONJ_IN_ABORTW::CLKONJ_IN_ABORT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKONJ_IN_ABORTW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKONJ_IN_ABORTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKONJ_IN_ABORTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The CLK output is active when sending abort command while data is transmitting even if the internal FIFO is full (for read) or empty (for write)."]
    #[inline]
    pub fn clkonj_in_abort_0(self) -> &'a mut W {
        self.variant(CLKONJ_IN_ABORTW::CLKONJ_IN_ABORT_0)
    }
    #[doc = "The CLK output is inactive when sending abort command while data is transmitting if the internal FIFO is full (for read) or empty (for write)."]
    #[inline]
    pub fn clkonj_in_abort_1(self) -> &'a mut W {
        self.variant(CLKONJ_IN_ABORTW::CLKONJ_IN_ABORT_1)
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
#[doc = "Values that can be written to the field `FRC_SDCLK_ON`"]
pub enum FRC_SDCLK_ONW {
    #[doc = "CLK active or inactive is fully controlled by the hardware."]
    FRC_SDCLK_ON_0,
    #[doc = "Force CLK active."]
    FRC_SDCLK_ON_1,
}
impl FRC_SDCLK_ONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRC_SDCLK_ONW::FRC_SDCLK_ON_0 => false,
            FRC_SDCLK_ONW::FRC_SDCLK_ON_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRC_SDCLK_ONW<'a> {
    w: &'a mut W,
}
impl<'a> _FRC_SDCLK_ONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRC_SDCLK_ONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CLK active or inactive is fully controlled by the hardware."]
    #[inline]
    pub fn frc_sdclk_on_0(self) -> &'a mut W {
        self.variant(FRC_SDCLK_ONW::FRC_SDCLK_ON_0)
    }
    #[doc = "Force CLK active."]
    #[inline]
    pub fn frc_sdclk_on_1(self) -> &'a mut W {
        self.variant(FRC_SDCLK_ONW::FRC_SDCLK_ON_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IPG_CLK_SOFT_EN`"]
pub enum IPG_CLK_SOFT_ENW {
    #[doc = "Gate off the IPG_CLK"]
    IPG_CLK_SOFT_EN_0,
    #[doc = "Enable the IPG_CLK"]
    IPG_CLK_SOFT_EN_1,
}
impl IPG_CLK_SOFT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IPG_CLK_SOFT_ENW::IPG_CLK_SOFT_EN_0 => false,
            IPG_CLK_SOFT_ENW::IPG_CLK_SOFT_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IPG_CLK_SOFT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _IPG_CLK_SOFT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IPG_CLK_SOFT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Gate off the IPG_CLK"]
    #[inline]
    pub fn ipg_clk_soft_en_0(self) -> &'a mut W {
        self.variant(IPG_CLK_SOFT_ENW::IPG_CLK_SOFT_EN_0)
    }
    #[doc = "Enable the IPG_CLK"]
    #[inline]
    pub fn ipg_clk_soft_en_1(self) -> &'a mut W {
        self.variant(IPG_CLK_SOFT_ENW::IPG_CLK_SOFT_EN_1)
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
#[doc = "Values that can be written to the field `HCLK_SOFT_EN`"]
pub enum HCLK_SOFT_ENW {
    #[doc = "Gate off the AHB clock."]
    HCLK_SOFT_EN_0,
    #[doc = "Enable the AHB clock."]
    HCLK_SOFT_EN_1,
}
impl HCLK_SOFT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HCLK_SOFT_ENW::HCLK_SOFT_EN_0 => false,
            HCLK_SOFT_ENW::HCLK_SOFT_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HCLK_SOFT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _HCLK_SOFT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HCLK_SOFT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Gate off the AHB clock."]
    #[inline]
    pub fn hclk_soft_en_0(self) -> &'a mut W {
        self.variant(HCLK_SOFT_ENW::HCLK_SOFT_EN_0)
    }
    #[doc = "Enable the AHB clock."]
    #[inline]
    pub fn hclk_soft_en_1(self) -> &'a mut W {
        self.variant(HCLK_SOFT_ENW::HCLK_SOFT_EN_1)
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
#[doc = "Values that can be written to the field `IPG_PERCLK_SOFT_EN`"]
pub enum IPG_PERCLK_SOFT_ENW {
    #[doc = "Gate off the IPG_PERCLK"]
    IPG_PERCLK_SOFT_EN_0,
    #[doc = "Enable the IPG_PERCLK"]
    IPG_PERCLK_SOFT_EN_1,
}
impl IPG_PERCLK_SOFT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IPG_PERCLK_SOFT_ENW::IPG_PERCLK_SOFT_EN_0 => false,
            IPG_PERCLK_SOFT_ENW::IPG_PERCLK_SOFT_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IPG_PERCLK_SOFT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _IPG_PERCLK_SOFT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IPG_PERCLK_SOFT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Gate off the IPG_PERCLK"]
    #[inline]
    pub fn ipg_perclk_soft_en_0(self) -> &'a mut W {
        self.variant(IPG_PERCLK_SOFT_ENW::IPG_PERCLK_SOFT_EN_0)
    }
    #[doc = "Enable the IPG_PERCLK"]
    #[inline]
    pub fn ipg_perclk_soft_en_1(self) -> &'a mut W {
        self.variant(IPG_PERCLK_SOFT_ENW::IPG_PERCLK_SOFT_EN_1)
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
#[doc = "Values that can be written to the field `CARD_CLK_SOFT_EN`"]
pub enum CARD_CLK_SOFT_ENW {
    #[doc = "Gate off the sd_clk"]
    CARD_CLK_SOFT_EN_0,
    #[doc = "Enable the sd_clk"]
    CARD_CLK_SOFT_EN_1,
}
impl CARD_CLK_SOFT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CARD_CLK_SOFT_ENW::CARD_CLK_SOFT_EN_0 => false,
            CARD_CLK_SOFT_ENW::CARD_CLK_SOFT_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CARD_CLK_SOFT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CARD_CLK_SOFT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CARD_CLK_SOFT_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Gate off the sd_clk"]
    #[inline]
    pub fn card_clk_soft_en_0(self) -> &'a mut W {
        self.variant(CARD_CLK_SOFT_ENW::CARD_CLK_SOFT_EN_0)
    }
    #[doc = "Enable the sd_clk"]
    #[inline]
    pub fn card_clk_soft_en_1(self) -> &'a mut W {
        self.variant(CARD_CLK_SOFT_ENW::CARD_CLK_SOFT_EN_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CRC_CHK_DIS`"]
pub enum CRC_CHK_DISW {
    #[doc = "Check CRC16 for every read data packet and check CRC bits for every write data packet"]
    CRC_CHK_DIS_0,
    #[doc = "Ignore CRC16 check for every read data packet and ignore CRC bits check for every write data packet"]
    CRC_CHK_DIS_1,
}
impl CRC_CHK_DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRC_CHK_DISW::CRC_CHK_DIS_0 => false,
            CRC_CHK_DISW::CRC_CHK_DIS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRC_CHK_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC_CHK_DISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRC_CHK_DISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Check CRC16 for every read data packet and check CRC bits for every write data packet"]
    #[inline]
    pub fn crc_chk_dis_0(self) -> &'a mut W {
        self.variant(CRC_CHK_DISW::CRC_CHK_DIS_0)
    }
    #[doc = "Ignore CRC16 check for every read data packet and ignore CRC bits check for every write data packet"]
    #[inline]
    pub fn crc_chk_dis_1(self) -> &'a mut W {
        self.variant(CRC_CHK_DISW::CRC_CHK_DIS_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMD_BYTE_EN`"]
pub enum CMD_BYTE_ENW {
    #[doc = "Disable"]
    CMD_BYTE_EN_0,
    #[doc = "Enable"]
    CMD_BYTE_EN_1,
}
impl CMD_BYTE_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMD_BYTE_ENW::CMD_BYTE_EN_0 => false,
            CMD_BYTE_ENW::CMD_BYTE_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMD_BYTE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_BYTE_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD_BYTE_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn cmd_byte_en_0(self) -> &'a mut W {
        self.variant(CMD_BYTE_ENW::CMD_BYTE_EN_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn cmd_byte_en_1(self) -> &'a mut W {
        self.variant(CMD_BYTE_ENW::CMD_BYTE_EN_1)
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
    #[doc = "Bit 0 - External DMA Request Enable"]
    #[inline]
    pub fn ext_dma_en(&self) -> EXT_DMA_ENR {
        EXT_DMA_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Voltage Selection"]
    #[inline]
    pub fn vselect(&self) -> VSELECTR {
        VSELECTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Conflict check enable."]
    #[inline]
    pub fn conflict_chk_en(&self) -> CONFLICT_CHK_ENR {
        CONFLICT_CHK_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Check busy enable after auto CMD12 for write data packet"]
    #[inline]
    pub fn ac12_wr_chkbusy_en(&self) -> AC12_WR_CHKBUSY_ENR {
        AC12_WR_CHKBUSY_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Only for debug. Polarity of DATA3 pin when it is used as card detection."]
    #[inline]
    pub fn dat3_cd_pol(&self) -> DAT3_CD_POLR {
        DAT3_CD_POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Only for debug. Polarity of the CD_B pin:"]
    #[inline]
    pub fn cd_pol(&self) -> CD_POLR {
        CD_POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Only for debug. Polarity of the WP pin:"]
    #[inline]
    pub fn wp_pol(&self) -> WP_POLR {
        WP_POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Only for debug. Force CLK output active when sending Abort command:"]
    #[inline]
    pub fn clkonj_in_abort(&self) -> CLKONJ_IN_ABORTR {
        CLKONJ_IN_ABORTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Force CLK output active"]
    #[inline]
    pub fn frc_sdclk_on(&self) -> FRC_SDCLK_ONR {
        FRC_SDCLK_ONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - IPG_CLK Software Enable"]
    #[inline]
    pub fn ipg_clk_soft_en(&self) -> IPG_CLK_SOFT_ENR {
        IPG_CLK_SOFT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - AHB Clock Software Enable"]
    #[inline]
    pub fn hclk_soft_en(&self) -> HCLK_SOFT_ENR {
        HCLK_SOFT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - IPG_PERCLK Software Enable"]
    #[inline]
    pub fn ipg_perclk_soft_en(&self) -> IPG_PERCLK_SOFT_ENR {
        IPG_PERCLK_SOFT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Card Clock Software Enable"]
    #[inline]
    pub fn card_clk_soft_en(&self) -> CARD_CLK_SOFT_ENR {
        CARD_CLK_SOFT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - CRC Check Disable"]
    #[inline]
    pub fn crc_chk_dis(&self) -> CRC_CHK_DISR {
        CRC_CHK_DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:23 - Internal State Value"]
    #[inline]
    pub fn int_st_val(&self) -> INT_ST_VALR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INT_ST_VALR { bits }
    }
    #[doc = "Bit 31 - Byte access"]
    #[inline]
    pub fn cmd_byte_en(&self) -> CMD_BYTE_ENR {
        CMD_BYTE_ENR::_from({
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
        W { bits: 536901641 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - External DMA Request Enable"]
    #[inline]
    pub fn ext_dma_en(&mut self) -> _EXT_DMA_ENW {
        _EXT_DMA_ENW { w: self }
    }
    #[doc = "Bit 1 - Voltage Selection"]
    #[inline]
    pub fn vselect(&mut self) -> _VSELECTW {
        _VSELECTW { w: self }
    }
    #[doc = "Bit 2 - Conflict check enable."]
    #[inline]
    pub fn conflict_chk_en(&mut self) -> _CONFLICT_CHK_ENW {
        _CONFLICT_CHK_ENW { w: self }
    }
    #[doc = "Bit 3 - Check busy enable after auto CMD12 for write data packet"]
    #[inline]
    pub fn ac12_wr_chkbusy_en(&mut self) -> _AC12_WR_CHKBUSY_ENW {
        _AC12_WR_CHKBUSY_ENW { w: self }
    }
    #[doc = "Bit 4 - Only for debug. Polarity of DATA3 pin when it is used as card detection."]
    #[inline]
    pub fn dat3_cd_pol(&mut self) -> _DAT3_CD_POLW {
        _DAT3_CD_POLW { w: self }
    }
    #[doc = "Bit 5 - Only for debug. Polarity of the CD_B pin:"]
    #[inline]
    pub fn cd_pol(&mut self) -> _CD_POLW {
        _CD_POLW { w: self }
    }
    #[doc = "Bit 6 - Only for debug. Polarity of the WP pin:"]
    #[inline]
    pub fn wp_pol(&mut self) -> _WP_POLW {
        _WP_POLW { w: self }
    }
    #[doc = "Bit 7 - Only for debug. Force CLK output active when sending Abort command:"]
    #[inline]
    pub fn clkonj_in_abort(&mut self) -> _CLKONJ_IN_ABORTW {
        _CLKONJ_IN_ABORTW { w: self }
    }
    #[doc = "Bit 8 - Force CLK output active"]
    #[inline]
    pub fn frc_sdclk_on(&mut self) -> _FRC_SDCLK_ONW {
        _FRC_SDCLK_ONW { w: self }
    }
    #[doc = "Bit 11 - IPG_CLK Software Enable"]
    #[inline]
    pub fn ipg_clk_soft_en(&mut self) -> _IPG_CLK_SOFT_ENW {
        _IPG_CLK_SOFT_ENW { w: self }
    }
    #[doc = "Bit 12 - AHB Clock Software Enable"]
    #[inline]
    pub fn hclk_soft_en(&mut self) -> _HCLK_SOFT_ENW {
        _HCLK_SOFT_ENW { w: self }
    }
    #[doc = "Bit 13 - IPG_PERCLK Software Enable"]
    #[inline]
    pub fn ipg_perclk_soft_en(&mut self) -> _IPG_PERCLK_SOFT_ENW {
        _IPG_PERCLK_SOFT_ENW { w: self }
    }
    #[doc = "Bit 14 - Card Clock Software Enable"]
    #[inline]
    pub fn card_clk_soft_en(&mut self) -> _CARD_CLK_SOFT_ENW {
        _CARD_CLK_SOFT_ENW { w: self }
    }
    #[doc = "Bit 15 - CRC Check Disable"]
    #[inline]
    pub fn crc_chk_dis(&mut self) -> _CRC_CHK_DISW {
        _CRC_CHK_DISW { w: self }
    }
    #[doc = "Bit 31 - Byte access"]
    #[inline]
    pub fn cmd_byte_en(&mut self) -> _CMD_BYTE_ENW {
        _CMD_BYTE_ENW { w: self }
    }
}
