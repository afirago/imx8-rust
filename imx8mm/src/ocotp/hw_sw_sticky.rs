#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HW_SW_STICKY {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct RSVD0R {
    bits: bool,
}
impl RSVD0R {
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
pub struct SRK_REVOKE_LOCKR {
    bits: bool,
}
impl SRK_REVOKE_LOCKR {
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
pub struct FIELD_RETURN_LOCKR {
    bits: bool,
}
impl FIELD_RETURN_LOCKR {
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
pub struct BLOCK_ROM_PARTR {
    bits: bool,
}
impl BLOCK_ROM_PARTR {
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
pub struct JTAG_BLOCK_RELEASER {
    bits: bool,
}
impl JTAG_BLOCK_RELEASER {
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
pub struct DISABLE_READ_GROUP_MASKR {
    bits: bool,
}
impl DISABLE_READ_GROUP_MASKR {
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
pub struct DISABLE_READ_HDMI_FW_SRKR {
    bits: bool,
}
impl DISABLE_READ_HDMI_FW_SRKR {
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
pub struct DISABLE_READ_HDMI_KMEKR {
    bits: bool,
}
impl DISABLE_READ_HDMI_KMEKR {
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
pub struct DISABLE_READ_HDCP_TX_GLOBAL_CONSTANTR {
    bits: bool,
}
impl DISABLE_READ_HDCP_TX_GLOBAL_CONSTANTR {
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
pub struct DISABLE_READ_HDCP_TX_CERTR {
    bits: bool,
}
impl DISABLE_READ_HDCP_TX_CERTR {
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
pub struct DISABLE_READ_HDCP_DEVICE_KEYR {
    bits: bool,
}
impl DISABLE_READ_HDCP_DEVICE_KEYR {
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
pub struct RSVD1R {
    bits: u32,
}
impl RSVD1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SRK_REVOKE_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _SRK_REVOKE_LOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FIELD_RETURN_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _FIELD_RETURN_LOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BLOCK_ROM_PARTW<'a> {
    w: &'a mut W,
}
impl<'a> _BLOCK_ROM_PARTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _JTAG_BLOCK_RELEASEW<'a> {
    w: &'a mut W,
}
impl<'a> _JTAG_BLOCK_RELEASEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DISABLE_READ_GROUP_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLE_READ_GROUP_MASKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DISABLE_READ_HDMI_FW_SRKW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLE_READ_HDMI_FW_SRKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DISABLE_READ_HDMI_KMEKW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLE_READ_HDMI_KMEKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DISABLE_READ_HDCP_TX_GLOBAL_CONSTANTW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLE_READ_HDCP_TX_GLOBAL_CONSTANTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DISABLE_READ_HDCP_TX_CERTW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLE_READ_HDCP_TX_CERTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DISABLE_READ_HDCP_DEVICE_KEYW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLE_READ_HDCP_DEVICE_KEYW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Reserved"]
    #[inline]
    pub fn rsvd0(&self) -> RSVD0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSVD0R { bits }
    }
    #[doc = "Bit 1 - Shadow register write and OTP write lock for SRK_REVOKE region"]
    #[inline]
    pub fn srk_revoke_lock(&self) -> SRK_REVOKE_LOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SRK_REVOKE_LOCKR { bits }
    }
    #[doc = "Bit 2 - Shadow register write and OTP write lock for FIELD_RETURN region"]
    #[inline]
    pub fn field_return_lock(&self) -> FIELD_RETURN_LOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FIELD_RETURN_LOCKR { bits }
    }
    #[doc = "Bit 3 - Set by Arm during Boot after DTCP is initialized and before test mode entry, if ROM_PART_LOCK=1"]
    #[inline]
    pub fn block_rom_part(&self) -> BLOCK_ROM_PARTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BLOCK_ROM_PARTR { bits }
    }
    #[doc = "Bit 4 - Set by Arm during Boot after DTCP is initialized and before test mode entry"]
    #[inline]
    pub fn jtag_block_release(&self) -> JTAG_BLOCK_RELEASER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        JTAG_BLOCK_RELEASER { bits }
    }
    #[doc = "Bit 5 - Shadow register write and OTP write lock for GROUP_MASK region"]
    #[inline]
    pub fn disable_read_group_mask(&self) -> DISABLE_READ_GROUP_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISABLE_READ_GROUP_MASKR { bits }
    }
    #[doc = "Bit 6 - Shadow register write and OTP write lock for HDMI_FW_SRK region"]
    #[inline]
    pub fn disable_read_hdmi_fw_srk(&self) -> DISABLE_READ_HDMI_FW_SRKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISABLE_READ_HDMI_FW_SRKR { bits }
    }
    #[doc = "Bit 7 - Shadow register write and OTP write lock for HDMI_KMEK region"]
    #[inline]
    pub fn disable_read_hdmi_kmek(&self) -> DISABLE_READ_HDMI_KMEKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISABLE_READ_HDMI_KMEKR { bits }
    }
    #[doc = "Bit 8 - Shadow register write and OTP write lock for HDCP_TX_GLOBAL_CONSTANT region"]
    #[inline]
    pub fn disable_read_hdcp_tx_global_constant(&self) -> DISABLE_READ_HDCP_TX_GLOBAL_CONSTANTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISABLE_READ_HDCP_TX_GLOBAL_CONSTANTR { bits }
    }
    #[doc = "Bit 9 - Shadow register write and OTP write lock for HDCP_TX_CERT region"]
    #[inline]
    pub fn disable_read_hdcp_tx_cert(&self) -> DISABLE_READ_HDCP_TX_CERTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISABLE_READ_HDCP_TX_CERTR { bits }
    }
    #[doc = "Bit 10 - Shadow register write and OTP write lock for HDCP_DEVICE_HDCP region"]
    #[inline]
    pub fn disable_read_hdcp_device_key(&self) -> DISABLE_READ_HDCP_DEVICE_KEYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISABLE_READ_HDCP_DEVICE_KEYR { bits }
    }
    #[doc = "Bits 11:31 - Reserved"]
    #[inline]
    pub fn rsvd1(&self) -> RSVD1R {
        let bits = {
            const MASK: u32 = 2097151;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RSVD1R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Shadow register write and OTP write lock for SRK_REVOKE region"]
    #[inline]
    pub fn srk_revoke_lock(&mut self) -> _SRK_REVOKE_LOCKW {
        _SRK_REVOKE_LOCKW { w: self }
    }
    #[doc = "Bit 2 - Shadow register write and OTP write lock for FIELD_RETURN region"]
    #[inline]
    pub fn field_return_lock(&mut self) -> _FIELD_RETURN_LOCKW {
        _FIELD_RETURN_LOCKW { w: self }
    }
    #[doc = "Bit 3 - Set by Arm during Boot after DTCP is initialized and before test mode entry, if ROM_PART_LOCK=1"]
    #[inline]
    pub fn block_rom_part(&mut self) -> _BLOCK_ROM_PARTW {
        _BLOCK_ROM_PARTW { w: self }
    }
    #[doc = "Bit 4 - Set by Arm during Boot after DTCP is initialized and before test mode entry"]
    #[inline]
    pub fn jtag_block_release(&mut self) -> _JTAG_BLOCK_RELEASEW {
        _JTAG_BLOCK_RELEASEW { w: self }
    }
    #[doc = "Bit 5 - Shadow register write and OTP write lock for GROUP_MASK region"]
    #[inline]
    pub fn disable_read_group_mask(&mut self) -> _DISABLE_READ_GROUP_MASKW {
        _DISABLE_READ_GROUP_MASKW { w: self }
    }
    #[doc = "Bit 6 - Shadow register write and OTP write lock for HDMI_FW_SRK region"]
    #[inline]
    pub fn disable_read_hdmi_fw_srk(&mut self) -> _DISABLE_READ_HDMI_FW_SRKW {
        _DISABLE_READ_HDMI_FW_SRKW { w: self }
    }
    #[doc = "Bit 7 - Shadow register write and OTP write lock for HDMI_KMEK region"]
    #[inline]
    pub fn disable_read_hdmi_kmek(&mut self) -> _DISABLE_READ_HDMI_KMEKW {
        _DISABLE_READ_HDMI_KMEKW { w: self }
    }
    #[doc = "Bit 8 - Shadow register write and OTP write lock for HDCP_TX_GLOBAL_CONSTANT region"]
    #[inline]
    pub fn disable_read_hdcp_tx_global_constant(
        &mut self,
    ) -> _DISABLE_READ_HDCP_TX_GLOBAL_CONSTANTW {
        _DISABLE_READ_HDCP_TX_GLOBAL_CONSTANTW { w: self }
    }
    #[doc = "Bit 9 - Shadow register write and OTP write lock for HDCP_TX_CERT region"]
    #[inline]
    pub fn disable_read_hdcp_tx_cert(&mut self) -> _DISABLE_READ_HDCP_TX_CERTW {
        _DISABLE_READ_HDCP_TX_CERTW { w: self }
    }
    #[doc = "Bit 10 - Shadow register write and OTP write lock for HDCP_DEVICE_HDCP region"]
    #[inline]
    pub fn disable_read_hdcp_device_key(&mut self) -> _DISABLE_READ_HDCP_DEVICE_KEYW {
        _DISABLE_READ_HDCP_DEVICE_KEYW { w: self }
    }
}
