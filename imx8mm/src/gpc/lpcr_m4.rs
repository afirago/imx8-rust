#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LPCR_M4 {
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
#[doc = "Possible values of the field `LPM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPM0R {
    #[doc = "Remain in RUN mode"]
    LPM0_0,
    #[doc = "Transfer to WAIT mode"]
    LPM0_1,
    #[doc = "Transfer to STOP mode"]
    LPM0_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LPM0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPM0R::LPM0_0 => 0,
            LPM0R::LPM0_1 => 1,
            LPM0R::LPM0_2 => 2,
            LPM0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPM0R {
        match value {
            0 => LPM0R::LPM0_0,
            1 => LPM0R::LPM0_1,
            2 => LPM0R::LPM0_2,
            i => LPM0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LPM0_0`"]
    #[inline]
    pub fn is_lpm0_0(&self) -> bool {
        *self == LPM0R::LPM0_0
    }
    #[doc = "Checks if the value of the field is `LPM0_1`"]
    #[inline]
    pub fn is_lpm0_1(&self) -> bool {
        *self == LPM0R::LPM0_1
    }
    #[doc = "Checks if the value of the field is `LPM0_2`"]
    #[inline]
    pub fn is_lpm0_2(&self) -> bool {
        *self == LPM0R::LPM0_2
    }
}
#[doc = r" Value of the field"]
pub struct EN_M4_PDNR {
    bits: bool,
}
impl EN_M4_PDNR {
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
pub struct EN_M4_PUPR {
    bits: bool,
}
impl EN_M4_PUPR {
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
#[doc = "Possible values of the field `CPU_CLK_ON_LPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU_CLK_ON_LPMR {
    #[doc = "M4 clock disabled on wait/stop mode."]
    CPU_CLK_ON_LPM_0,
    #[doc = "M4 clock enabled on wait/stop mode."]
    CPU_CLK_ON_LPM_1,
}
impl CPU_CLK_ON_LPMR {
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
            CPU_CLK_ON_LPMR::CPU_CLK_ON_LPM_0 => false,
            CPU_CLK_ON_LPMR::CPU_CLK_ON_LPM_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPU_CLK_ON_LPMR {
        match value {
            false => CPU_CLK_ON_LPMR::CPU_CLK_ON_LPM_0,
            true => CPU_CLK_ON_LPMR::CPU_CLK_ON_LPM_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPU_CLK_ON_LPM_0`"]
    #[inline]
    pub fn is_cpu_clk_on_lpm_0(&self) -> bool {
        *self == CPU_CLK_ON_LPMR::CPU_CLK_ON_LPM_0
    }
    #[doc = "Checks if the value of the field is `CPU_CLK_ON_LPM_1`"]
    #[inline]
    pub fn is_cpu_clk_on_lpm_1(&self) -> bool {
        *self == CPU_CLK_ON_LPMR::CPU_CLK_ON_LPM_1
    }
}
#[doc = "Possible values of the field `MASK_M4_WFI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_M4_WFIR {
    #[doc = "WFI for M4 is not masked"]
    MASK_M4_WFI_0,
    #[doc = "WFI for M4 is masked"]
    MASK_M4_WFI_1,
}
impl MASK_M4_WFIR {
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
            MASK_M4_WFIR::MASK_M4_WFI_0 => false,
            MASK_M4_WFIR::MASK_M4_WFI_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_M4_WFIR {
        match value {
            false => MASK_M4_WFIR::MASK_M4_WFI_0,
            true => MASK_M4_WFIR::MASK_M4_WFI_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_M4_WFI_0`"]
    #[inline]
    pub fn is_mask_m4_wfi_0(&self) -> bool {
        *self == MASK_M4_WFIR::MASK_M4_WFI_0
    }
    #[doc = "Checks if the value of the field is `MASK_M4_WFI_1`"]
    #[inline]
    pub fn is_mask_m4_wfi_1(&self) -> bool {
        *self == MASK_M4_WFIR::MASK_M4_WFI_1
    }
}
#[doc = "Possible values of the field `MASK_DSM_TRIGGER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_DSM_TRIGGERR {
    #[doc = "DSM trigger of M4 platform will not be masked"]
    MASK_DSM_TRIGGER_0,
    #[doc = "DSM trigger of M4 platform will be masked"]
    MASK_DSM_TRIGGER_1,
}
impl MASK_DSM_TRIGGERR {
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
            MASK_DSM_TRIGGERR::MASK_DSM_TRIGGER_0 => false,
            MASK_DSM_TRIGGERR::MASK_DSM_TRIGGER_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_DSM_TRIGGERR {
        match value {
            false => MASK_DSM_TRIGGERR::MASK_DSM_TRIGGER_0,
            true => MASK_DSM_TRIGGERR::MASK_DSM_TRIGGER_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_DSM_TRIGGER_0`"]
    #[inline]
    pub fn is_mask_dsm_trigger_0(&self) -> bool {
        *self == MASK_DSM_TRIGGERR::MASK_DSM_TRIGGER_0
    }
    #[doc = "Checks if the value of the field is `MASK_DSM_TRIGGER_1`"]
    #[inline]
    pub fn is_mask_dsm_trigger_1(&self) -> bool {
        *self == MASK_DSM_TRIGGERR::MASK_DSM_TRIGGER_1
    }
}
#[doc = "Values that can be written to the field `LPM0`"]
pub enum LPM0W {
    #[doc = "Remain in RUN mode"]
    LPM0_0,
    #[doc = "Transfer to WAIT mode"]
    LPM0_1,
    #[doc = "Transfer to STOP mode"]
    LPM0_2,
}
impl LPM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPM0W::LPM0_0 => 0,
            LPM0W::LPM0_1 => 1,
            LPM0W::LPM0_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPM0W<'a> {
    w: &'a mut W,
}
impl<'a> _LPM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPM0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Remain in RUN mode"]
    #[inline]
    pub fn lpm0_0(self) -> &'a mut W {
        self.variant(LPM0W::LPM0_0)
    }
    #[doc = "Transfer to WAIT mode"]
    #[inline]
    pub fn lpm0_1(self) -> &'a mut W {
        self.variant(LPM0W::LPM0_1)
    }
    #[doc = "Transfer to STOP mode"]
    #[inline]
    pub fn lpm0_2(self) -> &'a mut W {
        self.variant(LPM0W::LPM0_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EN_M4_PDNW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_M4_PDNW<'a> {
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
pub struct _EN_M4_PUPW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_M4_PUPW<'a> {
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
#[doc = "Values that can be written to the field `CPU_CLK_ON_LPM`"]
pub enum CPU_CLK_ON_LPMW {
    #[doc = "M4 clock disabled on wait/stop mode."]
    CPU_CLK_ON_LPM_0,
    #[doc = "M4 clock enabled on wait/stop mode."]
    CPU_CLK_ON_LPM_1,
}
impl CPU_CLK_ON_LPMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPU_CLK_ON_LPMW::CPU_CLK_ON_LPM_0 => false,
            CPU_CLK_ON_LPMW::CPU_CLK_ON_LPM_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPU_CLK_ON_LPMW<'a> {
    w: &'a mut W,
}
impl<'a> _CPU_CLK_ON_LPMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPU_CLK_ON_LPMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "M4 clock disabled on wait/stop mode."]
    #[inline]
    pub fn cpu_clk_on_lpm_0(self) -> &'a mut W {
        self.variant(CPU_CLK_ON_LPMW::CPU_CLK_ON_LPM_0)
    }
    #[doc = "M4 clock enabled on wait/stop mode."]
    #[inline]
    pub fn cpu_clk_on_lpm_1(self) -> &'a mut W {
        self.variant(CPU_CLK_ON_LPMW::CPU_CLK_ON_LPM_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MASK_M4_WFI`"]
pub enum MASK_M4_WFIW {
    #[doc = "WFI for M4 is not masked"]
    MASK_M4_WFI_0,
    #[doc = "WFI for M4 is masked"]
    MASK_M4_WFI_1,
}
impl MASK_M4_WFIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASK_M4_WFIW::MASK_M4_WFI_0 => false,
            MASK_M4_WFIW::MASK_M4_WFI_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_M4_WFIW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_M4_WFIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_M4_WFIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "WFI for M4 is not masked"]
    #[inline]
    pub fn mask_m4_wfi_0(self) -> &'a mut W {
        self.variant(MASK_M4_WFIW::MASK_M4_WFI_0)
    }
    #[doc = "WFI for M4 is masked"]
    #[inline]
    pub fn mask_m4_wfi_1(self) -> &'a mut W {
        self.variant(MASK_M4_WFIW::MASK_M4_WFI_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MASK_DSM_TRIGGER`"]
pub enum MASK_DSM_TRIGGERW {
    #[doc = "DSM trigger of M4 platform will not be masked"]
    MASK_DSM_TRIGGER_0,
    #[doc = "DSM trigger of M4 platform will be masked"]
    MASK_DSM_TRIGGER_1,
}
impl MASK_DSM_TRIGGERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASK_DSM_TRIGGERW::MASK_DSM_TRIGGER_0 => false,
            MASK_DSM_TRIGGERW::MASK_DSM_TRIGGER_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASK_DSM_TRIGGERW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_DSM_TRIGGERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_DSM_TRIGGERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DSM trigger of M4 platform will not be masked"]
    #[inline]
    pub fn mask_dsm_trigger_0(self) -> &'a mut W {
        self.variant(MASK_DSM_TRIGGERW::MASK_DSM_TRIGGER_0)
    }
    #[doc = "DSM trigger of M4 platform will be masked"]
    #[inline]
    pub fn mask_dsm_trigger_1(self) -> &'a mut W {
        self.variant(MASK_DSM_TRIGGERW::MASK_DSM_TRIGGER_1)
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
    #[doc = "Bits 0:1 - Setting the low power mode that system will enter on next assertion of dsm_request signal."]
    #[inline]
    pub fn lpm0(&self) -> LPM0R {
        LPM0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Enable m4 virtual PGC power down with LPM enter"]
    #[inline]
    pub fn en_m4_pdn(&self) -> EN_M4_PDNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EN_M4_PDNR { bits }
    }
    #[doc = "Bit 3 - Enable m4 virtual PGC power up with LPM enter"]
    #[inline]
    pub fn en_m4_pup(&self) -> EN_M4_PUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EN_M4_PUPR { bits }
    }
    #[doc = "Bit 14 - Define if M4 clocks will be disabled on wait/stop mode."]
    #[inline]
    pub fn cpu_clk_on_lpm(&self) -> CPU_CLK_ON_LPMR {
        CPU_CLK_ON_LPMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - M4 WFI Mask"]
    #[inline]
    pub fn mask_m4_wfi(&self) -> MASK_M4_WFIR {
        MASK_M4_WFIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - M4 WFI Mask"]
    #[inline]
    pub fn mask_dsm_trigger(&self) -> MASK_DSM_TRIGGERR {
        MASK_DSM_TRIGGERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 16368 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Setting the low power mode that system will enter on next assertion of dsm_request signal."]
    #[inline]
    pub fn lpm0(&mut self) -> _LPM0W {
        _LPM0W { w: self }
    }
    #[doc = "Bit 2 - Enable m4 virtual PGC power down with LPM enter"]
    #[inline]
    pub fn en_m4_pdn(&mut self) -> _EN_M4_PDNW {
        _EN_M4_PDNW { w: self }
    }
    #[doc = "Bit 3 - Enable m4 virtual PGC power up with LPM enter"]
    #[inline]
    pub fn en_m4_pup(&mut self) -> _EN_M4_PUPW {
        _EN_M4_PUPW { w: self }
    }
    #[doc = "Bit 14 - Define if M4 clocks will be disabled on wait/stop mode."]
    #[inline]
    pub fn cpu_clk_on_lpm(&mut self) -> _CPU_CLK_ON_LPMW {
        _CPU_CLK_ON_LPMW { w: self }
    }
    #[doc = "Bit 16 - M4 WFI Mask"]
    #[inline]
    pub fn mask_m4_wfi(&mut self) -> _MASK_M4_WFIW {
        _MASK_M4_WFIW { w: self }
    }
    #[doc = "Bit 31 - M4 WFI Mask"]
    #[inline]
    pub fn mask_dsm_trigger(&mut self) -> _MASK_DSM_TRIGGERW {
        _MASK_DSM_TRIGGERW { w: self }
    }
}
