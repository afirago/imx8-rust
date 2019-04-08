#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Basic Low power control register of A53 platform"]
    pub lpcr_a53_bsc: LPCR_A53_BSC,
    #[doc = "0x04 - Advanced Low power control register of A53 platform"]
    pub lpcr_a53_ad: LPCR_A53_AD,
    #[doc = "0x08 - Low power control register of CPU1"]
    pub lpcr_m4: LPCR_M4,
    _reserved0: [u8; 8usize],
    #[doc = "0x14 - System low power control register"]
    pub slpcr: SLPCR,
    #[doc = "0x18 - MASTER LPM Handshake"]
    pub mst_cpu_mapping: MST_CPU_MAPPING,
    _reserved1: [u8; 4usize],
    #[doc = "0x20 - Memory low power control register"]
    pub mlpcr: MLPCR,
    #[doc = "0x24 - PGC acknowledge signal selection of A53 platform"]
    pub pgc_ack_sel_a53: PGC_ACK_SEL_A53,
    #[doc = "0x28 - PGC acknowledge signal selection of M4 platform"]
    pub pgc_ack_sel_m4: PGC_ACK_SEL_M4,
    #[doc = "0x2c - GPC Miscellaneous register"]
    pub misc: MISC,
    #[doc = "0x30 - IRQ masking register 1 of A53 core0"]
    pub imr1_core0_a53: IMR1_CORE0_A53,
    #[doc = "0x34 - IRQ masking register 2 of A53 core0"]
    pub imr2_core0_a53: IMR2_CORE0_A53,
    #[doc = "0x38 - IRQ masking register 3 of A53 core0"]
    pub imr3_core0_a53: IMR3_CORE0_A53,
    #[doc = "0x3c - IRQ masking register 4 of A53 core0"]
    pub imr4_core0_a53: IMR4_CORE0_A53,
    #[doc = "0x40 - IRQ masking register 1 of A53 core1"]
    pub imr1_core1_a53: IMR1_CORE1_A53,
    #[doc = "0x44 - IRQ masking register 2 of A53 core1"]
    pub imr2_core1_a53: IMR2_CORE1_A53,
    #[doc = "0x48 - IRQ masking register 3 of A53 core1"]
    pub imr3_core1_a53: IMR3_CORE1_A53,
    #[doc = "0x4c - IRQ masking register 4 of A53 core1"]
    pub imr4_core1_a53: IMR4_CORE1_A53,
    #[doc = "0x50 - IRQ masking register 1 of M4"]
    pub imr1_m4: IMR1_M4,
    #[doc = "0x54 - IRQ masking register 2 of M4"]
    pub imr2_m4: IMR2_M4,
    #[doc = "0x58 - IRQ masking register 3 of M4"]
    pub imr3_m4: IMR3_M4,
    #[doc = "0x5c - IRQ masking register 4 of M4"]
    pub imr4_m4: IMR4_M4,
    _reserved2: [u8; 16usize],
    #[doc = "0x70 - IRQ status register 1 of A53"]
    pub isr1_a53: ISR1_A53,
    #[doc = "0x74 - IRQ status register 2 of A53"]
    pub isr2_a53: ISR2_A53,
    #[doc = "0x78 - IRQ status register 3 of A53"]
    pub isr3_a53: ISR3_A53,
    #[doc = "0x7c - IRQ status register 4 of A53"]
    pub isr4_a53: ISR4_A53,
    #[doc = "0x80 - IRQ status register 1 of M4"]
    pub isr1_m4: ISR1_M4,
    #[doc = "0x84 - IRQ status register 2 of M4"]
    pub isr2_m4: ISR2_M4,
    #[doc = "0x88 - IRQ status register 3 of M4"]
    pub isr3_m4: ISR3_M4,
    #[doc = "0x8c - IRQ status register 4 of M4"]
    pub isr4_m4: ISR4_M4,
    _reserved3: [u8; 32usize],
    #[doc = "0xb0 - Slot configure register for A53 core"]
    pub slt0_cfg: SLT0_CFG,
    #[doc = "0xb4 - Slot configure register for A53 core"]
    pub slt1_cfg: SLT1_CFG,
    #[doc = "0xb8 - Slot configure register for A53 core"]
    pub slt2_cfg: SLT2_CFG,
    #[doc = "0xbc - Slot configure register for A53 core"]
    pub slt3_cfg: SLT3_CFG,
    #[doc = "0xc0 - Slot configure register for A53 core"]
    pub slt4_cfg: SLT4_CFG,
    #[doc = "0xc4 - Slot configure register for A53 core"]
    pub slt5_cfg: SLT5_CFG,
    #[doc = "0xc8 - Slot configure register for A53 core"]
    pub slt6_cfg: SLT6_CFG,
    #[doc = "0xcc - Slot configure register for A53 core"]
    pub slt7_cfg: SLT7_CFG,
    #[doc = "0xd0 - Slot configure register for A53 core"]
    pub slt8_cfg: SLT8_CFG,
    #[doc = "0xd4 - Slot configure register for A53 core"]
    pub slt9_cfg: SLT9_CFG,
    #[doc = "0xd8 - Slot configure register for A53 core"]
    pub slt10_cfg: SLT10_CFG,
    #[doc = "0xdc - Slot configure register for A53 core"]
    pub slt11_cfg: SLT11_CFG,
    #[doc = "0xe0 - Slot configure register for A53 core"]
    pub slt12_cfg: SLT12_CFG,
    #[doc = "0xe4 - Slot configure register for A53 core"]
    pub slt13_cfg: SLT13_CFG,
    #[doc = "0xe8 - Slot configure register for A53 core"]
    pub slt14_cfg: SLT14_CFG,
    #[doc = "0xec - PGC CPU mapping"]
    pub pgc_cpu_0_1_mapping: PGC_CPU_0_1_MAPPING,
    #[doc = "0xf0 - CPU PGC software power up trigger"]
    pub cpu_pgc_sw_pup_req: CPU_PGC_SW_PUP_REQ,
    #[doc = "0xf4 - MIX PGC software power up trigger"]
    pub mix_pgc_sw_pup_req: MIX_PGC_SW_PUP_REQ,
    #[doc = "0xf8 - PU PGC software up trigger"]
    pub pu_pgc_sw_pup_req: PU_PGC_SW_PUP_REQ,
    #[doc = "0xfc - CPU PGC software down trigger"]
    pub cpu_pgc_sw_pdn_req: CPU_PGC_SW_PDN_REQ,
    #[doc = "0x100 - MIX PGC software power down trigger"]
    pub mix_pgc_sw_pdn_req: MIX_PGC_SW_PDN_REQ,
    #[doc = "0x104 - PU PGC software down trigger"]
    pub pu_pgc_sw_pdn_req: PU_PGC_SW_PDN_REQ,
    #[doc = "0x108 - Basic Low power control register of A53 platform"]
    pub lpcr_a53_bsc2: LPCR_A53_BSC2,
    _reserved4: [u8; 36usize],
    #[doc = "0x130 - CPU PGC software up trigger status1"]
    pub cpu_pgc_pup_status1: CPU_PGC_PUP_STATUS1,
    #[doc = "0x134 - A53 MIX software up trigger status register"]
    pub a53_mix_pgc_pup_status: [A53_MIX_PGC_PUP_STATUS; 3],
    #[doc = "0x140 - M4 MIX PGC software up trigger status register"]
    pub m4_mix_pgc_pup_status: [M4_MIX_PGC_PUP_STATUS; 3],
    #[doc = "0x14c - A53 PU software up trigger status register"]
    pub a53_pu_pgc_pup_status: [A53_PU_PGC_PUP_STATUS; 3],
    #[doc = "0x158 - M4 PU PGC software up trigger status register"]
    pub m4_pu_pgc_pup_status: [M4_PU_PGC_PUP_STATUS; 3],
    _reserved5: [u8; 12usize],
    #[doc = "0x170 - CPU PGC software dn trigger status1"]
    pub cpu_pgc_pdn_status1: CPU_PGC_PDN_STATUS1,
    #[doc = "0x174 - A53 MIX software down trigger status register"]
    pub a53_mix_pgc_pdn_status: [A53_MIX_PGC_PDN_STATUS; 3],
    #[doc = "0x180 - M4 MIX PGC software power down trigger status register"]
    pub m4_mix_pgc_pdn_status: [M4_MIX_PGC_PDN_STATUS; 3],
    #[doc = "0x18c - A53 PU PGC software down trigger status"]
    pub a53_pu_pgc_pdn_status: [A53_PU_PGC_PDN_STATUS; 3],
    #[doc = "0x198 - M4 PU PGC software down trigger status"]
    pub m4_pu_pgc_pdn_status: [M4_PU_PGC_PDN_STATUS; 3],
    _reserved6: [u8; 12usize],
    #[doc = "0x1b0 - A53 MIX PDN FLG"]
    pub a53_mix_pdn_flg: A53_MIX_PDN_FLG,
    #[doc = "0x1b4 - A53 PU PDN FLG"]
    pub a53_pu_pdn_flg: A53_PU_PDN_FLG,
    #[doc = "0x1b8 - M4 MIX PDN FLG"]
    pub m4_mix_pdn_flg: M4_MIX_PDN_FLG,
    #[doc = "0x1bc - M4 PU PDN FLG"]
    pub m4_pu_pdn_flg: M4_PU_PDN_FLG,
    #[doc = "0x1c0 - IRQ masking register 1 of A53 core2"]
    pub imr1_core2_a53: IMR1_CORE2_A53,
    #[doc = "0x1c4 - IRQ masking register 2 of A53 core2"]
    pub imr2_core2_a53: IMR2_CORE2_A53,
    #[doc = "0x1c8 - IRQ masking register 3 of A53 core2"]
    pub imr3_core2_a53: IMR3_CORE2_A53,
    #[doc = "0x1cc - IRQ masking register 4 of A53 core2"]
    pub imr4_core2_a53: IMR4_CORE2_A53,
    #[doc = "0x1d0 - IRQ masking register 1 of A53 core3"]
    pub imr1_core3_a53: IMR1_CORE3_A53,
    #[doc = "0x1d4 - IRQ masking register 2 of A53 core3"]
    pub imr2_core3_a53: IMR2_CORE3_A53,
    #[doc = "0x1d8 - IRQ masking register 3 of A53 core3"]
    pub imr3_core3_a53: IMR3_CORE3_A53,
    #[doc = "0x1dc - IRQ masking register 4 of A53 core3"]
    pub imr4_core3_a53: IMR4_CORE3_A53,
    #[doc = "0x1e0 - PGC acknowledge signal selection of A53 platform for PUs"]
    pub ack_sel_a53_pu: ACK_SEL_A53_PU,
    #[doc = "0x1e4 - PGC acknowledge signal selection of M4 platform for PUs"]
    pub ack_sel_m4_pu: ACK_SEL_M4_PU,
    #[doc = "0x1e8 - Slot configure register for A53 core"]
    pub slt15_cfg: SLT15_CFG,
    #[doc = "0x1ec - Slot configure register for A53 core"]
    pub slt16_cfg: SLT16_CFG,
    #[doc = "0x1f0 - Slot configure register for A53 core"]
    pub slt17_cfg: SLT17_CFG,
    #[doc = "0x1f4 - Slot configure register for A53 core"]
    pub slt18_cfg: SLT18_CFG,
    #[doc = "0x1f8 - Slot configure register for A53 core"]
    pub slt19_cfg: SLT19_CFG,
    #[doc = "0x1fc - Power handshake register"]
    pub pu_pwrhsk: PU_PWRHSK,
    #[doc = "0x200 - Slot configure register for PUs"]
    pub slt_cfg_pu: [SLT_CFG_PU; 20],
}
#[doc = "Basic Low power control register of A53 platform"]
pub struct LPCR_A53_BSC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Basic Low power control register of A53 platform"]
pub mod lpcr_a53_bsc;
#[doc = "Advanced Low power control register of A53 platform"]
pub struct LPCR_A53_AD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Advanced Low power control register of A53 platform"]
pub mod lpcr_a53_ad;
#[doc = "Low power control register of CPU1"]
pub struct LPCR_M4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low power control register of CPU1"]
pub mod lpcr_m4;
#[doc = "System low power control register"]
pub struct SLPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System low power control register"]
pub mod slpcr;
#[doc = "MASTER LPM Handshake"]
pub struct MST_CPU_MAPPING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MASTER LPM Handshake"]
pub mod mst_cpu_mapping;
#[doc = "Memory low power control register"]
pub struct MLPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory low power control register"]
pub mod mlpcr;
#[doc = "PGC acknowledge signal selection of A53 platform"]
pub struct PGC_ACK_SEL_A53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PGC acknowledge signal selection of A53 platform"]
pub mod pgc_ack_sel_a53;
#[doc = "PGC acknowledge signal selection of M4 platform"]
pub struct PGC_ACK_SEL_M4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PGC acknowledge signal selection of M4 platform"]
pub mod pgc_ack_sel_m4;
#[doc = "GPC Miscellaneous register"]
pub struct MISC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC Miscellaneous register"]
pub mod misc;
#[doc = "IRQ masking register 1 of A53 core0"]
pub struct IMR1_CORE0_A53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ masking register 1 of A53 core0"]
pub mod imr1_core0_a53;
#[doc = "IRQ masking register 2 of A53 core0"]
pub struct IMR2_CORE0_A53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ masking register 2 of A53 core0"]
pub mod imr2_core0_a53;
#[doc = "IRQ masking register 3 of A53 core0"]
pub struct IMR3_CORE0_A53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ masking register 3 of A53 core0"]
pub mod imr3_core0_a53;
#[doc = "IRQ masking register 4 of A53 core0"]
pub struct IMR4_CORE0_A53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ masking register 4 of A53 core0"]
pub mod imr4_core0_a53;
#[doc = "IRQ masking register 1 of A53 core1"]
pub struct IMR1_CORE1_A53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ masking register 1 of A53 core1"]
pub mod imr1_core1_a53;
#[doc = "IRQ masking register 2 of A53 core1"]
pub struct IMR2_CORE1_A53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ masking register 2 of A53 core1"]
pub mod imr2_core1_a53;
#[doc = "IRQ masking register 3 of A53 core1"]
pub struct IMR3_CORE1_A53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ masking register 3 of A53 core1"]
pub mod imr3_core1_a53;
#[doc = "IRQ masking register 4 of A53 core1"]
pub struct IMR4_CORE1_A53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ masking register 4 of A53 core1"]
pub mod imr4_core1_a53;
#[doc = "IRQ masking register 1 of M4"]
pub struct IMR1_M4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ masking register 1 of M4"]
pub mod imr1_m4;
#[doc = "IRQ masking register 2 of M4"]
pub struct IMR2_M4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ masking register 2 of M4"]
pub mod imr2_m4;
#[doc = "IRQ masking register 3 of M4"]
pub struct IMR3_M4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ masking register 3 of M4"]
pub mod imr3_m4;
#[doc = "IRQ masking register 4 of M4"]
pub struct IMR4_M4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ masking register 4 of M4"]
pub mod imr4_m4;
#[doc = "IRQ status register 1 of A53"]
pub struct ISR1_A53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ status register 1 of A53"]
pub mod isr1_a53;
#[doc = "IRQ status register 2 of A53"]
pub struct ISR2_A53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ status register 2 of A53"]
pub mod isr2_a53;
#[doc = "IRQ status register 3 of A53"]
pub struct ISR3_A53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ status register 3 of A53"]
pub mod isr3_a53;
#[doc = "IRQ status register 4 of A53"]
pub struct ISR4_A53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ status register 4 of A53"]
pub mod isr4_a53;
#[doc = "IRQ status register 1 of M4"]
pub struct ISR1_M4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ status register 1 of M4"]
pub mod isr1_m4;
#[doc = "IRQ status register 2 of M4"]
pub struct ISR2_M4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ status register 2 of M4"]
pub mod isr2_m4;
#[doc = "IRQ status register 3 of M4"]
pub struct ISR3_M4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ status register 3 of M4"]
pub mod isr3_m4;
#[doc = "IRQ status register 4 of M4"]
pub struct ISR4_M4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ status register 4 of M4"]
pub mod isr4_m4;
#[doc = "Slot configure register for A53 core"]
pub struct SLT0_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot configure register for A53 core"]
pub mod slt0_cfg;
#[doc = "Slot configure register for A53 core"]
pub struct SLT1_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot configure register for A53 core"]
pub mod slt1_cfg;
#[doc = "Slot configure register for A53 core"]
pub struct SLT2_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot configure register for A53 core"]
pub mod slt2_cfg;
#[doc = "Slot configure register for A53 core"]
pub struct SLT3_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot configure register for A53 core"]
pub mod slt3_cfg;
#[doc = "Slot configure register for A53 core"]
pub struct SLT4_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot configure register for A53 core"]
pub mod slt4_cfg;
#[doc = "Slot configure register for A53 core"]
pub struct SLT5_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot configure register for A53 core"]
pub mod slt5_cfg;
#[doc = "Slot configure register for A53 core"]
pub struct SLT6_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot configure register for A53 core"]
pub mod slt6_cfg;
#[doc = "Slot configure register for A53 core"]
pub struct SLT7_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot configure register for A53 core"]
pub mod slt7_cfg;
#[doc = "Slot configure register for A53 core"]
pub struct SLT8_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot configure register for A53 core"]
pub mod slt8_cfg;
#[doc = "Slot configure register for A53 core"]
pub struct SLT9_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot configure register for A53 core"]
pub mod slt9_cfg;
#[doc = "Slot configure register for A53 core"]
pub struct SLT10_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot configure register for A53 core"]
pub mod slt10_cfg;
#[doc = "Slot configure register for A53 core"]
pub struct SLT11_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot configure register for A53 core"]
pub mod slt11_cfg;
#[doc = "Slot configure register for A53 core"]
pub struct SLT12_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot configure register for A53 core"]
pub mod slt12_cfg;
#[doc = "Slot configure register for A53 core"]
pub struct SLT13_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot configure register for A53 core"]
pub mod slt13_cfg;
#[doc = "Slot configure register for A53 core"]
pub struct SLT14_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot configure register for A53 core"]
pub mod slt14_cfg;
#[doc = "PGC CPU mapping"]
pub struct PGC_CPU_0_1_MAPPING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PGC CPU mapping"]
pub mod pgc_cpu_0_1_mapping;
#[doc = "CPU PGC software power up trigger"]
pub struct CPU_PGC_SW_PUP_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU PGC software power up trigger"]
pub mod cpu_pgc_sw_pup_req;
#[doc = "MIX PGC software power up trigger"]
pub struct MIX_PGC_SW_PUP_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MIX PGC software power up trigger"]
pub mod mix_pgc_sw_pup_req;
#[doc = "PU PGC software up trigger"]
pub struct PU_PGC_SW_PUP_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PU PGC software up trigger"]
pub mod pu_pgc_sw_pup_req;
#[doc = "CPU PGC software down trigger"]
pub struct CPU_PGC_SW_PDN_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU PGC software down trigger"]
pub mod cpu_pgc_sw_pdn_req;
#[doc = "MIX PGC software power down trigger"]
pub struct MIX_PGC_SW_PDN_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MIX PGC software power down trigger"]
pub mod mix_pgc_sw_pdn_req;
#[doc = "PU PGC software down trigger"]
pub struct PU_PGC_SW_PDN_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PU PGC software down trigger"]
pub mod pu_pgc_sw_pdn_req;
#[doc = "Basic Low power control register of A53 platform"]
pub struct LPCR_A53_BSC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Basic Low power control register of A53 platform"]
pub mod lpcr_a53_bsc2;
#[doc = "CPU PGC software up trigger status1"]
pub struct CPU_PGC_PUP_STATUS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU PGC software up trigger status1"]
pub mod cpu_pgc_pup_status1;
#[doc = "A53 MIX software up trigger status register"]
pub struct A53_MIX_PGC_PUP_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A53 MIX software up trigger status register"]
pub mod a53_mix_pgc_pup_status;
#[doc = "M4 MIX PGC software up trigger status register"]
pub struct M4_MIX_PGC_PUP_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "M4 MIX PGC software up trigger status register"]
pub mod m4_mix_pgc_pup_status;
#[doc = "A53 PU software up trigger status register"]
pub struct A53_PU_PGC_PUP_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A53 PU software up trigger status register"]
pub mod a53_pu_pgc_pup_status;
#[doc = "M4 PU PGC software up trigger status register"]
pub struct M4_PU_PGC_PUP_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "M4 PU PGC software up trigger status register"]
pub mod m4_pu_pgc_pup_status;
#[doc = "CPU PGC software dn trigger status1"]
pub struct CPU_PGC_PDN_STATUS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU PGC software dn trigger status1"]
pub mod cpu_pgc_pdn_status1;
#[doc = "A53 MIX software down trigger status register"]
pub struct A53_MIX_PGC_PDN_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A53 MIX software down trigger status register"]
pub mod a53_mix_pgc_pdn_status;
#[doc = "M4 MIX PGC software power down trigger status register"]
pub struct M4_MIX_PGC_PDN_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "M4 MIX PGC software power down trigger status register"]
pub mod m4_mix_pgc_pdn_status;
#[doc = "A53 PU PGC software down trigger status"]
pub struct A53_PU_PGC_PDN_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A53 PU PGC software down trigger status"]
pub mod a53_pu_pgc_pdn_status;
#[doc = "M4 PU PGC software down trigger status"]
pub struct M4_PU_PGC_PDN_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "M4 PU PGC software down trigger status"]
pub mod m4_pu_pgc_pdn_status;
#[doc = "A53 MIX PDN FLG"]
pub struct A53_MIX_PDN_FLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A53 MIX PDN FLG"]
pub mod a53_mix_pdn_flg;
#[doc = "A53 PU PDN FLG"]
pub struct A53_PU_PDN_FLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A53 PU PDN FLG"]
pub mod a53_pu_pdn_flg;
#[doc = "M4 MIX PDN FLG"]
pub struct M4_MIX_PDN_FLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "M4 MIX PDN FLG"]
pub mod m4_mix_pdn_flg;
#[doc = "M4 PU PDN FLG"]
pub struct M4_PU_PDN_FLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "M4 PU PDN FLG"]
pub mod m4_pu_pdn_flg;
#[doc = "IRQ masking register 1 of A53 core2"]
pub struct IMR1_CORE2_A53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ masking register 1 of A53 core2"]
pub mod imr1_core2_a53;
#[doc = "IRQ masking register 2 of A53 core2"]
pub struct IMR2_CORE2_A53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ masking register 2 of A53 core2"]
pub mod imr2_core2_a53;
#[doc = "IRQ masking register 3 of A53 core2"]
pub struct IMR3_CORE2_A53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ masking register 3 of A53 core2"]
pub mod imr3_core2_a53;
#[doc = "IRQ masking register 4 of A53 core2"]
pub struct IMR4_CORE2_A53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ masking register 4 of A53 core2"]
pub mod imr4_core2_a53;
#[doc = "IRQ masking register 1 of A53 core3"]
pub struct IMR1_CORE3_A53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ masking register 1 of A53 core3"]
pub mod imr1_core3_a53;
#[doc = "IRQ masking register 2 of A53 core3"]
pub struct IMR2_CORE3_A53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ masking register 2 of A53 core3"]
pub mod imr2_core3_a53;
#[doc = "IRQ masking register 3 of A53 core3"]
pub struct IMR3_CORE3_A53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ masking register 3 of A53 core3"]
pub mod imr3_core3_a53;
#[doc = "IRQ masking register 4 of A53 core3"]
pub struct IMR4_CORE3_A53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ masking register 4 of A53 core3"]
pub mod imr4_core3_a53;
#[doc = "PGC acknowledge signal selection of A53 platform for PUs"]
pub struct ACK_SEL_A53_PU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PGC acknowledge signal selection of A53 platform for PUs"]
pub mod ack_sel_a53_pu;
#[doc = "PGC acknowledge signal selection of M4 platform for PUs"]
pub struct ACK_SEL_M4_PU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PGC acknowledge signal selection of M4 platform for PUs"]
pub mod ack_sel_m4_pu;
#[doc = "Slot configure register for A53 core"]
pub struct SLT15_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot configure register for A53 core"]
pub mod slt15_cfg;
#[doc = "Slot configure register for A53 core"]
pub struct SLT16_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot configure register for A53 core"]
pub mod slt16_cfg;
#[doc = "Slot configure register for A53 core"]
pub struct SLT17_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot configure register for A53 core"]
pub mod slt17_cfg;
#[doc = "Slot configure register for A53 core"]
pub struct SLT18_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot configure register for A53 core"]
pub mod slt18_cfg;
#[doc = "Slot configure register for A53 core"]
pub struct SLT19_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot configure register for A53 core"]
pub mod slt19_cfg;
#[doc = "Power handshake register"]
pub struct PU_PWRHSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power handshake register"]
pub mod pu_pwrhsk;
#[doc = "Slot configure register for PUs"]
pub struct SLT_CFG_PU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot configure register for PUs"]
pub mod slt_cfg_pu;
