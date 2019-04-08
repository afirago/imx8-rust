#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PU_PWRHSK {
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
pub struct GPC_DDR1_CORE_CSYSREQR {
    bits: bool,
}
impl GPC_DDR1_CORE_CSYSREQR {
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
pub struct GPC_DDR1_AXI_CSYSREQR {
    bits: bool,
}
impl GPC_DDR1_AXI_CSYSREQR {
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
pub struct GPC_NOC2DDR_PWRDNREQNR {
    bits: bool,
}
impl GPC_NOC2DDR_PWRDNREQNR {
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
pub struct GPC_HSIOMIX_ADBS_PWRDNREQNR {
    bits: bool,
}
impl GPC_HSIOMIX_ADBS_PWRDNREQNR {
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
pub struct GPC_NOC2HSIOMIX_ADBS_PWRDNREQNR {
    bits: bool,
}
impl GPC_NOC2HSIOMIX_ADBS_PWRDNREQNR {
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
pub struct GPC_DISPMIX_PWRDNREQNR {
    bits: bool,
}
impl GPC_DISPMIX_PWRDNREQNR {
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
pub struct GPC_VPUPMIX_PWRDNREQNR {
    bits: bool,
}
impl GPC_VPUPMIX_PWRDNREQNR {
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
pub struct GPC_GPUPMIX2NOC_3D_PWRDNREQNR {
    bits: bool,
}
impl GPC_GPUPMIX2NOC_3D_PWRDNREQNR {
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
pub struct GPC_GPUPMIX2NOC_2D_PWRDNREQNR {
    bits: bool,
}
impl GPC_GPUPMIX2NOC_2D_PWRDNREQNR {
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
pub struct GPC_NOC2GPUPMIX_PWRDNREQNR {
    bits: bool,
}
impl GPC_NOC2GPUPMIX_PWRDNREQNR {
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
pub struct GPC_DDR1_CORE_CSYSACKR {
    bits: bool,
}
impl GPC_DDR1_CORE_CSYSACKR {
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
pub struct GPC_DDR1_CORE_CACTIVER {
    bits: bool,
}
impl GPC_DDR1_CORE_CACTIVER {
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
pub struct GPC_DDR1_AXI_CSYSACKR {
    bits: bool,
}
impl GPC_DDR1_AXI_CSYSACKR {
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
pub struct GPC_DDR1_AXI_CACTIVER {
    bits: bool,
}
impl GPC_DDR1_AXI_CACTIVER {
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
pub struct GPC_NOC2DDR1_PWRDNACKNR {
    bits: bool,
}
impl GPC_NOC2DDR1_PWRDNACKNR {
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
pub struct GPC_SUPERMIX2NOC_PWRDNACKNR {
    bits: bool,
}
impl GPC_SUPERMIX2NOC_PWRDNACKNR {
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
pub struct GPC_NOC2SUPERMIX_PWRDNACKNR {
    bits: bool,
}
impl GPC_NOC2SUPERMIX_PWRDNACKNR {
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
pub struct GPC_HSIOMIX2NOC_PWRDNACKNR {
    bits: bool,
}
impl GPC_HSIOMIX2NOC_PWRDNACKNR {
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
pub struct GPC_NOC2HSIOMIX_PWRDNACKNR {
    bits: bool,
}
impl GPC_NOC2HSIOMIX_PWRDNACKNR {
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
pub struct GPC_DISPMIX_PWRDNACKNR {
    bits: bool,
}
impl GPC_DISPMIX_PWRDNACKNR {
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
pub struct GPC_VPUMIX_PWRDNACKNR {
    bits: bool,
}
impl GPC_VPUMIX_PWRDNACKNR {
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
pub struct GPC_GPUMIX2NOC_3D_PWRDNACKNR {
    bits: bool,
}
impl GPC_GPUMIX2NOC_3D_PWRDNACKNR {
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
pub struct GPC_GPUMIX2NOC_2D_PWRDNACKNR {
    bits: bool,
}
impl GPC_GPUMIX2NOC_2D_PWRDNACKNR {
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
pub struct GPC_NOC2GPUMIX_PWRDNACKNR {
    bits: bool,
}
impl GPC_NOC2GPUMIX_PWRDNACKNR {
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
pub struct _GPC_DDR1_CORE_CSYSREQW<'a> {
    w: &'a mut W,
}
impl<'a> _GPC_DDR1_CORE_CSYSREQW<'a> {
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
pub struct _GPC_DDR1_AXI_CSYSREQW<'a> {
    w: &'a mut W,
}
impl<'a> _GPC_DDR1_AXI_CSYSREQW<'a> {
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
pub struct _GPC_NOC2DDR_PWRDNREQNW<'a> {
    w: &'a mut W,
}
impl<'a> _GPC_NOC2DDR_PWRDNREQNW<'a> {
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
pub struct _GPC_HSIOMIX_ADBS_PWRDNREQNW<'a> {
    w: &'a mut W,
}
impl<'a> _GPC_HSIOMIX_ADBS_PWRDNREQNW<'a> {
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
pub struct _GPC_NOC2HSIOMIX_ADBS_PWRDNREQNW<'a> {
    w: &'a mut W,
}
impl<'a> _GPC_NOC2HSIOMIX_ADBS_PWRDNREQNW<'a> {
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
pub struct _GPC_DISPMIX_PWRDNREQNW<'a> {
    w: &'a mut W,
}
impl<'a> _GPC_DISPMIX_PWRDNREQNW<'a> {
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
pub struct _GPC_VPUPMIX_PWRDNREQNW<'a> {
    w: &'a mut W,
}
impl<'a> _GPC_VPUPMIX_PWRDNREQNW<'a> {
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
pub struct _GPC_GPUPMIX2NOC_3D_PWRDNREQNW<'a> {
    w: &'a mut W,
}
impl<'a> _GPC_GPUPMIX2NOC_3D_PWRDNREQNW<'a> {
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
pub struct _GPC_GPUPMIX2NOC_2D_PWRDNREQNW<'a> {
    w: &'a mut W,
}
impl<'a> _GPC_GPUPMIX2NOC_2D_PWRDNREQNW<'a> {
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
pub struct _GPC_NOC2GPUPMIX_PWRDNREQNW<'a> {
    w: &'a mut W,
}
impl<'a> _GPC_NOC2GPUPMIX_PWRDNREQNW<'a> {
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
pub struct _GPC_DDR1_CORE_CSYSACKW<'a> {
    w: &'a mut W,
}
impl<'a> _GPC_DDR1_CORE_CSYSACKW<'a> {
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
pub struct _GPC_DDR1_CORE_CACTIVEW<'a> {
    w: &'a mut W,
}
impl<'a> _GPC_DDR1_CORE_CACTIVEW<'a> {
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
pub struct _GPC_DDR1_AXI_CSYSACKW<'a> {
    w: &'a mut W,
}
impl<'a> _GPC_DDR1_AXI_CSYSACKW<'a> {
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
pub struct _GPC_DDR1_AXI_CACTIVEW<'a> {
    w: &'a mut W,
}
impl<'a> _GPC_DDR1_AXI_CACTIVEW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - DDR1 controller Hardware Low-Power Request"]
    #[inline]
    pub fn gpc_ddr1_core_csysreq(&self) -> GPC_DDR1_CORE_CSYSREQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPC_DDR1_CORE_CSYSREQR { bits }
    }
    #[doc = "Bit 1 - DDR1 AXI Low-Power Request"]
    #[inline]
    pub fn gpc_ddr1_axi_csysreq(&self) -> GPC_DDR1_AXI_CSYSREQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPC_DDR1_AXI_CSYSREQR { bits }
    }
    #[doc = "Bit 2 - NOC2DDR ADB400 power down request. Active 0"]
    #[inline]
    pub fn gpc_noc2ddr_pwrdnreqn(&self) -> GPC_NOC2DDR_PWRDNREQNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPC_NOC2DDR_PWRDNREQNR { bits }
    }
    #[doc = "Bit 5 - HSIOMIX2NOC ADB400 power down request. Active 0"]
    #[inline]
    pub fn gpc_hsiomix_adbs_pwrdnreqn(&self) -> GPC_HSIOMIX_ADBS_PWRDNREQNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPC_HSIOMIX_ADBS_PWRDNREQNR { bits }
    }
    #[doc = "Bit 6 - NOC2HSIOMIX ADB400 power down request. Active 0"]
    #[inline]
    pub fn gpc_noc2hsiomix_adbs_pwrdnreqn(&self) -> GPC_NOC2HSIOMIX_ADBS_PWRDNREQNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPC_NOC2HSIOMIX_ADBS_PWRDNREQNR { bits }
    }
    #[doc = "Bit 7 - DISPMIX ADB400 power down request. Active 0"]
    #[inline]
    pub fn gpc_dispmix_pwrdnreqn(&self) -> GPC_DISPMIX_PWRDNREQNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPC_DISPMIX_PWRDNREQNR { bits }
    }
    #[doc = "Bit 8 - VPUPMIX ADB400 power down request. Active 0"]
    #[inline]
    pub fn gpc_vpupmix_pwrdnreqn(&self) -> GPC_VPUPMIX_PWRDNREQNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPC_VPUPMIX_PWRDNREQNR { bits }
    }
    #[doc = "Bit 9 - GPUMIX2NOC 3D ADB400 power down request. Active 0"]
    #[inline]
    pub fn gpc_gpupmix2noc_3d_pwrdnreqn(&self) -> GPC_GPUPMIX2NOC_3D_PWRDNREQNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPC_GPUPMIX2NOC_3D_PWRDNREQNR { bits }
    }
    #[doc = "Bit 10 - GPUMIX2NOC 2D ADB400 power down request. Active 0"]
    #[inline]
    pub fn gpc_gpupmix2noc_2d_pwrdnreqn(&self) -> GPC_GPUPMIX2NOC_2D_PWRDNREQNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPC_GPUPMIX2NOC_2D_PWRDNREQNR { bits }
    }
    #[doc = "Bit 11 - NOC2GPUMIX ADB400 power down request. Active 0"]
    #[inline]
    pub fn gpc_noc2gpupmix_pwrdnreqn(&self) -> GPC_NOC2GPUPMIX_PWRDNREQNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPC_NOC2GPUPMIX_PWRDNREQNR { bits }
    }
    #[doc = "Bit 16 - DDR1 controller Hardware Low_Power ack"]
    #[inline]
    pub fn gpc_ddr1_core_csysack(&self) -> GPC_DDR1_CORE_CSYSACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPC_DDR1_CORE_CSYSACKR { bits }
    }
    #[doc = "Bit 17 - DDR1 controller Hardware Low-Power Clock active"]
    #[inline]
    pub fn gpc_ddr1_core_cactive(&self) -> GPC_DDR1_CORE_CACTIVER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPC_DDR1_CORE_CACTIVER { bits }
    }
    #[doc = "Bit 18 - DDR1 AXI Low-Power Request ack"]
    #[inline]
    pub fn gpc_ddr1_axi_csysack(&self) -> GPC_DDR1_AXI_CSYSACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPC_DDR1_AXI_CSYSACKR { bits }
    }
    #[doc = "Bit 19 - DDR1 AXI Clock Active"]
    #[inline]
    pub fn gpc_ddr1_axi_cactive(&self) -> GPC_DDR1_AXI_CACTIVER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPC_DDR1_AXI_CACTIVER { bits }
    }
    #[doc = "Bit 20 - NOC2DDR ADB400 power down ack. Active 0"]
    #[inline]
    pub fn gpc_noc2ddr1_pwrdnackn(&self) -> GPC_NOC2DDR1_PWRDNACKNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPC_NOC2DDR1_PWRDNACKNR { bits }
    }
    #[doc = "Bit 21 - SUPERMIX2NOC ADB400 power down ack. Active 0"]
    #[inline]
    pub fn gpc_supermix2noc_pwrdnackn(&self) -> GPC_SUPERMIX2NOC_PWRDNACKNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPC_SUPERMIX2NOC_PWRDNACKNR { bits }
    }
    #[doc = "Bit 22 - NOC2SUPERMIX ADB400 power down ack. Active 0"]
    #[inline]
    pub fn gpc_noc2supermix_pwrdnackn(&self) -> GPC_NOC2SUPERMIX_PWRDNACKNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPC_NOC2SUPERMIX_PWRDNACKNR { bits }
    }
    #[doc = "Bit 23 - HSIOMIX2NOC ADB400 power down ack.Active 0"]
    #[inline]
    pub fn gpc_hsiomix2noc_pwrdnackn(&self) -> GPC_HSIOMIX2NOC_PWRDNACKNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPC_HSIOMIX2NOC_PWRDNACKNR { bits }
    }
    #[doc = "Bit 24 - NOC2HSIOMIX ADB400 power down ack. Active 0"]
    #[inline]
    pub fn gpc_noc2hsiomix_pwrdnackn(&self) -> GPC_NOC2HSIOMIX_PWRDNACKNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPC_NOC2HSIOMIX_PWRDNACKNR { bits }
    }
    #[doc = "Bit 25 - DISPMIX ADB400 power down ack. Active 0"]
    #[inline]
    pub fn gpc_dispmix_pwrdnackn(&self) -> GPC_DISPMIX_PWRDNACKNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPC_DISPMIX_PWRDNACKNR { bits }
    }
    #[doc = "Bit 26 - VPU ADB400 power down ack. Active 0"]
    #[inline]
    pub fn gpc_vpumix_pwrdnackn(&self) -> GPC_VPUMIX_PWRDNACKNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPC_VPUMIX_PWRDNACKNR { bits }
    }
    #[doc = "Bit 27 - GPUMIX2NOC(3D) ADB400 power down ack. Active 0"]
    #[inline]
    pub fn gpc_gpumix2noc_3d_pwrdnackn(&self) -> GPC_GPUMIX2NOC_3D_PWRDNACKNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPC_GPUMIX2NOC_3D_PWRDNACKNR { bits }
    }
    #[doc = "Bit 28 - GPUMIX2NOC(2D) ADB400 power down ack. Active 0"]
    #[inline]
    pub fn gpc_gpumix2noc_2d_pwrdnackn(&self) -> GPC_GPUMIX2NOC_2D_PWRDNACKNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPC_GPUMIX2NOC_2D_PWRDNACKNR { bits }
    }
    #[doc = "Bit 29 - NOC2GPUMIX ADB400 power down ack. Active 0"]
    #[inline]
    pub fn gpc_noc2gpumix_pwrdnackn(&self) -> GPC_NOC2GPUMIX_PWRDNACKNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPC_NOC2GPUMIX_PWRDNACKNR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 65535 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - DDR1 controller Hardware Low-Power Request"]
    #[inline]
    pub fn gpc_ddr1_core_csysreq(&mut self) -> _GPC_DDR1_CORE_CSYSREQW {
        _GPC_DDR1_CORE_CSYSREQW { w: self }
    }
    #[doc = "Bit 1 - DDR1 AXI Low-Power Request"]
    #[inline]
    pub fn gpc_ddr1_axi_csysreq(&mut self) -> _GPC_DDR1_AXI_CSYSREQW {
        _GPC_DDR1_AXI_CSYSREQW { w: self }
    }
    #[doc = "Bit 2 - NOC2DDR ADB400 power down request. Active 0"]
    #[inline]
    pub fn gpc_noc2ddr_pwrdnreqn(&mut self) -> _GPC_NOC2DDR_PWRDNREQNW {
        _GPC_NOC2DDR_PWRDNREQNW { w: self }
    }
    #[doc = "Bit 5 - HSIOMIX2NOC ADB400 power down request. Active 0"]
    #[inline]
    pub fn gpc_hsiomix_adbs_pwrdnreqn(&mut self) -> _GPC_HSIOMIX_ADBS_PWRDNREQNW {
        _GPC_HSIOMIX_ADBS_PWRDNREQNW { w: self }
    }
    #[doc = "Bit 6 - NOC2HSIOMIX ADB400 power down request. Active 0"]
    #[inline]
    pub fn gpc_noc2hsiomix_adbs_pwrdnreqn(&mut self) -> _GPC_NOC2HSIOMIX_ADBS_PWRDNREQNW {
        _GPC_NOC2HSIOMIX_ADBS_PWRDNREQNW { w: self }
    }
    #[doc = "Bit 7 - DISPMIX ADB400 power down request. Active 0"]
    #[inline]
    pub fn gpc_dispmix_pwrdnreqn(&mut self) -> _GPC_DISPMIX_PWRDNREQNW {
        _GPC_DISPMIX_PWRDNREQNW { w: self }
    }
    #[doc = "Bit 8 - VPUPMIX ADB400 power down request. Active 0"]
    #[inline]
    pub fn gpc_vpupmix_pwrdnreqn(&mut self) -> _GPC_VPUPMIX_PWRDNREQNW {
        _GPC_VPUPMIX_PWRDNREQNW { w: self }
    }
    #[doc = "Bit 9 - GPUMIX2NOC 3D ADB400 power down request. Active 0"]
    #[inline]
    pub fn gpc_gpupmix2noc_3d_pwrdnreqn(&mut self) -> _GPC_GPUPMIX2NOC_3D_PWRDNREQNW {
        _GPC_GPUPMIX2NOC_3D_PWRDNREQNW { w: self }
    }
    #[doc = "Bit 10 - GPUMIX2NOC 2D ADB400 power down request. Active 0"]
    #[inline]
    pub fn gpc_gpupmix2noc_2d_pwrdnreqn(&mut self) -> _GPC_GPUPMIX2NOC_2D_PWRDNREQNW {
        _GPC_GPUPMIX2NOC_2D_PWRDNREQNW { w: self }
    }
    #[doc = "Bit 11 - NOC2GPUMIX ADB400 power down request. Active 0"]
    #[inline]
    pub fn gpc_noc2gpupmix_pwrdnreqn(&mut self) -> _GPC_NOC2GPUPMIX_PWRDNREQNW {
        _GPC_NOC2GPUPMIX_PWRDNREQNW { w: self }
    }
    #[doc = "Bit 16 - DDR1 controller Hardware Low_Power ack"]
    #[inline]
    pub fn gpc_ddr1_core_csysack(&mut self) -> _GPC_DDR1_CORE_CSYSACKW {
        _GPC_DDR1_CORE_CSYSACKW { w: self }
    }
    #[doc = "Bit 17 - DDR1 controller Hardware Low-Power Clock active"]
    #[inline]
    pub fn gpc_ddr1_core_cactive(&mut self) -> _GPC_DDR1_CORE_CACTIVEW {
        _GPC_DDR1_CORE_CACTIVEW { w: self }
    }
    #[doc = "Bit 18 - DDR1 AXI Low-Power Request ack"]
    #[inline]
    pub fn gpc_ddr1_axi_csysack(&mut self) -> _GPC_DDR1_AXI_CSYSACKW {
        _GPC_DDR1_AXI_CSYSACKW { w: self }
    }
    #[doc = "Bit 19 - DDR1 AXI Clock Active"]
    #[inline]
    pub fn gpc_ddr1_axi_cactive(&mut self) -> _GPC_DDR1_AXI_CACTIVEW {
        _GPC_DDR1_AXI_CACTIVEW { w: self }
    }
}
