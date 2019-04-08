#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AS_CTRL {
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
pub struct AS_ENABLER {
    bits: bool,
}
impl AS_ENABLER {
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
pub struct ALPHA_CTRLR {
    bits: u8,
}
impl ALPHA_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ENABLE_COLORKEYR {
    bits: bool,
}
impl ENABLE_COLORKEYR {
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
pub struct FORMATR {
    bits: u8,
}
impl FORMATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ALPHAR {
    bits: u8,
}
impl ALPHAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ROPR {
    bits: u8,
}
impl ROPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ALPHA_INVERTR {
    bits: bool,
}
impl ALPHA_INVERTR {
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
pub struct INPUT_DATA_SWIZZLER {
    bits: u8,
}
impl INPUT_DATA_SWIZZLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PS_DISABLER {
    bits: bool,
}
impl PS_DISABLER {
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
pub struct RVDS1R {
    bits: u8,
}
impl RVDS1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CSI_SYNC_ON_IRQR {
    bits: bool,
}
impl CSI_SYNC_ON_IRQR {
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
pub struct CSI_SYNC_ON_IRQ_ENR {
    bits: bool,
}
impl CSI_SYNC_ON_IRQ_ENR {
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
pub struct CSI_VSYNC_MODER {
    bits: bool,
}
impl CSI_VSYNC_MODER {
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
pub struct CSI_VSYNC_POLR {
    bits: bool,
}
impl CSI_VSYNC_POLR {
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
pub struct CSI_VSYNC_ENABLER {
    bits: bool,
}
impl CSI_VSYNC_ENABLER {
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
pub struct _AS_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _AS_ENABLEW<'a> {
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
pub struct _ALPHA_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _ALPHA_CTRLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_COLORKEYW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_COLORKEYW<'a> {
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
pub struct _FORMATW<'a> {
    w: &'a mut W,
}
impl<'a> _FORMATW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ALPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _ALPHAW<'a> {
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
#[doc = r" Proxy"]
pub struct _ROPW<'a> {
    w: &'a mut W,
}
impl<'a> _ROPW<'a> {
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
pub struct _ALPHA_INVERTW<'a> {
    w: &'a mut W,
}
impl<'a> _ALPHA_INVERTW<'a> {
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
pub struct _INPUT_DATA_SWIZZLEW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUT_DATA_SWIZZLEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PS_DISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _PS_DISABLEW<'a> {
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
pub struct _CSI_SYNC_ON_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CSI_SYNC_ON_IRQW<'a> {
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
pub struct _CSI_SYNC_ON_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CSI_SYNC_ON_IRQ_ENW<'a> {
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
pub struct _CSI_VSYNC_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CSI_VSYNC_MODEW<'a> {
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
pub struct _CSI_VSYNC_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _CSI_VSYNC_POLW<'a> {
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
pub struct _CSI_VSYNC_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _CSI_VSYNC_ENABLEW<'a> {
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
    #[doc = "Bit 0 - When this bit is set by software, the LCDIF will start fetching AS buffer data in bus master mode and combine it with another buffer"]
    #[inline]
    pub fn as_enable(&self) -> AS_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AS_ENABLER { bits }
    }
    #[doc = "Bits 1:2 - Determines how the alpha value is constructed for this alpha surface"]
    #[inline]
    pub fn alpha_ctrl(&self) -> ALPHA_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ALPHA_CTRLR { bits }
    }
    #[doc = "Bit 3 - Indicates that colorkey functionality is enabled for this alpha surface"]
    #[inline]
    pub fn enable_colorkey(&self) -> ENABLE_COLORKEYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_COLORKEYR { bits }
    }
    #[doc = "Bits 4:7 - Indicates the input buffer format for AS"]
    #[inline]
    pub fn format(&self) -> FORMATR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FORMATR { bits }
    }
    #[doc = "Bits 8:15 - Alpha modifier used when the ALPHA_MULTIPLY or ALPHA_OVERRIDE values are programmed in REG_AS_CTRL\\[ALPHA_CTRL\\]"]
    #[inline]
    pub fn alpha(&self) -> ALPHAR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ALPHAR { bits }
    }
    #[doc = "Bits 16:19 - Indicates a raster operation to perform when enabled"]
    #[inline]
    pub fn rop(&self) -> ROPR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ROPR { bits }
    }
    #[doc = "Bit 20 - Setting this bit to logic 0 will not alter the alpha value"]
    #[inline]
    pub fn alpha_invert(&self) -> ALPHA_INVERTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ALPHA_INVERTR { bits }
    }
    #[doc = "Bits 21:22 - This field specifies how to swap the bytes either in the HW_LCDIF_DATA register or those fetched by the AXI master part of LCDIF"]
    #[inline]
    pub fn input_data_swizzle(&self) -> INPUT_DATA_SWIZZLER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INPUT_DATA_SWIZZLER { bits }
    }
    #[doc = "Bit 23 - When this bit is set by software, the LCDIF will disable PS buffer data."]
    #[inline]
    pub fn ps_disable(&self) -> PS_DISABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PS_DISABLER { bits }
    }
    #[doc = "Bits 24:26 - Reserved, always set to zero."]
    #[inline]
    pub fn rvds1(&self) -> RVDS1R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RVDS1R { bits }
    }
    #[doc = "Bit 27 - this bit is set by software to decide which vsync generate mode"]
    #[inline]
    pub fn csi_sync_on_irq(&self) -> CSI_SYNC_ON_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSI_SYNC_ON_IRQR { bits }
    }
    #[doc = "Bit 28 - This bit is set to enable an interrupt when LCDIF lock with CSI vsync input."]
    #[inline]
    pub fn csi_sync_on_irq_en(&self) -> CSI_SYNC_ON_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSI_SYNC_ON_IRQ_ENR { bits }
    }
    #[doc = "Bit 29 - this bit is set by software to decide which vsync generate mode"]
    #[inline]
    pub fn csi_vsync_mode(&self) -> CSI_VSYNC_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSI_VSYNC_MODER { bits }
    }
    #[doc = "Bit 30 - Default 0 active low during VSYNC_PULSE_WIDTH time and will be high during the rest of the VSYNC period"]
    #[inline]
    pub fn csi_vsync_pol(&self) -> CSI_VSYNC_POLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSI_VSYNC_POLR { bits }
    }
    #[doc = "Bit 31 - When this bit is set by software, the LCDIF work as sync mode with CSI input."]
    #[inline]
    pub fn csi_vsync_enable(&self) -> CSI_VSYNC_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSI_VSYNC_ENABLER { bits }
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
    #[doc = "Bit 0 - When this bit is set by software, the LCDIF will start fetching AS buffer data in bus master mode and combine it with another buffer"]
    #[inline]
    pub fn as_enable(&mut self) -> _AS_ENABLEW {
        _AS_ENABLEW { w: self }
    }
    #[doc = "Bits 1:2 - Determines how the alpha value is constructed for this alpha surface"]
    #[inline]
    pub fn alpha_ctrl(&mut self) -> _ALPHA_CTRLW {
        _ALPHA_CTRLW { w: self }
    }
    #[doc = "Bit 3 - Indicates that colorkey functionality is enabled for this alpha surface"]
    #[inline]
    pub fn enable_colorkey(&mut self) -> _ENABLE_COLORKEYW {
        _ENABLE_COLORKEYW { w: self }
    }
    #[doc = "Bits 4:7 - Indicates the input buffer format for AS"]
    #[inline]
    pub fn format(&mut self) -> _FORMATW {
        _FORMATW { w: self }
    }
    #[doc = "Bits 8:15 - Alpha modifier used when the ALPHA_MULTIPLY or ALPHA_OVERRIDE values are programmed in REG_AS_CTRL\\[ALPHA_CTRL\\]"]
    #[inline]
    pub fn alpha(&mut self) -> _ALPHAW {
        _ALPHAW { w: self }
    }
    #[doc = "Bits 16:19 - Indicates a raster operation to perform when enabled"]
    #[inline]
    pub fn rop(&mut self) -> _ROPW {
        _ROPW { w: self }
    }
    #[doc = "Bit 20 - Setting this bit to logic 0 will not alter the alpha value"]
    #[inline]
    pub fn alpha_invert(&mut self) -> _ALPHA_INVERTW {
        _ALPHA_INVERTW { w: self }
    }
    #[doc = "Bits 21:22 - This field specifies how to swap the bytes either in the HW_LCDIF_DATA register or those fetched by the AXI master part of LCDIF"]
    #[inline]
    pub fn input_data_swizzle(&mut self) -> _INPUT_DATA_SWIZZLEW {
        _INPUT_DATA_SWIZZLEW { w: self }
    }
    #[doc = "Bit 23 - When this bit is set by software, the LCDIF will disable PS buffer data."]
    #[inline]
    pub fn ps_disable(&mut self) -> _PS_DISABLEW {
        _PS_DISABLEW { w: self }
    }
    #[doc = "Bit 27 - this bit is set by software to decide which vsync generate mode"]
    #[inline]
    pub fn csi_sync_on_irq(&mut self) -> _CSI_SYNC_ON_IRQW {
        _CSI_SYNC_ON_IRQW { w: self }
    }
    #[doc = "Bit 28 - This bit is set to enable an interrupt when LCDIF lock with CSI vsync input."]
    #[inline]
    pub fn csi_sync_on_irq_en(&mut self) -> _CSI_SYNC_ON_IRQ_ENW {
        _CSI_SYNC_ON_IRQ_ENW { w: self }
    }
    #[doc = "Bit 29 - this bit is set by software to decide which vsync generate mode"]
    #[inline]
    pub fn csi_vsync_mode(&mut self) -> _CSI_VSYNC_MODEW {
        _CSI_VSYNC_MODEW { w: self }
    }
    #[doc = "Bit 30 - Default 0 active low during VSYNC_PULSE_WIDTH time and will be high during the rest of the VSYNC period"]
    #[inline]
    pub fn csi_vsync_pol(&mut self) -> _CSI_VSYNC_POLW {
        _CSI_VSYNC_POLW { w: self }
    }
    #[doc = "Bit 31 - When this bit is set by software, the LCDIF work as sync mode with CSI input."]
    #[inline]
    pub fn csi_vsync_enable(&mut self) -> _CSI_VSYNC_ENABLEW {
        _CSI_VSYNC_ENABLEW { w: self }
    }
}
