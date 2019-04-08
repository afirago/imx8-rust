#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SLPCR {
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
#[doc = "Possible values of the field `BYPASS_PMIC_READY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_PMIC_READYR {
    #[doc = "Don't bypass the PMIC_READY signal - GPC will wait for its assertion during exit of low power mode if standby voltage was enabled"]
    BYPASS_PMIC_READY_0,
    #[doc = "Bypass the PMIC_READY signal - GPC will wait for its assertion during exit of low power mode if standby voltage was enabled"]
    BYPASS_PMIC_READY_1,
}
impl BYPASS_PMIC_READYR {
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
            BYPASS_PMIC_READYR::BYPASS_PMIC_READY_0 => false,
            BYPASS_PMIC_READYR::BYPASS_PMIC_READY_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BYPASS_PMIC_READYR {
        match value {
            false => BYPASS_PMIC_READYR::BYPASS_PMIC_READY_0,
            true => BYPASS_PMIC_READYR::BYPASS_PMIC_READY_1,
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS_PMIC_READY_0`"]
    #[inline]
    pub fn is_bypass_pmic_ready_0(&self) -> bool {
        *self == BYPASS_PMIC_READYR::BYPASS_PMIC_READY_0
    }
    #[doc = "Checks if the value of the field is `BYPASS_PMIC_READY_1`"]
    #[inline]
    pub fn is_bypass_pmic_ready_1(&self) -> bool {
        *self == BYPASS_PMIC_READYR::BYPASS_PMIC_READY_1
    }
}
#[doc = "Possible values of the field `SBYOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBYOSR {
    #[doc = "On chip oscillator will not be powered down, after next entrance to DSM."]
    SBYOS_0,
    #[doc = "On chip oscillator will be powered down, after next entrance to DSM. When returning from DSM, external oscillator will be enabled again, on chip oscillator will return to oscillator mode , and after oscnt count GPC will continue with the exit from DSM process."]
    SBYOS_1,
}
impl SBYOSR {
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
            SBYOSR::SBYOS_0 => false,
            SBYOSR::SBYOS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SBYOSR {
        match value {
            false => SBYOSR::SBYOS_0,
            true => SBYOSR::SBYOS_1,
        }
    }
    #[doc = "Checks if the value of the field is `SBYOS_0`"]
    #[inline]
    pub fn is_sbyos_0(&self) -> bool {
        *self == SBYOSR::SBYOS_0
    }
    #[doc = "Checks if the value of the field is `SBYOS_1`"]
    #[inline]
    pub fn is_sbyos_1(&self) -> bool {
        *self == SBYOSR::SBYOS_1
    }
}
#[doc = "Possible values of the field `VSTBY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VSTBYR {
    #[doc = "Voltage will not be changed to standby voltage after next entrance to stop mode. (PMIC_STBY_REQ will remain negated - '0')"]
    VSTBY_0,
    #[doc = "Voltage will be changed to standby voltage after next entrance to stop mode."]
    VSTBY_1,
}
impl VSTBYR {
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
            VSTBYR::VSTBY_0 => false,
            VSTBYR::VSTBY_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VSTBYR {
        match value {
            false => VSTBYR::VSTBY_0,
            true => VSTBYR::VSTBY_1,
        }
    }
    #[doc = "Checks if the value of the field is `VSTBY_0`"]
    #[inline]
    pub fn is_vstby_0(&self) -> bool {
        *self == VSTBYR::VSTBY_0
    }
    #[doc = "Checks if the value of the field is `VSTBY_1`"]
    #[inline]
    pub fn is_vstby_1(&self) -> bool {
        *self == VSTBYR::VSTBY_1
    }
}
#[doc = "Possible values of the field `STBY_COUNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STBY_COUNTR {
    #[doc = "GPC will wait 4 ckil clock cycles"]
    STBY_COUNT_0,
    #[doc = "GPC will wait 8 ckil clock cycles"]
    STBY_COUNT_1,
    #[doc = "GPC will wait 16 ckil clock cycles"]
    STBY_COUNT_2,
    #[doc = "GPC will wait 32 ckil clock cycles"]
    STBY_COUNT_3,
    #[doc = "GPC will wait 64 ckil clock cycles"]
    STBY_COUNT_4,
    #[doc = "GPC will wait 128 ckil clock cycles"]
    STBY_COUNT_5,
    #[doc = "GPC will wait 256 ckil clock cycles"]
    STBY_COUNT_6,
    #[doc = "GPC will wait 512 ckil clock cycles"]
    STBY_COUNT_7,
}
impl STBY_COUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STBY_COUNTR::STBY_COUNT_0 => 0,
            STBY_COUNTR::STBY_COUNT_1 => 1,
            STBY_COUNTR::STBY_COUNT_2 => 2,
            STBY_COUNTR::STBY_COUNT_3 => 3,
            STBY_COUNTR::STBY_COUNT_4 => 4,
            STBY_COUNTR::STBY_COUNT_5 => 5,
            STBY_COUNTR::STBY_COUNT_6 => 6,
            STBY_COUNTR::STBY_COUNT_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STBY_COUNTR {
        match value {
            0 => STBY_COUNTR::STBY_COUNT_0,
            1 => STBY_COUNTR::STBY_COUNT_1,
            2 => STBY_COUNTR::STBY_COUNT_2,
            3 => STBY_COUNTR::STBY_COUNT_3,
            4 => STBY_COUNTR::STBY_COUNT_4,
            5 => STBY_COUNTR::STBY_COUNT_5,
            6 => STBY_COUNTR::STBY_COUNT_6,
            7 => STBY_COUNTR::STBY_COUNT_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STBY_COUNT_0`"]
    #[inline]
    pub fn is_stby_count_0(&self) -> bool {
        *self == STBY_COUNTR::STBY_COUNT_0
    }
    #[doc = "Checks if the value of the field is `STBY_COUNT_1`"]
    #[inline]
    pub fn is_stby_count_1(&self) -> bool {
        *self == STBY_COUNTR::STBY_COUNT_1
    }
    #[doc = "Checks if the value of the field is `STBY_COUNT_2`"]
    #[inline]
    pub fn is_stby_count_2(&self) -> bool {
        *self == STBY_COUNTR::STBY_COUNT_2
    }
    #[doc = "Checks if the value of the field is `STBY_COUNT_3`"]
    #[inline]
    pub fn is_stby_count_3(&self) -> bool {
        *self == STBY_COUNTR::STBY_COUNT_3
    }
    #[doc = "Checks if the value of the field is `STBY_COUNT_4`"]
    #[inline]
    pub fn is_stby_count_4(&self) -> bool {
        *self == STBY_COUNTR::STBY_COUNT_4
    }
    #[doc = "Checks if the value of the field is `STBY_COUNT_5`"]
    #[inline]
    pub fn is_stby_count_5(&self) -> bool {
        *self == STBY_COUNTR::STBY_COUNT_5
    }
    #[doc = "Checks if the value of the field is `STBY_COUNT_6`"]
    #[inline]
    pub fn is_stby_count_6(&self) -> bool {
        *self == STBY_COUNTR::STBY_COUNT_6
    }
    #[doc = "Checks if the value of the field is `STBY_COUNT_7`"]
    #[inline]
    pub fn is_stby_count_7(&self) -> bool {
        *self == STBY_COUNTR::STBY_COUNT_7
    }
}
#[doc = "Possible values of the field `COSC_PWRDOWN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COSC_PWRDOWNR {
    #[doc = "On-chip oscillator will not be powered down, i.e. cosc_pwrdown = 0"]
    COSC_PWRDOWN_0,
    #[doc = "On-chip oscillator will be powered down, i.e. cosc_pwrdown = 1"]
    COSC_PWRDOWN_1,
}
impl COSC_PWRDOWNR {
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
            COSC_PWRDOWNR::COSC_PWRDOWN_0 => false,
            COSC_PWRDOWNR::COSC_PWRDOWN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COSC_PWRDOWNR {
        match value {
            false => COSC_PWRDOWNR::COSC_PWRDOWN_0,
            true => COSC_PWRDOWNR::COSC_PWRDOWN_1,
        }
    }
    #[doc = "Checks if the value of the field is `COSC_PWRDOWN_0`"]
    #[inline]
    pub fn is_cosc_pwrdown_0(&self) -> bool {
        *self == COSC_PWRDOWNR::COSC_PWRDOWN_0
    }
    #[doc = "Checks if the value of the field is `COSC_PWRDOWN_1`"]
    #[inline]
    pub fn is_cosc_pwrdown_1(&self) -> bool {
        *self == COSC_PWRDOWNR::COSC_PWRDOWN_1
    }
}
#[doc = "Possible values of the field `COSC_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COSC_ENR {
    #[doc = "Disable on-chip oscillator"]
    COSC_EN_0,
    #[doc = "Enable on-chip oscillator"]
    COSC_EN_1,
}
impl COSC_ENR {
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
            COSC_ENR::COSC_EN_0 => false,
            COSC_ENR::COSC_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COSC_ENR {
        match value {
            false => COSC_ENR::COSC_EN_0,
            true => COSC_ENR::COSC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `COSC_EN_0`"]
    #[inline]
    pub fn is_cosc_en_0(&self) -> bool {
        *self == COSC_ENR::COSC_EN_0
    }
    #[doc = "Checks if the value of the field is `COSC_EN_1`"]
    #[inline]
    pub fn is_cosc_en_1(&self) -> bool {
        *self == COSC_ENR::COSC_EN_1
    }
}
#[doc = "Possible values of the field `OSCCNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCCNTR {
    #[doc = "count 1 ckil"]
    OSCCNT_0,
    #[doc = "count 256 ckils"]
    OSCCNT_255,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OSCCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OSCCNTR::OSCCNT_0 => 0,
            OSCCNTR::OSCCNT_255 => 255,
            OSCCNTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OSCCNTR {
        match value {
            0 => OSCCNTR::OSCCNT_0,
            255 => OSCCNTR::OSCCNT_255,
            i => OSCCNTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OSCCNT_0`"]
    #[inline]
    pub fn is_osccnt_0(&self) -> bool {
        *self == OSCCNTR::OSCCNT_0
    }
    #[doc = "Checks if the value of the field is `OSCCNT_255`"]
    #[inline]
    pub fn is_osccnt_255(&self) -> bool {
        *self == OSCCNTR::OSCCNT_255
    }
}
#[doc = r" Value of the field"]
pub struct EN_A53_FASTWUP_WAIT_MODER {
    bits: bool,
}
impl EN_A53_FASTWUP_WAIT_MODER {
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
pub struct EN_A53_FASTWUP_STOP_MODER {
    bits: bool,
}
impl EN_A53_FASTWUP_STOP_MODER {
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
pub struct EN_M4_FASTWUP_WAIT_MODER {
    bits: bool,
}
impl EN_M4_FASTWUP_WAIT_MODER {
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
pub struct EN_M4_FASTWUP_STOP_MODER {
    bits: bool,
}
impl EN_M4_FASTWUP_STOP_MODER {
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
#[doc = "Possible values of the field `DISABLE_A53_IS_DSM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLE_A53_IS_DSMR {
    #[doc = "Enable A53 isolation signal in DSM"]
    DISABLE_A53_IS_DSM_0,
    #[doc = "Disable A53 isolation signal in DSM"]
    DISABLE_A53_IS_DSM_1,
}
impl DISABLE_A53_IS_DSMR {
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
            DISABLE_A53_IS_DSMR::DISABLE_A53_IS_DSM_0 => false,
            DISABLE_A53_IS_DSMR::DISABLE_A53_IS_DSM_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISABLE_A53_IS_DSMR {
        match value {
            false => DISABLE_A53_IS_DSMR::DISABLE_A53_IS_DSM_0,
            true => DISABLE_A53_IS_DSMR::DISABLE_A53_IS_DSM_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_A53_IS_DSM_0`"]
    #[inline]
    pub fn is_disable_a53_is_dsm_0(&self) -> bool {
        *self == DISABLE_A53_IS_DSMR::DISABLE_A53_IS_DSM_0
    }
    #[doc = "Checks if the value of the field is `DISABLE_A53_IS_DSM_1`"]
    #[inline]
    pub fn is_disable_a53_is_dsm_1(&self) -> bool {
        *self == DISABLE_A53_IS_DSMR::DISABLE_A53_IS_DSM_1
    }
}
#[doc = "Possible values of the field `REG_BYPASS_COUNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_BYPASS_COUNTR {
    #[doc = "no delay"]
    REG_BYPASS_COUNT_0,
    #[doc = "1 CKIL clock period delay"]
    REG_BYPASS_COUNT_1,
    #[doc = "63 CKIL clock period delay"]
    REG_BYPASS_COUNT_63,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REG_BYPASS_COUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REG_BYPASS_COUNTR::REG_BYPASS_COUNT_0 => 0,
            REG_BYPASS_COUNTR::REG_BYPASS_COUNT_1 => 1,
            REG_BYPASS_COUNTR::REG_BYPASS_COUNT_63 => 63,
            REG_BYPASS_COUNTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REG_BYPASS_COUNTR {
        match value {
            0 => REG_BYPASS_COUNTR::REG_BYPASS_COUNT_0,
            1 => REG_BYPASS_COUNTR::REG_BYPASS_COUNT_1,
            63 => REG_BYPASS_COUNTR::REG_BYPASS_COUNT_63,
            i => REG_BYPASS_COUNTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `REG_BYPASS_COUNT_0`"]
    #[inline]
    pub fn is_reg_bypass_count_0(&self) -> bool {
        *self == REG_BYPASS_COUNTR::REG_BYPASS_COUNT_0
    }
    #[doc = "Checks if the value of the field is `REG_BYPASS_COUNT_1`"]
    #[inline]
    pub fn is_reg_bypass_count_1(&self) -> bool {
        *self == REG_BYPASS_COUNTR::REG_BYPASS_COUNT_1
    }
    #[doc = "Checks if the value of the field is `REG_BYPASS_COUNT_63`"]
    #[inline]
    pub fn is_reg_bypass_count_63(&self) -> bool {
        *self == REG_BYPASS_COUNTR::REG_BYPASS_COUNT_63
    }
}
#[doc = "Possible values of the field `RBC_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBC_ENR {
    #[doc = "REG_BYPASS_COUNTER disabled"]
    RBC_EN_0,
    #[doc = "REG_BYPASS_COUNTER enabled"]
    RBC_EN_1,
}
impl RBC_ENR {
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
            RBC_ENR::RBC_EN_0 => false,
            RBC_ENR::RBC_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RBC_ENR {
        match value {
            false => RBC_ENR::RBC_EN_0,
            true => RBC_ENR::RBC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RBC_EN_0`"]
    #[inline]
    pub fn is_rbc_en_0(&self) -> bool {
        *self == RBC_ENR::RBC_EN_0
    }
    #[doc = "Checks if the value of the field is `RBC_EN_1`"]
    #[inline]
    pub fn is_rbc_en_1(&self) -> bool {
        *self == RBC_ENR::RBC_EN_1
    }
}
#[doc = "Possible values of the field `EN_DSM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_DSMR {
    #[doc = "DSM disabled"]
    EN_DSM_0,
    #[doc = "DSM enabled"]
    EN_DSM_1,
}
impl EN_DSMR {
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
            EN_DSMR::EN_DSM_0 => false,
            EN_DSMR::EN_DSM_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_DSMR {
        match value {
            false => EN_DSMR::EN_DSM_0,
            true => EN_DSMR::EN_DSM_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_DSM_0`"]
    #[inline]
    pub fn is_en_dsm_0(&self) -> bool {
        *self == EN_DSMR::EN_DSM_0
    }
    #[doc = "Checks if the value of the field is `EN_DSM_1`"]
    #[inline]
    pub fn is_en_dsm_1(&self) -> bool {
        *self == EN_DSMR::EN_DSM_1
    }
}
#[doc = "Values that can be written to the field `BYPASS_PMIC_READY`"]
pub enum BYPASS_PMIC_READYW {
    #[doc = "Don't bypass the PMIC_READY signal - GPC will wait for its assertion during exit of low power mode if standby voltage was enabled"]
    BYPASS_PMIC_READY_0,
    #[doc = "Bypass the PMIC_READY signal - GPC will wait for its assertion during exit of low power mode if standby voltage was enabled"]
    BYPASS_PMIC_READY_1,
}
impl BYPASS_PMIC_READYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BYPASS_PMIC_READYW::BYPASS_PMIC_READY_0 => false,
            BYPASS_PMIC_READYW::BYPASS_PMIC_READY_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BYPASS_PMIC_READYW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASS_PMIC_READYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BYPASS_PMIC_READYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Don't bypass the PMIC_READY signal - GPC will wait for its assertion during exit of low power mode if standby voltage was enabled"]
    #[inline]
    pub fn bypass_pmic_ready_0(self) -> &'a mut W {
        self.variant(BYPASS_PMIC_READYW::BYPASS_PMIC_READY_0)
    }
    #[doc = "Bypass the PMIC_READY signal - GPC will wait for its assertion during exit of low power mode if standby voltage was enabled"]
    #[inline]
    pub fn bypass_pmic_ready_1(self) -> &'a mut W {
        self.variant(BYPASS_PMIC_READYW::BYPASS_PMIC_READY_1)
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
#[doc = "Values that can be written to the field `SBYOS`"]
pub enum SBYOSW {
    #[doc = "On chip oscillator will not be powered down, after next entrance to DSM."]
    SBYOS_0,
    #[doc = "On chip oscillator will be powered down, after next entrance to DSM. When returning from DSM, external oscillator will be enabled again, on chip oscillator will return to oscillator mode , and after oscnt count GPC will continue with the exit from DSM process."]
    SBYOS_1,
}
impl SBYOSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SBYOSW::SBYOS_0 => false,
            SBYOSW::SBYOS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SBYOSW<'a> {
    w: &'a mut W,
}
impl<'a> _SBYOSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SBYOSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "On chip oscillator will not be powered down, after next entrance to DSM."]
    #[inline]
    pub fn sbyos_0(self) -> &'a mut W {
        self.variant(SBYOSW::SBYOS_0)
    }
    #[doc = "On chip oscillator will be powered down, after next entrance to DSM. When returning from DSM, external oscillator will be enabled again, on chip oscillator will return to oscillator mode , and after oscnt count GPC will continue with the exit from DSM process."]
    #[inline]
    pub fn sbyos_1(self) -> &'a mut W {
        self.variant(SBYOSW::SBYOS_1)
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
#[doc = "Values that can be written to the field `VSTBY`"]
pub enum VSTBYW {
    #[doc = "Voltage will not be changed to standby voltage after next entrance to stop mode. (PMIC_STBY_REQ will remain negated - '0')"]
    VSTBY_0,
    #[doc = "Voltage will be changed to standby voltage after next entrance to stop mode."]
    VSTBY_1,
}
impl VSTBYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VSTBYW::VSTBY_0 => false,
            VSTBYW::VSTBY_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VSTBYW<'a> {
    w: &'a mut W,
}
impl<'a> _VSTBYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VSTBYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Voltage will not be changed to standby voltage after next entrance to stop mode. (PMIC_STBY_REQ will remain negated - '0')"]
    #[inline]
    pub fn vstby_0(self) -> &'a mut W {
        self.variant(VSTBYW::VSTBY_0)
    }
    #[doc = "Voltage will be changed to standby voltage after next entrance to stop mode."]
    #[inline]
    pub fn vstby_1(self) -> &'a mut W {
        self.variant(VSTBYW::VSTBY_1)
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
#[doc = "Values that can be written to the field `STBY_COUNT`"]
pub enum STBY_COUNTW {
    #[doc = "GPC will wait 4 ckil clock cycles"]
    STBY_COUNT_0,
    #[doc = "GPC will wait 8 ckil clock cycles"]
    STBY_COUNT_1,
    #[doc = "GPC will wait 16 ckil clock cycles"]
    STBY_COUNT_2,
    #[doc = "GPC will wait 32 ckil clock cycles"]
    STBY_COUNT_3,
    #[doc = "GPC will wait 64 ckil clock cycles"]
    STBY_COUNT_4,
    #[doc = "GPC will wait 128 ckil clock cycles"]
    STBY_COUNT_5,
    #[doc = "GPC will wait 256 ckil clock cycles"]
    STBY_COUNT_6,
    #[doc = "GPC will wait 512 ckil clock cycles"]
    STBY_COUNT_7,
}
impl STBY_COUNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STBY_COUNTW::STBY_COUNT_0 => 0,
            STBY_COUNTW::STBY_COUNT_1 => 1,
            STBY_COUNTW::STBY_COUNT_2 => 2,
            STBY_COUNTW::STBY_COUNT_3 => 3,
            STBY_COUNTW::STBY_COUNT_4 => 4,
            STBY_COUNTW::STBY_COUNT_5 => 5,
            STBY_COUNTW::STBY_COUNT_6 => 6,
            STBY_COUNTW::STBY_COUNT_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STBY_COUNTW<'a> {
    w: &'a mut W,
}
impl<'a> _STBY_COUNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STBY_COUNTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "GPC will wait 4 ckil clock cycles"]
    #[inline]
    pub fn stby_count_0(self) -> &'a mut W {
        self.variant(STBY_COUNTW::STBY_COUNT_0)
    }
    #[doc = "GPC will wait 8 ckil clock cycles"]
    #[inline]
    pub fn stby_count_1(self) -> &'a mut W {
        self.variant(STBY_COUNTW::STBY_COUNT_1)
    }
    #[doc = "GPC will wait 16 ckil clock cycles"]
    #[inline]
    pub fn stby_count_2(self) -> &'a mut W {
        self.variant(STBY_COUNTW::STBY_COUNT_2)
    }
    #[doc = "GPC will wait 32 ckil clock cycles"]
    #[inline]
    pub fn stby_count_3(self) -> &'a mut W {
        self.variant(STBY_COUNTW::STBY_COUNT_3)
    }
    #[doc = "GPC will wait 64 ckil clock cycles"]
    #[inline]
    pub fn stby_count_4(self) -> &'a mut W {
        self.variant(STBY_COUNTW::STBY_COUNT_4)
    }
    #[doc = "GPC will wait 128 ckil clock cycles"]
    #[inline]
    pub fn stby_count_5(self) -> &'a mut W {
        self.variant(STBY_COUNTW::STBY_COUNT_5)
    }
    #[doc = "GPC will wait 256 ckil clock cycles"]
    #[inline]
    pub fn stby_count_6(self) -> &'a mut W {
        self.variant(STBY_COUNTW::STBY_COUNT_6)
    }
    #[doc = "GPC will wait 512 ckil clock cycles"]
    #[inline]
    pub fn stby_count_7(self) -> &'a mut W {
        self.variant(STBY_COUNTW::STBY_COUNT_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COSC_PWRDOWN`"]
pub enum COSC_PWRDOWNW {
    #[doc = "On-chip oscillator will not be powered down, i.e. cosc_pwrdown = 0"]
    COSC_PWRDOWN_0,
    #[doc = "On-chip oscillator will be powered down, i.e. cosc_pwrdown = 1"]
    COSC_PWRDOWN_1,
}
impl COSC_PWRDOWNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COSC_PWRDOWNW::COSC_PWRDOWN_0 => false,
            COSC_PWRDOWNW::COSC_PWRDOWN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COSC_PWRDOWNW<'a> {
    w: &'a mut W,
}
impl<'a> _COSC_PWRDOWNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COSC_PWRDOWNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "On-chip oscillator will not be powered down, i.e. cosc_pwrdown = 0"]
    #[inline]
    pub fn cosc_pwrdown_0(self) -> &'a mut W {
        self.variant(COSC_PWRDOWNW::COSC_PWRDOWN_0)
    }
    #[doc = "On-chip oscillator will be powered down, i.e. cosc_pwrdown = 1"]
    #[inline]
    pub fn cosc_pwrdown_1(self) -> &'a mut W {
        self.variant(COSC_PWRDOWNW::COSC_PWRDOWN_1)
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
#[doc = "Values that can be written to the field `COSC_EN`"]
pub enum COSC_ENW {
    #[doc = "Disable on-chip oscillator"]
    COSC_EN_0,
    #[doc = "Enable on-chip oscillator"]
    COSC_EN_1,
}
impl COSC_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COSC_ENW::COSC_EN_0 => false,
            COSC_ENW::COSC_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COSC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _COSC_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COSC_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable on-chip oscillator"]
    #[inline]
    pub fn cosc_en_0(self) -> &'a mut W {
        self.variant(COSC_ENW::COSC_EN_0)
    }
    #[doc = "Enable on-chip oscillator"]
    #[inline]
    pub fn cosc_en_1(self) -> &'a mut W {
        self.variant(COSC_ENW::COSC_EN_1)
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
#[doc = "Values that can be written to the field `OSCCNT`"]
pub enum OSCCNTW {
    #[doc = "count 1 ckil"]
    OSCCNT_0,
    #[doc = "count 256 ckils"]
    OSCCNT_255,
}
impl OSCCNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OSCCNTW::OSCCNT_0 => 0,
            OSCCNTW::OSCCNT_255 => 255,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSCCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCCNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSCCNTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "count 1 ckil"]
    #[inline]
    pub fn osccnt_0(self) -> &'a mut W {
        self.variant(OSCCNTW::OSCCNT_0)
    }
    #[doc = "count 256 ckils"]
    #[inline]
    pub fn osccnt_255(self) -> &'a mut W {
        self.variant(OSCCNTW::OSCCNT_255)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EN_A53_FASTWUP_WAIT_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_A53_FASTWUP_WAIT_MODEW<'a> {
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
pub struct _EN_A53_FASTWUP_STOP_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_A53_FASTWUP_STOP_MODEW<'a> {
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
pub struct _EN_M4_FASTWUP_WAIT_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_M4_FASTWUP_WAIT_MODEW<'a> {
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
pub struct _EN_M4_FASTWUP_STOP_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_M4_FASTWUP_STOP_MODEW<'a> {
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
#[doc = "Values that can be written to the field `DISABLE_A53_IS_DSM`"]
pub enum DISABLE_A53_IS_DSMW {
    #[doc = "Enable A53 isolation signal in DSM"]
    DISABLE_A53_IS_DSM_0,
    #[doc = "Disable A53 isolation signal in DSM"]
    DISABLE_A53_IS_DSM_1,
}
impl DISABLE_A53_IS_DSMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DISABLE_A53_IS_DSMW::DISABLE_A53_IS_DSM_0 => false,
            DISABLE_A53_IS_DSMW::DISABLE_A53_IS_DSM_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISABLE_A53_IS_DSMW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLE_A53_IS_DSMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISABLE_A53_IS_DSMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable A53 isolation signal in DSM"]
    #[inline]
    pub fn disable_a53_is_dsm_0(self) -> &'a mut W {
        self.variant(DISABLE_A53_IS_DSMW::DISABLE_A53_IS_DSM_0)
    }
    #[doc = "Disable A53 isolation signal in DSM"]
    #[inline]
    pub fn disable_a53_is_dsm_1(self) -> &'a mut W {
        self.variant(DISABLE_A53_IS_DSMW::DISABLE_A53_IS_DSM_1)
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
#[doc = "Values that can be written to the field `REG_BYPASS_COUNT`"]
pub enum REG_BYPASS_COUNTW {
    #[doc = "no delay"]
    REG_BYPASS_COUNT_0,
    #[doc = "1 CKIL clock period delay"]
    REG_BYPASS_COUNT_1,
    #[doc = "63 CKIL clock period delay"]
    REG_BYPASS_COUNT_63,
}
impl REG_BYPASS_COUNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REG_BYPASS_COUNTW::REG_BYPASS_COUNT_0 => 0,
            REG_BYPASS_COUNTW::REG_BYPASS_COUNT_1 => 1,
            REG_BYPASS_COUNTW::REG_BYPASS_COUNT_63 => 63,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REG_BYPASS_COUNTW<'a> {
    w: &'a mut W,
}
impl<'a> _REG_BYPASS_COUNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REG_BYPASS_COUNTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "no delay"]
    #[inline]
    pub fn reg_bypass_count_0(self) -> &'a mut W {
        self.variant(REG_BYPASS_COUNTW::REG_BYPASS_COUNT_0)
    }
    #[doc = "1 CKIL clock period delay"]
    #[inline]
    pub fn reg_bypass_count_1(self) -> &'a mut W {
        self.variant(REG_BYPASS_COUNTW::REG_BYPASS_COUNT_1)
    }
    #[doc = "63 CKIL clock period delay"]
    #[inline]
    pub fn reg_bypass_count_63(self) -> &'a mut W {
        self.variant(REG_BYPASS_COUNTW::REG_BYPASS_COUNT_63)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RBC_EN`"]
pub enum RBC_ENW {
    #[doc = "REG_BYPASS_COUNTER disabled"]
    RBC_EN_0,
    #[doc = "REG_BYPASS_COUNTER enabled"]
    RBC_EN_1,
}
impl RBC_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RBC_ENW::RBC_EN_0 => false,
            RBC_ENW::RBC_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RBC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RBC_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RBC_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "REG_BYPASS_COUNTER disabled"]
    #[inline]
    pub fn rbc_en_0(self) -> &'a mut W {
        self.variant(RBC_ENW::RBC_EN_0)
    }
    #[doc = "REG_BYPASS_COUNTER enabled"]
    #[inline]
    pub fn rbc_en_1(self) -> &'a mut W {
        self.variant(RBC_ENW::RBC_EN_1)
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
#[doc = "Values that can be written to the field `EN_DSM`"]
pub enum EN_DSMW {
    #[doc = "DSM disabled"]
    EN_DSM_0,
    #[doc = "DSM enabled"]
    EN_DSM_1,
}
impl EN_DSMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_DSMW::EN_DSM_0 => false,
            EN_DSMW::EN_DSM_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_DSMW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_DSMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_DSMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DSM disabled"]
    #[inline]
    pub fn en_dsm_0(self) -> &'a mut W {
        self.variant(EN_DSMW::EN_DSM_0)
    }
    #[doc = "DSM enabled"]
    #[inline]
    pub fn en_dsm_1(self) -> &'a mut W {
        self.variant(EN_DSMW::EN_DSM_1)
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
    #[doc = "Bit 0 - By asserting this bit GPC will bypass waiting for PMIC_READY signal when coming out of DSM"]
    #[inline]
    pub fn bypass_pmic_ready(&self) -> BYPASS_PMIC_READYR {
        BYPASS_PMIC_READYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Standby clock oscillator bit"]
    #[inline]
    pub fn sbyos(&self) -> SBYOSR {
        SBYOSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Voltage standby request bit"]
    #[inline]
    pub fn vstby(&self) -> VSTBYR {
        VSTBYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:5 - Standby counter definition"]
    #[inline]
    pub fn stby_count(&self) -> STBY_COUNTR {
        STBY_COUNTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - In run mode, software can manually control powering down of on chip oscillator, i"]
    #[inline]
    pub fn cosc_pwrdown(&self) -> COSC_PWRDOWNR {
        COSC_PWRDOWNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - On-chip oscillator enable bit - this bit value is reflected on the output cosc_en"]
    #[inline]
    pub fn cosc_en(&self) -> COSC_ENR {
        COSC_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:15 - Oscillator ready counter value"]
    #[inline]
    pub fn osccnt(&self) -> OSCCNTR {
        OSCCNTR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Enable A53 fast wake up wait mode, relevant PLLs will not be closed in this mode."]
    #[inline]
    pub fn en_a53_fastwup_wait_mode(&self) -> EN_A53_FASTWUP_WAIT_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EN_A53_FASTWUP_WAIT_MODER { bits }
    }
    #[doc = "Bit 17 - Enable A53 fast wake up stop mode, relevant PLLs will not be closed in this mode."]
    #[inline]
    pub fn en_a53_fastwup_stop_mode(&self) -> EN_A53_FASTWUP_STOP_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EN_A53_FASTWUP_STOP_MODER { bits }
    }
    #[doc = "Bit 18 - Enable M4 fast wake up wait mode, relevant PLLs will not be closed in this mode."]
    #[inline]
    pub fn en_m4_fastwup_wait_mode(&self) -> EN_M4_FASTWUP_WAIT_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EN_M4_FASTWUP_WAIT_MODER { bits }
    }
    #[doc = "Bit 19 - Enable M4 fast wake up stop mode, relevant PLLs will not be closed in this mode."]
    #[inline]
    pub fn en_m4_fastwup_stop_mode(&self) -> EN_M4_FASTWUP_STOP_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EN_M4_FASTWUP_STOP_MODER { bits }
    }
    #[doc = "Bit 23 - no description available"]
    #[inline]
    pub fn disable_a53_is_dsm(&self) -> DISABLE_A53_IS_DSMR {
        DISABLE_A53_IS_DSMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:29 - Counter for REG_BYPASS signal assertion after standby voltage request by PMIC_STBY_REQ."]
    #[inline]
    pub fn reg_bypass_count(&self) -> REG_BYPASS_COUNTR {
        REG_BYPASS_COUNTR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 30 - Enable for REG_BYPASS_COUNTER"]
    #[inline]
    pub fn rbc_en(&self) -> RBC_ENR {
        RBC_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - DSM enable"]
    #[inline]
    pub fn en_dsm(&self) -> EN_DSMR {
        EN_DSMR::_from({
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
        W { bits: 3758161794 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - By asserting this bit GPC will bypass waiting for PMIC_READY signal when coming out of DSM"]
    #[inline]
    pub fn bypass_pmic_ready(&mut self) -> _BYPASS_PMIC_READYW {
        _BYPASS_PMIC_READYW { w: self }
    }
    #[doc = "Bit 1 - Standby clock oscillator bit"]
    #[inline]
    pub fn sbyos(&mut self) -> _SBYOSW {
        _SBYOSW { w: self }
    }
    #[doc = "Bit 2 - Voltage standby request bit"]
    #[inline]
    pub fn vstby(&mut self) -> _VSTBYW {
        _VSTBYW { w: self }
    }
    #[doc = "Bits 3:5 - Standby counter definition"]
    #[inline]
    pub fn stby_count(&mut self) -> _STBY_COUNTW {
        _STBY_COUNTW { w: self }
    }
    #[doc = "Bit 6 - In run mode, software can manually control powering down of on chip oscillator, i"]
    #[inline]
    pub fn cosc_pwrdown(&mut self) -> _COSC_PWRDOWNW {
        _COSC_PWRDOWNW { w: self }
    }
    #[doc = "Bit 7 - On-chip oscillator enable bit - this bit value is reflected on the output cosc_en"]
    #[inline]
    pub fn cosc_en(&mut self) -> _COSC_ENW {
        _COSC_ENW { w: self }
    }
    #[doc = "Bits 8:15 - Oscillator ready counter value"]
    #[inline]
    pub fn osccnt(&mut self) -> _OSCCNTW {
        _OSCCNTW { w: self }
    }
    #[doc = "Bit 16 - Enable A53 fast wake up wait mode, relevant PLLs will not be closed in this mode."]
    #[inline]
    pub fn en_a53_fastwup_wait_mode(&mut self) -> _EN_A53_FASTWUP_WAIT_MODEW {
        _EN_A53_FASTWUP_WAIT_MODEW { w: self }
    }
    #[doc = "Bit 17 - Enable A53 fast wake up stop mode, relevant PLLs will not be closed in this mode."]
    #[inline]
    pub fn en_a53_fastwup_stop_mode(&mut self) -> _EN_A53_FASTWUP_STOP_MODEW {
        _EN_A53_FASTWUP_STOP_MODEW { w: self }
    }
    #[doc = "Bit 18 - Enable M4 fast wake up wait mode, relevant PLLs will not be closed in this mode."]
    #[inline]
    pub fn en_m4_fastwup_wait_mode(&mut self) -> _EN_M4_FASTWUP_WAIT_MODEW {
        _EN_M4_FASTWUP_WAIT_MODEW { w: self }
    }
    #[doc = "Bit 19 - Enable M4 fast wake up stop mode, relevant PLLs will not be closed in this mode."]
    #[inline]
    pub fn en_m4_fastwup_stop_mode(&mut self) -> _EN_M4_FASTWUP_STOP_MODEW {
        _EN_M4_FASTWUP_STOP_MODEW { w: self }
    }
    #[doc = "Bit 23 - no description available"]
    #[inline]
    pub fn disable_a53_is_dsm(&mut self) -> _DISABLE_A53_IS_DSMW {
        _DISABLE_A53_IS_DSMW { w: self }
    }
    #[doc = "Bits 24:29 - Counter for REG_BYPASS signal assertion after standby voltage request by PMIC_STBY_REQ."]
    #[inline]
    pub fn reg_bypass_count(&mut self) -> _REG_BYPASS_COUNTW {
        _REG_BYPASS_COUNTW { w: self }
    }
    #[doc = "Bit 30 - Enable for REG_BYPASS_COUNTER"]
    #[inline]
    pub fn rbc_en(&mut self) -> _RBC_ENW {
        _RBC_ENW { w: self }
    }
    #[doc = "Bit 31 - DSM enable"]
    #[inline]
    pub fn en_dsm(&mut self) -> _EN_DSMW {
        _EN_DSMW { w: self }
    }
}
