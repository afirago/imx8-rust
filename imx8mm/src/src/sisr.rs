#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SISR {
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
#[doc = "Possible values of the field `OTGPHY1_PASSED_RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTGPHY1_PASSED_RESETR {
    #[doc = "Interrupt generated not due to OTG PHY1 passed reset"]
    OTGPHY1_PASSED_RESET_0,
    #[doc = "Interrupt generated due to OTG PHY1 passed reset"]
    OTGPHY1_PASSED_RESET_1,
}
impl OTGPHY1_PASSED_RESETR {
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
            OTGPHY1_PASSED_RESETR::OTGPHY1_PASSED_RESET_0 => false,
            OTGPHY1_PASSED_RESETR::OTGPHY1_PASSED_RESET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OTGPHY1_PASSED_RESETR {
        match value {
            false => OTGPHY1_PASSED_RESETR::OTGPHY1_PASSED_RESET_0,
            true => OTGPHY1_PASSED_RESETR::OTGPHY1_PASSED_RESET_1,
        }
    }
    #[doc = "Checks if the value of the field is `OTGPHY1_PASSED_RESET_0`"]
    #[inline]
    pub fn is_otgphy1_passed_reset_0(&self) -> bool {
        *self == OTGPHY1_PASSED_RESETR::OTGPHY1_PASSED_RESET_0
    }
    #[doc = "Checks if the value of the field is `OTGPHY1_PASSED_RESET_1`"]
    #[inline]
    pub fn is_otgphy1_passed_reset_1(&self) -> bool {
        *self == OTGPHY1_PASSED_RESETR::OTGPHY1_PASSED_RESET_1
    }
}
#[doc = "Possible values of the field `OTGPHY2_PASSED_RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTGPHY2_PASSED_RESETR {
    #[doc = "Interrupt generated not due to OTG PHY2 passed reset"]
    OTGPHY2_PASSED_RESET_0,
    #[doc = "Interrupt generated due to OTG PHY2 passed reset"]
    OTGPHY2_PASSED_RESET_1,
}
impl OTGPHY2_PASSED_RESETR {
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
            OTGPHY2_PASSED_RESETR::OTGPHY2_PASSED_RESET_0 => false,
            OTGPHY2_PASSED_RESETR::OTGPHY2_PASSED_RESET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OTGPHY2_PASSED_RESETR {
        match value {
            false => OTGPHY2_PASSED_RESETR::OTGPHY2_PASSED_RESET_0,
            true => OTGPHY2_PASSED_RESETR::OTGPHY2_PASSED_RESET_1,
        }
    }
    #[doc = "Checks if the value of the field is `OTGPHY2_PASSED_RESET_0`"]
    #[inline]
    pub fn is_otgphy2_passed_reset_0(&self) -> bool {
        *self == OTGPHY2_PASSED_RESETR::OTGPHY2_PASSED_RESET_0
    }
    #[doc = "Checks if the value of the field is `OTGPHY2_PASSED_RESET_1`"]
    #[inline]
    pub fn is_otgphy2_passed_reset_1(&self) -> bool {
        *self == OTGPHY2_PASSED_RESETR::OTGPHY2_PASSED_RESET_1
    }
}
#[doc = "Possible values of the field `PCIE1_PHY_PASSED_RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCIE1_PHY_PASSED_RESETR {
    #[doc = "Interrupt generated not due to PCIE1 PHY passed reset"]
    PCIE1_PHY_PASSED_RESET_0,
    #[doc = "Interrupt generated due to PCIE1 PHY passed reset"]
    PCIE1_PHY_PASSED_RESET_1,
}
impl PCIE1_PHY_PASSED_RESETR {
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
            PCIE1_PHY_PASSED_RESETR::PCIE1_PHY_PASSED_RESET_0 => false,
            PCIE1_PHY_PASSED_RESETR::PCIE1_PHY_PASSED_RESET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PCIE1_PHY_PASSED_RESETR {
        match value {
            false => PCIE1_PHY_PASSED_RESETR::PCIE1_PHY_PASSED_RESET_0,
            true => PCIE1_PHY_PASSED_RESETR::PCIE1_PHY_PASSED_RESET_1,
        }
    }
    #[doc = "Checks if the value of the field is `PCIE1_PHY_PASSED_RESET_0`"]
    #[inline]
    pub fn is_pcie1_phy_passed_reset_0(&self) -> bool {
        *self == PCIE1_PHY_PASSED_RESETR::PCIE1_PHY_PASSED_RESET_0
    }
    #[doc = "Checks if the value of the field is `PCIE1_PHY_PASSED_RESET_1`"]
    #[inline]
    pub fn is_pcie1_phy_passed_reset_1(&self) -> bool {
        *self == PCIE1_PHY_PASSED_RESETR::PCIE1_PHY_PASSED_RESET_1
    }
}
#[doc = "Possible values of the field `DISPLAY_PASSED_RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISPLAY_PASSED_RESETR {
    #[doc = "Interrupt generated not due to DISPLAY passed reset"]
    DISPLAY_PASSED_RESET_0,
    #[doc = "Interrupt generated due to DISPLAY passed reset"]
    DISPLAY_PASSED_RESET_1,
}
impl DISPLAY_PASSED_RESETR {
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
            DISPLAY_PASSED_RESETR::DISPLAY_PASSED_RESET_0 => false,
            DISPLAY_PASSED_RESETR::DISPLAY_PASSED_RESET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISPLAY_PASSED_RESETR {
        match value {
            false => DISPLAY_PASSED_RESETR::DISPLAY_PASSED_RESET_0,
            true => DISPLAY_PASSED_RESETR::DISPLAY_PASSED_RESET_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISPLAY_PASSED_RESET_0`"]
    #[inline]
    pub fn is_display_passed_reset_0(&self) -> bool {
        *self == DISPLAY_PASSED_RESETR::DISPLAY_PASSED_RESET_0
    }
    #[doc = "Checks if the value of the field is `DISPLAY_PASSED_RESET_1`"]
    #[inline]
    pub fn is_display_passed_reset_1(&self) -> bool {
        *self == DISPLAY_PASSED_RESETR::DISPLAY_PASSED_RESET_1
    }
}
#[doc = "Possible values of the field `M4C_PASSED_RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M4C_PASSED_RESETR {
    #[doc = "interrupt generated not due to m4 core reset"]
    M4C_PASSED_RESET_0,
    #[doc = "interrupt generated due to m4 core reset"]
    M4C_PASSED_RESET_1,
}
impl M4C_PASSED_RESETR {
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
            M4C_PASSED_RESETR::M4C_PASSED_RESET_0 => false,
            M4C_PASSED_RESETR::M4C_PASSED_RESET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M4C_PASSED_RESETR {
        match value {
            false => M4C_PASSED_RESETR::M4C_PASSED_RESET_0,
            true => M4C_PASSED_RESETR::M4C_PASSED_RESET_1,
        }
    }
    #[doc = "Checks if the value of the field is `M4C_PASSED_RESET_0`"]
    #[inline]
    pub fn is_m4c_passed_reset_0(&self) -> bool {
        *self == M4C_PASSED_RESETR::M4C_PASSED_RESET_0
    }
    #[doc = "Checks if the value of the field is `M4C_PASSED_RESET_1`"]
    #[inline]
    pub fn is_m4c_passed_reset_1(&self) -> bool {
        *self == M4C_PASSED_RESETR::M4C_PASSED_RESET_1
    }
}
#[doc = "Possible values of the field `M4P_PASSED_RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M4P_PASSED_RESETR {
    #[doc = "interrupt generated not due to m4 platform reset"]
    M4P_PASSED_RESET_0,
    #[doc = "interrupt generated due to m4 platform reset"]
    M4P_PASSED_RESET_1,
}
impl M4P_PASSED_RESETR {
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
            M4P_PASSED_RESETR::M4P_PASSED_RESET_0 => false,
            M4P_PASSED_RESETR::M4P_PASSED_RESET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M4P_PASSED_RESETR {
        match value {
            false => M4P_PASSED_RESETR::M4P_PASSED_RESET_0,
            true => M4P_PASSED_RESETR::M4P_PASSED_RESET_1,
        }
    }
    #[doc = "Checks if the value of the field is `M4P_PASSED_RESET_0`"]
    #[inline]
    pub fn is_m4p_passed_reset_0(&self) -> bool {
        *self == M4P_PASSED_RESETR::M4P_PASSED_RESET_0
    }
    #[doc = "Checks if the value of the field is `M4P_PASSED_RESET_1`"]
    #[inline]
    pub fn is_m4p_passed_reset_1(&self) -> bool {
        *self == M4P_PASSED_RESETR::M4P_PASSED_RESET_1
    }
}
#[doc = "Possible values of the field `GPU_PASSED_RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPU_PASSED_RESETR {
    #[doc = "interrupt generated not due to GPU reset"]
    GPU_PASSED_RESET_0,
    #[doc = "interrupt generated due to GPU reset"]
    GPU_PASSED_RESET_1,
}
impl GPU_PASSED_RESETR {
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
            GPU_PASSED_RESETR::GPU_PASSED_RESET_0 => false,
            GPU_PASSED_RESETR::GPU_PASSED_RESET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPU_PASSED_RESETR {
        match value {
            false => GPU_PASSED_RESETR::GPU_PASSED_RESET_0,
            true => GPU_PASSED_RESETR::GPU_PASSED_RESET_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPU_PASSED_RESET_0`"]
    #[inline]
    pub fn is_gpu_passed_reset_0(&self) -> bool {
        *self == GPU_PASSED_RESETR::GPU_PASSED_RESET_0
    }
    #[doc = "Checks if the value of the field is `GPU_PASSED_RESET_1`"]
    #[inline]
    pub fn is_gpu_passed_reset_1(&self) -> bool {
        *self == GPU_PASSED_RESETR::GPU_PASSED_RESET_1
    }
}
#[doc = "Possible values of the field `VPU_PASSED_RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VPU_PASSED_RESETR {
    #[doc = "interrupt generated not due to VPU reset"]
    VPU_PASSED_RESET_0,
    #[doc = "interrupt generated due to VPU reset"]
    VPU_PASSED_RESET_1,
}
impl VPU_PASSED_RESETR {
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
            VPU_PASSED_RESETR::VPU_PASSED_RESET_0 => false,
            VPU_PASSED_RESETR::VPU_PASSED_RESET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VPU_PASSED_RESETR {
        match value {
            false => VPU_PASSED_RESETR::VPU_PASSED_RESET_0,
            true => VPU_PASSED_RESETR::VPU_PASSED_RESET_1,
        }
    }
    #[doc = "Checks if the value of the field is `VPU_PASSED_RESET_0`"]
    #[inline]
    pub fn is_vpu_passed_reset_0(&self) -> bool {
        *self == VPU_PASSED_RESETR::VPU_PASSED_RESET_0
    }
    #[doc = "Checks if the value of the field is `VPU_PASSED_RESET_1`"]
    #[inline]
    pub fn is_vpu_passed_reset_1(&self) -> bool {
        *self == VPU_PASSED_RESETR::VPU_PASSED_RESET_1
    }
}
#[doc = "Values that can be written to the field `OTGPHY1_PASSED_RESET`"]
pub enum OTGPHY1_PASSED_RESETW {
    #[doc = "Interrupt generated not due to OTG PHY1 passed reset"]
    OTGPHY1_PASSED_RESET_0,
    #[doc = "Interrupt generated due to OTG PHY1 passed reset"]
    OTGPHY1_PASSED_RESET_1,
}
impl OTGPHY1_PASSED_RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OTGPHY1_PASSED_RESETW::OTGPHY1_PASSED_RESET_0 => false,
            OTGPHY1_PASSED_RESETW::OTGPHY1_PASSED_RESET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OTGPHY1_PASSED_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _OTGPHY1_PASSED_RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OTGPHY1_PASSED_RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt generated not due to OTG PHY1 passed reset"]
    #[inline]
    pub fn otgphy1_passed_reset_0(self) -> &'a mut W {
        self.variant(OTGPHY1_PASSED_RESETW::OTGPHY1_PASSED_RESET_0)
    }
    #[doc = "Interrupt generated due to OTG PHY1 passed reset"]
    #[inline]
    pub fn otgphy1_passed_reset_1(self) -> &'a mut W {
        self.variant(OTGPHY1_PASSED_RESETW::OTGPHY1_PASSED_RESET_1)
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
#[doc = "Values that can be written to the field `OTGPHY2_PASSED_RESET`"]
pub enum OTGPHY2_PASSED_RESETW {
    #[doc = "Interrupt generated not due to OTG PHY2 passed reset"]
    OTGPHY2_PASSED_RESET_0,
    #[doc = "Interrupt generated due to OTG PHY2 passed reset"]
    OTGPHY2_PASSED_RESET_1,
}
impl OTGPHY2_PASSED_RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OTGPHY2_PASSED_RESETW::OTGPHY2_PASSED_RESET_0 => false,
            OTGPHY2_PASSED_RESETW::OTGPHY2_PASSED_RESET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OTGPHY2_PASSED_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _OTGPHY2_PASSED_RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OTGPHY2_PASSED_RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt generated not due to OTG PHY2 passed reset"]
    #[inline]
    pub fn otgphy2_passed_reset_0(self) -> &'a mut W {
        self.variant(OTGPHY2_PASSED_RESETW::OTGPHY2_PASSED_RESET_0)
    }
    #[doc = "Interrupt generated due to OTG PHY2 passed reset"]
    #[inline]
    pub fn otgphy2_passed_reset_1(self) -> &'a mut W {
        self.variant(OTGPHY2_PASSED_RESETW::OTGPHY2_PASSED_RESET_1)
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
#[doc = "Values that can be written to the field `PCIE1_PHY_PASSED_RESET`"]
pub enum PCIE1_PHY_PASSED_RESETW {
    #[doc = "Interrupt generated not due to PCIE1 PHY passed reset"]
    PCIE1_PHY_PASSED_RESET_0,
    #[doc = "Interrupt generated due to PCIE1 PHY passed reset"]
    PCIE1_PHY_PASSED_RESET_1,
}
impl PCIE1_PHY_PASSED_RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PCIE1_PHY_PASSED_RESETW::PCIE1_PHY_PASSED_RESET_0 => false,
            PCIE1_PHY_PASSED_RESETW::PCIE1_PHY_PASSED_RESET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCIE1_PHY_PASSED_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _PCIE1_PHY_PASSED_RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCIE1_PHY_PASSED_RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt generated not due to PCIE1 PHY passed reset"]
    #[inline]
    pub fn pcie1_phy_passed_reset_0(self) -> &'a mut W {
        self.variant(PCIE1_PHY_PASSED_RESETW::PCIE1_PHY_PASSED_RESET_0)
    }
    #[doc = "Interrupt generated due to PCIE1 PHY passed reset"]
    #[inline]
    pub fn pcie1_phy_passed_reset_1(self) -> &'a mut W {
        self.variant(PCIE1_PHY_PASSED_RESETW::PCIE1_PHY_PASSED_RESET_1)
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
#[doc = "Values that can be written to the field `DISPLAY_PASSED_RESET`"]
pub enum DISPLAY_PASSED_RESETW {
    #[doc = "Interrupt generated not due to DISPLAY passed reset"]
    DISPLAY_PASSED_RESET_0,
    #[doc = "Interrupt generated due to DISPLAY passed reset"]
    DISPLAY_PASSED_RESET_1,
}
impl DISPLAY_PASSED_RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DISPLAY_PASSED_RESETW::DISPLAY_PASSED_RESET_0 => false,
            DISPLAY_PASSED_RESETW::DISPLAY_PASSED_RESET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISPLAY_PASSED_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _DISPLAY_PASSED_RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISPLAY_PASSED_RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt generated not due to DISPLAY passed reset"]
    #[inline]
    pub fn display_passed_reset_0(self) -> &'a mut W {
        self.variant(DISPLAY_PASSED_RESETW::DISPLAY_PASSED_RESET_0)
    }
    #[doc = "Interrupt generated due to DISPLAY passed reset"]
    #[inline]
    pub fn display_passed_reset_1(self) -> &'a mut W {
        self.variant(DISPLAY_PASSED_RESETW::DISPLAY_PASSED_RESET_1)
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
#[doc = "Values that can be written to the field `M4C_PASSED_RESET`"]
pub enum M4C_PASSED_RESETW {
    #[doc = "interrupt generated not due to m4 core reset"]
    M4C_PASSED_RESET_0,
    #[doc = "interrupt generated due to m4 core reset"]
    M4C_PASSED_RESET_1,
}
impl M4C_PASSED_RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M4C_PASSED_RESETW::M4C_PASSED_RESET_0 => false,
            M4C_PASSED_RESETW::M4C_PASSED_RESET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M4C_PASSED_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _M4C_PASSED_RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M4C_PASSED_RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "interrupt generated not due to m4 core reset"]
    #[inline]
    pub fn m4c_passed_reset_0(self) -> &'a mut W {
        self.variant(M4C_PASSED_RESETW::M4C_PASSED_RESET_0)
    }
    #[doc = "interrupt generated due to m4 core reset"]
    #[inline]
    pub fn m4c_passed_reset_1(self) -> &'a mut W {
        self.variant(M4C_PASSED_RESETW::M4C_PASSED_RESET_1)
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
#[doc = "Values that can be written to the field `M4P_PASSED_RESET`"]
pub enum M4P_PASSED_RESETW {
    #[doc = "interrupt generated not due to m4 platform reset"]
    M4P_PASSED_RESET_0,
    #[doc = "interrupt generated due to m4 platform reset"]
    M4P_PASSED_RESET_1,
}
impl M4P_PASSED_RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M4P_PASSED_RESETW::M4P_PASSED_RESET_0 => false,
            M4P_PASSED_RESETW::M4P_PASSED_RESET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M4P_PASSED_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _M4P_PASSED_RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M4P_PASSED_RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "interrupt generated not due to m4 platform reset"]
    #[inline]
    pub fn m4p_passed_reset_0(self) -> &'a mut W {
        self.variant(M4P_PASSED_RESETW::M4P_PASSED_RESET_0)
    }
    #[doc = "interrupt generated due to m4 platform reset"]
    #[inline]
    pub fn m4p_passed_reset_1(self) -> &'a mut W {
        self.variant(M4P_PASSED_RESETW::M4P_PASSED_RESET_1)
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
#[doc = "Values that can be written to the field `GPU_PASSED_RESET`"]
pub enum GPU_PASSED_RESETW {
    #[doc = "interrupt generated not due to GPU reset"]
    GPU_PASSED_RESET_0,
    #[doc = "interrupt generated due to GPU reset"]
    GPU_PASSED_RESET_1,
}
impl GPU_PASSED_RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPU_PASSED_RESETW::GPU_PASSED_RESET_0 => false,
            GPU_PASSED_RESETW::GPU_PASSED_RESET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPU_PASSED_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _GPU_PASSED_RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPU_PASSED_RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "interrupt generated not due to GPU reset"]
    #[inline]
    pub fn gpu_passed_reset_0(self) -> &'a mut W {
        self.variant(GPU_PASSED_RESETW::GPU_PASSED_RESET_0)
    }
    #[doc = "interrupt generated due to GPU reset"]
    #[inline]
    pub fn gpu_passed_reset_1(self) -> &'a mut W {
        self.variant(GPU_PASSED_RESETW::GPU_PASSED_RESET_1)
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
#[doc = "Values that can be written to the field `VPU_PASSED_RESET`"]
pub enum VPU_PASSED_RESETW {
    #[doc = "interrupt generated not due to VPU reset"]
    VPU_PASSED_RESET_0,
    #[doc = "interrupt generated due to VPU reset"]
    VPU_PASSED_RESET_1,
}
impl VPU_PASSED_RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VPU_PASSED_RESETW::VPU_PASSED_RESET_0 => false,
            VPU_PASSED_RESETW::VPU_PASSED_RESET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VPU_PASSED_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _VPU_PASSED_RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VPU_PASSED_RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "interrupt generated not due to VPU reset"]
    #[inline]
    pub fn vpu_passed_reset_0(self) -> &'a mut W {
        self.variant(VPU_PASSED_RESETW::VPU_PASSED_RESET_0)
    }
    #[doc = "interrupt generated due to VPU reset"]
    #[inline]
    pub fn vpu_passed_reset_1(self) -> &'a mut W {
        self.variant(VPU_PASSED_RESETW::VPU_PASSED_RESET_1)
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
    #[doc = "Bit 2 - Interrupt generated to indicate that OTG PHY1 passed software reset and is ready to be used"]
    #[inline]
    pub fn otgphy1_passed_reset(&self) -> OTGPHY1_PASSED_RESETR {
        OTGPHY1_PASSED_RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Interrupt generated to indicate that OTG PHY2 passed software reset and is ready to be used"]
    #[inline]
    pub fn otgphy2_passed_reset(&self) -> OTGPHY2_PASSED_RESETR {
        OTGPHY2_PASSED_RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Interrupt generated to indicate that PCIE1 PHY passed software reset and is ready to be used"]
    #[inline]
    pub fn pcie1_phy_passed_reset(&self) -> PCIE1_PHY_PASSED_RESETR {
        PCIE1_PHY_PASSED_RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Interrupt generated to indicate that DISPLAY passed software reset and is ready to be used"]
    #[inline]
    pub fn display_passed_reset(&self) -> DISPLAY_PASSED_RESETR {
        DISPLAY_PASSED_RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Interrupt generated to indicate that m4 core passed software reset and is ready to be used"]
    #[inline]
    pub fn m4c_passed_reset(&self) -> M4C_PASSED_RESETR {
        M4C_PASSED_RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Interrupt generated to indicate that m4 platform passed software reset and is ready to be used"]
    #[inline]
    pub fn m4p_passed_reset(&self) -> M4P_PASSED_RESETR {
        M4P_PASSED_RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Interrupt generated to indicate that GPU passed software reset and is ready to be used"]
    #[inline]
    pub fn gpu_passed_reset(&self) -> GPU_PASSED_RESETR {
        GPU_PASSED_RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Interrupt generated to indicate that VPU passed software reset and is ready to be used"]
    #[inline]
    pub fn vpu_passed_reset(&self) -> VPU_PASSED_RESETR {
        VPU_PASSED_RESETR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - Interrupt generated to indicate that OTG PHY1 passed software reset and is ready to be used"]
    #[inline]
    pub fn otgphy1_passed_reset(&mut self) -> _OTGPHY1_PASSED_RESETW {
        _OTGPHY1_PASSED_RESETW { w: self }
    }
    #[doc = "Bit 3 - Interrupt generated to indicate that OTG PHY2 passed software reset and is ready to be used"]
    #[inline]
    pub fn otgphy2_passed_reset(&mut self) -> _OTGPHY2_PASSED_RESETW {
        _OTGPHY2_PASSED_RESETW { w: self }
    }
    #[doc = "Bit 5 - Interrupt generated to indicate that PCIE1 PHY passed software reset and is ready to be used"]
    #[inline]
    pub fn pcie1_phy_passed_reset(&mut self) -> _PCIE1_PHY_PASSED_RESETW {
        _PCIE1_PHY_PASSED_RESETW { w: self }
    }
    #[doc = "Bit 7 - Interrupt generated to indicate that DISPLAY passed software reset and is ready to be used"]
    #[inline]
    pub fn display_passed_reset(&mut self) -> _DISPLAY_PASSED_RESETW {
        _DISPLAY_PASSED_RESETW { w: self }
    }
    #[doc = "Bit 8 - Interrupt generated to indicate that m4 core passed software reset and is ready to be used"]
    #[inline]
    pub fn m4c_passed_reset(&mut self) -> _M4C_PASSED_RESETW {
        _M4C_PASSED_RESETW { w: self }
    }
    #[doc = "Bit 9 - Interrupt generated to indicate that m4 platform passed software reset and is ready to be used"]
    #[inline]
    pub fn m4p_passed_reset(&mut self) -> _M4P_PASSED_RESETW {
        _M4P_PASSED_RESETW { w: self }
    }
    #[doc = "Bit 10 - Interrupt generated to indicate that GPU passed software reset and is ready to be used"]
    #[inline]
    pub fn gpu_passed_reset(&mut self) -> _GPU_PASSED_RESETW {
        _GPU_PASSED_RESETW { w: self }
    }
    #[doc = "Bit 11 - Interrupt generated to indicate that VPU passed software reset and is ready to be used"]
    #[inline]
    pub fn vpu_passed_reset(&mut self) -> _VPU_PASSED_RESETW {
        _VPU_PASSED_RESETW { w: self }
    }
}
