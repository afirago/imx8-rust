#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::UART3_RXD_SELECT_INPUT {
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
#[doc = "Possible values of the field `DAISY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAISYR {
    #[doc = "Selecting ALT1 mode of pad ECSPI1_SCLK for UART3_RXD."]
    ECSPI1_SCLK_ALT1,
    #[doc = "Selecting ALT1 mode of pad ECSPI1_MOSI for UART3_RXD."]
    ECSPI1_MOSI_ALT1,
    #[doc = "Selecting ALT0 mode of pad UART3_RXD for UART3_RXD."]
    UART3_RXD_ALT0,
    #[doc = "Selecting ALT0 mode of pad UART3_TXD for UART3_RXD."]
    UART3_TXD_ALT0,
}
impl DAISYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DAISYR::ECSPI1_SCLK_ALT1 => 0,
            DAISYR::ECSPI1_MOSI_ALT1 => 1,
            DAISYR::UART3_RXD_ALT0 => 2,
            DAISYR::UART3_TXD_ALT0 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DAISYR {
        match value {
            0 => DAISYR::ECSPI1_SCLK_ALT1,
            1 => DAISYR::ECSPI1_MOSI_ALT1,
            2 => DAISYR::UART3_RXD_ALT0,
            3 => DAISYR::UART3_TXD_ALT0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ECSPI1_SCLK_ALT1`"]
    #[inline]
    pub fn is_ecspi1_sclk_alt1(&self) -> bool {
        *self == DAISYR::ECSPI1_SCLK_ALT1
    }
    #[doc = "Checks if the value of the field is `ECSPI1_MOSI_ALT1`"]
    #[inline]
    pub fn is_ecspi1_mosi_alt1(&self) -> bool {
        *self == DAISYR::ECSPI1_MOSI_ALT1
    }
    #[doc = "Checks if the value of the field is `UART3_RXD_ALT0`"]
    #[inline]
    pub fn is_uart3_rxd_alt0(&self) -> bool {
        *self == DAISYR::UART3_RXD_ALT0
    }
    #[doc = "Checks if the value of the field is `UART3_TXD_ALT0`"]
    #[inline]
    pub fn is_uart3_txd_alt0(&self) -> bool {
        *self == DAISYR::UART3_TXD_ALT0
    }
}
#[doc = "Values that can be written to the field `DAISY`"]
pub enum DAISYW {
    #[doc = "Selecting ALT1 mode of pad ECSPI1_SCLK for UART3_RXD."]
    ECSPI1_SCLK_ALT1,
    #[doc = "Selecting ALT1 mode of pad ECSPI1_MOSI for UART3_RXD."]
    ECSPI1_MOSI_ALT1,
    #[doc = "Selecting ALT0 mode of pad UART3_RXD for UART3_RXD."]
    UART3_RXD_ALT0,
    #[doc = "Selecting ALT0 mode of pad UART3_TXD for UART3_RXD."]
    UART3_TXD_ALT0,
}
impl DAISYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DAISYW::ECSPI1_SCLK_ALT1 => 0,
            DAISYW::ECSPI1_MOSI_ALT1 => 1,
            DAISYW::UART3_RXD_ALT0 => 2,
            DAISYW::UART3_TXD_ALT0 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DAISYW<'a> {
    w: &'a mut W,
}
impl<'a> _DAISYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DAISYW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Selecting ALT1 mode of pad ECSPI1_SCLK for UART3_RXD."]
    #[inline]
    pub fn ecspi1_sclk_alt1(self) -> &'a mut W {
        self.variant(DAISYW::ECSPI1_SCLK_ALT1)
    }
    #[doc = "Selecting ALT1 mode of pad ECSPI1_MOSI for UART3_RXD."]
    #[inline]
    pub fn ecspi1_mosi_alt1(self) -> &'a mut W {
        self.variant(DAISYW::ECSPI1_MOSI_ALT1)
    }
    #[doc = "Selecting ALT0 mode of pad UART3_RXD for UART3_RXD."]
    #[inline]
    pub fn uart3_rxd_alt0(self) -> &'a mut W {
        self.variant(DAISYW::UART3_RXD_ALT0)
    }
    #[doc = "Selecting ALT0 mode of pad UART3_TXD for UART3_RXD."]
    #[inline]
    pub fn uart3_txd_alt0(self) -> &'a mut W {
        self.variant(DAISYW::UART3_TXD_ALT0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:1 - Input Select (DAISY) Field"]
    #[inline]
    pub fn daisy(&self) -> DAISYR {
        DAISYR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:1 - Input Select (DAISY) Field"]
    #[inline]
    pub fn daisy(&mut self) -> _DAISYW {
        _DAISYW { w: self }
    }
}
