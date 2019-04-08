#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::VIR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct NDIDR {
    bits: u8,
}
impl NDIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NMSTRR {
    bits: u8,
}
impl NMSTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NPERR {
    bits: u8,
}
impl NPERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NRGNR {
    bits: u8,
}
impl NRGNR {
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
    #[doc = "Bits 0:3 - Number of Domains"]
    #[inline]
    pub fn ndid(&self) -> NDIDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NDIDR { bits }
    }
    #[doc = "Bits 4:11 - Number of Masters"]
    #[inline]
    pub fn nmstr(&self) -> NMSTRR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NMSTRR { bits }
    }
    #[doc = "Bits 12:19 - Number of Peripherals"]
    #[inline]
    pub fn nper(&self) -> NPERR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NPERR { bits }
    }
    #[doc = "Bits 20:27 - Number of Memory Regions"]
    #[inline]
    pub fn nrgn(&self) -> NRGNR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NRGNR { bits }
    }
}
