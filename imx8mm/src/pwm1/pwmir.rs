#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWMIR {
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
#[doc = "Possible values of the field `FIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIER {
    #[doc = "FIFO Empty interrupt disabled"]
    FIE_0,
    #[doc = "FIFO Empty interrupt enabled"]
    FIE_1,
}
impl FIER {
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
            FIER::FIE_0 => false,
            FIER::FIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIER {
        match value {
            false => FIER::FIE_0,
            true => FIER::FIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FIE_0`"]
    #[inline]
    pub fn is_fie_0(&self) -> bool {
        *self == FIER::FIE_0
    }
    #[doc = "Checks if the value of the field is `FIE_1`"]
    #[inline]
    pub fn is_fie_1(&self) -> bool {
        *self == FIER::FIE_1
    }
}
#[doc = "Possible values of the field `RIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIER {
    #[doc = "Roll-over interrupt not enabled"]
    RIE_0,
    #[doc = "Roll-over Interrupt enabled"]
    RIE_1,
}
impl RIER {
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
            RIER::RIE_0 => false,
            RIER::RIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RIER {
        match value {
            false => RIER::RIE_0,
            true => RIER::RIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RIE_0`"]
    #[inline]
    pub fn is_rie_0(&self) -> bool {
        *self == RIER::RIE_0
    }
    #[doc = "Checks if the value of the field is `RIE_1`"]
    #[inline]
    pub fn is_rie_1(&self) -> bool {
        *self == RIER::RIE_1
    }
}
#[doc = "Possible values of the field `CIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIER {
    #[doc = "Compare Interrupt not enabled"]
    CIE_0,
    #[doc = "Compare Interrupt enabled"]
    CIE_1,
}
impl CIER {
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
            CIER::CIE_0 => false,
            CIER::CIE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CIER {
        match value {
            false => CIER::CIE_0,
            true => CIER::CIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CIE_0`"]
    #[inline]
    pub fn is_cie_0(&self) -> bool {
        *self == CIER::CIE_0
    }
    #[doc = "Checks if the value of the field is `CIE_1`"]
    #[inline]
    pub fn is_cie_1(&self) -> bool {
        *self == CIER::CIE_1
    }
}
#[doc = "Values that can be written to the field `FIE`"]
pub enum FIEW {
    #[doc = "FIFO Empty interrupt disabled"]
    FIE_0,
    #[doc = "FIFO Empty interrupt enabled"]
    FIE_1,
}
impl FIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FIEW::FIE_0 => false,
            FIEW::FIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIEW<'a> {
    w: &'a mut W,
}
impl<'a> _FIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FIFO Empty interrupt disabled"]
    #[inline]
    pub fn fie_0(self) -> &'a mut W {
        self.variant(FIEW::FIE_0)
    }
    #[doc = "FIFO Empty interrupt enabled"]
    #[inline]
    pub fn fie_1(self) -> &'a mut W {
        self.variant(FIEW::FIE_1)
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
#[doc = "Values that can be written to the field `RIE`"]
pub enum RIEW {
    #[doc = "Roll-over interrupt not enabled"]
    RIE_0,
    #[doc = "Roll-over Interrupt enabled"]
    RIE_1,
}
impl RIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RIEW::RIE_0 => false,
            RIEW::RIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Roll-over interrupt not enabled"]
    #[inline]
    pub fn rie_0(self) -> &'a mut W {
        self.variant(RIEW::RIE_0)
    }
    #[doc = "Roll-over Interrupt enabled"]
    #[inline]
    pub fn rie_1(self) -> &'a mut W {
        self.variant(RIEW::RIE_1)
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
#[doc = "Values that can be written to the field `CIE`"]
pub enum CIEW {
    #[doc = "Compare Interrupt not enabled"]
    CIE_0,
    #[doc = "Compare Interrupt enabled"]
    CIE_1,
}
impl CIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CIEW::CIE_0 => false,
            CIEW::CIE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CIEW<'a> {
    w: &'a mut W,
}
impl<'a> _CIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Compare Interrupt not enabled"]
    #[inline]
    pub fn cie_0(self) -> &'a mut W {
        self.variant(CIEW::CIE_0)
    }
    #[doc = "Compare Interrupt enabled"]
    #[inline]
    pub fn cie_1(self) -> &'a mut W {
        self.variant(CIEW::CIE_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - FIFO Empty Interrupt Enable. This bit controls the generation of the FIFO Empty interrupt."]
    #[inline]
    pub fn fie(&self) -> FIER {
        FIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Roll-over Interrupt Enable. This bit controls the generation of the Rollover interrupt."]
    #[inline]
    pub fn rie(&self) -> RIER {
        RIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Compare Interrupt Enable. This bit controls the generation of the Compare interrupt."]
    #[inline]
    pub fn cie(&self) -> CIER {
        CIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - FIFO Empty Interrupt Enable. This bit controls the generation of the FIFO Empty interrupt."]
    #[inline]
    pub fn fie(&mut self) -> _FIEW {
        _FIEW { w: self }
    }
    #[doc = "Bit 1 - Roll-over Interrupt Enable. This bit controls the generation of the Rollover interrupt."]
    #[inline]
    pub fn rie(&mut self) -> _RIEW {
        _RIEW { w: self }
    }
    #[doc = "Bit 2 - Compare Interrupt Enable. This bit controls the generation of the Compare interrupt."]
    #[inline]
    pub fn cie(&mut self) -> _CIEW {
        _CIEW { w: self }
    }
}
