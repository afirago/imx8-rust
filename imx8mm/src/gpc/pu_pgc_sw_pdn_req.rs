#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PU_PGC_SW_PDN_REQ {
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
pub struct MIPI_DSI_SW_PDN_REQR {
    bits: bool,
}
impl MIPI_DSI_SW_PDN_REQR {
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
pub struct PCIE_SW_PDN_REQR {
    bits: bool,
}
impl PCIE_SW_PDN_REQR {
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
pub struct USB_OTG1_SW_PDN_REQR {
    bits: bool,
}
impl USB_OTG1_SW_PDN_REQR {
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
pub struct USB_OTG2_SW_PDN_REQR {
    bits: bool,
}
impl USB_OTG2_SW_PDN_REQR {
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
pub struct DDR1_SW_PDN_REQR {
    bits: bool,
}
impl DDR1_SW_PDN_REQR {
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
pub struct GPU_2D_SW_PDN_REQR {
    bits: bool,
}
impl GPU_2D_SW_PDN_REQR {
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
pub struct GPUMIX_SW_PDN_REQR {
    bits: bool,
}
impl GPUMIX_SW_PDN_REQR {
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
pub struct VPUMIX_SW_PDN_REQR {
    bits: bool,
}
impl VPUMIX_SW_PDN_REQR {
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
pub struct GPU_3D_SW_PDN_REQR {
    bits: bool,
}
impl GPU_3D_SW_PDN_REQR {
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
pub struct DISPMIX_SW_PDN_REQR {
    bits: bool,
}
impl DISPMIX_SW_PDN_REQR {
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
pub struct VPU_G1_SW_PDN_REQR {
    bits: bool,
}
impl VPU_G1_SW_PDN_REQR {
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
pub struct VPU_G2_SW_PDN_REQR {
    bits: bool,
}
impl VPU_G2_SW_PDN_REQR {
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
pub struct VPU_H1_SW_PDN_REQR {
    bits: bool,
}
impl VPU_H1_SW_PDN_REQR {
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
pub struct _MIPI_DSI_SW_PDN_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _MIPI_DSI_SW_PDN_REQW<'a> {
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
pub struct _PCIE_SW_PDN_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _PCIE_SW_PDN_REQW<'a> {
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
pub struct _USB_OTG1_SW_PDN_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_OTG1_SW_PDN_REQW<'a> {
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
pub struct _USB_OTG2_SW_PDN_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_OTG2_SW_PDN_REQW<'a> {
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
pub struct _DDR1_SW_PDN_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _DDR1_SW_PDN_REQW<'a> {
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
pub struct _GPU_2D_SW_PDN_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _GPU_2D_SW_PDN_REQW<'a> {
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
pub struct _GPUMIX_SW_PDN_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _GPUMIX_SW_PDN_REQW<'a> {
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
pub struct _VPUMIX_SW_PDN_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _VPUMIX_SW_PDN_REQW<'a> {
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
pub struct _GPU_3D_SW_PDN_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _GPU_3D_SW_PDN_REQW<'a> {
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
pub struct _DISPMIX_SW_PDN_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _DISPMIX_SW_PDN_REQW<'a> {
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
pub struct _VPU_G1_SW_PDN_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _VPU_G1_SW_PDN_REQW<'a> {
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
pub struct _VPU_G2_SW_PDN_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _VPU_G2_SW_PDN_REQW<'a> {
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
pub struct _VPU_H1_SW_PDN_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _VPU_H1_SW_PDN_REQW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Software power down trigger for MIPI_DSI"]
    #[inline]
    pub fn mipi_dsi_sw_pdn_req(&self) -> MIPI_DSI_SW_PDN_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MIPI_DSI_SW_PDN_REQR { bits }
    }
    #[doc = "Bit 1 - Software power down trigger for PCIE"]
    #[inline]
    pub fn pcie_sw_pdn_req(&self) -> PCIE_SW_PDN_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCIE_SW_PDN_REQR { bits }
    }
    #[doc = "Bit 2 - Software power down trigger for USB_OTG1"]
    #[inline]
    pub fn usb_otg1_sw_pdn_req(&self) -> USB_OTG1_SW_PDN_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USB_OTG1_SW_PDN_REQR { bits }
    }
    #[doc = "Bit 3 - Software power down trigger for USB_OTG2"]
    #[inline]
    pub fn usb_otg2_sw_pdn_req(&self) -> USB_OTG2_SW_PDN_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USB_OTG2_SW_PDN_REQR { bits }
    }
    #[doc = "Bit 5 - Software power down trigger for DDR1"]
    #[inline]
    pub fn ddr1_sw_pdn_req(&self) -> DDR1_SW_PDN_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DDR1_SW_PDN_REQR { bits }
    }
    #[doc = "Bit 6 - Software power down trigger for GPU_2D"]
    #[inline]
    pub fn gpu_2d_sw_pdn_req(&self) -> GPU_2D_SW_PDN_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPU_2D_SW_PDN_REQR { bits }
    }
    #[doc = "Bit 7 - Software power down trigger for GPUMIX"]
    #[inline]
    pub fn gpumix_sw_pdn_req(&self) -> GPUMIX_SW_PDN_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPUMIX_SW_PDN_REQR { bits }
    }
    #[doc = "Bit 8 - Software power down trigger for VPUMIX"]
    #[inline]
    pub fn vpumix_sw_pdn_req(&self) -> VPUMIX_SW_PDN_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPUMIX_SW_PDN_REQR { bits }
    }
    #[doc = "Bit 9 - Software power down trigger for GPU_3D"]
    #[inline]
    pub fn gpu_3d_sw_pdn_req(&self) -> GPU_3D_SW_PDN_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPU_3D_SW_PDN_REQR { bits }
    }
    #[doc = "Bit 10 - Software power down trigger for DISPMIX"]
    #[inline]
    pub fn dispmix_sw_pdn_req(&self) -> DISPMIX_SW_PDN_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISPMIX_SW_PDN_REQR { bits }
    }
    #[doc = "Bit 11 - Software power down trigger for VPU_G1"]
    #[inline]
    pub fn vpu_g1_sw_pdn_req(&self) -> VPU_G1_SW_PDN_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPU_G1_SW_PDN_REQR { bits }
    }
    #[doc = "Bit 12 - Software power down trigger for VPU_G2"]
    #[inline]
    pub fn vpu_g2_sw_pdn_req(&self) -> VPU_G2_SW_PDN_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPU_G2_SW_PDN_REQR { bits }
    }
    #[doc = "Bit 13 - Software power down trigger for VPU_H1"]
    #[inline]
    pub fn vpu_h1_sw_pdn_req(&self) -> VPU_H1_SW_PDN_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPU_H1_SW_PDN_REQR { bits }
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
    #[doc = "Bit 0 - Software power down trigger for MIPI_DSI"]
    #[inline]
    pub fn mipi_dsi_sw_pdn_req(&mut self) -> _MIPI_DSI_SW_PDN_REQW {
        _MIPI_DSI_SW_PDN_REQW { w: self }
    }
    #[doc = "Bit 1 - Software power down trigger for PCIE"]
    #[inline]
    pub fn pcie_sw_pdn_req(&mut self) -> _PCIE_SW_PDN_REQW {
        _PCIE_SW_PDN_REQW { w: self }
    }
    #[doc = "Bit 2 - Software power down trigger for USB_OTG1"]
    #[inline]
    pub fn usb_otg1_sw_pdn_req(&mut self) -> _USB_OTG1_SW_PDN_REQW {
        _USB_OTG1_SW_PDN_REQW { w: self }
    }
    #[doc = "Bit 3 - Software power down trigger for USB_OTG2"]
    #[inline]
    pub fn usb_otg2_sw_pdn_req(&mut self) -> _USB_OTG2_SW_PDN_REQW {
        _USB_OTG2_SW_PDN_REQW { w: self }
    }
    #[doc = "Bit 5 - Software power down trigger for DDR1"]
    #[inline]
    pub fn ddr1_sw_pdn_req(&mut self) -> _DDR1_SW_PDN_REQW {
        _DDR1_SW_PDN_REQW { w: self }
    }
    #[doc = "Bit 6 - Software power down trigger for GPU_2D"]
    #[inline]
    pub fn gpu_2d_sw_pdn_req(&mut self) -> _GPU_2D_SW_PDN_REQW {
        _GPU_2D_SW_PDN_REQW { w: self }
    }
    #[doc = "Bit 7 - Software power down trigger for GPUMIX"]
    #[inline]
    pub fn gpumix_sw_pdn_req(&mut self) -> _GPUMIX_SW_PDN_REQW {
        _GPUMIX_SW_PDN_REQW { w: self }
    }
    #[doc = "Bit 8 - Software power down trigger for VPUMIX"]
    #[inline]
    pub fn vpumix_sw_pdn_req(&mut self) -> _VPUMIX_SW_PDN_REQW {
        _VPUMIX_SW_PDN_REQW { w: self }
    }
    #[doc = "Bit 9 - Software power down trigger for GPU_3D"]
    #[inline]
    pub fn gpu_3d_sw_pdn_req(&mut self) -> _GPU_3D_SW_PDN_REQW {
        _GPU_3D_SW_PDN_REQW { w: self }
    }
    #[doc = "Bit 10 - Software power down trigger for DISPMIX"]
    #[inline]
    pub fn dispmix_sw_pdn_req(&mut self) -> _DISPMIX_SW_PDN_REQW {
        _DISPMIX_SW_PDN_REQW { w: self }
    }
    #[doc = "Bit 11 - Software power down trigger for VPU_G1"]
    #[inline]
    pub fn vpu_g1_sw_pdn_req(&mut self) -> _VPU_G1_SW_PDN_REQW {
        _VPU_G1_SW_PDN_REQW { w: self }
    }
    #[doc = "Bit 12 - Software power down trigger for VPU_G2"]
    #[inline]
    pub fn vpu_g2_sw_pdn_req(&mut self) -> _VPU_G2_SW_PDN_REQW {
        _VPU_G2_SW_PDN_REQW { w: self }
    }
    #[doc = "Bit 13 - Software power down trigger for VPU_H1"]
    #[inline]
    pub fn vpu_h1_sw_pdn_req(&mut self) -> _VPU_H1_SW_PDN_REQW {
        _VPU_H1_SW_PDN_REQW { w: self }
    }
}
