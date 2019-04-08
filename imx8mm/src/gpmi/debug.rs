#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DEBUG {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CMD_ENDR {
    bits: u8,
}
impl CMD_ENDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DMAREQR {
    bits: u8,
}
impl DMAREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DMA_SENSER {
    bits: u8,
}
impl DMA_SENSER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WAIT_FOR_READY_ENDR {
    bits: u8,
}
impl WAIT_FOR_READY_ENDR {
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
    #[doc = "Bits 0:7 - Read Only view of the Command End toggle signals to DMA. One per channel"]
    #[inline]
    pub fn cmd_end(&self) -> CMD_ENDR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMD_ENDR { bits }
    }
    #[doc = "Bits 8:15 - Read-only view of DMA request line for 8 DMA channels"]
    #[inline]
    pub fn dmareq(&self) -> DMAREQR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DMAREQR { bits }
    }
    #[doc = "Bits 16:23 - Read-only view of sense state of the 8 DMA channels"]
    #[inline]
    pub fn dma_sense(&self) -> DMA_SENSER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DMA_SENSER { bits }
    }
    #[doc = "Bits 24:31 - Read Only view of the Wait_For_Ready End toggle signals to DMA. One per channel"]
    #[inline]
    pub fn wait_for_ready_end(&self) -> WAIT_FOR_READY_ENDR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WAIT_FOR_READY_ENDR { bits }
    }
}
