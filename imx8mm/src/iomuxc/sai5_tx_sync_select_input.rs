#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SAI5_TX_SYNC_SELECT_INPUT {
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
    #[doc = "Selecting ALT3 mode of pad SAI5_RXD1 for SAI5_TX_SYNC."]
    SAI5_RXD1_ALT3,
    #[doc = "Selecting ALT1 mode of pad SAI1_TXFS for SAI5_TX_SYNC."]
    SAI1_TXFS_ALT1,
    #[doc = "Selecting ALT1 mode of pad SAI2_RXFS for SAI5_TX_SYNC."]
    SAI2_RXFS_ALT1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DAISYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DAISYR::SAI5_RXD1_ALT3 => 0,
            DAISYR::SAI1_TXFS_ALT1 => 1,
            DAISYR::SAI2_RXFS_ALT1 => 2,
            DAISYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DAISYR {
        match value {
            0 => DAISYR::SAI5_RXD1_ALT3,
            1 => DAISYR::SAI1_TXFS_ALT1,
            2 => DAISYR::SAI2_RXFS_ALT1,
            i => DAISYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SAI5_RXD1_ALT3`"]
    #[inline]
    pub fn is_sai5_rxd1_alt3(&self) -> bool {
        *self == DAISYR::SAI5_RXD1_ALT3
    }
    #[doc = "Checks if the value of the field is `SAI1_TXFS_ALT1`"]
    #[inline]
    pub fn is_sai1_txfs_alt1(&self) -> bool {
        *self == DAISYR::SAI1_TXFS_ALT1
    }
    #[doc = "Checks if the value of the field is `SAI2_RXFS_ALT1`"]
    #[inline]
    pub fn is_sai2_rxfs_alt1(&self) -> bool {
        *self == DAISYR::SAI2_RXFS_ALT1
    }
}
#[doc = "Values that can be written to the field `DAISY`"]
pub enum DAISYW {
    #[doc = "Selecting ALT3 mode of pad SAI5_RXD1 for SAI5_TX_SYNC."]
    SAI5_RXD1_ALT3,
    #[doc = "Selecting ALT1 mode of pad SAI1_TXFS for SAI5_TX_SYNC."]
    SAI1_TXFS_ALT1,
    #[doc = "Selecting ALT1 mode of pad SAI2_RXFS for SAI5_TX_SYNC."]
    SAI2_RXFS_ALT1,
}
impl DAISYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DAISYW::SAI5_RXD1_ALT3 => 0,
            DAISYW::SAI1_TXFS_ALT1 => 1,
            DAISYW::SAI2_RXFS_ALT1 => 2,
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
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Selecting ALT3 mode of pad SAI5_RXD1 for SAI5_TX_SYNC."]
    #[inline]
    pub fn sai5_rxd1_alt3(self) -> &'a mut W {
        self.variant(DAISYW::SAI5_RXD1_ALT3)
    }
    #[doc = "Selecting ALT1 mode of pad SAI1_TXFS for SAI5_TX_SYNC."]
    #[inline]
    pub fn sai1_txfs_alt1(self) -> &'a mut W {
        self.variant(DAISYW::SAI1_TXFS_ALT1)
    }
    #[doc = "Selecting ALT1 mode of pad SAI2_RXFS for SAI5_TX_SYNC."]
    #[inline]
    pub fn sai2_rxfs_alt1(self) -> &'a mut W {
        self.variant(DAISYW::SAI2_RXFS_ALT1)
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
