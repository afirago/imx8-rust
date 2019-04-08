#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCIEPHY_RCR {
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
#[doc = r" Value of the field"]
pub struct PCIE_PHY_POWER_ON_RESETR {
    bits: bool,
}
impl PCIE_PHY_POWER_ON_RESETR {
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
pub struct PCIEPHY_BTNRSTR {
    bits: bool,
}
impl PCIEPHY_BTNRSTR {
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
pub struct PCIEPHY_PERSTR {
    bits: bool,
}
impl PCIEPHY_PERSTR {
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
pub struct PCIE_CTRL_APPS_CLK_REQR {
    bits: bool,
}
impl PCIE_CTRL_APPS_CLK_REQR {
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
pub struct PCIE_CTRL_APPS_RSTR {
    bits: bool,
}
impl PCIE_CTRL_APPS_RSTR {
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
pub struct PCIE_CTRL_APPS_ENR {
    bits: bool,
}
impl PCIE_CTRL_APPS_ENR {
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
pub struct PCIE_CTRL_APPS_READYR {
    bits: bool,
}
impl PCIE_CTRL_APPS_READYR {
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
pub struct PCIE_CTRL_APPS_ENTERR {
    bits: bool,
}
impl PCIE_CTRL_APPS_ENTERR {
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
pub struct PCIE_CTRL_APPS_EXITR {
    bits: bool,
}
impl PCIE_CTRL_APPS_EXITR {
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
pub struct PCIE_CTRL_APPS_PMER {
    bits: bool,
}
impl PCIE_CTRL_APPS_PMER {
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
pub struct PCIE_CTRL_APPS_TURNOFFR {
    bits: bool,
}
impl PCIE_CTRL_APPS_TURNOFFR {
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
pub struct PCIE_CTRL_CFG_L1_AUXR {
    bits: bool,
}
impl PCIE_CTRL_CFG_L1_AUXR {
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
pub struct PCIE_CTRL_SYS_INTR {
    bits: bool,
}
impl PCIE_CTRL_SYS_INTR {
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
pub struct PCIE_CTRL_APP_UNLOCK_MSGR {
    bits: bool,
}
impl PCIE_CTRL_APP_UNLOCK_MSGR {
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
pub struct PCIE_CTRL_APP_XFER_PENDINGR {
    bits: bool,
}
impl PCIE_CTRL_APP_XFER_PENDINGR {
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
#[doc = "Possible values of the field `DOMAIN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOMAIN0R {
    #[doc = "This register is not assigned to domain0. The master from domain3 cannot write to this register."]
    DOMAIN0_0,
    #[doc = "This register is assigned to domain0. The master from domain3 can write to this register"]
    DOMAIN0_1,
}
impl DOMAIN0R {
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
            DOMAIN0R::DOMAIN0_0 => false,
            DOMAIN0R::DOMAIN0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOMAIN0R {
        match value {
            false => DOMAIN0R::DOMAIN0_0,
            true => DOMAIN0R::DOMAIN0_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOMAIN0_0`"]
    #[inline]
    pub fn is_domain0_0(&self) -> bool {
        *self == DOMAIN0R::DOMAIN0_0
    }
    #[doc = "Checks if the value of the field is `DOMAIN0_1`"]
    #[inline]
    pub fn is_domain0_1(&self) -> bool {
        *self == DOMAIN0R::DOMAIN0_1
    }
}
#[doc = "Possible values of the field `DOMAIN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOMAIN1R {
    #[doc = "This register is not assigned to domain1. The master from domain1 cannot write to this register."]
    DOMAIN1_0,
    #[doc = "This register is assigned to domain1. The master from domain1 can write to this register"]
    DOMAIN1_1,
}
impl DOMAIN1R {
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
            DOMAIN1R::DOMAIN1_0 => false,
            DOMAIN1R::DOMAIN1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOMAIN1R {
        match value {
            false => DOMAIN1R::DOMAIN1_0,
            true => DOMAIN1R::DOMAIN1_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOMAIN1_0`"]
    #[inline]
    pub fn is_domain1_0(&self) -> bool {
        *self == DOMAIN1R::DOMAIN1_0
    }
    #[doc = "Checks if the value of the field is `DOMAIN1_1`"]
    #[inline]
    pub fn is_domain1_1(&self) -> bool {
        *self == DOMAIN1R::DOMAIN1_1
    }
}
#[doc = "Possible values of the field `DOMAIN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOMAIN2R {
    #[doc = "This register is not assigned to domain2. The master from domain2 cannot write to this register."]
    DOMAIN2_0,
    #[doc = "This register is assigned to domain2. The master from domain2 can write to this register"]
    DOMAIN2_1,
}
impl DOMAIN2R {
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
            DOMAIN2R::DOMAIN2_0 => false,
            DOMAIN2R::DOMAIN2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOMAIN2R {
        match value {
            false => DOMAIN2R::DOMAIN2_0,
            true => DOMAIN2R::DOMAIN2_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOMAIN2_0`"]
    #[inline]
    pub fn is_domain2_0(&self) -> bool {
        *self == DOMAIN2R::DOMAIN2_0
    }
    #[doc = "Checks if the value of the field is `DOMAIN2_1`"]
    #[inline]
    pub fn is_domain2_1(&self) -> bool {
        *self == DOMAIN2R::DOMAIN2_1
    }
}
#[doc = "Possible values of the field `DOMAIN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOMAIN3R {
    #[doc = "This register is not assigned to domain3. The master from domain3 cannot write to this register."]
    DOMAIN3_0,
    #[doc = "This register is assigned to domain3. The master from domain3 can write to this register"]
    DOMAIN3_1,
}
impl DOMAIN3R {
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
            DOMAIN3R::DOMAIN3_0 => false,
            DOMAIN3R::DOMAIN3_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOMAIN3R {
        match value {
            false => DOMAIN3R::DOMAIN3_0,
            true => DOMAIN3R::DOMAIN3_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOMAIN3_0`"]
    #[inline]
    pub fn is_domain3_0(&self) -> bool {
        *self == DOMAIN3R::DOMAIN3_0
    }
    #[doc = "Checks if the value of the field is `DOMAIN3_1`"]
    #[inline]
    pub fn is_domain3_1(&self) -> bool {
        *self == DOMAIN3R::DOMAIN3_1
    }
}
#[doc = "Possible values of the field `LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKR {
    #[doc = "31\\] and \\[27:24\\] bits can be modified"]
    LOCK_0,
    #[doc = "31\\] and \\[27:24\\] bits cannot be modified"]
    LOCK_1,
}
impl LOCKR {
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
            LOCKR::LOCK_0 => false,
            LOCKR::LOCK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCKR {
        match value {
            false => LOCKR::LOCK_0,
            true => LOCKR::LOCK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_0`"]
    #[inline]
    pub fn is_lock_0(&self) -> bool {
        *self == LOCKR::LOCK_0
    }
    #[doc = "Checks if the value of the field is `LOCK_1`"]
    #[inline]
    pub fn is_lock_1(&self) -> bool {
        *self == LOCKR::LOCK_1
    }
}
#[doc = "Possible values of the field `DOM_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOM_ENR {
    #[doc = "Disables domain control. All of this register's bits except \\[31:30\\] and \\[27:24\\] can be modified by any masters"]
    DOM_EN_0,
    #[doc = "Enables domain control. All of this register's bits except \\[31:30\\] and \\[27:24\\] can only be modified by the masters from the domains specified in \\[27:24\\] area."]
    DOM_EN_1,
}
impl DOM_ENR {
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
            DOM_ENR::DOM_EN_0 => false,
            DOM_ENR::DOM_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOM_ENR {
        match value {
            false => DOM_ENR::DOM_EN_0,
            true => DOM_ENR::DOM_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOM_EN_0`"]
    #[inline]
    pub fn is_dom_en_0(&self) -> bool {
        *self == DOM_ENR::DOM_EN_0
    }
    #[doc = "Checks if the value of the field is `DOM_EN_1`"]
    #[inline]
    pub fn is_dom_en_1(&self) -> bool {
        *self == DOM_ENR::DOM_EN_1
    }
}
#[doc = r" Proxy"]
pub struct _PCIE_PHY_POWER_ON_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _PCIE_PHY_POWER_ON_RESETW<'a> {
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
pub struct _PCIEPHY_BTNRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _PCIEPHY_BTNRSTW<'a> {
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
#[doc = r" Proxy"]
pub struct _PCIEPHY_PERSTW<'a> {
    w: &'a mut W,
}
impl<'a> _PCIEPHY_PERSTW<'a> {
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
pub struct _PCIE_CTRL_APPS_CLK_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _PCIE_CTRL_APPS_CLK_REQW<'a> {
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
pub struct _PCIE_CTRL_APPS_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _PCIE_CTRL_APPS_RSTW<'a> {
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
#[doc = r" Proxy"]
pub struct _PCIE_CTRL_APPS_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PCIE_CTRL_APPS_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _PCIE_CTRL_APPS_READYW<'a> {
    w: &'a mut W,
}
impl<'a> _PCIE_CTRL_APPS_READYW<'a> {
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
pub struct _PCIE_CTRL_APPS_ENTERW<'a> {
    w: &'a mut W,
}
impl<'a> _PCIE_CTRL_APPS_ENTERW<'a> {
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
pub struct _PCIE_CTRL_APPS_EXITW<'a> {
    w: &'a mut W,
}
impl<'a> _PCIE_CTRL_APPS_EXITW<'a> {
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
pub struct _PCIE_CTRL_APPS_PMEW<'a> {
    w: &'a mut W,
}
impl<'a> _PCIE_CTRL_APPS_PMEW<'a> {
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
pub struct _PCIE_CTRL_APPS_TURNOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _PCIE_CTRL_APPS_TURNOFFW<'a> {
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
pub struct _PCIE_CTRL_CFG_L1_AUXW<'a> {
    w: &'a mut W,
}
impl<'a> _PCIE_CTRL_CFG_L1_AUXW<'a> {
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
#[doc = r" Proxy"]
pub struct _PCIE_CTRL_SYS_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _PCIE_CTRL_SYS_INTW<'a> {
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
#[doc = r" Proxy"]
pub struct _PCIE_CTRL_APP_UNLOCK_MSGW<'a> {
    w: &'a mut W,
}
impl<'a> _PCIE_CTRL_APP_UNLOCK_MSGW<'a> {
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
#[doc = r" Proxy"]
pub struct _PCIE_CTRL_APP_XFER_PENDINGW<'a> {
    w: &'a mut W,
}
impl<'a> _PCIE_CTRL_APP_XFER_PENDINGW<'a> {
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
#[doc = "Values that can be written to the field `DOMAIN0`"]
pub enum DOMAIN0W {
    #[doc = "This register is not assigned to domain0. The master from domain3 cannot write to this register."]
    DOMAIN0_0,
    #[doc = "This register is assigned to domain0. The master from domain3 can write to this register"]
    DOMAIN0_1,
}
impl DOMAIN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOMAIN0W::DOMAIN0_0 => false,
            DOMAIN0W::DOMAIN0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOMAIN0W<'a> {
    w: &'a mut W,
}
impl<'a> _DOMAIN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOMAIN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This register is not assigned to domain0. The master from domain3 cannot write to this register."]
    #[inline]
    pub fn domain0_0(self) -> &'a mut W {
        self.variant(DOMAIN0W::DOMAIN0_0)
    }
    #[doc = "This register is assigned to domain0. The master from domain3 can write to this register"]
    #[inline]
    pub fn domain0_1(self) -> &'a mut W {
        self.variant(DOMAIN0W::DOMAIN0_1)
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
#[doc = "Values that can be written to the field `DOMAIN1`"]
pub enum DOMAIN1W {
    #[doc = "This register is not assigned to domain1. The master from domain1 cannot write to this register."]
    DOMAIN1_0,
    #[doc = "This register is assigned to domain1. The master from domain1 can write to this register"]
    DOMAIN1_1,
}
impl DOMAIN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOMAIN1W::DOMAIN1_0 => false,
            DOMAIN1W::DOMAIN1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOMAIN1W<'a> {
    w: &'a mut W,
}
impl<'a> _DOMAIN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOMAIN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This register is not assigned to domain1. The master from domain1 cannot write to this register."]
    #[inline]
    pub fn domain1_0(self) -> &'a mut W {
        self.variant(DOMAIN1W::DOMAIN1_0)
    }
    #[doc = "This register is assigned to domain1. The master from domain1 can write to this register"]
    #[inline]
    pub fn domain1_1(self) -> &'a mut W {
        self.variant(DOMAIN1W::DOMAIN1_1)
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
#[doc = "Values that can be written to the field `DOMAIN2`"]
pub enum DOMAIN2W {
    #[doc = "This register is not assigned to domain2. The master from domain2 cannot write to this register."]
    DOMAIN2_0,
    #[doc = "This register is assigned to domain2. The master from domain2 can write to this register"]
    DOMAIN2_1,
}
impl DOMAIN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOMAIN2W::DOMAIN2_0 => false,
            DOMAIN2W::DOMAIN2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOMAIN2W<'a> {
    w: &'a mut W,
}
impl<'a> _DOMAIN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOMAIN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This register is not assigned to domain2. The master from domain2 cannot write to this register."]
    #[inline]
    pub fn domain2_0(self) -> &'a mut W {
        self.variant(DOMAIN2W::DOMAIN2_0)
    }
    #[doc = "This register is assigned to domain2. The master from domain2 can write to this register"]
    #[inline]
    pub fn domain2_1(self) -> &'a mut W {
        self.variant(DOMAIN2W::DOMAIN2_1)
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
#[doc = "Values that can be written to the field `DOMAIN3`"]
pub enum DOMAIN3W {
    #[doc = "This register is not assigned to domain3. The master from domain3 cannot write to this register."]
    DOMAIN3_0,
    #[doc = "This register is assigned to domain3. The master from domain3 can write to this register"]
    DOMAIN3_1,
}
impl DOMAIN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOMAIN3W::DOMAIN3_0 => false,
            DOMAIN3W::DOMAIN3_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOMAIN3W<'a> {
    w: &'a mut W,
}
impl<'a> _DOMAIN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOMAIN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This register is not assigned to domain3. The master from domain3 cannot write to this register."]
    #[inline]
    pub fn domain3_0(self) -> &'a mut W {
        self.variant(DOMAIN3W::DOMAIN3_0)
    }
    #[doc = "This register is assigned to domain3. The master from domain3 can write to this register"]
    #[inline]
    pub fn domain3_1(self) -> &'a mut W {
        self.variant(DOMAIN3W::DOMAIN3_1)
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
#[doc = "Values that can be written to the field `LOCK`"]
pub enum LOCKW {
    #[doc = "31\\] and \\[27:24\\] bits can be modified"]
    LOCK_0,
    #[doc = "31\\] and \\[27:24\\] bits cannot be modified"]
    LOCK_1,
}
impl LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCKW::LOCK_0 => false,
            LOCKW::LOCK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "31\\] and \\[27:24\\] bits can be modified"]
    #[inline]
    pub fn lock_0(self) -> &'a mut W {
        self.variant(LOCKW::LOCK_0)
    }
    #[doc = "31\\] and \\[27:24\\] bits cannot be modified"]
    #[inline]
    pub fn lock_1(self) -> &'a mut W {
        self.variant(LOCKW::LOCK_1)
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
#[doc = "Values that can be written to the field `DOM_EN`"]
pub enum DOM_ENW {
    #[doc = "Disables domain control. All of this register's bits except \\[31:30\\] and \\[27:24\\] can be modified by any masters"]
    DOM_EN_0,
    #[doc = "Enables domain control. All of this register's bits except \\[31:30\\] and \\[27:24\\] can only be modified by the masters from the domains specified in \\[27:24\\] area."]
    DOM_EN_1,
}
impl DOM_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOM_ENW::DOM_EN_0 => false,
            DOM_ENW::DOM_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOM_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DOM_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOM_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables domain control. All of this register's bits except \\[31:30\\] and \\[27:24\\] can be modified by any masters"]
    #[inline]
    pub fn dom_en_0(self) -> &'a mut W {
        self.variant(DOM_ENW::DOM_EN_0)
    }
    #[doc = "Enables domain control. All of this register's bits except \\[31:30\\] and \\[27:24\\] can only be modified by the masters from the domains specified in \\[27:24\\] area."]
    #[inline]
    pub fn dom_en_1(self) -> &'a mut W {
        self.variant(DOM_ENW::DOM_EN_1)
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
    #[doc = "Bit 0 - PCIE_PHY_POWER_ON_RESET"]
    #[inline]
    pub fn pcie_phy_power_on_reset(&self) -> PCIE_PHY_POWER_ON_RESETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCIE_PHY_POWER_ON_RESETR { bits }
    }
    #[doc = "Bit 2 - PCIE PHY button"]
    #[inline]
    pub fn pciephy_btnrst(&self) -> PCIEPHY_BTNRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCIEPHY_BTNRSTR { bits }
    }
    #[doc = "Bit 3 - Pciephy_perst"]
    #[inline]
    pub fn pciephy_perst(&self) -> PCIEPHY_PERSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCIEPHY_PERSTR { bits }
    }
    #[doc = "Bit 4 - Pcie_ctrl_app_clk_req_n"]
    #[inline]
    pub fn pcie_ctrl_apps_clk_req(&self) -> PCIE_CTRL_APPS_CLK_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCIE_CTRL_APPS_CLK_REQR { bits }
    }
    #[doc = "Bit 5 - Pcie_ctrl_app_init_rst"]
    #[inline]
    pub fn pcie_ctrl_apps_rst(&self) -> PCIE_CTRL_APPS_RSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCIE_CTRL_APPS_RSTR { bits }
    }
    #[doc = "Bit 6 - Pcie_ctrl_app_ltssm_enable"]
    #[inline]
    pub fn pcie_ctrl_apps_en(&self) -> PCIE_CTRL_APPS_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCIE_CTRL_APPS_ENR { bits }
    }
    #[doc = "Bit 7 - Pcie_ctrl_app_ready_entr_l23"]
    #[inline]
    pub fn pcie_ctrl_apps_ready(&self) -> PCIE_CTRL_APPS_READYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCIE_CTRL_APPS_READYR { bits }
    }
    #[doc = "Bit 8 - Pcie_ctrl_app_req_entr_l1"]
    #[inline]
    pub fn pcie_ctrl_apps_enter(&self) -> PCIE_CTRL_APPS_ENTERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCIE_CTRL_APPS_ENTERR { bits }
    }
    #[doc = "Bit 9 - Pcie_ctrl_app_req_exit_l1"]
    #[inline]
    pub fn pcie_ctrl_apps_exit(&self) -> PCIE_CTRL_APPS_EXITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCIE_CTRL_APPS_EXITR { bits }
    }
    #[doc = "Bit 10 - Pcie_ctrl_apps_pm_xmt_pme"]
    #[inline]
    pub fn pcie_ctrl_apps_pme(&self) -> PCIE_CTRL_APPS_PMER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCIE_CTRL_APPS_PMER { bits }
    }
    #[doc = "Bit 11 - Pcie_ctrl_apps_pm_xmt_turnoff"]
    #[inline]
    pub fn pcie_ctrl_apps_turnoff(&self) -> PCIE_CTRL_APPS_TURNOFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCIE_CTRL_APPS_TURNOFFR { bits }
    }
    #[doc = "Bit 12 - Pcie_ctrl_cfg_l1_aux_clk_switch_core_clk_gate_en"]
    #[inline]
    pub fn pcie_ctrl_cfg_l1_aux(&self) -> PCIE_CTRL_CFG_L1_AUXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCIE_CTRL_CFG_L1_AUXR { bits }
    }
    #[doc = "Bit 14 - PCIE_CTRL_SYS_INT"]
    #[inline]
    pub fn pcie_ctrl_sys_int(&self) -> PCIE_CTRL_SYS_INTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCIE_CTRL_SYS_INTR { bits }
    }
    #[doc = "Bit 15 - PCIE_CTRL_APP_UNLOCK_MSG"]
    #[inline]
    pub fn pcie_ctrl_app_unlock_msg(&self) -> PCIE_CTRL_APP_UNLOCK_MSGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCIE_CTRL_APP_UNLOCK_MSGR { bits }
    }
    #[doc = "Bit 16 - PCIE_CTRL_APP_XFER_PENDING"]
    #[inline]
    pub fn pcie_ctrl_app_xfer_pending(&self) -> PCIE_CTRL_APP_XFER_PENDINGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCIE_CTRL_APP_XFER_PENDINGR { bits }
    }
    #[doc = "Bit 24 - Domain0 assignment control. Effective when dom_en is set to 1."]
    #[inline]
    pub fn domain0(&self) -> DOMAIN0R {
        DOMAIN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Domain1 assignment control. Effective when dom_en is set to 1."]
    #[inline]
    pub fn domain1(&self) -> DOMAIN1R {
        DOMAIN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Domain2 assignment control. Effective when dom_en is set to 1."]
    #[inline]
    pub fn domain2(&self) -> DOMAIN2R {
        DOMAIN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Domain3 assignment control. Effective when dom_en is set to 1."]
    #[inline]
    pub fn domain3(&self) -> DOMAIN3R {
        DOMAIN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Domain control bits lock Lock bit is a write-once register, once it is set to 1, it can't be write to 0"]
    #[inline]
    pub fn lock(&self) -> LOCKR {
        LOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Domain Control enable for this register"]
    #[inline]
    pub fn dom_en(&self) -> DOM_ENR {
        DOM_ENR::_from({
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
        W { bits: 10 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - PCIE_PHY_POWER_ON_RESET"]
    #[inline]
    pub fn pcie_phy_power_on_reset(&mut self) -> _PCIE_PHY_POWER_ON_RESETW {
        _PCIE_PHY_POWER_ON_RESETW { w: self }
    }
    #[doc = "Bit 2 - PCIE PHY button"]
    #[inline]
    pub fn pciephy_btnrst(&mut self) -> _PCIEPHY_BTNRSTW {
        _PCIEPHY_BTNRSTW { w: self }
    }
    #[doc = "Bit 3 - Pciephy_perst"]
    #[inline]
    pub fn pciephy_perst(&mut self) -> _PCIEPHY_PERSTW {
        _PCIEPHY_PERSTW { w: self }
    }
    #[doc = "Bit 4 - Pcie_ctrl_app_clk_req_n"]
    #[inline]
    pub fn pcie_ctrl_apps_clk_req(&mut self) -> _PCIE_CTRL_APPS_CLK_REQW {
        _PCIE_CTRL_APPS_CLK_REQW { w: self }
    }
    #[doc = "Bit 5 - Pcie_ctrl_app_init_rst"]
    #[inline]
    pub fn pcie_ctrl_apps_rst(&mut self) -> _PCIE_CTRL_APPS_RSTW {
        _PCIE_CTRL_APPS_RSTW { w: self }
    }
    #[doc = "Bit 6 - Pcie_ctrl_app_ltssm_enable"]
    #[inline]
    pub fn pcie_ctrl_apps_en(&mut self) -> _PCIE_CTRL_APPS_ENW {
        _PCIE_CTRL_APPS_ENW { w: self }
    }
    #[doc = "Bit 7 - Pcie_ctrl_app_ready_entr_l23"]
    #[inline]
    pub fn pcie_ctrl_apps_ready(&mut self) -> _PCIE_CTRL_APPS_READYW {
        _PCIE_CTRL_APPS_READYW { w: self }
    }
    #[doc = "Bit 8 - Pcie_ctrl_app_req_entr_l1"]
    #[inline]
    pub fn pcie_ctrl_apps_enter(&mut self) -> _PCIE_CTRL_APPS_ENTERW {
        _PCIE_CTRL_APPS_ENTERW { w: self }
    }
    #[doc = "Bit 9 - Pcie_ctrl_app_req_exit_l1"]
    #[inline]
    pub fn pcie_ctrl_apps_exit(&mut self) -> _PCIE_CTRL_APPS_EXITW {
        _PCIE_CTRL_APPS_EXITW { w: self }
    }
    #[doc = "Bit 10 - Pcie_ctrl_apps_pm_xmt_pme"]
    #[inline]
    pub fn pcie_ctrl_apps_pme(&mut self) -> _PCIE_CTRL_APPS_PMEW {
        _PCIE_CTRL_APPS_PMEW { w: self }
    }
    #[doc = "Bit 11 - Pcie_ctrl_apps_pm_xmt_turnoff"]
    #[inline]
    pub fn pcie_ctrl_apps_turnoff(&mut self) -> _PCIE_CTRL_APPS_TURNOFFW {
        _PCIE_CTRL_APPS_TURNOFFW { w: self }
    }
    #[doc = "Bit 12 - Pcie_ctrl_cfg_l1_aux_clk_switch_core_clk_gate_en"]
    #[inline]
    pub fn pcie_ctrl_cfg_l1_aux(&mut self) -> _PCIE_CTRL_CFG_L1_AUXW {
        _PCIE_CTRL_CFG_L1_AUXW { w: self }
    }
    #[doc = "Bit 14 - PCIE_CTRL_SYS_INT"]
    #[inline]
    pub fn pcie_ctrl_sys_int(&mut self) -> _PCIE_CTRL_SYS_INTW {
        _PCIE_CTRL_SYS_INTW { w: self }
    }
    #[doc = "Bit 15 - PCIE_CTRL_APP_UNLOCK_MSG"]
    #[inline]
    pub fn pcie_ctrl_app_unlock_msg(&mut self) -> _PCIE_CTRL_APP_UNLOCK_MSGW {
        _PCIE_CTRL_APP_UNLOCK_MSGW { w: self }
    }
    #[doc = "Bit 16 - PCIE_CTRL_APP_XFER_PENDING"]
    #[inline]
    pub fn pcie_ctrl_app_xfer_pending(&mut self) -> _PCIE_CTRL_APP_XFER_PENDINGW {
        _PCIE_CTRL_APP_XFER_PENDINGW { w: self }
    }
    #[doc = "Bit 24 - Domain0 assignment control. Effective when dom_en is set to 1."]
    #[inline]
    pub fn domain0(&mut self) -> _DOMAIN0W {
        _DOMAIN0W { w: self }
    }
    #[doc = "Bit 25 - Domain1 assignment control. Effective when dom_en is set to 1."]
    #[inline]
    pub fn domain1(&mut self) -> _DOMAIN1W {
        _DOMAIN1W { w: self }
    }
    #[doc = "Bit 26 - Domain2 assignment control. Effective when dom_en is set to 1."]
    #[inline]
    pub fn domain2(&mut self) -> _DOMAIN2W {
        _DOMAIN2W { w: self }
    }
    #[doc = "Bit 27 - Domain3 assignment control. Effective when dom_en is set to 1."]
    #[inline]
    pub fn domain3(&mut self) -> _DOMAIN3W {
        _DOMAIN3W { w: self }
    }
    #[doc = "Bit 30 - Domain control bits lock Lock bit is a write-once register, once it is set to 1, it can't be write to 0"]
    #[inline]
    pub fn lock(&mut self) -> _LOCKW {
        _LOCKW { w: self }
    }
    #[doc = "Bit 31 - Domain Control enable for this register"]
    #[inline]
    pub fn dom_en(&mut self) -> _DOM_ENW {
        _DOM_ENW { w: self }
    }
}
