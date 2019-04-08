#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DIGPROG {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DIGPROG_MINORR {
    bits: u8,
}
impl DIGPROG_MINORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DIGPROG_MAJOR_LOWERR {
    bits: u8,
}
impl DIGPROG_MAJOR_LOWERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DIGPROG_MAJOR_UPPERR {
    bits: u8,
}
impl DIGPROG_MAJOR_UPPERR {
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
    #[doc = "Bits 0:7 - Bit\\[7:4\\] is the base layer revision, Bit\\[3:0\\] is the metal layer revision 0x10 stands for Tapeout 1"]
    #[inline]
    pub fn digprog_minor(&self) -> DIGPROG_MINORR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DIGPROG_MINORR { bits }
    }
    #[doc = "Bits 8:15 - Bit\\[7:4\\] is 0x4, stands for \"Quad\" Bit\\[3:0\\] is 0x0, stands for \"Mini\""]
    #[inline]
    pub fn digprog_major_lower(&self) -> DIGPROG_MAJOR_LOWERR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DIGPROG_MAJOR_LOWERR { bits }
    }
    #[doc = "Bits 16:23 - Bit\\[7:4\\] is 0x8, stands for \"i.MX8\" Bit\\[3:0\\] is 0x2, stands for \"M\""]
    #[inline]
    pub fn digprog_major_upper(&self) -> DIGPROG_MAJOR_UPPERR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DIGPROG_MAJOR_UPPERR { bits }
    }
}
