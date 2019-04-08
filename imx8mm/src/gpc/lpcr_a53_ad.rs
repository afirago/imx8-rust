#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LPCR_A53_AD {
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
#[doc = "Possible values of the field `EN_C0_WFI_PDN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_C0_WFI_PDNR {
    #[doc = "CORE0 will not be power down with WFI request"]
    EN_C0_WFI_PDN_0,
    #[doc = "CORE0 will be power down with WFI request"]
    EN_C0_WFI_PDN_1,
}
impl EN_C0_WFI_PDNR {
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
            EN_C0_WFI_PDNR::EN_C0_WFI_PDN_0 => false,
            EN_C0_WFI_PDNR::EN_C0_WFI_PDN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_C0_WFI_PDNR {
        match value {
            false => EN_C0_WFI_PDNR::EN_C0_WFI_PDN_0,
            true => EN_C0_WFI_PDNR::EN_C0_WFI_PDN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_C0_WFI_PDN_0`"]
    #[inline]
    pub fn is_en_c0_wfi_pdn_0(&self) -> bool {
        *self == EN_C0_WFI_PDNR::EN_C0_WFI_PDN_0
    }
    #[doc = "Checks if the value of the field is `EN_C0_WFI_PDN_1`"]
    #[inline]
    pub fn is_en_c0_wfi_pdn_1(&self) -> bool {
        *self == EN_C0_WFI_PDNR::EN_C0_WFI_PDN_1
    }
}
#[doc = "Possible values of the field `EN_C0_PDN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_C0_PDNR {
    #[doc = "CORE0 will not be power down with low power mode request"]
    EN_C0_PDN_0,
    #[doc = "CORE0 will be power down with low power mode request"]
    EN_C0_PDN_1,
}
impl EN_C0_PDNR {
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
            EN_C0_PDNR::EN_C0_PDN_0 => false,
            EN_C0_PDNR::EN_C0_PDN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_C0_PDNR {
        match value {
            false => EN_C0_PDNR::EN_C0_PDN_0,
            true => EN_C0_PDNR::EN_C0_PDN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_C0_PDN_0`"]
    #[inline]
    pub fn is_en_c0_pdn_0(&self) -> bool {
        *self == EN_C0_PDNR::EN_C0_PDN_0
    }
    #[doc = "Checks if the value of the field is `EN_C0_PDN_1`"]
    #[inline]
    pub fn is_en_c0_pdn_1(&self) -> bool {
        *self == EN_C0_PDNR::EN_C0_PDN_1
    }
}
#[doc = "Possible values of the field `EN_C1_WFI_PDN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_C1_WFI_PDNR {
    #[doc = "CORE1 will not be power down with WFI request"]
    EN_C1_WFI_PDN_0,
    #[doc = "CORE1 will be power down with WFI request"]
    EN_C1_WFI_PDN_1,
}
impl EN_C1_WFI_PDNR {
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
            EN_C1_WFI_PDNR::EN_C1_WFI_PDN_0 => false,
            EN_C1_WFI_PDNR::EN_C1_WFI_PDN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_C1_WFI_PDNR {
        match value {
            false => EN_C1_WFI_PDNR::EN_C1_WFI_PDN_0,
            true => EN_C1_WFI_PDNR::EN_C1_WFI_PDN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_C1_WFI_PDN_0`"]
    #[inline]
    pub fn is_en_c1_wfi_pdn_0(&self) -> bool {
        *self == EN_C1_WFI_PDNR::EN_C1_WFI_PDN_0
    }
    #[doc = "Checks if the value of the field is `EN_C1_WFI_PDN_1`"]
    #[inline]
    pub fn is_en_c1_wfi_pdn_1(&self) -> bool {
        *self == EN_C1_WFI_PDNR::EN_C1_WFI_PDN_1
    }
}
#[doc = "Possible values of the field `EN_C1_PDN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_C1_PDNR {
    #[doc = "CORE1 will not be power down with low power mode request"]
    EN_C1_PDN_0,
    #[doc = "CORE1 will be power down with low power mode request"]
    EN_C1_PDN_1,
}
impl EN_C1_PDNR {
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
            EN_C1_PDNR::EN_C1_PDN_0 => false,
            EN_C1_PDNR::EN_C1_PDN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_C1_PDNR {
        match value {
            false => EN_C1_PDNR::EN_C1_PDN_0,
            true => EN_C1_PDNR::EN_C1_PDN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_C1_PDN_0`"]
    #[inline]
    pub fn is_en_c1_pdn_0(&self) -> bool {
        *self == EN_C1_PDNR::EN_C1_PDN_0
    }
    #[doc = "Checks if the value of the field is `EN_C1_PDN_1`"]
    #[inline]
    pub fn is_en_c1_pdn_1(&self) -> bool {
        *self == EN_C1_PDNR::EN_C1_PDN_1
    }
}
#[doc = "Possible values of the field `EN_PLAT_PDN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_PLAT_PDNR {
    #[doc = "SCU and L2 cache RAM will not be power down with low power mode request"]
    EN_PLAT_PDN_0,
    #[doc = "SCU and L2 cache RAM will be power down with low power mode request"]
    EN_PLAT_PDN_1,
}
impl EN_PLAT_PDNR {
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
            EN_PLAT_PDNR::EN_PLAT_PDN_0 => false,
            EN_PLAT_PDNR::EN_PLAT_PDN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_PLAT_PDNR {
        match value {
            false => EN_PLAT_PDNR::EN_PLAT_PDN_0,
            true => EN_PLAT_PDNR::EN_PLAT_PDN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_PLAT_PDN_0`"]
    #[inline]
    pub fn is_en_plat_pdn_0(&self) -> bool {
        *self == EN_PLAT_PDNR::EN_PLAT_PDN_0
    }
    #[doc = "Checks if the value of the field is `EN_PLAT_PDN_1`"]
    #[inline]
    pub fn is_en_plat_pdn_1(&self) -> bool {
        *self == EN_PLAT_PDNR::EN_PLAT_PDN_1
    }
}
#[doc = "Possible values of the field `EN_L2_WFI_PDN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_L2_WFI_PDNR {
    #[doc = "SCU and L2 will not be power down with WFI request"]
    EN_L2_WFI_PDN_0,
    #[doc = "SCU and L2 will be power down with WFI request (default)"]
    EN_L2_WFI_PDN_1,
}
impl EN_L2_WFI_PDNR {
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
            EN_L2_WFI_PDNR::EN_L2_WFI_PDN_0 => false,
            EN_L2_WFI_PDNR::EN_L2_WFI_PDN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_L2_WFI_PDNR {
        match value {
            false => EN_L2_WFI_PDNR::EN_L2_WFI_PDN_0,
            true => EN_L2_WFI_PDNR::EN_L2_WFI_PDN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_L2_WFI_PDN_0`"]
    #[inline]
    pub fn is_en_l2_wfi_pdn_0(&self) -> bool {
        *self == EN_L2_WFI_PDNR::EN_L2_WFI_PDN_0
    }
    #[doc = "Checks if the value of the field is `EN_L2_WFI_PDN_1`"]
    #[inline]
    pub fn is_en_l2_wfi_pdn_1(&self) -> bool {
        *self == EN_L2_WFI_PDNR::EN_L2_WFI_PDN_1
    }
}
#[doc = "Possible values of the field `EN_C0_IRQ_PUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_C0_IRQ_PUPR {
    #[doc = "CORE0 will power up with IRQ request"]
    EN_C0_IRQ_PUP_0,
    #[doc = "CORE0 will not power up with IRQ request"]
    EN_C0_IRQ_PUP_1,
}
impl EN_C0_IRQ_PUPR {
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
            EN_C0_IRQ_PUPR::EN_C0_IRQ_PUP_0 => false,
            EN_C0_IRQ_PUPR::EN_C0_IRQ_PUP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_C0_IRQ_PUPR {
        match value {
            false => EN_C0_IRQ_PUPR::EN_C0_IRQ_PUP_0,
            true => EN_C0_IRQ_PUPR::EN_C0_IRQ_PUP_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_C0_IRQ_PUP_0`"]
    #[inline]
    pub fn is_en_c0_irq_pup_0(&self) -> bool {
        *self == EN_C0_IRQ_PUPR::EN_C0_IRQ_PUP_0
    }
    #[doc = "Checks if the value of the field is `EN_C0_IRQ_PUP_1`"]
    #[inline]
    pub fn is_en_c0_irq_pup_1(&self) -> bool {
        *self == EN_C0_IRQ_PUPR::EN_C0_IRQ_PUP_1
    }
}
#[doc = "Possible values of the field `EN_C0_PUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_C0_PUPR {
    #[doc = "CORE0 will power up with low power mode request"]
    EN_C0_PUP_0,
    #[doc = "CORE0 will not power up with low power mode request"]
    EN_C0_PUP_1,
}
impl EN_C0_PUPR {
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
            EN_C0_PUPR::EN_C0_PUP_0 => false,
            EN_C0_PUPR::EN_C0_PUP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_C0_PUPR {
        match value {
            false => EN_C0_PUPR::EN_C0_PUP_0,
            true => EN_C0_PUPR::EN_C0_PUP_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_C0_PUP_0`"]
    #[inline]
    pub fn is_en_c0_pup_0(&self) -> bool {
        *self == EN_C0_PUPR::EN_C0_PUP_0
    }
    #[doc = "Checks if the value of the field is `EN_C0_PUP_1`"]
    #[inline]
    pub fn is_en_c0_pup_1(&self) -> bool {
        *self == EN_C0_PUPR::EN_C0_PUP_1
    }
}
#[doc = "Possible values of the field `EN_C1_IRQ_PUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_C1_IRQ_PUPR {
    #[doc = "CORE1 will power up with IRQ request"]
    EN_C1_IRQ_PUP_0,
    #[doc = "CORE1 will not power up with IRQ request"]
    EN_C1_IRQ_PUP_1,
}
impl EN_C1_IRQ_PUPR {
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
            EN_C1_IRQ_PUPR::EN_C1_IRQ_PUP_0 => false,
            EN_C1_IRQ_PUPR::EN_C1_IRQ_PUP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_C1_IRQ_PUPR {
        match value {
            false => EN_C1_IRQ_PUPR::EN_C1_IRQ_PUP_0,
            true => EN_C1_IRQ_PUPR::EN_C1_IRQ_PUP_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_C1_IRQ_PUP_0`"]
    #[inline]
    pub fn is_en_c1_irq_pup_0(&self) -> bool {
        *self == EN_C1_IRQ_PUPR::EN_C1_IRQ_PUP_0
    }
    #[doc = "Checks if the value of the field is `EN_C1_IRQ_PUP_1`"]
    #[inline]
    pub fn is_en_c1_irq_pup_1(&self) -> bool {
        *self == EN_C1_IRQ_PUPR::EN_C1_IRQ_PUP_1
    }
}
#[doc = "Possible values of the field `EN_C1_PUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_C1_PUPR {
    #[doc = "CORE1 will not power up with low power mode request (only used wake up from CPU01_OFF mode)"]
    EN_C1_PUP_0,
    #[doc = "CORE1 will power up with low power mode request"]
    EN_C1_PUP_1,
}
impl EN_C1_PUPR {
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
            EN_C1_PUPR::EN_C1_PUP_0 => false,
            EN_C1_PUPR::EN_C1_PUP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_C1_PUPR {
        match value {
            false => EN_C1_PUPR::EN_C1_PUP_0,
            true => EN_C1_PUPR::EN_C1_PUP_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_C1_PUP_0`"]
    #[inline]
    pub fn is_en_c1_pup_0(&self) -> bool {
        *self == EN_C1_PUPR::EN_C1_PUP_0
    }
    #[doc = "Checks if the value of the field is `EN_C1_PUP_1`"]
    #[inline]
    pub fn is_en_c1_pup_1(&self) -> bool {
        *self == EN_C1_PUPR::EN_C1_PUP_1
    }
}
#[doc = "Possible values of the field `EN_C2_WFI_PDN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_C2_WFI_PDNR {
    #[doc = "CORE2 will not be power down with WFI request"]
    EN_C2_WFI_PDN_0,
    #[doc = "CORE2 will be power down with WFI request"]
    EN_C2_WFI_PDN_1,
}
impl EN_C2_WFI_PDNR {
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
            EN_C2_WFI_PDNR::EN_C2_WFI_PDN_0 => false,
            EN_C2_WFI_PDNR::EN_C2_WFI_PDN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_C2_WFI_PDNR {
        match value {
            false => EN_C2_WFI_PDNR::EN_C2_WFI_PDN_0,
            true => EN_C2_WFI_PDNR::EN_C2_WFI_PDN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_C2_WFI_PDN_0`"]
    #[inline]
    pub fn is_en_c2_wfi_pdn_0(&self) -> bool {
        *self == EN_C2_WFI_PDNR::EN_C2_WFI_PDN_0
    }
    #[doc = "Checks if the value of the field is `EN_C2_WFI_PDN_1`"]
    #[inline]
    pub fn is_en_c2_wfi_pdn_1(&self) -> bool {
        *self == EN_C2_WFI_PDNR::EN_C2_WFI_PDN_1
    }
}
#[doc = "Possible values of the field `EN_C2_PDN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_C2_PDNR {
    #[doc = "CORE2 will not be power down with low power mode request"]
    EN_C2_PDN_0,
    #[doc = "CORE2 will be power down with low power mode request"]
    EN_C2_PDN_1,
}
impl EN_C2_PDNR {
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
            EN_C2_PDNR::EN_C2_PDN_0 => false,
            EN_C2_PDNR::EN_C2_PDN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_C2_PDNR {
        match value {
            false => EN_C2_PDNR::EN_C2_PDN_0,
            true => EN_C2_PDNR::EN_C2_PDN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_C2_PDN_0`"]
    #[inline]
    pub fn is_en_c2_pdn_0(&self) -> bool {
        *self == EN_C2_PDNR::EN_C2_PDN_0
    }
    #[doc = "Checks if the value of the field is `EN_C2_PDN_1`"]
    #[inline]
    pub fn is_en_c2_pdn_1(&self) -> bool {
        *self == EN_C2_PDNR::EN_C2_PDN_1
    }
}
#[doc = "Possible values of the field `EN_C3_WFI_PDN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_C3_WFI_PDNR {
    #[doc = "CORE3 will not be power down with WFI request"]
    EN_C3_WFI_PDN_0,
    #[doc = "CORE3 will be power down with WFI request"]
    EN_C3_WFI_PDN_1,
}
impl EN_C3_WFI_PDNR {
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
            EN_C3_WFI_PDNR::EN_C3_WFI_PDN_0 => false,
            EN_C3_WFI_PDNR::EN_C3_WFI_PDN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_C3_WFI_PDNR {
        match value {
            false => EN_C3_WFI_PDNR::EN_C3_WFI_PDN_0,
            true => EN_C3_WFI_PDNR::EN_C3_WFI_PDN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_C3_WFI_PDN_0`"]
    #[inline]
    pub fn is_en_c3_wfi_pdn_0(&self) -> bool {
        *self == EN_C3_WFI_PDNR::EN_C3_WFI_PDN_0
    }
    #[doc = "Checks if the value of the field is `EN_C3_WFI_PDN_1`"]
    #[inline]
    pub fn is_en_c3_wfi_pdn_1(&self) -> bool {
        *self == EN_C3_WFI_PDNR::EN_C3_WFI_PDN_1
    }
}
#[doc = "Possible values of the field `EN_C3_PDN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_C3_PDNR {
    #[doc = "CORE3 will not be power down with low power mode request"]
    EN_C3_PDN_0,
    #[doc = "CORE3 will be power down with low power mode request"]
    EN_C3_PDN_1,
}
impl EN_C3_PDNR {
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
            EN_C3_PDNR::EN_C3_PDN_0 => false,
            EN_C3_PDNR::EN_C3_PDN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_C3_PDNR {
        match value {
            false => EN_C3_PDNR::EN_C3_PDN_0,
            true => EN_C3_PDNR::EN_C3_PDN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_C3_PDN_0`"]
    #[inline]
    pub fn is_en_c3_pdn_0(&self) -> bool {
        *self == EN_C3_PDNR::EN_C3_PDN_0
    }
    #[doc = "Checks if the value of the field is `EN_C3_PDN_1`"]
    #[inline]
    pub fn is_en_c3_pdn_1(&self) -> bool {
        *self == EN_C3_PDNR::EN_C3_PDN_1
    }
}
#[doc = "Possible values of the field `EN_C2_IRQ_PUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_C2_IRQ_PUPR {
    #[doc = "CORE2 will power up with IRQ request"]
    EN_C2_IRQ_PUP_0,
    #[doc = "CORE2 will not power up with IRQ request"]
    EN_C2_IRQ_PUP_1,
}
impl EN_C2_IRQ_PUPR {
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
            EN_C2_IRQ_PUPR::EN_C2_IRQ_PUP_0 => false,
            EN_C2_IRQ_PUPR::EN_C2_IRQ_PUP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_C2_IRQ_PUPR {
        match value {
            false => EN_C2_IRQ_PUPR::EN_C2_IRQ_PUP_0,
            true => EN_C2_IRQ_PUPR::EN_C2_IRQ_PUP_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_C2_IRQ_PUP_0`"]
    #[inline]
    pub fn is_en_c2_irq_pup_0(&self) -> bool {
        *self == EN_C2_IRQ_PUPR::EN_C2_IRQ_PUP_0
    }
    #[doc = "Checks if the value of the field is `EN_C2_IRQ_PUP_1`"]
    #[inline]
    pub fn is_en_c2_irq_pup_1(&self) -> bool {
        *self == EN_C2_IRQ_PUPR::EN_C2_IRQ_PUP_1
    }
}
#[doc = "Possible values of the field `EN_C2_PUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_C2_PUPR {
    #[doc = "CORE2 will power up with lower power mode request"]
    EN_C2_PUP_0,
    #[doc = "CORE2 will not power up with low power mode request (only used wake up from CPU_OFF)"]
    EN_C2_PUP_1,
}
impl EN_C2_PUPR {
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
            EN_C2_PUPR::EN_C2_PUP_0 => false,
            EN_C2_PUPR::EN_C2_PUP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_C2_PUPR {
        match value {
            false => EN_C2_PUPR::EN_C2_PUP_0,
            true => EN_C2_PUPR::EN_C2_PUP_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_C2_PUP_0`"]
    #[inline]
    pub fn is_en_c2_pup_0(&self) -> bool {
        *self == EN_C2_PUPR::EN_C2_PUP_0
    }
    #[doc = "Checks if the value of the field is `EN_C2_PUP_1`"]
    #[inline]
    pub fn is_en_c2_pup_1(&self) -> bool {
        *self == EN_C2_PUPR::EN_C2_PUP_1
    }
}
#[doc = "Possible values of the field `EN_C3_IRQ_PUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_C3_IRQ_PUPR {
    #[doc = "CORE3 will power up with IRQ request"]
    EN_C3_IRQ_PUP_0,
    #[doc = "CORE3 will not power up with IRQ request"]
    EN_C3_IRQ_PUP_1,
}
impl EN_C3_IRQ_PUPR {
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
            EN_C3_IRQ_PUPR::EN_C3_IRQ_PUP_0 => false,
            EN_C3_IRQ_PUPR::EN_C3_IRQ_PUP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_C3_IRQ_PUPR {
        match value {
            false => EN_C3_IRQ_PUPR::EN_C3_IRQ_PUP_0,
            true => EN_C3_IRQ_PUPR::EN_C3_IRQ_PUP_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_C3_IRQ_PUP_0`"]
    #[inline]
    pub fn is_en_c3_irq_pup_0(&self) -> bool {
        *self == EN_C3_IRQ_PUPR::EN_C3_IRQ_PUP_0
    }
    #[doc = "Checks if the value of the field is `EN_C3_IRQ_PUP_1`"]
    #[inline]
    pub fn is_en_c3_irq_pup_1(&self) -> bool {
        *self == EN_C3_IRQ_PUPR::EN_C3_IRQ_PUP_1
    }
}
#[doc = "Possible values of the field `EN_C3_PUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_C3_PUPR {
    #[doc = "CORE3 will power up with lower power mode request"]
    EN_C3_PUP_0,
    #[doc = "CORE3 will not power up with low power mode request (only used wake up from CPU_OFF)"]
    EN_C3_PUP_1,
}
impl EN_C3_PUPR {
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
            EN_C3_PUPR::EN_C3_PUP_0 => false,
            EN_C3_PUPR::EN_C3_PUP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_C3_PUPR {
        match value {
            false => EN_C3_PUPR::EN_C3_PUP_0,
            true => EN_C3_PUPR::EN_C3_PUP_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_C3_PUP_0`"]
    #[inline]
    pub fn is_en_c3_pup_0(&self) -> bool {
        *self == EN_C3_PUPR::EN_C3_PUP_0
    }
    #[doc = "Checks if the value of the field is `EN_C3_PUP_1`"]
    #[inline]
    pub fn is_en_c3_pup_1(&self) -> bool {
        *self == EN_C3_PUPR::EN_C3_PUP_1
    }
}
#[doc = "Possible values of the field `L2PGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L2PGER {
    #[doc = "L2 cache RAM will power down with SCU power domain in A53 platform (used for ALL_OFF mode)"]
    L2PGE_0,
    #[doc = "L2 cache RAM will not power down with SCU power domain in A53 platform (used for ALL_OFF mode)"]
    L2PGE_1,
}
impl L2PGER {
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
            L2PGER::L2PGE_0 => false,
            L2PGER::L2PGE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> L2PGER {
        match value {
            false => L2PGER::L2PGE_0,
            true => L2PGER::L2PGE_1,
        }
    }
    #[doc = "Checks if the value of the field is `L2PGE_0`"]
    #[inline]
    pub fn is_l2pge_0(&self) -> bool {
        *self == L2PGER::L2PGE_0
    }
    #[doc = "Checks if the value of the field is `L2PGE_1`"]
    #[inline]
    pub fn is_l2pge_1(&self) -> bool {
        *self == L2PGER::L2PGE_1
    }
}
#[doc = "Values that can be written to the field `EN_C0_WFI_PDN`"]
pub enum EN_C0_WFI_PDNW {
    #[doc = "CORE0 will not be power down with WFI request"]
    EN_C0_WFI_PDN_0,
    #[doc = "CORE0 will be power down with WFI request"]
    EN_C0_WFI_PDN_1,
}
impl EN_C0_WFI_PDNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_C0_WFI_PDNW::EN_C0_WFI_PDN_0 => false,
            EN_C0_WFI_PDNW::EN_C0_WFI_PDN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_C0_WFI_PDNW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_C0_WFI_PDNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_C0_WFI_PDNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CORE0 will not be power down with WFI request"]
    #[inline]
    pub fn en_c0_wfi_pdn_0(self) -> &'a mut W {
        self.variant(EN_C0_WFI_PDNW::EN_C0_WFI_PDN_0)
    }
    #[doc = "CORE0 will be power down with WFI request"]
    #[inline]
    pub fn en_c0_wfi_pdn_1(self) -> &'a mut W {
        self.variant(EN_C0_WFI_PDNW::EN_C0_WFI_PDN_1)
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
#[doc = "Values that can be written to the field `EN_C0_PDN`"]
pub enum EN_C0_PDNW {
    #[doc = "CORE0 will not be power down with low power mode request"]
    EN_C0_PDN_0,
    #[doc = "CORE0 will be power down with low power mode request"]
    EN_C0_PDN_1,
}
impl EN_C0_PDNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_C0_PDNW::EN_C0_PDN_0 => false,
            EN_C0_PDNW::EN_C0_PDN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_C0_PDNW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_C0_PDNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_C0_PDNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CORE0 will not be power down with low power mode request"]
    #[inline]
    pub fn en_c0_pdn_0(self) -> &'a mut W {
        self.variant(EN_C0_PDNW::EN_C0_PDN_0)
    }
    #[doc = "CORE0 will be power down with low power mode request"]
    #[inline]
    pub fn en_c0_pdn_1(self) -> &'a mut W {
        self.variant(EN_C0_PDNW::EN_C0_PDN_1)
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
#[doc = "Values that can be written to the field `EN_C1_WFI_PDN`"]
pub enum EN_C1_WFI_PDNW {
    #[doc = "CORE1 will not be power down with WFI request"]
    EN_C1_WFI_PDN_0,
    #[doc = "CORE1 will be power down with WFI request"]
    EN_C1_WFI_PDN_1,
}
impl EN_C1_WFI_PDNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_C1_WFI_PDNW::EN_C1_WFI_PDN_0 => false,
            EN_C1_WFI_PDNW::EN_C1_WFI_PDN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_C1_WFI_PDNW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_C1_WFI_PDNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_C1_WFI_PDNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CORE1 will not be power down with WFI request"]
    #[inline]
    pub fn en_c1_wfi_pdn_0(self) -> &'a mut W {
        self.variant(EN_C1_WFI_PDNW::EN_C1_WFI_PDN_0)
    }
    #[doc = "CORE1 will be power down with WFI request"]
    #[inline]
    pub fn en_c1_wfi_pdn_1(self) -> &'a mut W {
        self.variant(EN_C1_WFI_PDNW::EN_C1_WFI_PDN_1)
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
#[doc = "Values that can be written to the field `EN_C1_PDN`"]
pub enum EN_C1_PDNW {
    #[doc = "CORE1 will not be power down with low power mode request"]
    EN_C1_PDN_0,
    #[doc = "CORE1 will be power down with low power mode request"]
    EN_C1_PDN_1,
}
impl EN_C1_PDNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_C1_PDNW::EN_C1_PDN_0 => false,
            EN_C1_PDNW::EN_C1_PDN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_C1_PDNW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_C1_PDNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_C1_PDNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CORE1 will not be power down with low power mode request"]
    #[inline]
    pub fn en_c1_pdn_0(self) -> &'a mut W {
        self.variant(EN_C1_PDNW::EN_C1_PDN_0)
    }
    #[doc = "CORE1 will be power down with low power mode request"]
    #[inline]
    pub fn en_c1_pdn_1(self) -> &'a mut W {
        self.variant(EN_C1_PDNW::EN_C1_PDN_1)
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
#[doc = "Values that can be written to the field `EN_PLAT_PDN`"]
pub enum EN_PLAT_PDNW {
    #[doc = "SCU and L2 cache RAM will not be power down with low power mode request"]
    EN_PLAT_PDN_0,
    #[doc = "SCU and L2 cache RAM will be power down with low power mode request"]
    EN_PLAT_PDN_1,
}
impl EN_PLAT_PDNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_PLAT_PDNW::EN_PLAT_PDN_0 => false,
            EN_PLAT_PDNW::EN_PLAT_PDN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_PLAT_PDNW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_PLAT_PDNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_PLAT_PDNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SCU and L2 cache RAM will not be power down with low power mode request"]
    #[inline]
    pub fn en_plat_pdn_0(self) -> &'a mut W {
        self.variant(EN_PLAT_PDNW::EN_PLAT_PDN_0)
    }
    #[doc = "SCU and L2 cache RAM will be power down with low power mode request"]
    #[inline]
    pub fn en_plat_pdn_1(self) -> &'a mut W {
        self.variant(EN_PLAT_PDNW::EN_PLAT_PDN_1)
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
#[doc = "Values that can be written to the field `EN_L2_WFI_PDN`"]
pub enum EN_L2_WFI_PDNW {
    #[doc = "SCU and L2 will not be power down with WFI request"]
    EN_L2_WFI_PDN_0,
    #[doc = "SCU and L2 will be power down with WFI request (default)"]
    EN_L2_WFI_PDN_1,
}
impl EN_L2_WFI_PDNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_L2_WFI_PDNW::EN_L2_WFI_PDN_0 => false,
            EN_L2_WFI_PDNW::EN_L2_WFI_PDN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_L2_WFI_PDNW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_L2_WFI_PDNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_L2_WFI_PDNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SCU and L2 will not be power down with WFI request"]
    #[inline]
    pub fn en_l2_wfi_pdn_0(self) -> &'a mut W {
        self.variant(EN_L2_WFI_PDNW::EN_L2_WFI_PDN_0)
    }
    #[doc = "SCU and L2 will be power down with WFI request (default)"]
    #[inline]
    pub fn en_l2_wfi_pdn_1(self) -> &'a mut W {
        self.variant(EN_L2_WFI_PDNW::EN_L2_WFI_PDN_1)
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
#[doc = "Values that can be written to the field `EN_C0_IRQ_PUP`"]
pub enum EN_C0_IRQ_PUPW {
    #[doc = "CORE0 will power up with IRQ request"]
    EN_C0_IRQ_PUP_0,
    #[doc = "CORE0 will not power up with IRQ request"]
    EN_C0_IRQ_PUP_1,
}
impl EN_C0_IRQ_PUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_C0_IRQ_PUPW::EN_C0_IRQ_PUP_0 => false,
            EN_C0_IRQ_PUPW::EN_C0_IRQ_PUP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_C0_IRQ_PUPW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_C0_IRQ_PUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_C0_IRQ_PUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CORE0 will power up with IRQ request"]
    #[inline]
    pub fn en_c0_irq_pup_0(self) -> &'a mut W {
        self.variant(EN_C0_IRQ_PUPW::EN_C0_IRQ_PUP_0)
    }
    #[doc = "CORE0 will not power up with IRQ request"]
    #[inline]
    pub fn en_c0_irq_pup_1(self) -> &'a mut W {
        self.variant(EN_C0_IRQ_PUPW::EN_C0_IRQ_PUP_1)
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
#[doc = "Values that can be written to the field `EN_C0_PUP`"]
pub enum EN_C0_PUPW {
    #[doc = "CORE0 will power up with low power mode request"]
    EN_C0_PUP_0,
    #[doc = "CORE0 will not power up with low power mode request"]
    EN_C0_PUP_1,
}
impl EN_C0_PUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_C0_PUPW::EN_C0_PUP_0 => false,
            EN_C0_PUPW::EN_C0_PUP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_C0_PUPW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_C0_PUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_C0_PUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CORE0 will power up with low power mode request"]
    #[inline]
    pub fn en_c0_pup_0(self) -> &'a mut W {
        self.variant(EN_C0_PUPW::EN_C0_PUP_0)
    }
    #[doc = "CORE0 will not power up with low power mode request"]
    #[inline]
    pub fn en_c0_pup_1(self) -> &'a mut W {
        self.variant(EN_C0_PUPW::EN_C0_PUP_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EN_C1_IRQ_PUP`"]
pub enum EN_C1_IRQ_PUPW {
    #[doc = "CORE1 will power up with IRQ request"]
    EN_C1_IRQ_PUP_0,
    #[doc = "CORE1 will not power up with IRQ request"]
    EN_C1_IRQ_PUP_1,
}
impl EN_C1_IRQ_PUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_C1_IRQ_PUPW::EN_C1_IRQ_PUP_0 => false,
            EN_C1_IRQ_PUPW::EN_C1_IRQ_PUP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_C1_IRQ_PUPW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_C1_IRQ_PUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_C1_IRQ_PUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CORE1 will power up with IRQ request"]
    #[inline]
    pub fn en_c1_irq_pup_0(self) -> &'a mut W {
        self.variant(EN_C1_IRQ_PUPW::EN_C1_IRQ_PUP_0)
    }
    #[doc = "CORE1 will not power up with IRQ request"]
    #[inline]
    pub fn en_c1_irq_pup_1(self) -> &'a mut W {
        self.variant(EN_C1_IRQ_PUPW::EN_C1_IRQ_PUP_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EN_C1_PUP`"]
pub enum EN_C1_PUPW {
    #[doc = "CORE1 will not power up with low power mode request (only used wake up from CPU01_OFF mode)"]
    EN_C1_PUP_0,
    #[doc = "CORE1 will power up with low power mode request"]
    EN_C1_PUP_1,
}
impl EN_C1_PUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_C1_PUPW::EN_C1_PUP_0 => false,
            EN_C1_PUPW::EN_C1_PUP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_C1_PUPW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_C1_PUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_C1_PUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CORE1 will not power up with low power mode request (only used wake up from CPU01_OFF mode)"]
    #[inline]
    pub fn en_c1_pup_0(self) -> &'a mut W {
        self.variant(EN_C1_PUPW::EN_C1_PUP_0)
    }
    #[doc = "CORE1 will power up with low power mode request"]
    #[inline]
    pub fn en_c1_pup_1(self) -> &'a mut W {
        self.variant(EN_C1_PUPW::EN_C1_PUP_1)
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
#[doc = "Values that can be written to the field `EN_C2_WFI_PDN`"]
pub enum EN_C2_WFI_PDNW {
    #[doc = "CORE2 will not be power down with WFI request"]
    EN_C2_WFI_PDN_0,
    #[doc = "CORE2 will be power down with WFI request"]
    EN_C2_WFI_PDN_1,
}
impl EN_C2_WFI_PDNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_C2_WFI_PDNW::EN_C2_WFI_PDN_0 => false,
            EN_C2_WFI_PDNW::EN_C2_WFI_PDN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_C2_WFI_PDNW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_C2_WFI_PDNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_C2_WFI_PDNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CORE2 will not be power down with WFI request"]
    #[inline]
    pub fn en_c2_wfi_pdn_0(self) -> &'a mut W {
        self.variant(EN_C2_WFI_PDNW::EN_C2_WFI_PDN_0)
    }
    #[doc = "CORE2 will be power down with WFI request"]
    #[inline]
    pub fn en_c2_wfi_pdn_1(self) -> &'a mut W {
        self.variant(EN_C2_WFI_PDNW::EN_C2_WFI_PDN_1)
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
#[doc = "Values that can be written to the field `EN_C2_PDN`"]
pub enum EN_C2_PDNW {
    #[doc = "CORE2 will not be power down with low power mode request"]
    EN_C2_PDN_0,
    #[doc = "CORE2 will be power down with low power mode request"]
    EN_C2_PDN_1,
}
impl EN_C2_PDNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_C2_PDNW::EN_C2_PDN_0 => false,
            EN_C2_PDNW::EN_C2_PDN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_C2_PDNW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_C2_PDNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_C2_PDNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CORE2 will not be power down with low power mode request"]
    #[inline]
    pub fn en_c2_pdn_0(self) -> &'a mut W {
        self.variant(EN_C2_PDNW::EN_C2_PDN_0)
    }
    #[doc = "CORE2 will be power down with low power mode request"]
    #[inline]
    pub fn en_c2_pdn_1(self) -> &'a mut W {
        self.variant(EN_C2_PDNW::EN_C2_PDN_1)
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
#[doc = "Values that can be written to the field `EN_C3_WFI_PDN`"]
pub enum EN_C3_WFI_PDNW {
    #[doc = "CORE3 will not be power down with WFI request"]
    EN_C3_WFI_PDN_0,
    #[doc = "CORE3 will be power down with WFI request"]
    EN_C3_WFI_PDN_1,
}
impl EN_C3_WFI_PDNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_C3_WFI_PDNW::EN_C3_WFI_PDN_0 => false,
            EN_C3_WFI_PDNW::EN_C3_WFI_PDN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_C3_WFI_PDNW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_C3_WFI_PDNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_C3_WFI_PDNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CORE3 will not be power down with WFI request"]
    #[inline]
    pub fn en_c3_wfi_pdn_0(self) -> &'a mut W {
        self.variant(EN_C3_WFI_PDNW::EN_C3_WFI_PDN_0)
    }
    #[doc = "CORE3 will be power down with WFI request"]
    #[inline]
    pub fn en_c3_wfi_pdn_1(self) -> &'a mut W {
        self.variant(EN_C3_WFI_PDNW::EN_C3_WFI_PDN_1)
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
#[doc = "Values that can be written to the field `EN_C3_PDN`"]
pub enum EN_C3_PDNW {
    #[doc = "CORE3 will not be power down with low power mode request"]
    EN_C3_PDN_0,
    #[doc = "CORE3 will be power down with low power mode request"]
    EN_C3_PDN_1,
}
impl EN_C3_PDNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_C3_PDNW::EN_C3_PDN_0 => false,
            EN_C3_PDNW::EN_C3_PDN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_C3_PDNW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_C3_PDNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_C3_PDNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CORE3 will not be power down with low power mode request"]
    #[inline]
    pub fn en_c3_pdn_0(self) -> &'a mut W {
        self.variant(EN_C3_PDNW::EN_C3_PDN_0)
    }
    #[doc = "CORE3 will be power down with low power mode request"]
    #[inline]
    pub fn en_c3_pdn_1(self) -> &'a mut W {
        self.variant(EN_C3_PDNW::EN_C3_PDN_1)
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
#[doc = "Values that can be written to the field `EN_C2_IRQ_PUP`"]
pub enum EN_C2_IRQ_PUPW {
    #[doc = "CORE2 will power up with IRQ request"]
    EN_C2_IRQ_PUP_0,
    #[doc = "CORE2 will not power up with IRQ request"]
    EN_C2_IRQ_PUP_1,
}
impl EN_C2_IRQ_PUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_C2_IRQ_PUPW::EN_C2_IRQ_PUP_0 => false,
            EN_C2_IRQ_PUPW::EN_C2_IRQ_PUP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_C2_IRQ_PUPW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_C2_IRQ_PUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_C2_IRQ_PUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CORE2 will power up with IRQ request"]
    #[inline]
    pub fn en_c2_irq_pup_0(self) -> &'a mut W {
        self.variant(EN_C2_IRQ_PUPW::EN_C2_IRQ_PUP_0)
    }
    #[doc = "CORE2 will not power up with IRQ request"]
    #[inline]
    pub fn en_c2_irq_pup_1(self) -> &'a mut W {
        self.variant(EN_C2_IRQ_PUPW::EN_C2_IRQ_PUP_1)
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
#[doc = "Values that can be written to the field `EN_C2_PUP`"]
pub enum EN_C2_PUPW {
    #[doc = "CORE2 will power up with lower power mode request"]
    EN_C2_PUP_0,
    #[doc = "CORE2 will not power up with low power mode request (only used wake up from CPU_OFF)"]
    EN_C2_PUP_1,
}
impl EN_C2_PUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_C2_PUPW::EN_C2_PUP_0 => false,
            EN_C2_PUPW::EN_C2_PUP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_C2_PUPW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_C2_PUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_C2_PUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CORE2 will power up with lower power mode request"]
    #[inline]
    pub fn en_c2_pup_0(self) -> &'a mut W {
        self.variant(EN_C2_PUPW::EN_C2_PUP_0)
    }
    #[doc = "CORE2 will not power up with low power mode request (only used wake up from CPU_OFF)"]
    #[inline]
    pub fn en_c2_pup_1(self) -> &'a mut W {
        self.variant(EN_C2_PUPW::EN_C2_PUP_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EN_C3_IRQ_PUP`"]
pub enum EN_C3_IRQ_PUPW {
    #[doc = "CORE3 will power up with IRQ request"]
    EN_C3_IRQ_PUP_0,
    #[doc = "CORE3 will not power up with IRQ request"]
    EN_C3_IRQ_PUP_1,
}
impl EN_C3_IRQ_PUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_C3_IRQ_PUPW::EN_C3_IRQ_PUP_0 => false,
            EN_C3_IRQ_PUPW::EN_C3_IRQ_PUP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_C3_IRQ_PUPW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_C3_IRQ_PUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_C3_IRQ_PUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CORE3 will power up with IRQ request"]
    #[inline]
    pub fn en_c3_irq_pup_0(self) -> &'a mut W {
        self.variant(EN_C3_IRQ_PUPW::EN_C3_IRQ_PUP_0)
    }
    #[doc = "CORE3 will not power up with IRQ request"]
    #[inline]
    pub fn en_c3_irq_pup_1(self) -> &'a mut W {
        self.variant(EN_C3_IRQ_PUPW::EN_C3_IRQ_PUP_1)
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
#[doc = "Values that can be written to the field `EN_C3_PUP`"]
pub enum EN_C3_PUPW {
    #[doc = "CORE3 will power up with lower power mode request"]
    EN_C3_PUP_0,
    #[doc = "CORE3 will not power up with low power mode request (only used wake up from CPU_OFF)"]
    EN_C3_PUP_1,
}
impl EN_C3_PUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_C3_PUPW::EN_C3_PUP_0 => false,
            EN_C3_PUPW::EN_C3_PUP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_C3_PUPW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_C3_PUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_C3_PUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CORE3 will power up with lower power mode request"]
    #[inline]
    pub fn en_c3_pup_0(self) -> &'a mut W {
        self.variant(EN_C3_PUPW::EN_C3_PUP_0)
    }
    #[doc = "CORE3 will not power up with low power mode request (only used wake up from CPU_OFF)"]
    #[inline]
    pub fn en_c3_pup_1(self) -> &'a mut W {
        self.variant(EN_C3_PUPW::EN_C3_PUP_1)
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
#[doc = "Values that can be written to the field `L2PGE`"]
pub enum L2PGEW {
    #[doc = "L2 cache RAM will power down with SCU power domain in A53 platform (used for ALL_OFF mode)"]
    L2PGE_0,
    #[doc = "L2 cache RAM will not power down with SCU power domain in A53 platform (used for ALL_OFF mode)"]
    L2PGE_1,
}
impl L2PGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            L2PGEW::L2PGE_0 => false,
            L2PGEW::L2PGE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _L2PGEW<'a> {
    w: &'a mut W,
}
impl<'a> _L2PGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: L2PGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "L2 cache RAM will power down with SCU power domain in A53 platform (used for ALL_OFF mode)"]
    #[inline]
    pub fn l2pge_0(self) -> &'a mut W {
        self.variant(L2PGEW::L2PGE_0)
    }
    #[doc = "L2 cache RAM will not power down with SCU power domain in A53 platform (used for ALL_OFF mode)"]
    #[inline]
    pub fn l2pge_1(self) -> &'a mut W {
        self.variant(L2PGEW::L2PGE_1)
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
    #[doc = "Bit 0 - no description available"]
    #[inline]
    pub fn en_c0_wfi_pdn(&self) -> EN_C0_WFI_PDNR {
        EN_C0_WFI_PDNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - no description available"]
    #[inline]
    pub fn en_c0_pdn(&self) -> EN_C0_PDNR {
        EN_C0_PDNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - no description available"]
    #[inline]
    pub fn en_c1_wfi_pdn(&self) -> EN_C1_WFI_PDNR {
        EN_C1_WFI_PDNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - no description available"]
    #[inline]
    pub fn en_c1_pdn(&self) -> EN_C1_PDNR {
        EN_C1_PDNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - no description available"]
    #[inline]
    pub fn en_plat_pdn(&self) -> EN_PLAT_PDNR {
        EN_PLAT_PDNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Before reset, L2 WFI is 1 and make GPC generate an error DSM request"]
    #[inline]
    pub fn en_l2_wfi_pdn(&self) -> EN_L2_WFI_PDNR {
        EN_L2_WFI_PDNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - no description available"]
    #[inline]
    pub fn en_c0_irq_pup(&self) -> EN_C0_IRQ_PUPR {
        EN_C0_IRQ_PUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - (only used wake up from CPU01_OFF mode)"]
    #[inline]
    pub fn en_c0_pup(&self) -> EN_C0_PUPR {
        EN_C0_PUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - no description available"]
    #[inline]
    pub fn en_c1_irq_pup(&self) -> EN_C1_IRQ_PUPR {
        EN_C1_IRQ_PUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - no description available"]
    #[inline]
    pub fn en_c1_pup(&self) -> EN_C1_PUPR {
        EN_C1_PUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - no description available"]
    #[inline]
    pub fn en_c2_wfi_pdn(&self) -> EN_C2_WFI_PDNR {
        EN_C2_WFI_PDNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - no description available"]
    #[inline]
    pub fn en_c2_pdn(&self) -> EN_C2_PDNR {
        EN_C2_PDNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - no description available"]
    #[inline]
    pub fn en_c3_wfi_pdn(&self) -> EN_C3_WFI_PDNR {
        EN_C3_WFI_PDNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - no description available"]
    #[inline]
    pub fn en_c3_pdn(&self) -> EN_C3_PDNR {
        EN_C3_PDNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - no description available"]
    #[inline]
    pub fn en_c2_irq_pup(&self) -> EN_C2_IRQ_PUPR {
        EN_C2_IRQ_PUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - no description available"]
    #[inline]
    pub fn en_c2_pup(&self) -> EN_C2_PUPR {
        EN_C2_PUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - no description available"]
    #[inline]
    pub fn en_c3_irq_pup(&self) -> EN_C3_IRQ_PUPR {
        EN_C3_IRQ_PUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - no description available"]
    #[inline]
    pub fn en_c3_pup(&self) -> EN_C3_PUPR {
        EN_C3_PUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - no description available"]
    #[inline]
    pub fn l2pge(&self) -> L2PGER {
        L2PGER::_from({
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
        W { bits: 32 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - no description available"]
    #[inline]
    pub fn en_c0_wfi_pdn(&mut self) -> _EN_C0_WFI_PDNW {
        _EN_C0_WFI_PDNW { w: self }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline]
    pub fn en_c0_pdn(&mut self) -> _EN_C0_PDNW {
        _EN_C0_PDNW { w: self }
    }
    #[doc = "Bit 2 - no description available"]
    #[inline]
    pub fn en_c1_wfi_pdn(&mut self) -> _EN_C1_WFI_PDNW {
        _EN_C1_WFI_PDNW { w: self }
    }
    #[doc = "Bit 3 - no description available"]
    #[inline]
    pub fn en_c1_pdn(&mut self) -> _EN_C1_PDNW {
        _EN_C1_PDNW { w: self }
    }
    #[doc = "Bit 4 - no description available"]
    #[inline]
    pub fn en_plat_pdn(&mut self) -> _EN_PLAT_PDNW {
        _EN_PLAT_PDNW { w: self }
    }
    #[doc = "Bit 5 - Before reset, L2 WFI is 1 and make GPC generate an error DSM request"]
    #[inline]
    pub fn en_l2_wfi_pdn(&mut self) -> _EN_L2_WFI_PDNW {
        _EN_L2_WFI_PDNW { w: self }
    }
    #[doc = "Bit 8 - no description available"]
    #[inline]
    pub fn en_c0_irq_pup(&mut self) -> _EN_C0_IRQ_PUPW {
        _EN_C0_IRQ_PUPW { w: self }
    }
    #[doc = "Bit 9 - (only used wake up from CPU01_OFF mode)"]
    #[inline]
    pub fn en_c0_pup(&mut self) -> _EN_C0_PUPW {
        _EN_C0_PUPW { w: self }
    }
    #[doc = "Bit 10 - no description available"]
    #[inline]
    pub fn en_c1_irq_pup(&mut self) -> _EN_C1_IRQ_PUPW {
        _EN_C1_IRQ_PUPW { w: self }
    }
    #[doc = "Bit 11 - no description available"]
    #[inline]
    pub fn en_c1_pup(&mut self) -> _EN_C1_PUPW {
        _EN_C1_PUPW { w: self }
    }
    #[doc = "Bit 16 - no description available"]
    #[inline]
    pub fn en_c2_wfi_pdn(&mut self) -> _EN_C2_WFI_PDNW {
        _EN_C2_WFI_PDNW { w: self }
    }
    #[doc = "Bit 17 - no description available"]
    #[inline]
    pub fn en_c2_pdn(&mut self) -> _EN_C2_PDNW {
        _EN_C2_PDNW { w: self }
    }
    #[doc = "Bit 18 - no description available"]
    #[inline]
    pub fn en_c3_wfi_pdn(&mut self) -> _EN_C3_WFI_PDNW {
        _EN_C3_WFI_PDNW { w: self }
    }
    #[doc = "Bit 19 - no description available"]
    #[inline]
    pub fn en_c3_pdn(&mut self) -> _EN_C3_PDNW {
        _EN_C3_PDNW { w: self }
    }
    #[doc = "Bit 24 - no description available"]
    #[inline]
    pub fn en_c2_irq_pup(&mut self) -> _EN_C2_IRQ_PUPW {
        _EN_C2_IRQ_PUPW { w: self }
    }
    #[doc = "Bit 25 - no description available"]
    #[inline]
    pub fn en_c2_pup(&mut self) -> _EN_C2_PUPW {
        _EN_C2_PUPW { w: self }
    }
    #[doc = "Bit 26 - no description available"]
    #[inline]
    pub fn en_c3_irq_pup(&mut self) -> _EN_C3_IRQ_PUPW {
        _EN_C3_IRQ_PUPW { w: self }
    }
    #[doc = "Bit 27 - no description available"]
    #[inline]
    pub fn en_c3_pup(&mut self) -> _EN_C3_PUPW {
        _EN_C3_PUPW { w: self }
    }
    #[doc = "Bit 31 - no description available"]
    #[inline]
    pub fn l2pge(&mut self) -> _L2PGEW {
        _L2PGEW { w: self }
    }
}
