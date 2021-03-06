#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ISR3_A53 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct ISR3_A53R {
    bits: u32,
}
impl ISR3_A53R {
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
    #[doc = "Bits 0:31 - A53 IRQ\\[95:64\\] status"]
    #[inline]
    pub fn isr3_a53(&self) -> ISR3_A53R {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        ISR3_A53R { bits }
    }
}
