#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::QOS {
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
#[doc = "Possible values of the field `TX_SCHEME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_SCHEMER {
    #[doc = "Credit-based scheme"]
    TX_SCHEME_0,
    #[doc = "Round-robin scheme"]
    TX_SCHEME_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TX_SCHEMER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TX_SCHEMER::TX_SCHEME_0 => 0,
            TX_SCHEMER::TX_SCHEME_1 => 1,
            TX_SCHEMER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TX_SCHEMER {
        match value {
            0 => TX_SCHEMER::TX_SCHEME_0,
            1 => TX_SCHEMER::TX_SCHEME_1,
            i => TX_SCHEMER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TX_SCHEME_0`"]
    #[inline]
    pub fn is_tx_scheme_0(&self) -> bool {
        *self == TX_SCHEMER::TX_SCHEME_0
    }
    #[doc = "Checks if the value of the field is `TX_SCHEME_1`"]
    #[inline]
    pub fn is_tx_scheme_1(&self) -> bool {
        *self == TX_SCHEMER::TX_SCHEME_1
    }
}
#[doc = "Possible values of the field `RX_FLUSH0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FLUSH0R {
    #[doc = "Disable"]
    RX_FLUSH0_0,
    #[doc = "Enable"]
    RX_FLUSH0_1,
}
impl RX_FLUSH0R {
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
            RX_FLUSH0R::RX_FLUSH0_0 => false,
            RX_FLUSH0R::RX_FLUSH0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_FLUSH0R {
        match value {
            false => RX_FLUSH0R::RX_FLUSH0_0,
            true => RX_FLUSH0R::RX_FLUSH0_1,
        }
    }
    #[doc = "Checks if the value of the field is `RX_FLUSH0_0`"]
    #[inline]
    pub fn is_rx_flush0_0(&self) -> bool {
        *self == RX_FLUSH0R::RX_FLUSH0_0
    }
    #[doc = "Checks if the value of the field is `RX_FLUSH0_1`"]
    #[inline]
    pub fn is_rx_flush0_1(&self) -> bool {
        *self == RX_FLUSH0R::RX_FLUSH0_1
    }
}
#[doc = "Possible values of the field `RX_FLUSH1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FLUSH1R {
    #[doc = "Disable"]
    RX_FLUSH1_0,
    #[doc = "Enable"]
    RX_FLUSH1_1,
}
impl RX_FLUSH1R {
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
            RX_FLUSH1R::RX_FLUSH1_0 => false,
            RX_FLUSH1R::RX_FLUSH1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_FLUSH1R {
        match value {
            false => RX_FLUSH1R::RX_FLUSH1_0,
            true => RX_FLUSH1R::RX_FLUSH1_1,
        }
    }
    #[doc = "Checks if the value of the field is `RX_FLUSH1_0`"]
    #[inline]
    pub fn is_rx_flush1_0(&self) -> bool {
        *self == RX_FLUSH1R::RX_FLUSH1_0
    }
    #[doc = "Checks if the value of the field is `RX_FLUSH1_1`"]
    #[inline]
    pub fn is_rx_flush1_1(&self) -> bool {
        *self == RX_FLUSH1R::RX_FLUSH1_1
    }
}
#[doc = "Possible values of the field `RX_FLUSH2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FLUSH2R {
    #[doc = "Disable"]
    RX_FLUSH2_0,
    #[doc = "Enable"]
    RX_FLUSH2_1,
}
impl RX_FLUSH2R {
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
            RX_FLUSH2R::RX_FLUSH2_0 => false,
            RX_FLUSH2R::RX_FLUSH2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_FLUSH2R {
        match value {
            false => RX_FLUSH2R::RX_FLUSH2_0,
            true => RX_FLUSH2R::RX_FLUSH2_1,
        }
    }
    #[doc = "Checks if the value of the field is `RX_FLUSH2_0`"]
    #[inline]
    pub fn is_rx_flush2_0(&self) -> bool {
        *self == RX_FLUSH2R::RX_FLUSH2_0
    }
    #[doc = "Checks if the value of the field is `RX_FLUSH2_1`"]
    #[inline]
    pub fn is_rx_flush2_1(&self) -> bool {
        *self == RX_FLUSH2R::RX_FLUSH2_1
    }
}
#[doc = "Values that can be written to the field `TX_SCHEME`"]
pub enum TX_SCHEMEW {
    #[doc = "Credit-based scheme"]
    TX_SCHEME_0,
    #[doc = "Round-robin scheme"]
    TX_SCHEME_1,
}
impl TX_SCHEMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TX_SCHEMEW::TX_SCHEME_0 => 0,
            TX_SCHEMEW::TX_SCHEME_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_SCHEMEW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_SCHEMEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_SCHEMEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Credit-based scheme"]
    #[inline]
    pub fn tx_scheme_0(self) -> &'a mut W {
        self.variant(TX_SCHEMEW::TX_SCHEME_0)
    }
    #[doc = "Round-robin scheme"]
    #[inline]
    pub fn tx_scheme_1(self) -> &'a mut W {
        self.variant(TX_SCHEMEW::TX_SCHEME_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_FLUSH0`"]
pub enum RX_FLUSH0W {
    #[doc = "Disable"]
    RX_FLUSH0_0,
    #[doc = "Enable"]
    RX_FLUSH0_1,
}
impl RX_FLUSH0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_FLUSH0W::RX_FLUSH0_0 => false,
            RX_FLUSH0W::RX_FLUSH0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_FLUSH0W<'a> {
    w: &'a mut W,
}
impl<'a> _RX_FLUSH0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_FLUSH0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn rx_flush0_0(self) -> &'a mut W {
        self.variant(RX_FLUSH0W::RX_FLUSH0_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn rx_flush0_1(self) -> &'a mut W {
        self.variant(RX_FLUSH0W::RX_FLUSH0_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_FLUSH1`"]
pub enum RX_FLUSH1W {
    #[doc = "Disable"]
    RX_FLUSH1_0,
    #[doc = "Enable"]
    RX_FLUSH1_1,
}
impl RX_FLUSH1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_FLUSH1W::RX_FLUSH1_0 => false,
            RX_FLUSH1W::RX_FLUSH1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_FLUSH1W<'a> {
    w: &'a mut W,
}
impl<'a> _RX_FLUSH1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_FLUSH1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn rx_flush1_0(self) -> &'a mut W {
        self.variant(RX_FLUSH1W::RX_FLUSH1_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn rx_flush1_1(self) -> &'a mut W {
        self.variant(RX_FLUSH1W::RX_FLUSH1_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX_FLUSH2`"]
pub enum RX_FLUSH2W {
    #[doc = "Disable"]
    RX_FLUSH2_0,
    #[doc = "Enable"]
    RX_FLUSH2_1,
}
impl RX_FLUSH2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_FLUSH2W::RX_FLUSH2_0 => false,
            RX_FLUSH2W::RX_FLUSH2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_FLUSH2W<'a> {
    w: &'a mut W,
}
impl<'a> _RX_FLUSH2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_FLUSH2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn rx_flush2_0(self) -> &'a mut W {
        self.variant(RX_FLUSH2W::RX_FLUSH2_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn rx_flush2_1(self) -> &'a mut W {
        self.variant(RX_FLUSH2W::RX_FLUSH2_1)
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
        const OFFSET: u8 = 5;
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
    #[doc = "Bits 0:2 - TX scheme configuration"]
    #[inline]
    pub fn tx_scheme(&self) -> TX_SCHEMER {
        TX_SCHEMER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - RX Flush Ring 0"]
    #[inline]
    pub fn rx_flush0(&self) -> RX_FLUSH0R {
        RX_FLUSH0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - RX Flush Ring 1"]
    #[inline]
    pub fn rx_flush1(&self) -> RX_FLUSH1R {
        RX_FLUSH1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - RX Flush Ring 2"]
    #[inline]
    pub fn rx_flush2(&self) -> RX_FLUSH2R {
        RX_FLUSH2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
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
    #[doc = "Bits 0:2 - TX scheme configuration"]
    #[inline]
    pub fn tx_scheme(&mut self) -> _TX_SCHEMEW {
        _TX_SCHEMEW { w: self }
    }
    #[doc = "Bit 3 - RX Flush Ring 0"]
    #[inline]
    pub fn rx_flush0(&mut self) -> _RX_FLUSH0W {
        _RX_FLUSH0W { w: self }
    }
    #[doc = "Bit 4 - RX Flush Ring 1"]
    #[inline]
    pub fn rx_flush1(&mut self) -> _RX_FLUSH1W {
        _RX_FLUSH1W { w: self }
    }
    #[doc = "Bit 5 - RX Flush Ring 2"]
    #[inline]
    pub fn rx_flush2(&mut self) -> _RX_FLUSH2W {
        _RX_FLUSH2W { w: self }
    }
}
