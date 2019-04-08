#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TARGET_ROOT104_SET {
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
#[doc = "Possible values of the field `POST_PODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POST_PODFR {
    #[doc = "Divide by 1"]
    POST_PODF_0,
    #[doc = "Divide by 2"]
    POST_PODF_1,
    #[doc = "Divide by 3"]
    POST_PODF_2,
    #[doc = "Divide by 4"]
    POST_PODF_3,
    #[doc = "Divide by 5"]
    POST_PODF_4,
    #[doc = "Divide by 6"]
    POST_PODF_5,
    #[doc = "Divide by 64"]
    POST_PODF_63,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl POST_PODFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            POST_PODFR::POST_PODF_0 => 0,
            POST_PODFR::POST_PODF_1 => 1,
            POST_PODFR::POST_PODF_2 => 2,
            POST_PODFR::POST_PODF_3 => 3,
            POST_PODFR::POST_PODF_4 => 4,
            POST_PODFR::POST_PODF_5 => 5,
            POST_PODFR::POST_PODF_63 => 63,
            POST_PODFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> POST_PODFR {
        match value {
            0 => POST_PODFR::POST_PODF_0,
            1 => POST_PODFR::POST_PODF_1,
            2 => POST_PODFR::POST_PODF_2,
            3 => POST_PODFR::POST_PODF_3,
            4 => POST_PODFR::POST_PODF_4,
            5 => POST_PODFR::POST_PODF_5,
            63 => POST_PODFR::POST_PODF_63,
            i => POST_PODFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `POST_PODF_0`"]
    #[inline]
    pub fn is_post_podf_0(&self) -> bool {
        *self == POST_PODFR::POST_PODF_0
    }
    #[doc = "Checks if the value of the field is `POST_PODF_1`"]
    #[inline]
    pub fn is_post_podf_1(&self) -> bool {
        *self == POST_PODFR::POST_PODF_1
    }
    #[doc = "Checks if the value of the field is `POST_PODF_2`"]
    #[inline]
    pub fn is_post_podf_2(&self) -> bool {
        *self == POST_PODFR::POST_PODF_2
    }
    #[doc = "Checks if the value of the field is `POST_PODF_3`"]
    #[inline]
    pub fn is_post_podf_3(&self) -> bool {
        *self == POST_PODFR::POST_PODF_3
    }
    #[doc = "Checks if the value of the field is `POST_PODF_4`"]
    #[inline]
    pub fn is_post_podf_4(&self) -> bool {
        *self == POST_PODFR::POST_PODF_4
    }
    #[doc = "Checks if the value of the field is `POST_PODF_5`"]
    #[inline]
    pub fn is_post_podf_5(&self) -> bool {
        *self == POST_PODFR::POST_PODF_5
    }
    #[doc = "Checks if the value of the field is `POST_PODF_63`"]
    #[inline]
    pub fn is_post_podf_63(&self) -> bool {
        *self == POST_PODFR::POST_PODF_63
    }
}
#[doc = "Possible values of the field `PRE_PODF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRE_PODFR {
    #[doc = "Divide by 1"]
    PRE_PODF_0,
    #[doc = "Divide by 2"]
    PRE_PODF_1,
    #[doc = "Divide by 3"]
    PRE_PODF_2,
    #[doc = "Divide by 4"]
    PRE_PODF_3,
    #[doc = "Divide by 5"]
    PRE_PODF_4,
    #[doc = "Divide by 6"]
    PRE_PODF_5,
    #[doc = "Divide by 7"]
    PRE_PODF_6,
    #[doc = "Divide by 8"]
    PRE_PODF_7,
}
impl PRE_PODFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRE_PODFR::PRE_PODF_0 => 0,
            PRE_PODFR::PRE_PODF_1 => 1,
            PRE_PODFR::PRE_PODF_2 => 2,
            PRE_PODFR::PRE_PODF_3 => 3,
            PRE_PODFR::PRE_PODF_4 => 4,
            PRE_PODFR::PRE_PODF_5 => 5,
            PRE_PODFR::PRE_PODF_6 => 6,
            PRE_PODFR::PRE_PODF_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRE_PODFR {
        match value {
            0 => PRE_PODFR::PRE_PODF_0,
            1 => PRE_PODFR::PRE_PODF_1,
            2 => PRE_PODFR::PRE_PODF_2,
            3 => PRE_PODFR::PRE_PODF_3,
            4 => PRE_PODFR::PRE_PODF_4,
            5 => PRE_PODFR::PRE_PODF_5,
            6 => PRE_PODFR::PRE_PODF_6,
            7 => PRE_PODFR::PRE_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRE_PODF_0`"]
    #[inline]
    pub fn is_pre_podf_0(&self) -> bool {
        *self == PRE_PODFR::PRE_PODF_0
    }
    #[doc = "Checks if the value of the field is `PRE_PODF_1`"]
    #[inline]
    pub fn is_pre_podf_1(&self) -> bool {
        *self == PRE_PODFR::PRE_PODF_1
    }
    #[doc = "Checks if the value of the field is `PRE_PODF_2`"]
    #[inline]
    pub fn is_pre_podf_2(&self) -> bool {
        *self == PRE_PODFR::PRE_PODF_2
    }
    #[doc = "Checks if the value of the field is `PRE_PODF_3`"]
    #[inline]
    pub fn is_pre_podf_3(&self) -> bool {
        *self == PRE_PODFR::PRE_PODF_3
    }
    #[doc = "Checks if the value of the field is `PRE_PODF_4`"]
    #[inline]
    pub fn is_pre_podf_4(&self) -> bool {
        *self == PRE_PODFR::PRE_PODF_4
    }
    #[doc = "Checks if the value of the field is `PRE_PODF_5`"]
    #[inline]
    pub fn is_pre_podf_5(&self) -> bool {
        *self == PRE_PODFR::PRE_PODF_5
    }
    #[doc = "Checks if the value of the field is `PRE_PODF_6`"]
    #[inline]
    pub fn is_pre_podf_6(&self) -> bool {
        *self == PRE_PODFR::PRE_PODF_6
    }
    #[doc = "Checks if the value of the field is `PRE_PODF_7`"]
    #[inline]
    pub fn is_pre_podf_7(&self) -> bool {
        *self == PRE_PODFR::PRE_PODF_7
    }
}
#[doc = r" Value of the field"]
pub struct MUXR {
    bits: u8,
}
impl MUXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLER {
    #[doc = "clock root is OFF"]
    ENABLE_0,
    #[doc = "clock root is ON"]
    ENABLE_1,
}
impl ENABLER {
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
            ENABLER::ENABLE_0 => false,
            ENABLER::ENABLE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENABLER {
        match value {
            false => ENABLER::ENABLE_0,
            true => ENABLER::ENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_0`"]
    #[inline]
    pub fn is_enable_0(&self) -> bool {
        *self == ENABLER::ENABLE_0
    }
    #[doc = "Checks if the value of the field is `ENABLE_1`"]
    #[inline]
    pub fn is_enable_1(&self) -> bool {
        *self == ENABLER::ENABLE_1
    }
}
#[doc = "Values that can be written to the field `POST_PODF`"]
pub enum POST_PODFW {
    #[doc = "Divide by 1"]
    POST_PODF_0,
    #[doc = "Divide by 2"]
    POST_PODF_1,
    #[doc = "Divide by 3"]
    POST_PODF_2,
    #[doc = "Divide by 4"]
    POST_PODF_3,
    #[doc = "Divide by 5"]
    POST_PODF_4,
    #[doc = "Divide by 6"]
    POST_PODF_5,
    #[doc = "Divide by 64"]
    POST_PODF_63,
}
impl POST_PODFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            POST_PODFW::POST_PODF_0 => 0,
            POST_PODFW::POST_PODF_1 => 1,
            POST_PODFW::POST_PODF_2 => 2,
            POST_PODFW::POST_PODF_3 => 3,
            POST_PODFW::POST_PODF_4 => 4,
            POST_PODFW::POST_PODF_5 => 5,
            POST_PODFW::POST_PODF_63 => 63,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POST_PODFW<'a> {
    w: &'a mut W,
}
impl<'a> _POST_PODFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POST_PODFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn post_podf_0(self) -> &'a mut W {
        self.variant(POST_PODFW::POST_PODF_0)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn post_podf_1(self) -> &'a mut W {
        self.variant(POST_PODFW::POST_PODF_1)
    }
    #[doc = "Divide by 3"]
    #[inline]
    pub fn post_podf_2(self) -> &'a mut W {
        self.variant(POST_PODFW::POST_PODF_2)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn post_podf_3(self) -> &'a mut W {
        self.variant(POST_PODFW::POST_PODF_3)
    }
    #[doc = "Divide by 5"]
    #[inline]
    pub fn post_podf_4(self) -> &'a mut W {
        self.variant(POST_PODFW::POST_PODF_4)
    }
    #[doc = "Divide by 6"]
    #[inline]
    pub fn post_podf_5(self) -> &'a mut W {
        self.variant(POST_PODFW::POST_PODF_5)
    }
    #[doc = "Divide by 64"]
    #[inline]
    pub fn post_podf_63(self) -> &'a mut W {
        self.variant(POST_PODFW::POST_PODF_63)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRE_PODF`"]
pub enum PRE_PODFW {
    #[doc = "Divide by 1"]
    PRE_PODF_0,
    #[doc = "Divide by 2"]
    PRE_PODF_1,
    #[doc = "Divide by 3"]
    PRE_PODF_2,
    #[doc = "Divide by 4"]
    PRE_PODF_3,
    #[doc = "Divide by 5"]
    PRE_PODF_4,
    #[doc = "Divide by 6"]
    PRE_PODF_5,
    #[doc = "Divide by 7"]
    PRE_PODF_6,
    #[doc = "Divide by 8"]
    PRE_PODF_7,
}
impl PRE_PODFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRE_PODFW::PRE_PODF_0 => 0,
            PRE_PODFW::PRE_PODF_1 => 1,
            PRE_PODFW::PRE_PODF_2 => 2,
            PRE_PODFW::PRE_PODF_3 => 3,
            PRE_PODFW::PRE_PODF_4 => 4,
            PRE_PODFW::PRE_PODF_5 => 5,
            PRE_PODFW::PRE_PODF_6 => 6,
            PRE_PODFW::PRE_PODF_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRE_PODFW<'a> {
    w: &'a mut W,
}
impl<'a> _PRE_PODFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRE_PODFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn pre_podf_0(self) -> &'a mut W {
        self.variant(PRE_PODFW::PRE_PODF_0)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn pre_podf_1(self) -> &'a mut W {
        self.variant(PRE_PODFW::PRE_PODF_1)
    }
    #[doc = "Divide by 3"]
    #[inline]
    pub fn pre_podf_2(self) -> &'a mut W {
        self.variant(PRE_PODFW::PRE_PODF_2)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn pre_podf_3(self) -> &'a mut W {
        self.variant(PRE_PODFW::PRE_PODF_3)
    }
    #[doc = "Divide by 5"]
    #[inline]
    pub fn pre_podf_4(self) -> &'a mut W {
        self.variant(PRE_PODFW::PRE_PODF_4)
    }
    #[doc = "Divide by 6"]
    #[inline]
    pub fn pre_podf_5(self) -> &'a mut W {
        self.variant(PRE_PODFW::PRE_PODF_5)
    }
    #[doc = "Divide by 7"]
    #[inline]
    pub fn pre_podf_6(self) -> &'a mut W {
        self.variant(PRE_PODFW::PRE_PODF_6)
    }
    #[doc = "Divide by 8"]
    #[inline]
    pub fn pre_podf_7(self) -> &'a mut W {
        self.variant(PRE_PODFW::PRE_PODF_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MUXW<'a> {
    w: &'a mut W,
}
impl<'a> _MUXW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENABLE`"]
pub enum ENABLEW {
    #[doc = "clock root is OFF"]
    ENABLE_0,
    #[doc = "clock root is ON"]
    ENABLE_1,
}
impl ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENABLEW::ENABLE_0 => false,
            ENABLEW::ENABLE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "clock root is OFF"]
    #[inline]
    pub fn enable_0(self) -> &'a mut W {
        self.variant(ENABLEW::ENABLE_0)
    }
    #[doc = "clock root is ON"]
    #[inline]
    pub fn enable_1(self) -> &'a mut W {
        self.variant(ENABLEW::ENABLE_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - Post divider divide the number Divider value is n + 1"]
    #[inline]
    pub fn post_podf(&self) -> POST_PODFR {
        POST_PODFR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - Pre divider divide the number Divider value is n+1 This field does not apply for CORE, DRAM, DRAM_PHYM"]
    #[inline]
    pub fn pre_podf(&self) -> PRE_PODFR {
        PRE_PODFR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:26 - Selection of clock sources This field is 1 bit long for DRAM and CORE"]
    #[inline]
    pub fn mux(&self) -> MUXR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MUXR { bits }
    }
    #[doc = "Bit 28 - Enable this clock"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:5 - Post divider divide the number Divider value is n + 1"]
    #[inline]
    pub fn post_podf(&mut self) -> _POST_PODFW {
        _POST_PODFW { w: self }
    }
    #[doc = "Bits 16:18 - Pre divider divide the number Divider value is n+1 This field does not apply for CORE, DRAM, DRAM_PHYM"]
    #[inline]
    pub fn pre_podf(&mut self) -> _PRE_PODFW {
        _PRE_PODFW { w: self }
    }
    #[doc = "Bits 24:26 - Selection of clock sources This field is 1 bit long for DRAM and CORE"]
    #[inline]
    pub fn mux(&mut self) -> _MUXW {
        _MUXW { w: self }
    }
    #[doc = "Bit 28 - Enable this clock"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
}
