#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LCDIF General Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - LCDIF General Control Register"]
    pub ctrl_set: CTRL_SET,
    #[doc = "0x08 - LCDIF General Control Register"]
    pub ctrl_clr: CTRL_CLR,
    #[doc = "0x0c - LCDIF General Control Register"]
    pub ctrl_tog: CTRL_TOG,
    #[doc = "0x10 - LCDIF General Control1 Register"]
    pub ctrl1: CTRL1,
    #[doc = "0x14 - LCDIF General Control1 Register"]
    pub ctrl1_set: CTRL1_SET,
    #[doc = "0x18 - LCDIF General Control1 Register"]
    pub ctrl1_clr: CTRL1_CLR,
    #[doc = "0x1c - LCDIF General Control1 Register"]
    pub ctrl1_tog: CTRL1_TOG,
    #[doc = "0x20 - LCDIF General Control2 Register"]
    pub ctrl2: CTRL2,
    #[doc = "0x24 - LCDIF General Control2 Register"]
    pub ctrl2_set: CTRL2_SET,
    #[doc = "0x28 - LCDIF General Control2 Register"]
    pub ctrl2_clr: CTRL2_CLR,
    #[doc = "0x2c - LCDIF General Control2 Register"]
    pub ctrl2_tog: CTRL2_TOG,
    #[doc = "0x30 - LCDIF Horizontal and Vertical Valid Data Count Register"]
    pub transfer_count: TRANSFER_COUNT,
    _reserved0: [u8; 12usize],
    #[doc = "0x40 - LCD Interface Current Buffer Address Register"]
    pub cur_buf: CUR_BUF,
    _reserved1: [u8; 12usize],
    #[doc = "0x50 - LCD Interface Next Buffer Address Register"]
    pub next_buf: NEXT_BUF,
    _reserved2: [u8; 12usize],
    #[doc = "0x60 - LCD Interface Timing Register"]
    pub timing: TIMING,
    _reserved3: [u8; 12usize],
    #[doc = "0x70 - LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
    pub vdctrl0: VDCTRL0,
    #[doc = "0x74 - LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
    pub vdctrl0_set: VDCTRL0_SET,
    #[doc = "0x78 - LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
    pub vdctrl0_clr: VDCTRL0_CLR,
    #[doc = "0x7c - LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
    pub vdctrl0_tog: VDCTRL0_TOG,
    #[doc = "0x80 - LCDIF VSYNC Mode and Dotclk Mode Control Register1"]
    pub vdctrl1: VDCTRL1,
    _reserved4: [u8; 12usize],
    #[doc = "0x90 - LCDIF VSYNC Mode and Dotclk Mode Control Register2"]
    pub vdctrl2: VDCTRL2,
    _reserved5: [u8; 12usize],
    #[doc = "0xa0 - LCDIF VSYNC Mode and Dotclk Mode Control Register3"]
    pub vdctrl3: VDCTRL3,
    _reserved6: [u8; 12usize],
    #[doc = "0xb0 - LCDIF VSYNC Mode and Dotclk Mode Control Register4"]
    pub vdctrl4: VDCTRL4,
    _reserved7: [u8; 12usize],
    #[doc = "0xc0 - Digital Video Interface Control0 Register"]
    pub dvictrl0: DVICTRL0,
    _reserved8: [u8; 12usize],
    #[doc = "0xd0 - Digital Video Interface Control1 Register"]
    pub dvictrl1: DVICTRL1,
    _reserved9: [u8; 12usize],
    #[doc = "0xe0 - Digital Video Interface Control2 Register"]
    pub dvictrl2: DVICTRL2,
    _reserved10: [u8; 12usize],
    #[doc = "0xf0 - Digital Video Interface Control3 Register"]
    pub dvictrl3: DVICTRL3,
    _reserved11: [u8; 12usize],
    #[doc = "0x100 - Digital Video Interface Control4 Register"]
    pub dvictrl4: DVICTRL4,
    _reserved12: [u8; 12usize],
    #[doc = "0x110 - RGB to YCbCr 4:2:2 CSC Coefficient0 Register"]
    pub csc_coeff0: CSC_COEFF0,
    _reserved13: [u8; 12usize],
    #[doc = "0x120 - RGB to YCbCr 4:2:2 CSC Coefficient1 Register"]
    pub csc_coeff1: CSC_COEFF1,
    _reserved14: [u8; 12usize],
    #[doc = "0x130 - RGB to YCbCr 4:2:2 CSC Coefficent2 Register"]
    pub csc_coeff2: CSC_COEFF2,
    _reserved15: [u8; 12usize],
    #[doc = "0x140 - RGB to YCbCr 4:2:2 CSC Coefficient3 Register"]
    pub csc_coeff3: CSC_COEFF3,
    _reserved16: [u8; 12usize],
    #[doc = "0x150 - RGB to YCbCr 4:2:2 CSC Coefficient4 Register"]
    pub csc_coeff4: CSC_COEFF4,
    _reserved17: [u8; 12usize],
    #[doc = "0x160 - RGB to YCbCr 4:2:2 CSC Offset Register"]
    pub csc_offset: CSC_OFFSET,
    _reserved18: [u8; 12usize],
    #[doc = "0x170 - RGB to YCbCr 4:2:2 CSC Limit Register"]
    pub csc_limit: CSC_LIMIT,
    _reserved19: [u8; 12usize],
    #[doc = "0x180 - LCD Interface Data Register"]
    pub data: DATA,
    _reserved20: [u8; 12usize],
    #[doc = "0x190 - Bus Master Error Status Register"]
    pub bm_error_stat: BM_ERROR_STAT,
    _reserved21: [u8; 12usize],
    #[doc = "0x1a0 - CRC Status Register"]
    pub crc_stat: CRC_STAT,
    _reserved22: [u8; 12usize],
    #[doc = "0x1b0 - LCD Interface Status Register"]
    pub stat: STAT,
    _reserved23: [u8; 76usize],
    #[doc = "0x200 - LCDIF Threshold Register"]
    pub thres: THRES,
    _reserved24: [u8; 12usize],
    #[doc = "0x210 - LCDIF AS Buffer Control Register"]
    pub as_ctrl: AS_CTRL,
    _reserved25: [u8; 12usize],
    #[doc = "0x220 - Alpha Surface Buffer Pointer"]
    pub as_buf: AS_BUF,
    _reserved26: [u8; 12usize],
    #[doc = "0x230 - no description available"]
    pub as_next_buf: AS_NEXT_BUF,
    _reserved27: [u8; 12usize],
    #[doc = "0x240 - LCDIF Overlay Color Key Low"]
    pub as_clrkeylow: AS_CLRKEYLOW,
    _reserved28: [u8; 12usize],
    #[doc = "0x250 - LCDIF Overlay Color Key High"]
    pub as_clrkeyhigh: AS_CLRKEYHIGH,
    _reserved29: [u8; 12usize],
    #[doc = "0x260 - LCD working insync mode with CSI for VSYNC delay"]
    pub sync_delay: SYNC_DELAY,
}
#[doc = "LCDIF General Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF General Control Register"]
pub mod ctrl;
#[doc = "LCDIF General Control Register"]
pub struct CTRL_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF General Control Register"]
pub mod ctrl_set;
#[doc = "LCDIF General Control Register"]
pub struct CTRL_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF General Control Register"]
pub mod ctrl_clr;
#[doc = "LCDIF General Control Register"]
pub struct CTRL_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF General Control Register"]
pub mod ctrl_tog;
#[doc = "LCDIF General Control1 Register"]
pub struct CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF General Control1 Register"]
pub mod ctrl1;
#[doc = "LCDIF General Control1 Register"]
pub struct CTRL1_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF General Control1 Register"]
pub mod ctrl1_set;
#[doc = "LCDIF General Control1 Register"]
pub struct CTRL1_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF General Control1 Register"]
pub mod ctrl1_clr;
#[doc = "LCDIF General Control1 Register"]
pub struct CTRL1_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF General Control1 Register"]
pub mod ctrl1_tog;
#[doc = "LCDIF General Control2 Register"]
pub struct CTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF General Control2 Register"]
pub mod ctrl2;
#[doc = "LCDIF General Control2 Register"]
pub struct CTRL2_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF General Control2 Register"]
pub mod ctrl2_set;
#[doc = "LCDIF General Control2 Register"]
pub struct CTRL2_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF General Control2 Register"]
pub mod ctrl2_clr;
#[doc = "LCDIF General Control2 Register"]
pub struct CTRL2_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF General Control2 Register"]
pub mod ctrl2_tog;
#[doc = "LCDIF Horizontal and Vertical Valid Data Count Register"]
pub struct TRANSFER_COUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF Horizontal and Vertical Valid Data Count Register"]
pub mod transfer_count;
#[doc = "LCD Interface Current Buffer Address Register"]
pub struct CUR_BUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCD Interface Current Buffer Address Register"]
pub mod cur_buf;
#[doc = "LCD Interface Next Buffer Address Register"]
pub struct NEXT_BUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCD Interface Next Buffer Address Register"]
pub mod next_buf;
#[doc = "LCD Interface Timing Register"]
pub struct TIMING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCD Interface Timing Register"]
pub mod timing;
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
pub struct VDCTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
pub mod vdctrl0;
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
pub struct VDCTRL0_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
pub mod vdctrl0_set;
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
pub struct VDCTRL0_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
pub mod vdctrl0_clr;
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
pub struct VDCTRL0_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register0"]
pub mod vdctrl0_tog;
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register1"]
pub struct VDCTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register1"]
pub mod vdctrl1;
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register2"]
pub struct VDCTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register2"]
pub mod vdctrl2;
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register3"]
pub struct VDCTRL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register3"]
pub mod vdctrl3;
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register4"]
pub struct VDCTRL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF VSYNC Mode and Dotclk Mode Control Register4"]
pub mod vdctrl4;
#[doc = "Digital Video Interface Control0 Register"]
pub struct DVICTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital Video Interface Control0 Register"]
pub mod dvictrl0;
#[doc = "Digital Video Interface Control1 Register"]
pub struct DVICTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital Video Interface Control1 Register"]
pub mod dvictrl1;
#[doc = "Digital Video Interface Control2 Register"]
pub struct DVICTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital Video Interface Control2 Register"]
pub mod dvictrl2;
#[doc = "Digital Video Interface Control3 Register"]
pub struct DVICTRL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital Video Interface Control3 Register"]
pub mod dvictrl3;
#[doc = "Digital Video Interface Control4 Register"]
pub struct DVICTRL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital Video Interface Control4 Register"]
pub mod dvictrl4;
#[doc = "RGB to YCbCr 4:2:2 CSC Coefficient0 Register"]
pub struct CSC_COEFF0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RGB to YCbCr 4:2:2 CSC Coefficient0 Register"]
pub mod csc_coeff0;
#[doc = "RGB to YCbCr 4:2:2 CSC Coefficient1 Register"]
pub struct CSC_COEFF1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RGB to YCbCr 4:2:2 CSC Coefficient1 Register"]
pub mod csc_coeff1;
#[doc = "RGB to YCbCr 4:2:2 CSC Coefficent2 Register"]
pub struct CSC_COEFF2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RGB to YCbCr 4:2:2 CSC Coefficent2 Register"]
pub mod csc_coeff2;
#[doc = "RGB to YCbCr 4:2:2 CSC Coefficient3 Register"]
pub struct CSC_COEFF3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RGB to YCbCr 4:2:2 CSC Coefficient3 Register"]
pub mod csc_coeff3;
#[doc = "RGB to YCbCr 4:2:2 CSC Coefficient4 Register"]
pub struct CSC_COEFF4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RGB to YCbCr 4:2:2 CSC Coefficient4 Register"]
pub mod csc_coeff4;
#[doc = "RGB to YCbCr 4:2:2 CSC Offset Register"]
pub struct CSC_OFFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RGB to YCbCr 4:2:2 CSC Offset Register"]
pub mod csc_offset;
#[doc = "RGB to YCbCr 4:2:2 CSC Limit Register"]
pub struct CSC_LIMIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RGB to YCbCr 4:2:2 CSC Limit Register"]
pub mod csc_limit;
#[doc = "LCD Interface Data Register"]
pub struct DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCD Interface Data Register"]
pub mod data;
#[doc = "Bus Master Error Status Register"]
pub struct BM_ERROR_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bus Master Error Status Register"]
pub mod bm_error_stat;
#[doc = "CRC Status Register"]
pub struct CRC_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Status Register"]
pub mod crc_stat;
#[doc = "LCD Interface Status Register"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCD Interface Status Register"]
pub mod stat;
#[doc = "LCDIF Threshold Register"]
pub struct THRES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF Threshold Register"]
pub mod thres;
#[doc = "LCDIF AS Buffer Control Register"]
pub struct AS_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF AS Buffer Control Register"]
pub mod as_ctrl;
#[doc = "Alpha Surface Buffer Pointer"]
pub struct AS_BUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alpha Surface Buffer Pointer"]
pub mod as_buf;
#[doc = "no description available"]
pub struct AS_NEXT_BUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod as_next_buf;
#[doc = "LCDIF Overlay Color Key Low"]
pub struct AS_CLRKEYLOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF Overlay Color Key Low"]
pub mod as_clrkeylow;
#[doc = "LCDIF Overlay Color Key High"]
pub struct AS_CLRKEYHIGH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCDIF Overlay Color Key High"]
pub mod as_clrkeyhigh;
#[doc = "LCD working insync mode with CSI for VSYNC delay"]
pub struct SYNC_DELAY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCD working insync mode with CSI for VSYNC delay"]
pub mod sync_delay;
