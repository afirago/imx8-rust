#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STROBE_DLL_CTRL {
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
pub struct STROBE_DLL_CTRL_ENABLER {
    bits: bool,
}
impl STROBE_DLL_CTRL_ENABLER {
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
pub struct STROBE_DLL_CTRL_RESETR {
    bits: bool,
}
impl STROBE_DLL_CTRL_RESETR {
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
pub struct STROBE_DLL_CTRL_SLV_FORCE_UPDR {
    bits: bool,
}
impl STROBE_DLL_CTRL_SLV_FORCE_UPDR {
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
pub struct STROBE_DLL_CTRL_SLV_DLY_TARGETR {
    bits: u8,
}
impl STROBE_DLL_CTRL_SLV_DLY_TARGETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct STROBE_DLL_CTRL_GATE_UPDATE_0R {
    bits: bool,
}
impl STROBE_DLL_CTRL_GATE_UPDATE_0R {
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
pub struct STROBE_DLL_CTRL_GATE_UPDATE_1R {
    bits: bool,
}
impl STROBE_DLL_CTRL_GATE_UPDATE_1R {
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
pub struct STROBE_DLL_CTRL_SLV_OVERRIDER {
    bits: bool,
}
impl STROBE_DLL_CTRL_SLV_OVERRIDER {
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
pub struct STROBE_DLL_CTRL_SLV_OVERRIDE_VALR {
    bits: u8,
}
impl STROBE_DLL_CTRL_SLV_OVERRIDE_VALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct STROBE_DLL_CTRL_SLV_UPDATE_INTR {
    bits: u8,
}
impl STROBE_DLL_CTRL_SLV_UPDATE_INTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct STROBE_DLL_CTRL_REF_UPDATE_INTR {
    bits: u8,
}
impl STROBE_DLL_CTRL_REF_UPDATE_INTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _STROBE_DLL_CTRL_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _STROBE_DLL_CTRL_ENABLEW<'a> {
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
pub struct _STROBE_DLL_CTRL_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _STROBE_DLL_CTRL_RESETW<'a> {
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
#[doc = r" Proxy"]
pub struct _STROBE_DLL_CTRL_SLV_FORCE_UPDW<'a> {
    w: &'a mut W,
}
impl<'a> _STROBE_DLL_CTRL_SLV_FORCE_UPDW<'a> {
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
pub struct _STROBE_DLL_CTRL_SLV_DLY_TARGETW<'a> {
    w: &'a mut W,
}
impl<'a> _STROBE_DLL_CTRL_SLV_DLY_TARGETW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STROBE_DLL_CTRL_GATE_UPDATE_0W<'a> {
    w: &'a mut W,
}
impl<'a> _STROBE_DLL_CTRL_GATE_UPDATE_0W<'a> {
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
pub struct _STROBE_DLL_CTRL_GATE_UPDATE_1W<'a> {
    w: &'a mut W,
}
impl<'a> _STROBE_DLL_CTRL_GATE_UPDATE_1W<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STROBE_DLL_CTRL_SLV_OVERRIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _STROBE_DLL_CTRL_SLV_OVERRIDEW<'a> {
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
pub struct _STROBE_DLL_CTRL_SLV_OVERRIDE_VALW<'a> {
    w: &'a mut W,
}
impl<'a> _STROBE_DLL_CTRL_SLV_OVERRIDE_VALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STROBE_DLL_CTRL_SLV_UPDATE_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _STROBE_DLL_CTRL_SLV_UPDATE_INTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STROBE_DLL_CTRL_REF_UPDATE_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _STROBE_DLL_CTRL_REF_UPDATE_INTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
    #[doc = "Bit 0 - Strobe DLL Control Enable"]
    #[inline]
    pub fn strobe_dll_ctrl_enable(&self) -> STROBE_DLL_CTRL_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STROBE_DLL_CTRL_ENABLER { bits }
    }
    #[doc = "Bit 1 - Strobe DLL Control Reset"]
    #[inline]
    pub fn strobe_dll_ctrl_reset(&self) -> STROBE_DLL_CTRL_RESETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STROBE_DLL_CTRL_RESETR { bits }
    }
    #[doc = "Bit 2 - Strobe DLL Control Slave Force Updated"]
    #[inline]
    pub fn strobe_dll_ctrl_slv_force_upd(&self) -> STROBE_DLL_CTRL_SLV_FORCE_UPDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STROBE_DLL_CTRL_SLV_FORCE_UPDR { bits }
    }
    #[doc = "Bits 3:5 - Strobe DLL Control Slave Delay Target"]
    #[inline]
    pub fn strobe_dll_ctrl_slv_dly_target(&self) -> STROBE_DLL_CTRL_SLV_DLY_TARGETR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STROBE_DLL_CTRL_SLV_DLY_TARGETR { bits }
    }
    #[doc = "Bit 6 - Strobe DLL Control Gate Update"]
    #[inline]
    pub fn strobe_dll_ctrl_gate_update_0(&self) -> STROBE_DLL_CTRL_GATE_UPDATE_0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STROBE_DLL_CTRL_GATE_UPDATE_0R { bits }
    }
    #[doc = "Bit 7 - Strobe DLL Control Gate Update"]
    #[inline]
    pub fn strobe_dll_ctrl_gate_update_1(&self) -> STROBE_DLL_CTRL_GATE_UPDATE_1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STROBE_DLL_CTRL_GATE_UPDATE_1R { bits }
    }
    #[doc = "Bit 8 - Strobe DLL Control Slave Override"]
    #[inline]
    pub fn strobe_dll_ctrl_slv_override(&self) -> STROBE_DLL_CTRL_SLV_OVERRIDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STROBE_DLL_CTRL_SLV_OVERRIDER { bits }
    }
    #[doc = "Bits 9:15 - Strobe DLL Control Slave Override Value"]
    #[inline]
    pub fn strobe_dll_ctrl_slv_override_val(&self) -> STROBE_DLL_CTRL_SLV_OVERRIDE_VALR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STROBE_DLL_CTRL_SLV_OVERRIDE_VALR { bits }
    }
    #[doc = "Bits 20:27 - Strobe DLL Control Slave Update Interval"]
    #[inline]
    pub fn strobe_dll_ctrl_slv_update_int(&self) -> STROBE_DLL_CTRL_SLV_UPDATE_INTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STROBE_DLL_CTRL_SLV_UPDATE_INTR { bits }
    }
    #[doc = "Bits 28:31 - Strobe DLL Control Reference Update Interval"]
    #[inline]
    pub fn strobe_dll_ctrl_ref_update_int(&self) -> STROBE_DLL_CTRL_REF_UPDATE_INTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STROBE_DLL_CTRL_REF_UPDATE_INTR { bits }
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
    #[doc = "Bit 0 - Strobe DLL Control Enable"]
    #[inline]
    pub fn strobe_dll_ctrl_enable(&mut self) -> _STROBE_DLL_CTRL_ENABLEW {
        _STROBE_DLL_CTRL_ENABLEW { w: self }
    }
    #[doc = "Bit 1 - Strobe DLL Control Reset"]
    #[inline]
    pub fn strobe_dll_ctrl_reset(&mut self) -> _STROBE_DLL_CTRL_RESETW {
        _STROBE_DLL_CTRL_RESETW { w: self }
    }
    #[doc = "Bit 2 - Strobe DLL Control Slave Force Updated"]
    #[inline]
    pub fn strobe_dll_ctrl_slv_force_upd(&mut self) -> _STROBE_DLL_CTRL_SLV_FORCE_UPDW {
        _STROBE_DLL_CTRL_SLV_FORCE_UPDW { w: self }
    }
    #[doc = "Bits 3:5 - Strobe DLL Control Slave Delay Target"]
    #[inline]
    pub fn strobe_dll_ctrl_slv_dly_target(&mut self) -> _STROBE_DLL_CTRL_SLV_DLY_TARGETW {
        _STROBE_DLL_CTRL_SLV_DLY_TARGETW { w: self }
    }
    #[doc = "Bit 6 - Strobe DLL Control Gate Update"]
    #[inline]
    pub fn strobe_dll_ctrl_gate_update_0(&mut self) -> _STROBE_DLL_CTRL_GATE_UPDATE_0W {
        _STROBE_DLL_CTRL_GATE_UPDATE_0W { w: self }
    }
    #[doc = "Bit 7 - Strobe DLL Control Gate Update"]
    #[inline]
    pub fn strobe_dll_ctrl_gate_update_1(&mut self) -> _STROBE_DLL_CTRL_GATE_UPDATE_1W {
        _STROBE_DLL_CTRL_GATE_UPDATE_1W { w: self }
    }
    #[doc = "Bit 8 - Strobe DLL Control Slave Override"]
    #[inline]
    pub fn strobe_dll_ctrl_slv_override(&mut self) -> _STROBE_DLL_CTRL_SLV_OVERRIDEW {
        _STROBE_DLL_CTRL_SLV_OVERRIDEW { w: self }
    }
    #[doc = "Bits 9:15 - Strobe DLL Control Slave Override Value"]
    #[inline]
    pub fn strobe_dll_ctrl_slv_override_val(&mut self) -> _STROBE_DLL_CTRL_SLV_OVERRIDE_VALW {
        _STROBE_DLL_CTRL_SLV_OVERRIDE_VALW { w: self }
    }
    #[doc = "Bits 20:27 - Strobe DLL Control Slave Update Interval"]
    #[inline]
    pub fn strobe_dll_ctrl_slv_update_int(&mut self) -> _STROBE_DLL_CTRL_SLV_UPDATE_INTW {
        _STROBE_DLL_CTRL_SLV_UPDATE_INTW { w: self }
    }
    #[doc = "Bits 28:31 - Strobe DLL Control Reference Update Interval"]
    #[inline]
    pub fn strobe_dll_ctrl_ref_update_int(&mut self) -> _STROBE_DLL_CTRL_REF_UPDATE_INTW {
        _STROBE_DLL_CTRL_REF_UPDATE_INTW { w: self }
    }
}
