#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SBMR2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SEC_CONFIGR {
    bits: u8,
}
impl SEC_CONFIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DIR_BT_DISR {
    bits: bool,
}
impl DIR_BT_DISR {
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
pub struct BT_FUSE_SELR {
    bits: bool,
}
impl BT_FUSE_SELR {
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
pub struct FORCE_COLD_BOOTR {
    bits: u8,
}
impl FORCE_COLD_BOOTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IPP_BOOT_MODER {
    bits: u8,
}
impl IPP_BOOT_MODER {
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
    #[doc = "Bits 0:1 - SEC_CONFIG\\[1\\] shows the state of the SEC_CONFIG\\[1\\] fuse and SEC_CONFIG\\[0\\] shows the state of the SEC_CONFIG\\[0\\] fuse"]
    #[inline]
    pub fn sec_config(&self) -> SEC_CONFIGR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SEC_CONFIGR { bits }
    }
    #[doc = "Bit 3 - DIR_BT_DIS shows the state of the DIR_BT_DIS fuse"]
    #[inline]
    pub fn dir_bt_dis(&self) -> DIR_BT_DISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIR_BT_DISR { bits }
    }
    #[doc = "Bit 4 - BT_FUSE_SEL (connected to gpio bt_fuse_sel) shows the state of the BT_FUSE_SEL fuse"]
    #[inline]
    pub fn bt_fuse_sel(&self) -> BT_FUSE_SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BT_FUSE_SELR { bits }
    }
    #[doc = "Bits 5:7 - See Fusemap for additional information."]
    #[inline]
    pub fn force_cold_boot(&self) -> FORCE_COLD_BOOTR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FORCE_COLD_BOOTR { bits }
    }
    #[doc = "Bits 24:25 - IPP_BOOT_MODE shows the latched state of the BOOT_MODE1 and BOOT_MODE0 signals on the rising edge of POR_B"]
    #[inline]
    pub fn ipp_boot_mode(&self) -> IPP_BOOT_MODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IPP_BOOT_MODER { bits }
    }
}
