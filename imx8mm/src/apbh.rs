#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AHB to APBH Bridge Control and Status Register 0"]
    pub ctrl0: CTRL0,
    #[doc = "0x04 - AHB to APBH Bridge Control and Status Register 0"]
    pub ctrl0_set: CTRL0_SET,
    #[doc = "0x08 - AHB to APBH Bridge Control and Status Register 0"]
    pub ctrl0_clr: CTRL0_CLR,
    #[doc = "0x0c - AHB to APBH Bridge Control and Status Register 0"]
    pub ctrl0_tog: CTRL0_TOG,
    #[doc = "0x10 - AHB to APBH Bridge Control and Status Register 1"]
    pub ctrl1: CTRL1,
    #[doc = "0x14 - AHB to APBH Bridge Control and Status Register 1"]
    pub ctrl1_set: CTRL1_SET,
    #[doc = "0x18 - AHB to APBH Bridge Control and Status Register 1"]
    pub ctrl1_clr: CTRL1_CLR,
    #[doc = "0x1c - AHB to APBH Bridge Control and Status Register 1"]
    pub ctrl1_tog: CTRL1_TOG,
    #[doc = "0x20 - AHB to APBH Bridge Control and Status Register 2"]
    pub ctrl2: CTRL2,
    #[doc = "0x24 - AHB to APBH Bridge Control and Status Register 2"]
    pub ctrl2_set: CTRL2_SET,
    #[doc = "0x28 - AHB to APBH Bridge Control and Status Register 2"]
    pub ctrl2_clr: CTRL2_CLR,
    #[doc = "0x2c - AHB to APBH Bridge Control and Status Register 2"]
    pub ctrl2_tog: CTRL2_TOG,
    #[doc = "0x30 - AHB to APBH Bridge Channel Register"]
    pub channel_ctrl: CHANNEL_CTRL,
    #[doc = "0x34 - AHB to APBH Bridge Channel Register"]
    pub channel_ctrl_set: CHANNEL_CTRL_SET,
    #[doc = "0x38 - AHB to APBH Bridge Channel Register"]
    pub channel_ctrl_clr: CHANNEL_CTRL_CLR,
    #[doc = "0x3c - AHB to APBH Bridge Channel Register"]
    pub channel_ctrl_tog: CHANNEL_CTRL_TOG,
    #[doc = "0x40 - AHB to APBH DMA Device Assignment Register"]
    pub devsel: DEVSEL,
    _reserved0: [u8; 12usize],
    #[doc = "0x50 - AHB to APBH DMA burst size"]
    pub dma_burst_size: DMA_BURST_SIZE,
    _reserved1: [u8; 12usize],
    #[doc = "0x60 - AHB to APBH DMA Debug Register"]
    pub debug: DEBUG,
    _reserved2: [u8; 156usize],
    #[doc = "0x100 - APBH DMA Channel n Current Command Address Register"]
    pub ch0_curcmdar: CH_CURCMDAR,
    _reserved3: [u8; 12usize],
    #[doc = "0x110 - APBH DMA Channel n Next Command Address Register"]
    pub ch0_nxtcmdar: CH_NXTCMDAR,
    _reserved4: [u8; 12usize],
    #[doc = "0x120 - APBH DMA Channel n Command Register"]
    pub ch0_cmd: CH_CMD,
    _reserved5: [u8; 12usize],
    #[doc = "0x130 - APBH DMA Channel n Buffer Address Register"]
    pub ch0_bar: CH_BAR,
    _reserved6: [u8; 12usize],
    #[doc = "0x140 - APBH DMA Channel n Semaphore Register"]
    pub ch0_sema: CH_SEMA,
    _reserved7: [u8; 12usize],
    #[doc = "0x150 - AHB to APBH DMA Channel n Debug Information"]
    pub ch0_debug1: CH_DEBUG1,
    _reserved8: [u8; 12usize],
    #[doc = "0x160 - AHB to APBH DMA Channel n Debug Information"]
    pub ch0_debug2: CH_DEBUG2,
    _reserved9: [u8; 12usize],
    #[doc = "0x170 - APBH DMA Channel n Current Command Address Register"]
    pub ch1_curcmdar: CH_CURCMDAR,
    _reserved10: [u8; 12usize],
    #[doc = "0x180 - APBH DMA Channel n Next Command Address Register"]
    pub ch1_nxtcmdar: CH_NXTCMDAR,
    _reserved11: [u8; 12usize],
    #[doc = "0x190 - APBH DMA Channel n Command Register"]
    pub ch1_cmd: CH_CMD,
    _reserved12: [u8; 12usize],
    #[doc = "0x1a0 - APBH DMA Channel n Buffer Address Register"]
    pub ch1_bar: CH_BAR,
    _reserved13: [u8; 12usize],
    #[doc = "0x1b0 - APBH DMA Channel n Semaphore Register"]
    pub ch1_sema: CH_SEMA,
    _reserved14: [u8; 12usize],
    #[doc = "0x1c0 - AHB to APBH DMA Channel n Debug Information"]
    pub ch1_debug1: CH_DEBUG1,
    _reserved15: [u8; 12usize],
    #[doc = "0x1d0 - AHB to APBH DMA Channel n Debug Information"]
    pub ch1_debug2: CH_DEBUG2,
    _reserved16: [u8; 12usize],
    #[doc = "0x1e0 - APBH DMA Channel n Current Command Address Register"]
    pub ch2_curcmdar: CH_CURCMDAR,
    _reserved17: [u8; 12usize],
    #[doc = "0x1f0 - APBH DMA Channel n Next Command Address Register"]
    pub ch2_nxtcmdar: CH_NXTCMDAR,
    _reserved18: [u8; 12usize],
    #[doc = "0x200 - APBH DMA Channel n Command Register"]
    pub ch2_cmd: CH_CMD,
    _reserved19: [u8; 12usize],
    #[doc = "0x210 - APBH DMA Channel n Buffer Address Register"]
    pub ch2_bar: CH_BAR,
    _reserved20: [u8; 12usize],
    #[doc = "0x220 - APBH DMA Channel n Semaphore Register"]
    pub ch2_sema: CH_SEMA,
    _reserved21: [u8; 12usize],
    #[doc = "0x230 - AHB to APBH DMA Channel n Debug Information"]
    pub ch2_debug1: CH_DEBUG1,
    _reserved22: [u8; 12usize],
    #[doc = "0x240 - AHB to APBH DMA Channel n Debug Information"]
    pub ch2_debug2: CH_DEBUG2,
    _reserved23: [u8; 12usize],
    #[doc = "0x250 - APBH DMA Channel n Current Command Address Register"]
    pub ch3_curcmdar: CH_CURCMDAR,
    _reserved24: [u8; 12usize],
    #[doc = "0x260 - APBH DMA Channel n Next Command Address Register"]
    pub ch3_nxtcmdar: CH_NXTCMDAR,
    _reserved25: [u8; 12usize],
    #[doc = "0x270 - APBH DMA Channel n Command Register"]
    pub ch3_cmd: CH_CMD,
    _reserved26: [u8; 12usize],
    #[doc = "0x280 - APBH DMA Channel n Buffer Address Register"]
    pub ch3_bar: CH_BAR,
    _reserved27: [u8; 12usize],
    #[doc = "0x290 - APBH DMA Channel n Semaphore Register"]
    pub ch3_sema: CH_SEMA,
    _reserved28: [u8; 12usize],
    #[doc = "0x2a0 - AHB to APBH DMA Channel n Debug Information"]
    pub ch3_debug1: CH_DEBUG1,
    _reserved29: [u8; 12usize],
    #[doc = "0x2b0 - AHB to APBH DMA Channel n Debug Information"]
    pub ch3_debug2: CH_DEBUG2,
    _reserved30: [u8; 12usize],
    #[doc = "0x2c0 - APBH DMA Channel n Current Command Address Register"]
    pub ch4_curcmdar: CH_CURCMDAR,
    _reserved31: [u8; 12usize],
    #[doc = "0x2d0 - APBH DMA Channel n Next Command Address Register"]
    pub ch4_nxtcmdar: CH_NXTCMDAR,
    _reserved32: [u8; 12usize],
    #[doc = "0x2e0 - APBH DMA Channel n Command Register"]
    pub ch4_cmd: CH_CMD,
    _reserved33: [u8; 12usize],
    #[doc = "0x2f0 - APBH DMA Channel n Buffer Address Register"]
    pub ch4_bar: CH_BAR,
    _reserved34: [u8; 12usize],
    #[doc = "0x300 - APBH DMA Channel n Semaphore Register"]
    pub ch4_sema: CH_SEMA,
    _reserved35: [u8; 12usize],
    #[doc = "0x310 - AHB to APBH DMA Channel n Debug Information"]
    pub ch4_debug1: CH_DEBUG1,
    _reserved36: [u8; 12usize],
    #[doc = "0x320 - AHB to APBH DMA Channel n Debug Information"]
    pub ch4_debug2: CH_DEBUG2,
    _reserved37: [u8; 12usize],
    #[doc = "0x330 - APBH DMA Channel n Current Command Address Register"]
    pub ch5_curcmdar: CH_CURCMDAR,
    _reserved38: [u8; 12usize],
    #[doc = "0x340 - APBH DMA Channel n Next Command Address Register"]
    pub ch5_nxtcmdar: CH_NXTCMDAR,
    _reserved39: [u8; 12usize],
    #[doc = "0x350 - APBH DMA Channel n Command Register"]
    pub ch5_cmd: CH_CMD,
    _reserved40: [u8; 12usize],
    #[doc = "0x360 - APBH DMA Channel n Buffer Address Register"]
    pub ch5_bar: CH_BAR,
    _reserved41: [u8; 12usize],
    #[doc = "0x370 - APBH DMA Channel n Semaphore Register"]
    pub ch5_sema: CH_SEMA,
    _reserved42: [u8; 12usize],
    #[doc = "0x380 - AHB to APBH DMA Channel n Debug Information"]
    pub ch5_debug1: CH_DEBUG1,
    _reserved43: [u8; 12usize],
    #[doc = "0x390 - AHB to APBH DMA Channel n Debug Information"]
    pub ch5_debug2: CH_DEBUG2,
    _reserved44: [u8; 12usize],
    #[doc = "0x3a0 - APBH DMA Channel n Current Command Address Register"]
    pub ch6_curcmdar: CH_CURCMDAR,
    _reserved45: [u8; 12usize],
    #[doc = "0x3b0 - APBH DMA Channel n Next Command Address Register"]
    pub ch6_nxtcmdar: CH_NXTCMDAR,
    _reserved46: [u8; 12usize],
    #[doc = "0x3c0 - APBH DMA Channel n Command Register"]
    pub ch6_cmd: CH_CMD,
    _reserved47: [u8; 12usize],
    #[doc = "0x3d0 - APBH DMA Channel n Buffer Address Register"]
    pub ch6_bar: CH_BAR,
    _reserved48: [u8; 12usize],
    #[doc = "0x3e0 - APBH DMA Channel n Semaphore Register"]
    pub ch6_sema: CH_SEMA,
    _reserved49: [u8; 12usize],
    #[doc = "0x3f0 - AHB to APBH DMA Channel n Debug Information"]
    pub ch6_debug1: CH_DEBUG1,
    _reserved50: [u8; 12usize],
    #[doc = "0x400 - AHB to APBH DMA Channel n Debug Information"]
    pub ch6_debug2: CH_DEBUG2,
    _reserved51: [u8; 12usize],
    #[doc = "0x410 - APBH DMA Channel n Current Command Address Register"]
    pub ch7_curcmdar: CH_CURCMDAR,
    _reserved52: [u8; 12usize],
    #[doc = "0x420 - APBH DMA Channel n Next Command Address Register"]
    pub ch7_nxtcmdar: CH_NXTCMDAR,
    _reserved53: [u8; 12usize],
    #[doc = "0x430 - APBH DMA Channel n Command Register"]
    pub ch7_cmd: CH_CMD,
    _reserved54: [u8; 12usize],
    #[doc = "0x440 - APBH DMA Channel n Buffer Address Register"]
    pub ch7_bar: CH_BAR,
    _reserved55: [u8; 12usize],
    #[doc = "0x450 - APBH DMA Channel n Semaphore Register"]
    pub ch7_sema: CH_SEMA,
    _reserved56: [u8; 12usize],
    #[doc = "0x460 - AHB to APBH DMA Channel n Debug Information"]
    pub ch7_debug1: CH_DEBUG1,
    _reserved57: [u8; 12usize],
    #[doc = "0x470 - AHB to APBH DMA Channel n Debug Information"]
    pub ch7_debug2: CH_DEBUG2,
    _reserved58: [u8; 12usize],
    #[doc = "0x480 - APBH DMA Channel n Current Command Address Register"]
    pub ch8_curcmdar: CH_CURCMDAR,
    _reserved59: [u8; 12usize],
    #[doc = "0x490 - APBH DMA Channel n Next Command Address Register"]
    pub ch8_nxtcmdar: CH_NXTCMDAR,
    _reserved60: [u8; 12usize],
    #[doc = "0x4a0 - APBH DMA Channel n Command Register"]
    pub ch8_cmd: CH_CMD,
    _reserved61: [u8; 12usize],
    #[doc = "0x4b0 - APBH DMA Channel n Buffer Address Register"]
    pub ch8_bar: CH_BAR,
    _reserved62: [u8; 12usize],
    #[doc = "0x4c0 - APBH DMA Channel n Semaphore Register"]
    pub ch8_sema: CH_SEMA,
    _reserved63: [u8; 12usize],
    #[doc = "0x4d0 - AHB to APBH DMA Channel n Debug Information"]
    pub ch8_debug1: CH_DEBUG1,
    _reserved64: [u8; 12usize],
    #[doc = "0x4e0 - AHB to APBH DMA Channel n Debug Information"]
    pub ch8_debug2: CH_DEBUG2,
    _reserved65: [u8; 12usize],
    #[doc = "0x4f0 - APBH DMA Channel n Current Command Address Register"]
    pub ch9_curcmdar: CH_CURCMDAR,
    _reserved66: [u8; 12usize],
    #[doc = "0x500 - APBH DMA Channel n Next Command Address Register"]
    pub ch9_nxtcmdar: CH_NXTCMDAR,
    _reserved67: [u8; 12usize],
    #[doc = "0x510 - APBH DMA Channel n Command Register"]
    pub ch9_cmd: CH_CMD,
    _reserved68: [u8; 12usize],
    #[doc = "0x520 - APBH DMA Channel n Buffer Address Register"]
    pub ch9_bar: CH_BAR,
    _reserved69: [u8; 12usize],
    #[doc = "0x530 - APBH DMA Channel n Semaphore Register"]
    pub ch9_sema: CH_SEMA,
    _reserved70: [u8; 12usize],
    #[doc = "0x540 - AHB to APBH DMA Channel n Debug Information"]
    pub ch9_debug1: CH_DEBUG1,
    _reserved71: [u8; 12usize],
    #[doc = "0x550 - AHB to APBH DMA Channel n Debug Information"]
    pub ch9_debug2: CH_DEBUG2,
    _reserved72: [u8; 12usize],
    #[doc = "0x560 - APBH DMA Channel n Current Command Address Register"]
    pub ch10_curcmdar: CH_CURCMDAR,
    _reserved73: [u8; 12usize],
    #[doc = "0x570 - APBH DMA Channel n Next Command Address Register"]
    pub ch10_nxtcmdar: CH_NXTCMDAR,
    _reserved74: [u8; 12usize],
    #[doc = "0x580 - APBH DMA Channel n Command Register"]
    pub ch10_cmd: CH_CMD,
    _reserved75: [u8; 12usize],
    #[doc = "0x590 - APBH DMA Channel n Buffer Address Register"]
    pub ch10_bar: CH_BAR,
    _reserved76: [u8; 12usize],
    #[doc = "0x5a0 - APBH DMA Channel n Semaphore Register"]
    pub ch10_sema: CH_SEMA,
    _reserved77: [u8; 12usize],
    #[doc = "0x5b0 - AHB to APBH DMA Channel n Debug Information"]
    pub ch10_debug1: CH_DEBUG1,
    _reserved78: [u8; 12usize],
    #[doc = "0x5c0 - AHB to APBH DMA Channel n Debug Information"]
    pub ch10_debug2: CH_DEBUG2,
    _reserved79: [u8; 12usize],
    #[doc = "0x5d0 - APBH DMA Channel n Current Command Address Register"]
    pub ch11_curcmdar: CH_CURCMDAR,
    _reserved80: [u8; 12usize],
    #[doc = "0x5e0 - APBH DMA Channel n Next Command Address Register"]
    pub ch11_nxtcmdar: CH_NXTCMDAR,
    _reserved81: [u8; 12usize],
    #[doc = "0x5f0 - APBH DMA Channel n Command Register"]
    pub ch11_cmd: CH_CMD,
    _reserved82: [u8; 12usize],
    #[doc = "0x600 - APBH DMA Channel n Buffer Address Register"]
    pub ch11_bar: CH_BAR,
    _reserved83: [u8; 12usize],
    #[doc = "0x610 - APBH DMA Channel n Semaphore Register"]
    pub ch11_sema: CH_SEMA,
    _reserved84: [u8; 12usize],
    #[doc = "0x620 - AHB to APBH DMA Channel n Debug Information"]
    pub ch11_debug1: CH_DEBUG1,
    _reserved85: [u8; 12usize],
    #[doc = "0x630 - AHB to APBH DMA Channel n Debug Information"]
    pub ch11_debug2: CH_DEBUG2,
    _reserved86: [u8; 12usize],
    #[doc = "0x640 - APBH DMA Channel n Current Command Address Register"]
    pub ch12_curcmdar: CH_CURCMDAR,
    _reserved87: [u8; 12usize],
    #[doc = "0x650 - APBH DMA Channel n Next Command Address Register"]
    pub ch12_nxtcmdar: CH_NXTCMDAR,
    _reserved88: [u8; 12usize],
    #[doc = "0x660 - APBH DMA Channel n Command Register"]
    pub ch12_cmd: CH_CMD,
    _reserved89: [u8; 12usize],
    #[doc = "0x670 - APBH DMA Channel n Buffer Address Register"]
    pub ch12_bar: CH_BAR,
    _reserved90: [u8; 12usize],
    #[doc = "0x680 - APBH DMA Channel n Semaphore Register"]
    pub ch12_sema: CH_SEMA,
    _reserved91: [u8; 12usize],
    #[doc = "0x690 - AHB to APBH DMA Channel n Debug Information"]
    pub ch12_debug1: CH_DEBUG1,
    _reserved92: [u8; 12usize],
    #[doc = "0x6a0 - AHB to APBH DMA Channel n Debug Information"]
    pub ch12_debug2: CH_DEBUG2,
    _reserved93: [u8; 12usize],
    #[doc = "0x6b0 - APBH DMA Channel n Current Command Address Register"]
    pub ch13_curcmdar: CH_CURCMDAR,
    _reserved94: [u8; 12usize],
    #[doc = "0x6c0 - APBH DMA Channel n Next Command Address Register"]
    pub ch13_nxtcmdar: CH_NXTCMDAR,
    _reserved95: [u8; 12usize],
    #[doc = "0x6d0 - APBH DMA Channel n Command Register"]
    pub ch13_cmd: CH_CMD,
    _reserved96: [u8; 12usize],
    #[doc = "0x6e0 - APBH DMA Channel n Buffer Address Register"]
    pub ch13_bar: CH_BAR,
    _reserved97: [u8; 12usize],
    #[doc = "0x6f0 - APBH DMA Channel n Semaphore Register"]
    pub ch13_sema: CH_SEMA,
    _reserved98: [u8; 12usize],
    #[doc = "0x700 - AHB to APBH DMA Channel n Debug Information"]
    pub ch13_debug1: CH_DEBUG1,
    _reserved99: [u8; 12usize],
    #[doc = "0x710 - AHB to APBH DMA Channel n Debug Information"]
    pub ch13_debug2: CH_DEBUG2,
    _reserved100: [u8; 12usize],
    #[doc = "0x720 - APBH DMA Channel n Current Command Address Register"]
    pub ch14_curcmdar: CH_CURCMDAR,
    _reserved101: [u8; 12usize],
    #[doc = "0x730 - APBH DMA Channel n Next Command Address Register"]
    pub ch14_nxtcmdar: CH_NXTCMDAR,
    _reserved102: [u8; 12usize],
    #[doc = "0x740 - APBH DMA Channel n Command Register"]
    pub ch14_cmd: CH_CMD,
    _reserved103: [u8; 12usize],
    #[doc = "0x750 - APBH DMA Channel n Buffer Address Register"]
    pub ch14_bar: CH_BAR,
    _reserved104: [u8; 12usize],
    #[doc = "0x760 - APBH DMA Channel n Semaphore Register"]
    pub ch14_sema: CH_SEMA,
    _reserved105: [u8; 12usize],
    #[doc = "0x770 - AHB to APBH DMA Channel n Debug Information"]
    pub ch14_debug1: CH_DEBUG1,
    _reserved106: [u8; 12usize],
    #[doc = "0x780 - AHB to APBH DMA Channel n Debug Information"]
    pub ch14_debug2: CH_DEBUG2,
    _reserved107: [u8; 12usize],
    #[doc = "0x790 - APBH DMA Channel n Current Command Address Register"]
    pub ch15_curcmdar: CH_CURCMDAR,
    _reserved108: [u8; 12usize],
    #[doc = "0x7a0 - APBH DMA Channel n Next Command Address Register"]
    pub ch15_nxtcmdar: CH_NXTCMDAR,
    _reserved109: [u8; 12usize],
    #[doc = "0x7b0 - APBH DMA Channel n Command Register"]
    pub ch15_cmd: CH_CMD,
    _reserved110: [u8; 12usize],
    #[doc = "0x7c0 - APBH DMA Channel n Buffer Address Register"]
    pub ch15_bar: CH_BAR,
    _reserved111: [u8; 12usize],
    #[doc = "0x7d0 - APBH DMA Channel n Semaphore Register"]
    pub ch15_sema: CH_SEMA,
    _reserved112: [u8; 12usize],
    #[doc = "0x7e0 - AHB to APBH DMA Channel n Debug Information"]
    pub ch15_debug1: CH_DEBUG1,
    _reserved113: [u8; 12usize],
    #[doc = "0x7f0 - AHB to APBH DMA Channel n Debug Information"]
    pub ch15_debug2: CH_DEBUG2,
    _reserved114: [u8; 12usize],
    #[doc = "0x800 - APBH Bridge Version Register"]
    pub version: VERSION,
}
#[doc = "AHB to APBH Bridge Control and Status Register 0"]
pub struct CTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB to APBH Bridge Control and Status Register 0"]
pub mod ctrl0;
#[doc = "AHB to APBH Bridge Control and Status Register 0"]
pub struct CTRL0_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB to APBH Bridge Control and Status Register 0"]
pub mod ctrl0_set;
#[doc = "AHB to APBH Bridge Control and Status Register 0"]
pub struct CTRL0_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB to APBH Bridge Control and Status Register 0"]
pub mod ctrl0_clr;
#[doc = "AHB to APBH Bridge Control and Status Register 0"]
pub struct CTRL0_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB to APBH Bridge Control and Status Register 0"]
pub mod ctrl0_tog;
#[doc = "AHB to APBH Bridge Control and Status Register 1"]
pub struct CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB to APBH Bridge Control and Status Register 1"]
pub mod ctrl1;
#[doc = "AHB to APBH Bridge Control and Status Register 1"]
pub struct CTRL1_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB to APBH Bridge Control and Status Register 1"]
pub mod ctrl1_set;
#[doc = "AHB to APBH Bridge Control and Status Register 1"]
pub struct CTRL1_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB to APBH Bridge Control and Status Register 1"]
pub mod ctrl1_clr;
#[doc = "AHB to APBH Bridge Control and Status Register 1"]
pub struct CTRL1_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB to APBH Bridge Control and Status Register 1"]
pub mod ctrl1_tog;
#[doc = "AHB to APBH Bridge Control and Status Register 2"]
pub struct CTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB to APBH Bridge Control and Status Register 2"]
pub mod ctrl2;
#[doc = "AHB to APBH Bridge Control and Status Register 2"]
pub struct CTRL2_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB to APBH Bridge Control and Status Register 2"]
pub mod ctrl2_set;
#[doc = "AHB to APBH Bridge Control and Status Register 2"]
pub struct CTRL2_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB to APBH Bridge Control and Status Register 2"]
pub mod ctrl2_clr;
#[doc = "AHB to APBH Bridge Control and Status Register 2"]
pub struct CTRL2_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB to APBH Bridge Control and Status Register 2"]
pub mod ctrl2_tog;
#[doc = "AHB to APBH Bridge Channel Register"]
pub struct CHANNEL_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB to APBH Bridge Channel Register"]
pub mod channel_ctrl;
#[doc = "AHB to APBH Bridge Channel Register"]
pub struct CHANNEL_CTRL_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB to APBH Bridge Channel Register"]
pub mod channel_ctrl_set;
#[doc = "AHB to APBH Bridge Channel Register"]
pub struct CHANNEL_CTRL_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB to APBH Bridge Channel Register"]
pub mod channel_ctrl_clr;
#[doc = "AHB to APBH Bridge Channel Register"]
pub struct CHANNEL_CTRL_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB to APBH Bridge Channel Register"]
pub mod channel_ctrl_tog;
#[doc = "AHB to APBH DMA Device Assignment Register"]
pub struct DEVSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB to APBH DMA Device Assignment Register"]
pub mod devsel;
#[doc = "AHB to APBH DMA burst size"]
pub struct DMA_BURST_SIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB to APBH DMA burst size"]
pub mod dma_burst_size;
#[doc = "AHB to APBH DMA Debug Register"]
pub struct DEBUG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB to APBH DMA Debug Register"]
pub mod debug;
#[doc = "APBH DMA Channel n Current Command Address Register"]
pub struct CH_CURCMDAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APBH DMA Channel n Current Command Address Register"]
pub mod ch_curcmdar;
#[doc = "APBH DMA Channel n Next Command Address Register"]
pub struct CH_NXTCMDAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APBH DMA Channel n Next Command Address Register"]
pub mod ch_nxtcmdar;
#[doc = "APBH DMA Channel n Command Register"]
pub struct CH_CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APBH DMA Channel n Command Register"]
pub mod ch_cmd;
#[doc = "APBH DMA Channel n Buffer Address Register"]
pub struct CH_BAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APBH DMA Channel n Buffer Address Register"]
pub mod ch_bar;
#[doc = "APBH DMA Channel n Semaphore Register"]
pub struct CH_SEMA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APBH DMA Channel n Semaphore Register"]
pub mod ch_sema;
#[doc = "AHB to APBH DMA Channel n Debug Information"]
pub struct CH_DEBUG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB to APBH DMA Channel n Debug Information"]
pub mod ch_debug1;
#[doc = "AHB to APBH DMA Channel n Debug Information"]
pub struct CH_DEBUG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB to APBH DMA Channel n Debug Information"]
pub mod ch_debug2;
#[doc = "APBH Bridge Version Register"]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APBH Bridge Version Register"]
pub mod version;
