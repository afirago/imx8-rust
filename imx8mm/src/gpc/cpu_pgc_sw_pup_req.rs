#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CPU_PGC_SW_PUP_REQ {
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
pub struct CORE0_A53_SW_PUP_REQR {
    bits: bool,
}
impl CORE0_A53_SW_PUP_REQR {
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
pub struct CORE1_A53_SW_PUP_REQR {
    bits: bool,
}
impl CORE1_A53_SW_PUP_REQR {
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
pub struct CORE2_A53_SW_PUP_REQR {
    bits: bool,
}
impl CORE2_A53_SW_PUP_REQR {
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
pub struct CORE3_A53_SW_PUP_REQR {
    bits: bool,
}
impl CORE3_A53_SW_PUP_REQR {
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
pub struct SCU_A53_SW_PUP_REQR {
    bits: bool,
}
impl SCU_A53_SW_PUP_REQR {
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
pub struct _CORE0_A53_SW_PUP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _CORE0_A53_SW_PUP_REQW<'a> {
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
pub struct _CORE1_A53_SW_PUP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _CORE1_A53_SW_PUP_REQW<'a> {
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
pub struct _CORE2_A53_SW_PUP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _CORE2_A53_SW_PUP_REQW<'a> {
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
pub struct _CORE3_A53_SW_PUP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _CORE3_A53_SW_PUP_REQW<'a> {
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
pub struct _SCU_A53_SW_PUP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _SCU_A53_SW_PUP_REQW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Software power up trigger for Core0 A53 PGC"]
    #[inline]
    pub fn core0_a53_sw_pup_req(&self) -> CORE0_A53_SW_PUP_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CORE0_A53_SW_PUP_REQR { bits }
    }
    #[doc = "Bit 1 - Software power up trigger for Core1 A53 PGC"]
    #[inline]
    pub fn core1_a53_sw_pup_req(&self) -> CORE1_A53_SW_PUP_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CORE1_A53_SW_PUP_REQR { bits }
    }
    #[doc = "Bit 2 - Software power up trigger for Core2 A53"]
    #[inline]
    pub fn core2_a53_sw_pup_req(&self) -> CORE2_A53_SW_PUP_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CORE2_A53_SW_PUP_REQR { bits }
    }
    #[doc = "Bit 3 - Software power up trigger for Core3 A53 PGC"]
    #[inline]
    pub fn core3_a53_sw_pup_req(&self) -> CORE3_A53_SW_PUP_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CORE3_A53_SW_PUP_REQR { bits }
    }
    #[doc = "Bit 4 - Software power up trigger for SCU A53 PGC"]
    #[inline]
    pub fn scu_a53_sw_pup_req(&self) -> SCU_A53_SW_PUP_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCU_A53_SW_PUP_REQR { bits }
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
    #[doc = "Bit 0 - Software power up trigger for Core0 A53 PGC"]
    #[inline]
    pub fn core0_a53_sw_pup_req(&mut self) -> _CORE0_A53_SW_PUP_REQW {
        _CORE0_A53_SW_PUP_REQW { w: self }
    }
    #[doc = "Bit 1 - Software power up trigger for Core1 A53 PGC"]
    #[inline]
    pub fn core1_a53_sw_pup_req(&mut self) -> _CORE1_A53_SW_PUP_REQW {
        _CORE1_A53_SW_PUP_REQW { w: self }
    }
    #[doc = "Bit 2 - Software power up trigger for Core2 A53"]
    #[inline]
    pub fn core2_a53_sw_pup_req(&mut self) -> _CORE2_A53_SW_PUP_REQW {
        _CORE2_A53_SW_PUP_REQW { w: self }
    }
    #[doc = "Bit 3 - Software power up trigger for Core3 A53 PGC"]
    #[inline]
    pub fn core3_a53_sw_pup_req(&mut self) -> _CORE3_A53_SW_PUP_REQW {
        _CORE3_A53_SW_PUP_REQW { w: self }
    }
    #[doc = "Bit 4 - Software power up trigger for SCU A53 PGC"]
    #[inline]
    pub fn scu_a53_sw_pup_req(&mut self) -> _SCU_A53_SW_PUP_REQW {
        _SCU_A53_SW_PUP_REQW { w: self }
    }
}
