#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CH_CMD {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `COMMAND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMMANDR {
    #[doc = "Perform any requested PIO word transfers but terminate command before any DMA transfer."]
    NO_DMA_XFER,
    #[doc = "Perform any requested PIO word transfers and then perform a DMA transfer from the peripheral for the specified number of bytes."]
    DMA_WRITE,
    #[doc = "Perform any requested PIO word transfers and then perform a DMA transfer to the peripheral for the specified number of bytes."]
    DMA_READ,
    #[doc = "Perform any requested PIO word transfers and then perform a conditional branch to the next chained device. Follow the NEXCMD_ADDR pointer if the perpheral sense is true. Follow the BUFFER_ADDRESS as a chain pointer if the peripheral sense line is false."]
    DMA_SENSE,
}
impl COMMANDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            COMMANDR::NO_DMA_XFER => 0,
            COMMANDR::DMA_WRITE => 1,
            COMMANDR::DMA_READ => 2,
            COMMANDR::DMA_SENSE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> COMMANDR {
        match value {
            0 => COMMANDR::NO_DMA_XFER,
            1 => COMMANDR::DMA_WRITE,
            2 => COMMANDR::DMA_READ,
            3 => COMMANDR::DMA_SENSE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_DMA_XFER`"]
    #[inline]
    pub fn is_no_dma_xfer(&self) -> bool {
        *self == COMMANDR::NO_DMA_XFER
    }
    #[doc = "Checks if the value of the field is `DMA_WRITE`"]
    #[inline]
    pub fn is_dma_write(&self) -> bool {
        *self == COMMANDR::DMA_WRITE
    }
    #[doc = "Checks if the value of the field is `DMA_READ`"]
    #[inline]
    pub fn is_dma_read(&self) -> bool {
        *self == COMMANDR::DMA_READ
    }
    #[doc = "Checks if the value of the field is `DMA_SENSE`"]
    #[inline]
    pub fn is_dma_sense(&self) -> bool {
        *self == COMMANDR::DMA_SENSE
    }
}
#[doc = r" Value of the field"]
pub struct CHAINR {
    bits: bool,
}
impl CHAINR {
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
pub struct IRQONCMPLTR {
    bits: bool,
}
impl IRQONCMPLTR {
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
pub struct NANDLOCKR {
    bits: bool,
}
impl NANDLOCKR {
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
pub struct NANDWAIT4READYR {
    bits: bool,
}
impl NANDWAIT4READYR {
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
pub struct SEMAPHORER {
    bits: bool,
}
impl SEMAPHORER {
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
pub struct WAIT4ENDCMDR {
    bits: bool,
}
impl WAIT4ENDCMDR {
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
pub struct HALTONTERMINATER {
    bits: bool,
}
impl HALTONTERMINATER {
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
pub struct CMDWORDSR {
    bits: u8,
}
impl CMDWORDSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct XFER_COUNTR {
    bits: u16,
}
impl XFER_COUNTR {
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
    #[doc = "Bits 0:1 - This bitfield indicates the type of current command:"]
    #[inline]
    pub fn command(&self) -> COMMANDR {
        COMMANDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - A value of one indicates that another command is chained onto the end of the current command structure"]
    #[inline]
    pub fn chain(&self) -> CHAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHAINR { bits }
    }
    #[doc = "Bit 3 - A value of one indicates that the channel will cause the interrupt status bit to be set upon completion of the current command, i"]
    #[inline]
    pub fn irqoncmplt(&self) -> IRQONCMPLTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IRQONCMPLTR { bits }
    }
    #[doc = "Bit 4 - A value of one indicates that the NAND DMA channel will remain \"locked\" in the arbiter at the expense of other NAND DMA channels"]
    #[inline]
    pub fn nandlock(&self) -> NANDLOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NANDLOCKR { bits }
    }
    #[doc = "Bit 5 - A value of one indicates that the NAND DMA channel will will wait until the NAND device reports \"ready\" before executing the command"]
    #[inline]
    pub fn nandwait4ready(&self) -> NANDWAIT4READYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NANDWAIT4READYR { bits }
    }
    #[doc = "Bit 6 - A value of one indicates that the channel will decrement its semaphore at the completion of the current command structure"]
    #[inline]
    pub fn semaphore(&self) -> SEMAPHORER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SEMAPHORER { bits }
    }
    #[doc = "Bit 7 - A value of one indicates that the channel will wait for the end of command signal to be sent from the APBH device to the DMA before starting the next DMA command"]
    #[inline]
    pub fn wait4endcmd(&self) -> WAIT4ENDCMDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WAIT4ENDCMDR { bits }
    }
    #[doc = "Bit 8 - A value of one indicates that the channel will immeditately terminate the current descriptor and halt the DMA channel if a terminate signal is set"]
    #[inline]
    pub fn haltonterminate(&self) -> HALTONTERMINATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HALTONTERMINATER { bits }
    }
    #[doc = "Bits 12:15 - This field indicates the number of command words to send to the GPMI0, starting with the base PIO address of the GPMI0 control register and incrementing from there"]
    #[inline]
    pub fn cmdwords(&self) -> CMDWORDSR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMDWORDSR { bits }
    }
    #[doc = "Bits 16:31 - This field indicates the number of bytes to transfer to or from the appropriate PIO register in the GPMI0 device"]
    #[inline]
    pub fn xfer_count(&self) -> XFER_COUNTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        XFER_COUNTR { bits }
    }
}
