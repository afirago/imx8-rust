#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTP Controller Control Register"]
    pub hw_ctrl: HW_CTRL,
    #[doc = "0x04 - OTP Controller Control Register"]
    pub hw_ctrl_set: HW_CTRL_SET,
    #[doc = "0x08 - OTP Controller Control Register"]
    pub hw_ctrl_clr: HW_CTRL_CLR,
    #[doc = "0x0c - OTP Controller Control Register"]
    pub hw_ctrl_tog: HW_CTRL_TOG,
    #[doc = "0x10 - OTP Controller Timing Register"]
    pub hw_timing: HW_TIMING,
    _reserved0: [u8; 12usize],
    #[doc = "0x20 - OTP Controller Write Data Register"]
    pub hw_data: HW_DATA,
    _reserved1: [u8; 12usize],
    #[doc = "0x30 - OTP Controller Write Data Register"]
    pub hw_read_ctrl: HW_READ_CTRL,
    _reserved2: [u8; 12usize],
    #[doc = "0x40 - OTP Controller Read Data Register"]
    pub hw_read_fuse_data: HW_READ_FUSE_DATA,
    _reserved3: [u8; 12usize],
    #[doc = "0x50 - Sticky bit Register"]
    pub hw_sw_sticky: HW_SW_STICKY,
    _reserved4: [u8; 12usize],
    #[doc = "0x60 - Software Controllable Signals Register"]
    pub hw_scs: HW_SCS,
    #[doc = "0x64 - Software Controllable Signals Register"]
    pub hw_scs_set: HW_SCS_SET,
    #[doc = "0x68 - Software Controllable Signals Register"]
    pub hw_scs_clr: HW_SCS_CLR,
    #[doc = "0x6c - Software Controllable Signals Register"]
    pub hw_scs_tog: HW_SCS_TOG,
    _reserved5: [u8; 32usize],
    #[doc = "0x90 - OTP Controller Version Register"]
    pub hw_version: HW_VERSION,
    _reserved6: [u8; 876usize],
    #[doc = "0x400 - Value of OTP Bank0 Word0 (Lock controls)"]
    pub hw_lock: HW_LOCK,
    _reserved7: [u8; 12usize],
    #[doc = "0x410 - Value of OTP Bank0 Word1 (Tester Info.)"]
    pub hw_tester0: HW_TESTER0,
    _reserved8: [u8; 12usize],
    #[doc = "0x420 - Value of OTP Bank0 Word2 (tester Info.)"]
    pub hw_tester1: HW_TESTER1,
    _reserved9: [u8; 12usize],
    #[doc = "0x430 - Value of OTP Bank0 Word3 (Tester Info.)"]
    pub hw_tester2: HW_TESTER2,
    _reserved10: [u8; 12usize],
    #[doc = "0x440 - Value of OTP Bank1 Word0 (Tester Info.)"]
    pub hw_tester3: HW_TESTER3,
    _reserved11: [u8; 12usize],
    #[doc = "0x450 - Value of OTP Bank1 Word1 (Tester Info.)"]
    pub hw_tester4: HW_TESTER4,
    _reserved12: [u8; 12usize],
    #[doc = "0x460 - Value of OTP Bank1 Word2 (Tester Info.)"]
    pub hw_tester5: HW_TESTER5,
    _reserved13: [u8; 12usize],
    #[doc = "0x470 - Value of OTP Bank1 Word3 (Boot Configuration Info.)"]
    pub hw_boot_cfg0: HW_BOOT_CFG0,
    _reserved14: [u8; 12usize],
    #[doc = "0x480 - Value of OTP Bank2 Word0 (Boot Configuration Info.)"]
    pub hw_boot_cfg1: HW_BOOT_CFG1,
    _reserved15: [u8; 12usize],
    #[doc = "0x490 - Value of OTP Bank2 Word1 (Boot Configuration Info.)"]
    pub hw_boot_cfg2: HW_BOOT_CFG2,
    _reserved16: [u8; 12usize],
    #[doc = "0x4a0 - Value of OTP Bank2 Word2 (Boot Configuration Info.)"]
    pub hw_boot_cfg3: HW_BOOT_CFG3,
    _reserved17: [u8; 12usize],
    #[doc = "0x4b0 - Value of OTP Bank2 Word3 (BOOT Configuration Info.)"]
    pub hw_boot_cfg4: HW_BOOT_CFG4,
    _reserved18: [u8; 12usize],
    #[doc = "0x4c0 - Value of OTP Bank3 Word0 (Memory Related Info.)"]
    pub hw_mem_trim0: HW_MEM_TRIM0,
    _reserved19: [u8; 12usize],
    #[doc = "0x4d0 - Value of OTP Bank3 Word1 (Memory Related Info.)"]
    pub hw_mem_trim1: HW_MEM_TRIM1,
    _reserved20: [u8; 12usize],
    #[doc = "0x4e0 - Value of OTP Bank3 Word2 (Analog Info.)"]
    pub hw_ana0: HW_ANA0,
    _reserved21: [u8; 12usize],
    #[doc = "0x4f0 - Value of OTP Bank3 Word3 (Analog Info.)"]
    pub hw_ana1: HW_ANA1,
    _reserved22: [u8; 140usize],
    #[doc = "0x580 - Shadow Register for OTP Bank6 Word0 (SRK Hash)"]
    pub hw_srk0: HW_SRK0,
    _reserved23: [u8; 12usize],
    #[doc = "0x590 - Shadow Register for OTP Bank6 Word1 (SRK Hash)"]
    pub hw_srk1: HW_SRK1,
    _reserved24: [u8; 12usize],
    #[doc = "0x5a0 - Shadow Register for OTP Bank6 Word2 (SRK Hash)"]
    pub hw_srk2: HW_SRK2,
    _reserved25: [u8; 12usize],
    #[doc = "0x5b0 - Shadow Register for OTP Bank6 Word3 (SRK Hash)"]
    pub hw_srk3: HW_SRK3,
    _reserved26: [u8; 12usize],
    #[doc = "0x5c0 - Shadow Register for OTP Bank7 Word0 (SRK Hash)"]
    pub hw_srk4: HW_SRK4,
    _reserved27: [u8; 12usize],
    #[doc = "0x5d0 - Shadow Register for OTP Bank7 Word1 (SRK Hash)"]
    pub hw_srk5: HW_SRK5,
    _reserved28: [u8; 12usize],
    #[doc = "0x5e0 - Shadow Register for OTP Bank7 Word2 (SRK Hash)"]
    pub hw_srk6: HW_SRK6,
    _reserved29: [u8; 12usize],
    #[doc = "0x5f0 - Shadow Register for OTP Bank7 Word3 (SRK Hash)"]
    pub hw_srk7: HW_SRK7,
    _reserved30: [u8; 12usize],
    #[doc = "0x600 - Value of OTP Bank8 Word0 (Secure JTAG Response Field)"]
    pub hw_sjc_resp0: HW_SJC_RESP0,
    _reserved31: [u8; 12usize],
    #[doc = "0x610 - Value of OTP Bank8 Word1 (Secure JTAG Response Field)"]
    pub hw_sjc_resp1: HW_SJC_RESP1,
    _reserved32: [u8; 12usize],
    #[doc = "0x620 - Value of OTP Bank8 Word2 (USB ID info)"]
    pub hw_usb_id: HW_USB_ID,
    _reserved33: [u8; 12usize],
    #[doc = "0x630 - Value of OTP Bank5 Word6 (Field Return)"]
    pub hw_field_return: HW_FIELD_RETURN,
    _reserved34: [u8; 12usize],
    #[doc = "0x640 - Value of OTP Bank9 Word0 (MAC Address)"]
    pub hw_mac_addr0: HW_MAC_ADDR0,
    _reserved35: [u8; 12usize],
    #[doc = "0x650 - Value of OTP Bank9 Word1 (MAC Address)"]
    pub hw_mac_addr1: HW_MAC_ADDR1,
    _reserved36: [u8; 12usize],
    #[doc = "0x660 - Value of OTP Bank9 Word2 (MAC Address)"]
    pub hw_mac_addr2: HW_MAC_ADDR2,
    _reserved37: [u8; 12usize],
    #[doc = "0x670 - Value of OTP Bank9 Word3 (SRK Revoke)"]
    pub hw_srk_revoke: HW_SRK_REVOKE,
    _reserved38: [u8; 12usize],
    #[doc = "0x680 - Shadow Register for OTP Bank10 Word0 (MAU Key)"]
    pub hw_mau_key0: HW_MAU_KEY0,
    _reserved39: [u8; 12usize],
    #[doc = "0x690 - Shadow Register for OTP Bank10 Word1 (MAU Key)"]
    pub hw_mau_key1: HW_MAU_KEY1,
    _reserved40: [u8; 12usize],
    #[doc = "0x6a0 - Shadow Register for OTP Bank10 Word2 (MAU Key)"]
    pub hw_mau_key2: HW_MAU_KEY2,
    _reserved41: [u8; 12usize],
    #[doc = "0x6b0 - Shadow Register for OTP Bank10 Word3 (MAU Key)"]
    pub hw_mau_key3: HW_MAU_KEY3,
    _reserved42: [u8; 12usize],
    #[doc = "0x6c0 - Shadow Register for OTP Bank11 Word0 (MAU Key)"]
    pub hw_mau_key4: HW_MAU_KEY4,
    _reserved43: [u8; 12usize],
    #[doc = "0x6d0 - Shadow Register for OTP Bank11 Word1 (MAU Key)"]
    pub hw_mau_key5: HW_MAU_KEY5,
    _reserved44: [u8; 12usize],
    #[doc = "0x6e0 - Shadow Register for OTP Bank11 Word2 (MAU Key)"]
    pub hw_mau_key6: HW_MAU_KEY6,
    _reserved45: [u8; 12usize],
    #[doc = "0x6f0 - Shadow Register for OTP Bank11 Word3 (MAU Key)"]
    pub hw_mau_key7: HW_MAU_KEY7,
    _reserved46: [u8; 140usize],
    #[doc = "0x780 - Value of OTP Bank14 Word0 ()"]
    pub hw_gp10: HW_GP10,
    _reserved47: [u8; 12usize],
    #[doc = "0x790 - Value of OTP Bank14 Word1 ()"]
    pub hw_gp11: HW_GP11,
    _reserved48: [u8; 12usize],
    #[doc = "0x7a0 - Value of OTP Bank14 Word2 ()"]
    pub hw_gp20: HW_GP20,
    _reserved49: [u8; 12usize],
    #[doc = "0x7b0 - Value of OTP Bank14 Word3 ()"]
    pub hw_gp21: HW_GP21,
    _reserved50: [u8; 12usize],
    #[doc = "0x7c0 - Value of OTP Bank15 Word0 (CRC Key)"]
    pub hw_gp_crc0: HW_GP_CRC0,
    _reserved51: [u8; 12usize],
    #[doc = "0x7d0 - Value of OTP Bank15 Word1 (CRC Key)"]
    pub hw_gp_crc1: HW_GP_CRC1,
    _reserved52: [u8; 12usize],
    #[doc = "0x7e0 - Value of OTP Bank15 Word2 (CRC Key)"]
    pub hw_gp_crc2: HW_GP_CRC2,
    _reserved53: [u8; 12usize],
    #[doc = "0x7f0 - Value of OTP Bank15 Word3 (CRC Key)"]
    pub hw_group_mask: HW_GROUP_MASK,
    _reserved54: [u8; 12usize],
    #[doc = "0x800 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdmi_fw_srk0: HW_HDMI_FW_SRK0,
    _reserved55: [u8; 12usize],
    #[doc = "0x810 - Value of OTP Bank16 Word1 (HDCP Key)"]
    pub hw_hdmi_fw_srk1: HW_HDMI_FW_SRK1,
    _reserved56: [u8; 12usize],
    #[doc = "0x820 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdmi_fw_srk2: HW_HDMI_FW_SRK2,
    _reserved57: [u8; 12usize],
    #[doc = "0x830 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdmi_fw_srk3: HW_HDMI_FW_SRK3,
    _reserved58: [u8; 12usize],
    #[doc = "0x840 - Value of OTP Bank17 Word0 (HDCP Key)"]
    pub hw_hdmi_fw_srk4: HW_HDMI_FW_SRK4,
    _reserved59: [u8; 12usize],
    #[doc = "0x850 - Value of OTP Bank17 Word1 (HDCP Key)"]
    pub hw_hdmi_fw_srk5: HW_HDMI_FW_SRK5,
    _reserved60: [u8; 12usize],
    #[doc = "0x860 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdmi_fw_srk6: HW_HDMI_FW_SRK6,
    _reserved61: [u8; 12usize],
    #[doc = "0x870 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdmi_fw_srk7: HW_HDMI_FW_SRK7,
    _reserved62: [u8; 12usize],
    #[doc = "0x880 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdmi_kmek0: HW_HDMI_KMEK0,
    _reserved63: [u8; 12usize],
    #[doc = "0x890 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdmi_kmek1: HW_HDMI_KMEK1,
    _reserved64: [u8; 12usize],
    #[doc = "0x8a0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdmi_kmek2: HW_HDMI_KMEK2,
    _reserved65: [u8; 12usize],
    #[doc = "0x8b0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdmi_kmek3: HW_HDMI_KMEK3,
    _reserved66: [u8; 76usize],
    #[doc = "0x900 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cons0: HW_HDCP_TX_CONS0,
    _reserved67: [u8; 12usize],
    #[doc = "0x910 - Value of OTP Bank16 Word1 (HDCP Key)"]
    pub hw_hdcp_tx_cons1: HW_HDCP_TX_CONS1,
    _reserved68: [u8; 12usize],
    #[doc = "0x920 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cons2: HW_HDCP_TX_CONS2,
    _reserved69: [u8; 12usize],
    #[doc = "0x930 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cons3: HW_HDCP_TX_CONS3,
    _reserved70: [u8; 12usize],
    #[doc = "0x940 - Value of OTP Bank17 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert0: HW_HDCP_TX_CERT0,
    _reserved71: [u8; 12usize],
    #[doc = "0x950 - Value of OTP Bank17 Word1 (HDCP Key)"]
    pub hw_hdcp_tx_cert1: HW_HDCP_TX_CERT1,
    _reserved72: [u8; 12usize],
    #[doc = "0x960 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert2: HW_HDCP_TX_CERT2,
    _reserved73: [u8; 12usize],
    #[doc = "0x970 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert3: HW_HDCP_TX_CERT3,
    _reserved74: [u8; 12usize],
    #[doc = "0x980 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert4: HW_HDCP_TX_CERT4,
    _reserved75: [u8; 12usize],
    #[doc = "0x990 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert5: HW_HDCP_TX_CERT5,
    _reserved76: [u8; 12usize],
    #[doc = "0x9a0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert6: HW_HDCP_TX_CERT6,
    _reserved77: [u8; 12usize],
    #[doc = "0x9b0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert7: HW_HDCP_TX_CERT7,
    _reserved78: [u8; 12usize],
    #[doc = "0x9c0 - Value of OTP Bank17 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert8: HW_HDCP_TX_CERT8,
    _reserved79: [u8; 12usize],
    #[doc = "0x9d0 - Value of OTP Bank17 Word1 (HDCP Key)"]
    pub hw_hdcp_tx_cert9: HW_HDCP_TX_CERT9,
    _reserved80: [u8; 12usize],
    #[doc = "0x9e0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert10: HW_HDCP_TX_CERT10,
    _reserved81: [u8; 12usize],
    #[doc = "0x9f0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert11: HW_HDCP_TX_CERT11,
    _reserved82: [u8; 12usize],
    #[doc = "0xa00 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert12: HW_HDCP_TX_CERT12,
    _reserved83: [u8; 12usize],
    #[doc = "0xa10 - Value of OTP Bank16 Word1 (HDCP Key)"]
    pub hw_hdcp_tx_cert13: HW_HDCP_TX_CERT13,
    _reserved84: [u8; 12usize],
    #[doc = "0xa20 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert14: HW_HDCP_TX_CERT14,
    _reserved85: [u8; 12usize],
    #[doc = "0xa30 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert15: HW_HDCP_TX_CERT15,
    _reserved86: [u8; 12usize],
    #[doc = "0xa40 - Value of OTP Bank17 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert16: HW_HDCP_TX_CERT16,
    _reserved87: [u8; 12usize],
    #[doc = "0xa50 - Value of OTP Bank17 Word1 (HDCP Key)"]
    pub hw_hdcp_tx_cert17: HW_HDCP_TX_CERT17,
    _reserved88: [u8; 12usize],
    #[doc = "0xa60 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert18: HW_HDCP_TX_CERT18,
    _reserved89: [u8; 12usize],
    #[doc = "0xa70 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert19: HW_HDCP_TX_CERT19,
    _reserved90: [u8; 12usize],
    #[doc = "0xa80 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert20: HW_HDCP_TX_CERT20,
    _reserved91: [u8; 12usize],
    #[doc = "0xa90 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert21: HW_HDCP_TX_CERT21,
    _reserved92: [u8; 12usize],
    #[doc = "0xaa0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert22: HW_HDCP_TX_CERT22,
    _reserved93: [u8; 12usize],
    #[doc = "0xab0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert23: HW_HDCP_TX_CERT23,
    _reserved94: [u8; 12usize],
    #[doc = "0xac0 - Value of OTP Bank17 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert24: HW_HDCP_TX_CERT24,
    _reserved95: [u8; 12usize],
    #[doc = "0xad0 - Value of OTP Bank17 Word1 (HDCP Key)"]
    pub hw_hdcp_tx_cert25: HW_HDCP_TX_CERT25,
    _reserved96: [u8; 12usize],
    #[doc = "0xae0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert26: HW_HDCP_TX_CERT26,
    _reserved97: [u8; 12usize],
    #[doc = "0xaf0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert27: HW_HDCP_TX_CERT27,
    _reserved98: [u8; 12usize],
    #[doc = "0xb00 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert28: HW_HDCP_TX_CERT28,
    _reserved99: [u8; 12usize],
    #[doc = "0xb10 - Value of OTP Bank16 Word1 (HDCP Key)"]
    pub hw_hdcp_tx_cert29: HW_HDCP_TX_CERT29,
    _reserved100: [u8; 12usize],
    #[doc = "0xb20 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert30: HW_HDCP_TX_CERT30,
    _reserved101: [u8; 12usize],
    #[doc = "0xb30 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert31: HW_HDCP_TX_CERT31,
    _reserved102: [u8; 12usize],
    #[doc = "0xb40 - Value of OTP Bank17 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert32: HW_HDCP_TX_CERT32,
    _reserved103: [u8; 12usize],
    #[doc = "0xb50 - Value of OTP Bank17 Word1 (HDCP Key)"]
    pub hw_hdcp_tx_cert33: HW_HDCP_TX_CERT33,
    _reserved104: [u8; 12usize],
    #[doc = "0xb60 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert34: HW_HDCP_TX_CERT34,
    _reserved105: [u8; 12usize],
    #[doc = "0xb70 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert35: HW_HDCP_TX_CERT35,
    _reserved106: [u8; 12usize],
    #[doc = "0xb80 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert36: HW_HDCP_TX_CERT36,
    _reserved107: [u8; 12usize],
    #[doc = "0xb90 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert37: HW_HDCP_TX_CERT37,
    _reserved108: [u8; 12usize],
    #[doc = "0xba0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert38: HW_HDCP_TX_CERT38,
    _reserved109: [u8; 12usize],
    #[doc = "0xbb0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert39: HW_HDCP_TX_CERT39,
    _reserved110: [u8; 12usize],
    #[doc = "0xbc0 - Value of OTP Bank17 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert40: HW_HDCP_TX_CERT40,
    _reserved111: [u8; 12usize],
    #[doc = "0xbd0 - Value of OTP Bank17 Word1 (HDCP Key)"]
    pub hw_hdcp_tx_cert41: HW_HDCP_TX_CERT41,
    _reserved112: [u8; 12usize],
    #[doc = "0xbe0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert42: HW_HDCP_TX_CERT42,
    _reserved113: [u8; 12usize],
    #[doc = "0xbf0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert43: HW_HDCP_TX_CERT43,
    _reserved114: [u8; 12usize],
    #[doc = "0xc00 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert44: HW_HDCP_TX_CERT44,
    _reserved115: [u8; 12usize],
    #[doc = "0xc10 - Value of OTP Bank16 Word1 (HDCP Key)"]
    pub hw_hdcp_tx_cert45: HW_HDCP_TX_CERT45,
    _reserved116: [u8; 12usize],
    #[doc = "0xc20 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert46: HW_HDCP_TX_CERT46,
    _reserved117: [u8; 12usize],
    #[doc = "0xc30 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert47: HW_HDCP_TX_CERT47,
    _reserved118: [u8; 12usize],
    #[doc = "0xc40 - Value of OTP Bank17 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert48: HW_HDCP_TX_CERT48,
    _reserved119: [u8; 12usize],
    #[doc = "0xc50 - Value of OTP Bank17 Word1 (HDCP Key)"]
    pub hw_hdcp_tx_cert49: HW_HDCP_TX_CERT49,
    _reserved120: [u8; 12usize],
    #[doc = "0xc60 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert50: HW_HDCP_TX_CERT50,
    _reserved121: [u8; 12usize],
    #[doc = "0xc70 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert51: HW_HDCP_TX_CERT51,
    _reserved122: [u8; 12usize],
    #[doc = "0xc80 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert52: HW_HDCP_TX_CERT52,
    _reserved123: [u8; 12usize],
    #[doc = "0xc90 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert53: HW_HDCP_TX_CERT53,
    _reserved124: [u8; 12usize],
    #[doc = "0xca0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert54: HW_HDCP_TX_CERT54,
    _reserved125: [u8; 12usize],
    #[doc = "0xcb0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert55: HW_HDCP_TX_CERT55,
    _reserved126: [u8; 12usize],
    #[doc = "0xcc0 - Value of OTP Bank17 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert56: HW_HDCP_TX_CERT56,
    _reserved127: [u8; 12usize],
    #[doc = "0xcd0 - Value of OTP Bank17 Word1 (HDCP Key)"]
    pub hw_hdcp_tx_cert57: HW_HDCP_TX_CERT57,
    _reserved128: [u8; 12usize],
    #[doc = "0xce0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert58: HW_HDCP_TX_CERT58,
    _reserved129: [u8; 12usize],
    #[doc = "0xcf0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert59: HW_HDCP_TX_CERT59,
    _reserved130: [u8; 12usize],
    #[doc = "0xd00 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert60: HW_HDCP_TX_CERT60,
    _reserved131: [u8; 12usize],
    #[doc = "0xd10 - Value of OTP Bank16 Word1 (HDCP Key)"]
    pub hw_hdcp_tx_cert61: HW_HDCP_TX_CERT61,
    _reserved132: [u8; 12usize],
    #[doc = "0xd20 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert62: HW_HDCP_TX_CERT62,
    _reserved133: [u8; 12usize],
    #[doc = "0xd30 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert63: HW_HDCP_TX_CERT63,
    _reserved134: [u8; 12usize],
    #[doc = "0xd40 - Value of OTP Bank17 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert64: HW_HDCP_TX_CERT64,
    _reserved135: [u8; 12usize],
    #[doc = "0xd50 - Value of OTP Bank17 Word1 (HDCP Key)"]
    pub hw_hdcp_tx_cert65: HW_HDCP_TX_CERT65,
    _reserved136: [u8; 12usize],
    #[doc = "0xd60 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert66: HW_HDCP_TX_CERT66,
    _reserved137: [u8; 12usize],
    #[doc = "0xd70 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert67: HW_HDCP_TX_CERT67,
    _reserved138: [u8; 12usize],
    #[doc = "0xd80 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert68: HW_HDCP_TX_CERT68,
    _reserved139: [u8; 12usize],
    #[doc = "0xd90 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert69: HW_HDCP_TX_CERT69,
    _reserved140: [u8; 12usize],
    #[doc = "0xda0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert70: HW_HDCP_TX_CERT70,
    _reserved141: [u8; 12usize],
    #[doc = "0xdb0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert71: HW_HDCP_TX_CERT71,
    _reserved142: [u8; 12usize],
    #[doc = "0xdc0 - Value of OTP Bank17 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert72: HW_HDCP_TX_CERT72,
    _reserved143: [u8; 12usize],
    #[doc = "0xdd0 - Value of OTP Bank17 Word1 (HDCP Key)"]
    pub hw_hdcp_tx_cert73: HW_HDCP_TX_CERT73,
    _reserved144: [u8; 12usize],
    #[doc = "0xde0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert74: HW_HDCP_TX_CERT74,
    _reserved145: [u8; 12usize],
    #[doc = "0xdf0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert75: HW_HDCP_TX_CERT75,
    _reserved146: [u8; 12usize],
    #[doc = "0xe00 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert76: HW_HDCP_TX_CERT76,
    _reserved147: [u8; 12usize],
    #[doc = "0xe10 - Value of OTP Bank16 Word1 (HDCP Key)"]
    pub hw_hdcp_tx_cert77: HW_HDCP_TX_CERT77,
    _reserved148: [u8; 12usize],
    #[doc = "0xe20 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert78: HW_HDCP_TX_CERT78,
    _reserved149: [u8; 12usize],
    #[doc = "0xe30 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert79: HW_HDCP_TX_CERT79,
    _reserved150: [u8; 12usize],
    #[doc = "0xe40 - Value of OTP Bank17 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert80: HW_HDCP_TX_CERT80,
    _reserved151: [u8; 12usize],
    #[doc = "0xe50 - Value of OTP Bank17 Word1 (HDCP Key)"]
    pub hw_hdcp_tx_cert81: HW_HDCP_TX_CERT81,
    _reserved152: [u8; 12usize],
    #[doc = "0xe60 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert82: HW_HDCP_TX_CERT82,
    _reserved153: [u8; 12usize],
    #[doc = "0xe70 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert83: HW_HDCP_TX_CERT83,
    _reserved154: [u8; 12usize],
    #[doc = "0xe80 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert84: HW_HDCP_TX_CERT84,
    _reserved155: [u8; 12usize],
    #[doc = "0xe90 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert85: HW_HDCP_TX_CERT85,
    _reserved156: [u8; 12usize],
    #[doc = "0xea0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert86: HW_HDCP_TX_CERT86,
    _reserved157: [u8; 12usize],
    #[doc = "0xeb0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert87: HW_HDCP_TX_CERT87,
    _reserved158: [u8; 12usize],
    #[doc = "0xec0 - Value of OTP Bank17 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert88: HW_HDCP_TX_CERT88,
    _reserved159: [u8; 12usize],
    #[doc = "0xed0 - Value of OTP Bank17 Word1 (HDCP Key)"]
    pub hw_hdcp_tx_cert89: HW_HDCP_TX_CERT89,
    _reserved160: [u8; 12usize],
    #[doc = "0xee0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert90: HW_HDCP_TX_CERT90,
    _reserved161: [u8; 12usize],
    #[doc = "0xef0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert91: HW_HDCP_TX_CERT91,
    _reserved162: [u8; 12usize],
    #[doc = "0xf00 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert92: HW_HDCP_TX_CERT92,
    _reserved163: [u8; 12usize],
    #[doc = "0xf10 - Value of OTP Bank16 Word1 (HDCP Key)"]
    pub hw_hdcp_tx_cert93: HW_HDCP_TX_CERT93,
    _reserved164: [u8; 12usize],
    #[doc = "0xf20 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert94: HW_HDCP_TX_CERT94,
    _reserved165: [u8; 12usize],
    #[doc = "0xf30 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_tx_cert95: HW_HDCP_TX_CERT95,
    _reserved166: [u8; 12usize],
    #[doc = "0xf40 - Value of OTP Bank17 Word0 (HDCP Key)"]
    pub hw_hdcp_key0: HW_HDCP_KEY0,
    _reserved167: [u8; 12usize],
    #[doc = "0xf50 - Value of OTP Bank17 Word1 (HDCP Key)"]
    pub hw_hdcp_key1: HW_HDCP_KEY1,
    _reserved168: [u8; 12usize],
    #[doc = "0xf60 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key2: HW_HDCP_KEY2,
    _reserved169: [u8; 12usize],
    #[doc = "0xf70 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key3: HW_HDCP_KEY3,
    _reserved170: [u8; 12usize],
    #[doc = "0xf80 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key4: HW_HDCP_KEY4,
    _reserved171: [u8; 12usize],
    #[doc = "0xf90 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key5: HW_HDCP_KEY5,
    _reserved172: [u8; 12usize],
    #[doc = "0xfa0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key6: HW_HDCP_KEY6,
    _reserved173: [u8; 12usize],
    #[doc = "0xfb0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key7: HW_HDCP_KEY7,
    _reserved174: [u8; 12usize],
    #[doc = "0xfc0 - Value of OTP Bank17 Word0 (HDCP Key)"]
    pub hw_hdcp_key8: HW_HDCP_KEY8,
    _reserved175: [u8; 12usize],
    #[doc = "0xfd0 - Value of OTP Bank17 Word1 (HDCP Key)"]
    pub hw_hdcp_key9: HW_HDCP_KEY9,
    _reserved176: [u8; 12usize],
    #[doc = "0xfe0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key10: HW_HDCP_KEY10,
    _reserved177: [u8; 12usize],
    #[doc = "0xff0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key11: HW_HDCP_KEY11,
    _reserved178: [u8; 12usize],
    #[doc = "0x1000 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key12: HW_HDCP_KEY12,
    _reserved179: [u8; 12usize],
    #[doc = "0x1010 - Value of OTP Bank16 Word1 (HDCP Key)"]
    pub hw_hdcp_key13: HW_HDCP_KEY13,
    _reserved180: [u8; 12usize],
    #[doc = "0x1020 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key14: HW_HDCP_KEY14,
    _reserved181: [u8; 12usize],
    #[doc = "0x1030 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key15: HW_HDCP_KEY15,
    _reserved182: [u8; 12usize],
    #[doc = "0x1040 - Value of OTP Bank17 Word0 (HDCP Key)"]
    pub hw_hdcp_key16: HW_HDCP_KEY16,
    _reserved183: [u8; 12usize],
    #[doc = "0x1050 - Value of OTP Bank17 Word1 (HDCP Key)"]
    pub hw_hdcp_key17: HW_HDCP_KEY17,
    _reserved184: [u8; 12usize],
    #[doc = "0x1060 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key18: HW_HDCP_KEY18,
    _reserved185: [u8; 12usize],
    #[doc = "0x1070 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key19: HW_HDCP_KEY19,
    _reserved186: [u8; 12usize],
    #[doc = "0x1080 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key20: HW_HDCP_KEY20,
    _reserved187: [u8; 12usize],
    #[doc = "0x1090 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key21: HW_HDCP_KEY21,
    _reserved188: [u8; 12usize],
    #[doc = "0x10a0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key22: HW_HDCP_KEY22,
    _reserved189: [u8; 12usize],
    #[doc = "0x10b0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key23: HW_HDCP_KEY23,
    _reserved190: [u8; 12usize],
    #[doc = "0x10c0 - Value of OTP Bank17 Word0 (HDCP Key)"]
    pub hw_hdcp_key24: HW_HDCP_KEY24,
    _reserved191: [u8; 12usize],
    #[doc = "0x10d0 - Value of OTP Bank17 Word1 (HDCP Key)"]
    pub hw_hdcp_key25: HW_HDCP_KEY25,
    _reserved192: [u8; 12usize],
    #[doc = "0x10e0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key26: HW_HDCP_KEY26,
    _reserved193: [u8; 12usize],
    #[doc = "0x10f0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key27: HW_HDCP_KEY27,
    _reserved194: [u8; 12usize],
    #[doc = "0x1100 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key28: HW_HDCP_KEY28,
    _reserved195: [u8; 12usize],
    #[doc = "0x1110 - Value of OTP Bank16 Word1 (HDCP Key)"]
    pub hw_hdcp_key29: HW_HDCP_KEY29,
    _reserved196: [u8; 12usize],
    #[doc = "0x1120 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key30: HW_HDCP_KEY30,
    _reserved197: [u8; 12usize],
    #[doc = "0x1130 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key31: HW_HDCP_KEY31,
    _reserved198: [u8; 12usize],
    #[doc = "0x1140 - Value of OTP Bank17 Word0 (HDCP Key)"]
    pub hw_hdcp_key32: HW_HDCP_KEY32,
    _reserved199: [u8; 12usize],
    #[doc = "0x1150 - Value of OTP Bank17 Word1 (HDCP Key)"]
    pub hw_hdcp_key33: HW_HDCP_KEY33,
    _reserved200: [u8; 12usize],
    #[doc = "0x1160 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key34: HW_HDCP_KEY34,
    _reserved201: [u8; 12usize],
    #[doc = "0x1170 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key35: HW_HDCP_KEY35,
    _reserved202: [u8; 12usize],
    #[doc = "0x1180 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key36: HW_HDCP_KEY36,
    _reserved203: [u8; 12usize],
    #[doc = "0x1190 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key37: HW_HDCP_KEY37,
    _reserved204: [u8; 12usize],
    #[doc = "0x11a0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key38: HW_HDCP_KEY38,
    _reserved205: [u8; 12usize],
    #[doc = "0x11b0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key39: HW_HDCP_KEY39,
    _reserved206: [u8; 12usize],
    #[doc = "0x11c0 - Value of OTP Bank17 Word0 (HDCP Key)"]
    pub hw_hdcp_key40: HW_HDCP_KEY40,
    _reserved207: [u8; 12usize],
    #[doc = "0x11d0 - Value of OTP Bank17 Word1 (HDCP Key)"]
    pub hw_hdcp_key41: HW_HDCP_KEY41,
    _reserved208: [u8; 12usize],
    #[doc = "0x11e0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key42: HW_HDCP_KEY42,
    _reserved209: [u8; 12usize],
    #[doc = "0x11f0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key43: HW_HDCP_KEY43,
    _reserved210: [u8; 12usize],
    #[doc = "0x1200 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key44: HW_HDCP_KEY44,
    _reserved211: [u8; 12usize],
    #[doc = "0x1210 - Value of OTP Bank16 Word1 (HDCP Key)"]
    pub hw_hdcp_key45: HW_HDCP_KEY45,
    _reserved212: [u8; 12usize],
    #[doc = "0x1220 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key46: HW_HDCP_KEY46,
    _reserved213: [u8; 12usize],
    #[doc = "0x1230 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key47: HW_HDCP_KEY47,
    _reserved214: [u8; 12usize],
    #[doc = "0x1240 - Value of OTP Bank17 Word0 (HDCP Key)"]
    pub hw_hdcp_key48: HW_HDCP_KEY48,
    _reserved215: [u8; 12usize],
    #[doc = "0x1250 - Value of OTP Bank17 Word1 (HDCP Key)"]
    pub hw_hdcp_key49: HW_HDCP_KEY49,
    _reserved216: [u8; 12usize],
    #[doc = "0x1260 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key50: HW_HDCP_KEY50,
    _reserved217: [u8; 12usize],
    #[doc = "0x1270 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key51: HW_HDCP_KEY51,
    _reserved218: [u8; 12usize],
    #[doc = "0x1280 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key52: HW_HDCP_KEY52,
    _reserved219: [u8; 12usize],
    #[doc = "0x1290 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key53: HW_HDCP_KEY53,
    _reserved220: [u8; 12usize],
    #[doc = "0x12a0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key54: HW_HDCP_KEY54,
    _reserved221: [u8; 12usize],
    #[doc = "0x12b0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key55: HW_HDCP_KEY55,
    _reserved222: [u8; 12usize],
    #[doc = "0x12c0 - Value of OTP Bank17 Word0 (HDCP Key)"]
    pub hw_hdcp_key56: HW_HDCP_KEY56,
    _reserved223: [u8; 12usize],
    #[doc = "0x12d0 - Value of OTP Bank17 Word1 (HDCP Key)"]
    pub hw_hdcp_key57: HW_HDCP_KEY57,
    _reserved224: [u8; 12usize],
    #[doc = "0x12e0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key58: HW_HDCP_KEY58,
    _reserved225: [u8; 12usize],
    #[doc = "0x12f0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key59: HW_HDCP_KEY59,
    _reserved226: [u8; 12usize],
    #[doc = "0x1300 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key60: HW_HDCP_KEY60,
    _reserved227: [u8; 12usize],
    #[doc = "0x1310 - Value of OTP Bank16 Word1 (HDCP Key)"]
    pub hw_hdcp_key61: HW_HDCP_KEY61,
    _reserved228: [u8; 12usize],
    #[doc = "0x1320 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key62: HW_HDCP_KEY62,
    _reserved229: [u8; 12usize],
    #[doc = "0x1330 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key63: HW_HDCP_KEY63,
    _reserved230: [u8; 12usize],
    #[doc = "0x1340 - Value of OTP Bank17 Word0 (HDCP Key)"]
    pub hw_hdcp_key64: HW_HDCP_KEY64,
    _reserved231: [u8; 12usize],
    #[doc = "0x1350 - Value of OTP Bank17 Word1 (HDCP Key)"]
    pub hw_hdcp_key65: HW_HDCP_KEY65,
    _reserved232: [u8; 12usize],
    #[doc = "0x1360 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key66: HW_HDCP_KEY66,
    _reserved233: [u8; 12usize],
    #[doc = "0x1370 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key67: HW_HDCP_KEY67,
    _reserved234: [u8; 12usize],
    #[doc = "0x1380 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key68: HW_HDCP_KEY68,
    _reserved235: [u8; 12usize],
    #[doc = "0x1390 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key69: HW_HDCP_KEY69,
    _reserved236: [u8; 12usize],
    #[doc = "0x13a0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key70: HW_HDCP_KEY70,
    _reserved237: [u8; 12usize],
    #[doc = "0x13b0 - Value of OTP Bank16 Word0 (HDCP Key)"]
    pub hw_hdcp_key71: HW_HDCP_KEY71,
}
#[doc = "OTP Controller Control Register"]
pub struct HW_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP Controller Control Register"]
pub mod hw_ctrl;
#[doc = "OTP Controller Control Register"]
pub struct HW_CTRL_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP Controller Control Register"]
pub mod hw_ctrl_set;
#[doc = "OTP Controller Control Register"]
pub struct HW_CTRL_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP Controller Control Register"]
pub mod hw_ctrl_clr;
#[doc = "OTP Controller Control Register"]
pub struct HW_CTRL_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP Controller Control Register"]
pub mod hw_ctrl_tog;
#[doc = "OTP Controller Timing Register"]
pub struct HW_TIMING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP Controller Timing Register"]
pub mod hw_timing;
#[doc = "OTP Controller Write Data Register"]
pub struct HW_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP Controller Write Data Register"]
pub mod hw_data;
#[doc = "OTP Controller Write Data Register"]
pub struct HW_READ_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP Controller Write Data Register"]
pub mod hw_read_ctrl;
#[doc = "OTP Controller Read Data Register"]
pub struct HW_READ_FUSE_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP Controller Read Data Register"]
pub mod hw_read_fuse_data;
#[doc = "Sticky bit Register"]
pub struct HW_SW_STICKY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sticky bit Register"]
pub mod hw_sw_sticky;
#[doc = "Software Controllable Signals Register"]
pub struct HW_SCS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Controllable Signals Register"]
pub mod hw_scs;
#[doc = "Software Controllable Signals Register"]
pub struct HW_SCS_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Controllable Signals Register"]
pub mod hw_scs_set;
#[doc = "Software Controllable Signals Register"]
pub struct HW_SCS_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Controllable Signals Register"]
pub mod hw_scs_clr;
#[doc = "Software Controllable Signals Register"]
pub struct HW_SCS_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Controllable Signals Register"]
pub mod hw_scs_tog;
#[doc = "OTP Controller Version Register"]
pub struct HW_VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTP Controller Version Register"]
pub mod hw_version;
#[doc = "Value of OTP Bank0 Word0 (Lock controls)"]
pub struct HW_LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank0 Word0 (Lock controls)"]
pub mod hw_lock;
#[doc = "Value of OTP Bank0 Word1 (Tester Info.)"]
pub struct HW_TESTER0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank0 Word1 (Tester Info.)"]
pub mod hw_tester0;
#[doc = "Value of OTP Bank0 Word2 (tester Info.)"]
pub struct HW_TESTER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank0 Word2 (tester Info.)"]
pub mod hw_tester1;
#[doc = "Value of OTP Bank0 Word3 (Tester Info.)"]
pub struct HW_TESTER2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank0 Word3 (Tester Info.)"]
pub mod hw_tester2;
#[doc = "Value of OTP Bank1 Word0 (Tester Info.)"]
pub struct HW_TESTER3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank1 Word0 (Tester Info.)"]
pub mod hw_tester3;
#[doc = "Value of OTP Bank1 Word1 (Tester Info.)"]
pub struct HW_TESTER4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank1 Word1 (Tester Info.)"]
pub mod hw_tester4;
#[doc = "Value of OTP Bank1 Word2 (Tester Info.)"]
pub struct HW_TESTER5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank1 Word2 (Tester Info.)"]
pub mod hw_tester5;
#[doc = "Value of OTP Bank1 Word3 (Boot Configuration Info.)"]
pub struct HW_BOOT_CFG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank1 Word3 (Boot Configuration Info.)"]
pub mod hw_boot_cfg0;
#[doc = "Value of OTP Bank2 Word0 (Boot Configuration Info.)"]
pub struct HW_BOOT_CFG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank2 Word0 (Boot Configuration Info.)"]
pub mod hw_boot_cfg1;
#[doc = "Value of OTP Bank2 Word1 (Boot Configuration Info.)"]
pub struct HW_BOOT_CFG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank2 Word1 (Boot Configuration Info.)"]
pub mod hw_boot_cfg2;
#[doc = "Value of OTP Bank2 Word2 (Boot Configuration Info.)"]
pub struct HW_BOOT_CFG3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank2 Word2 (Boot Configuration Info.)"]
pub mod hw_boot_cfg3;
#[doc = "Value of OTP Bank2 Word3 (BOOT Configuration Info.)"]
pub struct HW_BOOT_CFG4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank2 Word3 (BOOT Configuration Info.)"]
pub mod hw_boot_cfg4;
#[doc = "Value of OTP Bank3 Word0 (Memory Related Info.)"]
pub struct HW_MEM_TRIM0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank3 Word0 (Memory Related Info.)"]
pub mod hw_mem_trim0;
#[doc = "Value of OTP Bank3 Word1 (Memory Related Info.)"]
pub struct HW_MEM_TRIM1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank3 Word1 (Memory Related Info.)"]
pub mod hw_mem_trim1;
#[doc = "Value of OTP Bank3 Word2 (Analog Info.)"]
pub struct HW_ANA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank3 Word2 (Analog Info.)"]
pub mod hw_ana0;
#[doc = "Value of OTP Bank3 Word3 (Analog Info.)"]
pub struct HW_ANA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank3 Word3 (Analog Info.)"]
pub mod hw_ana1;
#[doc = "Shadow Register for OTP Bank6 Word0 (SRK Hash)"]
pub struct HW_SRK0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Register for OTP Bank6 Word0 (SRK Hash)"]
pub mod hw_srk0;
#[doc = "Shadow Register for OTP Bank6 Word1 (SRK Hash)"]
pub struct HW_SRK1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Register for OTP Bank6 Word1 (SRK Hash)"]
pub mod hw_srk1;
#[doc = "Shadow Register for OTP Bank6 Word2 (SRK Hash)"]
pub struct HW_SRK2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Register for OTP Bank6 Word2 (SRK Hash)"]
pub mod hw_srk2;
#[doc = "Shadow Register for OTP Bank6 Word3 (SRK Hash)"]
pub struct HW_SRK3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Register for OTP Bank6 Word3 (SRK Hash)"]
pub mod hw_srk3;
#[doc = "Shadow Register for OTP Bank7 Word0 (SRK Hash)"]
pub struct HW_SRK4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Register for OTP Bank7 Word0 (SRK Hash)"]
pub mod hw_srk4;
#[doc = "Shadow Register for OTP Bank7 Word1 (SRK Hash)"]
pub struct HW_SRK5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Register for OTP Bank7 Word1 (SRK Hash)"]
pub mod hw_srk5;
#[doc = "Shadow Register for OTP Bank7 Word2 (SRK Hash)"]
pub struct HW_SRK6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Register for OTP Bank7 Word2 (SRK Hash)"]
pub mod hw_srk6;
#[doc = "Shadow Register for OTP Bank7 Word3 (SRK Hash)"]
pub struct HW_SRK7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Register for OTP Bank7 Word3 (SRK Hash)"]
pub mod hw_srk7;
#[doc = "Value of OTP Bank8 Word0 (Secure JTAG Response Field)"]
pub struct HW_SJC_RESP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank8 Word0 (Secure JTAG Response Field)"]
pub mod hw_sjc_resp0;
#[doc = "Value of OTP Bank8 Word1 (Secure JTAG Response Field)"]
pub struct HW_SJC_RESP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank8 Word1 (Secure JTAG Response Field)"]
pub mod hw_sjc_resp1;
#[doc = "Value of OTP Bank8 Word2 (USB ID info)"]
pub struct HW_USB_ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank8 Word2 (USB ID info)"]
pub mod hw_usb_id;
#[doc = "Value of OTP Bank5 Word6 (Field Return)"]
pub struct HW_FIELD_RETURN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank5 Word6 (Field Return)"]
pub mod hw_field_return;
#[doc = "Value of OTP Bank9 Word0 (MAC Address)"]
pub struct HW_MAC_ADDR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank9 Word0 (MAC Address)"]
pub mod hw_mac_addr0;
#[doc = "Value of OTP Bank9 Word1 (MAC Address)"]
pub struct HW_MAC_ADDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank9 Word1 (MAC Address)"]
pub mod hw_mac_addr1;
#[doc = "Value of OTP Bank9 Word2 (MAC Address)"]
pub struct HW_MAC_ADDR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank9 Word2 (MAC Address)"]
pub mod hw_mac_addr2;
#[doc = "Value of OTP Bank9 Word3 (SRK Revoke)"]
pub struct HW_SRK_REVOKE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank9 Word3 (SRK Revoke)"]
pub mod hw_srk_revoke;
#[doc = "Shadow Register for OTP Bank10 Word0 (MAU Key)"]
pub struct HW_MAU_KEY0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Register for OTP Bank10 Word0 (MAU Key)"]
pub mod hw_mau_key0;
#[doc = "Shadow Register for OTP Bank10 Word1 (MAU Key)"]
pub struct HW_MAU_KEY1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Register for OTP Bank10 Word1 (MAU Key)"]
pub mod hw_mau_key1;
#[doc = "Shadow Register for OTP Bank10 Word2 (MAU Key)"]
pub struct HW_MAU_KEY2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Register for OTP Bank10 Word2 (MAU Key)"]
pub mod hw_mau_key2;
#[doc = "Shadow Register for OTP Bank10 Word3 (MAU Key)"]
pub struct HW_MAU_KEY3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Register for OTP Bank10 Word3 (MAU Key)"]
pub mod hw_mau_key3;
#[doc = "Shadow Register for OTP Bank11 Word0 (MAU Key)"]
pub struct HW_MAU_KEY4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Register for OTP Bank11 Word0 (MAU Key)"]
pub mod hw_mau_key4;
#[doc = "Shadow Register for OTP Bank11 Word1 (MAU Key)"]
pub struct HW_MAU_KEY5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Register for OTP Bank11 Word1 (MAU Key)"]
pub mod hw_mau_key5;
#[doc = "Shadow Register for OTP Bank11 Word2 (MAU Key)"]
pub struct HW_MAU_KEY6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Register for OTP Bank11 Word2 (MAU Key)"]
pub mod hw_mau_key6;
#[doc = "Shadow Register for OTP Bank11 Word3 (MAU Key)"]
pub struct HW_MAU_KEY7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Register for OTP Bank11 Word3 (MAU Key)"]
pub mod hw_mau_key7;
#[doc = "Value of OTP Bank14 Word0 ()"]
pub struct HW_GP10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank14 Word0 ()"]
pub mod hw_gp10;
#[doc = "Value of OTP Bank14 Word1 ()"]
pub struct HW_GP11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank14 Word1 ()"]
pub mod hw_gp11;
#[doc = "Value of OTP Bank14 Word2 ()"]
pub struct HW_GP20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank14 Word2 ()"]
pub mod hw_gp20;
#[doc = "Value of OTP Bank14 Word3 ()"]
pub struct HW_GP21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank14 Word3 ()"]
pub mod hw_gp21;
#[doc = "Value of OTP Bank15 Word0 (CRC Key)"]
pub struct HW_GP_CRC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank15 Word0 (CRC Key)"]
pub mod hw_gp_crc0;
#[doc = "Value of OTP Bank15 Word1 (CRC Key)"]
pub struct HW_GP_CRC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank15 Word1 (CRC Key)"]
pub mod hw_gp_crc1;
#[doc = "Value of OTP Bank15 Word2 (CRC Key)"]
pub struct HW_GP_CRC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank15 Word2 (CRC Key)"]
pub mod hw_gp_crc2;
#[doc = "Value of OTP Bank15 Word3 (CRC Key)"]
pub struct HW_GROUP_MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank15 Word3 (CRC Key)"]
pub mod hw_group_mask;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDMI_FW_SRK0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdmi_fw_srk0;
#[doc = "Value of OTP Bank16 Word1 (HDCP Key)"]
pub struct HW_HDMI_FW_SRK1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word1 (HDCP Key)"]
pub mod hw_hdmi_fw_srk1;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDMI_FW_SRK2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdmi_fw_srk2;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDMI_FW_SRK3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdmi_fw_srk3;
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub struct HW_HDMI_FW_SRK4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub mod hw_hdmi_fw_srk4;
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub struct HW_HDMI_FW_SRK5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub mod hw_hdmi_fw_srk5;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDMI_FW_SRK6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdmi_fw_srk6;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDMI_FW_SRK7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdmi_fw_srk7;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDMI_KMEK0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdmi_kmek0;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDMI_KMEK1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdmi_kmek1;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDMI_KMEK2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdmi_kmek2;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDMI_KMEK3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdmi_kmek3;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CONS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cons0;
#[doc = "Value of OTP Bank16 Word1 (HDCP Key)"]
pub struct HW_HDCP_TX_CONS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word1 (HDCP Key)"]
pub mod hw_hdcp_tx_cons1;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CONS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cons2;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CONS3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cons3;
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert0;
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub mod hw_hdcp_tx_cert1;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert2;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert3;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert4;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert5;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert6;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert7;
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert8;
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub mod hw_hdcp_tx_cert9;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert10;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert11;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert12;
#[doc = "Value of OTP Bank16 Word1 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word1 (HDCP Key)"]
pub mod hw_hdcp_tx_cert13;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert14;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert15;
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert16;
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub mod hw_hdcp_tx_cert17;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert18;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert19;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert20;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert21;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert22;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert23;
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert24;
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub mod hw_hdcp_tx_cert25;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert26;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert27;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert28;
#[doc = "Value of OTP Bank16 Word1 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word1 (HDCP Key)"]
pub mod hw_hdcp_tx_cert29;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert30;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert31;
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert32;
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT33 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub mod hw_hdcp_tx_cert33;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert34;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT35 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert35;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT36 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert36;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT37 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert37;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT38 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert38;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT39 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert39;
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT40 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert40;
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT41 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub mod hw_hdcp_tx_cert41;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT42 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert42;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT43 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert43;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT44 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert44;
#[doc = "Value of OTP Bank16 Word1 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT45 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word1 (HDCP Key)"]
pub mod hw_hdcp_tx_cert45;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT46 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert46;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT47 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert47;
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT48 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert48;
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT49 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub mod hw_hdcp_tx_cert49;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT50 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert50;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT51 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert51;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT52 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert52;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert53;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT54 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert54;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT55 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert55;
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT56 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert56;
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT57 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub mod hw_hdcp_tx_cert57;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT58 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert58;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT59 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert59;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT60 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert60;
#[doc = "Value of OTP Bank16 Word1 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT61 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word1 (HDCP Key)"]
pub mod hw_hdcp_tx_cert61;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT62 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert62;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT63 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert63;
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT64 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert64;
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT65 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub mod hw_hdcp_tx_cert65;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT66 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert66;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT67 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert67;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT68 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert68;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT69 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert69;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT70 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert70;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT71 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert71;
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT72 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert72;
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT73 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub mod hw_hdcp_tx_cert73;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT74 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert74;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT75 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert75;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT76 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert76;
#[doc = "Value of OTP Bank16 Word1 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT77 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word1 (HDCP Key)"]
pub mod hw_hdcp_tx_cert77;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT78 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert78;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT79 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert79;
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT80 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert80;
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT81 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub mod hw_hdcp_tx_cert81;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT82 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert82;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT83 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert83;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT84 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert84;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT85 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert85;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT86 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert86;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT87 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert87;
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT88 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert88;
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT89 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub mod hw_hdcp_tx_cert89;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT90 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert90;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT91 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert91;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT92 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert92;
#[doc = "Value of OTP Bank16 Word1 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT93 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word1 (HDCP Key)"]
pub mod hw_hdcp_tx_cert93;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT94 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert94;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_TX_CERT95 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_tx_cert95;
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub mod hw_hdcp_key0;
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub struct HW_HDCP_KEY1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub mod hw_hdcp_key1;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key2;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key3;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key4;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key5;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key6;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key7;
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub mod hw_hdcp_key8;
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub struct HW_HDCP_KEY9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub mod hw_hdcp_key9;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key10;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key11;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key12;
#[doc = "Value of OTP Bank16 Word1 (HDCP Key)"]
pub struct HW_HDCP_KEY13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word1 (HDCP Key)"]
pub mod hw_hdcp_key13;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key14;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key15;
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub mod hw_hdcp_key16;
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub struct HW_HDCP_KEY17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub mod hw_hdcp_key17;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key18;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key19;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key20;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key21;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key22;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key23;
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub mod hw_hdcp_key24;
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub struct HW_HDCP_KEY25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub mod hw_hdcp_key25;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key26;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key27;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key28;
#[doc = "Value of OTP Bank16 Word1 (HDCP Key)"]
pub struct HW_HDCP_KEY29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word1 (HDCP Key)"]
pub mod hw_hdcp_key29;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key30;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key31;
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub mod hw_hdcp_key32;
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub struct HW_HDCP_KEY33 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub mod hw_hdcp_key33;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key34;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY35 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key35;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY36 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key36;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY37 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key37;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY38 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key38;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY39 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key39;
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY40 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub mod hw_hdcp_key40;
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub struct HW_HDCP_KEY41 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub mod hw_hdcp_key41;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY42 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key42;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY43 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key43;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY44 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key44;
#[doc = "Value of OTP Bank16 Word1 (HDCP Key)"]
pub struct HW_HDCP_KEY45 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word1 (HDCP Key)"]
pub mod hw_hdcp_key45;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY46 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key46;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY47 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key47;
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY48 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub mod hw_hdcp_key48;
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub struct HW_HDCP_KEY49 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub mod hw_hdcp_key49;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY50 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key50;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY51 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key51;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY52 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key52;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key53;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY54 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key54;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY55 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key55;
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY56 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub mod hw_hdcp_key56;
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub struct HW_HDCP_KEY57 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub mod hw_hdcp_key57;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY58 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key58;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY59 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key59;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY60 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key60;
#[doc = "Value of OTP Bank16 Word1 (HDCP Key)"]
pub struct HW_HDCP_KEY61 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word1 (HDCP Key)"]
pub mod hw_hdcp_key61;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY62 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key62;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY63 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key63;
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY64 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word0 (HDCP Key)"]
pub mod hw_hdcp_key64;
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub struct HW_HDCP_KEY65 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank17 Word1 (HDCP Key)"]
pub mod hw_hdcp_key65;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY66 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key66;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY67 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key67;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY68 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key68;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY69 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key69;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY70 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key70;
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub struct HW_HDCP_KEY71 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of OTP Bank16 Word0 (HDCP Key)"]
pub mod hw_hdcp_key71;
