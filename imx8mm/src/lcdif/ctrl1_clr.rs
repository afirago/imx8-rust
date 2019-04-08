#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL1_CLR {
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
#[doc = "Possible values of the field `RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETR {
    #[doc = "LCD_RESET output signal is low."]
    LCDRESET_LOW,
    #[doc = "LCD_RESET output signal is high."]
    LCDRESET_HIGH,
}
impl RESETR {
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
            RESETR::LCDRESET_LOW => false,
            RESETR::LCDRESET_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESETR {
        match value {
            false => RESETR::LCDRESET_LOW,
            true => RESETR::LCDRESET_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LCDRESET_LOW`"]
    #[inline]
    pub fn is_lcdreset_low(&self) -> bool {
        *self == RESETR::LCDRESET_LOW
    }
    #[doc = "Checks if the value of the field is `LCDRESET_HIGH`"]
    #[inline]
    pub fn is_lcdreset_high(&self) -> bool {
        *self == RESETR::LCDRESET_HIGH
    }
}
#[doc = "Possible values of the field `MODE86`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE86R {
    #[doc = "Pins LCD_WR_RWn and LCD_RD_E function as active low WR and active low RD signals respectively."]
    _8080_MODE,
    #[doc = "Pins LCD_WR_RWn and LCD_RD_E function as Read/Write and active high Enable signals respectively."]
    _6800_MODE,
}
impl MODE86R {
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
            MODE86R::_8080_MODE => false,
            MODE86R::_6800_MODE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MODE86R {
        match value {
            false => MODE86R::_8080_MODE,
            true => MODE86R::_6800_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `_8080_MODE`"]
    #[inline]
    pub fn is_8080_mode(&self) -> bool {
        *self == MODE86R::_8080_MODE
    }
    #[doc = "Checks if the value of the field is `_6800_MODE`"]
    #[inline]
    pub fn is_6800_mode(&self) -> bool {
        *self == MODE86R::_6800_MODE
    }
}
#[doc = "Possible values of the field `BUSY_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_ENABLER {
    #[doc = "The busy signal from the LCD controller will be ignored."]
    BUSY_DISABLED,
    #[doc = "Enable the use of the busy signal from the LCD controller."]
    BUSY_ENABLED,
}
impl BUSY_ENABLER {
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
            BUSY_ENABLER::BUSY_DISABLED => false,
            BUSY_ENABLER::BUSY_ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUSY_ENABLER {
        match value {
            false => BUSY_ENABLER::BUSY_DISABLED,
            true => BUSY_ENABLER::BUSY_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY_DISABLED`"]
    #[inline]
    pub fn is_busy_disabled(&self) -> bool {
        *self == BUSY_ENABLER::BUSY_DISABLED
    }
    #[doc = "Checks if the value of the field is `BUSY_ENABLED`"]
    #[inline]
    pub fn is_busy_enabled(&self) -> bool {
        *self == BUSY_ENABLER::BUSY_ENABLED
    }
}
#[doc = "Possible values of the field `VSYNC_EDGE_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VSYNC_EDGE_IRQR {
    #[doc = "No Interrupt Request Pending."]
    NO_REQUEST,
    #[doc = "Interrupt Request Pending."]
    REQUEST,
}
impl VSYNC_EDGE_IRQR {
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
            VSYNC_EDGE_IRQR::NO_REQUEST => false,
            VSYNC_EDGE_IRQR::REQUEST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VSYNC_EDGE_IRQR {
        match value {
            false => VSYNC_EDGE_IRQR::NO_REQUEST,
            true => VSYNC_EDGE_IRQR::REQUEST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_REQUEST`"]
    #[inline]
    pub fn is_no_request(&self) -> bool {
        *self == VSYNC_EDGE_IRQR::NO_REQUEST
    }
    #[doc = "Checks if the value of the field is `REQUEST`"]
    #[inline]
    pub fn is_request(&self) -> bool {
        *self == VSYNC_EDGE_IRQR::REQUEST
    }
}
#[doc = "Possible values of the field `CUR_FRAME_DONE_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CUR_FRAME_DONE_IRQR {
    #[doc = "No Interrupt Request Pending."]
    NO_REQUEST,
    #[doc = "Interrupt Request Pending."]
    REQUEST,
}
impl CUR_FRAME_DONE_IRQR {
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
            CUR_FRAME_DONE_IRQR::NO_REQUEST => false,
            CUR_FRAME_DONE_IRQR::REQUEST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CUR_FRAME_DONE_IRQR {
        match value {
            false => CUR_FRAME_DONE_IRQR::NO_REQUEST,
            true => CUR_FRAME_DONE_IRQR::REQUEST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_REQUEST`"]
    #[inline]
    pub fn is_no_request(&self) -> bool {
        *self == CUR_FRAME_DONE_IRQR::NO_REQUEST
    }
    #[doc = "Checks if the value of the field is `REQUEST`"]
    #[inline]
    pub fn is_request(&self) -> bool {
        *self == CUR_FRAME_DONE_IRQR::REQUEST
    }
}
#[doc = "Possible values of the field `UNDERFLOW_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNDERFLOW_IRQR {
    #[doc = "No Interrupt Request Pending."]
    NO_REQUEST,
    #[doc = "Interrupt Request Pending."]
    REQUEST,
}
impl UNDERFLOW_IRQR {
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
            UNDERFLOW_IRQR::NO_REQUEST => false,
            UNDERFLOW_IRQR::REQUEST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UNDERFLOW_IRQR {
        match value {
            false => UNDERFLOW_IRQR::NO_REQUEST,
            true => UNDERFLOW_IRQR::REQUEST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_REQUEST`"]
    #[inline]
    pub fn is_no_request(&self) -> bool {
        *self == UNDERFLOW_IRQR::NO_REQUEST
    }
    #[doc = "Checks if the value of the field is `REQUEST`"]
    #[inline]
    pub fn is_request(&self) -> bool {
        *self == UNDERFLOW_IRQR::REQUEST
    }
}
#[doc = "Possible values of the field `OVERFLOW_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERFLOW_IRQR {
    #[doc = "No Interrupt Request Pending."]
    NO_REQUEST,
    #[doc = "Interrupt Request Pending."]
    REQUEST,
}
impl OVERFLOW_IRQR {
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
            OVERFLOW_IRQR::NO_REQUEST => false,
            OVERFLOW_IRQR::REQUEST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVERFLOW_IRQR {
        match value {
            false => OVERFLOW_IRQR::NO_REQUEST,
            true => OVERFLOW_IRQR::REQUEST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_REQUEST`"]
    #[inline]
    pub fn is_no_request(&self) -> bool {
        *self == OVERFLOW_IRQR::NO_REQUEST
    }
    #[doc = "Checks if the value of the field is `REQUEST`"]
    #[inline]
    pub fn is_request(&self) -> bool {
        *self == OVERFLOW_IRQR::REQUEST
    }
}
#[doc = r" Value of the field"]
pub struct VSYNC_EDGE_IRQ_ENR {
    bits: bool,
}
impl VSYNC_EDGE_IRQ_ENR {
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
pub struct CUR_FRAME_DONE_IRQ_ENR {
    bits: bool,
}
impl CUR_FRAME_DONE_IRQ_ENR {
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
pub struct UNDERFLOW_IRQ_ENR {
    bits: bool,
}
impl UNDERFLOW_IRQ_ENR {
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
pub struct OVERFLOW_IRQ_ENR {
    bits: bool,
}
impl OVERFLOW_IRQ_ENR {
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
pub struct BYTE_PACKING_FORMATR {
    bits: u8,
}
impl BYTE_PACKING_FORMATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IRQ_ON_ALTERNATE_FIELDSR {
    bits: bool,
}
impl IRQ_ON_ALTERNATE_FIELDSR {
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
pub struct FIFO_CLEARR {
    bits: bool,
}
impl FIFO_CLEARR {
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
pub struct START_INTERLACE_FROM_SECOND_FIELDR {
    bits: bool,
}
impl START_INTERLACE_FROM_SECOND_FIELDR {
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
pub struct INTERLACE_FIELDSR {
    bits: bool,
}
impl INTERLACE_FIELDSR {
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
pub struct RECOVER_ON_UNDERFLOWR {
    bits: bool,
}
impl RECOVER_ON_UNDERFLOWR {
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
#[doc = "Possible values of the field `BM_ERROR_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BM_ERROR_IRQR {
    #[doc = "No Interrupt Request Pending."]
    NO_REQUEST,
    #[doc = "Interrupt Request Pending."]
    REQUEST,
}
impl BM_ERROR_IRQR {
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
            BM_ERROR_IRQR::NO_REQUEST => false,
            BM_ERROR_IRQR::REQUEST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BM_ERROR_IRQR {
        match value {
            false => BM_ERROR_IRQR::NO_REQUEST,
            true => BM_ERROR_IRQR::REQUEST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_REQUEST`"]
    #[inline]
    pub fn is_no_request(&self) -> bool {
        *self == BM_ERROR_IRQR::NO_REQUEST
    }
    #[doc = "Checks if the value of the field is `REQUEST`"]
    #[inline]
    pub fn is_request(&self) -> bool {
        *self == BM_ERROR_IRQR::REQUEST
    }
}
#[doc = r" Value of the field"]
pub struct BM_ERROR_IRQ_ENR {
    bits: bool,
}
impl BM_ERROR_IRQ_ENR {
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
pub struct COMBINE_MPU_WR_STRBR {
    bits: bool,
}
impl COMBINE_MPU_WR_STRBR {
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
#[doc = "Values that can be written to the field `RESET`"]
pub enum RESETW {
    #[doc = "LCD_RESET output signal is low."]
    LCDRESET_LOW,
    #[doc = "LCD_RESET output signal is high."]
    LCDRESET_HIGH,
}
impl RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESETW::LCDRESET_LOW => false,
            RESETW::LCDRESET_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LCD_RESET output signal is low."]
    #[inline]
    pub fn lcdreset_low(self) -> &'a mut W {
        self.variant(RESETW::LCDRESET_LOW)
    }
    #[doc = "LCD_RESET output signal is high."]
    #[inline]
    pub fn lcdreset_high(self) -> &'a mut W {
        self.variant(RESETW::LCDRESET_HIGH)
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
#[doc = "Values that can be written to the field `MODE86`"]
pub enum MODE86W {
    #[doc = "Pins LCD_WR_RWn and LCD_RD_E function as active low WR and active low RD signals respectively."]
    _8080_MODE,
    #[doc = "Pins LCD_WR_RWn and LCD_RD_E function as Read/Write and active high Enable signals respectively."]
    _6800_MODE,
}
impl MODE86W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MODE86W::_8080_MODE => false,
            MODE86W::_6800_MODE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODE86W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE86W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE86W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pins LCD_WR_RWn and LCD_RD_E function as active low WR and active low RD signals respectively."]
    #[inline]
    pub fn _8080_mode(self) -> &'a mut W {
        self.variant(MODE86W::_8080_MODE)
    }
    #[doc = "Pins LCD_WR_RWn and LCD_RD_E function as Read/Write and active high Enable signals respectively."]
    #[inline]
    pub fn _6800_mode(self) -> &'a mut W {
        self.variant(MODE86W::_6800_MODE)
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
#[doc = "Values that can be written to the field `BUSY_ENABLE`"]
pub enum BUSY_ENABLEW {
    #[doc = "The busy signal from the LCD controller will be ignored."]
    BUSY_DISABLED,
    #[doc = "Enable the use of the busy signal from the LCD controller."]
    BUSY_ENABLED,
}
impl BUSY_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUSY_ENABLEW::BUSY_DISABLED => false,
            BUSY_ENABLEW::BUSY_ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUSY_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _BUSY_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUSY_ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The busy signal from the LCD controller will be ignored."]
    #[inline]
    pub fn busy_disabled(self) -> &'a mut W {
        self.variant(BUSY_ENABLEW::BUSY_DISABLED)
    }
    #[doc = "Enable the use of the busy signal from the LCD controller."]
    #[inline]
    pub fn busy_enabled(self) -> &'a mut W {
        self.variant(BUSY_ENABLEW::BUSY_ENABLED)
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
#[doc = "Values that can be written to the field `VSYNC_EDGE_IRQ`"]
pub enum VSYNC_EDGE_IRQW {
    #[doc = "No Interrupt Request Pending."]
    NO_REQUEST,
    #[doc = "Interrupt Request Pending."]
    REQUEST,
}
impl VSYNC_EDGE_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VSYNC_EDGE_IRQW::NO_REQUEST => false,
            VSYNC_EDGE_IRQW::REQUEST => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VSYNC_EDGE_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _VSYNC_EDGE_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VSYNC_EDGE_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Interrupt Request Pending."]
    #[inline]
    pub fn no_request(self) -> &'a mut W {
        self.variant(VSYNC_EDGE_IRQW::NO_REQUEST)
    }
    #[doc = "Interrupt Request Pending."]
    #[inline]
    pub fn request(self) -> &'a mut W {
        self.variant(VSYNC_EDGE_IRQW::REQUEST)
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
#[doc = "Values that can be written to the field `CUR_FRAME_DONE_IRQ`"]
pub enum CUR_FRAME_DONE_IRQW {
    #[doc = "No Interrupt Request Pending."]
    NO_REQUEST,
    #[doc = "Interrupt Request Pending."]
    REQUEST,
}
impl CUR_FRAME_DONE_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CUR_FRAME_DONE_IRQW::NO_REQUEST => false,
            CUR_FRAME_DONE_IRQW::REQUEST => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CUR_FRAME_DONE_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CUR_FRAME_DONE_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CUR_FRAME_DONE_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Interrupt Request Pending."]
    #[inline]
    pub fn no_request(self) -> &'a mut W {
        self.variant(CUR_FRAME_DONE_IRQW::NO_REQUEST)
    }
    #[doc = "Interrupt Request Pending."]
    #[inline]
    pub fn request(self) -> &'a mut W {
        self.variant(CUR_FRAME_DONE_IRQW::REQUEST)
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
#[doc = "Values that can be written to the field `UNDERFLOW_IRQ`"]
pub enum UNDERFLOW_IRQW {
    #[doc = "No Interrupt Request Pending."]
    NO_REQUEST,
    #[doc = "Interrupt Request Pending."]
    REQUEST,
}
impl UNDERFLOW_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UNDERFLOW_IRQW::NO_REQUEST => false,
            UNDERFLOW_IRQW::REQUEST => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UNDERFLOW_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _UNDERFLOW_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UNDERFLOW_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Interrupt Request Pending."]
    #[inline]
    pub fn no_request(self) -> &'a mut W {
        self.variant(UNDERFLOW_IRQW::NO_REQUEST)
    }
    #[doc = "Interrupt Request Pending."]
    #[inline]
    pub fn request(self) -> &'a mut W {
        self.variant(UNDERFLOW_IRQW::REQUEST)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OVERFLOW_IRQ`"]
pub enum OVERFLOW_IRQW {
    #[doc = "No Interrupt Request Pending."]
    NO_REQUEST,
    #[doc = "Interrupt Request Pending."]
    REQUEST,
}
impl OVERFLOW_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OVERFLOW_IRQW::NO_REQUEST => false,
            OVERFLOW_IRQW::REQUEST => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVERFLOW_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERFLOW_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVERFLOW_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Interrupt Request Pending."]
    #[inline]
    pub fn no_request(self) -> &'a mut W {
        self.variant(OVERFLOW_IRQW::NO_REQUEST)
    }
    #[doc = "Interrupt Request Pending."]
    #[inline]
    pub fn request(self) -> &'a mut W {
        self.variant(OVERFLOW_IRQW::REQUEST)
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
pub struct _VSYNC_EDGE_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _VSYNC_EDGE_IRQ_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _CUR_FRAME_DONE_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CUR_FRAME_DONE_IRQ_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _UNDERFLOW_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _UNDERFLOW_IRQ_ENW<'a> {
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
pub struct _OVERFLOW_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERFLOW_IRQ_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _BYTE_PACKING_FORMATW<'a> {
    w: &'a mut W,
}
impl<'a> _BYTE_PACKING_FORMATW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IRQ_ON_ALTERNATE_FIELDSW<'a> {
    w: &'a mut W,
}
impl<'a> _IRQ_ON_ALTERNATE_FIELDSW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FIFO_CLEARW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFO_CLEARW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _START_INTERLACE_FROM_SECOND_FIELDW<'a> {
    w: &'a mut W,
}
impl<'a> _START_INTERLACE_FROM_SECOND_FIELDW<'a> {
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
pub struct _INTERLACE_FIELDSW<'a> {
    w: &'a mut W,
}
impl<'a> _INTERLACE_FIELDSW<'a> {
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
#[doc = r" Proxy"]
pub struct _RECOVER_ON_UNDERFLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _RECOVER_ON_UNDERFLOWW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BM_ERROR_IRQ`"]
pub enum BM_ERROR_IRQW {
    #[doc = "No Interrupt Request Pending."]
    NO_REQUEST,
    #[doc = "Interrupt Request Pending."]
    REQUEST,
}
impl BM_ERROR_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BM_ERROR_IRQW::NO_REQUEST => false,
            BM_ERROR_IRQW::REQUEST => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BM_ERROR_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _BM_ERROR_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BM_ERROR_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Interrupt Request Pending."]
    #[inline]
    pub fn no_request(self) -> &'a mut W {
        self.variant(BM_ERROR_IRQW::NO_REQUEST)
    }
    #[doc = "Interrupt Request Pending."]
    #[inline]
    pub fn request(self) -> &'a mut W {
        self.variant(BM_ERROR_IRQW::REQUEST)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BM_ERROR_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BM_ERROR_IRQ_ENW<'a> {
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
pub struct _COMBINE_MPU_WR_STRBW<'a> {
    w: &'a mut W,
}
impl<'a> _COMBINE_MPU_WR_STRBW<'a> {
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
    #[doc = "Bit 0 - Reset bit for the external LCD controller"]
    #[inline]
    pub fn reset(&self) -> RESETR {
        RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - This bit is used to select between the 8080 and 6800 series of microprocessor modes"]
    #[inline]
    pub fn mode86(&self) -> MODE86R {
        MODE86R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - This bit enables the use of the interface's busy signal input"]
    #[inline]
    pub fn busy_enable(&self) -> BUSY_ENABLER {
        BUSY_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline]
    pub fn vsync_edge_irq(&self) -> VSYNC_EDGE_IRQR {
        VSYNC_EDGE_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline]
    pub fn cur_frame_done_irq(&self) -> CUR_FRAME_DONE_IRQR {
        CUR_FRAME_DONE_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline]
    pub fn underflow_irq(&self) -> UNDERFLOW_IRQR {
        UNDERFLOW_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline]
    pub fn overflow_irq(&self) -> OVERFLOW_IRQR {
        OVERFLOW_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - This bit is set to enable an interrupt every time the hardware encounters the leading VSYNC edge in the VSYNC and DOTCLK modes, or the beginning of every field in DVI mode"]
    #[inline]
    pub fn vsync_edge_irq_en(&self) -> VSYNC_EDGE_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VSYNC_EDGE_IRQ_ENR { bits }
    }
    #[doc = "Bit 13 - This bit is set to 1 enable an interrupt every time the hardware enters in the vertical blanking state"]
    #[inline]
    pub fn cur_frame_done_irq_en(&self) -> CUR_FRAME_DONE_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CUR_FRAME_DONE_IRQ_ENR { bits }
    }
    #[doc = "Bit 14 - This bit is set to enable an underflow interrupt in the TXFIFO in the write mode."]
    #[inline]
    pub fn underflow_irq_en(&self) -> UNDERFLOW_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UNDERFLOW_IRQ_ENR { bits }
    }
    #[doc = "Bit 15 - This bit is set to enable an overflow interrupt in the TXFIFO in the write mode."]
    #[inline]
    pub fn overflow_irq_en(&self) -> OVERFLOW_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OVERFLOW_IRQ_ENR { bits }
    }
    #[doc = "Bits 16:19 - This bitfield is used to show which data bytes in a 32-bit word are valid"]
    #[inline]
    pub fn byte_packing_format(&self) -> BYTE_PACKING_FORMATR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BYTE_PACKING_FORMATR { bits }
    }
    #[doc = "Bit 20 - If this bit is set, the LCDIF block will assert the cur_frame_done interrupt only on alternate fields, otherwise it will issue the interrupt on both odd and even field"]
    #[inline]
    pub fn irq_on_alternate_fields(&self) -> IRQ_ON_ALTERNATE_FIELDSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IRQ_ON_ALTERNATE_FIELDSR { bits }
    }
    #[doc = "Bit 21 - Set this bit to clear all the data in the latency FIFO (LFIFO), TXFIFO and the RXFIFO."]
    #[inline]
    pub fn fifo_clear(&self) -> FIFO_CLEARR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FIFO_CLEARR { bits }
    }
    #[doc = "Bit 22 - The default is to grab the odd lines first and then the even lines"]
    #[inline]
    pub fn start_interlace_from_second_field(&self) -> START_INTERLACE_FROM_SECOND_FIELDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        START_INTERLACE_FROM_SECOND_FIELDR { bits }
    }
    #[doc = "Bit 23 - Set this bit if it is required that the LCDIF block fetches odd lines in one field and even lines in the other field"]
    #[inline]
    pub fn interlace_fields(&self) -> INTERLACE_FIELDSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTERLACE_FIELDSR { bits }
    }
    #[doc = "Bit 24 - Set this bit to enable the LCDIF block to recover in the next field/frame if there was an underflow in the current field/frame"]
    #[inline]
    pub fn recover_on_underflow(&self) -> RECOVER_ON_UNDERFLOWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RECOVER_ON_UNDERFLOWR { bits }
    }
    #[doc = "Bit 25 - This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline]
    pub fn bm_error_irq(&self) -> BM_ERROR_IRQR {
        BM_ERROR_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - This bit is set to enable bus master error interrupt in the LCDIF master mode."]
    #[inline]
    pub fn bm_error_irq_en(&self) -> BM_ERROR_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BM_ERROR_IRQ_ENR { bits }
    }
    #[doc = "Bit 27 - If this bit is not set, the write strobe will be driven on LCD_WR_RWn pin in the 8080 mode and on the LCD_RD_E pin in the 6800 mode"]
    #[inline]
    pub fn combine_mpu_wr_strb(&self) -> COMBINE_MPU_WR_STRBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COMBINE_MPU_WR_STRBR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 983040 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Reset bit for the external LCD controller"]
    #[inline]
    pub fn reset(&mut self) -> _RESETW {
        _RESETW { w: self }
    }
    #[doc = "Bit 1 - This bit is used to select between the 8080 and 6800 series of microprocessor modes"]
    #[inline]
    pub fn mode86(&mut self) -> _MODE86W {
        _MODE86W { w: self }
    }
    #[doc = "Bit 2 - This bit enables the use of the interface's busy signal input"]
    #[inline]
    pub fn busy_enable(&mut self) -> _BUSY_ENABLEW {
        _BUSY_ENABLEW { w: self }
    }
    #[doc = "Bit 8 - This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline]
    pub fn vsync_edge_irq(&mut self) -> _VSYNC_EDGE_IRQW {
        _VSYNC_EDGE_IRQW { w: self }
    }
    #[doc = "Bit 9 - This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline]
    pub fn cur_frame_done_irq(&mut self) -> _CUR_FRAME_DONE_IRQW {
        _CUR_FRAME_DONE_IRQW { w: self }
    }
    #[doc = "Bit 10 - This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline]
    pub fn underflow_irq(&mut self) -> _UNDERFLOW_IRQW {
        _UNDERFLOW_IRQW { w: self }
    }
    #[doc = "Bit 11 - This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline]
    pub fn overflow_irq(&mut self) -> _OVERFLOW_IRQW {
        _OVERFLOW_IRQW { w: self }
    }
    #[doc = "Bit 12 - This bit is set to enable an interrupt every time the hardware encounters the leading VSYNC edge in the VSYNC and DOTCLK modes, or the beginning of every field in DVI mode"]
    #[inline]
    pub fn vsync_edge_irq_en(&mut self) -> _VSYNC_EDGE_IRQ_ENW {
        _VSYNC_EDGE_IRQ_ENW { w: self }
    }
    #[doc = "Bit 13 - This bit is set to 1 enable an interrupt every time the hardware enters in the vertical blanking state"]
    #[inline]
    pub fn cur_frame_done_irq_en(&mut self) -> _CUR_FRAME_DONE_IRQ_ENW {
        _CUR_FRAME_DONE_IRQ_ENW { w: self }
    }
    #[doc = "Bit 14 - This bit is set to enable an underflow interrupt in the TXFIFO in the write mode."]
    #[inline]
    pub fn underflow_irq_en(&mut self) -> _UNDERFLOW_IRQ_ENW {
        _UNDERFLOW_IRQ_ENW { w: self }
    }
    #[doc = "Bit 15 - This bit is set to enable an overflow interrupt in the TXFIFO in the write mode."]
    #[inline]
    pub fn overflow_irq_en(&mut self) -> _OVERFLOW_IRQ_ENW {
        _OVERFLOW_IRQ_ENW { w: self }
    }
    #[doc = "Bits 16:19 - This bitfield is used to show which data bytes in a 32-bit word are valid"]
    #[inline]
    pub fn byte_packing_format(&mut self) -> _BYTE_PACKING_FORMATW {
        _BYTE_PACKING_FORMATW { w: self }
    }
    #[doc = "Bit 20 - If this bit is set, the LCDIF block will assert the cur_frame_done interrupt only on alternate fields, otherwise it will issue the interrupt on both odd and even field"]
    #[inline]
    pub fn irq_on_alternate_fields(&mut self) -> _IRQ_ON_ALTERNATE_FIELDSW {
        _IRQ_ON_ALTERNATE_FIELDSW { w: self }
    }
    #[doc = "Bit 21 - Set this bit to clear all the data in the latency FIFO (LFIFO), TXFIFO and the RXFIFO."]
    #[inline]
    pub fn fifo_clear(&mut self) -> _FIFO_CLEARW {
        _FIFO_CLEARW { w: self }
    }
    #[doc = "Bit 22 - The default is to grab the odd lines first and then the even lines"]
    #[inline]
    pub fn start_interlace_from_second_field(&mut self) -> _START_INTERLACE_FROM_SECOND_FIELDW {
        _START_INTERLACE_FROM_SECOND_FIELDW { w: self }
    }
    #[doc = "Bit 23 - Set this bit if it is required that the LCDIF block fetches odd lines in one field and even lines in the other field"]
    #[inline]
    pub fn interlace_fields(&mut self) -> _INTERLACE_FIELDSW {
        _INTERLACE_FIELDSW { w: self }
    }
    #[doc = "Bit 24 - Set this bit to enable the LCDIF block to recover in the next field/frame if there was an underflow in the current field/frame"]
    #[inline]
    pub fn recover_on_underflow(&mut self) -> _RECOVER_ON_UNDERFLOWW {
        _RECOVER_ON_UNDERFLOWW { w: self }
    }
    #[doc = "Bit 25 - This bit is set to indicate that an interrupt is requested by the LCDIF block"]
    #[inline]
    pub fn bm_error_irq(&mut self) -> _BM_ERROR_IRQW {
        _BM_ERROR_IRQW { w: self }
    }
    #[doc = "Bit 26 - This bit is set to enable bus master error interrupt in the LCDIF master mode."]
    #[inline]
    pub fn bm_error_irq_en(&mut self) -> _BM_ERROR_IRQ_ENW {
        _BM_ERROR_IRQ_ENW { w: self }
    }
    #[doc = "Bit 27 - If this bit is not set, the write strobe will be driven on LCD_WR_RWn pin in the 8080 mode and on the LCD_RD_E pin in the 6800 mode"]
    #[inline]
    pub fn combine_mpu_wr_strb(&mut self) -> _COMBINE_MPU_WR_STRBW {
        _COMBINE_MPU_WR_STRBW { w: self }
    }
}
