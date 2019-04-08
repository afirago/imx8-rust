#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HW_LOCK {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct TESTERR {
    bits: u8,
}
impl TESTERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BOOT_CFGR {
    bits: u8,
}
impl BOOT_CFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MEM_TRIMR {
    bits: u8,
}
impl MEM_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ANALOGR {
    bits: u8,
}
impl ANALOGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OTPMKR {
    bits: bool,
}
impl OTPMKR {
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
pub struct SRKR {
    bits: bool,
}
impl SRKR {
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
pub struct SJC_RESPR {
    bits: bool,
}
impl SJC_RESPR {
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
pub struct GROUP_MASKR {
    bits: bool,
}
impl GROUP_MASKR {
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
pub struct USB_IDR {
    bits: u8,
}
impl USB_IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MAC_ADDRR {
    bits: u8,
}
impl MAC_ADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MAU_KEYR {
    bits: bool,
}
impl MAU_KEYR {
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
pub struct ROM_PATCHR {
    bits: bool,
}
impl ROM_PATCHR {
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
pub struct GP_CRCR {
    bits: u8,
}
impl GP_CRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GP1R {
    bits: u8,
}
impl GP1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GP2R {
    bits: u8,
}
impl GP2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HDMI_KEYR {
    bits: u8,
}
impl HDMI_KEYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HDMI_CRCR {
    bits: u8,
}
impl HDMI_CRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HDCP_KEYR {
    bits: u8,
}
impl HDCP_KEYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HDCP_CRCR {
    bits: u8,
}
impl HDCP_CRCR {
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
    #[doc = "Bits 0:1 - Status of shadow register and OTP write lock for tester region"]
    #[inline]
    pub fn tester(&self) -> TESTERR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TESTERR { bits }
    }
    #[doc = "Bits 2:3 - Status of shadow register and OTP write lock for boot_cfg region"]
    #[inline]
    pub fn boot_cfg(&self) -> BOOT_CFGR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BOOT_CFGR { bits }
    }
    #[doc = "Bits 4:5 - Status of shadow register and OTP write lock for mem_trim region"]
    #[inline]
    pub fn mem_trim(&self) -> MEM_TRIMR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MEM_TRIMR { bits }
    }
    #[doc = "Bits 6:7 - Status of shadow register and OTP write lock for analog region"]
    #[inline]
    pub fn analog(&self) -> ANALOGR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ANALOGR { bits }
    }
    #[doc = "Bit 8 - Status of shadow register read and write, OTP read and write lock for otpmk region"]
    #[inline]
    pub fn otpmk(&self) -> OTPMKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OTPMKR { bits }
    }
    #[doc = "Bit 9 - Status of shadow register and OTP write lock for srk region"]
    #[inline]
    pub fn srk(&self) -> SRKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SRKR { bits }
    }
    #[doc = "Bit 10 - Status of shadow register read and write, OTP read and write lock for sjc_resp region"]
    #[inline]
    pub fn sjc_resp(&self) -> SJC_RESPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SJC_RESPR { bits }
    }
    #[doc = "Bit 11 - Status of shadow register and OTP write lock for group mask region"]
    #[inline]
    pub fn group_mask(&self) -> GROUP_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GROUP_MASKR { bits }
    }
    #[doc = "Bits 12:13 - Status of shadow register and OTP write lock for usb_id region"]
    #[inline]
    pub fn usb_id(&self) -> USB_IDR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USB_IDR { bits }
    }
    #[doc = "Bits 14:15 - Status of shadow register and OTP write lock for mac_addr region"]
    #[inline]
    pub fn mac_addr(&self) -> MAC_ADDRR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAC_ADDRR { bits }
    }
    #[doc = "Bit 16 - Status of shadow register read and write, OTP read and write lock for manufacture_key region"]
    #[inline]
    pub fn mau_key(&self) -> MAU_KEYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MAU_KEYR { bits }
    }
    #[doc = "Bit 17 - Status of shadow register and OTP write lock for rom_patch region"]
    #[inline]
    pub fn rom_patch(&self) -> ROM_PATCHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ROM_PATCHR { bits }
    }
    #[doc = "Bits 18:19 - Status of shadow register and OTP write lock for gp_crc region"]
    #[inline]
    pub fn gp_crc(&self) -> GP_CRCR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GP_CRCR { bits }
    }
    #[doc = "Bits 20:21 - Status of shadow register and OTP write lock for gp1 region"]
    #[inline]
    pub fn gp1(&self) -> GP1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GP1R { bits }
    }
    #[doc = "Bits 22:23 - Status of shadow register and OTP write lock for gp2 region"]
    #[inline]
    pub fn gp2(&self) -> GP2R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GP2R { bits }
    }
    #[doc = "Bits 24:25 - Status of shadow register write and read, OTP write and read lock for hdmi_key region"]
    #[inline]
    pub fn hdmi_key(&self) -> HDMI_KEYR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HDMI_KEYR { bits }
    }
    #[doc = "Bits 26:27 - Status of shadow register write and read, OTP write and read lock for hdcp_crc region"]
    #[inline]
    pub fn hdmi_crc(&self) -> HDMI_CRCR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HDMI_CRCR { bits }
    }
    #[doc = "Bits 28:29 - Status of shadow register write and read, OTP write and read lock for hdcp_key region"]
    #[inline]
    pub fn hdcp_key(&self) -> HDCP_KEYR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HDCP_KEYR { bits }
    }
    #[doc = "Bits 30:31 - Status of shadow register write and read, OTP write and read lock for hdcp_crc region"]
    #[inline]
    pub fn hdcp_crc(&self) -> HDCP_CRCR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HDCP_CRCR { bits }
    }
}
