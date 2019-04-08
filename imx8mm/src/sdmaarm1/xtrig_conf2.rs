#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::XTRIG_CONF2 {
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
pub struct NUM4R {
    bits: u8,
}
impl NUM4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CNF4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNF4R {
    #[doc = "channel"]
    CNF4_0,
    #[doc = "DMA request"]
    CNF4_1,
}
impl CNF4R {
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
            CNF4R::CNF4_0 => false,
            CNF4R::CNF4_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNF4R {
        match value {
            false => CNF4R::CNF4_0,
            true => CNF4R::CNF4_1,
        }
    }
    #[doc = "Checks if the value of the field is `CNF4_0`"]
    #[inline]
    pub fn is_cnf4_0(&self) -> bool {
        *self == CNF4R::CNF4_0
    }
    #[doc = "Checks if the value of the field is `CNF4_1`"]
    #[inline]
    pub fn is_cnf4_1(&self) -> bool {
        *self == CNF4R::CNF4_1
    }
}
#[doc = r" Value of the field"]
pub struct NUM5R {
    bits: u8,
}
impl NUM5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CNF5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNF5R {
    #[doc = "channel"]
    CNF5_0,
    #[doc = "DMA request"]
    CNF5_1,
}
impl CNF5R {
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
            CNF5R::CNF5_0 => false,
            CNF5R::CNF5_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNF5R {
        match value {
            false => CNF5R::CNF5_0,
            true => CNF5R::CNF5_1,
        }
    }
    #[doc = "Checks if the value of the field is `CNF5_0`"]
    #[inline]
    pub fn is_cnf5_0(&self) -> bool {
        *self == CNF5R::CNF5_0
    }
    #[doc = "Checks if the value of the field is `CNF5_1`"]
    #[inline]
    pub fn is_cnf5_1(&self) -> bool {
        *self == CNF5R::CNF5_1
    }
}
#[doc = r" Value of the field"]
pub struct NUM6R {
    bits: u8,
}
impl NUM6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CNF6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNF6R {
    #[doc = "channel"]
    CNF6_0,
    #[doc = "DMA request"]
    CNF6_1,
}
impl CNF6R {
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
            CNF6R::CNF6_0 => false,
            CNF6R::CNF6_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNF6R {
        match value {
            false => CNF6R::CNF6_0,
            true => CNF6R::CNF6_1,
        }
    }
    #[doc = "Checks if the value of the field is `CNF6_0`"]
    #[inline]
    pub fn is_cnf6_0(&self) -> bool {
        *self == CNF6R::CNF6_0
    }
    #[doc = "Checks if the value of the field is `CNF6_1`"]
    #[inline]
    pub fn is_cnf6_1(&self) -> bool {
        *self == CNF6R::CNF6_1
    }
}
#[doc = r" Value of the field"]
pub struct NUM7R {
    bits: u8,
}
impl NUM7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CNF7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNF7R {
    #[doc = "channel"]
    CNF7_0,
    #[doc = "DMA request"]
    CNF7_1,
}
impl CNF7R {
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
            CNF7R::CNF7_0 => false,
            CNF7R::CNF7_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNF7R {
        match value {
            false => CNF7R::CNF7_0,
            true => CNF7R::CNF7_1,
        }
    }
    #[doc = "Checks if the value of the field is `CNF7_0`"]
    #[inline]
    pub fn is_cnf7_0(&self) -> bool {
        *self == CNF7R::CNF7_0
    }
    #[doc = "Checks if the value of the field is `CNF7_1`"]
    #[inline]
    pub fn is_cnf7_1(&self) -> bool {
        *self == CNF7R::CNF7_1
    }
}
#[doc = r" Proxy"]
pub struct _NUM4W<'a> {
    w: &'a mut W,
}
impl<'a> _NUM4W<'a> {
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
#[doc = "Values that can be written to the field `CNF4`"]
pub enum CNF4W {
    #[doc = "channel"]
    CNF4_0,
    #[doc = "DMA request"]
    CNF4_1,
}
impl CNF4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CNF4W::CNF4_0 => false,
            CNF4W::CNF4_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNF4W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "channel"]
    #[inline]
    pub fn cnf4_0(self) -> &'a mut W {
        self.variant(CNF4W::CNF4_0)
    }
    #[doc = "DMA request"]
    #[inline]
    pub fn cnf4_1(self) -> &'a mut W {
        self.variant(CNF4W::CNF4_1)
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
pub struct _NUM5W<'a> {
    w: &'a mut W,
}
impl<'a> _NUM5W<'a> {
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
#[doc = "Values that can be written to the field `CNF5`"]
pub enum CNF5W {
    #[doc = "channel"]
    CNF5_0,
    #[doc = "DMA request"]
    CNF5_1,
}
impl CNF5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CNF5W::CNF5_0 => false,
            CNF5W::CNF5_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNF5W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "channel"]
    #[inline]
    pub fn cnf5_0(self) -> &'a mut W {
        self.variant(CNF5W::CNF5_0)
    }
    #[doc = "DMA request"]
    #[inline]
    pub fn cnf5_1(self) -> &'a mut W {
        self.variant(CNF5W::CNF5_1)
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
pub struct _NUM6W<'a> {
    w: &'a mut W,
}
impl<'a> _NUM6W<'a> {
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
#[doc = "Values that can be written to the field `CNF6`"]
pub enum CNF6W {
    #[doc = "channel"]
    CNF6_0,
    #[doc = "DMA request"]
    CNF6_1,
}
impl CNF6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CNF6W::CNF6_0 => false,
            CNF6W::CNF6_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNF6W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "channel"]
    #[inline]
    pub fn cnf6_0(self) -> &'a mut W {
        self.variant(CNF6W::CNF6_0)
    }
    #[doc = "DMA request"]
    #[inline]
    pub fn cnf6_1(self) -> &'a mut W {
        self.variant(CNF6W::CNF6_1)
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
pub struct _NUM7W<'a> {
    w: &'a mut W,
}
impl<'a> _NUM7W<'a> {
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
#[doc = "Values that can be written to the field `CNF7`"]
pub enum CNF7W {
    #[doc = "channel"]
    CNF7_0,
    #[doc = "DMA request"]
    CNF7_1,
}
impl CNF7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CNF7W::CNF7_0 => false,
            CNF7W::CNF7_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNF7W<'a> {
    w: &'a mut W,
}
impl<'a> _CNF7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNF7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "channel"]
    #[inline]
    pub fn cnf7_0(self) -> &'a mut W {
        self.variant(CNF7W::CNF7_0)
    }
    #[doc = "DMA request"]
    #[inline]
    pub fn cnf7_1(self) -> &'a mut W {
        self.variant(CNF7W::CNF7_1)
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
    pub fn num4(&self) -> NUM4R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NUM4R { bits }
    }
    #[doc = "Bit 6 - Configuration of the SDMA event line number i that is connected to the cross-trigger"]
    #[inline]
    pub fn cnf4(&self) -> CNF4R {
        CNF4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:13 - Contains the number of the DMA request or channel that triggers the pulse on the cross-trigger event line number i"]
    #[inline]
    pub fn num5(&self) -> NUM5R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NUM5R { bits }
    }
    #[doc = "Bit 14 - Configuration of the SDMA event line number i that is connected to the cross-trigger"]
    #[inline]
    pub fn cnf5(&self) -> CNF5R {
        CNF5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:21 - Contains the number of the DMA request or channel that triggers the pulse on the cross-trigger event line number i"]
    #[inline]
    pub fn num6(&self) -> NUM6R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NUM6R { bits }
    }
    #[doc = "Bit 22 - Configuration of the SDMA event line number i that is connected to the cross-trigger"]
    #[inline]
    pub fn cnf6(&self) -> CNF6R {
        CNF6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:29 - Contains the number of the DMA request or channel that triggers the pulse on the cross-trigger event line number i"]
    #[inline]
    pub fn num7(&self) -> NUM7R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NUM7R { bits }
    }
    #[doc = "Bit 30 - Configuration of the SDMA event line number i that is connected to the cross-trigger"]
    #[inline]
    pub fn cnf7(&self) -> CNF7R {
        CNF7R::_from({
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
    pub fn num4(&mut self) -> _NUM4W {
        _NUM4W { w: self }
    }
    #[doc = "Bit 6 - Configuration of the SDMA event line number i that is connected to the cross-trigger"]
    #[inline]
    pub fn cnf4(&mut self) -> _CNF4W {
        _CNF4W { w: self }
    }
    #[doc = "Bits 8:13 - Contains the number of the DMA request or channel that triggers the pulse on the cross-trigger event line number i"]
    #[inline]
    pub fn num5(&mut self) -> _NUM5W {
        _NUM5W { w: self }
    }
    #[doc = "Bit 14 - Configuration of the SDMA event line number i that is connected to the cross-trigger"]
    #[inline]
    pub fn cnf5(&mut self) -> _CNF5W {
        _CNF5W { w: self }
    }
    #[doc = "Bits 16:21 - Contains the number of the DMA request or channel that triggers the pulse on the cross-trigger event line number i"]
    #[inline]
    pub fn num6(&mut self) -> _NUM6W {
        _NUM6W { w: self }
    }
    #[doc = "Bit 22 - Configuration of the SDMA event line number i that is connected to the cross-trigger"]
    #[inline]
    pub fn cnf6(&mut self) -> _CNF6W {
        _CNF6W { w: self }
    }
    #[doc = "Bits 24:29 - Contains the number of the DMA request or channel that triggers the pulse on the cross-trigger event line number i"]
    #[inline]
    pub fn num7(&mut self) -> _NUM7W {
        _NUM7W { w: self }
    }
    #[doc = "Bit 30 - Configuration of the SDMA event line number i that is connected to the cross-trigger"]
    #[inline]
    pub fn cnf7(&mut self) -> _CNF7W {
        _CNF7W { w: self }
    }
}
