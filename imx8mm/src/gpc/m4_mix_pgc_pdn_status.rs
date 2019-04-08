#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::M4_MIX_PGC_PDN_STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct M4_MIX_PGC_PDN_STATUSR {
    bits: bool,
}
impl M4_MIX_PGC_PDN_STATUSR {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - no description available"]
    #[inline]
    pub fn m4_mix_pgc_pdn_status(&self) -> M4_MIX_PGC_PDN_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        M4_MIX_PGC_PDN_STATUSR { bits }
    }
}
