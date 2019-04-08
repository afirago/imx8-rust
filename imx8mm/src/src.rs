#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRC Reset Control Register"]
    pub scr: SCR,
    #[doc = "0x04 - A53 Reset Control Register"]
    pub a53rcr0: A53RCR0,
    #[doc = "0x08 - A53 Reset Control Register"]
    pub a53rcr1: A53RCR1,
    #[doc = "0x0c - M4 Reset Control Register"]
    pub m4rcr: M4RCR,
    _reserved0: [u8; 16usize],
    #[doc = "0x20 - USB OTG PHY1 Reset Control Register"]
    pub usbophy1_rcr: USBOPHY1_RCR,
    #[doc = "0x24 - USB OTG PHY2 Reset Control Register"]
    pub usbophy2_rcr: USBOPHY2_RCR,
    #[doc = "0x28 - MIPI PHY Reset Control Register"]
    pub mipiphy_rcr: MIPIPHY_RCR,
    #[doc = "0x2c - PCIE PHY Reset Control Register"]
    pub pciephy_rcr: PCIEPHY_RCR,
    _reserved1: [u8; 4usize],
    #[doc = "0x34 - DISPLAY Reset Control Register"]
    pub disp_rcr: DISP_RCR,
    _reserved2: [u8; 8usize],
    #[doc = "0x40 - GPU Reset Control Register"]
    pub gpu_rcr: GPU_RCR,
    #[doc = "0x44 - VPU Reset Control Register"]
    pub vpu_rcr: VPU_RCR,
    _reserved3: [u8; 16usize],
    #[doc = "0x58 - SRC Boot Mode Register 1"]
    pub sbmr1: SBMR1,
    #[doc = "0x5c - SRC Reset Status Register"]
    pub srsr: SRSR,
    _reserved4: [u8; 8usize],
    #[doc = "0x68 - SRC Interrupt Status Register"]
    pub sisr: SISR,
    #[doc = "0x6c - SRC Interrupt Mask Register"]
    pub simr: SIMR,
    #[doc = "0x70 - SRC Boot Mode Register 2"]
    pub sbmr2: SBMR2,
    #[doc = "0x74 - SRC General Purpose Register 1"]
    pub gpr1: GPR1,
    #[doc = "0x78 - SRC General Purpose Register 2"]
    pub gpr2: GPR2,
    #[doc = "0x7c - SRC General Purpose Register 3"]
    pub gpr3: GPR3,
    #[doc = "0x80 - SRC General Purpose Register 4"]
    pub gpr4: GPR4,
    #[doc = "0x84 - SRC General Purpose Register 5"]
    pub gpr5: GPR5,
    #[doc = "0x88 - SRC General Purpose Register 6"]
    pub gpr6: GPR6,
    #[doc = "0x8c - SRC General Purpose Register 7"]
    pub gpr7: GPR7,
    #[doc = "0x90 - SRC General Purpose Register 8"]
    pub gpr8: GPR8,
    #[doc = "0x94 - SRC General Purpose Register 9"]
    pub gpr9: GPR9,
    #[doc = "0x98 - SRC General Purpose Register 10"]
    pub gpr10: GPR10,
    _reserved5: [u8; 3940usize],
    #[doc = "0x1000 - SRC DDR Controller Reset Control Register"]
    pub ddrc_rcr: DDRC_RCR,
}
#[doc = "SRC Reset Control Register"]
pub struct SCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC Reset Control Register"]
pub mod scr;
#[doc = "A53 Reset Control Register"]
pub struct A53RCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A53 Reset Control Register"]
pub mod a53rcr0;
#[doc = "A53 Reset Control Register"]
pub struct A53RCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A53 Reset Control Register"]
pub mod a53rcr1;
#[doc = "M4 Reset Control Register"]
pub struct M4RCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "M4 Reset Control Register"]
pub mod m4rcr;
#[doc = "USB OTG PHY1 Reset Control Register"]
pub struct USBOPHY1_RCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB OTG PHY1 Reset Control Register"]
pub mod usbophy1_rcr;
#[doc = "USB OTG PHY2 Reset Control Register"]
pub struct USBOPHY2_RCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB OTG PHY2 Reset Control Register"]
pub mod usbophy2_rcr;
#[doc = "MIPI PHY Reset Control Register"]
pub struct MIPIPHY_RCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MIPI PHY Reset Control Register"]
pub mod mipiphy_rcr;
#[doc = "PCIE PHY Reset Control Register"]
pub struct PCIEPHY_RCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCIE PHY Reset Control Register"]
pub mod pciephy_rcr;
#[doc = "DISPLAY Reset Control Register"]
pub struct DISP_RCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DISPLAY Reset Control Register"]
pub mod disp_rcr;
#[doc = "GPU Reset Control Register"]
pub struct GPU_RCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPU Reset Control Register"]
pub mod gpu_rcr;
#[doc = "VPU Reset Control Register"]
pub struct VPU_RCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VPU Reset Control Register"]
pub mod vpu_rcr;
#[doc = "SRC Boot Mode Register 1"]
pub struct SBMR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC Boot Mode Register 1"]
pub mod sbmr1;
#[doc = "SRC Reset Status Register"]
pub struct SRSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC Reset Status Register"]
pub mod srsr;
#[doc = "SRC Interrupt Status Register"]
pub struct SISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC Interrupt Status Register"]
pub mod sisr;
#[doc = "SRC Interrupt Mask Register"]
pub struct SIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC Interrupt Mask Register"]
pub mod simr;
#[doc = "SRC Boot Mode Register 2"]
pub struct SBMR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC Boot Mode Register 2"]
pub mod sbmr2;
#[doc = "SRC General Purpose Register 1"]
pub struct GPR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC General Purpose Register 1"]
pub mod gpr1;
#[doc = "SRC General Purpose Register 2"]
pub struct GPR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC General Purpose Register 2"]
pub mod gpr2;
#[doc = "SRC General Purpose Register 3"]
pub struct GPR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC General Purpose Register 3"]
pub mod gpr3;
#[doc = "SRC General Purpose Register 4"]
pub struct GPR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC General Purpose Register 4"]
pub mod gpr4;
#[doc = "SRC General Purpose Register 5"]
pub struct GPR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC General Purpose Register 5"]
pub mod gpr5;
#[doc = "SRC General Purpose Register 6"]
pub struct GPR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC General Purpose Register 6"]
pub mod gpr6;
#[doc = "SRC General Purpose Register 7"]
pub struct GPR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC General Purpose Register 7"]
pub mod gpr7;
#[doc = "SRC General Purpose Register 8"]
pub struct GPR8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC General Purpose Register 8"]
pub mod gpr8;
#[doc = "SRC General Purpose Register 9"]
pub struct GPR9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC General Purpose Register 9"]
pub mod gpr9;
#[doc = "SRC General Purpose Register 10"]
pub struct GPR10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC General Purpose Register 10"]
pub mod gpr10;
#[doc = "SRC DDR Controller Reset Control Register"]
pub struct DDRC_RCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRC DDR Controller Reset Control Register"]
pub mod ddrc_rcr;
