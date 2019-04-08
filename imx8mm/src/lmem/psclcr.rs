#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PSCLCR {
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
#[doc = "Possible values of the field `LGO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LGOR {
    #[doc = "Write: no effect. Read: no line command active."]
    LGO_0,
    #[doc = "Write: initiate line command indicated by bits 27-24. Read: line command active."]
    LGO_1,
}
impl LGOR {
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
            LGOR::LGO_0 => false,
            LGOR::LGO_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LGOR {
        match value {
            false => LGOR::LGO_0,
            true => LGOR::LGO_1,
        }
    }
    #[doc = "Checks if the value of the field is `LGO_0`"]
    #[inline]
    pub fn is_lgo_0(&self) -> bool {
        *self == LGOR::LGO_0
    }
    #[doc = "Checks if the value of the field is `LGO_1`"]
    #[inline]
    pub fn is_lgo_1(&self) -> bool {
        *self == LGOR::LGO_1
    }
}
#[doc = r" Value of the field"]
pub struct CACHEADDRR {
    bits: u16,
}
impl CACHEADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `WSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WSELR {
    #[doc = "Way 0"]
    WSEL_0,
    #[doc = "Way 1"]
    WSEL_1,
}
impl WSELR {
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
            WSELR::WSEL_0 => false,
            WSELR::WSEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WSELR {
        match value {
            false => WSELR::WSEL_0,
            true => WSELR::WSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `WSEL_0`"]
    #[inline]
    pub fn is_wsel_0(&self) -> bool {
        *self == WSELR::WSEL_0
    }
    #[doc = "Checks if the value of the field is `WSEL_1`"]
    #[inline]
    pub fn is_wsel_1(&self) -> bool {
        *self == WSELR::WSEL_1
    }
}
#[doc = "Possible values of the field `TDSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDSELR {
    #[doc = "Data"]
    TDSEL_0,
    #[doc = "Tag"]
    TDSEL_1,
}
impl TDSELR {
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
            TDSELR::TDSEL_0 => false,
            TDSELR::TDSEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDSELR {
        match value {
            false => TDSELR::TDSEL_0,
            true => TDSELR::TDSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `TDSEL_0`"]
    #[inline]
    pub fn is_tdsel_0(&self) -> bool {
        *self == TDSELR::TDSEL_0
    }
    #[doc = "Checks if the value of the field is `TDSEL_1`"]
    #[inline]
    pub fn is_tdsel_1(&self) -> bool {
        *self == TDSELR::TDSEL_1
    }
}
#[doc = r" Value of the field"]
pub struct LCIVBR {
    bits: bool,
}
impl LCIVBR {
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
pub struct LCIMBR {
    bits: bool,
}
impl LCIMBR {
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
pub struct LCWAYR {
    bits: bool,
}
impl LCWAYR {
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
#[doc = "Possible values of the field `LCMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCMDR {
    #[doc = "Search and read or write"]
    LCMD_0,
    #[doc = "Invalidate"]
    LCMD_1,
    #[doc = "Push"]
    LCMD_2,
    #[doc = "Clear"]
    LCMD_3,
}
impl LCMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LCMDR::LCMD_0 => 0,
            LCMDR::LCMD_1 => 1,
            LCMDR::LCMD_2 => 2,
            LCMDR::LCMD_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LCMDR {
        match value {
            0 => LCMDR::LCMD_0,
            1 => LCMDR::LCMD_1,
            2 => LCMDR::LCMD_2,
            3 => LCMDR::LCMD_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LCMD_0`"]
    #[inline]
    pub fn is_lcmd_0(&self) -> bool {
        *self == LCMDR::LCMD_0
    }
    #[doc = "Checks if the value of the field is `LCMD_1`"]
    #[inline]
    pub fn is_lcmd_1(&self) -> bool {
        *self == LCMDR::LCMD_1
    }
    #[doc = "Checks if the value of the field is `LCMD_2`"]
    #[inline]
    pub fn is_lcmd_2(&self) -> bool {
        *self == LCMDR::LCMD_2
    }
    #[doc = "Checks if the value of the field is `LCMD_3`"]
    #[inline]
    pub fn is_lcmd_3(&self) -> bool {
        *self == LCMDR::LCMD_3
    }
}
#[doc = "Possible values of the field `LADSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LADSELR {
    #[doc = "Cache address"]
    LADSEL_0,
    #[doc = "Physical address"]
    LADSEL_1,
}
impl LADSELR {
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
            LADSELR::LADSEL_0 => false,
            LADSELR::LADSEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LADSELR {
        match value {
            false => LADSELR::LADSEL_0,
            true => LADSELR::LADSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `LADSEL_0`"]
    #[inline]
    pub fn is_ladsel_0(&self) -> bool {
        *self == LADSELR::LADSEL_0
    }
    #[doc = "Checks if the value of the field is `LADSEL_1`"]
    #[inline]
    pub fn is_ladsel_1(&self) -> bool {
        *self == LADSELR::LADSEL_1
    }
}
#[doc = "Possible values of the field `LACC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LACCR {
    #[doc = "Read"]
    LACC_0,
    #[doc = "Write"]
    LACC_1,
}
impl LACCR {
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
            LACCR::LACC_0 => false,
            LACCR::LACC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LACCR {
        match value {
            false => LACCR::LACC_0,
            true => LACCR::LACC_1,
        }
    }
    #[doc = "Checks if the value of the field is `LACC_0`"]
    #[inline]
    pub fn is_lacc_0(&self) -> bool {
        *self == LACCR::LACC_0
    }
    #[doc = "Checks if the value of the field is `LACC_1`"]
    #[inline]
    pub fn is_lacc_1(&self) -> bool {
        *self == LACCR::LACC_1
    }
}
#[doc = "Values that can be written to the field `LGO`"]
pub enum LGOW {
    #[doc = "Write: no effect. Read: no line command active."]
    LGO_0,
    #[doc = "Write: initiate line command indicated by bits 27-24. Read: line command active."]
    LGO_1,
}
impl LGOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LGOW::LGO_0 => false,
            LGOW::LGO_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LGOW<'a> {
    w: &'a mut W,
}
impl<'a> _LGOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LGOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: no effect. Read: no line command active."]
    #[inline]
    pub fn lgo_0(self) -> &'a mut W {
        self.variant(LGOW::LGO_0)
    }
    #[doc = "Write: initiate line command indicated by bits 27-24. Read: line command active."]
    #[inline]
    pub fn lgo_1(self) -> &'a mut W {
        self.variant(LGOW::LGO_1)
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
pub struct _CACHEADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _CACHEADDRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 2047;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WSEL`"]
pub enum WSELW {
    #[doc = "Way 0"]
    WSEL_0,
    #[doc = "Way 1"]
    WSEL_1,
}
impl WSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WSELW::WSEL_0 => false,
            WSELW::WSEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WSELW<'a> {
    w: &'a mut W,
}
impl<'a> _WSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Way 0"]
    #[inline]
    pub fn wsel_0(self) -> &'a mut W {
        self.variant(WSELW::WSEL_0)
    }
    #[doc = "Way 1"]
    #[inline]
    pub fn wsel_1(self) -> &'a mut W {
        self.variant(WSELW::WSEL_1)
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
#[doc = "Values that can be written to the field `TDSEL`"]
pub enum TDSELW {
    #[doc = "Data"]
    TDSEL_0,
    #[doc = "Tag"]
    TDSEL_1,
}
impl TDSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TDSELW::TDSEL_0 => false,
            TDSELW::TDSEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TDSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data"]
    #[inline]
    pub fn tdsel_0(self) -> &'a mut W {
        self.variant(TDSELW::TDSEL_0)
    }
    #[doc = "Tag"]
    #[inline]
    pub fn tdsel_1(self) -> &'a mut W {
        self.variant(TDSELW::TDSEL_1)
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
#[doc = "Values that can be written to the field `LCMD`"]
pub enum LCMDW {
    #[doc = "Search and read or write"]
    LCMD_0,
    #[doc = "Invalidate"]
    LCMD_1,
    #[doc = "Push"]
    LCMD_2,
    #[doc = "Clear"]
    LCMD_3,
}
impl LCMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LCMDW::LCMD_0 => 0,
            LCMDW::LCMD_1 => 1,
            LCMDW::LCMD_2 => 2,
            LCMDW::LCMD_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LCMDW<'a> {
    w: &'a mut W,
}
impl<'a> _LCMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LCMDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Search and read or write"]
    #[inline]
    pub fn lcmd_0(self) -> &'a mut W {
        self.variant(LCMDW::LCMD_0)
    }
    #[doc = "Invalidate"]
    #[inline]
    pub fn lcmd_1(self) -> &'a mut W {
        self.variant(LCMDW::LCMD_1)
    }
    #[doc = "Push"]
    #[inline]
    pub fn lcmd_2(self) -> &'a mut W {
        self.variant(LCMDW::LCMD_2)
    }
    #[doc = "Clear"]
    #[inline]
    pub fn lcmd_3(self) -> &'a mut W {
        self.variant(LCMDW::LCMD_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LADSEL`"]
pub enum LADSELW {
    #[doc = "Cache address"]
    LADSEL_0,
    #[doc = "Physical address"]
    LADSEL_1,
}
impl LADSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LADSELW::LADSEL_0 => false,
            LADSELW::LADSEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LADSELW<'a> {
    w: &'a mut W,
}
impl<'a> _LADSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LADSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Cache address"]
    #[inline]
    pub fn ladsel_0(self) -> &'a mut W {
        self.variant(LADSELW::LADSEL_0)
    }
    #[doc = "Physical address"]
    #[inline]
    pub fn ladsel_1(self) -> &'a mut W {
        self.variant(LADSELW::LADSEL_1)
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
#[doc = "Values that can be written to the field `LACC`"]
pub enum LACCW {
    #[doc = "Read"]
    LACC_0,
    #[doc = "Write"]
    LACC_1,
}
impl LACCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LACCW::LACC_0 => false,
            LACCW::LACC_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LACCW<'a> {
    w: &'a mut W,
}
impl<'a> _LACCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LACCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read"]
    #[inline]
    pub fn lacc_0(self) -> &'a mut W {
        self.variant(LACCW::LACC_0)
    }
    #[doc = "Write"]
    #[inline]
    pub fn lacc_1(self) -> &'a mut W {
        self.variant(LACCW::LACC_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Initiate Cache Line Command"]
    #[inline]
    pub fn lgo(&self) -> LGOR {
        LGOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:12 - Cache address"]
    #[inline]
    pub fn cacheaddr(&self) -> CACHEADDRR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CACHEADDRR { bits }
    }
    #[doc = "Bit 14 - Way select"]
    #[inline]
    pub fn wsel(&self) -> WSELR {
        WSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Tag/Data Select"]
    #[inline]
    pub fn tdsel(&self) -> TDSELR {
        TDSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Line Command Initial Valid Bit"]
    #[inline]
    pub fn lcivb(&self) -> LCIVBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LCIVBR { bits }
    }
    #[doc = "Bit 21 - Line Command Initial Modified Bit"]
    #[inline]
    pub fn lcimb(&self) -> LCIMBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LCIMBR { bits }
    }
    #[doc = "Bit 22 - Line Command Way"]
    #[inline]
    pub fn lcway(&self) -> LCWAYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LCWAYR { bits }
    }
    #[doc = "Bits 24:25 - Line Command"]
    #[inline]
    pub fn lcmd(&self) -> LCMDR {
        LCMDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - Line Address Select"]
    #[inline]
    pub fn ladsel(&self) -> LADSELR {
        LADSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Line access type"]
    #[inline]
    pub fn lacc(&self) -> LACCR {
        LACCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
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
    #[doc = "Bit 0 - Initiate Cache Line Command"]
    #[inline]
    pub fn lgo(&mut self) -> _LGOW {
        _LGOW { w: self }
    }
    #[doc = "Bits 2:12 - Cache address"]
    #[inline]
    pub fn cacheaddr(&mut self) -> _CACHEADDRW {
        _CACHEADDRW { w: self }
    }
    #[doc = "Bit 14 - Way select"]
    #[inline]
    pub fn wsel(&mut self) -> _WSELW {
        _WSELW { w: self }
    }
    #[doc = "Bit 16 - Tag/Data Select"]
    #[inline]
    pub fn tdsel(&mut self) -> _TDSELW {
        _TDSELW { w: self }
    }
    #[doc = "Bits 24:25 - Line Command"]
    #[inline]
    pub fn lcmd(&mut self) -> _LCMDW {
        _LCMDW { w: self }
    }
    #[doc = "Bit 26 - Line Address Select"]
    #[inline]
    pub fn ladsel(&mut self) -> _LADSELW {
        _LADSELW { w: self }
    }
    #[doc = "Bit 27 - Line access type"]
    #[inline]
    pub fn lacc(&mut self) -> _LACCW {
        _LACCW { w: self }
    }
}
