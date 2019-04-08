#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MISC {
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
#[doc = "Possible values of the field `M4_SLEEP_HOLD_REQ_B`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M4_SLEEP_HOLD_REQ_BR {
    #[doc = "Hold M4 platform in sleep mode. This bit is a software control bit to M4 platform."]
    M4_SLEEP_HOLD_REQ_B_0,
    #[doc = "Don't hold M4 platform in sleep mode."]
    M4_SLEEP_HOLD_REQ_B_1,
}
impl M4_SLEEP_HOLD_REQ_BR {
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
            M4_SLEEP_HOLD_REQ_BR::M4_SLEEP_HOLD_REQ_B_0 => false,
            M4_SLEEP_HOLD_REQ_BR::M4_SLEEP_HOLD_REQ_B_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M4_SLEEP_HOLD_REQ_BR {
        match value {
            false => M4_SLEEP_HOLD_REQ_BR::M4_SLEEP_HOLD_REQ_B_0,
            true => M4_SLEEP_HOLD_REQ_BR::M4_SLEEP_HOLD_REQ_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `M4_SLEEP_HOLD_REQ_B_0`"]
    #[inline]
    pub fn is_m4_sleep_hold_req_b_0(&self) -> bool {
        *self == M4_SLEEP_HOLD_REQ_BR::M4_SLEEP_HOLD_REQ_B_0
    }
    #[doc = "Checks if the value of the field is `M4_SLEEP_HOLD_REQ_B_1`"]
    #[inline]
    pub fn is_m4_sleep_hold_req_b_1(&self) -> bool {
        *self == M4_SLEEP_HOLD_REQ_BR::M4_SLEEP_HOLD_REQ_B_1
    }
}
#[doc = "Possible values of the field `A53_SLEEP_HOLD_REQ_B`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum A53_SLEEP_HOLD_REQ_BR {
    #[doc = "Hold A53 platform in sleep mode. This bit is a software control bit to A53 platform."]
    A53_SLEEP_HOLD_REQ_B_0,
    #[doc = "Don't hold A53 platform in sleep mode."]
    A53_SLEEP_HOLD_REQ_B_1,
}
impl A53_SLEEP_HOLD_REQ_BR {
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
            A53_SLEEP_HOLD_REQ_BR::A53_SLEEP_HOLD_REQ_B_0 => false,
            A53_SLEEP_HOLD_REQ_BR::A53_SLEEP_HOLD_REQ_B_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> A53_SLEEP_HOLD_REQ_BR {
        match value {
            false => A53_SLEEP_HOLD_REQ_BR::A53_SLEEP_HOLD_REQ_B_0,
            true => A53_SLEEP_HOLD_REQ_BR::A53_SLEEP_HOLD_REQ_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `A53_SLEEP_HOLD_REQ_B_0`"]
    #[inline]
    pub fn is_a53_sleep_hold_req_b_0(&self) -> bool {
        *self == A53_SLEEP_HOLD_REQ_BR::A53_SLEEP_HOLD_REQ_B_0
    }
    #[doc = "Checks if the value of the field is `A53_SLEEP_HOLD_REQ_B_1`"]
    #[inline]
    pub fn is_a53_sleep_hold_req_b_1(&self) -> bool {
        *self == A53_SLEEP_HOLD_REQ_BR::A53_SLEEP_HOLD_REQ_B_1
    }
}
#[doc = "Possible values of the field `GPC_IRQ_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPC_IRQ_MASKR {
    #[doc = "Not masked"]
    GPC_IRQ_MASK_0,
    #[doc = "Interrupt / event is masked"]
    GPC_IRQ_MASK_1,
}
impl GPC_IRQ_MASKR {
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
            GPC_IRQ_MASKR::GPC_IRQ_MASK_0 => false,
            GPC_IRQ_MASKR::GPC_IRQ_MASK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPC_IRQ_MASKR {
        match value {
            false => GPC_IRQ_MASKR::GPC_IRQ_MASK_0,
            true => GPC_IRQ_MASKR::GPC_IRQ_MASK_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPC_IRQ_MASK_0`"]
    #[inline]
    pub fn is_gpc_irq_mask_0(&self) -> bool {
        *self == GPC_IRQ_MASKR::GPC_IRQ_MASK_0
    }
    #[doc = "Checks if the value of the field is `GPC_IRQ_MASK_1`"]
    #[inline]
    pub fn is_gpc_irq_mask_1(&self) -> bool {
        *self == GPC_IRQ_MASKR::GPC_IRQ_MASK_1
    }
}
#[doc = "Possible values of the field `M4_PDN_REQ_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M4_PDN_REQ_MASKR {
    #[doc = "M4 power down request to virtual M4 PGC will be masked."]
    M4_PDN_REQ_MASK_0,
    #[doc = "M4 power down request to virtual M4 PGC will not be masked. Set this bit to 1'b1 when M4 virtual PGC is used."]
    M4_PDN_REQ_MASK_1,
}
impl M4_PDN_REQ_MASKR {
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
            M4_PDN_REQ_MASKR::M4_PDN_REQ_MASK_0 => false,
            M4_PDN_REQ_MASKR::M4_PDN_REQ_MASK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M4_PDN_REQ_MASKR {
        match value {
            false => M4_PDN_REQ_MASKR::M4_PDN_REQ_MASK_0,
            true => M4_PDN_REQ_MASKR::M4_PDN_REQ_MASK_1,
        }
    }
    #[doc = "Checks if the value of the field is `M4_PDN_REQ_MASK_0`"]
    #[inline]
    pub fn is_m4_pdn_req_mask_0(&self) -> bool {
        *self == M4_PDN_REQ_MASKR::M4_PDN_REQ_MASK_0
    }
    #[doc = "Checks if the value of the field is `M4_PDN_REQ_MASK_1`"]
    #[inline]
    pub fn is_m4_pdn_req_mask_1(&self) -> bool {
        *self == M4_PDN_REQ_MASKR::M4_PDN_REQ_MASK_1
    }
}
#[doc = r" Value of the field"]
pub struct A53_BYPASS_PUP_MASKR {
    bits: bool,
}
impl A53_BYPASS_PUP_MASKR {
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
pub struct M4_BYPASS_PUP_MASKR {
    bits: bool,
}
impl M4_BYPASS_PUP_MASKR {
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
#[doc = "Values that can be written to the field `M4_SLEEP_HOLD_REQ_B`"]
pub enum M4_SLEEP_HOLD_REQ_BW {
    #[doc = "Hold M4 platform in sleep mode. This bit is a software control bit to M4 platform."]
    M4_SLEEP_HOLD_REQ_B_0,
    #[doc = "Don't hold M4 platform in sleep mode."]
    M4_SLEEP_HOLD_REQ_B_1,
}
impl M4_SLEEP_HOLD_REQ_BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M4_SLEEP_HOLD_REQ_BW::M4_SLEEP_HOLD_REQ_B_0 => false,
            M4_SLEEP_HOLD_REQ_BW::M4_SLEEP_HOLD_REQ_B_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M4_SLEEP_HOLD_REQ_BW<'a> {
    w: &'a mut W,
}
impl<'a> _M4_SLEEP_HOLD_REQ_BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M4_SLEEP_HOLD_REQ_BW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hold M4 platform in sleep mode. This bit is a software control bit to M4 platform."]
    #[inline]
    pub fn m4_sleep_hold_req_b_0(self) -> &'a mut W {
        self.variant(M4_SLEEP_HOLD_REQ_BW::M4_SLEEP_HOLD_REQ_B_0)
    }
    #[doc = "Don't hold M4 platform in sleep mode."]
    #[inline]
    pub fn m4_sleep_hold_req_b_1(self) -> &'a mut W {
        self.variant(M4_SLEEP_HOLD_REQ_BW::M4_SLEEP_HOLD_REQ_B_1)
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
#[doc = "Values that can be written to the field `A53_SLEEP_HOLD_REQ_B`"]
pub enum A53_SLEEP_HOLD_REQ_BW {
    #[doc = "Hold A53 platform in sleep mode. This bit is a software control bit to A53 platform."]
    A53_SLEEP_HOLD_REQ_B_0,
    #[doc = "Don't hold A53 platform in sleep mode."]
    A53_SLEEP_HOLD_REQ_B_1,
}
impl A53_SLEEP_HOLD_REQ_BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            A53_SLEEP_HOLD_REQ_BW::A53_SLEEP_HOLD_REQ_B_0 => false,
            A53_SLEEP_HOLD_REQ_BW::A53_SLEEP_HOLD_REQ_B_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _A53_SLEEP_HOLD_REQ_BW<'a> {
    w: &'a mut W,
}
impl<'a> _A53_SLEEP_HOLD_REQ_BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: A53_SLEEP_HOLD_REQ_BW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hold A53 platform in sleep mode. This bit is a software control bit to A53 platform."]
    #[inline]
    pub fn a53_sleep_hold_req_b_0(self) -> &'a mut W {
        self.variant(A53_SLEEP_HOLD_REQ_BW::A53_SLEEP_HOLD_REQ_B_0)
    }
    #[doc = "Don't hold A53 platform in sleep mode."]
    #[inline]
    pub fn a53_sleep_hold_req_b_1(self) -> &'a mut W {
        self.variant(A53_SLEEP_HOLD_REQ_BW::A53_SLEEP_HOLD_REQ_B_1)
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
#[doc = "Values that can be written to the field `GPC_IRQ_MASK`"]
pub enum GPC_IRQ_MASKW {
    #[doc = "Not masked"]
    GPC_IRQ_MASK_0,
    #[doc = "Interrupt / event is masked"]
    GPC_IRQ_MASK_1,
}
impl GPC_IRQ_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPC_IRQ_MASKW::GPC_IRQ_MASK_0 => false,
            GPC_IRQ_MASKW::GPC_IRQ_MASK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPC_IRQ_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _GPC_IRQ_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPC_IRQ_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not masked"]
    #[inline]
    pub fn gpc_irq_mask_0(self) -> &'a mut W {
        self.variant(GPC_IRQ_MASKW::GPC_IRQ_MASK_0)
    }
    #[doc = "Interrupt / event is masked"]
    #[inline]
    pub fn gpc_irq_mask_1(self) -> &'a mut W {
        self.variant(GPC_IRQ_MASKW::GPC_IRQ_MASK_1)
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
#[doc = "Values that can be written to the field `M4_PDN_REQ_MASK`"]
pub enum M4_PDN_REQ_MASKW {
    #[doc = "M4 power down request to virtual M4 PGC will be masked."]
    M4_PDN_REQ_MASK_0,
    #[doc = "M4 power down request to virtual M4 PGC will not be masked. Set this bit to 1'b1 when M4 virtual PGC is used."]
    M4_PDN_REQ_MASK_1,
}
impl M4_PDN_REQ_MASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M4_PDN_REQ_MASKW::M4_PDN_REQ_MASK_0 => false,
            M4_PDN_REQ_MASKW::M4_PDN_REQ_MASK_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M4_PDN_REQ_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _M4_PDN_REQ_MASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M4_PDN_REQ_MASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "M4 power down request to virtual M4 PGC will be masked."]
    #[inline]
    pub fn m4_pdn_req_mask_0(self) -> &'a mut W {
        self.variant(M4_PDN_REQ_MASKW::M4_PDN_REQ_MASK_0)
    }
    #[doc = "M4 power down request to virtual M4 PGC will not be masked. Set this bit to 1'b1 when M4 virtual PGC is used."]
    #[inline]
    pub fn m4_pdn_req_mask_1(self) -> &'a mut W {
        self.variant(M4_PDN_REQ_MASKW::M4_PDN_REQ_MASK_1)
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
#[doc = r" Proxy"]
pub struct _A53_BYPASS_PUP_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _A53_BYPASS_PUP_MASKW<'a> {
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
pub struct _M4_BYPASS_PUP_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _M4_BYPASS_PUP_MASKW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - M4 sleep hold"]
    #[inline]
    pub fn m4_sleep_hold_req_b(&self) -> M4_SLEEP_HOLD_REQ_BR {
        M4_SLEEP_HOLD_REQ_BR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - A53 sleep hold"]
    #[inline]
    pub fn a53_sleep_hold_req_b(&self) -> A53_SLEEP_HOLD_REQ_BR {
        A53_SLEEP_HOLD_REQ_BR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - GPC interrupt/event masking"]
    #[inline]
    pub fn gpc_irq_mask(&self) -> GPC_IRQ_MASKR {
        GPC_IRQ_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - M4 power-down mask"]
    #[inline]
    pub fn m4_pdn_req_mask(&self) -> M4_PDN_REQ_MASKR {
        M4_PDN_REQ_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - no description available"]
    #[inline]
    pub fn a53_bypass_pup_mask(&self) -> A53_BYPASS_PUP_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A53_BYPASS_PUP_MASKR { bits }
    }
    #[doc = "Bit 25 - no description available"]
    #[inline]
    pub fn m4_bypass_pup_mask(&self) -> M4_BYPASS_PUP_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        M4_BYPASS_PUP_MASKR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 33 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - M4 sleep hold"]
    #[inline]
    pub fn m4_sleep_hold_req_b(&mut self) -> _M4_SLEEP_HOLD_REQ_BW {
        _M4_SLEEP_HOLD_REQ_BW { w: self }
    }
    #[doc = "Bit 1 - A53 sleep hold"]
    #[inline]
    pub fn a53_sleep_hold_req_b(&mut self) -> _A53_SLEEP_HOLD_REQ_BW {
        _A53_SLEEP_HOLD_REQ_BW { w: self }
    }
    #[doc = "Bit 5 - GPC interrupt/event masking"]
    #[inline]
    pub fn gpc_irq_mask(&mut self) -> _GPC_IRQ_MASKW {
        _GPC_IRQ_MASKW { w: self }
    }
    #[doc = "Bit 8 - M4 power-down mask"]
    #[inline]
    pub fn m4_pdn_req_mask(&mut self) -> _M4_PDN_REQ_MASKW {
        _M4_PDN_REQ_MASKW { w: self }
    }
    #[doc = "Bit 24 - no description available"]
    #[inline]
    pub fn a53_bypass_pup_mask(&mut self) -> _A53_BYPASS_PUP_MASKW {
        _A53_BYPASS_PUP_MASKW { w: self }
    }
    #[doc = "Bit 25 - no description available"]
    #[inline]
    pub fn m4_bypass_pup_mask(&mut self) -> _M4_BYPASS_PUP_MASKW {
        _M4_BYPASS_PUP_MASKW { w: self }
    }
}
