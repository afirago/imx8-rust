#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::A53CORE1_SR {
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
#[doc = "Possible values of the field `PSR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSRR {
    #[doc = "The target subsystem was not powered down for the previous power-down request."]
    PSR_0,
    #[doc = "The target subsystem was powered down for the previous power-down request."]
    PSR_1,
}
impl PSRR {
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
            PSRR::PSR_0 => false,
            PSRR::PSR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PSRR {
        match value {
            false => PSRR::PSR_0,
            true => PSRR::PSR_1,
        }
    }
    #[doc = "Checks if the value of the field is `PSR_0`"]
    #[inline]
    pub fn is_psr_0(&self) -> bool {
        *self == PSRR::PSR_0
    }
    #[doc = "Checks if the value of the field is `PSR_1`"]
    #[inline]
    pub fn is_psr_1(&self) -> bool {
        *self == PSRR::PSR_1
    }
}
#[doc = "Possible values of the field `L2RETN_FLAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L2RETN_FLAGR {
    #[doc = "A53 is not wakeup from L2 retention mode."]
    L2RETN_FLAG_0,
    #[doc = "A53 is wakeup from L2 retention mode."]
    L2RETN_FLAG_1,
}
impl L2RETN_FLAGR {
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
            L2RETN_FLAGR::L2RETN_FLAG_0 => false,
            L2RETN_FLAGR::L2RETN_FLAG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> L2RETN_FLAGR {
        match value {
            false => L2RETN_FLAGR::L2RETN_FLAG_0,
            true => L2RETN_FLAGR::L2RETN_FLAG_1,
        }
    }
    #[doc = "Checks if the value of the field is `L2RETN_FLAG_0`"]
    #[inline]
    pub fn is_l2retn_flag_0(&self) -> bool {
        *self == L2RETN_FLAGR::L2RETN_FLAG_0
    }
    #[doc = "Checks if the value of the field is `L2RETN_FLAG_1`"]
    #[inline]
    pub fn is_l2retn_flag_1(&self) -> bool {
        *self == L2RETN_FLAGR::L2RETN_FLAG_1
    }
}
#[doc = "Possible values of the field `ALLOFF_FLAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALLOFF_FLAGR {
    #[doc = "A53 is not wakeup from ALL_OFF mode."]
    ALLOFF_FLAG_0,
    #[doc = "A53 is wakeup from ALL_OFF mode."]
    ALLOFF_FLAG_1,
}
impl ALLOFF_FLAGR {
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
            ALLOFF_FLAGR::ALLOFF_FLAG_0 => false,
            ALLOFF_FLAGR::ALLOFF_FLAG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALLOFF_FLAGR {
        match value {
            false => ALLOFF_FLAGR::ALLOFF_FLAG_0,
            true => ALLOFF_FLAGR::ALLOFF_FLAG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOFF_FLAG_0`"]
    #[inline]
    pub fn is_alloff_flag_0(&self) -> bool {
        *self == ALLOFF_FLAGR::ALLOFF_FLAG_0
    }
    #[doc = "Checks if the value of the field is `ALLOFF_FLAG_1`"]
    #[inline]
    pub fn is_alloff_flag_1(&self) -> bool {
        *self == ALLOFF_FLAGR::ALLOFF_FLAG_1
    }
}
#[doc = "Possible values of the field `PUP_CLK_DIV_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUP_CLK_DIV_SELR {
    #[doc = "1"]
    PUP_CLK_DIV_SEL_0,
    #[doc = "1/2 count_clk"]
    PUP_CLK_DIV_SEL_1,
    #[doc = "1/4 count_clk"]
    PUP_CLK_DIV_SEL_2,
    #[doc = "1/8 count_clk"]
    PUP_CLK_DIV_SEL_3,
    #[doc = "1/16 count_clk"]
    PUP_CLK_DIV_SEL_4,
    #[doc = "1/32 count_clk"]
    PUP_CLK_DIV_SEL_5,
    #[doc = "1/64 count_clk"]
    PUP_CLK_DIV_SEL_6,
    #[doc = "1/128 count_clk"]
    PUP_CLK_DIV_SEL_7,
    #[doc = "1/256 count_clk"]
    PUP_CLK_DIV_SEL_8,
    #[doc = "1/512 count_clk"]
    PUP_CLK_DIV_SEL_9,
    #[doc = "1/1024 count_clk"]
    PUP_CLK_DIV_SEL_10,
    #[doc = "1/2056 count_clk"]
    PUP_CLK_DIV_SEL_11,
    #[doc = "1/4096 count_clk"]
    PUP_CLK_DIV_SEL_12,
    #[doc = "1/8192 count_clk"]
    PUP_CLK_DIV_SEL_13,
    #[doc = "1/16384 count_clk"]
    PUP_CLK_DIV_SEL_14,
    #[doc = "1/32768 count_clk"]
    PUP_CLK_DIV_SEL_15,
}
impl PUP_CLK_DIV_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_0 => 0,
            PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_1 => 1,
            PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_2 => 2,
            PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_3 => 3,
            PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_4 => 4,
            PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_5 => 5,
            PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_6 => 6,
            PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_7 => 7,
            PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_8 => 8,
            PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_9 => 9,
            PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_10 => 10,
            PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_11 => 11,
            PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_12 => 12,
            PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_13 => 13,
            PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_14 => 14,
            PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_15 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PUP_CLK_DIV_SELR {
        match value {
            0 => PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_0,
            1 => PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_1,
            2 => PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_2,
            3 => PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_3,
            4 => PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_4,
            5 => PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_5,
            6 => PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_6,
            7 => PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_7,
            8 => PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_8,
            9 => PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_9,
            10 => PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_10,
            11 => PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_11,
            12 => PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_12,
            13 => PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_13,
            14 => PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_14,
            15 => PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PUP_CLK_DIV_SEL_0`"]
    #[inline]
    pub fn is_pup_clk_div_sel_0(&self) -> bool {
        *self == PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_0
    }
    #[doc = "Checks if the value of the field is `PUP_CLK_DIV_SEL_1`"]
    #[inline]
    pub fn is_pup_clk_div_sel_1(&self) -> bool {
        *self == PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_1
    }
    #[doc = "Checks if the value of the field is `PUP_CLK_DIV_SEL_2`"]
    #[inline]
    pub fn is_pup_clk_div_sel_2(&self) -> bool {
        *self == PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_2
    }
    #[doc = "Checks if the value of the field is `PUP_CLK_DIV_SEL_3`"]
    #[inline]
    pub fn is_pup_clk_div_sel_3(&self) -> bool {
        *self == PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_3
    }
    #[doc = "Checks if the value of the field is `PUP_CLK_DIV_SEL_4`"]
    #[inline]
    pub fn is_pup_clk_div_sel_4(&self) -> bool {
        *self == PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_4
    }
    #[doc = "Checks if the value of the field is `PUP_CLK_DIV_SEL_5`"]
    #[inline]
    pub fn is_pup_clk_div_sel_5(&self) -> bool {
        *self == PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_5
    }
    #[doc = "Checks if the value of the field is `PUP_CLK_DIV_SEL_6`"]
    #[inline]
    pub fn is_pup_clk_div_sel_6(&self) -> bool {
        *self == PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_6
    }
    #[doc = "Checks if the value of the field is `PUP_CLK_DIV_SEL_7`"]
    #[inline]
    pub fn is_pup_clk_div_sel_7(&self) -> bool {
        *self == PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_7
    }
    #[doc = "Checks if the value of the field is `PUP_CLK_DIV_SEL_8`"]
    #[inline]
    pub fn is_pup_clk_div_sel_8(&self) -> bool {
        *self == PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_8
    }
    #[doc = "Checks if the value of the field is `PUP_CLK_DIV_SEL_9`"]
    #[inline]
    pub fn is_pup_clk_div_sel_9(&self) -> bool {
        *self == PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_9
    }
    #[doc = "Checks if the value of the field is `PUP_CLK_DIV_SEL_10`"]
    #[inline]
    pub fn is_pup_clk_div_sel_10(&self) -> bool {
        *self == PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_10
    }
    #[doc = "Checks if the value of the field is `PUP_CLK_DIV_SEL_11`"]
    #[inline]
    pub fn is_pup_clk_div_sel_11(&self) -> bool {
        *self == PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_11
    }
    #[doc = "Checks if the value of the field is `PUP_CLK_DIV_SEL_12`"]
    #[inline]
    pub fn is_pup_clk_div_sel_12(&self) -> bool {
        *self == PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_12
    }
    #[doc = "Checks if the value of the field is `PUP_CLK_DIV_SEL_13`"]
    #[inline]
    pub fn is_pup_clk_div_sel_13(&self) -> bool {
        *self == PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_13
    }
    #[doc = "Checks if the value of the field is `PUP_CLK_DIV_SEL_14`"]
    #[inline]
    pub fn is_pup_clk_div_sel_14(&self) -> bool {
        *self == PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_14
    }
    #[doc = "Checks if the value of the field is `PUP_CLK_DIV_SEL_15`"]
    #[inline]
    pub fn is_pup_clk_div_sel_15(&self) -> bool {
        *self == PUP_CLK_DIV_SELR::PUP_CLK_DIV_SEL_15
    }
}
#[doc = r" Value of the field"]
pub struct L2RSTDIS_DEASSERT_CNTR {
    bits: u16,
}
impl L2RSTDIS_DEASSERT_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `PUP_CLK_DIV_SEL`"]
pub enum PUP_CLK_DIV_SELW {
    #[doc = "1"]
    PUP_CLK_DIV_SEL_0,
    #[doc = "1/2 count_clk"]
    PUP_CLK_DIV_SEL_1,
    #[doc = "1/4 count_clk"]
    PUP_CLK_DIV_SEL_2,
    #[doc = "1/8 count_clk"]
    PUP_CLK_DIV_SEL_3,
    #[doc = "1/16 count_clk"]
    PUP_CLK_DIV_SEL_4,
    #[doc = "1/32 count_clk"]
    PUP_CLK_DIV_SEL_5,
    #[doc = "1/64 count_clk"]
    PUP_CLK_DIV_SEL_6,
    #[doc = "1/128 count_clk"]
    PUP_CLK_DIV_SEL_7,
    #[doc = "1/256 count_clk"]
    PUP_CLK_DIV_SEL_8,
    #[doc = "1/512 count_clk"]
    PUP_CLK_DIV_SEL_9,
    #[doc = "1/1024 count_clk"]
    PUP_CLK_DIV_SEL_10,
    #[doc = "1/2056 count_clk"]
    PUP_CLK_DIV_SEL_11,
    #[doc = "1/4096 count_clk"]
    PUP_CLK_DIV_SEL_12,
    #[doc = "1/8192 count_clk"]
    PUP_CLK_DIV_SEL_13,
    #[doc = "1/16384 count_clk"]
    PUP_CLK_DIV_SEL_14,
    #[doc = "1/32768 count_clk"]
    PUP_CLK_DIV_SEL_15,
}
impl PUP_CLK_DIV_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_0 => 0,
            PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_1 => 1,
            PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_2 => 2,
            PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_3 => 3,
            PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_4 => 4,
            PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_5 => 5,
            PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_6 => 6,
            PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_7 => 7,
            PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_8 => 8,
            PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_9 => 9,
            PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_10 => 10,
            PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_11 => 11,
            PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_12 => 12,
            PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_13 => 13,
            PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_14 => 14,
            PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PUP_CLK_DIV_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PUP_CLK_DIV_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUP_CLK_DIV_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1"]
    #[inline]
    pub fn pup_clk_div_sel_0(self) -> &'a mut W {
        self.variant(PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_0)
    }
    #[doc = "1/2 count_clk"]
    #[inline]
    pub fn pup_clk_div_sel_1(self) -> &'a mut W {
        self.variant(PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_1)
    }
    #[doc = "1/4 count_clk"]
    #[inline]
    pub fn pup_clk_div_sel_2(self) -> &'a mut W {
        self.variant(PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_2)
    }
    #[doc = "1/8 count_clk"]
    #[inline]
    pub fn pup_clk_div_sel_3(self) -> &'a mut W {
        self.variant(PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_3)
    }
    #[doc = "1/16 count_clk"]
    #[inline]
    pub fn pup_clk_div_sel_4(self) -> &'a mut W {
        self.variant(PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_4)
    }
    #[doc = "1/32 count_clk"]
    #[inline]
    pub fn pup_clk_div_sel_5(self) -> &'a mut W {
        self.variant(PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_5)
    }
    #[doc = "1/64 count_clk"]
    #[inline]
    pub fn pup_clk_div_sel_6(self) -> &'a mut W {
        self.variant(PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_6)
    }
    #[doc = "1/128 count_clk"]
    #[inline]
    pub fn pup_clk_div_sel_7(self) -> &'a mut W {
        self.variant(PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_7)
    }
    #[doc = "1/256 count_clk"]
    #[inline]
    pub fn pup_clk_div_sel_8(self) -> &'a mut W {
        self.variant(PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_8)
    }
    #[doc = "1/512 count_clk"]
    #[inline]
    pub fn pup_clk_div_sel_9(self) -> &'a mut W {
        self.variant(PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_9)
    }
    #[doc = "1/1024 count_clk"]
    #[inline]
    pub fn pup_clk_div_sel_10(self) -> &'a mut W {
        self.variant(PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_10)
    }
    #[doc = "1/2056 count_clk"]
    #[inline]
    pub fn pup_clk_div_sel_11(self) -> &'a mut W {
        self.variant(PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_11)
    }
    #[doc = "1/4096 count_clk"]
    #[inline]
    pub fn pup_clk_div_sel_12(self) -> &'a mut W {
        self.variant(PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_12)
    }
    #[doc = "1/8192 count_clk"]
    #[inline]
    pub fn pup_clk_div_sel_13(self) -> &'a mut W {
        self.variant(PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_13)
    }
    #[doc = "1/16384 count_clk"]
    #[inline]
    pub fn pup_clk_div_sel_14(self) -> &'a mut W {
        self.variant(PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_14)
    }
    #[doc = "1/32768 count_clk"]
    #[inline]
    pub fn pup_clk_div_sel_15(self) -> &'a mut W {
        self.variant(PUP_CLK_DIV_SELW::PUP_CLK_DIV_SEL_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _L2RSTDIS_DEASSERT_CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _L2RSTDIS_DEASSERT_CNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - Power status"]
    #[inline]
    pub fn psr(&self) -> PSRR {
        PSRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - L2 Retention Flag Software should write \"1\" to clear this flag after A53 is wakeup from L2 retention mode, otherwise it will always keep to 1 (This register control only for SCU Type PGC)"]
    #[inline]
    pub fn l2retn_flag(&self) -> L2RETN_FLAGR {
        L2RETN_FLAGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - All-off flag"]
    #[inline]
    pub fn alloff_flag(&self) -> ALLOFF_FLAGR {
        ALLOFF_FLAGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:6 - Clock divider select for the clock of power up counter(count_clk is 32KHz for CPU/SCU type PGC, ipg_clk(66MHz) for MIX/PU Type PGC)"]
    #[inline]
    pub fn pup_clk_div_sel(&self) -> PUP_CLK_DIV_SELR {
        PUP_CLK_DIV_SELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:17 - Count this value to de-assert L2RSTDISABLE to LOW after CPU0 or CPU1 power up This value can't be programmed to zero (This register control only for SCU Type PGC)"]
    #[inline]
    pub fn l2rstdis_deassert_cnt(&self) -> L2RSTDIS_DEASSERT_CNTR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        L2RSTDIS_DEASSERT_CNTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4096 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 3:6 - Clock divider select for the clock of power up counter(count_clk is 32KHz for CPU/SCU type PGC, ipg_clk(66MHz) for MIX/PU Type PGC)"]
    #[inline]
    pub fn pup_clk_div_sel(&mut self) -> _PUP_CLK_DIV_SELW {
        _PUP_CLK_DIV_SELW { w: self }
    }
    #[doc = "Bits 8:17 - Count this value to de-assert L2RSTDISABLE to LOW after CPU0 or CPU1 power up This value can't be programmed to zero (This register control only for SCU Type PGC)"]
    #[inline]
    pub fn l2rstdis_deassert_cnt(&mut self) -> _L2RSTDIS_DEASSERT_CNTW {
        _L2RSTDIS_DEASSERT_CNTW { w: self }
    }
}
