#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::XTRIG_CONF1 {
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
pub struct NUM0R {
    bits: u8,
}
impl NUM0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CNF0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNF0R {
    #[doc = "channel"]
    CNF0_0,
    #[doc = "DMA request"]
    CNF0_1,
}
impl CNF0R {
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
            CNF0R::CNF0_0 => false,
            CNF0R::CNF0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNF0R {
        match value {
            false => CNF0R::CNF0_0,
            true => CNF0R::CNF0_1,
        }
    }
    #[doc = "Checks if the value of the field is `CNF0_0`"]
    #[inline]
    pub fn is_cnf0_0(&self) -> bool {
        *self == CNF0R::CNF0_0
    }
    #[doc = "Checks if the value of the field is `CNF0_1`"]
    #[inline]
    pub fn is_cnf0_1(&self) -> bool {
        *self == CNF0R::CNF0_1
    }
}
#[doc = r" Value of the field"]
pub struct NUM1R {
    bits: u8,
}
impl NUM1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CNF1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNF1R {
    #[doc = "channel"]
    CNF1_0,
    #[doc = "DMA request"]
    CNF1_1,
}
impl CNF1R {
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
            CNF1R::CNF1_0 => false,
            CNF1R::CNF1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNF1R {
        match value {
            false => CNF1R::CNF1_0,
            true => CNF1R::CNF1_1,
        }
    }
    #[doc = "Checks if the value of the field is `CNF1_0`"]
    #[inline]
    pub fn is_cnf1_0(&self) -> bool {
        *self == CNF1R::CNF1_0
    }
    #[doc = "Checks if the value of the field is `CNF1_1`"]
    #[inline]
    pub fn is_cnf1_1(&self) -> bool {
        *self == CNF1R::CNF1_1
    }
}
#[doc = r" Value of the field"]
pub struct NUM2R {
    bits: u8,
}
impl NUM2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CNF2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNF2R {
    #[doc = "channel"]
    CNF2_0,
    #[doc = "DMA request"]
    CNF2_1,
}
impl CNF2R {
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
            CNF2R::CNF2_0 => false,
            CNF2R::CNF2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNF2R {
        match value {
            false => CNF2R::CNF2_0,
            true => CNF2R::CNF2_1,
        }
    }
    #[doc = "Checks if the value of the field is `CNF2_0`"]
    #[inline]
    pub fn is_cnf2_0(&self) -> bool {
        *self == CNF2R::CNF2_0
    }
    #[doc = "Checks if the value of the field is `CNF2_1`"]
    #[inline]
    pub fn is_cnf2_1(&self) -> bool {
        *self == CNF2R::CNF2_1
    }
}
#[doc = r" Value of the field"]
pub struct NUM3R {
    bits: u8,
}
impl NUM3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CNF3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNF3R {
    #[doc = "channel"]
    CNF3_0,
    #[doc = "DMA request"]
    CNF3_1,
}
impl CNF3R {
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
            CNF3R::CNF3_0 => false,
            CNF3R::CNF3_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNF3R {
        match value {
            false => CNF3R::CNF3_0,
            true => CNF3R::CNF3_1,
        }
    }
    #[doc = "Checks if the value of the field is `CNF3_0`"]
    #[inline]
    pub fn is_cnf3_0(&self) -> bool {
        *self == CNF3R::CNF3_0
    }
    #[doc = "Checks if the value of the field is `CNF3_1`"]
    #[inline]
    pub fn is_cnf3_1(&self) -> bool {
        *self == CNF3R::CNF3_1
    }
}
#[doc = r" Proxy"]
pub struct _NUM0W<'a> {
    w: &'a mut W,
}
impl<'a> _NUM0W<'a> {
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
#[doc = "Values that can be written to the field `CNF0`"]
pub enum CNF0W {
    #[doc = "channel"]
    CNF0_0,
    #[doc = "DMA request"]
    CNF0_1,
}
impl CNF0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CNF0W::CNF0_0 => false,
            CNF0W::CNF0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNF0W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "channel"]
    #[inline]
    pub fn cnf0_0(self) -> &'a mut W {
        self.variant(CNF0W::CNF0_0)
    }
    #[doc = "DMA request"]
    #[inline]
    pub fn cnf0_1(self) -> &'a mut W {
        self.variant(CNF0W::CNF0_1)
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
#[doc = r" Proxy"]
pub struct _NUM1W<'a> {
    w: &'a mut W,
}
impl<'a> _NUM1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CNF1`"]
pub enum CNF1W {
    #[doc = "channel"]
    CNF1_0,
    #[doc = "DMA request"]
    CNF1_1,
}
impl CNF1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CNF1W::CNF1_0 => false,
            CNF1W::CNF1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNF1W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "channel"]
    #[inline]
    pub fn cnf1_0(self) -> &'a mut W {
        self.variant(CNF1W::CNF1_0)
    }
    #[doc = "DMA request"]
    #[inline]
    pub fn cnf1_1(self) -> &'a mut W {
        self.variant(CNF1W::CNF1_1)
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
pub struct _NUM2W<'a> {
    w: &'a mut W,
}
impl<'a> _NUM2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CNF2`"]
pub enum CNF2W {
    #[doc = "channel"]
    CNF2_0,
    #[doc = "DMA request"]
    CNF2_1,
}
impl CNF2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CNF2W::CNF2_0 => false,
            CNF2W::CNF2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNF2W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "channel"]
    #[inline]
    pub fn cnf2_0(self) -> &'a mut W {
        self.variant(CNF2W::CNF2_0)
    }
    #[doc = "DMA request"]
    #[inline]
    pub fn cnf2_1(self) -> &'a mut W {
        self.variant(CNF2W::CNF2_1)
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
#[doc = r" Proxy"]
pub struct _NUM3W<'a> {
    w: &'a mut W,
}
impl<'a> _NUM3W<'a> {
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
#[doc = "Values that can be written to the field `CNF3`"]
pub enum CNF3W {
    #[doc = "channel"]
    CNF3_0,
    #[doc = "DMA request"]
    CNF3_1,
}
impl CNF3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CNF3W::CNF3_0 => false,
            CNF3W::CNF3_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNF3W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "channel"]
    #[inline]
    pub fn cnf3_0(self) -> &'a mut W {
        self.variant(CNF3W::CNF3_0)
    }
    #[doc = "DMA request"]
    #[inline]
    pub fn cnf3_1(self) -> &'a mut W {
        self.variant(CNF3W::CNF3_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - Contains the number of the DMA request or channel that triggers the pulse on the cross-trigger event line number i"]
    #[inline]
    pub fn num0(&self) -> NUM0R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NUM0R { bits }
    }
    #[doc = "Bit 6 - Configuration of the SDMA event line number i that is connected to the cross-trigger"]
    #[inline]
    pub fn cnf0(&self) -> CNF0R {
        CNF0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:13 - Contains the number of the DMA request or channel that triggers the pulse on the cross-trigger event line number i"]
    #[inline]
    pub fn num1(&self) -> NUM1R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NUM1R { bits }
    }
    #[doc = "Bit 14 - Configuration of the SDMA event line number i that is connected to the cross-trigger"]
    #[inline]
    pub fn cnf1(&self) -> CNF1R {
        CNF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:21 - Contains the number of the DMA request or channel that triggers the pulse on the cross-trigger event line number i"]
    #[inline]
    pub fn num2(&self) -> NUM2R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NUM2R { bits }
    }
    #[doc = "Bit 22 - Configuration of the SDMA event line number i that is connected to the cross-trigger"]
    #[inline]
    pub fn cnf2(&self) -> CNF2R {
        CNF2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:29 - Contains the number of the DMA request or channel that triggers the pulse on the cross-trigger event line number i"]
    #[inline]
    pub fn num3(&self) -> NUM3R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NUM3R { bits }
    }
    #[doc = "Bit 30 - Configuration of the SDMA event line number i that is connected to the cross-trigger"]
    #[inline]
    pub fn cnf3(&self) -> CNF3R {
        CNF3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:5 - Contains the number of the DMA request or channel that triggers the pulse on the cross-trigger event line number i"]
    #[inline]
    pub fn num0(&mut self) -> _NUM0W {
        _NUM0W { w: self }
    }
    #[doc = "Bit 6 - Configuration of the SDMA event line number i that is connected to the cross-trigger"]
    #[inline]
    pub fn cnf0(&mut self) -> _CNF0W {
        _CNF0W { w: self }
    }
    #[doc = "Bits 8:13 - Contains the number of the DMA request or channel that triggers the pulse on the cross-trigger event line number i"]
    #[inline]
    pub fn num1(&mut self) -> _NUM1W {
        _NUM1W { w: self }
    }
    #[doc = "Bit 14 - Configuration of the SDMA event line number i that is connected to the cross-trigger"]
    #[inline]
    pub fn cnf1(&mut self) -> _CNF1W {
        _CNF1W { w: self }
    }
    #[doc = "Bits 16:21 - Contains the number of the DMA request or channel that triggers the pulse on the cross-trigger event line number i"]
    #[inline]
    pub fn num2(&mut self) -> _NUM2W {
        _NUM2W { w: self }
    }
    #[doc = "Bit 22 - Configuration of the SDMA event line number i that is connected to the cross-trigger"]
    #[inline]
    pub fn cnf2(&mut self) -> _CNF2W {
        _CNF2W { w: self }
    }
    #[doc = "Bits 24:29 - Contains the number of the DMA request or channel that triggers the pulse on the cross-trigger event line number i"]
    #[inline]
    pub fn num3(&mut self) -> _NUM3W {
        _NUM3W { w: self }
    }
    #[doc = "Bit 30 - Configuration of the SDMA event line number i that is connected to the cross-trigger"]
    #[inline]
    pub fn cnf3(&mut self) -> _CNF3W {
        _CNF3W { w: self }
    }
}
