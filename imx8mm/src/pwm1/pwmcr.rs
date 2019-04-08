#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWMCR {
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
#[doc = "Possible values of the field `EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENR {
    #[doc = "PWM disabled"]
    EN_0,
    #[doc = "PWM enabled"]
    EN_1,
}
impl ENR {
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
            ENR::EN_0 => false,
            ENR::EN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENR {
        match value {
            false => ENR::EN_0,
            true => ENR::EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_0`"]
    #[inline]
    pub fn is_en_0(&self) -> bool {
        *self == ENR::EN_0
    }
    #[doc = "Checks if the value of the field is `EN_1`"]
    #[inline]
    pub fn is_en_1(&self) -> bool {
        *self == ENR::EN_1
    }
}
#[doc = "Possible values of the field `REPEAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REPEATR {
    #[doc = "Use each sample once"]
    REPEAT_0,
    #[doc = "Use each sample twice"]
    REPEAT_1,
    #[doc = "Use each sample four times"]
    REPEAT_2,
    #[doc = "Use each sample eight times"]
    REPEAT_3,
}
impl REPEATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REPEATR::REPEAT_0 => 0,
            REPEATR::REPEAT_1 => 1,
            REPEATR::REPEAT_2 => 2,
            REPEATR::REPEAT_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REPEATR {
        match value {
            0 => REPEATR::REPEAT_0,
            1 => REPEATR::REPEAT_1,
            2 => REPEATR::REPEAT_2,
            3 => REPEATR::REPEAT_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REPEAT_0`"]
    #[inline]
    pub fn is_repeat_0(&self) -> bool {
        *self == REPEATR::REPEAT_0
    }
    #[doc = "Checks if the value of the field is `REPEAT_1`"]
    #[inline]
    pub fn is_repeat_1(&self) -> bool {
        *self == REPEATR::REPEAT_1
    }
    #[doc = "Checks if the value of the field is `REPEAT_2`"]
    #[inline]
    pub fn is_repeat_2(&self) -> bool {
        *self == REPEATR::REPEAT_2
    }
    #[doc = "Checks if the value of the field is `REPEAT_3`"]
    #[inline]
    pub fn is_repeat_3(&self) -> bool {
        *self == REPEATR::REPEAT_3
    }
}
#[doc = "Possible values of the field `SWR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRR {
    #[doc = "PWM is out of reset"]
    SWR_0,
    #[doc = "PWM is undergoing reset"]
    SWR_1,
}
impl SWRR {
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
            SWRR::SWR_0 => false,
            SWRR::SWR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWRR {
        match value {
            false => SWRR::SWR_0,
            true => SWRR::SWR_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWR_0`"]
    #[inline]
    pub fn is_swr_0(&self) -> bool {
        *self == SWRR::SWR_0
    }
    #[doc = "Checks if the value of the field is `SWR_1`"]
    #[inline]
    pub fn is_swr_1(&self) -> bool {
        *self == SWRR::SWR_1
    }
}
#[doc = "Possible values of the field `PRESCALER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALERR {
    #[doc = "Divide by 1"]
    PRESCALER_0,
    #[doc = "Divide by 2"]
    PRESCALER_1,
    #[doc = "Divide by 4096"]
    PRESCALER_4095,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl PRESCALERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            PRESCALERR::PRESCALER_0 => 0,
            PRESCALERR::PRESCALER_1 => 1,
            PRESCALERR::PRESCALER_4095 => 4095,
            PRESCALERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> PRESCALERR {
        match value {
            0 => PRESCALERR::PRESCALER_0,
            1 => PRESCALERR::PRESCALER_1,
            4095 => PRESCALERR::PRESCALER_4095,
            i => PRESCALERR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRESCALER_0`"]
    #[inline]
    pub fn is_prescaler_0(&self) -> bool {
        *self == PRESCALERR::PRESCALER_0
    }
    #[doc = "Checks if the value of the field is `PRESCALER_1`"]
    #[inline]
    pub fn is_prescaler_1(&self) -> bool {
        *self == PRESCALERR::PRESCALER_1
    }
    #[doc = "Checks if the value of the field is `PRESCALER_4095`"]
    #[inline]
    pub fn is_prescaler_4095(&self) -> bool {
        *self == PRESCALERR::PRESCALER_4095
    }
}
#[doc = "Possible values of the field `CLKSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSRCR {
    #[doc = "Clock is off"]
    CLKSRC_0,
    #[doc = "ipg_clk"]
    CLKSRC_1,
    #[doc = "ipg_clk_highfreq"]
    CLKSRC_2,
    #[doc = "ipg_clk_32k"]
    CLKSRC_3,
}
impl CLKSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKSRCR::CLKSRC_0 => 0,
            CLKSRCR::CLKSRC_1 => 1,
            CLKSRCR::CLKSRC_2 => 2,
            CLKSRCR::CLKSRC_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKSRCR {
        match value {
            0 => CLKSRCR::CLKSRC_0,
            1 => CLKSRCR::CLKSRC_1,
            2 => CLKSRCR::CLKSRC_2,
            3 => CLKSRCR::CLKSRC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLKSRC_0`"]
    #[inline]
    pub fn is_clksrc_0(&self) -> bool {
        *self == CLKSRCR::CLKSRC_0
    }
    #[doc = "Checks if the value of the field is `CLKSRC_1`"]
    #[inline]
    pub fn is_clksrc_1(&self) -> bool {
        *self == CLKSRCR::CLKSRC_1
    }
    #[doc = "Checks if the value of the field is `CLKSRC_2`"]
    #[inline]
    pub fn is_clksrc_2(&self) -> bool {
        *self == CLKSRCR::CLKSRC_2
    }
    #[doc = "Checks if the value of the field is `CLKSRC_3`"]
    #[inline]
    pub fn is_clksrc_3(&self) -> bool {
        *self == CLKSRCR::CLKSRC_3
    }
}
#[doc = "Possible values of the field `POUTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POUTCR {
    #[doc = "Output pin is set at rollover and cleared at comparison"]
    POUTC_0,
    #[doc = "Output pin is cleared at rollover and set at comparison"]
    POUTC_1,
    #[doc = "PWM output is disconnected"]
    POUTC_2,
    #[doc = "PWM output is disconnected"]
    POUTC_3,
}
impl POUTCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            POUTCR::POUTC_0 => 0,
            POUTCR::POUTC_1 => 1,
            POUTCR::POUTC_2 => 2,
            POUTCR::POUTC_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> POUTCR {
        match value {
            0 => POUTCR::POUTC_0,
            1 => POUTCR::POUTC_1,
            2 => POUTCR::POUTC_2,
            3 => POUTCR::POUTC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `POUTC_0`"]
    #[inline]
    pub fn is_poutc_0(&self) -> bool {
        *self == POUTCR::POUTC_0
    }
    #[doc = "Checks if the value of the field is `POUTC_1`"]
    #[inline]
    pub fn is_poutc_1(&self) -> bool {
        *self == POUTCR::POUTC_1
    }
    #[doc = "Checks if the value of the field is `POUTC_2`"]
    #[inline]
    pub fn is_poutc_2(&self) -> bool {
        *self == POUTCR::POUTC_2
    }
    #[doc = "Checks if the value of the field is `POUTC_3`"]
    #[inline]
    pub fn is_poutc_3(&self) -> bool {
        *self == POUTCR::POUTC_3
    }
}
#[doc = "Possible values of the field `HCTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HCTRR {
    #[doc = "Half word swapping does not take place"]
    HCTR_0,
    #[doc = "Half words from write data bus are swapped"]
    HCTR_1,
}
impl HCTRR {
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
            HCTRR::HCTR_0 => false,
            HCTRR::HCTR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HCTRR {
        match value {
            false => HCTRR::HCTR_0,
            true => HCTRR::HCTR_1,
        }
    }
    #[doc = "Checks if the value of the field is `HCTR_0`"]
    #[inline]
    pub fn is_hctr_0(&self) -> bool {
        *self == HCTRR::HCTR_0
    }
    #[doc = "Checks if the value of the field is `HCTR_1`"]
    #[inline]
    pub fn is_hctr_1(&self) -> bool {
        *self == HCTRR::HCTR_1
    }
}
#[doc = "Possible values of the field `BCTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCTRR {
    #[doc = "byte ordering remains the same"]
    BCTR_0,
    #[doc = "byte ordering is reversed"]
    BCTR_1,
}
impl BCTRR {
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
            BCTRR::BCTR_0 => false,
            BCTRR::BCTR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BCTRR {
        match value {
            false => BCTRR::BCTR_0,
            true => BCTRR::BCTR_1,
        }
    }
    #[doc = "Checks if the value of the field is `BCTR_0`"]
    #[inline]
    pub fn is_bctr_0(&self) -> bool {
        *self == BCTRR::BCTR_0
    }
    #[doc = "Checks if the value of the field is `BCTR_1`"]
    #[inline]
    pub fn is_bctr_1(&self) -> bool {
        *self == BCTRR::BCTR_1
    }
}
#[doc = "Possible values of the field `DBGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGENR {
    #[doc = "Inactive in debug mode"]
    DBGEN_0,
    #[doc = "Active in debug mode"]
    DBGEN_1,
}
impl DBGENR {
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
            DBGENR::DBGEN_0 => false,
            DBGENR::DBGEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBGENR {
        match value {
            false => DBGENR::DBGEN_0,
            true => DBGENR::DBGEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBGEN_0`"]
    #[inline]
    pub fn is_dbgen_0(&self) -> bool {
        *self == DBGENR::DBGEN_0
    }
    #[doc = "Checks if the value of the field is `DBGEN_1`"]
    #[inline]
    pub fn is_dbgen_1(&self) -> bool {
        *self == DBGENR::DBGEN_1
    }
}
#[doc = "Possible values of the field `WAITEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITENR {
    #[doc = "Inactive in wait mode"]
    WAITEN_0,
    #[doc = "Active in wait mode"]
    WAITEN_1,
}
impl WAITENR {
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
            WAITENR::WAITEN_0 => false,
            WAITENR::WAITEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAITENR {
        match value {
            false => WAITENR::WAITEN_0,
            true => WAITENR::WAITEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `WAITEN_0`"]
    #[inline]
    pub fn is_waiten_0(&self) -> bool {
        *self == WAITENR::WAITEN_0
    }
    #[doc = "Checks if the value of the field is `WAITEN_1`"]
    #[inline]
    pub fn is_waiten_1(&self) -> bool {
        *self == WAITENR::WAITEN_1
    }
}
#[doc = "Possible values of the field `DOZEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZENR {
    #[doc = "Inactive in doze mode"]
    DOZEN_0,
    #[doc = "Active in doze mode"]
    DOZEN_1,
}
impl DOZENR {
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
            DOZENR::DOZEN_0 => false,
            DOZENR::DOZEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOZENR {
        match value {
            false => DOZENR::DOZEN_0,
            true => DOZENR::DOZEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOZEN_0`"]
    #[inline]
    pub fn is_dozen_0(&self) -> bool {
        *self == DOZENR::DOZEN_0
    }
    #[doc = "Checks if the value of the field is `DOZEN_1`"]
    #[inline]
    pub fn is_dozen_1(&self) -> bool {
        *self == DOZENR::DOZEN_1
    }
}
#[doc = "Possible values of the field `STOPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPENR {
    #[doc = "Inactive in stop mode"]
    STOPEN_0,
    #[doc = "Active in stop mode"]
    STOPEN_1,
}
impl STOPENR {
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
            STOPENR::STOPEN_0 => false,
            STOPENR::STOPEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOPENR {
        match value {
            false => STOPENR::STOPEN_0,
            true => STOPENR::STOPEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `STOPEN_0`"]
    #[inline]
    pub fn is_stopen_0(&self) -> bool {
        *self == STOPENR::STOPEN_0
    }
    #[doc = "Checks if the value of the field is `STOPEN_1`"]
    #[inline]
    pub fn is_stopen_1(&self) -> bool {
        *self == STOPENR::STOPEN_1
    }
}
#[doc = "Possible values of the field `FWM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWMR {
    #[doc = "FIFO empty flag is set when there are more than or equal to 1 empty slots in FIFO"]
    FWM_0,
    #[doc = "FIFO empty flag is set when there are more than or equal to 2 empty slots in FIFO"]
    FWM_1,
    #[doc = "FIFO empty flag is set when there are more than or equal to 3 empty slots in FIFO"]
    FWM_2,
    #[doc = "FIFO empty flag is set when there are more than or equal to 4 empty slots in FIFO"]
    FWM_3,
}
impl FWMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FWMR::FWM_0 => 0,
            FWMR::FWM_1 => 1,
            FWMR::FWM_2 => 2,
            FWMR::FWM_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FWMR {
        match value {
            0 => FWMR::FWM_0,
            1 => FWMR::FWM_1,
            2 => FWMR::FWM_2,
            3 => FWMR::FWM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FWM_0`"]
    #[inline]
    pub fn is_fwm_0(&self) -> bool {
        *self == FWMR::FWM_0
    }
    #[doc = "Checks if the value of the field is `FWM_1`"]
    #[inline]
    pub fn is_fwm_1(&self) -> bool {
        *self == FWMR::FWM_1
    }
    #[doc = "Checks if the value of the field is `FWM_2`"]
    #[inline]
    pub fn is_fwm_2(&self) -> bool {
        *self == FWMR::FWM_2
    }
    #[doc = "Checks if the value of the field is `FWM_3`"]
    #[inline]
    pub fn is_fwm_3(&self) -> bool {
        *self == FWMR::FWM_3
    }
}
#[doc = "Values that can be written to the field `EN`"]
pub enum ENW {
    #[doc = "PWM disabled"]
    EN_0,
    #[doc = "PWM enabled"]
    EN_1,
}
impl ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENW::EN_0 => false,
            ENW::EN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PWM disabled"]
    #[inline]
    pub fn en_0(self) -> &'a mut W {
        self.variant(ENW::EN_0)
    }
    #[doc = "PWM enabled"]
    #[inline]
    pub fn en_1(self) -> &'a mut W {
        self.variant(ENW::EN_1)
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
#[doc = "Values that can be written to the field `REPEAT`"]
pub enum REPEATW {
    #[doc = "Use each sample once"]
    REPEAT_0,
    #[doc = "Use each sample twice"]
    REPEAT_1,
    #[doc = "Use each sample four times"]
    REPEAT_2,
    #[doc = "Use each sample eight times"]
    REPEAT_3,
}
impl REPEATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REPEATW::REPEAT_0 => 0,
            REPEATW::REPEAT_1 => 1,
            REPEATW::REPEAT_2 => 2,
            REPEATW::REPEAT_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REPEATW<'a> {
    w: &'a mut W,
}
impl<'a> _REPEATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REPEATW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Use each sample once"]
    #[inline]
    pub fn repeat_0(self) -> &'a mut W {
        self.variant(REPEATW::REPEAT_0)
    }
    #[doc = "Use each sample twice"]
    #[inline]
    pub fn repeat_1(self) -> &'a mut W {
        self.variant(REPEATW::REPEAT_1)
    }
    #[doc = "Use each sample four times"]
    #[inline]
    pub fn repeat_2(self) -> &'a mut W {
        self.variant(REPEATW::REPEAT_2)
    }
    #[doc = "Use each sample eight times"]
    #[inline]
    pub fn repeat_3(self) -> &'a mut W {
        self.variant(REPEATW::REPEAT_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SWR`"]
pub enum SWRW {
    #[doc = "PWM is out of reset"]
    SWR_0,
    #[doc = "PWM is undergoing reset"]
    SWR_1,
}
impl SWRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWRW::SWR_0 => false,
            SWRW::SWR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWRW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PWM is out of reset"]
    #[inline]
    pub fn swr_0(self) -> &'a mut W {
        self.variant(SWRW::SWR_0)
    }
    #[doc = "PWM is undergoing reset"]
    #[inline]
    pub fn swr_1(self) -> &'a mut W {
        self.variant(SWRW::SWR_1)
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
#[doc = "Values that can be written to the field `PRESCALER`"]
pub enum PRESCALERW {
    #[doc = "Divide by 1"]
    PRESCALER_0,
    #[doc = "Divide by 2"]
    PRESCALER_1,
    #[doc = "Divide by 4096"]
    PRESCALER_4095,
}
impl PRESCALERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            PRESCALERW::PRESCALER_0 => 0,
            PRESCALERW::PRESCALER_1 => 1,
            PRESCALERW::PRESCALER_4095 => 4095,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCALERW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCALERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCALERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn prescaler_0(self) -> &'a mut W {
        self.variant(PRESCALERW::PRESCALER_0)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn prescaler_1(self) -> &'a mut W {
        self.variant(PRESCALERW::PRESCALER_1)
    }
    #[doc = "Divide by 4096"]
    #[inline]
    pub fn prescaler_4095(self) -> &'a mut W {
        self.variant(PRESCALERW::PRESCALER_4095)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKSRC`"]
pub enum CLKSRCW {
    #[doc = "Clock is off"]
    CLKSRC_0,
    #[doc = "ipg_clk"]
    CLKSRC_1,
    #[doc = "ipg_clk_highfreq"]
    CLKSRC_2,
    #[doc = "ipg_clk_32k"]
    CLKSRC_3,
}
impl CLKSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKSRCW::CLKSRC_0 => 0,
            CLKSRCW::CLKSRC_1 => 1,
            CLKSRCW::CLKSRC_2 => 2,
            CLKSRCW::CLKSRC_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKSRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Clock is off"]
    #[inline]
    pub fn clksrc_0(self) -> &'a mut W {
        self.variant(CLKSRCW::CLKSRC_0)
    }
    #[doc = "ipg_clk"]
    #[inline]
    pub fn clksrc_1(self) -> &'a mut W {
        self.variant(CLKSRCW::CLKSRC_1)
    }
    #[doc = "ipg_clk_highfreq"]
    #[inline]
    pub fn clksrc_2(self) -> &'a mut W {
        self.variant(CLKSRCW::CLKSRC_2)
    }
    #[doc = "ipg_clk_32k"]
    #[inline]
    pub fn clksrc_3(self) -> &'a mut W {
        self.variant(CLKSRCW::CLKSRC_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `POUTC`"]
pub enum POUTCW {
    #[doc = "Output pin is set at rollover and cleared at comparison"]
    POUTC_0,
    #[doc = "Output pin is cleared at rollover and set at comparison"]
    POUTC_1,
    #[doc = "PWM output is disconnected"]
    POUTC_2,
    #[doc = "PWM output is disconnected"]
    POUTC_3,
}
impl POUTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            POUTCW::POUTC_0 => 0,
            POUTCW::POUTC_1 => 1,
            POUTCW::POUTC_2 => 2,
            POUTCW::POUTC_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POUTCW<'a> {
    w: &'a mut W,
}
impl<'a> _POUTCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POUTCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output pin is set at rollover and cleared at comparison"]
    #[inline]
    pub fn poutc_0(self) -> &'a mut W {
        self.variant(POUTCW::POUTC_0)
    }
    #[doc = "Output pin is cleared at rollover and set at comparison"]
    #[inline]
    pub fn poutc_1(self) -> &'a mut W {
        self.variant(POUTCW::POUTC_1)
    }
    #[doc = "PWM output is disconnected"]
    #[inline]
    pub fn poutc_2(self) -> &'a mut W {
        self.variant(POUTCW::POUTC_2)
    }
    #[doc = "PWM output is disconnected"]
    #[inline]
    pub fn poutc_3(self) -> &'a mut W {
        self.variant(POUTCW::POUTC_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HCTR`"]
pub enum HCTRW {
    #[doc = "Half word swapping does not take place"]
    HCTR_0,
    #[doc = "Half words from write data bus are swapped"]
    HCTR_1,
}
impl HCTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HCTRW::HCTR_0 => false,
            HCTRW::HCTR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HCTRW<'a> {
    w: &'a mut W,
}
impl<'a> _HCTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HCTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Half word swapping does not take place"]
    #[inline]
    pub fn hctr_0(self) -> &'a mut W {
        self.variant(HCTRW::HCTR_0)
    }
    #[doc = "Half words from write data bus are swapped"]
    #[inline]
    pub fn hctr_1(self) -> &'a mut W {
        self.variant(HCTRW::HCTR_1)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BCTR`"]
pub enum BCTRW {
    #[doc = "byte ordering remains the same"]
    BCTR_0,
    #[doc = "byte ordering is reversed"]
    BCTR_1,
}
impl BCTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BCTRW::BCTR_0 => false,
            BCTRW::BCTR_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCTRW<'a> {
    w: &'a mut W,
}
impl<'a> _BCTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "byte ordering remains the same"]
    #[inline]
    pub fn bctr_0(self) -> &'a mut W {
        self.variant(BCTRW::BCTR_0)
    }
    #[doc = "byte ordering is reversed"]
    #[inline]
    pub fn bctr_1(self) -> &'a mut W {
        self.variant(BCTRW::BCTR_1)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DBGEN`"]
pub enum DBGENW {
    #[doc = "Inactive in debug mode"]
    DBGEN_0,
    #[doc = "Active in debug mode"]
    DBGEN_1,
}
impl DBGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBGENW::DBGEN_0 => false,
            DBGENW::DBGEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBGENW<'a> {
    w: &'a mut W,
}
impl<'a> _DBGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Inactive in debug mode"]
    #[inline]
    pub fn dbgen_0(self) -> &'a mut W {
        self.variant(DBGENW::DBGEN_0)
    }
    #[doc = "Active in debug mode"]
    #[inline]
    pub fn dbgen_1(self) -> &'a mut W {
        self.variant(DBGENW::DBGEN_1)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WAITEN`"]
pub enum WAITENW {
    #[doc = "Inactive in wait mode"]
    WAITEN_0,
    #[doc = "Active in wait mode"]
    WAITEN_1,
}
impl WAITENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAITENW::WAITEN_0 => false,
            WAITENW::WAITEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAITENW<'a> {
    w: &'a mut W,
}
impl<'a> _WAITENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAITENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Inactive in wait mode"]
    #[inline]
    pub fn waiten_0(self) -> &'a mut W {
        self.variant(WAITENW::WAITEN_0)
    }
    #[doc = "Active in wait mode"]
    #[inline]
    pub fn waiten_1(self) -> &'a mut W {
        self.variant(WAITENW::WAITEN_1)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DOZEN`"]
pub enum DOZENW {
    #[doc = "Inactive in doze mode"]
    DOZEN_0,
    #[doc = "Active in doze mode"]
    DOZEN_1,
}
impl DOZENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOZENW::DOZEN_0 => false,
            DOZENW::DOZEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOZENW<'a> {
    w: &'a mut W,
}
impl<'a> _DOZENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOZENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Inactive in doze mode"]
    #[inline]
    pub fn dozen_0(self) -> &'a mut W {
        self.variant(DOZENW::DOZEN_0)
    }
    #[doc = "Active in doze mode"]
    #[inline]
    pub fn dozen_1(self) -> &'a mut W {
        self.variant(DOZENW::DOZEN_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STOPEN`"]
pub enum STOPENW {
    #[doc = "Inactive in stop mode"]
    STOPEN_0,
    #[doc = "Active in stop mode"]
    STOPEN_1,
}
impl STOPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOPENW::STOPEN_0 => false,
            STOPENW::STOPEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPENW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Inactive in stop mode"]
    #[inline]
    pub fn stopen_0(self) -> &'a mut W {
        self.variant(STOPENW::STOPEN_0)
    }
    #[doc = "Active in stop mode"]
    #[inline]
    pub fn stopen_1(self) -> &'a mut W {
        self.variant(STOPENW::STOPEN_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FWM`"]
pub enum FWMW {
    #[doc = "FIFO empty flag is set when there are more than or equal to 1 empty slots in FIFO"]
    FWM_0,
    #[doc = "FIFO empty flag is set when there are more than or equal to 2 empty slots in FIFO"]
    FWM_1,
    #[doc = "FIFO empty flag is set when there are more than or equal to 3 empty slots in FIFO"]
    FWM_2,
    #[doc = "FIFO empty flag is set when there are more than or equal to 4 empty slots in FIFO"]
    FWM_3,
}
impl FWMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FWMW::FWM_0 => 0,
            FWMW::FWM_1 => 1,
            FWMW::FWM_2 => 2,
            FWMW::FWM_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FWMW<'a> {
    w: &'a mut W,
}
impl<'a> _FWMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FWMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FIFO empty flag is set when there are more than or equal to 1 empty slots in FIFO"]
    #[inline]
    pub fn fwm_0(self) -> &'a mut W {
        self.variant(FWMW::FWM_0)
    }
    #[doc = "FIFO empty flag is set when there are more than or equal to 2 empty slots in FIFO"]
    #[inline]
    pub fn fwm_1(self) -> &'a mut W {
        self.variant(FWMW::FWM_1)
    }
    #[doc = "FIFO empty flag is set when there are more than or equal to 3 empty slots in FIFO"]
    #[inline]
    pub fn fwm_2(self) -> &'a mut W {
        self.variant(FWMW::FWM_2)
    }
    #[doc = "FIFO empty flag is set when there are more than or equal to 4 empty slots in FIFO"]
    #[inline]
    pub fn fwm_3(self) -> &'a mut W {
        self.variant(FWMW::FWM_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
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
    #[doc = "Bit 0 - PWM Enable"]
    #[inline]
    pub fn en(&self) -> ENR {
        ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:2 - Sample Repeat"]
    #[inline]
    pub fn repeat(&self) -> REPEATR {
        REPEATR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Software Reset"]
    #[inline]
    pub fn swr(&self) -> SWRR {
        SWRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:15 - Counter Clock Prescaler Value"]
    #[inline]
    pub fn prescaler(&self) -> PRESCALERR {
        PRESCALERR::_from({
            const MASK: u16 = 4095;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 16:17 - Select Clock Source"]
    #[inline]
    pub fn clksrc(&self) -> CLKSRCR {
        CLKSRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - PWM Output Configuration. This bit field determines the mode of PWM output on the output pin."]
    #[inline]
    pub fn poutc(&self) -> POUTCR {
        POUTCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Half-word Data Swap Control"]
    #[inline]
    pub fn hctr(&self) -> HCTRR {
        HCTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Byte Data Swap Control"]
    #[inline]
    pub fn bctr(&self) -> BCTRR {
        BCTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Debug Mode Enable"]
    #[inline]
    pub fn dbgen(&self) -> DBGENR {
        DBGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Wait Mode Enable"]
    #[inline]
    pub fn waiten(&self) -> WAITENR {
        WAITENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Doze Mode Enable"]
    #[inline]
    pub fn dozen(&self) -> DOZENR {
        DOZENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Stop Mode Enable"]
    #[inline]
    pub fn stopen(&self) -> STOPENR {
        STOPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 26:27 - FIFO Water Mark"]
    #[inline]
    pub fn fwm(&self) -> FWMR {
        FWMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bit 0 - PWM Enable"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bits 1:2 - Sample Repeat"]
    #[inline]
    pub fn repeat(&mut self) -> _REPEATW {
        _REPEATW { w: self }
    }
    #[doc = "Bit 3 - Software Reset"]
    #[inline]
    pub fn swr(&mut self) -> _SWRW {
        _SWRW { w: self }
    }
    #[doc = "Bits 4:15 - Counter Clock Prescaler Value"]
    #[inline]
    pub fn prescaler(&mut self) -> _PRESCALERW {
        _PRESCALERW { w: self }
    }
    #[doc = "Bits 16:17 - Select Clock Source"]
    #[inline]
    pub fn clksrc(&mut self) -> _CLKSRCW {
        _CLKSRCW { w: self }
    }
    #[doc = "Bits 18:19 - PWM Output Configuration. This bit field determines the mode of PWM output on the output pin."]
    #[inline]
    pub fn poutc(&mut self) -> _POUTCW {
        _POUTCW { w: self }
    }
    #[doc = "Bit 20 - Half-word Data Swap Control"]
    #[inline]
    pub fn hctr(&mut self) -> _HCTRW {
        _HCTRW { w: self }
    }
    #[doc = "Bit 21 - Byte Data Swap Control"]
    #[inline]
    pub fn bctr(&mut self) -> _BCTRW {
        _BCTRW { w: self }
    }
    #[doc = "Bit 22 - Debug Mode Enable"]
    #[inline]
    pub fn dbgen(&mut self) -> _DBGENW {
        _DBGENW { w: self }
    }
    #[doc = "Bit 23 - Wait Mode Enable"]
    #[inline]
    pub fn waiten(&mut self) -> _WAITENW {
        _WAITENW { w: self }
    }
    #[doc = "Bit 24 - Doze Mode Enable"]
    #[inline]
    pub fn dozen(&mut self) -> _DOZENW {
        _DOZENW { w: self }
    }
    #[doc = "Bit 25 - Stop Mode Enable"]
    #[inline]
    pub fn stopen(&mut self) -> _STOPENW {
        _STOPENW { w: self }
    }
    #[doc = "Bits 26:27 - FIFO Water Mark"]
    #[inline]
    pub fn fwm(&mut self) -> _FWMW {
        _FWMW { w: self }
    }
}
