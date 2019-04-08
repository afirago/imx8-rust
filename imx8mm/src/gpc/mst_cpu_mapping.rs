#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MST_CPU_MAPPING {
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
#[doc = "Possible values of the field `MST0_CPU_MAPPING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MST0_CPU_MAPPINGR {
    #[doc = "GPC will not send out power off requirement"]
    MST0_CPU_MAPPING_0,
    #[doc = "GPC will send out power off requirement"]
    MST0_CPU_MAPPING_1,
}
impl MST0_CPU_MAPPINGR {
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
            MST0_CPU_MAPPINGR::MST0_CPU_MAPPING_0 => false,
            MST0_CPU_MAPPINGR::MST0_CPU_MAPPING_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MST0_CPU_MAPPINGR {
        match value {
            false => MST0_CPU_MAPPINGR::MST0_CPU_MAPPING_0,
            true => MST0_CPU_MAPPINGR::MST0_CPU_MAPPING_1,
        }
    }
    #[doc = "Checks if the value of the field is `MST0_CPU_MAPPING_0`"]
    #[inline]
    pub fn is_mst0_cpu_mapping_0(&self) -> bool {
        *self == MST0_CPU_MAPPINGR::MST0_CPU_MAPPING_0
    }
    #[doc = "Checks if the value of the field is `MST0_CPU_MAPPING_1`"]
    #[inline]
    pub fn is_mst0_cpu_mapping_1(&self) -> bool {
        *self == MST0_CPU_MAPPINGR::MST0_CPU_MAPPING_1
    }
}
#[doc = "Possible values of the field `MST1_CPU_MAPPING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MST1_CPU_MAPPINGR {
    #[doc = "GPC will not send out power off requirement"]
    MST1_CPU_MAPPING_0,
    #[doc = "GPC will send out power off requirement"]
    MST1_CPU_MAPPING_1,
}
impl MST1_CPU_MAPPINGR {
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
            MST1_CPU_MAPPINGR::MST1_CPU_MAPPING_0 => false,
            MST1_CPU_MAPPINGR::MST1_CPU_MAPPING_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MST1_CPU_MAPPINGR {
        match value {
            false => MST1_CPU_MAPPINGR::MST1_CPU_MAPPING_0,
            true => MST1_CPU_MAPPINGR::MST1_CPU_MAPPING_1,
        }
    }
    #[doc = "Checks if the value of the field is `MST1_CPU_MAPPING_0`"]
    #[inline]
    pub fn is_mst1_cpu_mapping_0(&self) -> bool {
        *self == MST1_CPU_MAPPINGR::MST1_CPU_MAPPING_0
    }
    #[doc = "Checks if the value of the field is `MST1_CPU_MAPPING_1`"]
    #[inline]
    pub fn is_mst1_cpu_mapping_1(&self) -> bool {
        *self == MST1_CPU_MAPPINGR::MST1_CPU_MAPPING_1
    }
}
#[doc = "Possible values of the field `MST2_CPU_MAPPING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MST2_CPU_MAPPINGR {
    #[doc = "GPC will not send out power off requirement"]
    MST2_CPU_MAPPING_0,
    #[doc = "GPC will send out power off requirement"]
    MST2_CPU_MAPPING_1,
}
impl MST2_CPU_MAPPINGR {
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
            MST2_CPU_MAPPINGR::MST2_CPU_MAPPING_0 => false,
            MST2_CPU_MAPPINGR::MST2_CPU_MAPPING_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MST2_CPU_MAPPINGR {
        match value {
            false => MST2_CPU_MAPPINGR::MST2_CPU_MAPPING_0,
            true => MST2_CPU_MAPPINGR::MST2_CPU_MAPPING_1,
        }
    }
    #[doc = "Checks if the value of the field is `MST2_CPU_MAPPING_0`"]
    #[inline]
    pub fn is_mst2_cpu_mapping_0(&self) -> bool {
        *self == MST2_CPU_MAPPINGR::MST2_CPU_MAPPING_0
    }
    #[doc = "Checks if the value of the field is `MST2_CPU_MAPPING_1`"]
    #[inline]
    pub fn is_mst2_cpu_mapping_1(&self) -> bool {
        *self == MST2_CPU_MAPPINGR::MST2_CPU_MAPPING_1
    }
}
#[doc = "Values that can be written to the field `MST0_CPU_MAPPING`"]
pub enum MST0_CPU_MAPPINGW {
    #[doc = "GPC will not send out power off requirement"]
    MST0_CPU_MAPPING_0,
    #[doc = "GPC will send out power off requirement"]
    MST0_CPU_MAPPING_1,
}
impl MST0_CPU_MAPPINGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MST0_CPU_MAPPINGW::MST0_CPU_MAPPING_0 => false,
            MST0_CPU_MAPPINGW::MST0_CPU_MAPPING_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MST0_CPU_MAPPINGW<'a> {
    w: &'a mut W,
}
impl<'a> _MST0_CPU_MAPPINGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MST0_CPU_MAPPINGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "GPC will not send out power off requirement"]
    #[inline]
    pub fn mst0_cpu_mapping_0(self) -> &'a mut W {
        self.variant(MST0_CPU_MAPPINGW::MST0_CPU_MAPPING_0)
    }
    #[doc = "GPC will send out power off requirement"]
    #[inline]
    pub fn mst0_cpu_mapping_1(self) -> &'a mut W {
        self.variant(MST0_CPU_MAPPINGW::MST0_CPU_MAPPING_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MST1_CPU_MAPPING`"]
pub enum MST1_CPU_MAPPINGW {
    #[doc = "GPC will not send out power off requirement"]
    MST1_CPU_MAPPING_0,
    #[doc = "GPC will send out power off requirement"]
    MST1_CPU_MAPPING_1,
}
impl MST1_CPU_MAPPINGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MST1_CPU_MAPPINGW::MST1_CPU_MAPPING_0 => false,
            MST1_CPU_MAPPINGW::MST1_CPU_MAPPING_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MST1_CPU_MAPPINGW<'a> {
    w: &'a mut W,
}
impl<'a> _MST1_CPU_MAPPINGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MST1_CPU_MAPPINGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "GPC will not send out power off requirement"]
    #[inline]
    pub fn mst1_cpu_mapping_0(self) -> &'a mut W {
        self.variant(MST1_CPU_MAPPINGW::MST1_CPU_MAPPING_0)
    }
    #[doc = "GPC will send out power off requirement"]
    #[inline]
    pub fn mst1_cpu_mapping_1(self) -> &'a mut W {
        self.variant(MST1_CPU_MAPPINGW::MST1_CPU_MAPPING_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MST2_CPU_MAPPING`"]
pub enum MST2_CPU_MAPPINGW {
    #[doc = "GPC will not send out power off requirement"]
    MST2_CPU_MAPPING_0,
    #[doc = "GPC will send out power off requirement"]
    MST2_CPU_MAPPING_1,
}
impl MST2_CPU_MAPPINGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MST2_CPU_MAPPINGW::MST2_CPU_MAPPING_0 => false,
            MST2_CPU_MAPPINGW::MST2_CPU_MAPPING_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MST2_CPU_MAPPINGW<'a> {
    w: &'a mut W,
}
impl<'a> _MST2_CPU_MAPPINGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MST2_CPU_MAPPINGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "GPC will not send out power off requirement"]
    #[inline]
    pub fn mst2_cpu_mapping_0(self) -> &'a mut W {
        self.variant(MST2_CPU_MAPPINGW::MST2_CPU_MAPPING_0)
    }
    #[doc = "GPC will send out power off requirement"]
    #[inline]
    pub fn mst2_cpu_mapping_1(self) -> &'a mut W {
        self.variant(MST2_CPU_MAPPINGW::MST2_CPU_MAPPING_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - MASTER0 CPU Mapping"]
    #[inline]
    pub fn mst0_cpu_mapping(&self) -> MST0_CPU_MAPPINGR {
        MST0_CPU_MAPPINGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - MASTER0 CPU Mapping"]
    #[inline]
    pub fn mst1_cpu_mapping(&self) -> MST1_CPU_MAPPINGR {
        MST1_CPU_MAPPINGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - MASTER2 CPU Mapping"]
    #[inline]
    pub fn mst2_cpu_mapping(&self) -> MST2_CPU_MAPPINGR {
        MST2_CPU_MAPPINGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 255 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - MASTER0 CPU Mapping"]
    #[inline]
    pub fn mst0_cpu_mapping(&mut self) -> _MST0_CPU_MAPPINGW {
        _MST0_CPU_MAPPINGW { w: self }
    }
    #[doc = "Bit 1 - MASTER0 CPU Mapping"]
    #[inline]
    pub fn mst1_cpu_mapping(&mut self) -> _MST1_CPU_MAPPINGW {
        _MST1_CPU_MAPPINGW { w: self }
    }
    #[doc = "Bit 2 - MASTER2 CPU Mapping"]
    #[inline]
    pub fn mst2_cpu_mapping(&mut self) -> _MST2_CPU_MAPPINGW {
        _MST2_CPU_MAPPINGW { w: self }
    }
}
