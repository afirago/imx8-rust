#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::I2CR {
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
#[doc = "Possible values of the field `TXAK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXAKR {
    #[doc = "An acknowledge signal is sent to the bus at the ninth clock bit after receiving one byte of data."]
    TXAK_0,
    #[doc = "No acknowledge signal response is sent (that is, the acknowledge bit = 1)."]
    TXAK_1,
}
impl TXAKR {
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
            TXAKR::TXAK_0 => false,
            TXAKR::TXAK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXAKR {
        match value {
            false => TXAKR::TXAK_0,
            true => TXAKR::TXAK_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXAK_0`"]
    #[inline]
    pub fn is_txak_0(&self) -> bool {
        *self == TXAKR::TXAK_0
    }
    #[doc = "Checks if the value of the field is `TXAK_1`"]
    #[inline]
    pub fn is_txak_1(&self) -> bool {
        *self == TXAKR::TXAK_1
    }
}
#[doc = "Possible values of the field `MTX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTXR {
    #[doc = "Receive.When a slave is addressed, the software should set MTX according to the slave read/write bit in the I2C status register (I2C_I2SR\\[SRW\\])."]
    MTX_0,
    #[doc = "Transmit.In Master mode, MTX should be set according to the type of transfer required. Therefore, for address cycles, MTX is always 1."]
    MTX_1,
}
impl MTXR {
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
            MTXR::MTX_0 => false,
            MTXR::MTX_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTXR {
        match value {
            false => MTXR::MTX_0,
            true => MTXR::MTX_1,
        }
    }
    #[doc = "Checks if the value of the field is `MTX_0`"]
    #[inline]
    pub fn is_mtx_0(&self) -> bool {
        *self == MTXR::MTX_0
    }
    #[doc = "Checks if the value of the field is `MTX_1`"]
    #[inline]
    pub fn is_mtx_1(&self) -> bool {
        *self == MTXR::MTX_1
    }
}
#[doc = "Possible values of the field `MSTA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTAR {
    #[doc = "Slave mode. Changing MSTA from 1 to 0 generates a Stop and selects Slave mode."]
    MSTA_0,
    #[doc = "Master mode. Changing MSTA from 0 to 1 signals a Start on the bus and selects Master mode."]
    MSTA_1,
}
impl MSTAR {
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
            MSTAR::MSTA_0 => false,
            MSTAR::MSTA_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTAR {
        match value {
            false => MSTAR::MSTA_0,
            true => MSTAR::MSTA_1,
        }
    }
    #[doc = "Checks if the value of the field is `MSTA_0`"]
    #[inline]
    pub fn is_msta_0(&self) -> bool {
        *self == MSTAR::MSTA_0
    }
    #[doc = "Checks if the value of the field is `MSTA_1`"]
    #[inline]
    pub fn is_msta_1(&self) -> bool {
        *self == MSTAR::MSTA_1
    }
}
#[doc = "Possible values of the field `IIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IIENR {
    #[doc = "I2C interrupts are disabled, but the status flag I2C_I2SR\\[IIF\\] continues to be set when an Interrupt condition occurs."]
    IIEN_0,
    #[doc = "I2C interrupts are enabled. An I2C interrupt occurs if I2C_I2SR\\[IIF\\] is also set."]
    IIEN_1,
}
impl IIENR {
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
            IIENR::IIEN_0 => false,
            IIENR::IIEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IIENR {
        match value {
            false => IIENR::IIEN_0,
            true => IIENR::IIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `IIEN_0`"]
    #[inline]
    pub fn is_iien_0(&self) -> bool {
        *self == IIENR::IIEN_0
    }
    #[doc = "Checks if the value of the field is `IIEN_1`"]
    #[inline]
    pub fn is_iien_1(&self) -> bool {
        *self == IIENR::IIEN_1
    }
}
#[doc = "Possible values of the field `IEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IENR {
    #[doc = "The block is disabled, but registers can still be accessed."]
    IEN_0,
    #[doc = "The I2C is enabled. This bit must be set before any other I2C_I2CR bits have an effect."]
    IEN_1,
}
impl IENR {
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
            IENR::IEN_0 => false,
            IENR::IEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IENR {
        match value {
            false => IENR::IEN_0,
            true => IENR::IEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `IEN_0`"]
    #[inline]
    pub fn is_ien_0(&self) -> bool {
        *self == IENR::IEN_0
    }
    #[doc = "Checks if the value of the field is `IEN_1`"]
    #[inline]
    pub fn is_ien_1(&self) -> bool {
        *self == IENR::IEN_1
    }
}
#[doc = "Values that can be written to the field `RSTA`"]
pub enum RSTAW {
    #[doc = "No repeat start"]
    RSTA_0,
    #[doc = "Generates a Repeated Start condition"]
    RSTA_1,
}
impl RSTAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTAW::RSTA_0 => false,
            RSTAW::RSTA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTAW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No repeat start"]
    #[inline]
    pub fn rsta_0(self) -> &'a mut W {
        self.variant(RSTAW::RSTA_0)
    }
    #[doc = "Generates a Repeated Start condition"]
    #[inline]
    pub fn rsta_1(self) -> &'a mut W {
        self.variant(RSTAW::RSTA_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXAK`"]
pub enum TXAKW {
    #[doc = "An acknowledge signal is sent to the bus at the ninth clock bit after receiving one byte of data."]
    TXAK_0,
    #[doc = "No acknowledge signal response is sent (that is, the acknowledge bit = 1)."]
    TXAK_1,
}
impl TXAKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXAKW::TXAK_0 => false,
            TXAKW::TXAK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXAKW<'a> {
    w: &'a mut W,
}
impl<'a> _TXAKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXAKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An acknowledge signal is sent to the bus at the ninth clock bit after receiving one byte of data."]
    #[inline]
    pub fn txak_0(self) -> &'a mut W {
        self.variant(TXAKW::TXAK_0)
    }
    #[doc = "No acknowledge signal response is sent (that is, the acknowledge bit = 1)."]
    #[inline]
    pub fn txak_1(self) -> &'a mut W {
        self.variant(TXAKW::TXAK_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MTX`"]
pub enum MTXW {
    #[doc = "Receive.When a slave is addressed, the software should set MTX according to the slave read/write bit in the I2C status register (I2C_I2SR\\[SRW\\])."]
    MTX_0,
    #[doc = "Transmit.In Master mode, MTX should be set according to the type of transfer required. Therefore, for address cycles, MTX is always 1."]
    MTX_1,
}
impl MTXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTXW::MTX_0 => false,
            MTXW::MTX_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTXW<'a> {
    w: &'a mut W,
}
impl<'a> _MTXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receive.When a slave is addressed, the software should set MTX according to the slave read/write bit in the I2C status register (I2C_I2SR\\[SRW\\])."]
    #[inline]
    pub fn mtx_0(self) -> &'a mut W {
        self.variant(MTXW::MTX_0)
    }
    #[doc = "Transmit.In Master mode, MTX should be set according to the type of transfer required. Therefore, for address cycles, MTX is always 1."]
    #[inline]
    pub fn mtx_1(self) -> &'a mut W {
        self.variant(MTXW::MTX_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MSTA`"]
pub enum MSTAW {
    #[doc = "Slave mode. Changing MSTA from 1 to 0 generates a Stop and selects Slave mode."]
    MSTA_0,
    #[doc = "Master mode. Changing MSTA from 0 to 1 signals a Start on the bus and selects Master mode."]
    MSTA_1,
}
impl MSTAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTAW::MSTA_0 => false,
            MSTAW::MSTA_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTAW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slave mode. Changing MSTA from 1 to 0 generates a Stop and selects Slave mode."]
    #[inline]
    pub fn msta_0(self) -> &'a mut W {
        self.variant(MSTAW::MSTA_0)
    }
    #[doc = "Master mode. Changing MSTA from 0 to 1 signals a Start on the bus and selects Master mode."]
    #[inline]
    pub fn msta_1(self) -> &'a mut W {
        self.variant(MSTAW::MSTA_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IIEN`"]
pub enum IIENW {
    #[doc = "I2C interrupts are disabled, but the status flag I2C_I2SR\\[IIF\\] continues to be set when an Interrupt condition occurs."]
    IIEN_0,
    #[doc = "I2C interrupts are enabled. An I2C interrupt occurs if I2C_I2SR\\[IIF\\] is also set."]
    IIEN_1,
}
impl IIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IIENW::IIEN_0 => false,
            IIENW::IIEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IIENW<'a> {
    w: &'a mut W,
}
impl<'a> _IIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "I2C interrupts are disabled, but the status flag I2C_I2SR\\[IIF\\] continues to be set when an Interrupt condition occurs."]
    #[inline]
    pub fn iien_0(self) -> &'a mut W {
        self.variant(IIENW::IIEN_0)
    }
    #[doc = "I2C interrupts are enabled. An I2C interrupt occurs if I2C_I2SR\\[IIF\\] is also set."]
    #[inline]
    pub fn iien_1(self) -> &'a mut W {
        self.variant(IIENW::IIEN_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IEN`"]
pub enum IENW {
    #[doc = "The block is disabled, but registers can still be accessed."]
    IEN_0,
    #[doc = "The I2C is enabled. This bit must be set before any other I2C_I2CR bits have an effect."]
    IEN_1,
}
impl IENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IENW::IEN_0 => false,
            IENW::IEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IENW<'a> {
    w: &'a mut W,
}
impl<'a> _IENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The block is disabled, but registers can still be accessed."]
    #[inline]
    pub fn ien_0(self) -> &'a mut W {
        self.variant(IENW::IEN_0)
    }
    #[doc = "The I2C is enabled. This bit must be set before any other I2C_I2CR bits have an effect."]
    #[inline]
    pub fn ien_1(self) -> &'a mut W {
        self.variant(IENW::IEN_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 3 - Transmit acknowledge enable"]
    #[inline]
    pub fn txak(&self) -> TXAKR {
        TXAKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Transmit/Receive mode select bit. Selects the direction of master and slave transfers."]
    #[inline]
    pub fn mtx(&self) -> MTXR {
        MTXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Master/Slave mode select bit"]
    #[inline]
    pub fn msta(&self) -> MSTAR {
        MSTAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - I2C interrupt enable"]
    #[inline]
    pub fn iien(&self) -> IIENR {
        IIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - I2C enable"]
    #[inline]
    pub fn ien(&self) -> IENR {
        IENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - Repeat start"]
    #[inline]
    pub fn rsta(&mut self) -> _RSTAW {
        _RSTAW { w: self }
    }
    #[doc = "Bit 3 - Transmit acknowledge enable"]
    #[inline]
    pub fn txak(&mut self) -> _TXAKW {
        _TXAKW { w: self }
    }
    #[doc = "Bit 4 - Transmit/Receive mode select bit. Selects the direction of master and slave transfers."]
    #[inline]
    pub fn mtx(&mut self) -> _MTXW {
        _MTXW { w: self }
    }
    #[doc = "Bit 5 - Master/Slave mode select bit"]
    #[inline]
    pub fn msta(&mut self) -> _MSTAW {
        _MSTAW { w: self }
    }
    #[doc = "Bit 6 - I2C interrupt enable"]
    #[inline]
    pub fn iien(&mut self) -> _IIENW {
        _IIENW { w: self }
    }
    #[doc = "Bit 7 - I2C enable"]
    #[inline]
    pub fn ien(&mut self) -> _IENW {
        _IENW { w: self }
    }
}
