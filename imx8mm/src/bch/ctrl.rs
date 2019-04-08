#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
pub struct COMPLETE_IRQR {
    bits: bool,
}
impl COMPLETE_IRQR {
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
pub struct RSVD0R {
    bits: bool,
}
impl RSVD0R {
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
pub struct DEBUG_STALL_IRQR {
    bits: bool,
}
impl DEBUG_STALL_IRQR {
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
pub struct BM_ERROR_IRQR {
    bits: bool,
}
impl BM_ERROR_IRQR {
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
#[doc = r" Value of the field"]
pub struct COMPLETE_IRQ_ENR {
    bits: bool,
}
impl COMPLETE_IRQ_ENR {
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
pub struct RSVD2R {
    bits: bool,
}
impl RSVD2R {
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
pub struct DEBUG_STALL_IRQ_ENR {
    bits: bool,
}
impl DEBUG_STALL_IRQ_ENR {
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
pub struct RSVD3R {
    bits: u8,
}
impl RSVD3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct M2M_ENABLER {
    bits: bool,
}
impl M2M_ENABLER {
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
pub struct M2M_ENCODER {
    bits: bool,
}
impl M2M_ENCODER {
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
pub struct M2M_LAYOUTR {
    bits: u8,
}
impl M2M_LAYOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RSVD4R {
    bits: u8,
}
impl RSVD4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DEBUGSYNDROMER {
    bits: bool,
}
impl DEBUGSYNDROMER {
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
pub struct RSVD5R {
    bits: u8,
}
impl RSVD5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CLKGATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKGATER {
    #[doc = "Allow BCH to operate normally."]
    RUN,
    #[doc = "Do not clock BCH gates in order to minimize power consumption."]
    NO_CLKS,
}
impl CLKGATER {
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
            CLKGATER::RUN => false,
            CLKGATER::NO_CLKS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKGATER {
        match value {
            false => CLKGATER::RUN,
            true => CLKGATER::NO_CLKS,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == CLKGATER::RUN
    }
    #[doc = "Checks if the value of the field is `NO_CLKS`"]
    #[inline]
    pub fn is_no_clks(&self) -> bool {
        *self == CLKGATER::NO_CLKS
    }
}
#[doc = "Possible values of the field `SFTRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SFTRSTR {
    #[doc = "Allow BCH to operate normally."]
    RUN,
    #[doc = "Hold BCH in reset."]
    RESET,
}
impl SFTRSTR {
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
            SFTRSTR::RUN => false,
            SFTRSTR::RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SFTRSTR {
        match value {
            false => SFTRSTR::RUN,
            true => SFTRSTR::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == SFTRSTR::RUN
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == SFTRSTR::RESET
    }
}
#[doc = r" Proxy"]
pub struct _COMPLETE_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPLETE_IRQW<'a> {
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
#[doc = r" Proxy"]
pub struct _DEBUG_STALL_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBUG_STALL_IRQW<'a> {
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
#[doc = r" Proxy"]
pub struct _BM_ERROR_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _BM_ERROR_IRQW<'a> {
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
pub struct _COMPLETE_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPLETE_IRQ_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _DEBUG_STALL_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBUG_STALL_IRQ_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _M2M_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _M2M_ENABLEW<'a> {
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
pub struct _M2M_ENCODEW<'a> {
    w: &'a mut W,
}
impl<'a> _M2M_ENCODEW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _M2M_LAYOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _M2M_LAYOUTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DEBUGSYNDROMEW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBUGSYNDROMEW<'a> {
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
#[doc = "Values that can be written to the field `CLKGATE`"]
pub enum CLKGATEW {
    #[doc = "Allow BCH to operate normally."]
    RUN,
    #[doc = "Do not clock BCH gates in order to minimize power consumption."]
    NO_CLKS,
}
impl CLKGATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKGATEW::RUN => false,
            CLKGATEW::NO_CLKS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKGATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKGATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allow BCH to operate normally."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(CLKGATEW::RUN)
    }
    #[doc = "Do not clock BCH gates in order to minimize power consumption."]
    #[inline]
    pub fn no_clks(self) -> &'a mut W {
        self.variant(CLKGATEW::NO_CLKS)
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
#[doc = "Values that can be written to the field `SFTRST`"]
pub enum SFTRSTW {
    #[doc = "Allow BCH to operate normally."]
    RUN,
    #[doc = "Hold BCH in reset."]
    RESET,
}
impl SFTRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SFTRSTW::RUN => false,
            SFTRSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SFTRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SFTRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SFTRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allow BCH to operate normally."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(SFTRSTW::RUN)
    }
    #[doc = "Hold BCH in reset."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(SFTRSTW::RESET)
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
    #[doc = "Bit 0 - This bit indicates the state of the external interrupt line"]
    #[inline]
    pub fn complete_irq(&self) -> COMPLETE_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COMPLETE_IRQR { bits }
    }
    #[doc = "Bit 1 - This field is reserved."]
    #[inline]
    pub fn rsvd0(&self) -> RSVD0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSVD0R { bits }
    }
    #[doc = "Bit 2 - DEBUG STALL Interrupt Status. Write a 1 to the SCT clear address to clear the interrupt status bit."]
    #[inline]
    pub fn debug_stall_irq(&self) -> DEBUG_STALL_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DEBUG_STALL_IRQR { bits }
    }
    #[doc = "Bit 3 - AHB Bus interface Error Interrupt Status"]
    #[inline]
    pub fn bm_error_irq(&self) -> BM_ERROR_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BM_ERROR_IRQR { bits }
    }
    #[doc = "Bits 4:7 - This field is reserved."]
    #[inline]
    pub fn rsvd1(&self) -> RSVD1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSVD1R { bits }
    }
    #[doc = "Bit 8 - 1 = interrupt on completion of correction is enabled."]
    #[inline]
    pub fn complete_irq_en(&self) -> COMPLETE_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COMPLETE_IRQ_ENR { bits }
    }
    #[doc = "Bit 9 - This field is reserved."]
    #[inline]
    pub fn rsvd2(&self) -> RSVD2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSVD2R { bits }
    }
    #[doc = "Bit 10 - 1 = interrupt on debug stall mode is enabled. The IRQ is raised on every block"]
    #[inline]
    pub fn debug_stall_irq_en(&self) -> DEBUG_STALL_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DEBUG_STALL_IRQ_ENR { bits }
    }
    #[doc = "Bits 11:15 - This field is reserved."]
    #[inline]
    pub fn rsvd3(&self) -> RSVD3R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSVD3R { bits }
    }
    #[doc = "Bit 16 - NOTE! WRITING THIS BIT INITIATES A MEMORY-TO-MEMORY OPERATION"]
    #[inline]
    pub fn m2m_enable(&self) -> M2M_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        M2M_ENABLER { bits }
    }
    #[doc = "Bit 17 - Selects encode (parity generation) or decode (correction) mode for memory-to-memory operations."]
    #[inline]
    pub fn m2m_encode(&self) -> M2M_ENCODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        M2M_ENCODER { bits }
    }
    #[doc = "Bits 18:19 - Selects the flash page format for memory-to-memory operations."]
    #[inline]
    pub fn m2m_layout(&self) -> M2M_LAYOUTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        M2M_LAYOUTR { bits }
    }
    #[doc = "Bits 20:21 - This field is reserved."]
    #[inline]
    pub fn rsvd4(&self) -> RSVD4R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSVD4R { bits }
    }
    #[doc = "Bit 22 - (For debug purposes only)"]
    #[inline]
    pub fn debugsyndrome(&self) -> DEBUGSYNDROMER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DEBUGSYNDROMER { bits }
    }
    #[doc = "Bits 23:29 - This field is reserved."]
    #[inline]
    pub fn rsvd5(&self) -> RSVD5R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSVD5R { bits }
    }
    #[doc = "Bit 30 - This bit must be set to 0 for normal operation. When set to 1 it gates off the clocks to the block."]
    #[inline]
    pub fn clkgate(&self) -> CLKGATER {
        CLKGATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Set this bit to 0 to enable normal BCH operation"]
    #[inline]
    pub fn sftrst(&self) -> SFTRSTR {
        SFTRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bit 0 - This bit indicates the state of the external interrupt line"]
    #[inline]
    pub fn complete_irq(&mut self) -> _COMPLETE_IRQW {
        _COMPLETE_IRQW { w: self }
    }
    #[doc = "Bit 2 - DEBUG STALL Interrupt Status. Write a 1 to the SCT clear address to clear the interrupt status bit."]
    #[inline]
    pub fn debug_stall_irq(&mut self) -> _DEBUG_STALL_IRQW {
        _DEBUG_STALL_IRQW { w: self }
    }
    #[doc = "Bit 3 - AHB Bus interface Error Interrupt Status"]
    #[inline]
    pub fn bm_error_irq(&mut self) -> _BM_ERROR_IRQW {
        _BM_ERROR_IRQW { w: self }
    }
    #[doc = "Bit 8 - 1 = interrupt on completion of correction is enabled."]
    #[inline]
    pub fn complete_irq_en(&mut self) -> _COMPLETE_IRQ_ENW {
        _COMPLETE_IRQ_ENW { w: self }
    }
    #[doc = "Bit 10 - 1 = interrupt on debug stall mode is enabled. The IRQ is raised on every block"]
    #[inline]
    pub fn debug_stall_irq_en(&mut self) -> _DEBUG_STALL_IRQ_ENW {
        _DEBUG_STALL_IRQ_ENW { w: self }
    }
    #[doc = "Bit 16 - NOTE! WRITING THIS BIT INITIATES A MEMORY-TO-MEMORY OPERATION"]
    #[inline]
    pub fn m2m_enable(&mut self) -> _M2M_ENABLEW {
        _M2M_ENABLEW { w: self }
    }
    #[doc = "Bit 17 - Selects encode (parity generation) or decode (correction) mode for memory-to-memory operations."]
    #[inline]
    pub fn m2m_encode(&mut self) -> _M2M_ENCODEW {
        _M2M_ENCODEW { w: self }
    }
    #[doc = "Bits 18:19 - Selects the flash page format for memory-to-memory operations."]
    #[inline]
    pub fn m2m_layout(&mut self) -> _M2M_LAYOUTW {
        _M2M_LAYOUTW { w: self }
    }
    #[doc = "Bit 22 - (For debug purposes only)"]
    #[inline]
    pub fn debugsyndrome(&mut self) -> _DEBUGSYNDROMEW {
        _DEBUGSYNDROMEW { w: self }
    }
    #[doc = "Bit 30 - This bit must be set to 0 for normal operation. When set to 1 it gates off the clocks to the block."]
    #[inline]
    pub fn clkgate(&mut self) -> _CLKGATEW {
        _CLKGATEW { w: self }
    }
    #[doc = "Bit 31 - Set this bit to 0 to enable normal BCH operation"]
    #[inline]
    pub fn sftrst(&mut self) -> _SFTRSTW {
        _SFTRSTW { w: self }
    }
}
