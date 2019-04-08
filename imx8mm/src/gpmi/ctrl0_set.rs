#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL0_SET {
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
pub struct XFER_COUNTR {
    bits: u16,
}
impl XFER_COUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `ADDRESS_INCREMENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESS_INCREMENTR {
    #[doc = "Address does not increment."]
    ADDRESS_INCREMENT_0,
    #[doc = "Increment address."]
    ADDRESS_INCREMENT_1,
}
impl ADDRESS_INCREMENTR {
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
            ADDRESS_INCREMENTR::ADDRESS_INCREMENT_0 => false,
            ADDRESS_INCREMENTR::ADDRESS_INCREMENT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADDRESS_INCREMENTR {
        match value {
            false => ADDRESS_INCREMENTR::ADDRESS_INCREMENT_0,
            true => ADDRESS_INCREMENTR::ADDRESS_INCREMENT_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADDRESS_INCREMENT_0`"]
    #[inline]
    pub fn is_address_increment_0(&self) -> bool {
        *self == ADDRESS_INCREMENTR::ADDRESS_INCREMENT_0
    }
    #[doc = "Checks if the value of the field is `ADDRESS_INCREMENT_1`"]
    #[inline]
    pub fn is_address_increment_1(&self) -> bool {
        *self == ADDRESS_INCREMENTR::ADDRESS_INCREMENT_1
    }
}
#[doc = r" Value of the field"]
pub struct ADDRESSR {
    bits: u8,
}
impl ADDRESSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CSR {
    bits: u8,
}
impl CSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `WORD_LENGTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WORD_LENGTHR {
    #[doc = "8-bit Data Bus mode."]
    WORD_LENGTH_1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl WORD_LENGTHR {
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
            WORD_LENGTHR::WORD_LENGTH_1 => true,
            WORD_LENGTHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WORD_LENGTHR {
        match value {
            true => WORD_LENGTHR::WORD_LENGTH_1,
            i => WORD_LENGTHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `WORD_LENGTH_1`"]
    #[inline]
    pub fn is_word_length_1(&self) -> bool {
        *self == WORD_LENGTHR::WORD_LENGTH_1
    }
}
#[doc = "Possible values of the field `COMMAND_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMMAND_MODER {
    #[doc = "Write mode."]
    COMMAND_MODE_0,
    #[doc = "Read Mode."]
    COMMAND_MODE_1,
    #[doc = "Read and Compare Mode (setting sense flop)."]
    COMMAND_MODE_2,
    #[doc = "Wait for Ready."]
    COMMAND_MODE_3,
}
impl COMMAND_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            COMMAND_MODER::COMMAND_MODE_0 => 0,
            COMMAND_MODER::COMMAND_MODE_1 => 1,
            COMMAND_MODER::COMMAND_MODE_2 => 2,
            COMMAND_MODER::COMMAND_MODE_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> COMMAND_MODER {
        match value {
            0 => COMMAND_MODER::COMMAND_MODE_0,
            1 => COMMAND_MODER::COMMAND_MODE_1,
            2 => COMMAND_MODER::COMMAND_MODE_2,
            3 => COMMAND_MODER::COMMAND_MODE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COMMAND_MODE_0`"]
    #[inline]
    pub fn is_command_mode_0(&self) -> bool {
        *self == COMMAND_MODER::COMMAND_MODE_0
    }
    #[doc = "Checks if the value of the field is `COMMAND_MODE_1`"]
    #[inline]
    pub fn is_command_mode_1(&self) -> bool {
        *self == COMMAND_MODER::COMMAND_MODE_1
    }
    #[doc = "Checks if the value of the field is `COMMAND_MODE_2`"]
    #[inline]
    pub fn is_command_mode_2(&self) -> bool {
        *self == COMMAND_MODER::COMMAND_MODE_2
    }
    #[doc = "Checks if the value of the field is `COMMAND_MODE_3`"]
    #[inline]
    pub fn is_command_mode_3(&self) -> bool {
        *self == COMMAND_MODER::COMMAND_MODE_3
    }
}
#[doc = "Possible values of the field `UDMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UDMAR {
    #[doc = "Use ATA-PIO mode on the external bus."]
    UDMA_0,
    #[doc = "Use ATA-Ultra DMA mode on the external bus."]
    UDMA_1,
}
impl UDMAR {
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
            UDMAR::UDMA_0 => false,
            UDMAR::UDMA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UDMAR {
        match value {
            false => UDMAR::UDMA_0,
            true => UDMAR::UDMA_1,
        }
    }
    #[doc = "Checks if the value of the field is `UDMA_0`"]
    #[inline]
    pub fn is_udma_0(&self) -> bool {
        *self == UDMAR::UDMA_0
    }
    #[doc = "Checks if the value of the field is `UDMA_1`"]
    #[inline]
    pub fn is_udma_1(&self) -> bool {
        *self == UDMAR::UDMA_1
    }
}
#[doc = r" Value of the field"]
pub struct LOCK_CSR {
    bits: bool,
}
impl LOCK_CSR {
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
pub struct DEV_IRQ_ENR {
    bits: bool,
}
impl DEV_IRQ_ENR {
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
pub struct RUNR {
    bits: bool,
}
impl RUNR {
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
pub struct CLKGATER {
    bits: bool,
}
impl CLKGATER {
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
pub struct SFTRSTR {
    bits: bool,
}
impl SFTRSTR {
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
#[doc = r" Proxy"]
pub struct _XFER_COUNTW<'a> {
    w: &'a mut W,
}
impl<'a> _XFER_COUNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADDRESS_INCREMENT`"]
pub enum ADDRESS_INCREMENTW {
    #[doc = "Address does not increment."]
    ADDRESS_INCREMENT_0,
    #[doc = "Increment address."]
    ADDRESS_INCREMENT_1,
}
impl ADDRESS_INCREMENTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADDRESS_INCREMENTW::ADDRESS_INCREMENT_0 => false,
            ADDRESS_INCREMENTW::ADDRESS_INCREMENT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADDRESS_INCREMENTW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRESS_INCREMENTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADDRESS_INCREMENTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Address does not increment."]
    #[inline]
    pub fn address_increment_0(self) -> &'a mut W {
        self.variant(ADDRESS_INCREMENTW::ADDRESS_INCREMENT_0)
    }
    #[doc = "Increment address."]
    #[inline]
    pub fn address_increment_1(self) -> &'a mut W {
        self.variant(ADDRESS_INCREMENTW::ADDRESS_INCREMENT_1)
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
#[doc = r" Proxy"]
pub struct _ADDRESSW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRESSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CSW<'a> {
    w: &'a mut W,
}
impl<'a> _CSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WORD_LENGTH`"]
pub enum WORD_LENGTHW {
    #[doc = "8-bit Data Bus mode."]
    WORD_LENGTH_1,
}
impl WORD_LENGTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WORD_LENGTHW::WORD_LENGTH_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WORD_LENGTHW<'a> {
    w: &'a mut W,
}
impl<'a> _WORD_LENGTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WORD_LENGTHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "8-bit Data Bus mode."]
    #[inline]
    pub fn word_length_1(self) -> &'a mut W {
        self.variant(WORD_LENGTHW::WORD_LENGTH_1)
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
#[doc = "Values that can be written to the field `COMMAND_MODE`"]
pub enum COMMAND_MODEW {
    #[doc = "Write mode."]
    COMMAND_MODE_0,
    #[doc = "Read Mode."]
    COMMAND_MODE_1,
    #[doc = "Read and Compare Mode (setting sense flop)."]
    COMMAND_MODE_2,
    #[doc = "Wait for Ready."]
    COMMAND_MODE_3,
}
impl COMMAND_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            COMMAND_MODEW::COMMAND_MODE_0 => 0,
            COMMAND_MODEW::COMMAND_MODE_1 => 1,
            COMMAND_MODEW::COMMAND_MODE_2 => 2,
            COMMAND_MODEW::COMMAND_MODE_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMMAND_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _COMMAND_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMMAND_MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Write mode."]
    #[inline]
    pub fn command_mode_0(self) -> &'a mut W {
        self.variant(COMMAND_MODEW::COMMAND_MODE_0)
    }
    #[doc = "Read Mode."]
    #[inline]
    pub fn command_mode_1(self) -> &'a mut W {
        self.variant(COMMAND_MODEW::COMMAND_MODE_1)
    }
    #[doc = "Read and Compare Mode (setting sense flop)."]
    #[inline]
    pub fn command_mode_2(self) -> &'a mut W {
        self.variant(COMMAND_MODEW::COMMAND_MODE_2)
    }
    #[doc = "Wait for Ready."]
    #[inline]
    pub fn command_mode_3(self) -> &'a mut W {
        self.variant(COMMAND_MODEW::COMMAND_MODE_3)
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
#[doc = "Values that can be written to the field `UDMA`"]
pub enum UDMAW {
    #[doc = "Use ATA-PIO mode on the external bus."]
    UDMA_0,
    #[doc = "Use ATA-Ultra DMA mode on the external bus."]
    UDMA_1,
}
impl UDMAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UDMAW::UDMA_0 => false,
            UDMAW::UDMA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UDMAW<'a> {
    w: &'a mut W,
}
impl<'a> _UDMAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UDMAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use ATA-PIO mode on the external bus."]
    #[inline]
    pub fn udma_0(self) -> &'a mut W {
        self.variant(UDMAW::UDMA_0)
    }
    #[doc = "Use ATA-Ultra DMA mode on the external bus."]
    #[inline]
    pub fn udma_1(self) -> &'a mut W {
        self.variant(UDMAW::UDMA_1)
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
#[doc = r" Proxy"]
pub struct _LOCK_CSW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_CSW<'a> {
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
pub struct _DEV_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DEV_IRQ_ENW<'a> {
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
pub struct _RUNW<'a> {
    w: &'a mut W,
}
impl<'a> _RUNW<'a> {
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
pub struct _CLKGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKGATEW<'a> {
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
pub struct _SFTRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SFTRSTW<'a> {
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
    #[doc = "Bits 0:15 - Number of bytes to transfer for this command. A value of zero will transfer 64K bytes."]
    #[inline]
    pub fn xfer_count(&self) -> XFER_COUNTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        XFER_COUNTR { bits }
    }
    #[doc = "Bit 16 - In ATA mode, the address will increment with each cycle"]
    #[inline]
    pub fn address_increment(&self) -> ADDRESS_INCREMENTR {
        ADDRESS_INCREMENTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 17:19 - Specifies the three address lines for ATA mode"]
    #[inline]
    pub fn address(&self) -> ADDRESSR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADDRESSR { bits }
    }
    #[doc = "Bits 20:22 - Selects which chip select is active for this command"]
    #[inline]
    pub fn cs(&self) -> CSR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CSR { bits }
    }
    #[doc = "Bit 23 - This bit should only be changed when RUN==0"]
    #[inline]
    pub fn word_length(&self) -> WORD_LENGTHR {
        WORD_LENGTHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:25 - WRITE = 0x0 Write mode"]
    #[inline]
    pub fn command_mode(&self) -> COMMAND_MODER {
        COMMAND_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - DISABLED = 0x0 Use ATA-PIO mode on the external bus"]
    #[inline]
    pub fn udma(&self) -> UDMAR {
        UDMAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - For ATA/NAND mode: 0= Deassert chip select (CS) after RUN is complete"]
    #[inline]
    pub fn lock_cs(&self) -> LOCK_CSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LOCK_CSR { bits }
    }
    #[doc = "Bit 28 - When set to '1' and ATA_IRQ pin is asserted, the GPMI_IRQ output will assert."]
    #[inline]
    pub fn dev_irq_en(&self) -> DEV_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DEV_IRQ_ENR { bits }
    }
    #[doc = "Bit 29 - The GPMI is busy running a command whenever this bit is set to '1'"]
    #[inline]
    pub fn run(&self) -> RUNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RUNR { bits }
    }
    #[doc = "Bit 30 - Set this bit zero for normal operation"]
    #[inline]
    pub fn clkgate(&self) -> CLKGATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKGATER { bits }
    }
    #[doc = "Bit 31 - Set to zero for normal operation"]
    #[inline]
    pub fn sftrst(&self) -> SFTRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SFTRSTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3221225472 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - Number of bytes to transfer for this command. A value of zero will transfer 64K bytes."]
    #[inline]
    pub fn xfer_count(&mut self) -> _XFER_COUNTW {
        _XFER_COUNTW { w: self }
    }
    #[doc = "Bit 16 - In ATA mode, the address will increment with each cycle"]
    #[inline]
    pub fn address_increment(&mut self) -> _ADDRESS_INCREMENTW {
        _ADDRESS_INCREMENTW { w: self }
    }
    #[doc = "Bits 17:19 - Specifies the three address lines for ATA mode"]
    #[inline]
    pub fn address(&mut self) -> _ADDRESSW {
        _ADDRESSW { w: self }
    }
    #[doc = "Bits 20:22 - Selects which chip select is active for this command"]
    #[inline]
    pub fn cs(&mut self) -> _CSW {
        _CSW { w: self }
    }
    #[doc = "Bit 23 - This bit should only be changed when RUN==0"]
    #[inline]
    pub fn word_length(&mut self) -> _WORD_LENGTHW {
        _WORD_LENGTHW { w: self }
    }
    #[doc = "Bits 24:25 - WRITE = 0x0 Write mode"]
    #[inline]
    pub fn command_mode(&mut self) -> _COMMAND_MODEW {
        _COMMAND_MODEW { w: self }
    }
    #[doc = "Bit 26 - DISABLED = 0x0 Use ATA-PIO mode on the external bus"]
    #[inline]
    pub fn udma(&mut self) -> _UDMAW {
        _UDMAW { w: self }
    }
    #[doc = "Bit 27 - For ATA/NAND mode: 0= Deassert chip select (CS) after RUN is complete"]
    #[inline]
    pub fn lock_cs(&mut self) -> _LOCK_CSW {
        _LOCK_CSW { w: self }
    }
    #[doc = "Bit 28 - When set to '1' and ATA_IRQ pin is asserted, the GPMI_IRQ output will assert."]
    #[inline]
    pub fn dev_irq_en(&mut self) -> _DEV_IRQ_ENW {
        _DEV_IRQ_ENW { w: self }
    }
    #[doc = "Bit 29 - The GPMI is busy running a command whenever this bit is set to '1'"]
    #[inline]
    pub fn run(&mut self) -> _RUNW {
        _RUNW { w: self }
    }
    #[doc = "Bit 30 - Set this bit zero for normal operation"]
    #[inline]
    pub fn clkgate(&mut self) -> _CLKGATEW {
        _CLKGATEW { w: self }
    }
    #[doc = "Bit 31 - Set to zero for normal operation"]
    #[inline]
    pub fn sftrst(&mut self) -> _SFTRSTW {
        _SFTRSTW { w: self }
    }
}
