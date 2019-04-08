#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPMI Control Register 0 Description"]
    pub ctrl0: CTRL0,
    #[doc = "0x04 - GPMI Control Register 0 Description"]
    pub ctrl0_set: CTRL0_SET,
    #[doc = "0x08 - GPMI Control Register 0 Description"]
    pub ctrl0_clr: CTRL0_CLR,
    #[doc = "0x0c - GPMI Control Register 0 Description"]
    pub ctrl0_tog: CTRL0_TOG,
    #[doc = "0x10 - GPMI Compare Register Description"]
    pub compare: COMPARE,
    _reserved0: [u8; 12usize],
    #[doc = "0x20 - GPMI Integrated ECC Control Register Description"]
    pub eccctrl: ECCCTRL,
    #[doc = "0x24 - GPMI Integrated ECC Control Register Description"]
    pub eccctrl_set: ECCCTRL_SET,
    #[doc = "0x28 - GPMI Integrated ECC Control Register Description"]
    pub eccctrl_clr: ECCCTRL_CLR,
    #[doc = "0x2c - GPMI Integrated ECC Control Register Description"]
    pub eccctrl_tog: ECCCTRL_TOG,
    #[doc = "0x30 - GPMI Integrated ECC Transfer Count Register Description"]
    pub ecccount: ECCCOUNT,
    _reserved1: [u8; 12usize],
    #[doc = "0x40 - GPMI Payload Address Register Description"]
    pub payload: PAYLOAD,
    _reserved2: [u8; 12usize],
    #[doc = "0x50 - GPMI Auxiliary Address Register Description"]
    pub auxiliary: AUXILIARY,
    _reserved3: [u8; 12usize],
    #[doc = "0x60 - GPMI Control Register 1 Description"]
    pub ctrl1: CTRL1,
    #[doc = "0x64 - GPMI Control Register 1 Description"]
    pub ctrl1_set: CTRL1_SET,
    #[doc = "0x68 - GPMI Control Register 1 Description"]
    pub ctrl1_clr: CTRL1_CLR,
    #[doc = "0x6c - GPMI Control Register 1 Description"]
    pub ctrl1_tog: CTRL1_TOG,
    #[doc = "0x70 - GPMI Timing Register 0 Description"]
    pub timing0: TIMING0,
    _reserved4: [u8; 12usize],
    #[doc = "0x80 - GPMI Timing Register 1 Description"]
    pub timing1: TIMING1,
    _reserved5: [u8; 12usize],
    #[doc = "0x90 - GPMI Timing Register 2 Description"]
    pub timing2: TIMING2,
    _reserved6: [u8; 12usize],
    #[doc = "0xa0 - GPMI DMA Data Transfer Register Description"]
    pub data: DATA,
    _reserved7: [u8; 12usize],
    #[doc = "0xb0 - GPMI Status Register Description"]
    pub stat: STAT,
    _reserved8: [u8; 12usize],
    #[doc = "0xc0 - GPMI Debug Information Register Description"]
    pub debug: DEBUG,
    _reserved9: [u8; 12usize],
    #[doc = "0xd0 - GPMI Version Register Description"]
    pub version: VERSION,
    _reserved10: [u8; 12usize],
    #[doc = "0xe0 - GPMI Debug2 Information Register Description"]
    pub debug2: DEBUG2,
    _reserved11: [u8; 12usize],
    #[doc = "0xf0 - GPMI Debug3 Information Register Description"]
    pub debug3: DEBUG3,
    _reserved12: [u8; 12usize],
    #[doc = "0x100 - GPMI Double Rate Read DLL Control Register Description"]
    pub read_ddr_dll_ctrl: READ_DDR_DLL_CTRL,
    _reserved13: [u8; 12usize],
    #[doc = "0x110 - GPMI Double Rate Write DLL Control Register Description"]
    pub write_ddr_dll_ctrl: WRITE_DDR_DLL_CTRL,
    _reserved14: [u8; 12usize],
    #[doc = "0x120 - GPMI Double Rate Read DLL Status Register Description"]
    pub read_ddr_dll_sts: READ_DDR_DLL_STS,
    _reserved15: [u8; 12usize],
    #[doc = "0x130 - GPMI Double Rate Write DLL Status Register Description"]
    pub write_ddr_dll_sts: WRITE_DDR_DLL_STS,
}
#[doc = "GPMI Control Register 0 Description"]
pub struct CTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Control Register 0 Description"]
pub mod ctrl0;
#[doc = "GPMI Control Register 0 Description"]
pub struct CTRL0_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Control Register 0 Description"]
pub mod ctrl0_set;
#[doc = "GPMI Control Register 0 Description"]
pub struct CTRL0_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Control Register 0 Description"]
pub mod ctrl0_clr;
#[doc = "GPMI Control Register 0 Description"]
pub struct CTRL0_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Control Register 0 Description"]
pub mod ctrl0_tog;
#[doc = "GPMI Compare Register Description"]
pub struct COMPARE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Compare Register Description"]
pub mod compare;
#[doc = "GPMI Integrated ECC Control Register Description"]
pub struct ECCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Integrated ECC Control Register Description"]
pub mod eccctrl;
#[doc = "GPMI Integrated ECC Control Register Description"]
pub struct ECCCTRL_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Integrated ECC Control Register Description"]
pub mod eccctrl_set;
#[doc = "GPMI Integrated ECC Control Register Description"]
pub struct ECCCTRL_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Integrated ECC Control Register Description"]
pub mod eccctrl_clr;
#[doc = "GPMI Integrated ECC Control Register Description"]
pub struct ECCCTRL_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Integrated ECC Control Register Description"]
pub mod eccctrl_tog;
#[doc = "GPMI Integrated ECC Transfer Count Register Description"]
pub struct ECCCOUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Integrated ECC Transfer Count Register Description"]
pub mod ecccount;
#[doc = "GPMI Payload Address Register Description"]
pub struct PAYLOAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Payload Address Register Description"]
pub mod payload;
#[doc = "GPMI Auxiliary Address Register Description"]
pub struct AUXILIARY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Auxiliary Address Register Description"]
pub mod auxiliary;
#[doc = "GPMI Control Register 1 Description"]
pub struct CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Control Register 1 Description"]
pub mod ctrl1;
#[doc = "GPMI Control Register 1 Description"]
pub struct CTRL1_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Control Register 1 Description"]
pub mod ctrl1_set;
#[doc = "GPMI Control Register 1 Description"]
pub struct CTRL1_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Control Register 1 Description"]
pub mod ctrl1_clr;
#[doc = "GPMI Control Register 1 Description"]
pub struct CTRL1_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Control Register 1 Description"]
pub mod ctrl1_tog;
#[doc = "GPMI Timing Register 0 Description"]
pub struct TIMING0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Timing Register 0 Description"]
pub mod timing0;
#[doc = "GPMI Timing Register 1 Description"]
pub struct TIMING1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Timing Register 1 Description"]
pub mod timing1;
#[doc = "GPMI Timing Register 2 Description"]
pub struct TIMING2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Timing Register 2 Description"]
pub mod timing2;
#[doc = "GPMI DMA Data Transfer Register Description"]
pub struct DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI DMA Data Transfer Register Description"]
pub mod data;
#[doc = "GPMI Status Register Description"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Status Register Description"]
pub mod stat;
#[doc = "GPMI Debug Information Register Description"]
pub struct DEBUG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Debug Information Register Description"]
pub mod debug;
#[doc = "GPMI Version Register Description"]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Version Register Description"]
pub mod version;
#[doc = "GPMI Debug2 Information Register Description"]
pub struct DEBUG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Debug2 Information Register Description"]
pub mod debug2;
#[doc = "GPMI Debug3 Information Register Description"]
pub struct DEBUG3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Debug3 Information Register Description"]
pub mod debug3;
#[doc = "GPMI Double Rate Read DLL Control Register Description"]
pub struct READ_DDR_DLL_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Double Rate Read DLL Control Register Description"]
pub mod read_ddr_dll_ctrl;
#[doc = "GPMI Double Rate Write DLL Control Register Description"]
pub struct WRITE_DDR_DLL_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Double Rate Write DLL Control Register Description"]
pub mod write_ddr_dll_ctrl;
#[doc = "GPMI Double Rate Read DLL Status Register Description"]
pub struct READ_DDR_DLL_STS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Double Rate Read DLL Status Register Description"]
pub mod read_ddr_dll_sts;
#[doc = "GPMI Double Rate Write DLL Status Register Description"]
pub struct WRITE_DDR_DLL_STS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPMI Double Rate Write DLL Status Register Description"]
pub mod write_ddr_dll_sts;
