#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DEBUG3 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DEV_WORD_CNTRR {
    bits: u16,
}
impl DEV_WORD_CNTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct APB_WORD_CNTRR {
    bits: u16,
}
impl APB_WORD_CNTRR {
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
    #[doc = "Bits 0:15 - Reflects the number of bytes remains to be transferred on the ATA/Nand bus."]
    #[inline]
    pub fn dev_word_cntr(&self) -> DEV_WORD_CNTRR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DEV_WORD_CNTRR { bits }
    }
    #[doc = "Bits 16:31 - Reflects the number of bytes remains to be transferred on the APB bus."]
    #[inline]
    pub fn apb_word_cntr(&self) -> APB_WORD_CNTRR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        APB_WORD_CNTRR { bits }
    }
}
