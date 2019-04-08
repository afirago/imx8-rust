#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Hardware BCH ECC Accelerator Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Hardware BCH ECC Accelerator Control Register"]
    pub ctrl_set: CTRL_SET,
    #[doc = "0x08 - Hardware BCH ECC Accelerator Control Register"]
    pub ctrl_clr: CTRL_CLR,
    #[doc = "0x0c - Hardware BCH ECC Accelerator Control Register"]
    pub ctrl_tog: CTRL_TOG,
    #[doc = "0x10 - Hardware ECC Accelerator Status Register 0"]
    pub status0: STATUS0,
    _reserved0: [u8; 12usize],
    #[doc = "0x20 - Hardware ECC Accelerator Mode Register"]
    pub mode: MODE,
    _reserved1: [u8; 12usize],
    #[doc = "0x30 - Hardware BCH ECC Loopback Encode Buffer Register"]
    pub encodeptr: ENCODEPTR,
    _reserved2: [u8; 12usize],
    #[doc = "0x40 - Hardware BCH ECC Loopback Data Buffer Register"]
    pub dataptr: DATAPTR,
    _reserved3: [u8; 12usize],
    #[doc = "0x50 - Hardware BCH ECC Loopback Metadata Buffer Register"]
    pub metaptr: METAPTR,
    _reserved4: [u8; 28usize],
    #[doc = "0x70 - Hardware ECC Accelerator Layout Select Register"]
    pub layoutselect: LAYOUTSELECT,
    _reserved5: [u8; 12usize],
    #[doc = "0x80 - Hardware BCH ECC Flash 0 Layout 0 Register"]
    pub flash0layout0: FLASH0LAYOUT0,
    _reserved6: [u8; 12usize],
    #[doc = "0x90 - Hardware BCH ECC Flash 0 Layout 1 Register"]
    pub flash0layout1: FLASH0LAYOUT1,
    _reserved7: [u8; 12usize],
    #[doc = "0xa0 - Hardware BCH ECC Flash 1 Layout 0 Register"]
    pub flash1layout0: FLASH1LAYOUT0,
    _reserved8: [u8; 12usize],
    #[doc = "0xb0 - Hardware BCH ECC Flash 1 Layout 1 Register"]
    pub flash1layout1: FLASH1LAYOUT1,
    _reserved9: [u8; 12usize],
    #[doc = "0xc0 - Hardware BCH ECC Flash 2 Layout 0 Register"]
    pub flash2layout0: FLASH2LAYOUT0,
    _reserved10: [u8; 12usize],
    #[doc = "0xd0 - Hardware BCH ECC Flash 2 Layout 1 Register"]
    pub flash2layout1: FLASH2LAYOUT1,
    _reserved11: [u8; 12usize],
    #[doc = "0xe0 - Hardware BCH ECC Flash 3 Layout 0 Register"]
    pub flash3layout0: FLASH3LAYOUT0,
    _reserved12: [u8; 12usize],
    #[doc = "0xf0 - Hardware BCH ECC Flash 3 Layout 1 Register"]
    pub flash3layout1: FLASH3LAYOUT1,
    _reserved13: [u8; 12usize],
    #[doc = "0x100 - Hardware BCH ECC Debug Register0"]
    pub debug0: DEBUG0,
    #[doc = "0x104 - Hardware BCH ECC Debug Register0"]
    pub debug0_set: DEBUG0_SET,
    #[doc = "0x108 - Hardware BCH ECC Debug Register0"]
    pub debug0_clr: DEBUG0_CLR,
    #[doc = "0x10c - Hardware BCH ECC Debug Register0"]
    pub debug0_tog: DEBUG0_TOG,
    #[doc = "0x110 - KES Debug Read Register"]
    pub dbgkesread: DBGKESREAD,
    _reserved14: [u8; 12usize],
    #[doc = "0x120 - Chien Search Debug Read Register"]
    pub dbgcsferead: DBGCSFEREAD,
    _reserved15: [u8; 12usize],
    #[doc = "0x130 - Syndrome Generator Debug Read Register"]
    pub dbgsyndgenread: DBGSYNDGENREAD,
    _reserved16: [u8; 12usize],
    #[doc = "0x140 - Bus Master and ECC Controller Debug Read Register"]
    pub dbgahbmread: DBGAHBMREAD,
    _reserved17: [u8; 12usize],
    #[doc = "0x150 - Block Name Register"]
    pub blockname: BLOCKNAME,
    _reserved18: [u8; 12usize],
    #[doc = "0x160 - BCH Version Register"]
    pub version: VERSION,
    _reserved19: [u8; 12usize],
    #[doc = "0x170 - Hardware BCH ECC Debug Register 1"]
    pub debug1: DEBUG1,
}
#[doc = "Hardware BCH ECC Accelerator Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware BCH ECC Accelerator Control Register"]
pub mod ctrl;
#[doc = "Hardware BCH ECC Accelerator Control Register"]
pub struct CTRL_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware BCH ECC Accelerator Control Register"]
pub mod ctrl_set;
#[doc = "Hardware BCH ECC Accelerator Control Register"]
pub struct CTRL_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware BCH ECC Accelerator Control Register"]
pub mod ctrl_clr;
#[doc = "Hardware BCH ECC Accelerator Control Register"]
pub struct CTRL_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware BCH ECC Accelerator Control Register"]
pub mod ctrl_tog;
#[doc = "Hardware ECC Accelerator Status Register 0"]
pub struct STATUS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware ECC Accelerator Status Register 0"]
pub mod status0;
#[doc = "Hardware ECC Accelerator Mode Register"]
pub struct MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware ECC Accelerator Mode Register"]
pub mod mode;
#[doc = "Hardware BCH ECC Loopback Encode Buffer Register"]
pub struct ENCODEPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware BCH ECC Loopback Encode Buffer Register"]
pub mod encodeptr;
#[doc = "Hardware BCH ECC Loopback Data Buffer Register"]
pub struct DATAPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware BCH ECC Loopback Data Buffer Register"]
pub mod dataptr;
#[doc = "Hardware BCH ECC Loopback Metadata Buffer Register"]
pub struct METAPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware BCH ECC Loopback Metadata Buffer Register"]
pub mod metaptr;
#[doc = "Hardware ECC Accelerator Layout Select Register"]
pub struct LAYOUTSELECT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware ECC Accelerator Layout Select Register"]
pub mod layoutselect;
#[doc = "Hardware BCH ECC Flash 0 Layout 0 Register"]
pub struct FLASH0LAYOUT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware BCH ECC Flash 0 Layout 0 Register"]
pub mod flash0layout0;
#[doc = "Hardware BCH ECC Flash 0 Layout 1 Register"]
pub struct FLASH0LAYOUT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware BCH ECC Flash 0 Layout 1 Register"]
pub mod flash0layout1;
#[doc = "Hardware BCH ECC Flash 1 Layout 0 Register"]
pub struct FLASH1LAYOUT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware BCH ECC Flash 1 Layout 0 Register"]
pub mod flash1layout0;
#[doc = "Hardware BCH ECC Flash 1 Layout 1 Register"]
pub struct FLASH1LAYOUT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware BCH ECC Flash 1 Layout 1 Register"]
pub mod flash1layout1;
#[doc = "Hardware BCH ECC Flash 2 Layout 0 Register"]
pub struct FLASH2LAYOUT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware BCH ECC Flash 2 Layout 0 Register"]
pub mod flash2layout0;
#[doc = "Hardware BCH ECC Flash 2 Layout 1 Register"]
pub struct FLASH2LAYOUT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware BCH ECC Flash 2 Layout 1 Register"]
pub mod flash2layout1;
#[doc = "Hardware BCH ECC Flash 3 Layout 0 Register"]
pub struct FLASH3LAYOUT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware BCH ECC Flash 3 Layout 0 Register"]
pub mod flash3layout0;
#[doc = "Hardware BCH ECC Flash 3 Layout 1 Register"]
pub struct FLASH3LAYOUT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware BCH ECC Flash 3 Layout 1 Register"]
pub mod flash3layout1;
#[doc = "Hardware BCH ECC Debug Register0"]
pub struct DEBUG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware BCH ECC Debug Register0"]
pub mod debug0;
#[doc = "Hardware BCH ECC Debug Register0"]
pub struct DEBUG0_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware BCH ECC Debug Register0"]
pub mod debug0_set;
#[doc = "Hardware BCH ECC Debug Register0"]
pub struct DEBUG0_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware BCH ECC Debug Register0"]
pub mod debug0_clr;
#[doc = "Hardware BCH ECC Debug Register0"]
pub struct DEBUG0_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware BCH ECC Debug Register0"]
pub mod debug0_tog;
#[doc = "KES Debug Read Register"]
pub struct DBGKESREAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "KES Debug Read Register"]
pub mod dbgkesread;
#[doc = "Chien Search Debug Read Register"]
pub struct DBGCSFEREAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chien Search Debug Read Register"]
pub mod dbgcsferead;
#[doc = "Syndrome Generator Debug Read Register"]
pub struct DBGSYNDGENREAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Syndrome Generator Debug Read Register"]
pub mod dbgsyndgenread;
#[doc = "Bus Master and ECC Controller Debug Read Register"]
pub struct DBGAHBMREAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bus Master and ECC Controller Debug Read Register"]
pub mod dbgahbmread;
#[doc = "Block Name Register"]
pub struct BLOCKNAME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Block Name Register"]
pub mod blockname;
#[doc = "BCH Version Register"]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BCH Version Register"]
pub mod version;
#[doc = "Hardware BCH ECC Debug Register 1"]
pub struct DEBUG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware BCH ECC Debug Register 1"]
pub mod debug1;
