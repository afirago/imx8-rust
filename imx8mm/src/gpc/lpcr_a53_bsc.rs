#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LPCR_A53_BSC {
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
#[doc = "Possible values of the field `LPM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPM0R {
    #[doc = "Remain in RUN mode"]
    LPM0_0,
    #[doc = "Transfer to WAIT mode"]
    LPM0_1,
    #[doc = "Transfer to STOP mode"]
    LPM0_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LPM0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPM0R::LPM0_0 => 0,
            LPM0R::LPM0_1 => 1,
            LPM0R::LPM0_2 => 2,
            LPM0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPM0R {
        match value {
            0 => LPM0R::LPM0_0,
            1 => LPM0R::LPM0_1,
            2 => LPM0R::LPM0_2,
            i => LPM0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LPM0_0`"]
    #[inline]
    pub fn is_lpm0_0(&self) -> bool {
        *self == LPM0R::LPM0_0
    }
    #[doc = "Checks if the value of the field is `LPM0_1`"]
    #[inline]
    pub fn is_lpm0_1(&self) -> bool {
        *self == LPM0R::LPM0_1
    }
    #[doc = "Checks if the value of the field is `LPM0_2`"]
    #[inline]
    pub fn is_lpm0_2(&self) -> bool {
        *self == LPM0R::LPM0_2
    }
}
#[doc = "Possible values of the field `LPM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPM1R {
    #[doc = "Remain in RUN mode"]
    LPM1_0,
    #[doc = "Transfer to WAIT mode"]
    LPM1_1,
    #[doc = "Transfer to STOP mode"]
    LPM1_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LPM1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPM1R::LPM1_0 => 0,
            LPM1R::LPM1_1 => 1,
            LPM1R::LPM1_2 => 2,
            LPM1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPM1R {
        match value {
            0 => LPM1R::LPM1_0,
            1 => LPM1R::LPM1_1,
            2 => LPM1R::LPM1_2,
            i => LPM1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LPM1_0`"]
    #[inline]
    pub fn is_lpm1_0(&self) -> bool {
        *self == LPM1R::LPM1_0
    }
    #[doc = "Checks if the value of the field is `LPM1_1`"]
    #[inline]
    pub fn is_lpm1_1(&self) -> bool {
        *self == LPM1R::LPM1_1
    }
    #[doc = "Checks if the value of the field is `LPM1_2`"]
    #[inline]
    pub fn is_lpm1_2(&self) -> bool {
        *self == LPM1R::LPM1_2
    }
}
#[doc = "Possible values of the field `MST0_LPM_HSK_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MST0_LPM_HSK_MASKR {
    #[doc = "enable MASTER0 LPM handshake, wait ACK from MASTER0"]
    MST0_LPM_HSK_MASK_0,
    #[doc = "disable MASTER0 LPM handshake, mask ACK from MASTER0"]
    MST0_LPM_HSK_MASK_1,
}
impl MST0_LPM_HSK_MASKR {
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
            MST0_LPM_HSK_MASKR::MST0_LPM_HSK_MASK_0 => false,
            MST0_LPM_HSK_MASKR::MST0_LPM_HSK_MASK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MST0_LPM_HSK_MASKR {
        match value {
            false => MST0_LPM_HSK_MASKR::MST0_LPM_HSK_MASK_0,
            true => MST0_LPM_HSK_MASKR::MST0_LPM_HSK_MASK_1,
        }
    }
    #[doc = "Checks if the value of the field is `MST0_LPM_HSK_MASK_0`"]
    #[inline]
    pub fn is_mst0_lpm_hsk_mask_0(&self) -> bool {
        *self == MST0_LPM_HSK_MASKR::MST0_LPM_HSK_MASK_0
    }
    #[doc = "Checks if the value of the field is `MST0_LPM_HSK_MASK_1`"]
    #[inline]
    pub fn is_mst0_lpm_hsk_mask_1(&self) -> bool {
        *self == MST0_LPM_HSK_MASKR::MST0_LPM_HSK_MASK_1
    }
}
#[doc = "Possible values of the field `MST1_LPM_HSK_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MST1_LPM_HSK_MASKR {
    #[doc = "enable MASTER1 LPM handshake, wait ACK from MASTER1"]
    MST1_LPM_HSK_MASK_0,
    #[doc = "disable MASTER1 LPM handshake, mask ACK from MASTER1"]
    MST1_LPM_HSK_MASK_1,
}
impl MST1_LPM_HSK_MASKR {
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
            MST1_LPM_HSK_MASKR::MST1_LPM_HSK_MASK_0 => false,
            MST1_LPM_HSK_MASKR::MST1_LPM_HSK_MASK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MST1_LPM_HSK_MASKR {
        match value {
            false => MST1_LPM_HSK_MASKR::MST1_LPM_HSK_MASK_0,
            true => MST1_LPM_HSK_MASKR::MST1_LPM_HSK_MASK_1,
        }
    }
    #[doc = "Checks if the value of the field is `MST1_LPM_HSK_MASK_0`"]
    #[inline]
    pub fn is_mst1_lpm_hsk_mask_0(&self) -> bool {
        *self == MST1_LPM_HSK_MASKR::MST1_LPM_HSK_MASK_0
    }
    #[doc = "Checks if the value of the field is `MST1_LPM_HSK_MASK_1`"]
    #[inline]
    pub fn is_mst1_lpm_hsk_mask_1(&self) -> bool {
        *self == MST1_LPM_HSK_MASKR::MST1_LPM_HSK_MASK_1
    }
}
#[doc = "Possible values of the field `MST2_LPM_HSK_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MST2_LPM_HSK_MASKR {
    #[doc = "enable MASTER2 LPM handshake, wait ACK from MASTER2"]
    MST2_LPM_HSK_MASK_0,
    #[doc = "disable MASTER2 LPM handshake, mask ACK from MASTER2"]
    MST2_LPM_HSK_MASK_1,
}
impl MST2_LPM_HSK_MASKR {
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
            MST2_LPM_HSK_MASKR::MST2_LPM_HSK_MASK_0 => false,
            MST2_LPM_HSK_MASKR::MST2_LPM_HSK_MASK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MST2_LPM_HSK_MASKR {
        match value {
            false => MST2_LPM_HSK_MASKR::MST2_LPM_HSK_MASK_0,
            true => MST2_LPM_HSK_MASKR::MST2_LPM_HSK_MASK_1,
        }
    }
    #[doc = "Checks if the value of the field is `MST2_LPM_HSK_MASK_0`"]
    #[inline]
    pub fn is_mst2_lpm_hsk_mask_0(&self) -> bool {
        *self == MST2_LPM_HSK_MASKR::MST2_LPM_HSK_MASK_0
    }
    #[doc = "Checks if the value of the field is `MST2_LPM_HSK_MASK_1`"]
    #[inline]
    pub fn is_mst2_lpm_hsk_mask_1(&self) -> bool {
        *self == MST2_LPM_HSK_MASKR::MST2_LPM_HSK_MASK_1
    }
}
#[doc = "Possible values of the field `CPU_CLK_ON_LPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU_CLK_ON_LPMR {
    #[doc = "A53 clock disabled on wait/stop mode"]
    CPU_CLK_ON_LPM_0,
    #[doc = "A53 clock enabled on wait/stop mode"]
    CPU_CLK_ON_LPM_1,
}
impl CPU_CLK_ON_LPMR {
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
            CPU_CLK_ON_LPMR::CPU_CLK_ON_LPM_0 => false,
            CPU_CLK_ON_LPMR::CPU_CLK_ON_LPM_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPU_CLK_ON_LPMR {
        match value {
            false => CPU_CLK_ON_LPMR::CPU_CLK_ON_LPM_0,
            true => CPU_CLK_ON_LPMR::CPU_CLK_ON_LPM_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPU_CLK_ON_LPM_0`"]
    #[inline]
    pub fn is_cpu_clk_on_lpm_0(&self) -> bool {
        *self == CPU_CLK_ON_LPMR::CPU_CLK_ON_LPM_0
    }
    #[doc = "Checks if the value of the field is `CPU_CLK_ON_LPM_1`"]
    #[inline]
    pub fn is_cpu_clk_on_lpm_1(&self) -> bool {
        *self == CPU_CLK_ON_LPMR::CPU_CLK_ON_LPM_1
    }
}
#[doc = "Possible values of the field `MASK_CORE0_WFI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_CORE0_WFIR {
    #[doc = "WFI for CORE0 is not masked"]
    MASK_CORE0_WFI_0,
    #[doc = "WFI for CORE0 is masked"]
    MASK_CORE0_WFI_1,
}
impl MASK_CORE0_WFIR {
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
            MASK_CORE0_WFIR::MASK_CORE0_WFI_0 => false,
            MASK_CORE0_WFIR::MASK_CORE0_WFI_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_CORE0_WFIR {
        match value {
            false => MASK_CORE0_WFIR::MASK_CORE0_WFI_0,
            true => MASK_CORE0_WFIR::MASK_CORE0_WFI_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_CORE0_WFI_0`"]
    #[inline]
    pub fn is_mask_core0_wfi_0(&self) -> bool {
        *self == MASK_CORE0_WFIR::MASK_CORE0_WFI_0
    }
    #[doc = "Checks if the value of the field is `MASK_CORE0_WFI_1`"]
    #[inline]
    pub fn is_mask_core0_wfi_1(&self) -> bool {
        *self == MASK_CORE0_WFIR::MASK_CORE0_WFI_1
    }
}
#[doc = "Possible values of the field `MASK_CORE1_WFI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_CORE1_WFIR {
    #[doc = "WFI for CORE1 is not masked"]
    MASK_CORE1_WFI_0,
    #[doc = "WFI for CORE1 is masked"]
    MASK_CORE1_WFI_1,
}
impl MASK_CORE1_WFIR {
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
            MASK_CORE1_WFIR::MASK_CORE1_WFI_0 => false,
            MASK_CORE1_WFIR::MASK_CORE1_WFI_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_CORE1_WFIR {
        match value {
            false => MASK_CORE1_WFIR::MASK_CORE1_WFI_0,
            true => MASK_CORE1_WFIR::MASK_CORE1_WFI_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_CORE1_WFI_0`"]
    #[inline]
    pub fn is_mask_core1_wfi_0(&self) -> bool {
        *self == MASK_CORE1_WFIR::MASK_CORE1_WFI_0
    }
    #[doc = "Checks if the value of the field is `MASK_CORE1_WFI_1`"]
    #[inline]
    pub fn is_mask_core1_wfi_1(&self) -> bool {
        *self == MASK_CORE1_WFIR::MASK_CORE1_WFI_1
    }
}
#[doc = "Possible values of the field `MASK_CORE2_WFI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_CORE2_WFIR {
    #[doc = "WFI for CORE2 is not masked"]
    MASK_CORE2_WFI_0,
    #[doc = "WFI for CORE2 is masked"]
    MASK_CORE2_WFI_1,
}
impl MASK_CORE2_WFIR {
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
            MASK_CORE2_WFIR::MASK_CORE2_WFI_0 => false,
            MASK_CORE2_WFIR::MASK_CORE2_WFI_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_CORE2_WFIR {
        match value {
            false => MASK_CORE2_WFIR::MASK_CORE2_WFI_0,
            true => MASK_CORE2_WFIR::MASK_CORE2_WFI_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_CORE2_WFI_0`"]
    #[inline]
    pub fn is_mask_core2_wfi_0(&self) -> bool {
        *self == MASK_CORE2_WFIR::MASK_CORE2_WFI_0
    }
    #[doc = "Checks if the value of the field is `MASK_CORE2_WFI_1`"]
    #[inline]
    pub fn is_mask_core2_wfi_1(&self) -> bool {
        *self == MASK_CORE2_WFIR::MASK_CORE2_WFI_1
    }
}
#[doc = "Possible values of the field `MASK_CORE3_WFI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_CORE3_WFIR {
    #[doc = "WFI for CORE3 is not masked"]
    MASK_CORE3_WFI_0,
    #[doc = "WFI for CORE3 is masked"]
    MASK_CORE3_WFI_1,
}
impl MASK_CORE3_WFIR {
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
            MASK_CORE3_WFIR::MASK_CORE3_WFI_0 => false,
            MASK_CORE3_WFIR::MASK_CORE3_WFI_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_CORE3_WFIR {
        match value {
            false => MASK_CORE3_WFIR::MASK_CORE3_WFI_0,
            true => MASK_CORE3_WFIR::MASK_CORE3_WFI_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_CORE3_WFI_0`"]
    #[inline]
    pub fn is_mask_core3_wfi_0(&self) -> bool {
        *self == MASK_CORE3_WFIR::MASK_CORE3_WFI_0
    }
    #[doc = "Checks if the value of the field is `MASK_CORE3_WFI_1`"]
    #[inline]
    pub fn is_mask_core3_wfi_1(&self) -> bool {
        *self == MASK_CORE3_WFIR::MASK_CORE3_WFI_1
    }
}
#[doc = "Possible values of the field `IRQ_SRC_C2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQ_SRC_C2R {
    #[doc = "core2 wakeup source from external INT\\[127:0\\], masked by IMR1. See Power Up Process for A53 Platform for more specific information."]
    IRQ_SRC_C2_0,
    #[doc = "core2 wakeup source from external GIC(nFIQ\\[1\\]/nIRQ\\[1\\]), SCU should not be powered down during low power mode when this bit is set to 1'b1."]
    IRQ_SRC_C2_1,
}
impl IRQ_SRC_C2R {
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
            IRQ_SRC_C2R::IRQ_SRC_C2_0 => false,
            IRQ_SRC_C2R::IRQ_SRC_C2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRQ_SRC_C2R {
        match value {
            false => IRQ_SRC_C2R::IRQ_SRC_C2_0,
            true => IRQ_SRC_C2R::IRQ_SRC_C2_1,
        }
    }
    #[doc = "Checks if the value of the field is `IRQ_SRC_C2_0`"]
    #[inline]
    pub fn is_irq_src_c2_0(&self) -> bool {
        *self == IRQ_SRC_C2R::IRQ_SRC_C2_0
    }
    #[doc = "Checks if the value of the field is `IRQ_SRC_C2_1`"]
    #[inline]
    pub fn is_irq_src_c2_1(&self) -> bool {
        *self == IRQ_SRC_C2R::IRQ_SRC_C2_1
    }
}
#[doc = "Possible values of the field `IRQ_SRC_C3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQ_SRC_C3R {
    #[doc = "core3 wakeup source from external INT\\[127:0\\], masked by IMR1. See Power Up Process for A53 Platform for more specific information."]
    IRQ_SRC_C3_0,
    #[doc = "core3 wakeup source from external GIC(nFIQ\\[1\\]/nIRQ\\[1\\]), SCU should not be powered down during low power mode when this bit is set to 1'b1."]
    IRQ_SRC_C3_1,
}
impl IRQ_SRC_C3R {
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
            IRQ_SRC_C3R::IRQ_SRC_C3_0 => false,
            IRQ_SRC_C3R::IRQ_SRC_C3_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRQ_SRC_C3R {
        match value {
            false => IRQ_SRC_C3R::IRQ_SRC_C3_0,
            true => IRQ_SRC_C3R::IRQ_SRC_C3_1,
        }
    }
    #[doc = "Checks if the value of the field is `IRQ_SRC_C3_0`"]
    #[inline]
    pub fn is_irq_src_c3_0(&self) -> bool {
        *self == IRQ_SRC_C3R::IRQ_SRC_C3_0
    }
    #[doc = "Checks if the value of the field is `IRQ_SRC_C3_1`"]
    #[inline]
    pub fn is_irq_src_c3_1(&self) -> bool {
        *self == IRQ_SRC_C3R::IRQ_SRC_C3_1
    }
}
#[doc = "Possible values of the field `MASK_SCU_WFI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_SCU_WFIR {
    #[doc = "WFI for SCU is not masked"]
    MASK_SCU_WFI_0,
    #[doc = "WFI for SCU is masked"]
    MASK_SCU_WFI_1,
}
impl MASK_SCU_WFIR {
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
            MASK_SCU_WFIR::MASK_SCU_WFI_0 => false,
            MASK_SCU_WFIR::MASK_SCU_WFI_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_SCU_WFIR {
        match value {
            false => MASK_SCU_WFIR::MASK_SCU_WFI_0,
            true => MASK_SCU_WFIR::MASK_SCU_WFI_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_SCU_WFI_0`"]
    #[inline]
    pub fn is_mask_scu_wfi_0(&self) -> bool {
        *self == MASK_SCU_WFIR::MASK_SCU_WFI_0
    }
    #[doc = "Checks if the value of the field is `MASK_SCU_WFI_1`"]
    #[inline]
    pub fn is_mask_scu_wfi_1(&self) -> bool {
        *self == MASK_SCU_WFIR::MASK_SCU_WFI_1
    }
}
#[doc = "Possible values of the field `MASK_L2CC_WFI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_L2CC_WFIR {
    #[doc = "WFI for L2 cache controller is not masked"]
    MASK_L2CC_WFI_0,
    #[doc = "WFI for L2 cache controller is masked"]
    MASK_L2CC_WFI_1,
}
impl MASK_L2CC_WFIR {
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
            MASK_L2CC_WFIR::MASK_L2CC_WFI_0 => false,
            MASK_L2CC_WFIR::MASK_L2CC_WFI_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_L2CC_WFIR {
        match value {
            false => MASK_L2CC_WFIR::MASK_L2CC_WFI_0,
            true => MASK_L2CC_WFIR::MASK_L2CC_WFI_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_L2CC_WFI_0`"]
    #[inline]
    pub fn is_mask_l2cc_wfi_0(&self) -> bool {
        *self == MASK_L2CC_WFIR::MASK_L2CC_WFI_0
    }
    #[doc = "Checks if the value of the field is `MASK_L2CC_WFI_1`"]
    #[inline]
    pub fn is_mask_l2cc_wfi_1(&self) -> bool {
        *self == MASK_L2CC_WFIR::MASK_L2CC_WFI_1
    }
}
#[doc = "Possible values of the field `IRQ_SRC_C0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQ_SRC_C0R {
    #[doc = "core0 wakeup source from external INT\\[127:0\\], masked by IMR0 refer to \"Power up process for A53 platform\" for more specific information"]
    IRQ_SRC_C0_0,
    #[doc = "core0 wakeup source from GIC(nFIQ\\[0\\]/nIRQ\\[0\\] ), SCU should not be power down during low power mode when this bit is set to 1'b1"]
    IRQ_SRC_C0_1,
}
impl IRQ_SRC_C0R {
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
            IRQ_SRC_C0R::IRQ_SRC_C0_0 => false,
            IRQ_SRC_C0R::IRQ_SRC_C0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRQ_SRC_C0R {
        match value {
            false => IRQ_SRC_C0R::IRQ_SRC_C0_0,
            true => IRQ_SRC_C0R::IRQ_SRC_C0_1,
        }
    }
    #[doc = "Checks if the value of the field is `IRQ_SRC_C0_0`"]
    #[inline]
    pub fn is_irq_src_c0_0(&self) -> bool {
        *self == IRQ_SRC_C0R::IRQ_SRC_C0_0
    }
    #[doc = "Checks if the value of the field is `IRQ_SRC_C0_1`"]
    #[inline]
    pub fn is_irq_src_c0_1(&self) -> bool {
        *self == IRQ_SRC_C0R::IRQ_SRC_C0_1
    }
}
#[doc = "Possible values of the field `IRQ_SRC_C1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQ_SRC_C1R {
    #[doc = "core1 wakeup source from external INT\\[127:0\\], masked by IMR1 refer to \"Power up process for A53 platform\" for more specific information"]
    IRQ_SRC_C1_0,
    #[doc = "core1 wakeup source from GIC(nFIQ\\[1\\]/nIRQ\\[1\\] ), SCU should not be power down during low power mode when this bit is set to 1'b1"]
    IRQ_SRC_C1_1,
}
impl IRQ_SRC_C1R {
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
            IRQ_SRC_C1R::IRQ_SRC_C1_0 => false,
            IRQ_SRC_C1R::IRQ_SRC_C1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRQ_SRC_C1R {
        match value {
            false => IRQ_SRC_C1R::IRQ_SRC_C1_0,
            true => IRQ_SRC_C1R::IRQ_SRC_C1_1,
        }
    }
    #[doc = "Checks if the value of the field is `IRQ_SRC_C1_0`"]
    #[inline]
    pub fn is_irq_src_c1_0(&self) -> bool {
        *self == IRQ_SRC_C1R::IRQ_SRC_C1_0
    }
    #[doc = "Checks if the value of the field is `IRQ_SRC_C1_1`"]
    #[inline]
    pub fn is_irq_src_c1_1(&self) -> bool {
        *self == IRQ_SRC_C1R::IRQ_SRC_C1_1
    }
}
#[doc = "Possible values of the field `IRQ_SRC_A53_WUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQ_SRC_A53_WUPR {
    #[doc = "LPM wakeup source be \"OR\" result of LPCR_A53_BSC\\[IRQ_SRC_C0\\]/LPCR_A53_BSC\\[IRQ_SRC_C1\\]/LPCR_A53_BSC\\[IRQ_SRC_C2\\]/LPCR_A53_BSC\\[IRQ_SRC_C3\\] setting"]
    IRQ_SRC_A53_WUP_0,
    #[doc = "LPM wakeup source from external INT\\[127:0\\], masked by IMR0"]
    IRQ_SRC_A53_WUP_1,
}
impl IRQ_SRC_A53_WUPR {
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
            IRQ_SRC_A53_WUPR::IRQ_SRC_A53_WUP_0 => false,
            IRQ_SRC_A53_WUPR::IRQ_SRC_A53_WUP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRQ_SRC_A53_WUPR {
        match value {
            false => IRQ_SRC_A53_WUPR::IRQ_SRC_A53_WUP_0,
            true => IRQ_SRC_A53_WUPR::IRQ_SRC_A53_WUP_1,
        }
    }
    #[doc = "Checks if the value of the field is `IRQ_SRC_A53_WUP_0`"]
    #[inline]
    pub fn is_irq_src_a53_wup_0(&self) -> bool {
        *self == IRQ_SRC_A53_WUPR::IRQ_SRC_A53_WUP_0
    }
    #[doc = "Checks if the value of the field is `IRQ_SRC_A53_WUP_1`"]
    #[inline]
    pub fn is_irq_src_a53_wup_1(&self) -> bool {
        *self == IRQ_SRC_A53_WUPR::IRQ_SRC_A53_WUP_1
    }
}
#[doc = "Possible values of the field `MASK_DSM_TRIGGER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_DSM_TRIGGERR {
    #[doc = "DSM trigger of A53 platform will not be masked"]
    MASK_DSM_TRIGGER_0,
    #[doc = "DSM trigger of A53 platform will be masked"]
    MASK_DSM_TRIGGER_1,
}
impl MASK_DSM_TRIGGERR {
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
            MASK_DSM_TRIGGERR::MASK_DSM_TRIGGER_0 => false,
            MASK_DSM_TRIGGERR::MASK_DSM_TRIGGER_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_DSM_TRIGGERR {
        match value {
            false => MASK_DSM_TRIGGERR::MASK_DSM_TRIGGER_0,
            true => MASK_DSM_TRIGGERR::MASK_DSM_TRIGGER_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_DSM_TRIGGER_0`"]
    #[inline]
    pub fn is_mask_dsm_trigger_0(&self) -> bool {
        *self == MASK_DSM_TRIGGERR::MASK_DSM_TRIGGER_0
    }
    #[doc = "Checks if the value of the field is `MASK_DSM_TRIGGER_1`"]
    #[inline]
    pub fn is_mask_dsm_trigger_1(&self) -> bool {
        *self == MASK_DSM_TRIGGERR::MASK_DSM_TRIGGER_1
    }
}
#[doc = "Values that can be written to the field `LPM0`"]
pub enum LPM0W {
    #[doc = "Remain in RUN mode"]
    LPM0_0,
    #[doc = "Transfer to WAIT mode"]
    LPM0_1,
    #[doc = "Transfer to STOP mode"]
    LPM0_2,
}
impl LPM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPM0W::LPM0_0 => 0,
            LPM0W::LPM0_1 => 1,
            LPM0W::LPM0_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPM0W<'a> {
    w: &'a mut W,
}
impl<'a> _LPM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPM0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Remain in RUN mode"]
    #[inline]
    pub fn lpm0_0(self) -> &'a mut W {
        self.variant(LPM0W::LPM0_0)
    }
    #[doc = "Transfer to WAIT mode"]
    #[inline]
    pub fn lpm0_1(self) -> &'a mut W {
        self.variant(LPM0W::LPM0_1)
    }
    #[doc = "Transfer to STOP mode"]
    #[inline]
    pub fn lpm0_2(self) -> &'a mut W {
        self.variant(LPM0W::LPM0_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPM1`"]
pub enum LPM1W {
    #[doc = "Remain in RUN mode"]
    LPM1_0,
    #[doc = "Transfer to WAIT mode"]
    LPM1_1,
    #[doc = "Transfer to STOP mode"]
    LPM1_2,
}
impl LPM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPM1W::LPM1_0 => 0,
            LPM1W::LPM1_1 => 1,
            LPM1W::LPM1_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPM1W<'a> {
    w: &'a mut W,
}
impl<'a> _LPM1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPM1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Remain in RUN mode"]
    #[inline]
    pub fn lpm1_0(self) -> &'a mut W {
        self.variant(LPM1W::LPM1_0)
    }
    #[doc = "Transfer to WAIT mode"]
    #[inline]
    pub fn lpm1_1(self) -> &'a mut W {
        self.variant(LPM1W::LPM1_1)
    }
    #[doc = "Transfer to STOP mode"]
    #[inline]
    pub fn lpm1_2(self) -> &'a mut W {
        self.variant(LPM1W::LPM1_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MST0_LPM_HSK_MASK`"]
pub enum MST0_LPM_HSK_MASKW {
    #[doc = "enable MASTER0 LPM handshake, wait ACK from MASTER0"]
    MST0_LPM_HSK_MASK_0,
    #[doc = "disable MASTER0 LPM handshake, mask ACK from MASTER0"]
    MST0_LPM_HSK_MASK_1,
}
impl MST0_LPM_HSK_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MST0_LPM_HSK_MASKW::MST0_LPM_HSK_MASK_0 => false,
            MST0_LPM_HSK_MASKW::MST0_LPM_HSK_MASK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MST0_LPM_HSK_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _MST0_LPM_HSK_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MST0_LPM_HSK_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "enable MASTER0 LPM handshake, wait ACK from MASTER0"]
    #[inline]
    pub fn mst0_lpm_hsk_mask_0(self) -> &'a mut W {
        self.variant(MST0_LPM_HSK_MASKW::MST0_LPM_HSK_MASK_0)
    }
    #[doc = "disable MASTER0 LPM handshake, mask ACK from MASTER0"]
    #[inline]
    pub fn mst0_lpm_hsk_mask_1(self) -> &'a mut W {
        self.variant(MST0_LPM_HSK_MASKW::MST0_LPM_HSK_MASK_1)
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
#[doc = "Values that can be written to the field `MST1_LPM_HSK_MASK`"]
pub enum MST1_LPM_HSK_MASKW {
    #[doc = "enable MASTER1 LPM handshake, wait ACK from MASTER1"]
    MST1_LPM_HSK_MASK_0,
    #[doc = "disable MASTER1 LPM handshake, mask ACK from MASTER1"]
    MST1_LPM_HSK_MASK_1,
}
impl MST1_LPM_HSK_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MST1_LPM_HSK_MASKW::MST1_LPM_HSK_MASK_0 => false,
            MST1_LPM_HSK_MASKW::MST1_LPM_HSK_MASK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MST1_LPM_HSK_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _MST1_LPM_HSK_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MST1_LPM_HSK_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "enable MASTER1 LPM handshake, wait ACK from MASTER1"]
    #[inline]
    pub fn mst1_lpm_hsk_mask_0(self) -> &'a mut W {
        self.variant(MST1_LPM_HSK_MASKW::MST1_LPM_HSK_MASK_0)
    }
    #[doc = "disable MASTER1 LPM handshake, mask ACK from MASTER1"]
    #[inline]
    pub fn mst1_lpm_hsk_mask_1(self) -> &'a mut W {
        self.variant(MST1_LPM_HSK_MASKW::MST1_LPM_HSK_MASK_1)
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
#[doc = "Values that can be written to the field `MST2_LPM_HSK_MASK`"]
pub enum MST2_LPM_HSK_MASKW {
    #[doc = "enable MASTER2 LPM handshake, wait ACK from MASTER2"]
    MST2_LPM_HSK_MASK_0,
    #[doc = "disable MASTER2 LPM handshake, mask ACK from MASTER2"]
    MST2_LPM_HSK_MASK_1,
}
impl MST2_LPM_HSK_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MST2_LPM_HSK_MASKW::MST2_LPM_HSK_MASK_0 => false,
            MST2_LPM_HSK_MASKW::MST2_LPM_HSK_MASK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MST2_LPM_HSK_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _MST2_LPM_HSK_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MST2_LPM_HSK_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "enable MASTER2 LPM handshake, wait ACK from MASTER2"]
    #[inline]
    pub fn mst2_lpm_hsk_mask_0(self) -> &'a mut W {
        self.variant(MST2_LPM_HSK_MASKW::MST2_LPM_HSK_MASK_0)
    }
    #[doc = "disable MASTER2 LPM handshake, mask ACK from MASTER2"]
    #[inline]
    pub fn mst2_lpm_hsk_mask_1(self) -> &'a mut W {
        self.variant(MST2_LPM_HSK_MASKW::MST2_LPM_HSK_MASK_1)
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
#[doc = "Values that can be written to the field `CPU_CLK_ON_LPM`"]
pub enum CPU_CLK_ON_LPMW {
    #[doc = "A53 clock disabled on wait/stop mode"]
    CPU_CLK_ON_LPM_0,
    #[doc = "A53 clock enabled on wait/stop mode"]
    CPU_CLK_ON_LPM_1,
}
impl CPU_CLK_ON_LPMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPU_CLK_ON_LPMW::CPU_CLK_ON_LPM_0 => false,
            CPU_CLK_ON_LPMW::CPU_CLK_ON_LPM_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPU_CLK_ON_LPMW<'a> {
    w: &'a mut W,
}
impl<'a> _CPU_CLK_ON_LPMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPU_CLK_ON_LPMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A53 clock disabled on wait/stop mode"]
    #[inline]
    pub fn cpu_clk_on_lpm_0(self) -> &'a mut W {
        self.variant(CPU_CLK_ON_LPMW::CPU_CLK_ON_LPM_0)
    }
    #[doc = "A53 clock enabled on wait/stop mode"]
    #[inline]
    pub fn cpu_clk_on_lpm_1(self) -> &'a mut W {
        self.variant(CPU_CLK_ON_LPMW::CPU_CLK_ON_LPM_1)
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
#[doc = "Values that can be written to the field `MASK_CORE0_WFI`"]
pub enum MASK_CORE0_WFIW {
    #[doc = "WFI for CORE0 is not masked"]
    MASK_CORE0_WFI_0,
    #[doc = "WFI for CORE0 is masked"]
    MASK_CORE0_WFI_1,
}
impl MASK_CORE0_WFIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASK_CORE0_WFIW::MASK_CORE0_WFI_0 => false,
            MASK_CORE0_WFIW::MASK_CORE0_WFI_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_CORE0_WFIW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_CORE0_WFIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_CORE0_WFIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "WFI for CORE0 is not masked"]
    #[inline]
    pub fn mask_core0_wfi_0(self) -> &'a mut W {
        self.variant(MASK_CORE0_WFIW::MASK_CORE0_WFI_0)
    }
    #[doc = "WFI for CORE0 is masked"]
    #[inline]
    pub fn mask_core0_wfi_1(self) -> &'a mut W {
        self.variant(MASK_CORE0_WFIW::MASK_CORE0_WFI_1)
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
#[doc = "Values that can be written to the field `MASK_CORE1_WFI`"]
pub enum MASK_CORE1_WFIW {
    #[doc = "WFI for CORE1 is not masked"]
    MASK_CORE1_WFI_0,
    #[doc = "WFI for CORE1 is masked"]
    MASK_CORE1_WFI_1,
}
impl MASK_CORE1_WFIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASK_CORE1_WFIW::MASK_CORE1_WFI_0 => false,
            MASK_CORE1_WFIW::MASK_CORE1_WFI_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_CORE1_WFIW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_CORE1_WFIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_CORE1_WFIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "WFI for CORE1 is not masked"]
    #[inline]
    pub fn mask_core1_wfi_0(self) -> &'a mut W {
        self.variant(MASK_CORE1_WFIW::MASK_CORE1_WFI_0)
    }
    #[doc = "WFI for CORE1 is masked"]
    #[inline]
    pub fn mask_core1_wfi_1(self) -> &'a mut W {
        self.variant(MASK_CORE1_WFIW::MASK_CORE1_WFI_1)
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
#[doc = "Values that can be written to the field `MASK_CORE2_WFI`"]
pub enum MASK_CORE2_WFIW {
    #[doc = "WFI for CORE2 is not masked"]
    MASK_CORE2_WFI_0,
    #[doc = "WFI for CORE2 is masked"]
    MASK_CORE2_WFI_1,
}
impl MASK_CORE2_WFIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASK_CORE2_WFIW::MASK_CORE2_WFI_0 => false,
            MASK_CORE2_WFIW::MASK_CORE2_WFI_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_CORE2_WFIW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_CORE2_WFIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_CORE2_WFIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "WFI for CORE2 is not masked"]
    #[inline]
    pub fn mask_core2_wfi_0(self) -> &'a mut W {
        self.variant(MASK_CORE2_WFIW::MASK_CORE2_WFI_0)
    }
    #[doc = "WFI for CORE2 is masked"]
    #[inline]
    pub fn mask_core2_wfi_1(self) -> &'a mut W {
        self.variant(MASK_CORE2_WFIW::MASK_CORE2_WFI_1)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MASK_CORE3_WFI`"]
pub enum MASK_CORE3_WFIW {
    #[doc = "WFI for CORE3 is not masked"]
    MASK_CORE3_WFI_0,
    #[doc = "WFI for CORE3 is masked"]
    MASK_CORE3_WFI_1,
}
impl MASK_CORE3_WFIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASK_CORE3_WFIW::MASK_CORE3_WFI_0 => false,
            MASK_CORE3_WFIW::MASK_CORE3_WFI_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_CORE3_WFIW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_CORE3_WFIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_CORE3_WFIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "WFI for CORE3 is not masked"]
    #[inline]
    pub fn mask_core3_wfi_0(self) -> &'a mut W {
        self.variant(MASK_CORE3_WFIW::MASK_CORE3_WFI_0)
    }
    #[doc = "WFI for CORE3 is masked"]
    #[inline]
    pub fn mask_core3_wfi_1(self) -> &'a mut W {
        self.variant(MASK_CORE3_WFIW::MASK_CORE3_WFI_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IRQ_SRC_C2`"]
pub enum IRQ_SRC_C2W {
    #[doc = "core2 wakeup source from external INT\\[127:0\\], masked by IMR1. See Power Up Process for A53 Platform for more specific information."]
    IRQ_SRC_C2_0,
    #[doc = "core2 wakeup source from external GIC(nFIQ\\[1\\]/nIRQ\\[1\\]), SCU should not be powered down during low power mode when this bit is set to 1'b1."]
    IRQ_SRC_C2_1,
}
impl IRQ_SRC_C2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRQ_SRC_C2W::IRQ_SRC_C2_0 => false,
            IRQ_SRC_C2W::IRQ_SRC_C2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRQ_SRC_C2W<'a> {
    w: &'a mut W,
}
impl<'a> _IRQ_SRC_C2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRQ_SRC_C2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "core2 wakeup source from external INT\\[127:0\\], masked by IMR1. See Power Up Process for A53 Platform for more specific information."]
    #[inline]
    pub fn irq_src_c2_0(self) -> &'a mut W {
        self.variant(IRQ_SRC_C2W::IRQ_SRC_C2_0)
    }
    #[doc = "core2 wakeup source from external GIC(nFIQ\\[1\\]/nIRQ\\[1\\]), SCU should not be powered down during low power mode when this bit is set to 1'b1."]
    #[inline]
    pub fn irq_src_c2_1(self) -> &'a mut W {
        self.variant(IRQ_SRC_C2W::IRQ_SRC_C2_1)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IRQ_SRC_C3`"]
pub enum IRQ_SRC_C3W {
    #[doc = "core3 wakeup source from external INT\\[127:0\\], masked by IMR1. See Power Up Process for A53 Platform for more specific information."]
    IRQ_SRC_C3_0,
    #[doc = "core3 wakeup source from external GIC(nFIQ\\[1\\]/nIRQ\\[1\\]), SCU should not be powered down during low power mode when this bit is set to 1'b1."]
    IRQ_SRC_C3_1,
}
impl IRQ_SRC_C3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRQ_SRC_C3W::IRQ_SRC_C3_0 => false,
            IRQ_SRC_C3W::IRQ_SRC_C3_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRQ_SRC_C3W<'a> {
    w: &'a mut W,
}
impl<'a> _IRQ_SRC_C3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRQ_SRC_C3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "core3 wakeup source from external INT\\[127:0\\], masked by IMR1. See Power Up Process for A53 Platform for more specific information."]
    #[inline]
    pub fn irq_src_c3_0(self) -> &'a mut W {
        self.variant(IRQ_SRC_C3W::IRQ_SRC_C3_0)
    }
    #[doc = "core3 wakeup source from external GIC(nFIQ\\[1\\]/nIRQ\\[1\\]), SCU should not be powered down during low power mode when this bit is set to 1'b1."]
    #[inline]
    pub fn irq_src_c3_1(self) -> &'a mut W {
        self.variant(IRQ_SRC_C3W::IRQ_SRC_C3_1)
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
#[doc = "Values that can be written to the field `MASK_SCU_WFI`"]
pub enum MASK_SCU_WFIW {
    #[doc = "WFI for SCU is not masked"]
    MASK_SCU_WFI_0,
    #[doc = "WFI for SCU is masked"]
    MASK_SCU_WFI_1,
}
impl MASK_SCU_WFIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASK_SCU_WFIW::MASK_SCU_WFI_0 => false,
            MASK_SCU_WFIW::MASK_SCU_WFI_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_SCU_WFIW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_SCU_WFIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_SCU_WFIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "WFI for SCU is not masked"]
    #[inline]
    pub fn mask_scu_wfi_0(self) -> &'a mut W {
        self.variant(MASK_SCU_WFIW::MASK_SCU_WFI_0)
    }
    #[doc = "WFI for SCU is masked"]
    #[inline]
    pub fn mask_scu_wfi_1(self) -> &'a mut W {
        self.variant(MASK_SCU_WFIW::MASK_SCU_WFI_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MASK_L2CC_WFI`"]
pub enum MASK_L2CC_WFIW {
    #[doc = "WFI for L2 cache controller is not masked"]
    MASK_L2CC_WFI_0,
    #[doc = "WFI for L2 cache controller is masked"]
    MASK_L2CC_WFI_1,
}
impl MASK_L2CC_WFIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASK_L2CC_WFIW::MASK_L2CC_WFI_0 => false,
            MASK_L2CC_WFIW::MASK_L2CC_WFI_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_L2CC_WFIW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_L2CC_WFIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_L2CC_WFIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "WFI for L2 cache controller is not masked"]
    #[inline]
    pub fn mask_l2cc_wfi_0(self) -> &'a mut W {
        self.variant(MASK_L2CC_WFIW::MASK_L2CC_WFI_0)
    }
    #[doc = "WFI for L2 cache controller is masked"]
    #[inline]
    pub fn mask_l2cc_wfi_1(self) -> &'a mut W {
        self.variant(MASK_L2CC_WFIW::MASK_L2CC_WFI_1)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IRQ_SRC_C0`"]
pub enum IRQ_SRC_C0W {
    #[doc = "core0 wakeup source from external INT\\[127:0\\], masked by IMR0 refer to \"Power up process for A53 platform\" for more specific information"]
    IRQ_SRC_C0_0,
    #[doc = "core0 wakeup source from GIC(nFIQ\\[0\\]/nIRQ\\[0\\] ), SCU should not be power down during low power mode when this bit is set to 1'b1"]
    IRQ_SRC_C0_1,
}
impl IRQ_SRC_C0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRQ_SRC_C0W::IRQ_SRC_C0_0 => false,
            IRQ_SRC_C0W::IRQ_SRC_C0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRQ_SRC_C0W<'a> {
    w: &'a mut W,
}
impl<'a> _IRQ_SRC_C0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRQ_SRC_C0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "core0 wakeup source from external INT\\[127:0\\], masked by IMR0 refer to \"Power up process for A53 platform\" for more specific information"]
    #[inline]
    pub fn irq_src_c0_0(self) -> &'a mut W {
        self.variant(IRQ_SRC_C0W::IRQ_SRC_C0_0)
    }
    #[doc = "core0 wakeup source from GIC(nFIQ\\[0\\]/nIRQ\\[0\\] ), SCU should not be power down during low power mode when this bit is set to 1'b1"]
    #[inline]
    pub fn irq_src_c0_1(self) -> &'a mut W {
        self.variant(IRQ_SRC_C0W::IRQ_SRC_C0_1)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IRQ_SRC_C1`"]
pub enum IRQ_SRC_C1W {
    #[doc = "core1 wakeup source from external INT\\[127:0\\], masked by IMR1 refer to \"Power up process for A53 platform\" for more specific information"]
    IRQ_SRC_C1_0,
    #[doc = "core1 wakeup source from GIC(nFIQ\\[1\\]/nIRQ\\[1\\] ), SCU should not be power down during low power mode when this bit is set to 1'b1"]
    IRQ_SRC_C1_1,
}
impl IRQ_SRC_C1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRQ_SRC_C1W::IRQ_SRC_C1_0 => false,
            IRQ_SRC_C1W::IRQ_SRC_C1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRQ_SRC_C1W<'a> {
    w: &'a mut W,
}
impl<'a> _IRQ_SRC_C1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRQ_SRC_C1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "core1 wakeup source from external INT\\[127:0\\], masked by IMR1 refer to \"Power up process for A53 platform\" for more specific information"]
    #[inline]
    pub fn irq_src_c1_0(self) -> &'a mut W {
        self.variant(IRQ_SRC_C1W::IRQ_SRC_C1_0)
    }
    #[doc = "core1 wakeup source from GIC(nFIQ\\[1\\]/nIRQ\\[1\\] ), SCU should not be power down during low power mode when this bit is set to 1'b1"]
    #[inline]
    pub fn irq_src_c1_1(self) -> &'a mut W {
        self.variant(IRQ_SRC_C1W::IRQ_SRC_C1_1)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IRQ_SRC_A53_WUP`"]
pub enum IRQ_SRC_A53_WUPW {
    #[doc = "LPM wakeup source be \"OR\" result of LPCR_A53_BSC\\[IRQ_SRC_C0\\]/LPCR_A53_BSC\\[IRQ_SRC_C1\\]/LPCR_A53_BSC\\[IRQ_SRC_C2\\]/LPCR_A53_BSC\\[IRQ_SRC_C3\\] setting"]
    IRQ_SRC_A53_WUP_0,
    #[doc = "LPM wakeup source from external INT\\[127:0\\], masked by IMR0"]
    IRQ_SRC_A53_WUP_1,
}
impl IRQ_SRC_A53_WUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRQ_SRC_A53_WUPW::IRQ_SRC_A53_WUP_0 => false,
            IRQ_SRC_A53_WUPW::IRQ_SRC_A53_WUP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRQ_SRC_A53_WUPW<'a> {
    w: &'a mut W,
}
impl<'a> _IRQ_SRC_A53_WUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRQ_SRC_A53_WUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LPM wakeup source be \"OR\" result of LPCR_A53_BSC\\[IRQ_SRC_C0\\]/LPCR_A53_BSC\\[IRQ_SRC_C1\\]/LPCR_A53_BSC\\[IRQ_SRC_C2\\]/LPCR_A53_BSC\\[IRQ_SRC_C3\\] setting"]
    #[inline]
    pub fn irq_src_a53_wup_0(self) -> &'a mut W {
        self.variant(IRQ_SRC_A53_WUPW::IRQ_SRC_A53_WUP_0)
    }
    #[doc = "LPM wakeup source from external INT\\[127:0\\], masked by IMR0"]
    #[inline]
    pub fn irq_src_a53_wup_1(self) -> &'a mut W {
        self.variant(IRQ_SRC_A53_WUPW::IRQ_SRC_A53_WUP_1)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MASK_DSM_TRIGGER`"]
pub enum MASK_DSM_TRIGGERW {
    #[doc = "DSM trigger of A53 platform will not be masked"]
    MASK_DSM_TRIGGER_0,
    #[doc = "DSM trigger of A53 platform will be masked"]
    MASK_DSM_TRIGGER_1,
}
impl MASK_DSM_TRIGGERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASK_DSM_TRIGGERW::MASK_DSM_TRIGGER_0 => false,
            MASK_DSM_TRIGGERW::MASK_DSM_TRIGGER_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_DSM_TRIGGERW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_DSM_TRIGGERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_DSM_TRIGGERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DSM trigger of A53 platform will not be masked"]
    #[inline]
    pub fn mask_dsm_trigger_0(self) -> &'a mut W {
        self.variant(MASK_DSM_TRIGGERW::MASK_DSM_TRIGGER_0)
    }
    #[doc = "DSM trigger of A53 platform will be masked"]
    #[inline]
    pub fn mask_dsm_trigger_1(self) -> &'a mut W {
        self.variant(MASK_DSM_TRIGGERW::MASK_DSM_TRIGGER_1)
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
    #[doc = "Bits 0:1 - CORE0 Setting the low power mode that system will enter on next assertion of dsm_request signal."]
    #[inline]
    pub fn lpm0(&self) -> LPM0R {
        LPM0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - CORE1 Setting the low power mode that system will enter on next assertion of dsm_request signal."]
    #[inline]
    pub fn lpm1(&self) -> LPM1R {
        LPM1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - MASTER0 LPM handshake mask"]
    #[inline]
    pub fn mst0_lpm_hsk_mask(&self) -> MST0_LPM_HSK_MASKR {
        MST0_LPM_HSK_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - MASTER1 LPM handshake mask"]
    #[inline]
    pub fn mst1_lpm_hsk_mask(&self) -> MST1_LPM_HSK_MASKR {
        MST1_LPM_HSK_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - MASTER2 LPM handshake mask"]
    #[inline]
    pub fn mst2_lpm_hsk_mask(&self) -> MST2_LPM_HSK_MASKR {
        MST2_LPM_HSK_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Define if A53 clocks will be disabled on wait/stop mode."]
    #[inline]
    pub fn cpu_clk_on_lpm(&self) -> CPU_CLK_ON_LPMR {
        CPU_CLK_ON_LPMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - CORE0 Wait For Interrupt Mask"]
    #[inline]
    pub fn mask_core0_wfi(&self) -> MASK_CORE0_WFIR {
        MASK_CORE0_WFIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - CORE1 Wait For Interrupt Mask"]
    #[inline]
    pub fn mask_core1_wfi(&self) -> MASK_CORE1_WFIR {
        MASK_CORE1_WFIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - CORE2 Wait For Interrupt Mask"]
    #[inline]
    pub fn mask_core2_wfi(&self) -> MASK_CORE2_WFIR {
        MASK_CORE2_WFIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - CORE3 Wait For Interrupt Mask"]
    #[inline]
    pub fn mask_core3_wfi(&self) -> MASK_CORE3_WFIR {
        MASK_CORE3_WFIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - LPCR_A53_BSC\\[IRQ_SRC_C0\\], LPCR_A53_BSC\\[IRQ_SRC_C1\\], LPCR_A53_BSC\\[IRQ_SRC_C2\\], LPCR_A53_BSC\\[IRQ_SRC_C3\\], and LPCR_A53_BSC\\[IRQ_SRC_A53_WUP\\] work together to decide the wake up source for A53 LPM and core0/core1/core2/core3 power"]
    #[inline]
    pub fn irq_src_c2(&self) -> IRQ_SRC_C2R {
        IRQ_SRC_C2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - LPCR_A53_BSC\\[IRQ_SRC_C0\\], LPCR_A53_BSC\\[IRQ_SRC_C1\\], LPCR_A53_BSC\\[IRQ_SRC_C2\\], LPCR_A53_BSC\\[IRQ_SRC_C3\\], and LPCR_A53_BSC\\[IRQ_SRC_A53_WUP\\] work together to decide the wake up source for A53 LPM and core0/core1/core2/core3 power"]
    #[inline]
    pub fn irq_src_c3(&self) -> IRQ_SRC_C3R {
        IRQ_SRC_C3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - SCU Wait For Interrupt Mask Register"]
    #[inline]
    pub fn mask_scu_wfi(&self) -> MASK_SCU_WFIR {
        MASK_SCU_WFIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - L2 cache controller Wait For Interrupt Mask Register"]
    #[inline]
    pub fn mask_l2cc_wfi(&self) -> MASK_L2CC_WFIR {
        MASK_L2CC_WFIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - LPCR_A53_BSC\\[IRQ_SRC_C0\\], LPCR_A53_BSC\\[IRQ_SRC_C1\\], LPCR_A53_BSC\\[IRQ_SRC_C2\\], LPCR_A53_BSC\\[IRQ_SRC_C3\\], and LPCR_A53_BSC\\[IRQ_SRC_A53_WUP\\] work together to decide the wake up source for A53 LPM and core0/core1/core2/core3 power"]
    #[inline]
    pub fn irq_src_c0(&self) -> IRQ_SRC_C0R {
        IRQ_SRC_C0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - LPCR_A53_BSC\\[IRQ_SRC_C0\\], LPCR_A53_BSC\\[IRQ_SRC_C1\\], LPCR_A53_BSC\\[IRQ_SRC_C2\\], LPCR_A53_BSC\\[IRQ_SRC_C3\\], and LPCR_A53_BSC\\[IRQ_SRC_A53_WUP\\] work together to decide the wake up source for A53 LPM and core0/core1/core2/core3 power"]
    #[inline]
    pub fn irq_src_c1(&self) -> IRQ_SRC_C1R {
        IRQ_SRC_C1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - LPCR_A53_BSC\\[IRQ_SRC_C0\\], LPCR_A53_BSC\\[IRQ_SRC_C1\\], LPCR_A53_BSC\\[IRQ_SRC_C2\\], LPCR_A53_BSC\\[IRQ_SRC_C3\\], and LPCR_A53_BSC\\[IRQ_SRC_A53_WUP\\] work together to decide the wake up source for A53 LPM and core0/core1/core2/core3 power"]
    #[inline]
    pub fn irq_src_a53_wup(&self) -> IRQ_SRC_A53_WUPR {
        IRQ_SRC_A53_WUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - DSM Trigger Mask"]
    #[inline]
    pub fn mask_dsm_trigger(&self) -> MASK_DSM_TRIGGERR {
        MASK_DSM_TRIGGERR::_from({
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
        W { bits: 16368 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - CORE0 Setting the low power mode that system will enter on next assertion of dsm_request signal."]
    #[inline]
    pub fn lpm0(&mut self) -> _LPM0W {
        _LPM0W { w: self }
    }
    #[doc = "Bits 2:3 - CORE1 Setting the low power mode that system will enter on next assertion of dsm_request signal."]
    #[inline]
    pub fn lpm1(&mut self) -> _LPM1W {
        _LPM1W { w: self }
    }
    #[doc = "Bit 6 - MASTER0 LPM handshake mask"]
    #[inline]
    pub fn mst0_lpm_hsk_mask(&mut self) -> _MST0_LPM_HSK_MASKW {
        _MST0_LPM_HSK_MASKW { w: self }
    }
    #[doc = "Bit 7 - MASTER1 LPM handshake mask"]
    #[inline]
    pub fn mst1_lpm_hsk_mask(&mut self) -> _MST1_LPM_HSK_MASKW {
        _MST1_LPM_HSK_MASKW { w: self }
    }
    #[doc = "Bit 8 - MASTER2 LPM handshake mask"]
    #[inline]
    pub fn mst2_lpm_hsk_mask(&mut self) -> _MST2_LPM_HSK_MASKW {
        _MST2_LPM_HSK_MASKW { w: self }
    }
    #[doc = "Bit 14 - Define if A53 clocks will be disabled on wait/stop mode."]
    #[inline]
    pub fn cpu_clk_on_lpm(&mut self) -> _CPU_CLK_ON_LPMW {
        _CPU_CLK_ON_LPMW { w: self }
    }
    #[doc = "Bit 16 - CORE0 Wait For Interrupt Mask"]
    #[inline]
    pub fn mask_core0_wfi(&mut self) -> _MASK_CORE0_WFIW {
        _MASK_CORE0_WFIW { w: self }
    }
    #[doc = "Bit 17 - CORE1 Wait For Interrupt Mask"]
    #[inline]
    pub fn mask_core1_wfi(&mut self) -> _MASK_CORE1_WFIW {
        _MASK_CORE1_WFIW { w: self }
    }
    #[doc = "Bit 18 - CORE2 Wait For Interrupt Mask"]
    #[inline]
    pub fn mask_core2_wfi(&mut self) -> _MASK_CORE2_WFIW {
        _MASK_CORE2_WFIW { w: self }
    }
    #[doc = "Bit 19 - CORE3 Wait For Interrupt Mask"]
    #[inline]
    pub fn mask_core3_wfi(&mut self) -> _MASK_CORE3_WFIW {
        _MASK_CORE3_WFIW { w: self }
    }
    #[doc = "Bit 22 - LPCR_A53_BSC\\[IRQ_SRC_C0\\], LPCR_A53_BSC\\[IRQ_SRC_C1\\], LPCR_A53_BSC\\[IRQ_SRC_C2\\], LPCR_A53_BSC\\[IRQ_SRC_C3\\], and LPCR_A53_BSC\\[IRQ_SRC_A53_WUP\\] work together to decide the wake up source for A53 LPM and core0/core1/core2/core3 power"]
    #[inline]
    pub fn irq_src_c2(&mut self) -> _IRQ_SRC_C2W {
        _IRQ_SRC_C2W { w: self }
    }
    #[doc = "Bit 23 - LPCR_A53_BSC\\[IRQ_SRC_C0\\], LPCR_A53_BSC\\[IRQ_SRC_C1\\], LPCR_A53_BSC\\[IRQ_SRC_C2\\], LPCR_A53_BSC\\[IRQ_SRC_C3\\], and LPCR_A53_BSC\\[IRQ_SRC_A53_WUP\\] work together to decide the wake up source for A53 LPM and core0/core1/core2/core3 power"]
    #[inline]
    pub fn irq_src_c3(&mut self) -> _IRQ_SRC_C3W {
        _IRQ_SRC_C3W { w: self }
    }
    #[doc = "Bit 24 - SCU Wait For Interrupt Mask Register"]
    #[inline]
    pub fn mask_scu_wfi(&mut self) -> _MASK_SCU_WFIW {
        _MASK_SCU_WFIW { w: self }
    }
    #[doc = "Bit 26 - L2 cache controller Wait For Interrupt Mask Register"]
    #[inline]
    pub fn mask_l2cc_wfi(&mut self) -> _MASK_L2CC_WFIW {
        _MASK_L2CC_WFIW { w: self }
    }
    #[doc = "Bit 28 - LPCR_A53_BSC\\[IRQ_SRC_C0\\], LPCR_A53_BSC\\[IRQ_SRC_C1\\], LPCR_A53_BSC\\[IRQ_SRC_C2\\], LPCR_A53_BSC\\[IRQ_SRC_C3\\], and LPCR_A53_BSC\\[IRQ_SRC_A53_WUP\\] work together to decide the wake up source for A53 LPM and core0/core1/core2/core3 power"]
    #[inline]
    pub fn irq_src_c0(&mut self) -> _IRQ_SRC_C0W {
        _IRQ_SRC_C0W { w: self }
    }
    #[doc = "Bit 29 - LPCR_A53_BSC\\[IRQ_SRC_C0\\], LPCR_A53_BSC\\[IRQ_SRC_C1\\], LPCR_A53_BSC\\[IRQ_SRC_C2\\], LPCR_A53_BSC\\[IRQ_SRC_C3\\], and LPCR_A53_BSC\\[IRQ_SRC_A53_WUP\\] work together to decide the wake up source for A53 LPM and core0/core1/core2/core3 power"]
    #[inline]
    pub fn irq_src_c1(&mut self) -> _IRQ_SRC_C1W {
        _IRQ_SRC_C1W { w: self }
    }
    #[doc = "Bit 30 - LPCR_A53_BSC\\[IRQ_SRC_C0\\], LPCR_A53_BSC\\[IRQ_SRC_C1\\], LPCR_A53_BSC\\[IRQ_SRC_C2\\], LPCR_A53_BSC\\[IRQ_SRC_C3\\], and LPCR_A53_BSC\\[IRQ_SRC_A53_WUP\\] work together to decide the wake up source for A53 LPM and core0/core1/core2/core3 power"]
    #[inline]
    pub fn irq_src_a53_wup(&mut self) -> _IRQ_SRC_A53_WUPW {
        _IRQ_SRC_A53_WUPW { w: self }
    }
    #[doc = "Bit 31 - DSM Trigger Mask"]
    #[inline]
    pub fn mask_dsm_trigger(&mut self) -> _MASK_DSM_TRIGGERW {
        _MASK_DSM_TRIGGERW { w: self }
    }
}
