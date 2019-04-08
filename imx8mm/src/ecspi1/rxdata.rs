#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RXDATA {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct ECSPI_RXDATAR {
    bits: u32,
}
impl ECSPI_RXDATAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Receive Data"]
    #[inline]
    pub fn ecspi_rxdata(&self) -> ECSPI_RXDATAR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        ECSPI_RXDATAR { bits }
    }
}
