#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEBUG0 {
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
pub struct DEBUG_REG_SELECTR {
    bits: u8,
}
impl DEBUG_REG_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RSVD0R {
    bits: u8,
}
impl RSVD0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BM_KES_TEST_BYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BM_KES_TEST_BYPASSR {
    #[doc = "Bus master address generator for SYND_GEN writes operates normally."]
    NORMAL,
    #[doc = "Bus master address generator always addresses last four bytes in Auxiliary block."]
    TEST_MODE,
}
impl BM_KES_TEST_BYPASSR {
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
            BM_KES_TEST_BYPASSR::NORMAL => false,
            BM_KES_TEST_BYPASSR::TEST_MODE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BM_KES_TEST_BYPASSR {
        match value {
            false => BM_KES_TEST_BYPASSR::NORMAL,
            true => BM_KES_TEST_BYPASSR::TEST_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == BM_KES_TEST_BYPASSR::NORMAL
    }
    #[doc = "Checks if the value of the field is `TEST_MODE`"]
    #[inline]
    pub fn is_test_mode(&self) -> bool {
        *self == BM_KES_TEST_BYPASSR::TEST_MODE
    }
}
#[doc = "Possible values of the field `KES_DEBUG_STALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KES_DEBUG_STALLR {
    #[doc = "KES FSM proceeds to next block supplied by bus master."]
    NORMAL,
    #[doc = "KES FSM waits after current equations are solved and the search engine is started."]
    WAIT,
}
impl KES_DEBUG_STALLR {
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
            KES_DEBUG_STALLR::NORMAL => false,
            KES_DEBUG_STALLR::WAIT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> KES_DEBUG_STALLR {
        match value {
            false => KES_DEBUG_STALLR::NORMAL,
            true => KES_DEBUG_STALLR::WAIT,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == KES_DEBUG_STALLR::NORMAL
    }
    #[doc = "Checks if the value of the field is `WAIT`"]
    #[inline]
    pub fn is_wait(&self) -> bool {
        *self == KES_DEBUG_STALLR::WAIT
    }
}
#[doc = r" Value of the field"]
pub struct KES_DEBUG_STEPR {
    bits: bool,
}
impl KES_DEBUG_STEPR {
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
#[doc = "Possible values of the field `KES_STANDALONE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KES_STANDALONER {
    #[doc = "Bus master address generator for SYND_GEN writes operates normally."]
    NORMAL,
    #[doc = "Bus master address generator always addresses last four bytes in Auxiliary block."]
    TEST_MODE,
}
impl KES_STANDALONER {
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
            KES_STANDALONER::NORMAL => false,
            KES_STANDALONER::TEST_MODE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> KES_STANDALONER {
        match value {
            false => KES_STANDALONER::NORMAL,
            true => KES_STANDALONER::TEST_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == KES_STANDALONER::NORMAL
    }
    #[doc = "Checks if the value of the field is `TEST_MODE`"]
    #[inline]
    pub fn is_test_mode(&self) -> bool {
        *self == KES_STANDALONER::TEST_MODE
    }
}
#[doc = r" Value of the field"]
pub struct KES_DEBUG_KICKR {
    bits: bool,
}
impl KES_DEBUG_KICKR {
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
#[doc = "Possible values of the field `KES_DEBUG_MODE4K`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KES_DEBUG_MODE4KR {
    #[doc = "Mode is set for 4K NAND pages."]
    _4K,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl KES_DEBUG_MODE4KR {
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
            KES_DEBUG_MODE4KR::_4K => true,
            KES_DEBUG_MODE4KR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> KES_DEBUG_MODE4KR {
        match value {
            true => KES_DEBUG_MODE4KR::_4K,
            i => KES_DEBUG_MODE4KR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4K`"]
    #[inline]
    pub fn is_4k(&self) -> bool {
        *self == KES_DEBUG_MODE4KR::_4K
    }
}
#[doc = "Possible values of the field `KES_DEBUG_PAYLOAD_FLAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KES_DEBUG_PAYLOAD_FLAGR {
    #[doc = "Payload is set for 512 bytes data block."]
    DATA,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl KES_DEBUG_PAYLOAD_FLAGR {
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
            KES_DEBUG_PAYLOAD_FLAGR::DATA => true,
            KES_DEBUG_PAYLOAD_FLAGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> KES_DEBUG_PAYLOAD_FLAGR {
        match value {
            true => KES_DEBUG_PAYLOAD_FLAGR::DATA,
            i => KES_DEBUG_PAYLOAD_FLAGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline]
    pub fn is_data(&self) -> bool {
        *self == KES_DEBUG_PAYLOAD_FLAGR::DATA
    }
}
#[doc = r" Value of the field"]
pub struct KES_DEBUG_SHIFT_SYNDR {
    bits: bool,
}
impl KES_DEBUG_SHIFT_SYNDR {
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
#[doc = "Possible values of the field `KES_DEBUG_SYNDROME_SYMBOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KES_DEBUG_SYNDROME_SYMBOLR {
    #[doc = "Bus master address generator for SYND_GEN writes operates normally."]
    NORMAL,
    #[doc = "Bus master address generator always addresses last four bytes in Auxiliary block."]
    TEST_MODE,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl KES_DEBUG_SYNDROME_SYMBOLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            KES_DEBUG_SYNDROME_SYMBOLR::NORMAL => 0,
            KES_DEBUG_SYNDROME_SYMBOLR::TEST_MODE => 1,
            KES_DEBUG_SYNDROME_SYMBOLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> KES_DEBUG_SYNDROME_SYMBOLR {
        match value {
            0 => KES_DEBUG_SYNDROME_SYMBOLR::NORMAL,
            1 => KES_DEBUG_SYNDROME_SYMBOLR::TEST_MODE,
            i => KES_DEBUG_SYNDROME_SYMBOLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == KES_DEBUG_SYNDROME_SYMBOLR::NORMAL
    }
    #[doc = "Checks if the value of the field is `TEST_MODE`"]
    #[inline]
    pub fn is_test_mode(&self) -> bool {
        *self == KES_DEBUG_SYNDROME_SYMBOLR::TEST_MODE
    }
}
#[doc = r" Value of the field"]
pub struct RSVD1R {
    bits: u8,
}
impl RSVD1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DEBUG_REG_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBUG_REG_SELECTW<'a> {
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
#[doc = "Values that can be written to the field `BM_KES_TEST_BYPASS`"]
pub enum BM_KES_TEST_BYPASSW {
    #[doc = "Bus master address generator for SYND_GEN writes operates normally."]
    NORMAL,
    #[doc = "Bus master address generator always addresses last four bytes in Auxiliary block."]
    TEST_MODE,
}
impl BM_KES_TEST_BYPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BM_KES_TEST_BYPASSW::NORMAL => false,
            BM_KES_TEST_BYPASSW::TEST_MODE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BM_KES_TEST_BYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _BM_KES_TEST_BYPASSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BM_KES_TEST_BYPASSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bus master address generator for SYND_GEN writes operates normally."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(BM_KES_TEST_BYPASSW::NORMAL)
    }
    #[doc = "Bus master address generator always addresses last four bytes in Auxiliary block."]
    #[inline]
    pub fn test_mode(self) -> &'a mut W {
        self.variant(BM_KES_TEST_BYPASSW::TEST_MODE)
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
#[doc = "Values that can be written to the field `KES_DEBUG_STALL`"]
pub enum KES_DEBUG_STALLW {
    #[doc = "KES FSM proceeds to next block supplied by bus master."]
    NORMAL,
    #[doc = "KES FSM waits after current equations are solved and the search engine is started."]
    WAIT,
}
impl KES_DEBUG_STALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            KES_DEBUG_STALLW::NORMAL => false,
            KES_DEBUG_STALLW::WAIT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KES_DEBUG_STALLW<'a> {
    w: &'a mut W,
}
impl<'a> _KES_DEBUG_STALLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KES_DEBUG_STALLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "KES FSM proceeds to next block supplied by bus master."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(KES_DEBUG_STALLW::NORMAL)
    }
    #[doc = "KES FSM waits after current equations are solved and the search engine is started."]
    #[inline]
    pub fn wait(self) -> &'a mut W {
        self.variant(KES_DEBUG_STALLW::WAIT)
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
#[doc = r" Proxy"]
pub struct _KES_DEBUG_STEPW<'a> {
    w: &'a mut W,
}
impl<'a> _KES_DEBUG_STEPW<'a> {
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
#[doc = "Values that can be written to the field `KES_STANDALONE`"]
pub enum KES_STANDALONEW {
    #[doc = "Bus master address generator for SYND_GEN writes operates normally."]
    NORMAL,
    #[doc = "Bus master address generator always addresses last four bytes in Auxiliary block."]
    TEST_MODE,
}
impl KES_STANDALONEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            KES_STANDALONEW::NORMAL => false,
            KES_STANDALONEW::TEST_MODE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KES_STANDALONEW<'a> {
    w: &'a mut W,
}
impl<'a> _KES_STANDALONEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KES_STANDALONEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bus master address generator for SYND_GEN writes operates normally."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(KES_STANDALONEW::NORMAL)
    }
    #[doc = "Bus master address generator always addresses last four bytes in Auxiliary block."]
    #[inline]
    pub fn test_mode(self) -> &'a mut W {
        self.variant(KES_STANDALONEW::TEST_MODE)
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
#[doc = r" Proxy"]
pub struct _KES_DEBUG_KICKW<'a> {
    w: &'a mut W,
}
impl<'a> _KES_DEBUG_KICKW<'a> {
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
#[doc = "Values that can be written to the field `KES_DEBUG_MODE4K`"]
pub enum KES_DEBUG_MODE4KW {
    #[doc = "Mode is set for 4K NAND pages."]
    _4K,
}
impl KES_DEBUG_MODE4KW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            KES_DEBUG_MODE4KW::_4K => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KES_DEBUG_MODE4KW<'a> {
    w: &'a mut W,
}
impl<'a> _KES_DEBUG_MODE4KW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KES_DEBUG_MODE4KW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Mode is set for 4K NAND pages."]
    #[inline]
    pub fn _4k(self) -> &'a mut W {
        self.variant(KES_DEBUG_MODE4KW::_4K)
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
#[doc = "Values that can be written to the field `KES_DEBUG_PAYLOAD_FLAG`"]
pub enum KES_DEBUG_PAYLOAD_FLAGW {
    #[doc = "Payload is set for 512 bytes data block."]
    DATA,
}
impl KES_DEBUG_PAYLOAD_FLAGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            KES_DEBUG_PAYLOAD_FLAGW::DATA => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KES_DEBUG_PAYLOAD_FLAGW<'a> {
    w: &'a mut W,
}
impl<'a> _KES_DEBUG_PAYLOAD_FLAGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KES_DEBUG_PAYLOAD_FLAGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Payload is set for 512 bytes data block."]
    #[inline]
    pub fn data(self) -> &'a mut W {
        self.variant(KES_DEBUG_PAYLOAD_FLAGW::DATA)
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
#[doc = r" Proxy"]
pub struct _KES_DEBUG_SHIFT_SYNDW<'a> {
    w: &'a mut W,
}
impl<'a> _KES_DEBUG_SHIFT_SYNDW<'a> {
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
#[doc = "Values that can be written to the field `KES_DEBUG_SYNDROME_SYMBOL`"]
pub enum KES_DEBUG_SYNDROME_SYMBOLW {
    #[doc = "Bus master address generator for SYND_GEN writes operates normally."]
    NORMAL,
    #[doc = "Bus master address generator always addresses last four bytes in Auxiliary block."]
    TEST_MODE,
}
impl KES_DEBUG_SYNDROME_SYMBOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            KES_DEBUG_SYNDROME_SYMBOLW::NORMAL => 0,
            KES_DEBUG_SYNDROME_SYMBOLW::TEST_MODE => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KES_DEBUG_SYNDROME_SYMBOLW<'a> {
    w: &'a mut W,
}
impl<'a> _KES_DEBUG_SYNDROME_SYMBOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KES_DEBUG_SYNDROME_SYMBOLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Bus master address generator for SYND_GEN writes operates normally."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(KES_DEBUG_SYNDROME_SYMBOLW::NORMAL)
    }
    #[doc = "Bus master address generator always addresses last four bytes in Auxiliary block."]
    #[inline]
    pub fn test_mode(self) -> &'a mut W {
        self.variant(KES_DEBUG_SYNDROME_SYMBOLW::TEST_MODE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:5 - The value loaded in this bit field is used to select the internal register state view of KES engine or the Chien search engine"]
    #[inline]
    pub fn debug_reg_select(&self) -> DEBUG_REG_SELECTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DEBUG_REG_SELECTR { bits }
    }
    #[doc = "Bits 6:7 - This field is reserved."]
    #[inline]
    pub fn rsvd0(&self) -> RSVD0R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSVD0R { bits }
    }
    #[doc = "Bit 8 - 1 = Point all SYND_GEN writes to dummy area at the end of the AUXILLIARY block so that diagnostics can preload all payload, parity bytes and computed syndrome bytes for test the KES engine"]
    #[inline]
    pub fn bm_kes_test_bypass(&self) -> BM_KES_TEST_BYPASSR {
        BM_KES_TEST_BYPASSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Set to one to cause KES FSM to stall after notifying Chien search engine to start processing its block but before notifying the bus master that the KES computation is complete"]
    #[inline]
    pub fn kes_debug_stall(&self) -> KES_DEBUG_STALLR {
        KES_DEBUG_STALLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Toggling this bit causes the KES FSM to skip passed the stall state if it is in DEBUG_STALL mode and completed processing a block"]
    #[inline]
    pub fn kes_debug_step(&self) -> KES_DEBUG_STEPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        KES_DEBUG_STEPR { bits }
    }
    #[doc = "Bit 11 - Set to one, cause the KES engine to suppress toggling the KES_BM_DONE signal to the bus master and suppress toggling the CF_BM_DONE signal by the CF engine"]
    #[inline]
    pub fn kes_standalone(&self) -> KES_STANDALONER {
        KES_STANDALONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Toggling causes KES engine FSM to start as if kick by the Bus Master"]
    #[inline]
    pub fn kes_debug_kick(&self) -> KES_DEBUG_KICKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        KES_DEBUG_KICKR { bits }
    }
    #[doc = "Bit 13 - When running the stand alone debug mode on the error calculator, the state of this bit is presented to the KES engine as the input mode (4K or 2K pages)"]
    #[inline]
    pub fn kes_debug_mode4k(&self) -> KES_DEBUG_MODE4KR {
        KES_DEBUG_MODE4KR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - When running the stand alone debug mode on the error calculator, the state of this bit is presented to the KES engine as the input payload flag"]
    #[inline]
    pub fn kes_debug_payload_flag(&self) -> KES_DEBUG_PAYLOAD_FLAGR {
        KES_DEBUG_PAYLOAD_FLAGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Toggling this bit causes the value in BCH_DEBUG0_KES_SYNDROME_SYMBOL to be shift into the syndrome register array at the input to the KES engine"]
    #[inline]
    pub fn kes_debug_shift_synd(&self) -> KES_DEBUG_SHIFT_SYNDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        KES_DEBUG_SHIFT_SYNDR { bits }
    }
    #[doc = "Bits 16:24 - The 9 bit value in this bit field shifts into the syndrome register array at the input of the KES engine whenever BCH_DEBUG0_KES_DEBUG_SHIFT_SYND is toggled"]
    #[inline]
    pub fn kes_debug_syndrome_symbol(&self) -> KES_DEBUG_SYNDROME_SYMBOLR {
        KES_DEBUG_SYNDROME_SYMBOLR::_from({
            const MASK: u16 = 511;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 25:31 - This field is reserved."]
    #[inline]
    pub fn rsvd1(&self) -> RSVD1R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSVD1R { bits }
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
    #[doc = "Bits 0:5 - The value loaded in this bit field is used to select the internal register state view of KES engine or the Chien search engine"]
    #[inline]
    pub fn debug_reg_select(&mut self) -> _DEBUG_REG_SELECTW {
        _DEBUG_REG_SELECTW { w: self }
    }
    #[doc = "Bit 8 - 1 = Point all SYND_GEN writes to dummy area at the end of the AUXILLIARY block so that diagnostics can preload all payload, parity bytes and computed syndrome bytes for test the KES engine"]
    #[inline]
    pub fn bm_kes_test_bypass(&mut self) -> _BM_KES_TEST_BYPASSW {
        _BM_KES_TEST_BYPASSW { w: self }
    }
    #[doc = "Bit 9 - Set to one to cause KES FSM to stall after notifying Chien search engine to start processing its block but before notifying the bus master that the KES computation is complete"]
    #[inline]
    pub fn kes_debug_stall(&mut self) -> _KES_DEBUG_STALLW {
        _KES_DEBUG_STALLW { w: self }
    }
    #[doc = "Bit 10 - Toggling this bit causes the KES FSM to skip passed the stall state if it is in DEBUG_STALL mode and completed processing a block"]
    #[inline]
    pub fn kes_debug_step(&mut self) -> _KES_DEBUG_STEPW {
        _KES_DEBUG_STEPW { w: self }
    }
    #[doc = "Bit 11 - Set to one, cause the KES engine to suppress toggling the KES_BM_DONE signal to the bus master and suppress toggling the CF_BM_DONE signal by the CF engine"]
    #[inline]
    pub fn kes_standalone(&mut self) -> _KES_STANDALONEW {
        _KES_STANDALONEW { w: self }
    }
    #[doc = "Bit 12 - Toggling causes KES engine FSM to start as if kick by the Bus Master"]
    #[inline]
    pub fn kes_debug_kick(&mut self) -> _KES_DEBUG_KICKW {
        _KES_DEBUG_KICKW { w: self }
    }
    #[doc = "Bit 13 - When running the stand alone debug mode on the error calculator, the state of this bit is presented to the KES engine as the input mode (4K or 2K pages)"]
    #[inline]
    pub fn kes_debug_mode4k(&mut self) -> _KES_DEBUG_MODE4KW {
        _KES_DEBUG_MODE4KW { w: self }
    }
    #[doc = "Bit 14 - When running the stand alone debug mode on the error calculator, the state of this bit is presented to the KES engine as the input payload flag"]
    #[inline]
    pub fn kes_debug_payload_flag(&mut self) -> _KES_DEBUG_PAYLOAD_FLAGW {
        _KES_DEBUG_PAYLOAD_FLAGW { w: self }
    }
    #[doc = "Bit 15 - Toggling this bit causes the value in BCH_DEBUG0_KES_SYNDROME_SYMBOL to be shift into the syndrome register array at the input to the KES engine"]
    #[inline]
    pub fn kes_debug_shift_synd(&mut self) -> _KES_DEBUG_SHIFT_SYNDW {
        _KES_DEBUG_SHIFT_SYNDW { w: self }
    }
    #[doc = "Bits 16:24 - The 9 bit value in this bit field shifts into the syndrome register array at the input of the KES engine whenever BCH_DEBUG0_KES_DEBUG_SHIFT_SYND is toggled"]
    #[inline]
    pub fn kes_debug_syndrome_symbol(&mut self) -> _KES_DEBUG_SYNDROME_SYMBOLW {
        _KES_DEBUG_SYNDROME_SYMBOLW { w: self }
    }
}
