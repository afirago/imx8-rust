#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ANAMIX_PLL_MNIT_CTL {
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
pub struct CLKOUT1_OUTPUT_DIV_VALR {
    bits: u8,
}
impl CLKOUT1_OUTPUT_DIV_VALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CLKOUT1_OUTPUT_SELR {
    bits: u8,
}
impl CLKOUT1_OUTPUT_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CLKOUT1_OUTPUT_CKER {
    bits: bool,
}
impl CLKOUT1_OUTPUT_CKER {
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
pub struct CLKOUT2_OUTPUT_DIV_VALR {
    bits: u8,
}
impl CLKOUT2_OUTPUT_DIV_VALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CLKOUT2_OUTPUT_SELR {
    bits: u8,
}
impl CLKOUT2_OUTPUT_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CLKOUT2_OUTPUT_CKER {
    bits: bool,
}
impl CLKOUT2_OUTPUT_CKER {
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
pub struct _CLKOUT1_OUTPUT_DIV_VALW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUT1_OUTPUT_DIV_VALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKOUT1_OUTPUT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUT1_OUTPUT_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKOUT1_OUTPUT_CKEW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUT1_OUTPUT_CKEW<'a> {
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
pub struct _CLKOUT2_OUTPUT_DIV_VALW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUT2_OUTPUT_DIV_VALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKOUT2_OUTPUT_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUT2_OUTPUT_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKOUT2_OUTPUT_CKEW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUT2_OUTPUT_CKEW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - CLKOUT1 output divide value"]
    #[inline]
    pub fn clkout1_output_div_val(&self) -> CLKOUT1_OUTPUT_DIV_VALR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CLKOUT1_OUTPUT_DIV_VALR { bits }
    }
    #[doc = "Bits 4:7 - CLKOUT1 Monitor output clock select 4'b0000 : audio_pll1_clk 4'b0001 : audio_pll2_clk 4'b0010 : video_pll1_clk 4'b0011 : reserved 4'b0100 : misc_mnit_clk 4'b0101 : gpu_pll_clk 4'b0110 : vpu_pll_clk 4'b0111 : arm_pll_clk 4'b1000 : system_pll1_clk 4'b1001 : system_pll2_clk 4'b1010 : system_pll3_clk 4'b1011 : CLKIN1 4'b1100 : CLKIN2 4'b1101 : sysosc_24m_clk 4'b1110 : reserved 4'b1111 : osc_32k_clk"]
    #[inline]
    pub fn clkout1_output_sel(&self) -> CLKOUT1_OUTPUT_SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CLKOUT1_OUTPUT_SELR { bits }
    }
    #[doc = "Bit 8 - CLKOUT1 Monitor output enable"]
    #[inline]
    pub fn clkout1_output_cke(&self) -> CLKOUT1_OUTPUT_CKER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKOUT1_OUTPUT_CKER { bits }
    }
    #[doc = "Bits 16:19 - CLKOUT2 output divide value"]
    #[inline]
    pub fn clkout2_output_div_val(&self) -> CLKOUT2_OUTPUT_DIV_VALR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CLKOUT2_OUTPUT_DIV_VALR { bits }
    }
    #[doc = "Bits 20:23 - CLKOUT2 Monitor output clock select 4'b0000 : audio_pll1_clk 4'b0001 : audio_pll2_clk 4'b0010 : video_pll1_clk 4'b0011 : reserved 4'b0100 : misc_mnit_clk 4'b0101 : gpu_pll_clk 4'b0110 : vpu_pll_clk 4'b0111 : arm_pll_clk 4'b1000 : system_pll1_clk 4'b1001 : system_pll2_clk 4'b1010 : system_pll3_clk 4'b1011 : CLKIN1 4'b1100 : CLKIN2 4'b1101 : sysosc_24m_clk 4'b1110 : reserved 4'b1111 : osc_32k_clk"]
    #[inline]
    pub fn clkout2_output_sel(&self) -> CLKOUT2_OUTPUT_SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CLKOUT2_OUTPUT_SELR { bits }
    }
    #[doc = "Bit 24 - CLKOUT2 Monitor output enable"]
    #[inline]
    pub fn clkout2_output_cke(&self) -> CLKOUT2_OUTPUT_CKER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKOUT2_OUTPUT_CKER { bits }
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
    #[doc = "Bits 0:3 - CLKOUT1 output divide value"]
    #[inline]
    pub fn clkout1_output_div_val(&mut self) -> _CLKOUT1_OUTPUT_DIV_VALW {
        _CLKOUT1_OUTPUT_DIV_VALW { w: self }
    }
    #[doc = "Bits 4:7 - CLKOUT1 Monitor output clock select 4'b0000 : audio_pll1_clk 4'b0001 : audio_pll2_clk 4'b0010 : video_pll1_clk 4'b0011 : reserved 4'b0100 : misc_mnit_clk 4'b0101 : gpu_pll_clk 4'b0110 : vpu_pll_clk 4'b0111 : arm_pll_clk 4'b1000 : system_pll1_clk 4'b1001 : system_pll2_clk 4'b1010 : system_pll3_clk 4'b1011 : CLKIN1 4'b1100 : CLKIN2 4'b1101 : sysosc_24m_clk 4'b1110 : reserved 4'b1111 : osc_32k_clk"]
    #[inline]
    pub fn clkout1_output_sel(&mut self) -> _CLKOUT1_OUTPUT_SELW {
        _CLKOUT1_OUTPUT_SELW { w: self }
    }
    #[doc = "Bit 8 - CLKOUT1 Monitor output enable"]
    #[inline]
    pub fn clkout1_output_cke(&mut self) -> _CLKOUT1_OUTPUT_CKEW {
        _CLKOUT1_OUTPUT_CKEW { w: self }
    }
    #[doc = "Bits 16:19 - CLKOUT2 output divide value"]
    #[inline]
    pub fn clkout2_output_div_val(&mut self) -> _CLKOUT2_OUTPUT_DIV_VALW {
        _CLKOUT2_OUTPUT_DIV_VALW { w: self }
    }
    #[doc = "Bits 20:23 - CLKOUT2 Monitor output clock select 4'b0000 : audio_pll1_clk 4'b0001 : audio_pll2_clk 4'b0010 : video_pll1_clk 4'b0011 : reserved 4'b0100 : misc_mnit_clk 4'b0101 : gpu_pll_clk 4'b0110 : vpu_pll_clk 4'b0111 : arm_pll_clk 4'b1000 : system_pll1_clk 4'b1001 : system_pll2_clk 4'b1010 : system_pll3_clk 4'b1011 : CLKIN1 4'b1100 : CLKIN2 4'b1101 : sysosc_24m_clk 4'b1110 : reserved 4'b1111 : osc_32k_clk"]
    #[inline]
    pub fn clkout2_output_sel(&mut self) -> _CLKOUT2_OUTPUT_SELW {
        _CLKOUT2_OUTPUT_SELW { w: self }
    }
    #[doc = "Bit 24 - CLKOUT2 Monitor output enable"]
    #[inline]
    pub fn clkout2_output_cke(&mut self) -> _CLKOUT2_OUTPUT_CKEW {
        _CLKOUT2_OUTPUT_CKEW { w: self }
    }
}
