#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::UART1_RXD_SELECT_INPUT {
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
    #[doc = "Selecting ALT0 mode of pad UART1_RXD for UART1_RXD."]
    UART1_RXD_ALT0,
    #[doc = "Selecting ALT0 mode of pad UART1_TXD for UART1_RXD."]
    UART1_TXD_ALT0,
    #[doc = "Selecting ALT4 mode of pad SAI2_RXFS for UART1_RXD."]
    SAI2_RXFS_ALT4,
    #[doc = "Selecting ALT4 mode of pad SAI2_RXC for UART1_RXD."]
    SAI2_RXC_ALT4,
}
impl DAISYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DAISYR::UART1_RXD_ALT0 => 0,
            DAISYR::UART1_TXD_ALT0 => 1,
            DAISYR::SAI2_RXFS_ALT4 => 2,
            DAISYR::SAI2_RXC_ALT4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DAISYR {
        match value {
            0 => DAISYR::UART1_RXD_ALT0,
            1 => DAISYR::UART1_TXD_ALT0,
            2 => DAISYR::SAI2_RXFS_ALT4,
            3 => DAISYR::SAI2_RXC_ALT4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UART1_RXD_ALT0`"]
    #[inline]
    pub fn is_uart1_rxd_alt0(&self) -> bool {
        *self == DAISYR::UART1_RXD_ALT0
    }
    #[doc = "Checks if the value of the field is `UART1_TXD_ALT0`"]
    #[inline]
    pub fn is_uart1_txd_alt0(&self) -> bool {
        *self == DAISYR::UART1_TXD_ALT0
    }
    #[doc = "Checks if the value of the field is `SAI2_RXFS_ALT4`"]
    #[inline]
    pub fn is_sai2_rxfs_alt4(&self) -> bool {
        *self == DAISYR::SAI2_RXFS_ALT4
    }
    #[doc = "Checks if the value of the field is `SAI2_RXC_ALT4`"]
    #[inline]
    pub fn is_sai2_rxc_alt4(&self) -> bool {
        *self == DAISYR::SAI2_RXC_ALT4
    }
}
#[doc = "Values that can be written to the field `DAISY`"]
pub enum DAISYW {
    #[doc = "Selecting ALT0 mode of pad UART1_RXD for UART1_RXD."]
    UART1_RXD_ALT0,
    #[doc = "Selecting ALT0 mode of pad UART1_TXD for UART1_RXD."]
    UART1_TXD_ALT0,
    #[doc = "Selecting ALT4 mode of pad SAI2_RXFS for UART1_RXD."]
    SAI2_RXFS_ALT4,
    #[doc = "Selecting ALT4 mode of pad SAI2_RXC for UART1_RXD."]
    SAI2_RXC_ALT4,
}
impl DAISYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DAISYW::UART1_RXD_ALT0 => 0,
            DAISYW::UART1_TXD_ALT0 => 1,
            DAISYW::SAI2_RXFS_ALT4 => 2,
            DAISYW::SAI2_RXC_ALT4 => 3,
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
    #[doc = "Selecting ALT0 mode of pad UART1_RXD for UART1_RXD."]
    #[inline]
    pub fn uart1_rxd_alt0(self) -> &'a mut W {
        self.variant(DAISYW::UART1_RXD_ALT0)
    }
    #[doc = "Selecting ALT0 mode of pad UART1_TXD for UART1_RXD."]
    #[inline]
    pub fn uart1_txd_alt0(self) -> &'a mut W {
        self.variant(DAISYW::UART1_TXD_ALT0)
    }
    #[doc = "Selecting ALT4 mode of pad SAI2_RXFS for UART1_RXD."]
    #[inline]
    pub fn sai2_rxfs_alt4(self) -> &'a mut W {
        self.variant(DAISYW::SAI2_RXFS_ALT4)
    }
    #[doc = "Selecting ALT4 mode of pad SAI2_RXC for UART1_RXD."]
    #[inline]
    pub fn sai2_rxc_alt4(self) -> &'a mut W {
        self.variant(DAISYW::SAI2_RXC_ALT4)
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
