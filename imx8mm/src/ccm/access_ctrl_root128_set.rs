#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ACCESS_CTRL_ROOT128_SET {
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
pub struct DOMAIN0_INFOR {
    bits: u8,
}
impl DOMAIN0_INFOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DOMAIN1_INFOR {
    bits: u8,
}
impl DOMAIN1_INFOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DOMAIN2_INFOR {
    bits: u8,
}
impl DOMAIN2_INFOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DOMAIN3_INFOR {
    bits: u8,
}
impl DOMAIN3_INFOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `OWNER_ID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OWNER_IDR {
    #[doc = "domaino"]
    OWNER_ID_0,
    #[doc = "domain1"]
    OWNER_ID_1,
    #[doc = "domain2"]
    OWNER_ID_2,
    #[doc = "domain3"]
    OWNER_ID_3,
}
impl OWNER_IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OWNER_IDR::OWNER_ID_0 => 0,
            OWNER_IDR::OWNER_ID_1 => 1,
            OWNER_IDR::OWNER_ID_2 => 2,
            OWNER_IDR::OWNER_ID_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OWNER_IDR {
        match value {
            0 => OWNER_IDR::OWNER_ID_0,
            1 => OWNER_IDR::OWNER_ID_1,
            2 => OWNER_IDR::OWNER_ID_2,
            3 => OWNER_IDR::OWNER_ID_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OWNER_ID_0`"]
    #[inline]
    pub fn is_owner_id_0(&self) -> bool {
        *self == OWNER_IDR::OWNER_ID_0
    }
    #[doc = "Checks if the value of the field is `OWNER_ID_1`"]
    #[inline]
    pub fn is_owner_id_1(&self) -> bool {
        *self == OWNER_IDR::OWNER_ID_1
    }
    #[doc = "Checks if the value of the field is `OWNER_ID_2`"]
    #[inline]
    pub fn is_owner_id_2(&self) -> bool {
        *self == OWNER_IDR::OWNER_ID_2
    }
    #[doc = "Checks if the value of the field is `OWNER_ID_3`"]
    #[inline]
    pub fn is_owner_id_3(&self) -> bool {
        *self == OWNER_IDR::OWNER_ID_3
    }
}
#[doc = "Possible values of the field `MUTEX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUTEXR {
    #[doc = "Semaphore is free to take"]
    MUTEX_0,
    #[doc = "Semaphore is taken"]
    MUTEX_1,
}
impl MUTEXR {
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
            MUTEXR::MUTEX_0 => false,
            MUTEXR::MUTEX_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MUTEXR {
        match value {
            false => MUTEXR::MUTEX_0,
            true => MUTEXR::MUTEX_1,
        }
    }
    #[doc = "Checks if the value of the field is `MUTEX_0`"]
    #[inline]
    pub fn is_mutex_0(&self) -> bool {
        *self == MUTEXR::MUTEX_0
    }
    #[doc = "Checks if the value of the field is `MUTEX_1`"]
    #[inline]
    pub fn is_mutex_1(&self) -> bool {
        *self == MUTEXR::MUTEX_1
    }
}
#[doc = "Possible values of the field `DOMAIN0_WHITELIST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOMAIN0_WHITELISTR {
    #[doc = "Domain cannot change the setting"]
    DOMAIN0_WHITELIST_0,
    #[doc = "Domain can change the setting"]
    DOMAIN0_WHITELIST_1,
}
impl DOMAIN0_WHITELISTR {
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
            DOMAIN0_WHITELISTR::DOMAIN0_WHITELIST_0 => false,
            DOMAIN0_WHITELISTR::DOMAIN0_WHITELIST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOMAIN0_WHITELISTR {
        match value {
            false => DOMAIN0_WHITELISTR::DOMAIN0_WHITELIST_0,
            true => DOMAIN0_WHITELISTR::DOMAIN0_WHITELIST_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOMAIN0_WHITELIST_0`"]
    #[inline]
    pub fn is_domain0_whitelist_0(&self) -> bool {
        *self == DOMAIN0_WHITELISTR::DOMAIN0_WHITELIST_0
    }
    #[doc = "Checks if the value of the field is `DOMAIN0_WHITELIST_1`"]
    #[inline]
    pub fn is_domain0_whitelist_1(&self) -> bool {
        *self == DOMAIN0_WHITELISTR::DOMAIN0_WHITELIST_1
    }
}
#[doc = "Possible values of the field `DOMAIN1_WHITELIST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOMAIN1_WHITELISTR {
    #[doc = "Domain cannot change the setting"]
    DOMAIN1_WHITELIST_0,
    #[doc = "Domain can change the setting"]
    DOMAIN1_WHITELIST_1,
}
impl DOMAIN1_WHITELISTR {
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
            DOMAIN1_WHITELISTR::DOMAIN1_WHITELIST_0 => false,
            DOMAIN1_WHITELISTR::DOMAIN1_WHITELIST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOMAIN1_WHITELISTR {
        match value {
            false => DOMAIN1_WHITELISTR::DOMAIN1_WHITELIST_0,
            true => DOMAIN1_WHITELISTR::DOMAIN1_WHITELIST_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOMAIN1_WHITELIST_0`"]
    #[inline]
    pub fn is_domain1_whitelist_0(&self) -> bool {
        *self == DOMAIN1_WHITELISTR::DOMAIN1_WHITELIST_0
    }
    #[doc = "Checks if the value of the field is `DOMAIN1_WHITELIST_1`"]
    #[inline]
    pub fn is_domain1_whitelist_1(&self) -> bool {
        *self == DOMAIN1_WHITELISTR::DOMAIN1_WHITELIST_1
    }
}
#[doc = "Possible values of the field `DOMAIN2_WHITELIST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOMAIN2_WHITELISTR {
    #[doc = "Domain cannot change the setting"]
    DOMAIN2_WHITELIST_0,
    #[doc = "Domain can change the setting"]
    DOMAIN2_WHITELIST_1,
}
impl DOMAIN2_WHITELISTR {
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
            DOMAIN2_WHITELISTR::DOMAIN2_WHITELIST_0 => false,
            DOMAIN2_WHITELISTR::DOMAIN2_WHITELIST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOMAIN2_WHITELISTR {
        match value {
            false => DOMAIN2_WHITELISTR::DOMAIN2_WHITELIST_0,
            true => DOMAIN2_WHITELISTR::DOMAIN2_WHITELIST_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOMAIN2_WHITELIST_0`"]
    #[inline]
    pub fn is_domain2_whitelist_0(&self) -> bool {
        *self == DOMAIN2_WHITELISTR::DOMAIN2_WHITELIST_0
    }
    #[doc = "Checks if the value of the field is `DOMAIN2_WHITELIST_1`"]
    #[inline]
    pub fn is_domain2_whitelist_1(&self) -> bool {
        *self == DOMAIN2_WHITELISTR::DOMAIN2_WHITELIST_1
    }
}
#[doc = "Possible values of the field `DOMAIN3_WHITELIST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOMAIN3_WHITELISTR {
    #[doc = "Domain cannot change the setting"]
    DOMAIN3_WHITELIST_0,
    #[doc = "Domain can change the setting"]
    DOMAIN3_WHITELIST_1,
}
impl DOMAIN3_WHITELISTR {
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
            DOMAIN3_WHITELISTR::DOMAIN3_WHITELIST_0 => false,
            DOMAIN3_WHITELISTR::DOMAIN3_WHITELIST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOMAIN3_WHITELISTR {
        match value {
            false => DOMAIN3_WHITELISTR::DOMAIN3_WHITELIST_0,
            true => DOMAIN3_WHITELISTR::DOMAIN3_WHITELIST_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOMAIN3_WHITELIST_0`"]
    #[inline]
    pub fn is_domain3_whitelist_0(&self) -> bool {
        *self == DOMAIN3_WHITELISTR::DOMAIN3_WHITELIST_0
    }
    #[doc = "Checks if the value of the field is `DOMAIN3_WHITELIST_1`"]
    #[inline]
    pub fn is_domain3_whitelist_1(&self) -> bool {
        *self == DOMAIN3_WHITELISTR::DOMAIN3_WHITELIST_1
    }
}
#[doc = "Possible values of the field `SEMA_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEMA_ENR {
    #[doc = "Disable"]
    SEMA_EN_0,
    #[doc = "Enable"]
    SEMA_EN_1,
}
impl SEMA_ENR {
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
            SEMA_ENR::SEMA_EN_0 => false,
            SEMA_ENR::SEMA_EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEMA_ENR {
        match value {
            false => SEMA_ENR::SEMA_EN_0,
            true => SEMA_ENR::SEMA_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEMA_EN_0`"]
    #[inline]
    pub fn is_sema_en_0(&self) -> bool {
        *self == SEMA_ENR::SEMA_EN_0
    }
    #[doc = "Checks if the value of the field is `SEMA_EN_1`"]
    #[inline]
    pub fn is_sema_en_1(&self) -> bool {
        *self == SEMA_ENR::SEMA_EN_1
    }
}
#[doc = "Possible values of the field `LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKR {
    #[doc = "Access control inactive"]
    LOCK_0,
    #[doc = "Access control active"]
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
#[doc = r" Proxy"]
pub struct _DOMAIN0_INFOW<'a> {
    w: &'a mut W,
}
impl<'a> _DOMAIN0_INFOW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DOMAIN1_INFOW<'a> {
    w: &'a mut W,
}
impl<'a> _DOMAIN1_INFOW<'a> {
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
#[doc = r" Proxy"]
pub struct _DOMAIN2_INFOW<'a> {
    w: &'a mut W,
}
impl<'a> _DOMAIN2_INFOW<'a> {
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
#[doc = r" Proxy"]
pub struct _DOMAIN3_INFOW<'a> {
    w: &'a mut W,
}
impl<'a> _DOMAIN3_INFOW<'a> {
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
#[doc = "Values that can be written to the field `MUTEX`"]
pub enum MUTEXW {
    #[doc = "Semaphore is free to take"]
    MUTEX_0,
    #[doc = "Semaphore is taken"]
    MUTEX_1,
}
impl MUTEXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MUTEXW::MUTEX_0 => false,
            MUTEXW::MUTEX_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MUTEXW<'a> {
    w: &'a mut W,
}
impl<'a> _MUTEXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MUTEXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Semaphore is free to take"]
    #[inline]
    pub fn mutex_0(self) -> &'a mut W {
        self.variant(MUTEXW::MUTEX_0)
    }
    #[doc = "Semaphore is taken"]
    #[inline]
    pub fn mutex_1(self) -> &'a mut W {
        self.variant(MUTEXW::MUTEX_1)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DOMAIN0_WHITELIST`"]
pub enum DOMAIN0_WHITELISTW {
    #[doc = "Domain cannot change the setting"]
    DOMAIN0_WHITELIST_0,
    #[doc = "Domain can change the setting"]
    DOMAIN0_WHITELIST_1,
}
impl DOMAIN0_WHITELISTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOMAIN0_WHITELISTW::DOMAIN0_WHITELIST_0 => false,
            DOMAIN0_WHITELISTW::DOMAIN0_WHITELIST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOMAIN0_WHITELISTW<'a> {
    w: &'a mut W,
}
impl<'a> _DOMAIN0_WHITELISTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOMAIN0_WHITELISTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Domain cannot change the setting"]
    #[inline]
    pub fn domain0_whitelist_0(self) -> &'a mut W {
        self.variant(DOMAIN0_WHITELISTW::DOMAIN0_WHITELIST_0)
    }
    #[doc = "Domain can change the setting"]
    #[inline]
    pub fn domain0_whitelist_1(self) -> &'a mut W {
        self.variant(DOMAIN0_WHITELISTW::DOMAIN0_WHITELIST_1)
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
#[doc = "Values that can be written to the field `DOMAIN1_WHITELIST`"]
pub enum DOMAIN1_WHITELISTW {
    #[doc = "Domain cannot change the setting"]
    DOMAIN1_WHITELIST_0,
    #[doc = "Domain can change the setting"]
    DOMAIN1_WHITELIST_1,
}
impl DOMAIN1_WHITELISTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOMAIN1_WHITELISTW::DOMAIN1_WHITELIST_0 => false,
            DOMAIN1_WHITELISTW::DOMAIN1_WHITELIST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOMAIN1_WHITELISTW<'a> {
    w: &'a mut W,
}
impl<'a> _DOMAIN1_WHITELISTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOMAIN1_WHITELISTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Domain cannot change the setting"]
    #[inline]
    pub fn domain1_whitelist_0(self) -> &'a mut W {
        self.variant(DOMAIN1_WHITELISTW::DOMAIN1_WHITELIST_0)
    }
    #[doc = "Domain can change the setting"]
    #[inline]
    pub fn domain1_whitelist_1(self) -> &'a mut W {
        self.variant(DOMAIN1_WHITELISTW::DOMAIN1_WHITELIST_1)
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
#[doc = "Values that can be written to the field `DOMAIN2_WHITELIST`"]
pub enum DOMAIN2_WHITELISTW {
    #[doc = "Domain cannot change the setting"]
    DOMAIN2_WHITELIST_0,
    #[doc = "Domain can change the setting"]
    DOMAIN2_WHITELIST_1,
}
impl DOMAIN2_WHITELISTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOMAIN2_WHITELISTW::DOMAIN2_WHITELIST_0 => false,
            DOMAIN2_WHITELISTW::DOMAIN2_WHITELIST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOMAIN2_WHITELISTW<'a> {
    w: &'a mut W,
}
impl<'a> _DOMAIN2_WHITELISTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOMAIN2_WHITELISTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Domain cannot change the setting"]
    #[inline]
    pub fn domain2_whitelist_0(self) -> &'a mut W {
        self.variant(DOMAIN2_WHITELISTW::DOMAIN2_WHITELIST_0)
    }
    #[doc = "Domain can change the setting"]
    #[inline]
    pub fn domain2_whitelist_1(self) -> &'a mut W {
        self.variant(DOMAIN2_WHITELISTW::DOMAIN2_WHITELIST_1)
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
#[doc = "Values that can be written to the field `DOMAIN3_WHITELIST`"]
pub enum DOMAIN3_WHITELISTW {
    #[doc = "Domain cannot change the setting"]
    DOMAIN3_WHITELIST_0,
    #[doc = "Domain can change the setting"]
    DOMAIN3_WHITELIST_1,
}
impl DOMAIN3_WHITELISTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOMAIN3_WHITELISTW::DOMAIN3_WHITELIST_0 => false,
            DOMAIN3_WHITELISTW::DOMAIN3_WHITELIST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOMAIN3_WHITELISTW<'a> {
    w: &'a mut W,
}
impl<'a> _DOMAIN3_WHITELISTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOMAIN3_WHITELISTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Domain cannot change the setting"]
    #[inline]
    pub fn domain3_whitelist_0(self) -> &'a mut W {
        self.variant(DOMAIN3_WHITELISTW::DOMAIN3_WHITELIST_0)
    }
    #[doc = "Domain can change the setting"]
    #[inline]
    pub fn domain3_whitelist_1(self) -> &'a mut W {
        self.variant(DOMAIN3_WHITELISTW::DOMAIN3_WHITELIST_1)
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
#[doc = "Values that can be written to the field `SEMA_EN`"]
pub enum SEMA_ENW {
    #[doc = "Disable"]
    SEMA_EN_0,
    #[doc = "Enable"]
    SEMA_EN_1,
}
impl SEMA_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEMA_ENW::SEMA_EN_0 => false,
            SEMA_ENW::SEMA_EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEMA_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SEMA_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEMA_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn sema_en_0(self) -> &'a mut W {
        self.variant(SEMA_ENW::SEMA_EN_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn sema_en_1(self) -> &'a mut W {
        self.variant(SEMA_ENW::SEMA_EN_1)
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
#[doc = "Values that can be written to the field `LOCK`"]
pub enum LOCKW {
    #[doc = "Access control inactive"]
    LOCK_0,
    #[doc = "Access control active"]
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
    #[doc = "Access control inactive"]
    #[inline]
    pub fn lock_0(self) -> &'a mut W {
        self.variant(LOCKW::LOCK_0)
    }
    #[doc = "Access control active"]
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
    #[doc = "Bits 0:3 - Information from domain 0 to pass to others This field can only be changed by domain 0"]
    #[inline]
    pub fn domain0_info(&self) -> DOMAIN0_INFOR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DOMAIN0_INFOR { bits }
    }
    #[doc = "Bits 4:7 - Information from domain 1 to pass to others This field can only be changed by domain 1"]
    #[inline]
    pub fn domain1_info(&self) -> DOMAIN1_INFOR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DOMAIN1_INFOR { bits }
    }
    #[doc = "Bits 8:11 - Information from domain 2 to pass to others This field can only be changed by domain 2"]
    #[inline]
    pub fn domain2_info(&self) -> DOMAIN2_INFOR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DOMAIN2_INFOR { bits }
    }
    #[doc = "Bits 12:15 - Information from domain 3 to pass to others This field can only be changed by domain 3"]
    #[inline]
    pub fn domain3_info(&self) -> DOMAIN3_INFOR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DOMAIN3_INFOR { bits }
    }
    #[doc = "Bits 16:17 - Current domain that owns semaphore This field is meaningless when MUTEX is 0"]
    #[inline]
    pub fn owner_id(&self) -> OWNER_IDR {
        OWNER_IDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Semaphore to control access"]
    #[inline]
    pub fn mutex(&self) -> MUTEXR {
        MUTEXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - White list of domains that can change setting of this clock root"]
    #[inline]
    pub fn domain0_whitelist(&self) -> DOMAIN0_WHITELISTR {
        DOMAIN0_WHITELISTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - White list of domains that can change setting of this clock root"]
    #[inline]
    pub fn domain1_whitelist(&self) -> DOMAIN1_WHITELISTR {
        DOMAIN1_WHITELISTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - White list of domains that can change setting of this clock root"]
    #[inline]
    pub fn domain2_whitelist(&self) -> DOMAIN2_WHITELISTR {
        DOMAIN2_WHITELISTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - White list of domains that can change setting of this clock root"]
    #[inline]
    pub fn domain3_whitelist(&self) -> DOMAIN3_WHITELISTR {
        DOMAIN3_WHITELISTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Enable internal semaphore This field cannot be changed when lock bit is 1"]
    #[inline]
    pub fn sema_en(&self) -> SEMA_ENR {
        SEMA_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Lock this clock root to use access control This bit can be set to 1 by software, and can be cleared only by system reset"]
    #[inline]
    pub fn lock(&self) -> LOCKR {
        LOCKR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Information from domain 0 to pass to others This field can only be changed by domain 0"]
    #[inline]
    pub fn domain0_info(&mut self) -> _DOMAIN0_INFOW {
        _DOMAIN0_INFOW { w: self }
    }
    #[doc = "Bits 4:7 - Information from domain 1 to pass to others This field can only be changed by domain 1"]
    #[inline]
    pub fn domain1_info(&mut self) -> _DOMAIN1_INFOW {
        _DOMAIN1_INFOW { w: self }
    }
    #[doc = "Bits 8:11 - Information from domain 2 to pass to others This field can only be changed by domain 2"]
    #[inline]
    pub fn domain2_info(&mut self) -> _DOMAIN2_INFOW {
        _DOMAIN2_INFOW { w: self }
    }
    #[doc = "Bits 12:15 - Information from domain 3 to pass to others This field can only be changed by domain 3"]
    #[inline]
    pub fn domain3_info(&mut self) -> _DOMAIN3_INFOW {
        _DOMAIN3_INFOW { w: self }
    }
    #[doc = "Bit 20 - Semaphore to control access"]
    #[inline]
    pub fn mutex(&mut self) -> _MUTEXW {
        _MUTEXW { w: self }
    }
    #[doc = "Bit 24 - White list of domains that can change setting of this clock root"]
    #[inline]
    pub fn domain0_whitelist(&mut self) -> _DOMAIN0_WHITELISTW {
        _DOMAIN0_WHITELISTW { w: self }
    }
    #[doc = "Bit 25 - White list of domains that can change setting of this clock root"]
    #[inline]
    pub fn domain1_whitelist(&mut self) -> _DOMAIN1_WHITELISTW {
        _DOMAIN1_WHITELISTW { w: self }
    }
    #[doc = "Bit 26 - White list of domains that can change setting of this clock root"]
    #[inline]
    pub fn domain2_whitelist(&mut self) -> _DOMAIN2_WHITELISTW {
        _DOMAIN2_WHITELISTW { w: self }
    }
    #[doc = "Bit 27 - White list of domains that can change setting of this clock root"]
    #[inline]
    pub fn domain3_whitelist(&mut self) -> _DOMAIN3_WHITELISTW {
        _DOMAIN3_WHITELISTW { w: self }
    }
    #[doc = "Bit 28 - Enable internal semaphore This field cannot be changed when lock bit is 1"]
    #[inline]
    pub fn sema_en(&mut self) -> _SEMA_ENW {
        _SEMA_ENW { w: self }
    }
    #[doc = "Bit 31 - Lock this clock root to use access control This bit can be set to 1 by software, and can be cleared only by system reset"]
    #[inline]
    pub fn lock(&mut self) -> _LOCKW {
        _LOCKW { w: self }
    }
}
