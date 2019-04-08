#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::A53_PU_PGC_PDN_STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct A53_MIPI_PDN_STATUSR {
    bits: bool,
}
impl A53_MIPI_PDN_STATUSR {
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
pub struct A53_PCIE_PDN_STATUSR {
    bits: bool,
}
impl A53_PCIE_PDN_STATUSR {
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
pub struct A53_OTG1_PDN_STATUSR {
    bits: bool,
}
impl A53_OTG1_PDN_STATUSR {
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
pub struct A53_OTG2_PDN_STATUSR {
    bits: bool,
}
impl A53_OTG2_PDN_STATUSR {
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
pub struct A53_DDR1_PDN_STATUSR {
    bits: bool,
}
impl A53_DDR1_PDN_STATUSR {
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
pub struct A53_GPU_2D_PDN_STATUSR {
    bits: bool,
}
impl A53_GPU_2D_PDN_STATUSR {
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
pub struct A53_GPUMIX_PDN_STATUSR {
    bits: bool,
}
impl A53_GPUMIX_PDN_STATUSR {
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
pub struct A53_VPUMIX_PDN_STATUSR {
    bits: bool,
}
impl A53_VPUMIX_PDN_STATUSR {
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
pub struct A53_GPU_3D_PDN_STATUSR {
    bits: bool,
}
impl A53_GPU_3D_PDN_STATUSR {
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
pub struct A53_DISPMIX_PDN_STATUSR {
    bits: bool,
}
impl A53_DISPMIX_PDN_STATUSR {
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
pub struct A53_VPU_G1_PDN_STATUSR {
    bits: bool,
}
impl A53_VPU_G1_PDN_STATUSR {
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
pub struct A53_VPU_G2_PDN_STATUSR {
    bits: bool,
}
impl A53_VPU_G2_PDN_STATUSR {
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
pub struct A53_VPU_H1_PDN_STATUSR {
    bits: bool,
}
impl A53_VPU_H1_PDN_STATUSR {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - no description available"]
    #[inline]
    pub fn a53_mipi_pdn_status(&self) -> A53_MIPI_PDN_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_MIPI_PDN_STATUSR { bits }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline]
    pub fn a53_pcie_pdn_status(&self) -> A53_PCIE_PDN_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_PCIE_PDN_STATUSR { bits }
    }
    #[doc = "Bit 2 - no description available"]
    #[inline]
    pub fn a53_otg1_pdn_status(&self) -> A53_OTG1_PDN_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_OTG1_PDN_STATUSR { bits }
    }
    #[doc = "Bit 3 - no description available"]
    #[inline]
    pub fn a53_otg2_pdn_status(&self) -> A53_OTG2_PDN_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_OTG2_PDN_STATUSR { bits }
    }
    #[doc = "Bit 5 - no description available"]
    #[inline]
    pub fn a53_ddr1_pdn_status(&self) -> A53_DDR1_PDN_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_DDR1_PDN_STATUSR { bits }
    }
    #[doc = "Bit 6 - no description available"]
    #[inline]
    pub fn a53_gpu_2d_pdn_status(&self) -> A53_GPU_2D_PDN_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_GPU_2D_PDN_STATUSR { bits }
    }
    #[doc = "Bit 7 - no description available"]
    #[inline]
    pub fn a53_gpumix_pdn_status(&self) -> A53_GPUMIX_PDN_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_GPUMIX_PDN_STATUSR { bits }
    }
    #[doc = "Bit 8 - no description available"]
    #[inline]
    pub fn a53_vpumix_pdn_status(&self) -> A53_VPUMIX_PDN_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_VPUMIX_PDN_STATUSR { bits }
    }
    #[doc = "Bit 9 - no description available"]
    #[inline]
    pub fn a53_gpu_3d_pdn_status(&self) -> A53_GPU_3D_PDN_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_GPU_3D_PDN_STATUSR { bits }
    }
    #[doc = "Bit 10 - no description available"]
    #[inline]
    pub fn a53_dispmix_pdn_status(&self) -> A53_DISPMIX_PDN_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_DISPMIX_PDN_STATUSR { bits }
    }
    #[doc = "Bit 11 - no description available"]
    #[inline]
    pub fn a53_vpu_g1_pdn_status(&self) -> A53_VPU_G1_PDN_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_VPU_G1_PDN_STATUSR { bits }
    }
    #[doc = "Bit 12 - no description available"]
    #[inline]
    pub fn a53_vpu_g2_pdn_status(&self) -> A53_VPU_G2_PDN_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_VPU_G2_PDN_STATUSR { bits }
    }
    #[doc = "Bit 13 - no description available"]
    #[inline]
    pub fn a53_vpu_h1_pdn_status(&self) -> A53_VPU_H1_PDN_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_VPU_H1_PDN_STATUSR { bits }
    }
}
