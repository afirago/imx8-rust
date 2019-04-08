#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `PRESENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESENTR {
    #[doc = "GPMI is not present in this product."]
    PRESENT_0,
    #[doc = "GPMI is present is in this product."]
    PRESENT_1,
}
impl PRESENTR {
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
            PRESENTR::PRESENT_0 => false,
            PRESENTR::PRESENT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRESENTR {
        match value {
            false => PRESENTR::PRESENT_0,
            true => PRESENTR::PRESENT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PRESENT_0`"]
    #[inline]
    pub fn is_present_0(&self) -> bool {
        *self == PRESENTR::PRESENT_0
    }
    #[doc = "Checks if the value of the field is `PRESENT_1`"]
    #[inline]
    pub fn is_present_1(&self) -> bool {
        *self == PRESENTR::PRESENT_1
    }
}
#[doc = "Possible values of the field `FIFO_FULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_FULLR {
    #[doc = "FIFO is not full."]
    FIFO_FULL_0,
    #[doc = "FIFO is full."]
    FIFO_FULL_1,
}
impl FIFO_FULLR {
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
            FIFO_FULLR::FIFO_FULL_0 => false,
            FIFO_FULLR::FIFO_FULL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIFO_FULLR {
        match value {
            false => FIFO_FULLR::FIFO_FULL_0,
            true => FIFO_FULLR::FIFO_FULL_1,
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_FULL_0`"]
    #[inline]
    pub fn is_fifo_full_0(&self) -> bool {
        *self == FIFO_FULLR::FIFO_FULL_0
    }
    #[doc = "Checks if the value of the field is `FIFO_FULL_1`"]
    #[inline]
    pub fn is_fifo_full_1(&self) -> bool {
        *self == FIFO_FULLR::FIFO_FULL_1
    }
}
#[doc = "Possible values of the field `FIFO_EMPTY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_EMPTYR {
    #[doc = "FIFO is not empty."]
    FIFO_EMPTY_0,
    #[doc = "FIFO is empty."]
    FIFO_EMPTY_1,
}
impl FIFO_EMPTYR {
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
            FIFO_EMPTYR::FIFO_EMPTY_0 => false,
            FIFO_EMPTYR::FIFO_EMPTY_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIFO_EMPTYR {
        match value {
            false => FIFO_EMPTYR::FIFO_EMPTY_0,
            true => FIFO_EMPTYR::FIFO_EMPTY_1,
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_EMPTY_0`"]
    #[inline]
    pub fn is_fifo_empty_0(&self) -> bool {
        *self == FIFO_EMPTYR::FIFO_EMPTY_0
    }
    #[doc = "Checks if the value of the field is `FIFO_EMPTY_1`"]
    #[inline]
    pub fn is_fifo_empty_1(&self) -> bool {
        *self == FIFO_EMPTYR::FIFO_EMPTY_1
    }
}
#[doc = "Possible values of the field `INVALID_BUFFER_MASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVALID_BUFFER_MASKR {
    #[doc = "ECC Buffer Mask is not invalid."]
    INVALID_BUFFER_MASK_0,
    #[doc = "ECC Buffer Mask is invalid."]
    INVALID_BUFFER_MASK_1,
}
impl INVALID_BUFFER_MASKR {
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
            INVALID_BUFFER_MASKR::INVALID_BUFFER_MASK_0 => false,
            INVALID_BUFFER_MASKR::INVALID_BUFFER_MASK_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVALID_BUFFER_MASKR {
        match value {
            false => INVALID_BUFFER_MASKR::INVALID_BUFFER_MASK_0,
            true => INVALID_BUFFER_MASKR::INVALID_BUFFER_MASK_1,
        }
    }
    #[doc = "Checks if the value of the field is `INVALID_BUFFER_MASK_0`"]
    #[inline]
    pub fn is_invalid_buffer_mask_0(&self) -> bool {
        *self == INVALID_BUFFER_MASKR::INVALID_BUFFER_MASK_0
    }
    #[doc = "Checks if the value of the field is `INVALID_BUFFER_MASK_1`"]
    #[inline]
    pub fn is_invalid_buffer_mask_1(&self) -> bool {
        *self == INVALID_BUFFER_MASKR::INVALID_BUFFER_MASK_1
    }
}
#[doc = r" Value of the field"]
pub struct ATA_IRQR {
    bits: bool,
}
impl ATA_IRQR {
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
#[doc = "Possible values of the field `DEV0_ERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEV0_ERRORR {
    #[doc = "No error condition present on ATA/NAND Device accessed by DMA channel 0."]
    DEV0_ERROR_0,
    #[doc = "An Error has occurred on ATA/NAND Device accessed by"]
    DEV0_ERROR_1,
}
impl DEV0_ERRORR {
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
            DEV0_ERRORR::DEV0_ERROR_0 => false,
            DEV0_ERRORR::DEV0_ERROR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEV0_ERRORR {
        match value {
            false => DEV0_ERRORR::DEV0_ERROR_0,
            true => DEV0_ERRORR::DEV0_ERROR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEV0_ERROR_0`"]
    #[inline]
    pub fn is_dev0_error_0(&self) -> bool {
        *self == DEV0_ERRORR::DEV0_ERROR_0
    }
    #[doc = "Checks if the value of the field is `DEV0_ERROR_1`"]
    #[inline]
    pub fn is_dev0_error_1(&self) -> bool {
        *self == DEV0_ERRORR::DEV0_ERROR_1
    }
}
#[doc = "Possible values of the field `DEV1_ERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEV1_ERRORR {
    #[doc = "No error condition present on ATA/NAND Device accessed by DMA channel 1."]
    DEV1_ERROR_0,
    #[doc = "An Error has occurred on ATA/NAND Device accessed by"]
    DEV1_ERROR_1,
}
impl DEV1_ERRORR {
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
            DEV1_ERRORR::DEV1_ERROR_0 => false,
            DEV1_ERRORR::DEV1_ERROR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEV1_ERRORR {
        match value {
            false => DEV1_ERRORR::DEV1_ERROR_0,
            true => DEV1_ERRORR::DEV1_ERROR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEV1_ERROR_0`"]
    #[inline]
    pub fn is_dev1_error_0(&self) -> bool {
        *self == DEV1_ERRORR::DEV1_ERROR_0
    }
    #[doc = "Checks if the value of the field is `DEV1_ERROR_1`"]
    #[inline]
    pub fn is_dev1_error_1(&self) -> bool {
        *self == DEV1_ERRORR::DEV1_ERROR_1
    }
}
#[doc = "Possible values of the field `DEV2_ERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEV2_ERRORR {
    #[doc = "No error condition present on ATA/NAND Device accessed by DMA channel 2."]
    DEV2_ERROR_0,
    #[doc = "An Error has occurred on ATA/NAND Device accessed by"]
    DEV2_ERROR_1,
}
impl DEV2_ERRORR {
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
            DEV2_ERRORR::DEV2_ERROR_0 => false,
            DEV2_ERRORR::DEV2_ERROR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEV2_ERRORR {
        match value {
            false => DEV2_ERRORR::DEV2_ERROR_0,
            true => DEV2_ERRORR::DEV2_ERROR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEV2_ERROR_0`"]
    #[inline]
    pub fn is_dev2_error_0(&self) -> bool {
        *self == DEV2_ERRORR::DEV2_ERROR_0
    }
    #[doc = "Checks if the value of the field is `DEV2_ERROR_1`"]
    #[inline]
    pub fn is_dev2_error_1(&self) -> bool {
        *self == DEV2_ERRORR::DEV2_ERROR_1
    }
}
#[doc = "Possible values of the field `DEV3_ERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEV3_ERRORR {
    #[doc = "No error condition present on ATA/NAND Device accessed by DMA channel 3."]
    DEV3_ERROR_0,
    #[doc = "An Error has occurred on ATA/NAND Device accessed by"]
    DEV3_ERROR_1,
}
impl DEV3_ERRORR {
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
            DEV3_ERRORR::DEV3_ERROR_0 => false,
            DEV3_ERRORR::DEV3_ERROR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEV3_ERRORR {
        match value {
            false => DEV3_ERRORR::DEV3_ERROR_0,
            true => DEV3_ERRORR::DEV3_ERROR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEV3_ERROR_0`"]
    #[inline]
    pub fn is_dev3_error_0(&self) -> bool {
        *self == DEV3_ERRORR::DEV3_ERROR_0
    }
    #[doc = "Checks if the value of the field is `DEV3_ERROR_1`"]
    #[inline]
    pub fn is_dev3_error_1(&self) -> bool {
        *self == DEV3_ERRORR::DEV3_ERROR_1
    }
}
#[doc = "Possible values of the field `DEV4_ERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEV4_ERRORR {
    #[doc = "No error condition present on ATA/NAND Device accessed by DMA channel 4."]
    DEV4_ERROR_0,
    #[doc = "An Error has occurred on ATA/NAND Device accessed by"]
    DEV4_ERROR_1,
}
impl DEV4_ERRORR {
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
            DEV4_ERRORR::DEV4_ERROR_0 => false,
            DEV4_ERRORR::DEV4_ERROR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEV4_ERRORR {
        match value {
            false => DEV4_ERRORR::DEV4_ERROR_0,
            true => DEV4_ERRORR::DEV4_ERROR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEV4_ERROR_0`"]
    #[inline]
    pub fn is_dev4_error_0(&self) -> bool {
        *self == DEV4_ERRORR::DEV4_ERROR_0
    }
    #[doc = "Checks if the value of the field is `DEV4_ERROR_1`"]
    #[inline]
    pub fn is_dev4_error_1(&self) -> bool {
        *self == DEV4_ERRORR::DEV4_ERROR_1
    }
}
#[doc = "Possible values of the field `DEV5_ERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEV5_ERRORR {
    #[doc = "No error condition present on ATA/NAND Device accessed by DMA channel 5."]
    DEV5_ERROR_0,
    #[doc = "An Error has occurred on ATA/NAND Device accessed by"]
    DEV5_ERROR_1,
}
impl DEV5_ERRORR {
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
            DEV5_ERRORR::DEV5_ERROR_0 => false,
            DEV5_ERRORR::DEV5_ERROR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEV5_ERRORR {
        match value {
            false => DEV5_ERRORR::DEV5_ERROR_0,
            true => DEV5_ERRORR::DEV5_ERROR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEV5_ERROR_0`"]
    #[inline]
    pub fn is_dev5_error_0(&self) -> bool {
        *self == DEV5_ERRORR::DEV5_ERROR_0
    }
    #[doc = "Checks if the value of the field is `DEV5_ERROR_1`"]
    #[inline]
    pub fn is_dev5_error_1(&self) -> bool {
        *self == DEV5_ERRORR::DEV5_ERROR_1
    }
}
#[doc = "Possible values of the field `DEV6_ERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEV6_ERRORR {
    #[doc = "No error condition present on ATA/NAND Device accessed by DMA channel 6."]
    DEV6_ERROR_0,
    #[doc = "An Error has occurred on ATA/NAND Device accessed by"]
    DEV6_ERROR_1,
}
impl DEV6_ERRORR {
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
            DEV6_ERRORR::DEV6_ERROR_0 => false,
            DEV6_ERRORR::DEV6_ERROR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEV6_ERRORR {
        match value {
            false => DEV6_ERRORR::DEV6_ERROR_0,
            true => DEV6_ERRORR::DEV6_ERROR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEV6_ERROR_0`"]
    #[inline]
    pub fn is_dev6_error_0(&self) -> bool {
        *self == DEV6_ERRORR::DEV6_ERROR_0
    }
    #[doc = "Checks if the value of the field is `DEV6_ERROR_1`"]
    #[inline]
    pub fn is_dev6_error_1(&self) -> bool {
        *self == DEV6_ERRORR::DEV6_ERROR_1
    }
}
#[doc = "Possible values of the field `DEV7_ERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEV7_ERRORR {
    #[doc = "No error condition present on ATA/NAND Device accessed by DMA channel 7."]
    DEV7_ERROR_0,
    #[doc = "An Error has occurred on ATA/NAND Device accessed by"]
    DEV7_ERROR_1,
}
impl DEV7_ERRORR {
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
            DEV7_ERRORR::DEV7_ERROR_0 => false,
            DEV7_ERRORR::DEV7_ERROR_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEV7_ERRORR {
        match value {
            false => DEV7_ERRORR::DEV7_ERROR_0,
            true => DEV7_ERRORR::DEV7_ERROR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEV7_ERROR_0`"]
    #[inline]
    pub fn is_dev7_error_0(&self) -> bool {
        *self == DEV7_ERRORR::DEV7_ERROR_0
    }
    #[doc = "Checks if the value of the field is `DEV7_ERROR_1`"]
    #[inline]
    pub fn is_dev7_error_1(&self) -> bool {
        *self == DEV7_ERRORR::DEV7_ERROR_1
    }
}
#[doc = r" Value of the field"]
pub struct RDY_TIMEOUTR {
    bits: u8,
}
impl RDY_TIMEOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct READY_BUSYR {
    bits: u8,
}
impl READY_BUSYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - UNAVAILABLE = 0x0 GPMI is not present in this product"]
    #[inline]
    pub fn present(&self) -> PRESENTR {
        PRESENTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - NOT_FULL = 0x0 FIFO is not full. FULL = 0x1 FIFO is full."]
    #[inline]
    pub fn fifo_full(&self) -> FIFO_FULLR {
        FIFO_FULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - NOT_EMPTY = 0x0 FIFO is not empty. EMPTY = 0x1 FIFO is empty."]
    #[inline]
    pub fn fifo_empty(&self) -> FIFO_EMPTYR {
        FIFO_EMPTYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Buffer Mask Validity bit."]
    #[inline]
    pub fn invalid_buffer_mask(&self) -> INVALID_BUFFER_MASKR {
        INVALID_BUFFER_MASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Status of the ATA_IRQ input pin."]
    #[inline]
    pub fn ata_irq(&self) -> ATA_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ATA_IRQR { bits }
    }
    #[doc = "Bits 5:7 - Always write zeroes to this bit field."]
    #[inline]
    pub fn rsvd1(&self) -> RSVD1R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSVD1R { bits }
    }
    #[doc = "Bit 8 - DMA channel 0 (Timeout or compare failure, depending on COMMAND_MODE)."]
    #[inline]
    pub fn dev0_error(&self) -> DEV0_ERRORR {
        DEV0_ERRORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - DMA channel 1 (Timeout or compare failure, depending on COMMAND_MODE)."]
    #[inline]
    pub fn dev1_error(&self) -> DEV1_ERRORR {
        DEV1_ERRORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - DMA channel 2 (Timeout or compare failure, depending on COMMAND_MODE)."]
    #[inline]
    pub fn dev2_error(&self) -> DEV2_ERRORR {
        DEV2_ERRORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - DMA channel 3 (Timeout or compare failure, depending on COMMAND_MODE)."]
    #[inline]
    pub fn dev3_error(&self) -> DEV3_ERRORR {
        DEV3_ERRORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - DMA channel 4 (Timeout or compare failure, depending on COMMAND_MODE)."]
    #[inline]
    pub fn dev4_error(&self) -> DEV4_ERRORR {
        DEV4_ERRORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - DMA channel 5 (Timeout or compare failure, depending on COMMAND_MODE)."]
    #[inline]
    pub fn dev5_error(&self) -> DEV5_ERRORR {
        DEV5_ERRORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - DMA channel 6 (Timeout or compare failure, depending on COMMAND_MODE)."]
    #[inline]
    pub fn dev6_error(&self) -> DEV6_ERRORR {
        DEV6_ERRORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - DMA channel 7 (Timeout or compare failure, depending on COMMAND_MODE)."]
    #[inline]
    pub fn dev7_error(&self) -> DEV7_ERRORR {
        DEV7_ERRORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:23 - State of the RDY/BUSY Timeout Flags"]
    #[inline]
    pub fn rdy_timeout(&self) -> RDY_TIMEOUTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RDY_TIMEOUTR { bits }
    }
    #[doc = "Bits 24:31 - Read-only view of NAND Ready_Busy Input pins."]
    #[inline]
    pub fn ready_busy(&self) -> READY_BUSYR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        READY_BUSYR { bits }
    }
}
