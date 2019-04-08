#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::M4_PU_PGC_PUP_STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct M4_MIPI_PUP_STATUSR {
    bits: bool,
}
impl M4_MIPI_PUP_STATUSR {
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
pub struct M4_PCIE_PUP_STATUSR {
    bits: bool,
}
impl M4_PCIE_PUP_STATUSR {
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
pub struct M4_OTG1_PUP_STATUSR {
    bits: bool,
}
impl M4_OTG1_PUP_STATUSR {
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
pub struct M4_OTG2_PUP_STATUSR {
    bits: bool,
}
impl M4_OTG2_PUP_STATUSR {
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
pub struct M4_DDR1_PUP_STATUSR {
    bits: bool,
}
impl M4_DDR1_PUP_STATUSR {
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
pub struct M4_GPU_2D_PUP_STATUSR {
    bits: bool,
}
impl M4_GPU_2D_PUP_STATUSR {
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
pub struct M4_GPUMIX_PUP_STATUSR {
    bits: bool,
}
impl M4_GPUMIX_PUP_STATUSR {
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
pub struct M4_VPUMIX_PUP_STATUSR {
    bits: bool,
}
impl M4_VPUMIX_PUP_STATUSR {
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
pub struct M4_GPU_3D_PUP_STATUSR {
    bits: bool,
}
impl M4_GPU_3D_PUP_STATUSR {
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
pub struct M4_DISPMIX_PUP_STATUSR {
    bits: bool,
}
impl M4_DISPMIX_PUP_STATUSR {
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
pub struct M4_VPU_G1_PUP_STATUSR {
    bits: bool,
}
impl M4_VPU_G1_PUP_STATUSR {
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
pub struct M4_VPU_G2_PUP_STATUSR {
    bits: bool,
}
impl M4_VPU_G2_PUP_STATUSR {
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
pub struct M4_VPU_H1_PUP_STATUSR {
    bits: bool,
}
impl M4_VPU_H1_PUP_STATUSR {
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
    pub fn m4_mipi_pup_status(&self) -> M4_MIPI_PUP_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        M4_MIPI_PUP_STATUSR { bits }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline]
    pub fn m4_pcie_pup_status(&self) -> M4_PCIE_PUP_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        M4_PCIE_PUP_STATUSR { bits }
    }
    #[doc = "Bit 2 - no description available"]
    #[inline]
    pub fn m4_otg1_pup_status(&self) -> M4_OTG1_PUP_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        M4_OTG1_PUP_STATUSR { bits }
    }
    #[doc = "Bit 3 - no description available"]
    #[inline]
    pub fn m4_otg2_pup_status(&self) -> M4_OTG2_PUP_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        M4_OTG2_PUP_STATUSR { bits }
    }
    #[doc = "Bit 5 - no description available"]
    #[inline]
    pub fn m4_ddr1_pup_status(&self) -> M4_DDR1_PUP_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        M4_DDR1_PUP_STATUSR { bits }
    }
    #[doc = "Bit 6 - no description available"]
    #[inline]
    pub fn m4_gpu_2d_pup_status(&self) -> M4_GPU_2D_PUP_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        M4_GPU_2D_PUP_STATUSR { bits }
    }
    #[doc = "Bit 7 - no description available"]
    #[inline]
    pub fn m4_gpumix_pup_status(&self) -> M4_GPUMIX_PUP_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        M4_GPUMIX_PUP_STATUSR { bits }
    }
    #[doc = "Bit 8 - no description available"]
    #[inline]
    pub fn m4_vpumix_pup_status(&self) -> M4_VPUMIX_PUP_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        M4_VPUMIX_PUP_STATUSR { bits }
    }
    #[doc = "Bit 9 - no description available"]
    #[inline]
    pub fn m4_gpu_3d_pup_status(&self) -> M4_GPU_3D_PUP_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        M4_GPU_3D_PUP_STATUSR { bits }
    }
    #[doc = "Bit 10 - no description available"]
    #[inline]
    pub fn m4_dispmix_pup_status(&self) -> M4_DISPMIX_PUP_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        M4_DISPMIX_PUP_STATUSR { bits }
    }
    #[doc = "Bit 11 - no description available"]
    #[inline]
    pub fn m4_vpu_g1_pup_status(&self) -> M4_VPU_G1_PUP_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        M4_VPU_G1_PUP_STATUSR { bits }
    }
    #[doc = "Bit 12 - no description available"]
    #[inline]
    pub fn m4_vpu_g2_pup_status(&self) -> M4_VPU_G2_PUP_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        M4_VPU_G2_PUP_STATUSR { bits }
    }
    #[doc = "Bit 13 - no description available"]
    #[inline]
    pub fn m4_vpu_h1_pup_status(&self) -> M4_VPU_H1_PUP_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        M4_VPU_H1_PUP_STATUSR { bits }
    }
}
