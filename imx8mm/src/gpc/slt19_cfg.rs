#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SLT19_CFG {
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
pub struct CORE0_A53_PDN_SLOT_CONTROLR {
    bits: bool,
}
impl CORE0_A53_PDN_SLOT_CONTROLR {
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
pub struct CORE0_A53_PUP_SLOT_CONTROLR {
    bits: bool,
}
impl CORE0_A53_PUP_SLOT_CONTROLR {
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
pub struct CORE1_A53_PDN_SLOT_CONTROLR {
    bits: bool,
}
impl CORE1_A53_PDN_SLOT_CONTROLR {
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
pub struct CORE1_A53_PUP_SLOT_CONTROLR {
    bits: bool,
}
impl CORE1_A53_PUP_SLOT_CONTROLR {
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
pub struct CORE2_A53_PDN_SLOT_CONTROLR {
    bits: bool,
}
impl CORE2_A53_PDN_SLOT_CONTROLR {
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
pub struct CORE2_A53_PUP_SLOT_CONTROLR {
    bits: bool,
}
impl CORE2_A53_PUP_SLOT_CONTROLR {
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
pub struct CORE3_A53_PDN_SLOT_CONTROLR {
    bits: bool,
}
impl CORE3_A53_PDN_SLOT_CONTROLR {
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
pub struct CORE3_A53_PUP_SLOT_CONTROLR {
    bits: bool,
}
impl CORE3_A53_PUP_SLOT_CONTROLR {
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
pub struct SCU_PDN_SLOT_CONTROLR {
    bits: bool,
}
impl SCU_PDN_SLOT_CONTROLR {
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
pub struct SCU_PUP_SLOT_CONTROLR {
    bits: bool,
}
impl SCU_PUP_SLOT_CONTROLR {
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
pub struct NOC_PDN_SLOT_CONTROLR {
    bits: bool,
}
impl NOC_PDN_SLOT_CONTROLR {
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
pub struct NOC_PUP_SLOT_CONTROLR {
    bits: bool,
}
impl NOC_PUP_SLOT_CONTROLR {
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
pub struct _CORE0_A53_PDN_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _CORE0_A53_PDN_SLOT_CONTROLW<'a> {
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
pub struct _CORE0_A53_PUP_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _CORE0_A53_PUP_SLOT_CONTROLW<'a> {
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
pub struct _CORE1_A53_PDN_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _CORE1_A53_PDN_SLOT_CONTROLW<'a> {
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
pub struct _CORE1_A53_PUP_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _CORE1_A53_PUP_SLOT_CONTROLW<'a> {
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
pub struct _CORE2_A53_PDN_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _CORE2_A53_PDN_SLOT_CONTROLW<'a> {
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
#[doc = r" Proxy"]
pub struct _CORE2_A53_PUP_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _CORE2_A53_PUP_SLOT_CONTROLW<'a> {
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
#[doc = r" Proxy"]
pub struct _CORE3_A53_PDN_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _CORE3_A53_PDN_SLOT_CONTROLW<'a> {
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
pub struct _CORE3_A53_PUP_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _CORE3_A53_PUP_SLOT_CONTROLW<'a> {
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
pub struct _SCU_PDN_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _SCU_PDN_SLOT_CONTROLW<'a> {
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
pub struct _SCU_PUP_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _SCU_PUP_SLOT_CONTROLW<'a> {
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
#[doc = r" Proxy"]
pub struct _NOC_PDN_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _NOC_PDN_SLOT_CONTROLW<'a> {
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
pub struct _NOC_PUP_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _NOC_PUP_SLOT_CONTROLW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - CORE0 A53 Power-down slot control"]
    #[inline]
    pub fn core0_a53_pdn_slot_control(&self) -> CORE0_A53_PDN_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CORE0_A53_PDN_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 1 - CORE0 A53 Power-up slot control"]
    #[inline]
    pub fn core0_a53_pup_slot_control(&self) -> CORE0_A53_PUP_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CORE0_A53_PUP_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 2 - CORE1 A53 Power-down slot control"]
    #[inline]
    pub fn core1_a53_pdn_slot_control(&self) -> CORE1_A53_PDN_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CORE1_A53_PDN_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 3 - CORE1 A53 Power-up slot control"]
    #[inline]
    pub fn core1_a53_pup_slot_control(&self) -> CORE1_A53_PUP_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CORE1_A53_PUP_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 4 - CORE2 A53 Power-down slot control"]
    #[inline]
    pub fn core2_a53_pdn_slot_control(&self) -> CORE2_A53_PDN_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CORE2_A53_PDN_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 5 - CORE2 A53 Power-up slot control"]
    #[inline]
    pub fn core2_a53_pup_slot_control(&self) -> CORE2_A53_PUP_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CORE2_A53_PUP_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 6 - CORE3 A53 Power-down slot control"]
    #[inline]
    pub fn core3_a53_pdn_slot_control(&self) -> CORE3_A53_PDN_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CORE3_A53_PDN_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 7 - CORE3 A53 Power-up slot control"]
    #[inline]
    pub fn core3_a53_pup_slot_control(&self) -> CORE3_A53_PUP_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CORE3_A53_PUP_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 8 - SCU Power-down slot control"]
    #[inline]
    pub fn scu_pdn_slot_control(&self) -> SCU_PDN_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCU_PDN_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 9 - SCU Power-up slot control"]
    #[inline]
    pub fn scu_pup_slot_control(&self) -> SCU_PUP_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCU_PUP_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 10 - NOC Power-down slot control"]
    #[inline]
    pub fn noc_pdn_slot_control(&self) -> NOC_PDN_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NOC_PDN_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 11 - NOC Power-up slot control"]
    #[inline]
    pub fn noc_pup_slot_control(&self) -> NOC_PUP_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NOC_PUP_SLOT_CONTROLR { bits }
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
    #[doc = "Bit 0 - CORE0 A53 Power-down slot control"]
    #[inline]
    pub fn core0_a53_pdn_slot_control(&mut self) -> _CORE0_A53_PDN_SLOT_CONTROLW {
        _CORE0_A53_PDN_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 1 - CORE0 A53 Power-up slot control"]
    #[inline]
    pub fn core0_a53_pup_slot_control(&mut self) -> _CORE0_A53_PUP_SLOT_CONTROLW {
        _CORE0_A53_PUP_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 2 - CORE1 A53 Power-down slot control"]
    #[inline]
    pub fn core1_a53_pdn_slot_control(&mut self) -> _CORE1_A53_PDN_SLOT_CONTROLW {
        _CORE1_A53_PDN_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 3 - CORE1 A53 Power-up slot control"]
    #[inline]
    pub fn core1_a53_pup_slot_control(&mut self) -> _CORE1_A53_PUP_SLOT_CONTROLW {
        _CORE1_A53_PUP_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 4 - CORE2 A53 Power-down slot control"]
    #[inline]
    pub fn core2_a53_pdn_slot_control(&mut self) -> _CORE2_A53_PDN_SLOT_CONTROLW {
        _CORE2_A53_PDN_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 5 - CORE2 A53 Power-up slot control"]
    #[inline]
    pub fn core2_a53_pup_slot_control(&mut self) -> _CORE2_A53_PUP_SLOT_CONTROLW {
        _CORE2_A53_PUP_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 6 - CORE3 A53 Power-down slot control"]
    #[inline]
    pub fn core3_a53_pdn_slot_control(&mut self) -> _CORE3_A53_PDN_SLOT_CONTROLW {
        _CORE3_A53_PDN_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 7 - CORE3 A53 Power-up slot control"]
    #[inline]
    pub fn core3_a53_pup_slot_control(&mut self) -> _CORE3_A53_PUP_SLOT_CONTROLW {
        _CORE3_A53_PUP_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 8 - SCU Power-down slot control"]
    #[inline]
    pub fn scu_pdn_slot_control(&mut self) -> _SCU_PDN_SLOT_CONTROLW {
        _SCU_PDN_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 9 - SCU Power-up slot control"]
    #[inline]
    pub fn scu_pup_slot_control(&mut self) -> _SCU_PUP_SLOT_CONTROLW {
        _SCU_PUP_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 10 - NOC Power-down slot control"]
    #[inline]
    pub fn noc_pdn_slot_control(&mut self) -> _NOC_PDN_SLOT_CONTROLW {
        _NOC_PDN_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 11 - NOC Power-up slot control"]
    #[inline]
    pub fn noc_pup_slot_control(&mut self) -> _NOC_PUP_SLOT_CONTROLW {
        _NOC_PUP_SLOT_CONTROLW { w: self }
    }
}
