#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PGC_CPU_0_1_MAPPING {
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
pub struct MF_A53_DOMAINR {
    bits: bool,
}
impl MF_A53_DOMAINR {
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
pub struct NOC_A53_DOMAINR {
    bits: bool,
}
impl NOC_A53_DOMAINR {
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
pub struct MIPI_A53_DOMAINR {
    bits: bool,
}
impl MIPI_A53_DOMAINR {
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
pub struct PCIE_A53_DOMAINR {
    bits: bool,
}
impl PCIE_A53_DOMAINR {
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
pub struct OTG1_A53_DOMAINR {
    bits: bool,
}
impl OTG1_A53_DOMAINR {
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
pub struct OTG2_A53_DOMAINR {
    bits: bool,
}
impl OTG2_A53_DOMAINR {
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
pub struct DDR1_A53_DOMAINR {
    bits: bool,
}
impl DDR1_A53_DOMAINR {
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
pub struct GPU_2D_A53_DOMAINR {
    bits: bool,
}
impl GPU_2D_A53_DOMAINR {
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
pub struct GPUMIX_A53_DOMAINR {
    bits: bool,
}
impl GPUMIX_A53_DOMAINR {
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
pub struct VPUMIX_A53_DOMAINR {
    bits: bool,
}
impl VPUMIX_A53_DOMAINR {
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
pub struct GPU_3D_A53_DOMAINR {
    bits: bool,
}
impl GPU_3D_A53_DOMAINR {
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
pub struct DISPMIX_A53_DOMAINR {
    bits: bool,
}
impl DISPMIX_A53_DOMAINR {
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
pub struct VPU_G1_A53_DOMAINR {
    bits: bool,
}
impl VPU_G1_A53_DOMAINR {
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
pub struct VPU_G2_A53_DOMAINR {
    bits: bool,
}
impl VPU_G2_A53_DOMAINR {
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
pub struct VPU_H1_A53_DOMAINR {
    bits: bool,
}
impl VPU_H1_A53_DOMAINR {
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
pub struct MF_M4_DOMAINR {
    bits: bool,
}
impl MF_M4_DOMAINR {
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
pub struct NOC_M4_DOMAINR {
    bits: bool,
}
impl NOC_M4_DOMAINR {
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
pub struct MIPI_M4_DOMAINR {
    bits: bool,
}
impl MIPI_M4_DOMAINR {
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
pub struct PCIE_M4_DOMAINR {
    bits: bool,
}
impl PCIE_M4_DOMAINR {
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
pub struct OTG1_M4_DOMAINR {
    bits: bool,
}
impl OTG1_M4_DOMAINR {
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
pub struct OTG2_M4_DOMAINR {
    bits: bool,
}
impl OTG2_M4_DOMAINR {
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
pub struct DDR1_M4_DOMAINR {
    bits: bool,
}
impl DDR1_M4_DOMAINR {
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
pub struct GPU_2D_M4_DOMAINR {
    bits: bool,
}
impl GPU_2D_M4_DOMAINR {
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
pub struct GPUMIX_M4_DOMAINR {
    bits: bool,
}
impl GPUMIX_M4_DOMAINR {
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
pub struct VPUMIX_M4_DOMAINR {
    bits: bool,
}
impl VPUMIX_M4_DOMAINR {
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
pub struct GPU_3D_M4_DOMAINR {
    bits: bool,
}
impl GPU_3D_M4_DOMAINR {
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
pub struct DISPMIX_M4_DOMAINR {
    bits: bool,
}
impl DISPMIX_M4_DOMAINR {
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
pub struct VPU_G1_M4_DOMAINR {
    bits: bool,
}
impl VPU_G1_M4_DOMAINR {
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
pub struct VPU_G2_M4_DOMAINR {
    bits: bool,
}
impl VPU_G2_M4_DOMAINR {
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
pub struct VPU_H1_M4_DOMAINR {
    bits: bool,
}
impl VPU_H1_M4_DOMAINR {
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
pub struct _MF_A53_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _MF_A53_DOMAINW<'a> {
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
pub struct _NOC_A53_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _NOC_A53_DOMAINW<'a> {
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
pub struct _MIPI_A53_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _MIPI_A53_DOMAINW<'a> {
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
pub struct _PCIE_A53_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _PCIE_A53_DOMAINW<'a> {
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
pub struct _OTG1_A53_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _OTG1_A53_DOMAINW<'a> {
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
pub struct _OTG2_A53_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _OTG2_A53_DOMAINW<'a> {
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
pub struct _DDR1_A53_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _DDR1_A53_DOMAINW<'a> {
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
pub struct _GPU_2D_A53_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _GPU_2D_A53_DOMAINW<'a> {
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
pub struct _GPUMIX_A53_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _GPUMIX_A53_DOMAINW<'a> {
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
pub struct _VPUMIX_A53_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _VPUMIX_A53_DOMAINW<'a> {
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
pub struct _GPU_3D_A53_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _GPU_3D_A53_DOMAINW<'a> {
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
pub struct _DISPMIX_A53_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _DISPMIX_A53_DOMAINW<'a> {
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
pub struct _VPU_G1_A53_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _VPU_G1_A53_DOMAINW<'a> {
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
pub struct _VPU_G2_A53_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _VPU_G2_A53_DOMAINW<'a> {
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
pub struct _VPU_H1_A53_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _VPU_H1_A53_DOMAINW<'a> {
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
pub struct _MF_M4_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _MF_M4_DOMAINW<'a> {
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
pub struct _NOC_M4_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _NOC_M4_DOMAINW<'a> {
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
pub struct _MIPI_M4_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _MIPI_M4_DOMAINW<'a> {
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
pub struct _PCIE_M4_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _PCIE_M4_DOMAINW<'a> {
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
pub struct _OTG1_M4_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _OTG1_M4_DOMAINW<'a> {
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
pub struct _OTG2_M4_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _OTG2_M4_DOMAINW<'a> {
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
pub struct _DDR1_M4_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _DDR1_M4_DOMAINW<'a> {
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
pub struct _GPU_2D_M4_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _GPU_2D_M4_DOMAINW<'a> {
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
#[doc = r" Proxy"]
pub struct _GPUMIX_M4_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _GPUMIX_M4_DOMAINW<'a> {
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
pub struct _VPUMIX_M4_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _VPUMIX_M4_DOMAINW<'a> {
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
pub struct _GPU_3D_M4_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _GPU_3D_M4_DOMAINW<'a> {
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
pub struct _DISPMIX_M4_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _DISPMIX_M4_DOMAINW<'a> {
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
pub struct _VPU_G1_M4_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _VPU_G1_M4_DOMAINW<'a> {
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
pub struct _VPU_G2_M4_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _VPU_G2_M4_DOMAINW<'a> {
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
pub struct _VPU_H1_M4_DOMAINW<'a> {
    w: &'a mut W,
}
impl<'a> _VPU_H1_M4_DOMAINW<'a> {
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
    #[doc = "Bit 0 - MF_A53_DOMAIN"]
    #[inline]
    pub fn mf_a53_domain(&self) -> MF_A53_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MF_A53_DOMAINR { bits }
    }
    #[doc = "Bit 1 - NOC_A53_DOMAIN"]
    #[inline]
    pub fn noc_a53_domain(&self) -> NOC_A53_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NOC_A53_DOMAINR { bits }
    }
    #[doc = "Bit 2 - MIPI A53 DOMAIN"]
    #[inline]
    pub fn mipi_a53_domain(&self) -> MIPI_A53_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MIPI_A53_DOMAINR { bits }
    }
    #[doc = "Bit 3 - PCIE_A53_DOMAIN"]
    #[inline]
    pub fn pcie_a53_domain(&self) -> PCIE_A53_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCIE_A53_DOMAINR { bits }
    }
    #[doc = "Bit 4 - OTG1_A53_DOMAIN"]
    #[inline]
    pub fn otg1_a53_domain(&self) -> OTG1_A53_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OTG1_A53_DOMAINR { bits }
    }
    #[doc = "Bit 5 - OTG2_A53_DOMAIN"]
    #[inline]
    pub fn otg2_a53_domain(&self) -> OTG2_A53_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OTG2_A53_DOMAINR { bits }
    }
    #[doc = "Bit 7 - DDR1_A53_DOMAIN"]
    #[inline]
    pub fn ddr1_a53_domain(&self) -> DDR1_A53_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DDR1_A53_DOMAINR { bits }
    }
    #[doc = "Bit 8 - GPU_2D_A53_DOMAIN"]
    #[inline]
    pub fn gpu_2d_a53_domain(&self) -> GPU_2D_A53_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPU_2D_A53_DOMAINR { bits }
    }
    #[doc = "Bit 9 - GPUMIX_A53_DOMAIN"]
    #[inline]
    pub fn gpumix_a53_domain(&self) -> GPUMIX_A53_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPUMIX_A53_DOMAINR { bits }
    }
    #[doc = "Bit 10 - VPUMIX_A53_DOMAIN"]
    #[inline]
    pub fn vpumix_a53_domain(&self) -> VPUMIX_A53_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPUMIX_A53_DOMAINR { bits }
    }
    #[doc = "Bit 11 - GPU_3D_A53_DOMAIN"]
    #[inline]
    pub fn gpu_3d_a53_domain(&self) -> GPU_3D_A53_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPU_3D_A53_DOMAINR { bits }
    }
    #[doc = "Bit 12 - DISP_MIXA53_DOMAIN"]
    #[inline]
    pub fn dispmix_a53_domain(&self) -> DISPMIX_A53_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISPMIX_A53_DOMAINR { bits }
    }
    #[doc = "Bit 13 - VPU_G1_A53_DOMAIN"]
    #[inline]
    pub fn vpu_g1_a53_domain(&self) -> VPU_G1_A53_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPU_G1_A53_DOMAINR { bits }
    }
    #[doc = "Bit 14 - VPU_G2_A53_DOMAIN"]
    #[inline]
    pub fn vpu_g2_a53_domain(&self) -> VPU_G2_A53_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPU_G2_A53_DOMAINR { bits }
    }
    #[doc = "Bit 15 - VPU_H1_A53_DOMAIN"]
    #[inline]
    pub fn vpu_h1_a53_domain(&self) -> VPU_H1_A53_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPU_H1_A53_DOMAINR { bits }
    }
    #[doc = "Bit 16 - MF_M4_DOMAIN"]
    #[inline]
    pub fn mf_m4_domain(&self) -> MF_M4_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MF_M4_DOMAINR { bits }
    }
    #[doc = "Bit 17 - NOC_M4_DOMAIN"]
    #[inline]
    pub fn noc_m4_domain(&self) -> NOC_M4_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NOC_M4_DOMAINR { bits }
    }
    #[doc = "Bit 18 - MIPI_M4_DOMAIN"]
    #[inline]
    pub fn mipi_m4_domain(&self) -> MIPI_M4_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MIPI_M4_DOMAINR { bits }
    }
    #[doc = "Bit 19 - PCIE_M4_DOMAIN"]
    #[inline]
    pub fn pcie_m4_domain(&self) -> PCIE_M4_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCIE_M4_DOMAINR { bits }
    }
    #[doc = "Bit 20 - OTG1_M4_DOMAIN"]
    #[inline]
    pub fn otg1_m4_domain(&self) -> OTG1_M4_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OTG1_M4_DOMAINR { bits }
    }
    #[doc = "Bit 21 - OTG2_M4_DOMAIN"]
    #[inline]
    pub fn otg2_m4_domain(&self) -> OTG2_M4_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OTG2_M4_DOMAINR { bits }
    }
    #[doc = "Bit 23 - DDR1_M4_DOMAIN"]
    #[inline]
    pub fn ddr1_m4_domain(&self) -> DDR1_M4_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DDR1_M4_DOMAINR { bits }
    }
    #[doc = "Bit 24 - GPU_2D_M4_DOMAIN"]
    #[inline]
    pub fn gpu_2d_m4_domain(&self) -> GPU_2D_M4_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPU_2D_M4_DOMAINR { bits }
    }
    #[doc = "Bit 25 - GPUMIX_M4_DOMAIN"]
    #[inline]
    pub fn gpumix_m4_domain(&self) -> GPUMIX_M4_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPUMIX_M4_DOMAINR { bits }
    }
    #[doc = "Bit 26 - VPUMIX_M4_DOMAIN"]
    #[inline]
    pub fn vpumix_m4_domain(&self) -> VPUMIX_M4_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPUMIX_M4_DOMAINR { bits }
    }
    #[doc = "Bit 27 - GPU_3D_M4_DOMAIN"]
    #[inline]
    pub fn gpu_3d_m4_domain(&self) -> GPU_3D_M4_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPU_3D_M4_DOMAINR { bits }
    }
    #[doc = "Bit 28 - DISPMIX_M4_DOMAIN"]
    #[inline]
    pub fn dispmix_m4_domain(&self) -> DISPMIX_M4_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISPMIX_M4_DOMAINR { bits }
    }
    #[doc = "Bit 29 - VPU_G1_M4_DOMAIN"]
    #[inline]
    pub fn vpu_g1_m4_domain(&self) -> VPU_G1_M4_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPU_G1_M4_DOMAINR { bits }
    }
    #[doc = "Bit 30 - VPU_G2_M4_DOMAIN"]
    #[inline]
    pub fn vpu_g2_m4_domain(&self) -> VPU_G2_M4_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPU_G2_M4_DOMAINR { bits }
    }
    #[doc = "Bit 31 - VPU_H1_M4_DOMAIN"]
    #[inline]
    pub fn vpu_h1_m4_domain(&self) -> VPU_H1_M4_DOMAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPU_H1_M4_DOMAINR { bits }
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
    #[doc = "Bit 0 - MF_A53_DOMAIN"]
    #[inline]
    pub fn mf_a53_domain(&mut self) -> _MF_A53_DOMAINW {
        _MF_A53_DOMAINW { w: self }
    }
    #[doc = "Bit 1 - NOC_A53_DOMAIN"]
    #[inline]
    pub fn noc_a53_domain(&mut self) -> _NOC_A53_DOMAINW {
        _NOC_A53_DOMAINW { w: self }
    }
    #[doc = "Bit 2 - MIPI A53 DOMAIN"]
    #[inline]
    pub fn mipi_a53_domain(&mut self) -> _MIPI_A53_DOMAINW {
        _MIPI_A53_DOMAINW { w: self }
    }
    #[doc = "Bit 3 - PCIE_A53_DOMAIN"]
    #[inline]
    pub fn pcie_a53_domain(&mut self) -> _PCIE_A53_DOMAINW {
        _PCIE_A53_DOMAINW { w: self }
    }
    #[doc = "Bit 4 - OTG1_A53_DOMAIN"]
    #[inline]
    pub fn otg1_a53_domain(&mut self) -> _OTG1_A53_DOMAINW {
        _OTG1_A53_DOMAINW { w: self }
    }
    #[doc = "Bit 5 - OTG2_A53_DOMAIN"]
    #[inline]
    pub fn otg2_a53_domain(&mut self) -> _OTG2_A53_DOMAINW {
        _OTG2_A53_DOMAINW { w: self }
    }
    #[doc = "Bit 7 - DDR1_A53_DOMAIN"]
    #[inline]
    pub fn ddr1_a53_domain(&mut self) -> _DDR1_A53_DOMAINW {
        _DDR1_A53_DOMAINW { w: self }
    }
    #[doc = "Bit 8 - GPU_2D_A53_DOMAIN"]
    #[inline]
    pub fn gpu_2d_a53_domain(&mut self) -> _GPU_2D_A53_DOMAINW {
        _GPU_2D_A53_DOMAINW { w: self }
    }
    #[doc = "Bit 9 - GPUMIX_A53_DOMAIN"]
    #[inline]
    pub fn gpumix_a53_domain(&mut self) -> _GPUMIX_A53_DOMAINW {
        _GPUMIX_A53_DOMAINW { w: self }
    }
    #[doc = "Bit 10 - VPUMIX_A53_DOMAIN"]
    #[inline]
    pub fn vpumix_a53_domain(&mut self) -> _VPUMIX_A53_DOMAINW {
        _VPUMIX_A53_DOMAINW { w: self }
    }
    #[doc = "Bit 11 - GPU_3D_A53_DOMAIN"]
    #[inline]
    pub fn gpu_3d_a53_domain(&mut self) -> _GPU_3D_A53_DOMAINW {
        _GPU_3D_A53_DOMAINW { w: self }
    }
    #[doc = "Bit 12 - DISP_MIXA53_DOMAIN"]
    #[inline]
    pub fn dispmix_a53_domain(&mut self) -> _DISPMIX_A53_DOMAINW {
        _DISPMIX_A53_DOMAINW { w: self }
    }
    #[doc = "Bit 13 - VPU_G1_A53_DOMAIN"]
    #[inline]
    pub fn vpu_g1_a53_domain(&mut self) -> _VPU_G1_A53_DOMAINW {
        _VPU_G1_A53_DOMAINW { w: self }
    }
    #[doc = "Bit 14 - VPU_G2_A53_DOMAIN"]
    #[inline]
    pub fn vpu_g2_a53_domain(&mut self) -> _VPU_G2_A53_DOMAINW {
        _VPU_G2_A53_DOMAINW { w: self }
    }
    #[doc = "Bit 15 - VPU_H1_A53_DOMAIN"]
    #[inline]
    pub fn vpu_h1_a53_domain(&mut self) -> _VPU_H1_A53_DOMAINW {
        _VPU_H1_A53_DOMAINW { w: self }
    }
    #[doc = "Bit 16 - MF_M4_DOMAIN"]
    #[inline]
    pub fn mf_m4_domain(&mut self) -> _MF_M4_DOMAINW {
        _MF_M4_DOMAINW { w: self }
    }
    #[doc = "Bit 17 - NOC_M4_DOMAIN"]
    #[inline]
    pub fn noc_m4_domain(&mut self) -> _NOC_M4_DOMAINW {
        _NOC_M4_DOMAINW { w: self }
    }
    #[doc = "Bit 18 - MIPI_M4_DOMAIN"]
    #[inline]
    pub fn mipi_m4_domain(&mut self) -> _MIPI_M4_DOMAINW {
        _MIPI_M4_DOMAINW { w: self }
    }
    #[doc = "Bit 19 - PCIE_M4_DOMAIN"]
    #[inline]
    pub fn pcie_m4_domain(&mut self) -> _PCIE_M4_DOMAINW {
        _PCIE_M4_DOMAINW { w: self }
    }
    #[doc = "Bit 20 - OTG1_M4_DOMAIN"]
    #[inline]
    pub fn otg1_m4_domain(&mut self) -> _OTG1_M4_DOMAINW {
        _OTG1_M4_DOMAINW { w: self }
    }
    #[doc = "Bit 21 - OTG2_M4_DOMAIN"]
    #[inline]
    pub fn otg2_m4_domain(&mut self) -> _OTG2_M4_DOMAINW {
        _OTG2_M4_DOMAINW { w: self }
    }
    #[doc = "Bit 23 - DDR1_M4_DOMAIN"]
    #[inline]
    pub fn ddr1_m4_domain(&mut self) -> _DDR1_M4_DOMAINW {
        _DDR1_M4_DOMAINW { w: self }
    }
    #[doc = "Bit 24 - GPU_2D_M4_DOMAIN"]
    #[inline]
    pub fn gpu_2d_m4_domain(&mut self) -> _GPU_2D_M4_DOMAINW {
        _GPU_2D_M4_DOMAINW { w: self }
    }
    #[doc = "Bit 25 - GPUMIX_M4_DOMAIN"]
    #[inline]
    pub fn gpumix_m4_domain(&mut self) -> _GPUMIX_M4_DOMAINW {
        _GPUMIX_M4_DOMAINW { w: self }
    }
    #[doc = "Bit 26 - VPUMIX_M4_DOMAIN"]
    #[inline]
    pub fn vpumix_m4_domain(&mut self) -> _VPUMIX_M4_DOMAINW {
        _VPUMIX_M4_DOMAINW { w: self }
    }
    #[doc = "Bit 27 - GPU_3D_M4_DOMAIN"]
    #[inline]
    pub fn gpu_3d_m4_domain(&mut self) -> _GPU_3D_M4_DOMAINW {
        _GPU_3D_M4_DOMAINW { w: self }
    }
    #[doc = "Bit 28 - DISPMIX_M4_DOMAIN"]
    #[inline]
    pub fn dispmix_m4_domain(&mut self) -> _DISPMIX_M4_DOMAINW {
        _DISPMIX_M4_DOMAINW { w: self }
    }
    #[doc = "Bit 29 - VPU_G1_M4_DOMAIN"]
    #[inline]
    pub fn vpu_g1_m4_domain(&mut self) -> _VPU_G1_M4_DOMAINW {
        _VPU_G1_M4_DOMAINW { w: self }
    }
    #[doc = "Bit 30 - VPU_G2_M4_DOMAIN"]
    #[inline]
    pub fn vpu_g2_m4_domain(&mut self) -> _VPU_G2_M4_DOMAINW {
        _VPU_G2_M4_DOMAINW { w: self }
    }
    #[doc = "Bit 31 - VPU_H1_M4_DOMAIN"]
    #[inline]
    pub fn vpu_h1_m4_domain(&mut self) -> _VPU_H1_M4_DOMAINW {
        _VPU_H1_M4_DOMAINW { w: self }
    }
}
