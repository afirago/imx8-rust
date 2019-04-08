#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::UMCR {
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
#[doc = "Possible values of the field `MDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDENR {
    #[doc = "Normal RS-232 or IrDA mode, see for detail."]
    MDEN_0,
    #[doc = "Enable RS-485 mode, see for detail"]
    MDEN_1,
}
impl MDENR {
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
            MDENR::MDEN_0 => false,
            MDENR::MDEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MDENR {
        match value {
            false => MDENR::MDEN_0,
            true => MDENR::MDEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MDEN_0`"]
    #[inline]
    pub fn is_mden_0(&self) -> bool {
        *self == MDENR::MDEN_0
    }
    #[doc = "Checks if the value of the field is `MDEN_1`"]
    #[inline]
    pub fn is_mden_1(&self) -> bool {
        *self == MDENR::MDEN_1
    }
}
#[doc = "Possible values of the field `SLAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAMR {
    #[doc = "Select Normal Address Detect mode"]
    SLAM_0,
    #[doc = "Select Automatic Address Detect mode"]
    SLAM_1,
}
impl SLAMR {
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
            SLAMR::SLAM_0 => false,
            SLAMR::SLAM_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLAMR {
        match value {
            false => SLAMR::SLAM_0,
            true => SLAMR::SLAM_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLAM_0`"]
    #[inline]
    pub fn is_slam_0(&self) -> bool {
        *self == SLAMR::SLAM_0
    }
    #[doc = "Checks if the value of the field is `SLAM_1`"]
    #[inline]
    pub fn is_slam_1(&self) -> bool {
        *self == SLAMR::SLAM_1
    }
}
#[doc = "Possible values of the field `TXB8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXB8R {
    #[doc = "0 will be transmitted as the RS485 9th data bit"]
    TXB8_0,
    #[doc = "1 will be transmitted as the RS485 9th data bit"]
    TXB8_1,
}
impl TXB8R {
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
            TXB8R::TXB8_0 => false,
            TXB8R::TXB8_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXB8R {
        match value {
            false => TXB8R::TXB8_0,
            true => TXB8R::TXB8_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXB8_0`"]
    #[inline]
    pub fn is_txb8_0(&self) -> bool {
        *self == TXB8R::TXB8_0
    }
    #[doc = "Checks if the value of the field is `TXB8_1`"]
    #[inline]
    pub fn is_txb8_1(&self) -> bool {
        *self == TXB8R::TXB8_1
    }
}
#[doc = "Possible values of the field `SADEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SADENR {
    #[doc = "Disable RS-485 Slave Address Detected Interrupt"]
    SADEN_0,
    #[doc = "Enable RS-485 Slave Address Detected Interrupt"]
    SADEN_1,
}
impl SADENR {
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
            SADENR::SADEN_0 => false,
            SADENR::SADEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SADENR {
        match value {
            false => SADENR::SADEN_0,
            true => SADENR::SADEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SADEN_0`"]
    #[inline]
    pub fn is_saden_0(&self) -> bool {
        *self == SADENR::SADEN_0
    }
    #[doc = "Checks if the value of the field is `SADEN_1`"]
    #[inline]
    pub fn is_saden_1(&self) -> bool {
        *self == SADENR::SADEN_1
    }
}
#[doc = r" Value of the field"]
pub struct SLADDRR {
    bits: u8,
}
impl SLADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `MDEN`"]
pub enum MDENW {
    #[doc = "Normal RS-232 or IrDA mode, see for detail."]
    MDEN_0,
    #[doc = "Enable RS-485 mode, see for detail"]
    MDEN_1,
}
impl MDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MDENW::MDEN_0 => false,
            MDENW::MDEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MDENW<'a> {
    w: &'a mut W,
}
impl<'a> _MDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal RS-232 or IrDA mode, see for detail."]
    #[inline]
    pub fn mden_0(self) -> &'a mut W {
        self.variant(MDENW::MDEN_0)
    }
    #[doc = "Enable RS-485 mode, see for detail"]
    #[inline]
    pub fn mden_1(self) -> &'a mut W {
        self.variant(MDENW::MDEN_1)
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
#[doc = "Values that can be written to the field `SLAM`"]
pub enum SLAMW {
    #[doc = "Select Normal Address Detect mode"]
    SLAM_0,
    #[doc = "Select Automatic Address Detect mode"]
    SLAM_1,
}
impl SLAMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLAMW::SLAM_0 => false,
            SLAMW::SLAM_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLAMW<'a> {
    w: &'a mut W,
}
impl<'a> _SLAMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLAMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Select Normal Address Detect mode"]
    #[inline]
    pub fn slam_0(self) -> &'a mut W {
        self.variant(SLAMW::SLAM_0)
    }
    #[doc = "Select Automatic Address Detect mode"]
    #[inline]
    pub fn slam_1(self) -> &'a mut W {
        self.variant(SLAMW::SLAM_1)
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
#[doc = "Values that can be written to the field `TXB8`"]
pub enum TXB8W {
    #[doc = "0 will be transmitted as the RS485 9th data bit"]
    TXB8_0,
    #[doc = "1 will be transmitted as the RS485 9th data bit"]
    TXB8_1,
}
impl TXB8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXB8W::TXB8_0 => false,
            TXB8W::TXB8_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXB8W<'a> {
    w: &'a mut W,
}
impl<'a> _TXB8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXB8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "0 will be transmitted as the RS485 9th data bit"]
    #[inline]
    pub fn txb8_0(self) -> &'a mut W {
        self.variant(TXB8W::TXB8_0)
    }
    #[doc = "1 will be transmitted as the RS485 9th data bit"]
    #[inline]
    pub fn txb8_1(self) -> &'a mut W {
        self.variant(TXB8W::TXB8_1)
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
#[doc = "Values that can be written to the field `SADEN`"]
pub enum SADENW {
    #[doc = "Disable RS-485 Slave Address Detected Interrupt"]
    SADEN_0,
    #[doc = "Enable RS-485 Slave Address Detected Interrupt"]
    SADEN_1,
}
impl SADENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SADENW::SADEN_0 => false,
            SADENW::SADEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SADENW<'a> {
    w: &'a mut W,
}
impl<'a> _SADENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SADENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable RS-485 Slave Address Detected Interrupt"]
    #[inline]
    pub fn saden_0(self) -> &'a mut W {
        self.variant(SADENW::SADEN_0)
    }
    #[doc = "Enable RS-485 Slave Address Detected Interrupt"]
    #[inline]
    pub fn saden_1(self) -> &'a mut W {
        self.variant(SADENW::SADEN_1)
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
#[doc = r" Proxy"]
pub struct _SLADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _SLADDRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - 9-bit data or Multidrop Mode (RS-485) Enable."]
    #[inline]
    pub fn mden(&self) -> MDENR {
        MDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - RS-485 Slave Address Detect Mode Selection."]
    #[inline]
    pub fn slam(&self) -> SLAMR {
        SLAMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Transmit RS-485 bit 8 (the ninth bit or 9th bit)"]
    #[inline]
    pub fn txb8(&self) -> TXB8R {
        TXB8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - RS-485 Slave Address Detected Interrupt Enable."]
    #[inline]
    pub fn saden(&self) -> SADENR {
        SADENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:15 - RS-485 Slave Address Character"]
    #[inline]
    pub fn sladdr(&self) -> SLADDRR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SLADDRR { bits }
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
    #[doc = "Bit 0 - 9-bit data or Multidrop Mode (RS-485) Enable."]
    #[inline]
    pub fn mden(&mut self) -> _MDENW {
        _MDENW { w: self }
    }
    #[doc = "Bit 1 - RS-485 Slave Address Detect Mode Selection."]
    #[inline]
    pub fn slam(&mut self) -> _SLAMW {
        _SLAMW { w: self }
    }
    #[doc = "Bit 2 - Transmit RS-485 bit 8 (the ninth bit or 9th bit)"]
    #[inline]
    pub fn txb8(&mut self) -> _TXB8W {
        _TXB8W { w: self }
    }
    #[doc = "Bit 3 - RS-485 Slave Address Detected Interrupt Enable."]
    #[inline]
    pub fn saden(&mut self) -> _SADENW {
        _SADENW { w: self }
    }
    #[doc = "Bits 8:15 - RS-485 Slave Address Character"]
    #[inline]
    pub fn sladdr(&mut self) -> _SLADDRW {
        _SLADDRW { w: self }
    }
}
