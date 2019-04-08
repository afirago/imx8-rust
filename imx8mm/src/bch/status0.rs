#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STATUS0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
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
#[doc = r" Value of the field"]
pub struct UNCORRECTABLER {
    bits: bool,
}
impl UNCORRECTABLER {
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
pub struct CORRECTEDR {
    bits: bool,
}
impl CORRECTEDR {
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
pub struct ALLONESR {
    bits: bool,
}
impl ALLONESR {
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
pub struct RSVD1R {
    bits: u8,
}
impl RSVD1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `STATUS_BLK0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATUS_BLK0R {
    #[doc = "No errors found on block."]
    ZERO,
    #[doc = "One error found on block."]
    ERROR1,
    #[doc = "One errors found on block."]
    ERROR2,
    #[doc = "One errors found on block."]
    ERROR3,
    #[doc = "One errors found on block."]
    ERROR4,
    #[doc = "Block exhibited uncorrectable errors."]
    UNCORRECTABLE,
    #[doc = "Page is erased."]
    ERASED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STATUS_BLK0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STATUS_BLK0R::ZERO => 0,
            STATUS_BLK0R::ERROR1 => 1,
            STATUS_BLK0R::ERROR2 => 2,
            STATUS_BLK0R::ERROR3 => 3,
            STATUS_BLK0R::ERROR4 => 4,
            STATUS_BLK0R::UNCORRECTABLE => 254,
            STATUS_BLK0R::ERASED => 255,
            STATUS_BLK0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STATUS_BLK0R {
        match value {
            0 => STATUS_BLK0R::ZERO,
            1 => STATUS_BLK0R::ERROR1,
            2 => STATUS_BLK0R::ERROR2,
            3 => STATUS_BLK0R::ERROR3,
            4 => STATUS_BLK0R::ERROR4,
            254 => STATUS_BLK0R::UNCORRECTABLE,
            255 => STATUS_BLK0R::ERASED,
            i => STATUS_BLK0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == STATUS_BLK0R::ZERO
    }
    #[doc = "Checks if the value of the field is `ERROR1`"]
    #[inline]
    pub fn is_error1(&self) -> bool {
        *self == STATUS_BLK0R::ERROR1
    }
    #[doc = "Checks if the value of the field is `ERROR2`"]
    #[inline]
    pub fn is_error2(&self) -> bool {
        *self == STATUS_BLK0R::ERROR2
    }
    #[doc = "Checks if the value of the field is `ERROR3`"]
    #[inline]
    pub fn is_error3(&self) -> bool {
        *self == STATUS_BLK0R::ERROR3
    }
    #[doc = "Checks if the value of the field is `ERROR4`"]
    #[inline]
    pub fn is_error4(&self) -> bool {
        *self == STATUS_BLK0R::ERROR4
    }
    #[doc = "Checks if the value of the field is `UNCORRECTABLE`"]
    #[inline]
    pub fn is_uncorrectable(&self) -> bool {
        *self == STATUS_BLK0R::UNCORRECTABLE
    }
    #[doc = "Checks if the value of the field is `ERASED`"]
    #[inline]
    pub fn is_erased(&self) -> bool {
        *self == STATUS_BLK0R::ERASED
    }
}
#[doc = r" Value of the field"]
pub struct COMPLETED_CER {
    bits: u8,
}
impl COMPLETED_CER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HANDLER {
    bits: u16,
}
impl HANDLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - This field is reserved."]
    #[inline]
    pub fn rsvd0(&self) -> RSVD0R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSVD0R { bits }
    }
    #[doc = "Bit 2 - 1 = Uncorrectable error encountered during last processing cycle."]
    #[inline]
    pub fn uncorrectable(&self) -> UNCORRECTABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UNCORRECTABLER { bits }
    }
    #[doc = "Bit 3 - 1 = At least one correctable error encountered during last processing cycle."]
    #[inline]
    pub fn corrected(&self) -> CORRECTEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CORRECTEDR { bits }
    }
    #[doc = "Bit 4 - 1 = All data bits of this transaction are ONE."]
    #[inline]
    pub fn allones(&self) -> ALLONESR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ALLONESR { bits }
    }
    #[doc = "Bits 5:7 - This field is reserved."]
    #[inline]
    pub fn rsvd1(&self) -> RSVD1R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSVD1R { bits }
    }
    #[doc = "Bits 8:15 - Count of symbols in error during processing of first block of flash (metadata block)"]
    #[inline]
    pub fn status_blk0(&self) -> STATUS_BLK0R {
        STATUS_BLK0R::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - This is the chip enable number corresponding to the NAND device from which this data came."]
    #[inline]
    pub fn completed_ce(&self) -> COMPLETED_CER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        COMPLETED_CER { bits }
    }
    #[doc = "Bits 20:31 - Software supplies a 12 bit handle for this transfer as part of the GPMI DMA PIO operation that started the transaction"]
    #[inline]
    pub fn handle(&self) -> HANDLER {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        HANDLER { bits }
    }
}
