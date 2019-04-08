#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SLT_CFG_PU {
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
pub struct MF_PDN_SLOT_CONTROLR {
    bits: bool,
}
impl MF_PDN_SLOT_CONTROLR {
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
pub struct MF_PUP_SLOT_CONTROLR {
    bits: bool,
}
impl MF_PUP_SLOT_CONTROLR {
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
pub struct MIPI_PDN_SLOT_CONTROLR {
    bits: bool,
}
impl MIPI_PDN_SLOT_CONTROLR {
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
pub struct MIPI_PUP_SLOT_CONTROLR {
    bits: bool,
}
impl MIPI_PUP_SLOT_CONTROLR {
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
pub struct PCIE_PDN_SLOT_CONTROLR {
    bits: bool,
}
impl PCIE_PDN_SLOT_CONTROLR {
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
pub struct PCIE_PUP_SLOT_CONTROLR {
    bits: bool,
}
impl PCIE_PUP_SLOT_CONTROLR {
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
pub struct OTG1_PDN_SLOT_CONTROLR {
    bits: bool,
}
impl OTG1_PDN_SLOT_CONTROLR {
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
pub struct OTG1_PUP_SLOT_CONTROLR {
    bits: bool,
}
impl OTG1_PUP_SLOT_CONTROLR {
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
pub struct OTG2_PDN_SLOT_CONTROLR {
    bits: bool,
}
impl OTG2_PDN_SLOT_CONTROLR {
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
pub struct OTG2_PUP_SLOT_CONTROLR {
    bits: bool,
}
impl OTG2_PUP_SLOT_CONTROLR {
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
pub struct M4_PDN_SLOT_CONTROLR {
    bits: bool,
}
impl M4_PDN_SLOT_CONTROLR {
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
pub struct M4_PUP_SLOT_CONTROLR {
    bits: bool,
}
impl M4_PUP_SLOT_CONTROLR {
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
pub struct DDR1_PDN_SLOT_CONTROLR {
    bits: bool,
}
impl DDR1_PDN_SLOT_CONTROLR {
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
pub struct DDR1_PUP_SLOT_CONTROLR {
    bits: bool,
}
impl DDR1_PUP_SLOT_CONTROLR {
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
pub struct GPU_2D_PDN_SLOT_CONTROLR {
    bits: bool,
}
impl GPU_2D_PDN_SLOT_CONTROLR {
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
pub struct GPU_2D_PUP_SLOT_CONTROLR {
    bits: bool,
}
impl GPU_2D_PUP_SLOT_CONTROLR {
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
pub struct GPUMIX_PDN_SLOT_CONTROLR {
    bits: bool,
}
impl GPUMIX_PDN_SLOT_CONTROLR {
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
pub struct GPUMIX_PUP_SLOT_CONTROLR {
    bits: bool,
}
impl GPUMIX_PUP_SLOT_CONTROLR {
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
pub struct VPUMIX_PDN_SLOT_CONTROLR {
    bits: bool,
}
impl VPUMIX_PDN_SLOT_CONTROLR {
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
pub struct VPUMIX_PUP_SLOT_CONTROLR {
    bits: bool,
}
impl VPUMIX_PUP_SLOT_CONTROLR {
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
pub struct GPU_3D_PDN_SLOT_CONTROLR {
    bits: bool,
}
impl GPU_3D_PDN_SLOT_CONTROLR {
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
pub struct GPU_3D_PUP_SLOT_CONTROLR {
    bits: bool,
}
impl GPU_3D_PUP_SLOT_CONTROLR {
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
pub struct DISPMIX_PDN_SLOT_CONTROLR {
    bits: bool,
}
impl DISPMIX_PDN_SLOT_CONTROLR {
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
pub struct DISPMIX_PUP_SLOT_CONTROLR {
    bits: bool,
}
impl DISPMIX_PUP_SLOT_CONTROLR {
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
pub struct VPU_G1_PDN_SLOT_CONTROLR {
    bits: bool,
}
impl VPU_G1_PDN_SLOT_CONTROLR {
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
pub struct VPU_G1_PUP_SLOT_CONTROLR {
    bits: bool,
}
impl VPU_G1_PUP_SLOT_CONTROLR {
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
pub struct VPU_G2_PDN_SLOT_CONTROLR {
    bits: bool,
}
impl VPU_G2_PDN_SLOT_CONTROLR {
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
pub struct VPU_G2_PUP_SLOT_CONTROLR {
    bits: bool,
}
impl VPU_G2_PUP_SLOT_CONTROLR {
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
pub struct VPU_H1_PDN_SLOT_CONTROLR {
    bits: bool,
}
impl VPU_H1_PDN_SLOT_CONTROLR {
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
pub struct VPU_H1_PUP_SLOT_CONTROLR {
    bits: bool,
}
impl VPU_H1_PUP_SLOT_CONTROLR {
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
pub struct _MF_PDN_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _MF_PDN_SLOT_CONTROLW<'a> {
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
pub struct _MF_PUP_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _MF_PUP_SLOT_CONTROLW<'a> {
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
pub struct _MIPI_PDN_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _MIPI_PDN_SLOT_CONTROLW<'a> {
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
pub struct _MIPI_PUP_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _MIPI_PUP_SLOT_CONTROLW<'a> {
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
pub struct _PCIE_PDN_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _PCIE_PDN_SLOT_CONTROLW<'a> {
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
pub struct _PCIE_PUP_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _PCIE_PUP_SLOT_CONTROLW<'a> {
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
pub struct _OTG1_PDN_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _OTG1_PDN_SLOT_CONTROLW<'a> {
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
pub struct _OTG1_PUP_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _OTG1_PUP_SLOT_CONTROLW<'a> {
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
pub struct _OTG2_PDN_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _OTG2_PDN_SLOT_CONTROLW<'a> {
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
pub struct _OTG2_PUP_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _OTG2_PUP_SLOT_CONTROLW<'a> {
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
pub struct _M4_PDN_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _M4_PDN_SLOT_CONTROLW<'a> {
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
pub struct _M4_PUP_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _M4_PUP_SLOT_CONTROLW<'a> {
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
pub struct _DDR1_PDN_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _DDR1_PDN_SLOT_CONTROLW<'a> {
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
pub struct _DDR1_PUP_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _DDR1_PUP_SLOT_CONTROLW<'a> {
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
pub struct _GPU_2D_PDN_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPU_2D_PDN_SLOT_CONTROLW<'a> {
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
pub struct _GPU_2D_PUP_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPU_2D_PUP_SLOT_CONTROLW<'a> {
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
pub struct _GPUMIX_PDN_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPUMIX_PDN_SLOT_CONTROLW<'a> {
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
pub struct _GPUMIX_PUP_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPUMIX_PUP_SLOT_CONTROLW<'a> {
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
pub struct _VPUMIX_PDN_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _VPUMIX_PDN_SLOT_CONTROLW<'a> {
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
pub struct _VPUMIX_PUP_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _VPUMIX_PUP_SLOT_CONTROLW<'a> {
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
pub struct _GPU_3D_PDN_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPU_3D_PDN_SLOT_CONTROLW<'a> {
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
pub struct _GPU_3D_PUP_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _GPU_3D_PUP_SLOT_CONTROLW<'a> {
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
pub struct _DISPMIX_PDN_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _DISPMIX_PDN_SLOT_CONTROLW<'a> {
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
pub struct _DISPMIX_PUP_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _DISPMIX_PUP_SLOT_CONTROLW<'a> {
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
pub struct _VPU_G1_PDN_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _VPU_G1_PDN_SLOT_CONTROLW<'a> {
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
pub struct _VPU_G1_PUP_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _VPU_G1_PUP_SLOT_CONTROLW<'a> {
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
pub struct _VPU_G2_PDN_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _VPU_G2_PDN_SLOT_CONTROLW<'a> {
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
pub struct _VPU_G2_PUP_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _VPU_G2_PUP_SLOT_CONTROLW<'a> {
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
pub struct _VPU_H1_PDN_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _VPU_H1_PDN_SLOT_CONTROLW<'a> {
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
pub struct _VPU_H1_PUP_SLOT_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _VPU_H1_PUP_SLOT_CONTROLW<'a> {
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
    #[doc = "Bit 0 - MF Power-down slot control"]
    #[inline]
    pub fn mf_pdn_slot_control(&self) -> MF_PDN_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MF_PDN_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 1 - MF Power-up slot control"]
    #[inline]
    pub fn mf_pup_slot_control(&self) -> MF_PUP_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MF_PUP_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 2 - MIPI Power-down slot control"]
    #[inline]
    pub fn mipi_pdn_slot_control(&self) -> MIPI_PDN_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MIPI_PDN_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 3 - MIPI Power-up slot control"]
    #[inline]
    pub fn mipi_pup_slot_control(&self) -> MIPI_PUP_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MIPI_PUP_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 4 - SCU Power-down slot control"]
    #[inline]
    pub fn pcie_pdn_slot_control(&self) -> PCIE_PDN_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCIE_PDN_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 5 - PCIE Power-up slot control"]
    #[inline]
    pub fn pcie_pup_slot_control(&self) -> PCIE_PUP_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCIE_PUP_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 6 - OTG1 Power-down slot control"]
    #[inline]
    pub fn otg1_pdn_slot_control(&self) -> OTG1_PDN_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OTG1_PDN_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 7 - OTG1 Power-up slot control"]
    #[inline]
    pub fn otg1_pup_slot_control(&self) -> OTG1_PUP_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OTG1_PUP_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 8 - OTG2 Power-down slot control"]
    #[inline]
    pub fn otg2_pdn_slot_control(&self) -> OTG2_PDN_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OTG2_PDN_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 9 - OTG2 Power-up slot control"]
    #[inline]
    pub fn otg2_pup_slot_control(&self) -> OTG2_PUP_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OTG2_PUP_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 12 - M4 Power-down slot control"]
    #[inline]
    pub fn m4_pdn_slot_control(&self) -> M4_PDN_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        M4_PDN_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 13 - M4 Power-up slot control"]
    #[inline]
    pub fn m4_pup_slot_control(&self) -> M4_PUP_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        M4_PUP_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 14 - DDR1 Power-down slot control"]
    #[inline]
    pub fn ddr1_pdn_slot_control(&self) -> DDR1_PDN_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DDR1_PDN_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 15 - DDR1 Power-up slot control"]
    #[inline]
    pub fn ddr1_pup_slot_control(&self) -> DDR1_PUP_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DDR1_PUP_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 16 - GPU_2D Power-down slot control"]
    #[inline]
    pub fn gpu_2d_pdn_slot_control(&self) -> GPU_2D_PDN_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPU_2D_PDN_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 17 - GPU_2D Power-up slot control"]
    #[inline]
    pub fn gpu_2d_pup_slot_control(&self) -> GPU_2D_PUP_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPU_2D_PUP_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 18 - GPUMIX Power-down slot control"]
    #[inline]
    pub fn gpumix_pdn_slot_control(&self) -> GPUMIX_PDN_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPUMIX_PDN_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 19 - GPUMIX Power-up slot control"]
    #[inline]
    pub fn gpumix_pup_slot_control(&self) -> GPUMIX_PUP_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPUMIX_PUP_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 20 - VPUMIX Power-down slot control"]
    #[inline]
    pub fn vpumix_pdn_slot_control(&self) -> VPUMIX_PDN_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPUMIX_PDN_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 21 - VPUMIX Power-up slot control"]
    #[inline]
    pub fn vpumix_pup_slot_control(&self) -> VPUMIX_PUP_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPUMIX_PUP_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 22 - GPU_3D Power-down slot control"]
    #[inline]
    pub fn gpu_3d_pdn_slot_control(&self) -> GPU_3D_PDN_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPU_3D_PDN_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 23 - GPU_3D Power-up slot control"]
    #[inline]
    pub fn gpu_3d_pup_slot_control(&self) -> GPU_3D_PUP_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPU_3D_PUP_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 24 - DISPMIX Power-down slot control"]
    #[inline]
    pub fn dispmix_pdn_slot_control(&self) -> DISPMIX_PDN_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISPMIX_PDN_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 25 - DISPMIX Power-up slot control"]
    #[inline]
    pub fn dispmix_pup_slot_control(&self) -> DISPMIX_PUP_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISPMIX_PUP_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 26 - VPU_G1 Power-down slot control"]
    #[inline]
    pub fn vpu_g1_pdn_slot_control(&self) -> VPU_G1_PDN_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPU_G1_PDN_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 27 - VPU_G1 Power-up slot control"]
    #[inline]
    pub fn vpu_g1_pup_slot_control(&self) -> VPU_G1_PUP_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPU_G1_PUP_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 28 - VPU_G2 Power-down slot control"]
    #[inline]
    pub fn vpu_g2_pdn_slot_control(&self) -> VPU_G2_PDN_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPU_G2_PDN_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 29 - VPU_G2 Power-up slot control"]
    #[inline]
    pub fn vpu_g2_pup_slot_control(&self) -> VPU_G2_PUP_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPU_G2_PUP_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 30 - VPU_H1 Power-down slot control"]
    #[inline]
    pub fn vpu_h1_pdn_slot_control(&self) -> VPU_H1_PDN_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPU_H1_PDN_SLOT_CONTROLR { bits }
    }
    #[doc = "Bit 31 - VPU_H1 Power-up slot control"]
    #[inline]
    pub fn vpu_h1_pup_slot_control(&self) -> VPU_H1_PUP_SLOT_CONTROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPU_H1_PUP_SLOT_CONTROLR { bits }
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
    #[doc = "Bit 0 - MF Power-down slot control"]
    #[inline]
    pub fn mf_pdn_slot_control(&mut self) -> _MF_PDN_SLOT_CONTROLW {
        _MF_PDN_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 1 - MF Power-up slot control"]
    #[inline]
    pub fn mf_pup_slot_control(&mut self) -> _MF_PUP_SLOT_CONTROLW {
        _MF_PUP_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 2 - MIPI Power-down slot control"]
    #[inline]
    pub fn mipi_pdn_slot_control(&mut self) -> _MIPI_PDN_SLOT_CONTROLW {
        _MIPI_PDN_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 3 - MIPI Power-up slot control"]
    #[inline]
    pub fn mipi_pup_slot_control(&mut self) -> _MIPI_PUP_SLOT_CONTROLW {
        _MIPI_PUP_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 4 - SCU Power-down slot control"]
    #[inline]
    pub fn pcie_pdn_slot_control(&mut self) -> _PCIE_PDN_SLOT_CONTROLW {
        _PCIE_PDN_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 5 - PCIE Power-up slot control"]
    #[inline]
    pub fn pcie_pup_slot_control(&mut self) -> _PCIE_PUP_SLOT_CONTROLW {
        _PCIE_PUP_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 6 - OTG1 Power-down slot control"]
    #[inline]
    pub fn otg1_pdn_slot_control(&mut self) -> _OTG1_PDN_SLOT_CONTROLW {
        _OTG1_PDN_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 7 - OTG1 Power-up slot control"]
    #[inline]
    pub fn otg1_pup_slot_control(&mut self) -> _OTG1_PUP_SLOT_CONTROLW {
        _OTG1_PUP_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 8 - OTG2 Power-down slot control"]
    #[inline]
    pub fn otg2_pdn_slot_control(&mut self) -> _OTG2_PDN_SLOT_CONTROLW {
        _OTG2_PDN_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 9 - OTG2 Power-up slot control"]
    #[inline]
    pub fn otg2_pup_slot_control(&mut self) -> _OTG2_PUP_SLOT_CONTROLW {
        _OTG2_PUP_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 12 - M4 Power-down slot control"]
    #[inline]
    pub fn m4_pdn_slot_control(&mut self) -> _M4_PDN_SLOT_CONTROLW {
        _M4_PDN_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 13 - M4 Power-up slot control"]
    #[inline]
    pub fn m4_pup_slot_control(&mut self) -> _M4_PUP_SLOT_CONTROLW {
        _M4_PUP_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 14 - DDR1 Power-down slot control"]
    #[inline]
    pub fn ddr1_pdn_slot_control(&mut self) -> _DDR1_PDN_SLOT_CONTROLW {
        _DDR1_PDN_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 15 - DDR1 Power-up slot control"]
    #[inline]
    pub fn ddr1_pup_slot_control(&mut self) -> _DDR1_PUP_SLOT_CONTROLW {
        _DDR1_PUP_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 16 - GPU_2D Power-down slot control"]
    #[inline]
    pub fn gpu_2d_pdn_slot_control(&mut self) -> _GPU_2D_PDN_SLOT_CONTROLW {
        _GPU_2D_PDN_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 17 - GPU_2D Power-up slot control"]
    #[inline]
    pub fn gpu_2d_pup_slot_control(&mut self) -> _GPU_2D_PUP_SLOT_CONTROLW {
        _GPU_2D_PUP_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 18 - GPUMIX Power-down slot control"]
    #[inline]
    pub fn gpumix_pdn_slot_control(&mut self) -> _GPUMIX_PDN_SLOT_CONTROLW {
        _GPUMIX_PDN_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 19 - GPUMIX Power-up slot control"]
    #[inline]
    pub fn gpumix_pup_slot_control(&mut self) -> _GPUMIX_PUP_SLOT_CONTROLW {
        _GPUMIX_PUP_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 20 - VPUMIX Power-down slot control"]
    #[inline]
    pub fn vpumix_pdn_slot_control(&mut self) -> _VPUMIX_PDN_SLOT_CONTROLW {
        _VPUMIX_PDN_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 21 - VPUMIX Power-up slot control"]
    #[inline]
    pub fn vpumix_pup_slot_control(&mut self) -> _VPUMIX_PUP_SLOT_CONTROLW {
        _VPUMIX_PUP_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 22 - GPU_3D Power-down slot control"]
    #[inline]
    pub fn gpu_3d_pdn_slot_control(&mut self) -> _GPU_3D_PDN_SLOT_CONTROLW {
        _GPU_3D_PDN_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 23 - GPU_3D Power-up slot control"]
    #[inline]
    pub fn gpu_3d_pup_slot_control(&mut self) -> _GPU_3D_PUP_SLOT_CONTROLW {
        _GPU_3D_PUP_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 24 - DISPMIX Power-down slot control"]
    #[inline]
    pub fn dispmix_pdn_slot_control(&mut self) -> _DISPMIX_PDN_SLOT_CONTROLW {
        _DISPMIX_PDN_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 25 - DISPMIX Power-up slot control"]
    #[inline]
    pub fn dispmix_pup_slot_control(&mut self) -> _DISPMIX_PUP_SLOT_CONTROLW {
        _DISPMIX_PUP_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 26 - VPU_G1 Power-down slot control"]
    #[inline]
    pub fn vpu_g1_pdn_slot_control(&mut self) -> _VPU_G1_PDN_SLOT_CONTROLW {
        _VPU_G1_PDN_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 27 - VPU_G1 Power-up slot control"]
    #[inline]
    pub fn vpu_g1_pup_slot_control(&mut self) -> _VPU_G1_PUP_SLOT_CONTROLW {
        _VPU_G1_PUP_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 28 - VPU_G2 Power-down slot control"]
    #[inline]
    pub fn vpu_g2_pdn_slot_control(&mut self) -> _VPU_G2_PDN_SLOT_CONTROLW {
        _VPU_G2_PDN_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 29 - VPU_G2 Power-up slot control"]
    #[inline]
    pub fn vpu_g2_pup_slot_control(&mut self) -> _VPU_G2_PUP_SLOT_CONTROLW {
        _VPU_G2_PUP_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 30 - VPU_H1 Power-down slot control"]
    #[inline]
    pub fn vpu_h1_pdn_slot_control(&mut self) -> _VPU_H1_PDN_SLOT_CONTROLW {
        _VPU_H1_PDN_SLOT_CONTROLW { w: self }
    }
    #[doc = "Bit 31 - VPU_H1 Power-up slot control"]
    #[inline]
    pub fn vpu_h1_pup_slot_control(&mut self) -> _VPU_H1_PUP_SLOT_CONTROLW {
        _VPU_H1_PUP_SLOT_CONTROLW { w: self }
    }
}
