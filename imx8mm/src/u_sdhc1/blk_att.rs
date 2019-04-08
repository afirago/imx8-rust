#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BLK_ATT {
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
#[doc = "Possible values of the field `BLKSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLKSIZER {
    #[doc = "No data transfer"]
    BLKSIZE_0,
    #[doc = "1 Byte"]
    BLKSIZE_1,
    #[doc = "2 Bytes"]
    BLKSIZE_2,
    #[doc = "3 Bytes"]
    BLKSIZE_3,
    #[doc = "4 Bytes"]
    BLKSIZE_4,
    #[doc = "4096 Bytes"]
    BLKSIZE_8,
    #[doc = "512 Bytes"]
    BLKSIZE_200,
    #[doc = "2048 Bytes"]
    BLKSIZE_800,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl BLKSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            BLKSIZER::BLKSIZE_0 => 0,
            BLKSIZER::BLKSIZE_1 => 1,
            BLKSIZER::BLKSIZE_2 => 2,
            BLKSIZER::BLKSIZE_3 => 3,
            BLKSIZER::BLKSIZE_4 => 4,
            BLKSIZER::BLKSIZE_8 => 8,
            BLKSIZER::BLKSIZE_200 => 200,
            BLKSIZER::BLKSIZE_800 => 800,
            BLKSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> BLKSIZER {
        match value {
            0 => BLKSIZER::BLKSIZE_0,
            1 => BLKSIZER::BLKSIZE_1,
            2 => BLKSIZER::BLKSIZE_2,
            3 => BLKSIZER::BLKSIZE_3,
            4 => BLKSIZER::BLKSIZE_4,
            8 => BLKSIZER::BLKSIZE_8,
            200 => BLKSIZER::BLKSIZE_200,
            800 => BLKSIZER::BLKSIZE_800,
            i => BLKSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_0`"]
    #[inline]
    pub fn is_blksize_0(&self) -> bool {
        *self == BLKSIZER::BLKSIZE_0
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_1`"]
    #[inline]
    pub fn is_blksize_1(&self) -> bool {
        *self == BLKSIZER::BLKSIZE_1
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_2`"]
    #[inline]
    pub fn is_blksize_2(&self) -> bool {
        *self == BLKSIZER::BLKSIZE_2
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_3`"]
    #[inline]
    pub fn is_blksize_3(&self) -> bool {
        *self == BLKSIZER::BLKSIZE_3
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_4`"]
    #[inline]
    pub fn is_blksize_4(&self) -> bool {
        *self == BLKSIZER::BLKSIZE_4
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_8`"]
    #[inline]
    pub fn is_blksize_8(&self) -> bool {
        *self == BLKSIZER::BLKSIZE_8
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_200`"]
    #[inline]
    pub fn is_blksize_200(&self) -> bool {
        *self == BLKSIZER::BLKSIZE_200
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_800`"]
    #[inline]
    pub fn is_blksize_800(&self) -> bool {
        *self == BLKSIZER::BLKSIZE_800
    }
}
#[doc = "Possible values of the field `BLKCNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLKCNTR {
    #[doc = "Stop Count"]
    BLKCNT_0,
    #[doc = "1 block"]
    BLKCNT_1,
    #[doc = "2 blocks"]
    BLKCNT_2,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl BLKCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            BLKCNTR::BLKCNT_0 => 0,
            BLKCNTR::BLKCNT_1 => 1,
            BLKCNTR::BLKCNT_2 => 2,
            BLKCNTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> BLKCNTR {
        match value {
            0 => BLKCNTR::BLKCNT_0,
            1 => BLKCNTR::BLKCNT_1,
            2 => BLKCNTR::BLKCNT_2,
            i => BLKCNTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLKCNT_0`"]
    #[inline]
    pub fn is_blkcnt_0(&self) -> bool {
        *self == BLKCNTR::BLKCNT_0
    }
    #[doc = "Checks if the value of the field is `BLKCNT_1`"]
    #[inline]
    pub fn is_blkcnt_1(&self) -> bool {
        *self == BLKCNTR::BLKCNT_1
    }
    #[doc = "Checks if the value of the field is `BLKCNT_2`"]
    #[inline]
    pub fn is_blkcnt_2(&self) -> bool {
        *self == BLKCNTR::BLKCNT_2
    }
}
#[doc = "Values that can be written to the field `BLKSIZE`"]
pub enum BLKSIZEW {
    #[doc = "No data transfer"]
    BLKSIZE_0,
    #[doc = "1 Byte"]
    BLKSIZE_1,
    #[doc = "2 Bytes"]
    BLKSIZE_2,
    #[doc = "3 Bytes"]
    BLKSIZE_3,
    #[doc = "4 Bytes"]
    BLKSIZE_4,
    #[doc = "4096 Bytes"]
    BLKSIZE_8,
    #[doc = "512 Bytes"]
    BLKSIZE_200,
    #[doc = "2048 Bytes"]
    BLKSIZE_800,
}
impl BLKSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            BLKSIZEW::BLKSIZE_0 => 0,
            BLKSIZEW::BLKSIZE_1 => 1,
            BLKSIZEW::BLKSIZE_2 => 2,
            BLKSIZEW::BLKSIZE_3 => 3,
            BLKSIZEW::BLKSIZE_4 => 4,
            BLKSIZEW::BLKSIZE_8 => 8,
            BLKSIZEW::BLKSIZE_200 => 200,
            BLKSIZEW::BLKSIZE_800 => 800,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLKSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _BLKSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLKSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No data transfer"]
    #[inline]
    pub fn blksize_0(self) -> &'a mut W {
        self.variant(BLKSIZEW::BLKSIZE_0)
    }
    #[doc = "1 Byte"]
    #[inline]
    pub fn blksize_1(self) -> &'a mut W {
        self.variant(BLKSIZEW::BLKSIZE_1)
    }
    #[doc = "2 Bytes"]
    #[inline]
    pub fn blksize_2(self) -> &'a mut W {
        self.variant(BLKSIZEW::BLKSIZE_2)
    }
    #[doc = "3 Bytes"]
    #[inline]
    pub fn blksize_3(self) -> &'a mut W {
        self.variant(BLKSIZEW::BLKSIZE_3)
    }
    #[doc = "4 Bytes"]
    #[inline]
    pub fn blksize_4(self) -> &'a mut W {
        self.variant(BLKSIZEW::BLKSIZE_4)
    }
    #[doc = "4096 Bytes"]
    #[inline]
    pub fn blksize_8(self) -> &'a mut W {
        self.variant(BLKSIZEW::BLKSIZE_8)
    }
    #[doc = "512 Bytes"]
    #[inline]
    pub fn blksize_200(self) -> &'a mut W {
        self.variant(BLKSIZEW::BLKSIZE_200)
    }
    #[doc = "2048 Bytes"]
    #[inline]
    pub fn blksize_800(self) -> &'a mut W {
        self.variant(BLKSIZEW::BLKSIZE_800)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 8191;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BLKCNT`"]
pub enum BLKCNTW {
    #[doc = "Stop Count"]
    BLKCNT_0,
    #[doc = "1 block"]
    BLKCNT_1,
    #[doc = "2 blocks"]
    BLKCNT_2,
}
impl BLKCNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            BLKCNTW::BLKCNT_0 => 0,
            BLKCNTW::BLKCNT_1 => 1,
            BLKCNTW::BLKCNT_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLKCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _BLKCNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLKCNTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Stop Count"]
    #[inline]
    pub fn blkcnt_0(self) -> &'a mut W {
        self.variant(BLKCNTW::BLKCNT_0)
    }
    #[doc = "1 block"]
    #[inline]
    pub fn blkcnt_1(self) -> &'a mut W {
        self.variant(BLKCNTW::BLKCNT_1)
    }
    #[doc = "2 blocks"]
    #[inline]
    pub fn blkcnt_2(self) -> &'a mut W {
        self.variant(BLKCNTW::BLKCNT_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:12 - Transfer Block Size: This register specifies the block size for block data transfers"]
    #[inline]
    pub fn blksize(&self) -> BLKSIZER {
        BLKSIZER::_from({
            const MASK: u16 = 8191;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 16:31 - Blocks Count For Current Transfer: This register is enabled when the Block Count Enable bit in the Transfer Mode register is set to 1 and is valid only for multiple block transfers"]
    #[inline]
    pub fn blkcnt(&self) -> BLKCNTR {
        BLKCNTR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
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
    #[doc = "Bits 0:12 - Transfer Block Size: This register specifies the block size for block data transfers"]
    #[inline]
    pub fn blksize(&mut self) -> _BLKSIZEW {
        _BLKSIZEW { w: self }
    }
    #[doc = "Bits 16:31 - Blocks Count For Current Transfer: This register is enabled when the Block Count Enable bit in the Transfer Mode register is set to 1 and is valid only for multiple block transfers"]
    #[inline]
    pub fn blkcnt(&mut self) -> _BLKCNTW {
        _BLKCNTW { w: self }
    }
}
