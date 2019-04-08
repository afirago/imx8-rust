#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TIMING2 {
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
pub struct DATA_PAUSER {
    bits: u8,
}
impl DATA_PAUSER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CMDADD_PAUSER {
    bits: u8,
}
impl CMDADD_PAUSER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct POSTAMBLE_DELAYR {
    bits: u8,
}
impl POSTAMBLE_DELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PREAMBLE_DELAYR {
    bits: u8,
}
impl PREAMBLE_DELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CE_DELAYR {
    bits: u8,
}
impl CE_DELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RSVD0R {
    bits: u8,
}
impl RSVD0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `READ_LATENCY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_LATENCYR {
    #[doc = "READ LATENCY is 0"]
    READ_LATENCY_0,
    #[doc = "READ LATENCY is 1"]
    READ_LATENCY_1,
    #[doc = "READ LATENCY is 2"]
    READ_LATENCY_2,
    #[doc = "READ LATENCY is 3"]
    READ_LATENCY_3,
    #[doc = "READ LATENCY is 4"]
    READ_LATENCY_4,
    #[doc = "READ LATENCY is 5"]
    READ_LATENCY_5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl READ_LATENCYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            READ_LATENCYR::READ_LATENCY_0 => 0,
            READ_LATENCYR::READ_LATENCY_1 => 1,
            READ_LATENCYR::READ_LATENCY_2 => 2,
            READ_LATENCYR::READ_LATENCY_3 => 3,
            READ_LATENCYR::READ_LATENCY_4 => 4,
            READ_LATENCYR::READ_LATENCY_5 => 5,
            READ_LATENCYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> READ_LATENCYR {
        match value {
            0 => READ_LATENCYR::READ_LATENCY_0,
            1 => READ_LATENCYR::READ_LATENCY_1,
            2 => READ_LATENCYR::READ_LATENCY_2,
            3 => READ_LATENCYR::READ_LATENCY_3,
            4 => READ_LATENCYR::READ_LATENCY_4,
            5 => READ_LATENCYR::READ_LATENCY_5,
            i => READ_LATENCYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `READ_LATENCY_0`"]
    #[inline]
    pub fn is_read_latency_0(&self) -> bool {
        *self == READ_LATENCYR::READ_LATENCY_0
    }
    #[doc = "Checks if the value of the field is `READ_LATENCY_1`"]
    #[inline]
    pub fn is_read_latency_1(&self) -> bool {
        *self == READ_LATENCYR::READ_LATENCY_1
    }
    #[doc = "Checks if the value of the field is `READ_LATENCY_2`"]
    #[inline]
    pub fn is_read_latency_2(&self) -> bool {
        *self == READ_LATENCYR::READ_LATENCY_2
    }
    #[doc = "Checks if the value of the field is `READ_LATENCY_3`"]
    #[inline]
    pub fn is_read_latency_3(&self) -> bool {
        *self == READ_LATENCYR::READ_LATENCY_3
    }
    #[doc = "Checks if the value of the field is `READ_LATENCY_4`"]
    #[inline]
    pub fn is_read_latency_4(&self) -> bool {
        *self == READ_LATENCYR::READ_LATENCY_4
    }
    #[doc = "Checks if the value of the field is `READ_LATENCY_5`"]
    #[inline]
    pub fn is_read_latency_5(&self) -> bool {
        *self == READ_LATENCYR::READ_LATENCY_5
    }
}
#[doc = r" Value of the field"]
pub struct TCRR {
    bits: u8,
}
impl TCRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRPSTHR {
    bits: u8,
}
impl TRPSTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DATA_PAUSEW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_PAUSEW<'a> {
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
pub struct _CMDADD_PAUSEW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDADD_PAUSEW<'a> {
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
pub struct _POSTAMBLE_DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _POSTAMBLE_DELAYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PREAMBLE_DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _PREAMBLE_DELAYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CE_DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _CE_DELAYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `READ_LATENCY`"]
pub enum READ_LATENCYW {
    #[doc = "READ LATENCY is 0"]
    READ_LATENCY_0,
    #[doc = "READ LATENCY is 1"]
    READ_LATENCY_1,
    #[doc = "READ LATENCY is 2"]
    READ_LATENCY_2,
    #[doc = "READ LATENCY is 3"]
    READ_LATENCY_3,
    #[doc = "READ LATENCY is 4"]
    READ_LATENCY_4,
    #[doc = "READ LATENCY is 5"]
    READ_LATENCY_5,
}
impl READ_LATENCYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            READ_LATENCYW::READ_LATENCY_0 => 0,
            READ_LATENCYW::READ_LATENCY_1 => 1,
            READ_LATENCYW::READ_LATENCY_2 => 2,
            READ_LATENCYW::READ_LATENCY_3 => 3,
            READ_LATENCYW::READ_LATENCY_4 => 4,
            READ_LATENCYW::READ_LATENCY_5 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _READ_LATENCYW<'a> {
    w: &'a mut W,
}
impl<'a> _READ_LATENCYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: READ_LATENCYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "READ LATENCY is 0"]
    #[inline]
    pub fn read_latency_0(self) -> &'a mut W {
        self.variant(READ_LATENCYW::READ_LATENCY_0)
    }
    #[doc = "READ LATENCY is 1"]
    #[inline]
    pub fn read_latency_1(self) -> &'a mut W {
        self.variant(READ_LATENCYW::READ_LATENCY_1)
    }
    #[doc = "READ LATENCY is 2"]
    #[inline]
    pub fn read_latency_2(self) -> &'a mut W {
        self.variant(READ_LATENCYW::READ_LATENCY_2)
    }
    #[doc = "READ LATENCY is 3"]
    #[inline]
    pub fn read_latency_3(self) -> &'a mut W {
        self.variant(READ_LATENCYW::READ_LATENCY_3)
    }
    #[doc = "READ LATENCY is 4"]
    #[inline]
    pub fn read_latency_4(self) -> &'a mut W {
        self.variant(READ_LATENCYW::READ_LATENCY_4)
    }
    #[doc = "READ LATENCY is 5"]
    #[inline]
    pub fn read_latency_5(self) -> &'a mut W {
        self.variant(READ_LATENCYW::READ_LATENCY_5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TCRW<'a> {
    w: &'a mut W,
}
impl<'a> _TCRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRPSTHW<'a> {
    w: &'a mut W,
}
impl<'a> _TRPSTHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 0:3 - GPMI delay time from data pause to data resume in GPMICLK cycles"]
    #[inline]
    pub fn data_pause(&self) -> DATA_PAUSER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATA_PAUSER { bits }
    }
    #[doc = "Bits 4:7 - GPMI delay time from command or addres pause to command or address resume in GPMICLK cycles"]
    #[inline]
    pub fn cmdadd_pause(&self) -> CMDADD_PAUSER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMDADD_PAUSER { bits }
    }
    #[doc = "Bits 8:11 - GPMI post-amble delay in GPMICLK cycles. A value of zero is interpreted as 16."]
    #[inline]
    pub fn postamble_delay(&self) -> POSTAMBLE_DELAYR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        POSTAMBLE_DELAYR { bits }
    }
    #[doc = "Bits 12:15 - GPMI pre-amble delay in GPMICLK cycles. A value of zero is interpreted as 16."]
    #[inline]
    pub fn preamble_delay(&self) -> PREAMBLE_DELAYR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PREAMBLE_DELAYR { bits }
    }
    #[doc = "Bits 16:20 - GPMI dealy from CEn assert to W/Rn changing edge. value of zero is interpreted as 32."]
    #[inline]
    pub fn ce_delay(&self) -> CE_DELAYR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CE_DELAYR { bits }
    }
    #[doc = "Bits 21:23 - Always write zeroes to this bit field."]
    #[inline]
    pub fn rsvd0(&self) -> RSVD0R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSVD0R { bits }
    }
    #[doc = "Bits 24:26 - This field is for double data rate read latency configuration. others READ LATENCY is 3"]
    #[inline]
    pub fn read_latency(&self) -> READ_LATENCYR {
        READ_LATENCYR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 27:28 - Only for Toggle NAND timing control delay (TCR+1) GPMICLK cycles for CEn_B low to RE_B low, 0 is less than or equal to TCR, which is less than the PREAMBLE_DELAY"]
    #[inline]
    pub fn tcr(&self) -> TCRR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TCRR { bits }
    }
    #[doc = "Bits 29:31 - Only for Toggle NAND timing control delay TRPSTH GPMICLK cycles for CEn_B high to RE_B high, A value of zero is interpreted as 8"]
    #[inline]
    pub fn trpsth(&self) -> TRPSTHR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRPSTHR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 50475830 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - GPMI delay time from data pause to data resume in GPMICLK cycles"]
    #[inline]
    pub fn data_pause(&mut self) -> _DATA_PAUSEW {
        _DATA_PAUSEW { w: self }
    }
    #[doc = "Bits 4:7 - GPMI delay time from command or addres pause to command or address resume in GPMICLK cycles"]
    #[inline]
    pub fn cmdadd_pause(&mut self) -> _CMDADD_PAUSEW {
        _CMDADD_PAUSEW { w: self }
    }
    #[doc = "Bits 8:11 - GPMI post-amble delay in GPMICLK cycles. A value of zero is interpreted as 16."]
    #[inline]
    pub fn postamble_delay(&mut self) -> _POSTAMBLE_DELAYW {
        _POSTAMBLE_DELAYW { w: self }
    }
    #[doc = "Bits 12:15 - GPMI pre-amble delay in GPMICLK cycles. A value of zero is interpreted as 16."]
    #[inline]
    pub fn preamble_delay(&mut self) -> _PREAMBLE_DELAYW {
        _PREAMBLE_DELAYW { w: self }
    }
    #[doc = "Bits 16:20 - GPMI dealy from CEn assert to W/Rn changing edge. value of zero is interpreted as 32."]
    #[inline]
    pub fn ce_delay(&mut self) -> _CE_DELAYW {
        _CE_DELAYW { w: self }
    }
    #[doc = "Bits 24:26 - This field is for double data rate read latency configuration. others READ LATENCY is 3"]
    #[inline]
    pub fn read_latency(&mut self) -> _READ_LATENCYW {
        _READ_LATENCYW { w: self }
    }
    #[doc = "Bits 27:28 - Only for Toggle NAND timing control delay (TCR+1) GPMICLK cycles for CEn_B low to RE_B low, 0 is less than or equal to TCR, which is less than the PREAMBLE_DELAY"]
    #[inline]
    pub fn tcr(&mut self) -> _TCRW {
        _TCRW { w: self }
    }
    #[doc = "Bits 29:31 - Only for Toggle NAND timing control delay TRPSTH GPMICLK cycles for CEn_B high to RE_B high, A value of zero is interpreted as 8"]
    #[inline]
    pub fn trpsth(&mut self) -> _TRPSTHW {
        _TRPSTHW { w: self }
    }
}
