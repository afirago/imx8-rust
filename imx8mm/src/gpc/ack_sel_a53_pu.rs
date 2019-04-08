#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ACK_SEL_A53_PU {
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
pub struct MF_PGC_PDN_ACKR {
    bits: bool,
}
impl MF_PGC_PDN_ACKR {
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
pub struct MIPI_PGC_PDN_ACKR {
    bits: bool,
}
impl MIPI_PGC_PDN_ACKR {
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
pub struct PCIE_PGC_PDN_ACKR {
    bits: bool,
}
impl PCIE_PGC_PDN_ACKR {
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
pub struct USB_OTG1_PGC_PDN_ACKR {
    bits: bool,
}
impl USB_OTG1_PGC_PDN_ACKR {
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
pub struct USB_OTG2_PGC_PDN_ACKR {
    bits: bool,
}
impl USB_OTG2_PGC_PDN_ACKR {
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
pub struct DDR1_PGC_PDN_ACKR {
    bits: bool,
}
impl DDR1_PGC_PDN_ACKR {
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
pub struct GPU_2D_PGC_PDN_ACKR {
    bits: bool,
}
impl GPU_2D_PGC_PDN_ACKR {
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
pub struct GPUMIX_PGC_PDN_ACKR {
    bits: bool,
}
impl GPUMIX_PGC_PDN_ACKR {
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
pub struct VPUMIX_PGC_PDN_ACKR {
    bits: bool,
}
impl VPUMIX_PGC_PDN_ACKR {
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
pub struct GPU_3D_PGC_PDN_ACKR {
    bits: bool,
}
impl GPU_3D_PGC_PDN_ACKR {
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
pub struct DISPMIX_PGC_PDN_ACKR {
    bits: bool,
}
impl DISPMIX_PGC_PDN_ACKR {
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
pub struct VPU_G1_PGC_PDN_ACKR {
    bits: bool,
}
impl VPU_G1_PGC_PDN_ACKR {
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
pub struct VPU_G2_PGC_PDN_ACKR {
    bits: bool,
}
impl VPU_G2_PGC_PDN_ACKR {
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
pub struct VPUMIX_H1_PGC_PDN_ACKR {
    bits: bool,
}
impl VPUMIX_H1_PGC_PDN_ACKR {
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
pub struct MF_PGC_PUP_ACKR {
    bits: bool,
}
impl MF_PGC_PUP_ACKR {
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
pub struct MIPI_PGC_PUP_ACKR {
    bits: bool,
}
impl MIPI_PGC_PUP_ACKR {
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
pub struct PCIE_PGC_PUP_ACKR {
    bits: bool,
}
impl PCIE_PGC_PUP_ACKR {
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
pub struct USB_OTG1_PGC_PUP_ACKR {
    bits: bool,
}
impl USB_OTG1_PGC_PUP_ACKR {
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
pub struct USB_OTG2_PGC_PUP_ACKR {
    bits: bool,
}
impl USB_OTG2_PGC_PUP_ACKR {
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
pub struct DDR1_PGC_PUP_ACKR {
    bits: bool,
}
impl DDR1_PGC_PUP_ACKR {
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
pub struct GPU_2D_PGC_PUP_ACKR {
    bits: bool,
}
impl GPU_2D_PGC_PUP_ACKR {
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
pub struct GPUMIX_PGC_PUP_ACKR {
    bits: bool,
}
impl GPUMIX_PGC_PUP_ACKR {
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
pub struct VPUMIX_PGC_PUP_ACKR {
    bits: bool,
}
impl VPUMIX_PGC_PUP_ACKR {
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
pub struct GPU_3D_PGC_PUP_ACKR {
    bits: bool,
}
impl GPU_3D_PGC_PUP_ACKR {
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
pub struct DISPMIX_PGC_PUP_ACKR {
    bits: bool,
}
impl DISPMIX_PGC_PUP_ACKR {
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
pub struct VPU_G1_PGC_PUP_ACKR {
    bits: bool,
}
impl VPU_G1_PGC_PUP_ACKR {
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
pub struct VPU_G2_PGC_PUP_ACKR {
    bits: bool,
}
impl VPU_G2_PGC_PUP_ACKR {
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
pub struct VPU_H1_PGC_PUP_ACKR {
    bits: bool,
}
impl VPU_H1_PGC_PUP_ACKR {
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
pub struct _MF_PGC_PDN_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _MF_PGC_PDN_ACKW<'a> {
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
pub struct _MIPI_PGC_PDN_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _MIPI_PGC_PDN_ACKW<'a> {
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
pub struct _PCIE_PGC_PDN_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _PCIE_PGC_PDN_ACKW<'a> {
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
pub struct _USB_OTG1_PGC_PDN_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_OTG1_PGC_PDN_ACKW<'a> {
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
pub struct _USB_OTG2_PGC_PDN_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_OTG2_PGC_PDN_ACKW<'a> {
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
pub struct _DDR1_PGC_PDN_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _DDR1_PGC_PDN_ACKW<'a> {
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
pub struct _GPU_2D_PGC_PDN_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _GPU_2D_PGC_PDN_ACKW<'a> {
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
pub struct _GPUMIX_PGC_PDN_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _GPUMIX_PGC_PDN_ACKW<'a> {
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
pub struct _VPUMIX_PGC_PDN_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _VPUMIX_PGC_PDN_ACKW<'a> {
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
pub struct _GPU_3D_PGC_PDN_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _GPU_3D_PGC_PDN_ACKW<'a> {
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
pub struct _DISPMIX_PGC_PDN_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _DISPMIX_PGC_PDN_ACKW<'a> {
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
pub struct _VPU_G1_PGC_PDN_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _VPU_G1_PGC_PDN_ACKW<'a> {
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
pub struct _VPU_G2_PGC_PDN_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _VPU_G2_PGC_PDN_ACKW<'a> {
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
pub struct _VPUMIX_H1_PGC_PDN_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _VPUMIX_H1_PGC_PDN_ACKW<'a> {
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
pub struct _MF_PGC_PUP_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _MF_PGC_PUP_ACKW<'a> {
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
pub struct _MIPI_PGC_PUP_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _MIPI_PGC_PUP_ACKW<'a> {
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
pub struct _PCIE_PGC_PUP_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _PCIE_PGC_PUP_ACKW<'a> {
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
pub struct _USB_OTG1_PGC_PUP_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_OTG1_PGC_PUP_ACKW<'a> {
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
pub struct _USB_OTG2_PGC_PUP_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_OTG2_PGC_PUP_ACKW<'a> {
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
pub struct _DDR1_PGC_PUP_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _DDR1_PGC_PUP_ACKW<'a> {
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
pub struct _GPU_2D_PGC_PUP_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _GPU_2D_PGC_PUP_ACKW<'a> {
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
pub struct _GPUMIX_PGC_PUP_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _GPUMIX_PGC_PUP_ACKW<'a> {
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
pub struct _VPUMIX_PGC_PUP_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _VPUMIX_PGC_PUP_ACKW<'a> {
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
pub struct _GPU_3D_PGC_PUP_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _GPU_3D_PGC_PUP_ACKW<'a> {
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
pub struct _DISPMIX_PGC_PUP_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _DISPMIX_PGC_PUP_ACKW<'a> {
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
pub struct _VPU_G1_PGC_PUP_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _VPU_G1_PGC_PUP_ACKW<'a> {
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
pub struct _VPU_G2_PGC_PUP_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _VPU_G2_PGC_PUP_ACKW<'a> {
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
pub struct _VPU_H1_PGC_PUP_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _VPU_H1_PGC_PUP_ACKW<'a> {
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
    #[doc = "Bit 0 - Select power down acknowledge signal of MIX PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn mf_pgc_pdn_ack(&self) -> MF_PGC_PDN_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MF_PGC_PDN_ACKR { bits }
    }
    #[doc = "Bit 2 - Select power down acknowledge signal of MIPI PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn mipi_pgc_pdn_ack(&self) -> MIPI_PGC_PDN_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MIPI_PGC_PDN_ACKR { bits }
    }
    #[doc = "Bit 3 - Select power down acknowledge signal of PCIE PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn pcie_pgc_pdn_ack(&self) -> PCIE_PGC_PDN_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCIE_PGC_PDN_ACKR { bits }
    }
    #[doc = "Bit 4 - Select power down acknowledge signal of USB_OTG1 PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn usb_otg1_pgc_pdn_ack(&self) -> USB_OTG1_PGC_PDN_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USB_OTG1_PGC_PDN_ACKR { bits }
    }
    #[doc = "Bit 5 - Select power down acknowledge signal of USB_OTG2 PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn usb_otg2_pgc_pdn_ack(&self) -> USB_OTG2_PGC_PDN_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USB_OTG2_PGC_PDN_ACKR { bits }
    }
    #[doc = "Bit 7 - Select power down acknowledge signal of DDR1 PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn ddr1_pgc_pdn_ack(&self) -> DDR1_PGC_PDN_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DDR1_PGC_PDN_ACKR { bits }
    }
    #[doc = "Bit 8 - Select power down acknowledge signal of GPU_2D PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn gpu_2d_pgc_pdn_ack(&self) -> GPU_2D_PGC_PDN_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPU_2D_PGC_PDN_ACKR { bits }
    }
    #[doc = "Bit 9 - Select power down acknowledge signal of GPUMIX PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn gpumix_pgc_pdn_ack(&self) -> GPUMIX_PGC_PDN_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPUMIX_PGC_PDN_ACKR { bits }
    }
    #[doc = "Bit 10 - Select power down acknowledge signal of VPUMIX PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn vpumix_pgc_pdn_ack(&self) -> VPUMIX_PGC_PDN_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPUMIX_PGC_PDN_ACKR { bits }
    }
    #[doc = "Bit 11 - Select power down acknowledge signal of GPU_3D PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn gpu_3d_pgc_pdn_ack(&self) -> GPU_3D_PGC_PDN_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPU_3D_PGC_PDN_ACKR { bits }
    }
    #[doc = "Bit 12 - Select power down acknowledge signal of DISPMIX PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn dispmix_pgc_pdn_ack(&self) -> DISPMIX_PGC_PDN_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISPMIX_PGC_PDN_ACKR { bits }
    }
    #[doc = "Bit 13 - Select power down acknowledge signal of VPU_G1 PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn vpu_g1_pgc_pdn_ack(&self) -> VPU_G1_PGC_PDN_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPU_G1_PGC_PDN_ACKR { bits }
    }
    #[doc = "Bit 14 - Select power down acknowledge signal of VPU_G2 PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn vpu_g2_pgc_pdn_ack(&self) -> VPU_G2_PGC_PDN_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPU_G2_PGC_PDN_ACKR { bits }
    }
    #[doc = "Bit 15 - Select power down acknowledge signal of VPUMIX_H1 PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn vpumix_h1_pgc_pdn_ack(&self) -> VPUMIX_H1_PGC_PDN_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPUMIX_H1_PGC_PDN_ACKR { bits }
    }
    #[doc = "Bit 16 - Select power down acknowledge signal of MIX PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn mf_pgc_pup_ack(&self) -> MF_PGC_PUP_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MF_PGC_PUP_ACKR { bits }
    }
    #[doc = "Bit 18 - Select power down acknowledge signal of MIPI PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn mipi_pgc_pup_ack(&self) -> MIPI_PGC_PUP_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MIPI_PGC_PUP_ACKR { bits }
    }
    #[doc = "Bit 19 - Select power down acknowledge signal of PCIE PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn pcie_pgc_pup_ack(&self) -> PCIE_PGC_PUP_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCIE_PGC_PUP_ACKR { bits }
    }
    #[doc = "Bit 20 - Select power down acknowledge signal of USB_OTG1 PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn usb_otg1_pgc_pup_ack(&self) -> USB_OTG1_PGC_PUP_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USB_OTG1_PGC_PUP_ACKR { bits }
    }
    #[doc = "Bit 21 - Select power down acknowledge signal of USB_OTG2 PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn usb_otg2_pgc_pup_ack(&self) -> USB_OTG2_PGC_PUP_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USB_OTG2_PGC_PUP_ACKR { bits }
    }
    #[doc = "Bit 23 - Select power down acknowledge signal of DDR1 PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn ddr1_pgc_pup_ack(&self) -> DDR1_PGC_PUP_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DDR1_PGC_PUP_ACKR { bits }
    }
    #[doc = "Bit 24 - Select power down acknowledge signal of GPU_2D PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn gpu_2d_pgc_pup_ack(&self) -> GPU_2D_PGC_PUP_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPU_2D_PGC_PUP_ACKR { bits }
    }
    #[doc = "Bit 25 - Select power down acknowledge signal of GPUMIX PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn gpumix_pgc_pup_ack(&self) -> GPUMIX_PGC_PUP_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPUMIX_PGC_PUP_ACKR { bits }
    }
    #[doc = "Bit 26 - Select power down acknowledge signal of VPUMIX PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn vpumix_pgc_pup_ack(&self) -> VPUMIX_PGC_PUP_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPUMIX_PGC_PUP_ACKR { bits }
    }
    #[doc = "Bit 27 - Select power down acknowledge signal of GPU_3D PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn gpu_3d_pgc_pup_ack(&self) -> GPU_3D_PGC_PUP_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPU_3D_PGC_PUP_ACKR { bits }
    }
    #[doc = "Bit 28 - Select power down acknowledge signal of DISPMIX PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn dispmix_pgc_pup_ack(&self) -> DISPMIX_PGC_PUP_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISPMIX_PGC_PUP_ACKR { bits }
    }
    #[doc = "Bit 29 - Select power down acknowledge signal of VPU_G1 PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn vpu_g1_pgc_pup_ack(&self) -> VPU_G1_PGC_PUP_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPU_G1_PGC_PUP_ACKR { bits }
    }
    #[doc = "Bit 30 - Select power down acknowledge signal of VPU_G2 PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn vpu_g2_pgc_pup_ack(&self) -> VPU_G2_PGC_PUP_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPU_G2_PGC_PUP_ACKR { bits }
    }
    #[doc = "Bit 31 - Select power down acknowledge signal of VPU_H1 PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn vpu_h1_pgc_pup_ack(&self) -> VPU_H1_PGC_PUP_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPU_H1_PGC_PUP_ACKR { bits }
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
    #[doc = "Bit 0 - Select power down acknowledge signal of MIX PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn mf_pgc_pdn_ack(&mut self) -> _MF_PGC_PDN_ACKW {
        _MF_PGC_PDN_ACKW { w: self }
    }
    #[doc = "Bit 2 - Select power down acknowledge signal of MIPI PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn mipi_pgc_pdn_ack(&mut self) -> _MIPI_PGC_PDN_ACKW {
        _MIPI_PGC_PDN_ACKW { w: self }
    }
    #[doc = "Bit 3 - Select power down acknowledge signal of PCIE PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn pcie_pgc_pdn_ack(&mut self) -> _PCIE_PGC_PDN_ACKW {
        _PCIE_PGC_PDN_ACKW { w: self }
    }
    #[doc = "Bit 4 - Select power down acknowledge signal of USB_OTG1 PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn usb_otg1_pgc_pdn_ack(&mut self) -> _USB_OTG1_PGC_PDN_ACKW {
        _USB_OTG1_PGC_PDN_ACKW { w: self }
    }
    #[doc = "Bit 5 - Select power down acknowledge signal of USB_OTG2 PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn usb_otg2_pgc_pdn_ack(&mut self) -> _USB_OTG2_PGC_PDN_ACKW {
        _USB_OTG2_PGC_PDN_ACKW { w: self }
    }
    #[doc = "Bit 7 - Select power down acknowledge signal of DDR1 PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn ddr1_pgc_pdn_ack(&mut self) -> _DDR1_PGC_PDN_ACKW {
        _DDR1_PGC_PDN_ACKW { w: self }
    }
    #[doc = "Bit 8 - Select power down acknowledge signal of GPU_2D PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn gpu_2d_pgc_pdn_ack(&mut self) -> _GPU_2D_PGC_PDN_ACKW {
        _GPU_2D_PGC_PDN_ACKW { w: self }
    }
    #[doc = "Bit 9 - Select power down acknowledge signal of GPUMIX PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn gpumix_pgc_pdn_ack(&mut self) -> _GPUMIX_PGC_PDN_ACKW {
        _GPUMIX_PGC_PDN_ACKW { w: self }
    }
    #[doc = "Bit 10 - Select power down acknowledge signal of VPUMIX PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn vpumix_pgc_pdn_ack(&mut self) -> _VPUMIX_PGC_PDN_ACKW {
        _VPUMIX_PGC_PDN_ACKW { w: self }
    }
    #[doc = "Bit 11 - Select power down acknowledge signal of GPU_3D PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn gpu_3d_pgc_pdn_ack(&mut self) -> _GPU_3D_PGC_PDN_ACKW {
        _GPU_3D_PGC_PDN_ACKW { w: self }
    }
    #[doc = "Bit 12 - Select power down acknowledge signal of DISPMIX PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn dispmix_pgc_pdn_ack(&mut self) -> _DISPMIX_PGC_PDN_ACKW {
        _DISPMIX_PGC_PDN_ACKW { w: self }
    }
    #[doc = "Bit 13 - Select power down acknowledge signal of VPU_G1 PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn vpu_g1_pgc_pdn_ack(&mut self) -> _VPU_G1_PGC_PDN_ACKW {
        _VPU_G1_PGC_PDN_ACKW { w: self }
    }
    #[doc = "Bit 14 - Select power down acknowledge signal of VPU_G2 PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn vpu_g2_pgc_pdn_ack(&mut self) -> _VPU_G2_PGC_PDN_ACKW {
        _VPU_G2_PGC_PDN_ACKW { w: self }
    }
    #[doc = "Bit 15 - Select power down acknowledge signal of VPUMIX_H1 PGC as the power down acknowledge for A53 LPM."]
    #[inline]
    pub fn vpumix_h1_pgc_pdn_ack(&mut self) -> _VPUMIX_H1_PGC_PDN_ACKW {
        _VPUMIX_H1_PGC_PDN_ACKW { w: self }
    }
    #[doc = "Bit 16 - Select power down acknowledge signal of MIX PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn mf_pgc_pup_ack(&mut self) -> _MF_PGC_PUP_ACKW {
        _MF_PGC_PUP_ACKW { w: self }
    }
    #[doc = "Bit 18 - Select power down acknowledge signal of MIPI PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn mipi_pgc_pup_ack(&mut self) -> _MIPI_PGC_PUP_ACKW {
        _MIPI_PGC_PUP_ACKW { w: self }
    }
    #[doc = "Bit 19 - Select power down acknowledge signal of PCIE PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn pcie_pgc_pup_ack(&mut self) -> _PCIE_PGC_PUP_ACKW {
        _PCIE_PGC_PUP_ACKW { w: self }
    }
    #[doc = "Bit 20 - Select power down acknowledge signal of USB_OTG1 PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn usb_otg1_pgc_pup_ack(&mut self) -> _USB_OTG1_PGC_PUP_ACKW {
        _USB_OTG1_PGC_PUP_ACKW { w: self }
    }
    #[doc = "Bit 21 - Select power down acknowledge signal of USB_OTG2 PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn usb_otg2_pgc_pup_ack(&mut self) -> _USB_OTG2_PGC_PUP_ACKW {
        _USB_OTG2_PGC_PUP_ACKW { w: self }
    }
    #[doc = "Bit 23 - Select power down acknowledge signal of DDR1 PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn ddr1_pgc_pup_ack(&mut self) -> _DDR1_PGC_PUP_ACKW {
        _DDR1_PGC_PUP_ACKW { w: self }
    }
    #[doc = "Bit 24 - Select power down acknowledge signal of GPU_2D PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn gpu_2d_pgc_pup_ack(&mut self) -> _GPU_2D_PGC_PUP_ACKW {
        _GPU_2D_PGC_PUP_ACKW { w: self }
    }
    #[doc = "Bit 25 - Select power down acknowledge signal of GPUMIX PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn gpumix_pgc_pup_ack(&mut self) -> _GPUMIX_PGC_PUP_ACKW {
        _GPUMIX_PGC_PUP_ACKW { w: self }
    }
    #[doc = "Bit 26 - Select power down acknowledge signal of VPUMIX PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn vpumix_pgc_pup_ack(&mut self) -> _VPUMIX_PGC_PUP_ACKW {
        _VPUMIX_PGC_PUP_ACKW { w: self }
    }
    #[doc = "Bit 27 - Select power down acknowledge signal of GPU_3D PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn gpu_3d_pgc_pup_ack(&mut self) -> _GPU_3D_PGC_PUP_ACKW {
        _GPU_3D_PGC_PUP_ACKW { w: self }
    }
    #[doc = "Bit 28 - Select power down acknowledge signal of DISPMIX PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn dispmix_pgc_pup_ack(&mut self) -> _DISPMIX_PGC_PUP_ACKW {
        _DISPMIX_PGC_PUP_ACKW { w: self }
    }
    #[doc = "Bit 29 - Select power down acknowledge signal of VPU_G1 PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn vpu_g1_pgc_pup_ack(&mut self) -> _VPU_G1_PGC_PUP_ACKW {
        _VPU_G1_PGC_PUP_ACKW { w: self }
    }
    #[doc = "Bit 30 - Select power down acknowledge signal of VPU_G2 PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn vpu_g2_pgc_pup_ack(&mut self) -> _VPU_G2_PGC_PUP_ACKW {
        _VPU_G2_PGC_PUP_ACKW { w: self }
    }
    #[doc = "Bit 31 - Select power down acknowledge signal of VPU_H1 PGC as the power up acknowledge for A53 LPM."]
    #[inline]
    pub fn vpu_h1_pgc_pup_ack(&mut self) -> _VPU_H1_PGC_PUP_ACKW {
        _VPU_H1_PGC_PUP_ACKW { w: self }
    }
}
