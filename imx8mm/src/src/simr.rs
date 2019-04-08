#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SIMR {
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
#[doc = "Possible values of the field `MASK_OTGPHY1_PASSED_RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_OTGPHY1_PASSED_RESETR {
    #[doc = "do not mask interrupt due to OTG PHY1 passed reset - interrupt will be created"]
    MASK_OTGPHY1_PASSED_RESET_0,
    #[doc = "mask interrupt due to OTG PHY1 passed reset"]
    MASK_OTGPHY1_PASSED_RESET_1,
}
impl MASK_OTGPHY1_PASSED_RESETR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MASK_OTGPHY1_PASSED_RESETR::MASK_OTGPHY1_PASSED_RESET_0 => false,
            MASK_OTGPHY1_PASSED_RESETR::MASK_OTGPHY1_PASSED_RESET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_OTGPHY1_PASSED_RESETR {
        match value {
            false => MASK_OTGPHY1_PASSED_RESETR::MASK_OTGPHY1_PASSED_RESET_0,
            true => MASK_OTGPHY1_PASSED_RESETR::MASK_OTGPHY1_PASSED_RESET_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_OTGPHY1_PASSED_RESET_0`"]
    #[inline]
    pub fn is_mask_otgphy1_passed_reset_0(&self) -> bool {
        *self == MASK_OTGPHY1_PASSED_RESETR::MASK_OTGPHY1_PASSED_RESET_0
    }
    #[doc = "Checks if the value of the field is `MASK_OTGPHY1_PASSED_RESET_1`"]
    #[inline]
    pub fn is_mask_otgphy1_passed_reset_1(&self) -> bool {
        *self == MASK_OTGPHY1_PASSED_RESETR::MASK_OTGPHY1_PASSED_RESET_1
    }
}
#[doc = "Possible values of the field `MASK_OTGPHY2_PASSED_RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_OTGPHY2_PASSED_RESETR {
    #[doc = "do not mask interrupt due to OTG PHY2 passed reset - interrupt will be created"]
    MASK_OTGPHY2_PASSED_RESET_0,
    #[doc = "mask interrupt due to OTG PHY2 passed reset"]
    MASK_OTGPHY2_PASSED_RESET_1,
}
impl MASK_OTGPHY2_PASSED_RESETR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MASK_OTGPHY2_PASSED_RESETR::MASK_OTGPHY2_PASSED_RESET_0 => false,
            MASK_OTGPHY2_PASSED_RESETR::MASK_OTGPHY2_PASSED_RESET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_OTGPHY2_PASSED_RESETR {
        match value {
            false => MASK_OTGPHY2_PASSED_RESETR::MASK_OTGPHY2_PASSED_RESET_0,
            true => MASK_OTGPHY2_PASSED_RESETR::MASK_OTGPHY2_PASSED_RESET_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_OTGPHY2_PASSED_RESET_0`"]
    #[inline]
    pub fn is_mask_otgphy2_passed_reset_0(&self) -> bool {
        *self == MASK_OTGPHY2_PASSED_RESETR::MASK_OTGPHY2_PASSED_RESET_0
    }
    #[doc = "Checks if the value of the field is `MASK_OTGPHY2_PASSED_RESET_1`"]
    #[inline]
    pub fn is_mask_otgphy2_passed_reset_1(&self) -> bool {
        *self == MASK_OTGPHY2_PASSED_RESETR::MASK_OTGPHY2_PASSED_RESET_1
    }
}
#[doc = "Possible values of the field `MASK_PCIE_PHY_PASSED_RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_PCIE_PHY_PASSED_RESETR {
    #[doc = "do not mask interrupt due to PCIE PHY passed reset - interrupt will be created"]
    MASK_PCIE_PHY_PASSED_RESET_0,
    #[doc = "mask interrupt due to PCIE PHY passed reset"]
    MASK_PCIE_PHY_PASSED_RESET_1,
}
impl MASK_PCIE_PHY_PASSED_RESETR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MASK_PCIE_PHY_PASSED_RESETR::MASK_PCIE_PHY_PASSED_RESET_0 => false,
            MASK_PCIE_PHY_PASSED_RESETR::MASK_PCIE_PHY_PASSED_RESET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_PCIE_PHY_PASSED_RESETR {
        match value {
            false => MASK_PCIE_PHY_PASSED_RESETR::MASK_PCIE_PHY_PASSED_RESET_0,
            true => MASK_PCIE_PHY_PASSED_RESETR::MASK_PCIE_PHY_PASSED_RESET_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_PCIE_PHY_PASSED_RESET_0`"]
    #[inline]
    pub fn is_mask_pcie_phy_passed_reset_0(&self) -> bool {
        *self == MASK_PCIE_PHY_PASSED_RESETR::MASK_PCIE_PHY_PASSED_RESET_0
    }
    #[doc = "Checks if the value of the field is `MASK_PCIE_PHY_PASSED_RESET_1`"]
    #[inline]
    pub fn is_mask_pcie_phy_passed_reset_1(&self) -> bool {
        *self == MASK_PCIE_PHY_PASSED_RESETR::MASK_PCIE_PHY_PASSED_RESET_1
    }
}
#[doc = "Possible values of the field `MASK_DISPLAY_PASSED_RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_DISPLAY_PASSED_RESETR {
    #[doc = "do not mask interrupt due to display passed reset - interrupt will be created"]
    MASK_DISPLAY_PASSED_RESET_0,
    #[doc = "mask interrupt due to display passed reset"]
    MASK_DISPLAY_PASSED_RESET_1,
}
impl MASK_DISPLAY_PASSED_RESETR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MASK_DISPLAY_PASSED_RESETR::MASK_DISPLAY_PASSED_RESET_0 => false,
            MASK_DISPLAY_PASSED_RESETR::MASK_DISPLAY_PASSED_RESET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_DISPLAY_PASSED_RESETR {
        match value {
            false => MASK_DISPLAY_PASSED_RESETR::MASK_DISPLAY_PASSED_RESET_0,
            true => MASK_DISPLAY_PASSED_RESETR::MASK_DISPLAY_PASSED_RESET_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_DISPLAY_PASSED_RESET_0`"]
    #[inline]
    pub fn is_mask_display_passed_reset_0(&self) -> bool {
        *self == MASK_DISPLAY_PASSED_RESETR::MASK_DISPLAY_PASSED_RESET_0
    }
    #[doc = "Checks if the value of the field is `MASK_DISPLAY_PASSED_RESET_1`"]
    #[inline]
    pub fn is_mask_display_passed_reset_1(&self) -> bool {
        *self == MASK_DISPLAY_PASSED_RESETR::MASK_DISPLAY_PASSED_RESET_1
    }
}
#[doc = "Possible values of the field `MASK_M4C_PASSED_RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_M4C_PASSED_RESETR {
    #[doc = "do not mask interrupt due to m4 core passed reset - interrupt will be created"]
    MASK_M4C_PASSED_RESET_0,
    #[doc = "mask interrupt due to m4 core passed reset"]
    MASK_M4C_PASSED_RESET_1,
}
impl MASK_M4C_PASSED_RESETR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MASK_M4C_PASSED_RESETR::MASK_M4C_PASSED_RESET_0 => false,
            MASK_M4C_PASSED_RESETR::MASK_M4C_PASSED_RESET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_M4C_PASSED_RESETR {
        match value {
            false => MASK_M4C_PASSED_RESETR::MASK_M4C_PASSED_RESET_0,
            true => MASK_M4C_PASSED_RESETR::MASK_M4C_PASSED_RESET_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_M4C_PASSED_RESET_0`"]
    #[inline]
    pub fn is_mask_m4c_passed_reset_0(&self) -> bool {
        *self == MASK_M4C_PASSED_RESETR::MASK_M4C_PASSED_RESET_0
    }
    #[doc = "Checks if the value of the field is `MASK_M4C_PASSED_RESET_1`"]
    #[inline]
    pub fn is_mask_m4c_passed_reset_1(&self) -> bool {
        *self == MASK_M4C_PASSED_RESETR::MASK_M4C_PASSED_RESET_1
    }
}
#[doc = "Possible values of the field `MASK_M4P_PASSED_RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_M4P_PASSED_RESETR {
    #[doc = "do not mask interrupt due to m4 platform passed reset - interrupt will be created"]
    MASK_M4P_PASSED_RESET_0,
    #[doc = "mask interrupt due to m4platform passed reset"]
    MASK_M4P_PASSED_RESET_1,
}
impl MASK_M4P_PASSED_RESETR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MASK_M4P_PASSED_RESETR::MASK_M4P_PASSED_RESET_0 => false,
            MASK_M4P_PASSED_RESETR::MASK_M4P_PASSED_RESET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_M4P_PASSED_RESETR {
        match value {
            false => MASK_M4P_PASSED_RESETR::MASK_M4P_PASSED_RESET_0,
            true => MASK_M4P_PASSED_RESETR::MASK_M4P_PASSED_RESET_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_M4P_PASSED_RESET_0`"]
    #[inline]
    pub fn is_mask_m4p_passed_reset_0(&self) -> bool {
        *self == MASK_M4P_PASSED_RESETR::MASK_M4P_PASSED_RESET_0
    }
    #[doc = "Checks if the value of the field is `MASK_M4P_PASSED_RESET_1`"]
    #[inline]
    pub fn is_mask_m4p_passed_reset_1(&self) -> bool {
        *self == MASK_M4P_PASSED_RESETR::MASK_M4P_PASSED_RESET_1
    }
}
#[doc = "Possible values of the field `MASK_GPU_PASSED_RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_GPU_PASSED_RESETR {
    #[doc = "do not mask interrupt due to GPU passed reset - interrupt will be created"]
    MASK_GPU_PASSED_RESET_0,
    #[doc = "mask interrupt due to GPU passed reset"]
    MASK_GPU_PASSED_RESET_1,
}
impl MASK_GPU_PASSED_RESETR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MASK_GPU_PASSED_RESETR::MASK_GPU_PASSED_RESET_0 => false,
            MASK_GPU_PASSED_RESETR::MASK_GPU_PASSED_RESET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_GPU_PASSED_RESETR {
        match value {
            false => MASK_GPU_PASSED_RESETR::MASK_GPU_PASSED_RESET_0,
            true => MASK_GPU_PASSED_RESETR::MASK_GPU_PASSED_RESET_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_GPU_PASSED_RESET_0`"]
    #[inline]
    pub fn is_mask_gpu_passed_reset_0(&self) -> bool {
        *self == MASK_GPU_PASSED_RESETR::MASK_GPU_PASSED_RESET_0
    }
    #[doc = "Checks if the value of the field is `MASK_GPU_PASSED_RESET_1`"]
    #[inline]
    pub fn is_mask_gpu_passed_reset_1(&self) -> bool {
        *self == MASK_GPU_PASSED_RESETR::MASK_GPU_PASSED_RESET_1
    }
}
#[doc = "Possible values of the field `MASK_VPU_PASSED_RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_VPU_PASSED_RESETR {
    #[doc = "do not mask interrupt due to VPU passed reset - interrupt will be created"]
    MASK_VPU_PASSED_RESET_0,
    #[doc = "mask interrupt due to VPU passed reset"]
    MASK_VPU_PASSED_RESET_1,
}
impl MASK_VPU_PASSED_RESETR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MASK_VPU_PASSED_RESETR::MASK_VPU_PASSED_RESET_0 => false,
            MASK_VPU_PASSED_RESETR::MASK_VPU_PASSED_RESET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_VPU_PASSED_RESETR {
        match value {
            false => MASK_VPU_PASSED_RESETR::MASK_VPU_PASSED_RESET_0,
            true => MASK_VPU_PASSED_RESETR::MASK_VPU_PASSED_RESET_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_VPU_PASSED_RESET_0`"]
    #[inline]
    pub fn is_mask_vpu_passed_reset_0(&self) -> bool {
        *self == MASK_VPU_PASSED_RESETR::MASK_VPU_PASSED_RESET_0
    }
    #[doc = "Checks if the value of the field is `MASK_VPU_PASSED_RESET_1`"]
    #[inline]
    pub fn is_mask_vpu_passed_reset_1(&self) -> bool {
        *self == MASK_VPU_PASSED_RESETR::MASK_VPU_PASSED_RESET_1
    }
}
#[doc = "Values that can be written to the field `MASK_OTGPHY1_PASSED_RESET`"]
pub enum MASK_OTGPHY1_PASSED_RESETW {
    #[doc = "do not mask interrupt due to OTG PHY1 passed reset - interrupt will be created"]
    MASK_OTGPHY1_PASSED_RESET_0,
    #[doc = "mask interrupt due to OTG PHY1 passed reset"]
    MASK_OTGPHY1_PASSED_RESET_1,
}
impl MASK_OTGPHY1_PASSED_RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASK_OTGPHY1_PASSED_RESETW::MASK_OTGPHY1_PASSED_RESET_0 => false,
            MASK_OTGPHY1_PASSED_RESETW::MASK_OTGPHY1_PASSED_RESET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_OTGPHY1_PASSED_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_OTGPHY1_PASSED_RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_OTGPHY1_PASSED_RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not mask interrupt due to OTG PHY1 passed reset - interrupt will be created"]
    #[inline]
    pub fn mask_otgphy1_passed_reset_0(self) -> &'a mut W {
        self.variant(MASK_OTGPHY1_PASSED_RESETW::MASK_OTGPHY1_PASSED_RESET_0)
    }
    #[doc = "mask interrupt due to OTG PHY1 passed reset"]
    #[inline]
    pub fn mask_otgphy1_passed_reset_1(self) -> &'a mut W {
        self.variant(MASK_OTGPHY1_PASSED_RESETW::MASK_OTGPHY1_PASSED_RESET_1)
    }
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
#[doc = "Values that can be written to the field `MASK_OTGPHY2_PASSED_RESET`"]
pub enum MASK_OTGPHY2_PASSED_RESETW {
    #[doc = "do not mask interrupt due to OTG PHY2 passed reset - interrupt will be created"]
    MASK_OTGPHY2_PASSED_RESET_0,
    #[doc = "mask interrupt due to OTG PHY2 passed reset"]
    MASK_OTGPHY2_PASSED_RESET_1,
}
impl MASK_OTGPHY2_PASSED_RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASK_OTGPHY2_PASSED_RESETW::MASK_OTGPHY2_PASSED_RESET_0 => false,
            MASK_OTGPHY2_PASSED_RESETW::MASK_OTGPHY2_PASSED_RESET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_OTGPHY2_PASSED_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_OTGPHY2_PASSED_RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_OTGPHY2_PASSED_RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not mask interrupt due to OTG PHY2 passed reset - interrupt will be created"]
    #[inline]
    pub fn mask_otgphy2_passed_reset_0(self) -> &'a mut W {
        self.variant(MASK_OTGPHY2_PASSED_RESETW::MASK_OTGPHY2_PASSED_RESET_0)
    }
    #[doc = "mask interrupt due to OTG PHY2 passed reset"]
    #[inline]
    pub fn mask_otgphy2_passed_reset_1(self) -> &'a mut W {
        self.variant(MASK_OTGPHY2_PASSED_RESETW::MASK_OTGPHY2_PASSED_RESET_1)
    }
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
#[doc = "Values that can be written to the field `MASK_PCIE_PHY_PASSED_RESET`"]
pub enum MASK_PCIE_PHY_PASSED_RESETW {
    #[doc = "do not mask interrupt due to PCIE PHY passed reset - interrupt will be created"]
    MASK_PCIE_PHY_PASSED_RESET_0,
    #[doc = "mask interrupt due to PCIE PHY passed reset"]
    MASK_PCIE_PHY_PASSED_RESET_1,
}
impl MASK_PCIE_PHY_PASSED_RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASK_PCIE_PHY_PASSED_RESETW::MASK_PCIE_PHY_PASSED_RESET_0 => false,
            MASK_PCIE_PHY_PASSED_RESETW::MASK_PCIE_PHY_PASSED_RESET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_PCIE_PHY_PASSED_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_PCIE_PHY_PASSED_RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_PCIE_PHY_PASSED_RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not mask interrupt due to PCIE PHY passed reset - interrupt will be created"]
    #[inline]
    pub fn mask_pcie_phy_passed_reset_0(self) -> &'a mut W {
        self.variant(MASK_PCIE_PHY_PASSED_RESETW::MASK_PCIE_PHY_PASSED_RESET_0)
    }
    #[doc = "mask interrupt due to PCIE PHY passed reset"]
    #[inline]
    pub fn mask_pcie_phy_passed_reset_1(self) -> &'a mut W {
        self.variant(MASK_PCIE_PHY_PASSED_RESETW::MASK_PCIE_PHY_PASSED_RESET_1)
    }
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
#[doc = "Values that can be written to the field `MASK_DISPLAY_PASSED_RESET`"]
pub enum MASK_DISPLAY_PASSED_RESETW {
    #[doc = "do not mask interrupt due to display passed reset - interrupt will be created"]
    MASK_DISPLAY_PASSED_RESET_0,
    #[doc = "mask interrupt due to display passed reset"]
    MASK_DISPLAY_PASSED_RESET_1,
}
impl MASK_DISPLAY_PASSED_RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASK_DISPLAY_PASSED_RESETW::MASK_DISPLAY_PASSED_RESET_0 => false,
            MASK_DISPLAY_PASSED_RESETW::MASK_DISPLAY_PASSED_RESET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_DISPLAY_PASSED_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_DISPLAY_PASSED_RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_DISPLAY_PASSED_RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not mask interrupt due to display passed reset - interrupt will be created"]
    #[inline]
    pub fn mask_display_passed_reset_0(self) -> &'a mut W {
        self.variant(MASK_DISPLAY_PASSED_RESETW::MASK_DISPLAY_PASSED_RESET_0)
    }
    #[doc = "mask interrupt due to display passed reset"]
    #[inline]
    pub fn mask_display_passed_reset_1(self) -> &'a mut W {
        self.variant(MASK_DISPLAY_PASSED_RESETW::MASK_DISPLAY_PASSED_RESET_1)
    }
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
#[doc = "Values that can be written to the field `MASK_M4C_PASSED_RESET`"]
pub enum MASK_M4C_PASSED_RESETW {
    #[doc = "do not mask interrupt due to m4 core passed reset - interrupt will be created"]
    MASK_M4C_PASSED_RESET_0,
    #[doc = "mask interrupt due to m4 core passed reset"]
    MASK_M4C_PASSED_RESET_1,
}
impl MASK_M4C_PASSED_RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASK_M4C_PASSED_RESETW::MASK_M4C_PASSED_RESET_0 => false,
            MASK_M4C_PASSED_RESETW::MASK_M4C_PASSED_RESET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_M4C_PASSED_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_M4C_PASSED_RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_M4C_PASSED_RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not mask interrupt due to m4 core passed reset - interrupt will be created"]
    #[inline]
    pub fn mask_m4c_passed_reset_0(self) -> &'a mut W {
        self.variant(MASK_M4C_PASSED_RESETW::MASK_M4C_PASSED_RESET_0)
    }
    #[doc = "mask interrupt due to m4 core passed reset"]
    #[inline]
    pub fn mask_m4c_passed_reset_1(self) -> &'a mut W {
        self.variant(MASK_M4C_PASSED_RESETW::MASK_M4C_PASSED_RESET_1)
    }
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
#[doc = "Values that can be written to the field `MASK_M4P_PASSED_RESET`"]
pub enum MASK_M4P_PASSED_RESETW {
    #[doc = "do not mask interrupt due to m4 platform passed reset - interrupt will be created"]
    MASK_M4P_PASSED_RESET_0,
    #[doc = "mask interrupt due to m4platform passed reset"]
    MASK_M4P_PASSED_RESET_1,
}
impl MASK_M4P_PASSED_RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASK_M4P_PASSED_RESETW::MASK_M4P_PASSED_RESET_0 => false,
            MASK_M4P_PASSED_RESETW::MASK_M4P_PASSED_RESET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_M4P_PASSED_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_M4P_PASSED_RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_M4P_PASSED_RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not mask interrupt due to m4 platform passed reset - interrupt will be created"]
    #[inline]
    pub fn mask_m4p_passed_reset_0(self) -> &'a mut W {
        self.variant(MASK_M4P_PASSED_RESETW::MASK_M4P_PASSED_RESET_0)
    }
    #[doc = "mask interrupt due to m4platform passed reset"]
    #[inline]
    pub fn mask_m4p_passed_reset_1(self) -> &'a mut W {
        self.variant(MASK_M4P_PASSED_RESETW::MASK_M4P_PASSED_RESET_1)
    }
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
#[doc = "Values that can be written to the field `MASK_GPU_PASSED_RESET`"]
pub enum MASK_GPU_PASSED_RESETW {
    #[doc = "do not mask interrupt due to GPU passed reset - interrupt will be created"]
    MASK_GPU_PASSED_RESET_0,
    #[doc = "mask interrupt due to GPU passed reset"]
    MASK_GPU_PASSED_RESET_1,
}
impl MASK_GPU_PASSED_RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASK_GPU_PASSED_RESETW::MASK_GPU_PASSED_RESET_0 => false,
            MASK_GPU_PASSED_RESETW::MASK_GPU_PASSED_RESET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_GPU_PASSED_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_GPU_PASSED_RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_GPU_PASSED_RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not mask interrupt due to GPU passed reset - interrupt will be created"]
    #[inline]
    pub fn mask_gpu_passed_reset_0(self) -> &'a mut W {
        self.variant(MASK_GPU_PASSED_RESETW::MASK_GPU_PASSED_RESET_0)
    }
    #[doc = "mask interrupt due to GPU passed reset"]
    #[inline]
    pub fn mask_gpu_passed_reset_1(self) -> &'a mut W {
        self.variant(MASK_GPU_PASSED_RESETW::MASK_GPU_PASSED_RESET_1)
    }
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
#[doc = "Values that can be written to the field `MASK_VPU_PASSED_RESET`"]
pub enum MASK_VPU_PASSED_RESETW {
    #[doc = "do not mask interrupt due to VPU passed reset - interrupt will be created"]
    MASK_VPU_PASSED_RESET_0,
    #[doc = "mask interrupt due to VPU passed reset"]
    MASK_VPU_PASSED_RESET_1,
}
impl MASK_VPU_PASSED_RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASK_VPU_PASSED_RESETW::MASK_VPU_PASSED_RESET_0 => false,
            MASK_VPU_PASSED_RESETW::MASK_VPU_PASSED_RESET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_VPU_PASSED_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_VPU_PASSED_RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_VPU_PASSED_RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not mask interrupt due to VPU passed reset - interrupt will be created"]
    #[inline]
    pub fn mask_vpu_passed_reset_0(self) -> &'a mut W {
        self.variant(MASK_VPU_PASSED_RESETW::MASK_VPU_PASSED_RESET_0)
    }
    #[doc = "mask interrupt due to VPU passed reset"]
    #[inline]
    pub fn mask_vpu_passed_reset_1(self) -> &'a mut W {
        self.variant(MASK_VPU_PASSED_RESETW::MASK_VPU_PASSED_RESET_1)
    }
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
    #[doc = "Bit 2 - mask interrupt generation due to OTG PHY1 passed reset"]
    #[inline]
    pub fn mask_otgphy1_passed_reset(&self) -> MASK_OTGPHY1_PASSED_RESETR {
        MASK_OTGPHY1_PASSED_RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - mask interrupt generation due to OTG PHY2 passed reset"]
    #[inline]
    pub fn mask_otgphy2_passed_reset(&self) -> MASK_OTGPHY2_PASSED_RESETR {
        MASK_OTGPHY2_PASSED_RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Mask interrupt generation due to PCIE PHY passed reset"]
    #[inline]
    pub fn mask_pcie_phy_passed_reset(&self) -> MASK_PCIE_PHY_PASSED_RESETR {
        MASK_PCIE_PHY_PASSED_RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Mask interrupt generation due to display passed reset"]
    #[inline]
    pub fn mask_display_passed_reset(&self) -> MASK_DISPLAY_PASSED_RESETR {
        MASK_DISPLAY_PASSED_RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - mask interrupt generation due to m4 core passed reset"]
    #[inline]
    pub fn mask_m4c_passed_reset(&self) -> MASK_M4C_PASSED_RESETR {
        MASK_M4C_PASSED_RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - mask interrupt generation due to m4 platform passed reset"]
    #[inline]
    pub fn mask_m4p_passed_reset(&self) -> MASK_M4P_PASSED_RESETR {
        MASK_M4P_PASSED_RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Mask interrupt generation due to GPU passed reset"]
    #[inline]
    pub fn mask_gpu_passed_reset(&self) -> MASK_GPU_PASSED_RESETR {
        MASK_GPU_PASSED_RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Mask interrupt generation due to VPU passed reset"]
    #[inline]
    pub fn mask_vpu_passed_reset(&self) -> MASK_VPU_PASSED_RESETR {
        MASK_VPU_PASSED_RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1023 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - mask interrupt generation due to OTG PHY1 passed reset"]
    #[inline]
    pub fn mask_otgphy1_passed_reset(&mut self) -> _MASK_OTGPHY1_PASSED_RESETW {
        _MASK_OTGPHY1_PASSED_RESETW { w: self }
    }
    #[doc = "Bit 3 - mask interrupt generation due to OTG PHY2 passed reset"]
    #[inline]
    pub fn mask_otgphy2_passed_reset(&mut self) -> _MASK_OTGPHY2_PASSED_RESETW {
        _MASK_OTGPHY2_PASSED_RESETW { w: self }
    }
    #[doc = "Bit 5 - Mask interrupt generation due to PCIE PHY passed reset"]
    #[inline]
    pub fn mask_pcie_phy_passed_reset(&mut self) -> _MASK_PCIE_PHY_PASSED_RESETW {
        _MASK_PCIE_PHY_PASSED_RESETW { w: self }
    }
    #[doc = "Bit 7 - Mask interrupt generation due to display passed reset"]
    #[inline]
    pub fn mask_display_passed_reset(&mut self) -> _MASK_DISPLAY_PASSED_RESETW {
        _MASK_DISPLAY_PASSED_RESETW { w: self }
    }
    #[doc = "Bit 8 - mask interrupt generation due to m4 core passed reset"]
    #[inline]
    pub fn mask_m4c_passed_reset(&mut self) -> _MASK_M4C_PASSED_RESETW {
        _MASK_M4C_PASSED_RESETW { w: self }
    }
    #[doc = "Bit 9 - mask interrupt generation due to m4 platform passed reset"]
    #[inline]
    pub fn mask_m4p_passed_reset(&mut self) -> _MASK_M4P_PASSED_RESETW {
        _MASK_M4P_PASSED_RESETW { w: self }
    }
    #[doc = "Bit 10 - Mask interrupt generation due to GPU passed reset"]
    #[inline]
    pub fn mask_gpu_passed_reset(&mut self) -> _MASK_GPU_PASSED_RESETW {
        _MASK_GPU_PASSED_RESETW { w: self }
    }
    #[doc = "Bit 11 - Mask interrupt generation due to VPU passed reset"]
    #[inline]
    pub fn mask_vpu_passed_reset(&mut self) -> _MASK_VPU_PASSED_RESETW {
        _MASK_VPU_PASSED_RESETW { w: self }
    }
}
