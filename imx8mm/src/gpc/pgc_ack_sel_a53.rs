#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PGC_ACK_SEL_A53 {
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
pub struct A53_C0_PGC_PDN_ACKR {
    bits: bool,
}
impl A53_C0_PGC_PDN_ACKR {
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
pub struct A53_C1_PGC_PDN_ACKR {
    bits: bool,
}
impl A53_C1_PGC_PDN_ACKR {
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
pub struct A53_PLAT_PGC_PDN_ACKR {
    bits: bool,
}
impl A53_PLAT_PGC_PDN_ACKR {
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
pub struct NOC_PGC_PDN_ACKR {
    bits: bool,
}
impl NOC_PGC_PDN_ACKR {
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
pub struct A53_C2_PGC_PDN_ACKR {
    bits: bool,
}
impl A53_C2_PGC_PDN_ACKR {
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
pub struct A53_C3_PGC_PDN_ACKR {
    bits: bool,
}
impl A53_C3_PGC_PDN_ACKR {
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
pub struct A53_PGC_PDN_ACKR {
    bits: bool,
}
impl A53_PGC_PDN_ACKR {
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
pub struct A53_C0_PGC_PUP_ACKR {
    bits: bool,
}
impl A53_C0_PGC_PUP_ACKR {
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
pub struct A53_C1_PGC_PUP_ACKR {
    bits: bool,
}
impl A53_C1_PGC_PUP_ACKR {
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
pub struct A53_PLAT_PGC_PUP_ACKR {
    bits: bool,
}
impl A53_PLAT_PGC_PUP_ACKR {
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
pub struct NOC_PGC_PUP_ACKR {
    bits: bool,
}
impl NOC_PGC_PUP_ACKR {
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
pub struct A53_C2_PGC_PUP_ACKR {
    bits: bool,
}
impl A53_C2_PGC_PUP_ACKR {
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
pub struct A53_C3_PGC_PUP_ACKR {
    bits: bool,
}
impl A53_C3_PGC_PUP_ACKR {
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
pub struct A53_PGC_PUP_ACKR {
    bits: bool,
}
impl A53_PGC_PUP_ACKR {
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
pub struct _A53_C0_PGC_PDN_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _A53_C0_PGC_PDN_ACKW<'a> {
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
pub struct _A53_C1_PGC_PDN_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _A53_C1_PGC_PDN_ACKW<'a> {
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
pub struct _A53_PLAT_PGC_PDN_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _A53_PLAT_PGC_PDN_ACKW<'a> {
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
pub struct _NOC_PGC_PDN_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _NOC_PGC_PDN_ACKW<'a> {
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
pub struct _A53_C2_PGC_PDN_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _A53_C2_PGC_PDN_ACKW<'a> {
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
pub struct _A53_C3_PGC_PDN_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _A53_C3_PGC_PDN_ACKW<'a> {
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
pub struct _A53_PGC_PDN_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _A53_PGC_PDN_ACKW<'a> {
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
pub struct _A53_C0_PGC_PUP_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _A53_C0_PGC_PUP_ACKW<'a> {
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
pub struct _A53_C1_PGC_PUP_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _A53_C1_PGC_PUP_ACKW<'a> {
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
pub struct _A53_PLAT_PGC_PUP_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _A53_PLAT_PGC_PUP_ACKW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NOC_PGC_PUP_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _NOC_PGC_PUP_ACKW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _A53_C2_PGC_PUP_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _A53_C2_PGC_PUP_ACKW<'a> {
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
pub struct _A53_C3_PGC_PUP_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _A53_C3_PGC_PUP_ACKW<'a> {
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
pub struct _A53_PGC_PUP_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _A53_PGC_PUP_ACKW<'a> {
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
    #[doc = "Bit 0 - Select power down acknowledge signal of A53 CORE0 PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn a53_c0_pgc_pdn_ack(&self) -> A53_C0_PGC_PDN_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_C0_PGC_PDN_ACKR { bits }
    }
    #[doc = "Bit 1 - Select power down acknowledge signal of A53 CORE1 PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn a53_c1_pgc_pdn_ack(&self) -> A53_C1_PGC_PDN_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_C1_PGC_PDN_ACKR { bits }
    }
    #[doc = "Bit 2 - Select power down acknowledge signal of A53 PLATFORM PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn a53_plat_pgc_pdn_ack(&self) -> A53_PLAT_PGC_PDN_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_PLAT_PGC_PDN_ACKR { bits }
    }
    #[doc = "Bit 3 - Select power down acknowledge signal of NOC PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn noc_pgc_pdn_ack(&self) -> NOC_PGC_PDN_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NOC_PGC_PDN_ACKR { bits }
    }
    #[doc = "Bit 13 - Select power down acknowledge signal of A53 CORE2 PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn a53_c2_pgc_pdn_ack(&self) -> A53_C2_PGC_PDN_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_C2_PGC_PDN_ACKR { bits }
    }
    #[doc = "Bit 14 - Select power down acknowledge signal of A53 CORE3 PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn a53_c3_pgc_pdn_ack(&self) -> A53_C3_PGC_PDN_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_C3_PGC_PDN_ACKR { bits }
    }
    #[doc = "Bit 15 - Select power down acknowledge signal of A53 (dummy) PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn a53_pgc_pdn_ack(&self) -> A53_PGC_PDN_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_PGC_PDN_ACKR { bits }
    }
    #[doc = "Bit 16 - Select power up acknowledge signal of A53 CORE0 PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn a53_c0_pgc_pup_ack(&self) -> A53_C0_PGC_PUP_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_C0_PGC_PUP_ACKR { bits }
    }
    #[doc = "Bit 17 - Select power up acknowledge signal of A53 CORE1 PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn a53_c1_pgc_pup_ack(&self) -> A53_C1_PGC_PUP_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_C1_PGC_PUP_ACKR { bits }
    }
    #[doc = "Bit 18 - Select power up acknowledge signal of A53 PLATFORM PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn a53_plat_pgc_pup_ack(&self) -> A53_PLAT_PGC_PUP_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_PLAT_PGC_PUP_ACKR { bits }
    }
    #[doc = "Bit 19 - Select power up acknowledge signal of NOC PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn noc_pgc_pup_ack(&self) -> NOC_PGC_PUP_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NOC_PGC_PUP_ACKR { bits }
    }
    #[doc = "Bit 29 - Select power up acknowledge signal of A53 CORE2 PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn a53_c2_pgc_pup_ack(&self) -> A53_C2_PGC_PUP_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_C2_PGC_PUP_ACKR { bits }
    }
    #[doc = "Bit 30 - Select power up acknowledge signal of A53 CORE3 PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn a53_c3_pgc_pup_ack(&self) -> A53_C3_PGC_PUP_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_C3_PGC_PUP_ACKR { bits }
    }
    #[doc = "Bit 31 - Select power up acknowledge signal of A53 (dummy) PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn a53_pgc_pup_ack(&self) -> A53_PGC_PUP_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_PGC_PUP_ACKR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2147516416 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Select power down acknowledge signal of A53 CORE0 PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn a53_c0_pgc_pdn_ack(&mut self) -> _A53_C0_PGC_PDN_ACKW {
        _A53_C0_PGC_PDN_ACKW { w: self }
    }
    #[doc = "Bit 1 - Select power down acknowledge signal of A53 CORE1 PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn a53_c1_pgc_pdn_ack(&mut self) -> _A53_C1_PGC_PDN_ACKW {
        _A53_C1_PGC_PDN_ACKW { w: self }
    }
    #[doc = "Bit 2 - Select power down acknowledge signal of A53 PLATFORM PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn a53_plat_pgc_pdn_ack(&mut self) -> _A53_PLAT_PGC_PDN_ACKW {
        _A53_PLAT_PGC_PDN_ACKW { w: self }
    }
    #[doc = "Bit 3 - Select power down acknowledge signal of NOC PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn noc_pgc_pdn_ack(&mut self) -> _NOC_PGC_PDN_ACKW {
        _NOC_PGC_PDN_ACKW { w: self }
    }
    #[doc = "Bit 13 - Select power down acknowledge signal of A53 CORE2 PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn a53_c2_pgc_pdn_ack(&mut self) -> _A53_C2_PGC_PDN_ACKW {
        _A53_C2_PGC_PDN_ACKW { w: self }
    }
    #[doc = "Bit 14 - Select power down acknowledge signal of A53 CORE3 PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn a53_c3_pgc_pdn_ack(&mut self) -> _A53_C3_PGC_PDN_ACKW {
        _A53_C3_PGC_PDN_ACKW { w: self }
    }
    #[doc = "Bit 15 - Select power down acknowledge signal of A53 (dummy) PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn a53_pgc_pdn_ack(&mut self) -> _A53_PGC_PDN_ACKW {
        _A53_PGC_PDN_ACKW { w: self }
    }
    #[doc = "Bit 16 - Select power up acknowledge signal of A53 CORE0 PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn a53_c0_pgc_pup_ack(&mut self) -> _A53_C0_PGC_PUP_ACKW {
        _A53_C0_PGC_PUP_ACKW { w: self }
    }
    #[doc = "Bit 17 - Select power up acknowledge signal of A53 CORE1 PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn a53_c1_pgc_pup_ack(&mut self) -> _A53_C1_PGC_PUP_ACKW {
        _A53_C1_PGC_PUP_ACKW { w: self }
    }
    #[doc = "Bit 18 - Select power up acknowledge signal of A53 PLATFORM PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn a53_plat_pgc_pup_ack(&mut self) -> _A53_PLAT_PGC_PUP_ACKW {
        _A53_PLAT_PGC_PUP_ACKW { w: self }
    }
    #[doc = "Bit 19 - Select power up acknowledge signal of NOC PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn noc_pgc_pup_ack(&mut self) -> _NOC_PGC_PUP_ACKW {
        _NOC_PGC_PUP_ACKW { w: self }
    }
    #[doc = "Bit 29 - Select power up acknowledge signal of A53 CORE2 PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn a53_c2_pgc_pup_ack(&mut self) -> _A53_C2_PGC_PUP_ACKW {
        _A53_C2_PGC_PUP_ACKW { w: self }
    }
    #[doc = "Bit 30 - Select power up acknowledge signal of A53 CORE3 PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn a53_c3_pgc_pup_ack(&mut self) -> _A53_C3_PGC_PUP_ACKW {
        _A53_C3_PGC_PUP_ACKW { w: self }
    }
    #[doc = "Bit 31 - Select power up acknowledge signal of A53 (dummy) PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn a53_pgc_pup_ack(&mut self) -> _A53_PGC_PUP_ACKW {
        _A53_PGC_PUP_ACKW { w: self }
    }
}
