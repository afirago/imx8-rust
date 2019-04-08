#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYS_PLL1_GEN_CTRL {
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
#[doc = "Possible values of the field `PLL_REF_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_REF_CLK_SELR {
    #[doc = "SYS_XTAL"]
    PLL_REF_CLK_SEL_0,
    #[doc = "PAD_CLK"]
    PLL_REF_CLK_SEL_1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PLL_REF_CLK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PLL_REF_CLK_SELR::PLL_REF_CLK_SEL_0 => 0,
            PLL_REF_CLK_SELR::PLL_REF_CLK_SEL_1 => 1,
            PLL_REF_CLK_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PLL_REF_CLK_SELR {
        match value {
            0 => PLL_REF_CLK_SELR::PLL_REF_CLK_SEL_0,
            1 => PLL_REF_CLK_SELR::PLL_REF_CLK_SEL_1,
            i => PLL_REF_CLK_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_REF_CLK_SEL_0`"]
    #[inline]
    pub fn is_pll_ref_clk_sel_0(&self) -> bool {
        *self == PLL_REF_CLK_SELR::PLL_REF_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `PLL_REF_CLK_SEL_1`"]
    #[inline]
    pub fn is_pll_ref_clk_sel_1(&self) -> bool {
        *self == PLL_REF_CLK_SELR::PLL_REF_CLK_SEL_1
    }
}
#[doc = "Possible values of the field `PAD_CLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD_CLK_SELR {
    #[doc = "CLKIN1 XOR CLKIN2"]
    PAD_CLK_SEL_0,
    #[doc = "CLKIN2"]
    PAD_CLK_SEL_1,
    #[doc = "CLKIN1"]
    PAD_CLK_SEL_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD_CLK_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD_CLK_SELR::PAD_CLK_SEL_0 => 0,
            PAD_CLK_SELR::PAD_CLK_SEL_1 => 1,
            PAD_CLK_SELR::PAD_CLK_SEL_2 => 2,
            PAD_CLK_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD_CLK_SELR {
        match value {
            0 => PAD_CLK_SELR::PAD_CLK_SEL_0,
            1 => PAD_CLK_SELR::PAD_CLK_SEL_1,
            2 => PAD_CLK_SELR::PAD_CLK_SEL_2,
            i => PAD_CLK_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PAD_CLK_SEL_0`"]
    #[inline]
    pub fn is_pad_clk_sel_0(&self) -> bool {
        *self == PAD_CLK_SELR::PAD_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `PAD_CLK_SEL_1`"]
    #[inline]
    pub fn is_pad_clk_sel_1(&self) -> bool {
        *self == PAD_CLK_SELR::PAD_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `PAD_CLK_SEL_2`"]
    #[inline]
    pub fn is_pad_clk_sel_2(&self) -> bool {
        *self == PAD_CLK_SELR::PAD_CLK_SEL_2
    }
}
#[doc = r" Value of the field"]
pub struct PLL_BYPASSR {
    bits: bool,
}
impl PLL_BYPASSR {
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
pub struct PLL_RST_OVERRIDER {
    bits: bool,
}
impl PLL_RST_OVERRIDER {
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
pub struct PLL_RSTR {
    bits: bool,
}
impl PLL_RSTR {
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
pub struct PLL_CLKE_OVERRIDER {
    bits: bool,
}
impl PLL_CLKE_OVERRIDER {
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
pub struct PLL_CLKER {
    bits: bool,
}
impl PLL_CLKER {
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
pub struct PLL_DIV2_CLKE_OVERRIDER {
    bits: bool,
}
impl PLL_DIV2_CLKE_OVERRIDER {
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
pub struct PLL_DIV2_CLKER {
    bits: bool,
}
impl PLL_DIV2_CLKER {
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
pub struct PLL_DIV3_CLKE_OVERRIDER {
    bits: bool,
}
impl PLL_DIV3_CLKE_OVERRIDER {
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
pub struct PLL_DIV3_CLKER {
    bits: bool,
}
impl PLL_DIV3_CLKER {
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
pub struct PLL_DIV4_CLKE_OVERRIDER {
    bits: bool,
}
impl PLL_DIV4_CLKE_OVERRIDER {
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
pub struct PLL_DIV4_CLKER {
    bits: bool,
}
impl PLL_DIV4_CLKER {
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
pub struct PLL_DIV5_CLKE_OVERRIDER {
    bits: bool,
}
impl PLL_DIV5_CLKE_OVERRIDER {
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
pub struct PLL_DIV5_CLKER {
    bits: bool,
}
impl PLL_DIV5_CLKER {
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
pub struct PLL_DIV6_CLKE_OVERRIDER {
    bits: bool,
}
impl PLL_DIV6_CLKE_OVERRIDER {
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
pub struct PLL_DIV6_CLKER {
    bits: bool,
}
impl PLL_DIV6_CLKER {
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
pub struct PLL_DIV8_CLKE_OVERRIDER {
    bits: bool,
}
impl PLL_DIV8_CLKE_OVERRIDER {
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
pub struct PLL_DIV8_CLKER {
    bits: bool,
}
impl PLL_DIV8_CLKER {
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
pub struct PLL_DIV10_CLKE_OVERRIDER {
    bits: bool,
}
impl PLL_DIV10_CLKE_OVERRIDER {
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
pub struct PLL_DIV10_CLKER {
    bits: bool,
}
impl PLL_DIV10_CLKER {
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
pub struct PLL_DIV20_CLKE_OVERRIDER {
    bits: bool,
}
impl PLL_DIV20_CLKE_OVERRIDER {
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
pub struct PLL_DIV20_CLKER {
    bits: bool,
}
impl PLL_DIV20_CLKER {
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
pub struct PLL_EXT_BYPASSR {
    bits: bool,
}
impl PLL_EXT_BYPASSR {
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
#[doc = "Possible values of the field `PLL_LOCK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_LOCK_SELR {
    #[doc = "Using PLL maximum lock time"]
    PLL_LOCK_SEL_0,
    #[doc = "Using PLL output lock"]
    PLL_LOCK_SEL_1,
}
impl PLL_LOCK_SELR {
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
            PLL_LOCK_SELR::PLL_LOCK_SEL_0 => false,
            PLL_LOCK_SELR::PLL_LOCK_SEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLL_LOCK_SELR {
        match value {
            false => PLL_LOCK_SELR::PLL_LOCK_SEL_0,
            true => PLL_LOCK_SELR::PLL_LOCK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `PLL_LOCK_SEL_0`"]
    #[inline]
    pub fn is_pll_lock_sel_0(&self) -> bool {
        *self == PLL_LOCK_SELR::PLL_LOCK_SEL_0
    }
    #[doc = "Checks if the value of the field is `PLL_LOCK_SEL_1`"]
    #[inline]
    pub fn is_pll_lock_sel_1(&self) -> bool {
        *self == PLL_LOCK_SELR::PLL_LOCK_SEL_1
    }
}
#[doc = r" Value of the field"]
pub struct PLL_LOCKR {
    bits: bool,
}
impl PLL_LOCKR {
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
#[doc = "Values that can be written to the field `PLL_REF_CLK_SEL`"]
pub enum PLL_REF_CLK_SELW {
    #[doc = "SYS_XTAL"]
    PLL_REF_CLK_SEL_0,
    #[doc = "PAD_CLK"]
    PLL_REF_CLK_SEL_1,
}
impl PLL_REF_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PLL_REF_CLK_SELW::PLL_REF_CLK_SEL_0 => 0,
            PLL_REF_CLK_SELW::PLL_REF_CLK_SEL_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLL_REF_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_REF_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLL_REF_CLK_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SYS_XTAL"]
    #[inline]
    pub fn pll_ref_clk_sel_0(self) -> &'a mut W {
        self.variant(PLL_REF_CLK_SELW::PLL_REF_CLK_SEL_0)
    }
    #[doc = "PAD_CLK"]
    #[inline]
    pub fn pll_ref_clk_sel_1(self) -> &'a mut W {
        self.variant(PLL_REF_CLK_SELW::PLL_REF_CLK_SEL_1)
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
#[doc = "Values that can be written to the field `PAD_CLK_SEL`"]
pub enum PAD_CLK_SELW {
    #[doc = "CLKIN1 XOR CLKIN2"]
    PAD_CLK_SEL_0,
    #[doc = "CLKIN2"]
    PAD_CLK_SEL_1,
    #[doc = "CLKIN1"]
    PAD_CLK_SEL_2,
}
impl PAD_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAD_CLK_SELW::PAD_CLK_SEL_0 => 0,
            PAD_CLK_SELW::PAD_CLK_SEL_1 => 1,
            PAD_CLK_SELW::PAD_CLK_SEL_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAD_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD_CLK_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CLKIN1 XOR CLKIN2"]
    #[inline]
    pub fn pad_clk_sel_0(self) -> &'a mut W {
        self.variant(PAD_CLK_SELW::PAD_CLK_SEL_0)
    }
    #[doc = "CLKIN2"]
    #[inline]
    pub fn pad_clk_sel_1(self) -> &'a mut W {
        self.variant(PAD_CLK_SELW::PAD_CLK_SEL_1)
    }
    #[doc = "CLKIN1"]
    #[inline]
    pub fn pad_clk_sel_2(self) -> &'a mut W {
        self.variant(PAD_CLK_SELW::PAD_CLK_SEL_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLL_BYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_BYPASSW<'a> {
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
pub struct _PLL_RST_OVERRIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_RST_OVERRIDEW<'a> {
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
pub struct _PLL_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_RSTW<'a> {
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
pub struct _PLL_CLKE_OVERRIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_CLKE_OVERRIDEW<'a> {
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
pub struct _PLL_CLKEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_CLKEW<'a> {
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
pub struct _PLL_DIV2_CLKE_OVERRIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_DIV2_CLKE_OVERRIDEW<'a> {
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
pub struct _PLL_DIV2_CLKEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_DIV2_CLKEW<'a> {
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
pub struct _PLL_DIV3_CLKE_OVERRIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_DIV3_CLKE_OVERRIDEW<'a> {
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
pub struct _PLL_DIV3_CLKEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_DIV3_CLKEW<'a> {
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
pub struct _PLL_DIV4_CLKE_OVERRIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_DIV4_CLKE_OVERRIDEW<'a> {
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
pub struct _PLL_DIV4_CLKEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_DIV4_CLKEW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLL_DIV5_CLKE_OVERRIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_DIV5_CLKE_OVERRIDEW<'a> {
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
pub struct _PLL_DIV5_CLKEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_DIV5_CLKEW<'a> {
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
pub struct _PLL_DIV6_CLKE_OVERRIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_DIV6_CLKE_OVERRIDEW<'a> {
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
pub struct _PLL_DIV6_CLKEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_DIV6_CLKEW<'a> {
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
pub struct _PLL_DIV8_CLKE_OVERRIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_DIV8_CLKE_OVERRIDEW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLL_DIV8_CLKEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_DIV8_CLKEW<'a> {
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
pub struct _PLL_DIV10_CLKE_OVERRIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_DIV10_CLKE_OVERRIDEW<'a> {
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
pub struct _PLL_DIV10_CLKEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_DIV10_CLKEW<'a> {
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
pub struct _PLL_DIV20_CLKE_OVERRIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_DIV20_CLKE_OVERRIDEW<'a> {
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
pub struct _PLL_DIV20_CLKEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_DIV20_CLKEW<'a> {
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
pub struct _PLL_EXT_BYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_EXT_BYPASSW<'a> {
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
#[doc = "Values that can be written to the field `PLL_LOCK_SEL`"]
pub enum PLL_LOCK_SELW {
    #[doc = "Using PLL maximum lock time"]
    PLL_LOCK_SEL_0,
    #[doc = "Using PLL output lock"]
    PLL_LOCK_SEL_1,
}
impl PLL_LOCK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLL_LOCK_SELW::PLL_LOCK_SEL_0 => false,
            PLL_LOCK_SELW::PLL_LOCK_SEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLL_LOCK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_LOCK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLL_LOCK_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Using PLL maximum lock time"]
    #[inline]
    pub fn pll_lock_sel_0(self) -> &'a mut W {
        self.variant(PLL_LOCK_SELW::PLL_LOCK_SEL_0)
    }
    #[doc = "Using PLL output lock"]
    #[inline]
    pub fn pll_lock_sel_1(self) -> &'a mut W {
        self.variant(PLL_LOCK_SELW::PLL_LOCK_SEL_1)
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
        const OFFSET: u8 = 29;
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
    #[doc = "Bits 0:1 - PLL reference clock select"]
    #[inline]
    pub fn pll_ref_clk_sel(&self) -> PLL_REF_CLK_SELR {
        PLL_REF_CLK_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - PAD clock select, the output clock is PAD_CLK, PLL reference clock option"]
    #[inline]
    pub fn pad_clk_sel(&self) -> PAD_CLK_SELR {
        PAD_CLK_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - PLL output clock bypass"]
    #[inline]
    pub fn pll_bypass(&self) -> PLL_BYPASSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_BYPASSR { bits }
    }
    #[doc = "Bit 8 - PLL reset overrided by CCM"]
    #[inline]
    pub fn pll_rst_override(&self) -> PLL_RST_OVERRIDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_RST_OVERRIDER { bits }
    }
    #[doc = "Bit 9 - PLL reset (active low)"]
    #[inline]
    pub fn pll_rst(&self) -> PLL_RSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_RSTR { bits }
    }
    #[doc = "Bit 10 - Override the PLL_CLKE, clock gating enable signal from CCM"]
    #[inline]
    pub fn pll_clke_override(&self) -> PLL_CLKE_OVERRIDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_CLKE_OVERRIDER { bits }
    }
    #[doc = "Bit 11 - PLL output clock clock gating enable"]
    #[inline]
    pub fn pll_clke(&self) -> PLL_CLKER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_CLKER { bits }
    }
    #[doc = "Bit 12 - PLL clock divided by 2 output gating enable overrided by CCM"]
    #[inline]
    pub fn pll_div2_clke_override(&self) -> PLL_DIV2_CLKE_OVERRIDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_DIV2_CLKE_OVERRIDER { bits }
    }
    #[doc = "Bit 13 - PLL clock divided by 2 output gating enable"]
    #[inline]
    pub fn pll_div2_clke(&self) -> PLL_DIV2_CLKER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_DIV2_CLKER { bits }
    }
    #[doc = "Bit 14 - PLL clock divided by 3 output gating enable overrided by CCM"]
    #[inline]
    pub fn pll_div3_clke_override(&self) -> PLL_DIV3_CLKE_OVERRIDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_DIV3_CLKE_OVERRIDER { bits }
    }
    #[doc = "Bit 15 - PLL clock divided by 3 output gating enable"]
    #[inline]
    pub fn pll_div3_clke(&self) -> PLL_DIV3_CLKER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_DIV3_CLKER { bits }
    }
    #[doc = "Bit 16 - PLL clock divided by 4 output gating enable overrided by CCM"]
    #[inline]
    pub fn pll_div4_clke_override(&self) -> PLL_DIV4_CLKE_OVERRIDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_DIV4_CLKE_OVERRIDER { bits }
    }
    #[doc = "Bit 17 - PLL clock divided by 4 output gating enable"]
    #[inline]
    pub fn pll_div4_clke(&self) -> PLL_DIV4_CLKER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_DIV4_CLKER { bits }
    }
    #[doc = "Bit 18 - PLL clock divided by 5 output gating enable overrided by CCM"]
    #[inline]
    pub fn pll_div5_clke_override(&self) -> PLL_DIV5_CLKE_OVERRIDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_DIV5_CLKE_OVERRIDER { bits }
    }
    #[doc = "Bit 19 - PLL clock divided by 5 output gating enable"]
    #[inline]
    pub fn pll_div5_clke(&self) -> PLL_DIV5_CLKER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_DIV5_CLKER { bits }
    }
    #[doc = "Bit 20 - PLL clock divided by 6 output gating enable overrided by CCM"]
    #[inline]
    pub fn pll_div6_clke_override(&self) -> PLL_DIV6_CLKE_OVERRIDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_DIV6_CLKE_OVERRIDER { bits }
    }
    #[doc = "Bit 21 - PLL clock divided by 6 output gating enable"]
    #[inline]
    pub fn pll_div6_clke(&self) -> PLL_DIV6_CLKER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_DIV6_CLKER { bits }
    }
    #[doc = "Bit 22 - PLL clock divided by 8 output gating enable overrided by CCM"]
    #[inline]
    pub fn pll_div8_clke_override(&self) -> PLL_DIV8_CLKE_OVERRIDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_DIV8_CLKE_OVERRIDER { bits }
    }
    #[doc = "Bit 23 - PLL clock divided by 8 output gating enable"]
    #[inline]
    pub fn pll_div8_clke(&self) -> PLL_DIV8_CLKER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_DIV8_CLKER { bits }
    }
    #[doc = "Bit 24 - PLL clock divided by 10 output gating enable overrided by CCM"]
    #[inline]
    pub fn pll_div10_clke_override(&self) -> PLL_DIV10_CLKE_OVERRIDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_DIV10_CLKE_OVERRIDER { bits }
    }
    #[doc = "Bit 25 - PLL clock divided by 10 output gating enable"]
    #[inline]
    pub fn pll_div10_clke(&self) -> PLL_DIV10_CLKER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_DIV10_CLKER { bits }
    }
    #[doc = "Bit 26 - PLL clock divided by 20 output gating enable overrided by CCM"]
    #[inline]
    pub fn pll_div20_clke_override(&self) -> PLL_DIV20_CLKE_OVERRIDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_DIV20_CLKE_OVERRIDER { bits }
    }
    #[doc = "Bit 27 - PLL clock divided by 20 output gating enable"]
    #[inline]
    pub fn pll_div20_clke(&self) -> PLL_DIV20_CLKER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_DIV20_CLKER { bits }
    }
    #[doc = "Bit 28 - PLL analog block bypass, clock output traces to PLL source"]
    #[inline]
    pub fn pll_ext_bypass(&self) -> PLL_EXT_BYPASSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_EXT_BYPASSR { bits }
    }
    #[doc = "Bit 29 - PLL lock select"]
    #[inline]
    pub fn pll_lock_sel(&self) -> PLL_LOCK_SELR {
        PLL_LOCK_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - PLL lock signal"]
    #[inline]
    pub fn pll_lock(&self) -> PLL_LOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_LOCKR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 178956304 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - PLL reference clock select"]
    #[inline]
    pub fn pll_ref_clk_sel(&mut self) -> _PLL_REF_CLK_SELW {
        _PLL_REF_CLK_SELW { w: self }
    }
    #[doc = "Bits 2:3 - PAD clock select, the output clock is PAD_CLK, PLL reference clock option"]
    #[inline]
    pub fn pad_clk_sel(&mut self) -> _PAD_CLK_SELW {
        _PAD_CLK_SELW { w: self }
    }
    #[doc = "Bit 4 - PLL output clock bypass"]
    #[inline]
    pub fn pll_bypass(&mut self) -> _PLL_BYPASSW {
        _PLL_BYPASSW { w: self }
    }
    #[doc = "Bit 8 - PLL reset overrided by CCM"]
    #[inline]
    pub fn pll_rst_override(&mut self) -> _PLL_RST_OVERRIDEW {
        _PLL_RST_OVERRIDEW { w: self }
    }
    #[doc = "Bit 9 - PLL reset (active low)"]
    #[inline]
    pub fn pll_rst(&mut self) -> _PLL_RSTW {
        _PLL_RSTW { w: self }
    }
    #[doc = "Bit 10 - Override the PLL_CLKE, clock gating enable signal from CCM"]
    #[inline]
    pub fn pll_clke_override(&mut self) -> _PLL_CLKE_OVERRIDEW {
        _PLL_CLKE_OVERRIDEW { w: self }
    }
    #[doc = "Bit 11 - PLL output clock clock gating enable"]
    #[inline]
    pub fn pll_clke(&mut self) -> _PLL_CLKEW {
        _PLL_CLKEW { w: self }
    }
    #[doc = "Bit 12 - PLL clock divided by 2 output gating enable overrided by CCM"]
    #[inline]
    pub fn pll_div2_clke_override(&mut self) -> _PLL_DIV2_CLKE_OVERRIDEW {
        _PLL_DIV2_CLKE_OVERRIDEW { w: self }
    }
    #[doc = "Bit 13 - PLL clock divided by 2 output gating enable"]
    #[inline]
    pub fn pll_div2_clke(&mut self) -> _PLL_DIV2_CLKEW {
        _PLL_DIV2_CLKEW { w: self }
    }
    #[doc = "Bit 14 - PLL clock divided by 3 output gating enable overrided by CCM"]
    #[inline]
    pub fn pll_div3_clke_override(&mut self) -> _PLL_DIV3_CLKE_OVERRIDEW {
        _PLL_DIV3_CLKE_OVERRIDEW { w: self }
    }
    #[doc = "Bit 15 - PLL clock divided by 3 output gating enable"]
    #[inline]
    pub fn pll_div3_clke(&mut self) -> _PLL_DIV3_CLKEW {
        _PLL_DIV3_CLKEW { w: self }
    }
    #[doc = "Bit 16 - PLL clock divided by 4 output gating enable overrided by CCM"]
    #[inline]
    pub fn pll_div4_clke_override(&mut self) -> _PLL_DIV4_CLKE_OVERRIDEW {
        _PLL_DIV4_CLKE_OVERRIDEW { w: self }
    }
    #[doc = "Bit 17 - PLL clock divided by 4 output gating enable"]
    #[inline]
    pub fn pll_div4_clke(&mut self) -> _PLL_DIV4_CLKEW {
        _PLL_DIV4_CLKEW { w: self }
    }
    #[doc = "Bit 18 - PLL clock divided by 5 output gating enable overrided by CCM"]
    #[inline]
    pub fn pll_div5_clke_override(&mut self) -> _PLL_DIV5_CLKE_OVERRIDEW {
        _PLL_DIV5_CLKE_OVERRIDEW { w: self }
    }
    #[doc = "Bit 19 - PLL clock divided by 5 output gating enable"]
    #[inline]
    pub fn pll_div5_clke(&mut self) -> _PLL_DIV5_CLKEW {
        _PLL_DIV5_CLKEW { w: self }
    }
    #[doc = "Bit 20 - PLL clock divided by 6 output gating enable overrided by CCM"]
    #[inline]
    pub fn pll_div6_clke_override(&mut self) -> _PLL_DIV6_CLKE_OVERRIDEW {
        _PLL_DIV6_CLKE_OVERRIDEW { w: self }
    }
    #[doc = "Bit 21 - PLL clock divided by 6 output gating enable"]
    #[inline]
    pub fn pll_div6_clke(&mut self) -> _PLL_DIV6_CLKEW {
        _PLL_DIV6_CLKEW { w: self }
    }
    #[doc = "Bit 22 - PLL clock divided by 8 output gating enable overrided by CCM"]
    #[inline]
    pub fn pll_div8_clke_override(&mut self) -> _PLL_DIV8_CLKE_OVERRIDEW {
        _PLL_DIV8_CLKE_OVERRIDEW { w: self }
    }
    #[doc = "Bit 23 - PLL clock divided by 8 output gating enable"]
    #[inline]
    pub fn pll_div8_clke(&mut self) -> _PLL_DIV8_CLKEW {
        _PLL_DIV8_CLKEW { w: self }
    }
    #[doc = "Bit 24 - PLL clock divided by 10 output gating enable overrided by CCM"]
    #[inline]
    pub fn pll_div10_clke_override(&mut self) -> _PLL_DIV10_CLKE_OVERRIDEW {
        _PLL_DIV10_CLKE_OVERRIDEW { w: self }
    }
    #[doc = "Bit 25 - PLL clock divided by 10 output gating enable"]
    #[inline]
    pub fn pll_div10_clke(&mut self) -> _PLL_DIV10_CLKEW {
        _PLL_DIV10_CLKEW { w: self }
    }
    #[doc = "Bit 26 - PLL clock divided by 20 output gating enable overrided by CCM"]
    #[inline]
    pub fn pll_div20_clke_override(&mut self) -> _PLL_DIV20_CLKE_OVERRIDEW {
        _PLL_DIV20_CLKE_OVERRIDEW { w: self }
    }
    #[doc = "Bit 27 - PLL clock divided by 20 output gating enable"]
    #[inline]
    pub fn pll_div20_clke(&mut self) -> _PLL_DIV20_CLKEW {
        _PLL_DIV20_CLKEW { w: self }
    }
    #[doc = "Bit 28 - PLL analog block bypass, clock output traces to PLL source"]
    #[inline]
    pub fn pll_ext_bypass(&mut self) -> _PLL_EXT_BYPASSW {
        _PLL_EXT_BYPASSW { w: self }
    }
    #[doc = "Bit 29 - PLL lock select"]
    #[inline]
    pub fn pll_lock_sel(&mut self) -> _PLL_LOCK_SELW {
        _PLL_LOCK_SELW { w: self }
    }
}
