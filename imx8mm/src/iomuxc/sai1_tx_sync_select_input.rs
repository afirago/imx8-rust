#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SAI1_TX_SYNC_SELECT_INPUT {
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
    #[doc = "Selecting ALT2 mode of pad SAI5_RXD1 for SAI1_TX_SYNC."]
    SAI5_RXD1_ALT2,
    #[doc = "Selecting ALT2 mode of pad SAI5_RXD2 for SAI1_TX_SYNC."]
    SAI5_RXD2_ALT2,
    #[doc = "Selecting ALT2 mode of pad SAI5_RXD3 for SAI1_TX_SYNC."]
    SAI5_RXD3_ALT2,
    #[doc = "Selecting ALT0 mode of pad SAI1_TXFS for SAI1_TX_SYNC."]
    SAI1_TXFS_ALT0,
    #[doc = "Selecting ALT2 mode of pad SAI1_RXD7 for SAI1_TX_SYNC."]
    SAI1_RXD7_ALT2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DAISYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DAISYR::SAI5_RXD1_ALT2 => 0,
            DAISYR::SAI5_RXD2_ALT2 => 1,
            DAISYR::SAI5_RXD3_ALT2 => 2,
            DAISYR::SAI1_TXFS_ALT0 => 3,
            DAISYR::SAI1_RXD7_ALT2 => 4,
            DAISYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DAISYR {
        match value {
            0 => DAISYR::SAI5_RXD1_ALT2,
            1 => DAISYR::SAI5_RXD2_ALT2,
            2 => DAISYR::SAI5_RXD3_ALT2,
            3 => DAISYR::SAI1_TXFS_ALT0,
            4 => DAISYR::SAI1_RXD7_ALT2,
            i => DAISYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SAI5_RXD1_ALT2`"]
    #[inline]
    pub fn is_sai5_rxd1_alt2(&self) -> bool {
        *self == DAISYR::SAI5_RXD1_ALT2
    }
    #[doc = "Checks if the value of the field is `SAI5_RXD2_ALT2`"]
    #[inline]
    pub fn is_sai5_rxd2_alt2(&self) -> bool {
        *self == DAISYR::SAI5_RXD2_ALT2
    }
    #[doc = "Checks if the value of the field is `SAI5_RXD3_ALT2`"]
    #[inline]
    pub fn is_sai5_rxd3_alt2(&self) -> bool {
        *self == DAISYR::SAI5_RXD3_ALT2
    }
    #[doc = "Checks if the value of the field is `SAI1_TXFS_ALT0`"]
    #[inline]
    pub fn is_sai1_txfs_alt0(&self) -> bool {
        *self == DAISYR::SAI1_TXFS_ALT0
    }
    #[doc = "Checks if the value of the field is `SAI1_RXD7_ALT2`"]
    #[inline]
    pub fn is_sai1_rxd7_alt2(&self) -> bool {
        *self == DAISYR::SAI1_RXD7_ALT2
    }
}
#[doc = "Values that can be written to the field `DAISY`"]
pub enum DAISYW {
    #[doc = "Selecting ALT2 mode of pad SAI5_RXD1 for SAI1_TX_SYNC."]
    SAI5_RXD1_ALT2,
    #[doc = "Selecting ALT2 mode of pad SAI5_RXD2 for SAI1_TX_SYNC."]
    SAI5_RXD2_ALT2,
    #[doc = "Selecting ALT2 mode of pad SAI5_RXD3 for SAI1_TX_SYNC."]
    SAI5_RXD3_ALT2,
    #[doc = "Selecting ALT0 mode of pad SAI1_TXFS for SAI1_TX_SYNC."]
    SAI1_TXFS_ALT0,
    #[doc = "Selecting ALT2 mode of pad SAI1_RXD7 for SAI1_TX_SYNC."]
    SAI1_RXD7_ALT2,
}
impl DAISYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DAISYW::SAI5_RXD1_ALT2 => 0,
            DAISYW::SAI5_RXD2_ALT2 => 1,
            DAISYW::SAI5_RXD3_ALT2 => 2,
            DAISYW::SAI1_TXFS_ALT0 => 3,
            DAISYW::SAI1_RXD7_ALT2 => 4,
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
    #[doc = "Selecting ALT2 mode of pad SAI5_RXD1 for SAI1_TX_SYNC."]
    #[inline]
    pub fn sai5_rxd1_alt2(self) -> &'a mut W {
        self.variant(DAISYW::SAI5_RXD1_ALT2)
    }
    #[doc = "Selecting ALT2 mode of pad SAI5_RXD2 for SAI1_TX_SYNC."]
    #[inline]
    pub fn sai5_rxd2_alt2(self) -> &'a mut W {
        self.variant(DAISYW::SAI5_RXD2_ALT2)
    }
    #[doc = "Selecting ALT2 mode of pad SAI5_RXD3 for SAI1_TX_SYNC."]
    #[inline]
    pub fn sai5_rxd3_alt2(self) -> &'a mut W {
        self.variant(DAISYW::SAI5_RXD3_ALT2)
    }
    #[doc = "Selecting ALT0 mode of pad SAI1_TXFS for SAI1_TX_SYNC."]
    #[inline]
    pub fn sai1_txfs_alt0(self) -> &'a mut W {
        self.variant(DAISYW::SAI1_TXFS_ALT0)
    }
    #[doc = "Selecting ALT2 mode of pad SAI1_RXD7 for SAI1_TX_SYNC."]
    #[inline]
    pub fn sai1_rxd7_alt2(self) -> &'a mut W {
        self.variant(DAISYW::SAI1_RXD7_ALT2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 0:2 - Input Select (DAISY) Field"]
    #[inline]
    pub fn daisy(&self) -> DAISYR {
        DAISYR::_from({
            const MASK: u8 = 7;
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
    #[doc = "Bits 0:2 - Input Select (DAISY) Field"]
    #[inline]
    pub fn daisy(&mut self) -> _DAISYW {
        _DAISYW { w: self }
    }
}
