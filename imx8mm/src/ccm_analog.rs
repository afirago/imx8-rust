#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AUDIO PLL1 General Function Control Register"]
    pub audio_pll1_gen_ctrl: AUDIO_PLL1_GEN_CTRL,
    #[doc = "0x04 - AUDIO PLL1 Divide and Fraction Data Control 0 Register"]
    pub audio_pll1_fdiv_ctl0: AUDIO_PLL1_FDIV_CTL0,
    #[doc = "0x08 - AUDIO PLL1 Divide and Fraction Data Control 1 Register"]
    pub audio_pll1_fdiv_ctl1: AUDIO_PLL1_FDIV_CTL1,
    #[doc = "0x0c - AUDIO PLL1 PLL SSCG Control Register"]
    pub audio_pll1_sscg_ctrl: AUDIO_PLL1_SSCG_CTRL,
    #[doc = "0x10 - AUDIO PLL1 PLL Monitoring Control Register"]
    pub audio_pll1_mnit_ctrl: AUDIO_PLL1_MNIT_CTRL,
    #[doc = "0x14 - AUDIO PLL2 General Function Control Register"]
    pub audio_pll2_gen_ctrl: AUDIO_PLL2_GEN_CTRL,
    #[doc = "0x18 - AUDIO PLL2 Divide and Fraction Data Control 0 Register"]
    pub audio_pll2_fdiv_ctl0: AUDIO_PLL2_FDIV_CTL0,
    #[doc = "0x1c - AUDIO PLL2 Divide and Fraction Data Control 1 Register"]
    pub audio_pll2_fdiv_ctl1: AUDIO_PLL2_FDIV_CTL1,
    #[doc = "0x20 - AUDIO PLL2 PLL SSCG Control Register"]
    pub audio_pll2_sscg_ctrl: AUDIO_PLL2_SSCG_CTRL,
    #[doc = "0x24 - AUDIO PLL2 PLL Monitoring Control Register"]
    pub audio_pll2_mnit_ctrl: AUDIO_PLL2_MNIT_CTRL,
    #[doc = "0x28 - VIDEO PLL1 General Function Control Register"]
    pub video_pll1_gen_ctrl: VIDEO_PLL1_GEN_CTRL,
    #[doc = "0x2c - VIDEO PLL1 Divide and Fraction Data Control 0 Register"]
    pub video_pll1_fdiv_ctl0: VIDEO_PLL1_FDIV_CTL0,
    #[doc = "0x30 - VIDEO PLL1 Divide and Fraction Data Control 1 Register"]
    pub video_pll1_fdiv_ctl1: VIDEO_PLL1_FDIV_CTL1,
    #[doc = "0x34 - VIDEO PLL1 PLL SSCG Control Register"]
    pub video_pll1_sscg_ctrl: VIDEO_PLL1_SSCG_CTRL,
    #[doc = "0x38 - VIDEO PLL1 PLL Monitoring Control Register"]
    pub video_pll1_mnit_ctrl: VIDEO_PLL1_MNIT_CTRL,
    _reserved0: [u8; 20usize],
    #[doc = "0x50 - DRAM PLL General Function Control Register"]
    pub dram_pll_gen_ctrl: DRAM_PLL_GEN_CTRL,
    #[doc = "0x54 - DRAM PLL Divide and Fraction Data Control 0 Register"]
    pub dram_pll_fdiv_ctl0: DRAM_PLL_FDIV_CTL0,
    #[doc = "0x58 - DRAM PLL Divide and Fraction Data Control 1 Register"]
    pub dram_pll_fdiv_ctl1: DRAM_PLL_FDIV_CTL1,
    #[doc = "0x5c - DRAM PLL PLL SSCG Control Register"]
    pub dram_pll_sscg_ctrl: DRAM_PLL_SSCG_CTRL,
    #[doc = "0x60 - DRAM PLL PLL Monitoring Control Register"]
    pub dram_pll_mnit_ctrl: DRAM_PLL_MNIT_CTRL,
    #[doc = "0x64 - GPU PLL General Function Control Register"]
    pub gpu_pll_gen_ctrl: GPU_PLL_GEN_CTRL,
    #[doc = "0x68 - GPU PLL Divide and Fraction Data Control 0 Register"]
    pub gpu_pll_fdiv_ctl0: GPU_PLL_FDIV_CTL0,
    #[doc = "0x6c - PLL Lock Detector Control Register"]
    pub gpu_pll_lockd_ctrl: GPU_PLL_LOCKD_CTRL,
    #[doc = "0x70 - PLL Monitoring Control Register"]
    pub gpu_pll_mnit_ctrl: GPU_PLL_MNIT_CTRL,
    #[doc = "0x74 - VPU PLL General Function Control Register"]
    pub vpu_pll_gen_ctrl: VPU_PLL_GEN_CTRL,
    #[doc = "0x78 - VPU PLL Divide and Fraction Data Control 0 Register"]
    pub vpu_pll_fdiv_ctl0: VPU_PLL_FDIV_CTL0,
    #[doc = "0x7c - PLL Lock Detector Control Register"]
    pub vpu_pll_lockd_ctrl: VPU_PLL_LOCKD_CTRL,
    #[doc = "0x80 - PLL Monitoring Control Register"]
    pub vpu_pll_mnit_ctrl: VPU_PLL_MNIT_CTRL,
    #[doc = "0x84 - ARM PLL General Function Control Register"]
    pub arm_pll_gen_ctrl: ARM_PLL_GEN_CTRL,
    #[doc = "0x88 - ARM PLL Divide and Fraction Data Control 0 Register"]
    pub arm_pll_fdiv_ctl0: ARM_PLL_FDIV_CTL0,
    #[doc = "0x8c - PLL Lock Detector Control Register"]
    pub arm_pll_lockd_ctrl: ARM_PLL_LOCKD_CTRL,
    #[doc = "0x90 - PLL Monitoring Control Register"]
    pub arm_pll_mnit_ctrl: ARM_PLL_MNIT_CTRL,
    #[doc = "0x94 - SYS PLL1 General Function Control Register"]
    pub sys_pll1_gen_ctrl: SYS_PLL1_GEN_CTRL,
    #[doc = "0x98 - SYS PLL1 Divide and Fraction Data Control 0 Register"]
    pub sys_pll1_fdiv_ctl0: SYS_PLL1_FDIV_CTL0,
    #[doc = "0x9c - PLL Lock Detector Control Register"]
    pub sys_pll1_lockd_ctrl: SYS_PLL1_LOCKD_CTRL,
    _reserved1: [u8; 96usize],
    #[doc = "0x100 - PLL Monitoring Control Register"]
    pub sys_pll1_mnit_ctrl: SYS_PLL1_MNIT_CTRL,
    #[doc = "0x104 - SYS PLL2 General Function Control Register"]
    pub sys_pll2_gen_ctrl: SYS_PLL2_GEN_CTRL,
    #[doc = "0x108 - SYS PLL2 Divide and Fraction Data Control 0 Register"]
    pub sys_pll2_fdiv_ctl0: SYS_PLL2_FDIV_CTL0,
    #[doc = "0x10c - PLL Lock Detector Control Register"]
    pub sys_pll2_lockd_ctrl: SYS_PLL2_LOCKD_CTRL,
    #[doc = "0x110 - PLL Monitoring Control Register"]
    pub sys_pll2_mnit_ctrl: SYS_PLL2_MNIT_CTRL,
    #[doc = "0x114 - SYS PLL3 General Function Control Register"]
    pub sys_pll3_gen_ctrl: SYS_PLL3_GEN_CTRL,
    #[doc = "0x118 - SYS PLL3 Divide and Fraction Data Control 0 Register"]
    pub sys_pll3_fdiv_ctl0: SYS_PLL3_FDIV_CTL0,
    #[doc = "0x11c - PLL Lock Detector Control Register"]
    pub sys_pll3_lockd_ctrl: SYS_PLL3_LOCKD_CTRL,
    #[doc = "0x120 - PLL Monitoring Control Register"]
    pub sys_pll3_mnit_ctrl: SYS_PLL3_MNIT_CTRL,
    #[doc = "0x124 - Osc Misc Configuration Register"]
    pub osc_misc_cfg: OSC_MISC_CFG,
    #[doc = "0x128 - PLL Clock Output for Test Enable and Select Register"]
    pub anamix_pll_mnit_ctl: ANAMIX_PLL_MNIT_CTL,
    _reserved2: [u8; 1748usize],
    #[doc = "0x800 - DIGPROG Register"]
    pub digprog: DIGPROG,
}
#[doc = "AUDIO PLL1 General Function Control Register"]
pub struct AUDIO_PLL1_GEN_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AUDIO PLL1 General Function Control Register"]
pub mod audio_pll1_gen_ctrl;
#[doc = "AUDIO PLL1 Divide and Fraction Data Control 0 Register"]
pub struct AUDIO_PLL1_FDIV_CTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AUDIO PLL1 Divide and Fraction Data Control 0 Register"]
pub mod audio_pll1_fdiv_ctl0;
#[doc = "AUDIO PLL1 Divide and Fraction Data Control 1 Register"]
pub struct AUDIO_PLL1_FDIV_CTL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AUDIO PLL1 Divide and Fraction Data Control 1 Register"]
pub mod audio_pll1_fdiv_ctl1;
#[doc = "AUDIO PLL1 PLL SSCG Control Register"]
pub struct AUDIO_PLL1_SSCG_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AUDIO PLL1 PLL SSCG Control Register"]
pub mod audio_pll1_sscg_ctrl;
#[doc = "AUDIO PLL1 PLL Monitoring Control Register"]
pub struct AUDIO_PLL1_MNIT_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AUDIO PLL1 PLL Monitoring Control Register"]
pub mod audio_pll1_mnit_ctrl;
#[doc = "AUDIO PLL2 General Function Control Register"]
pub struct AUDIO_PLL2_GEN_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AUDIO PLL2 General Function Control Register"]
pub mod audio_pll2_gen_ctrl;
#[doc = "AUDIO PLL2 Divide and Fraction Data Control 0 Register"]
pub struct AUDIO_PLL2_FDIV_CTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AUDIO PLL2 Divide and Fraction Data Control 0 Register"]
pub mod audio_pll2_fdiv_ctl0;
#[doc = "AUDIO PLL2 Divide and Fraction Data Control 1 Register"]
pub struct AUDIO_PLL2_FDIV_CTL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AUDIO PLL2 Divide and Fraction Data Control 1 Register"]
pub mod audio_pll2_fdiv_ctl1;
#[doc = "AUDIO PLL2 PLL SSCG Control Register"]
pub struct AUDIO_PLL2_SSCG_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AUDIO PLL2 PLL SSCG Control Register"]
pub mod audio_pll2_sscg_ctrl;
#[doc = "AUDIO PLL2 PLL Monitoring Control Register"]
pub struct AUDIO_PLL2_MNIT_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AUDIO PLL2 PLL Monitoring Control Register"]
pub mod audio_pll2_mnit_ctrl;
#[doc = "VIDEO PLL1 General Function Control Register"]
pub struct VIDEO_PLL1_GEN_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VIDEO PLL1 General Function Control Register"]
pub mod video_pll1_gen_ctrl;
#[doc = "VIDEO PLL1 Divide and Fraction Data Control 0 Register"]
pub struct VIDEO_PLL1_FDIV_CTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VIDEO PLL1 Divide and Fraction Data Control 0 Register"]
pub mod video_pll1_fdiv_ctl0;
#[doc = "VIDEO PLL1 Divide and Fraction Data Control 1 Register"]
pub struct VIDEO_PLL1_FDIV_CTL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VIDEO PLL1 Divide and Fraction Data Control 1 Register"]
pub mod video_pll1_fdiv_ctl1;
#[doc = "VIDEO PLL1 PLL SSCG Control Register"]
pub struct VIDEO_PLL1_SSCG_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VIDEO PLL1 PLL SSCG Control Register"]
pub mod video_pll1_sscg_ctrl;
#[doc = "VIDEO PLL1 PLL Monitoring Control Register"]
pub struct VIDEO_PLL1_MNIT_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VIDEO PLL1 PLL Monitoring Control Register"]
pub mod video_pll1_mnit_ctrl;
#[doc = "DRAM PLL General Function Control Register"]
pub struct DRAM_PLL_GEN_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DRAM PLL General Function Control Register"]
pub mod dram_pll_gen_ctrl;
#[doc = "DRAM PLL Divide and Fraction Data Control 0 Register"]
pub struct DRAM_PLL_FDIV_CTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DRAM PLL Divide and Fraction Data Control 0 Register"]
pub mod dram_pll_fdiv_ctl0;
#[doc = "DRAM PLL Divide and Fraction Data Control 1 Register"]
pub struct DRAM_PLL_FDIV_CTL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DRAM PLL Divide and Fraction Data Control 1 Register"]
pub mod dram_pll_fdiv_ctl1;
#[doc = "DRAM PLL PLL SSCG Control Register"]
pub struct DRAM_PLL_SSCG_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DRAM PLL PLL SSCG Control Register"]
pub mod dram_pll_sscg_ctrl;
#[doc = "DRAM PLL PLL Monitoring Control Register"]
pub struct DRAM_PLL_MNIT_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DRAM PLL PLL Monitoring Control Register"]
pub mod dram_pll_mnit_ctrl;
#[doc = "GPU PLL General Function Control Register"]
pub struct GPU_PLL_GEN_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPU PLL General Function Control Register"]
pub mod gpu_pll_gen_ctrl;
#[doc = "GPU PLL Divide and Fraction Data Control 0 Register"]
pub struct GPU_PLL_FDIV_CTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPU PLL Divide and Fraction Data Control 0 Register"]
pub mod gpu_pll_fdiv_ctl0;
#[doc = "PLL Lock Detector Control Register"]
pub struct GPU_PLL_LOCKD_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Lock Detector Control Register"]
pub mod gpu_pll_lockd_ctrl;
#[doc = "PLL Monitoring Control Register"]
pub struct GPU_PLL_MNIT_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Monitoring Control Register"]
pub mod gpu_pll_mnit_ctrl;
#[doc = "VPU PLL General Function Control Register"]
pub struct VPU_PLL_GEN_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VPU PLL General Function Control Register"]
pub mod vpu_pll_gen_ctrl;
#[doc = "VPU PLL Divide and Fraction Data Control 0 Register"]
pub struct VPU_PLL_FDIV_CTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VPU PLL Divide and Fraction Data Control 0 Register"]
pub mod vpu_pll_fdiv_ctl0;
#[doc = "PLL Lock Detector Control Register"]
pub struct VPU_PLL_LOCKD_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Lock Detector Control Register"]
pub mod vpu_pll_lockd_ctrl;
#[doc = "PLL Monitoring Control Register"]
pub struct VPU_PLL_MNIT_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Monitoring Control Register"]
pub mod vpu_pll_mnit_ctrl;
#[doc = "ARM PLL General Function Control Register"]
pub struct ARM_PLL_GEN_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ARM PLL General Function Control Register"]
pub mod arm_pll_gen_ctrl;
#[doc = "ARM PLL Divide and Fraction Data Control 0 Register"]
pub struct ARM_PLL_FDIV_CTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ARM PLL Divide and Fraction Data Control 0 Register"]
pub mod arm_pll_fdiv_ctl0;
#[doc = "PLL Lock Detector Control Register"]
pub struct ARM_PLL_LOCKD_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Lock Detector Control Register"]
pub mod arm_pll_lockd_ctrl;
#[doc = "PLL Monitoring Control Register"]
pub struct ARM_PLL_MNIT_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Monitoring Control Register"]
pub mod arm_pll_mnit_ctrl;
#[doc = "SYS PLL1 General Function Control Register"]
pub struct SYS_PLL1_GEN_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SYS PLL1 General Function Control Register"]
pub mod sys_pll1_gen_ctrl;
#[doc = "SYS PLL1 Divide and Fraction Data Control 0 Register"]
pub struct SYS_PLL1_FDIV_CTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SYS PLL1 Divide and Fraction Data Control 0 Register"]
pub mod sys_pll1_fdiv_ctl0;
#[doc = "PLL Lock Detector Control Register"]
pub struct SYS_PLL1_LOCKD_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Lock Detector Control Register"]
pub mod sys_pll1_lockd_ctrl;
#[doc = "PLL Monitoring Control Register"]
pub struct SYS_PLL1_MNIT_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Monitoring Control Register"]
pub mod sys_pll1_mnit_ctrl;
#[doc = "SYS PLL2 General Function Control Register"]
pub struct SYS_PLL2_GEN_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SYS PLL2 General Function Control Register"]
pub mod sys_pll2_gen_ctrl;
#[doc = "SYS PLL2 Divide and Fraction Data Control 0 Register"]
pub struct SYS_PLL2_FDIV_CTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SYS PLL2 Divide and Fraction Data Control 0 Register"]
pub mod sys_pll2_fdiv_ctl0;
#[doc = "PLL Lock Detector Control Register"]
pub struct SYS_PLL2_LOCKD_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Lock Detector Control Register"]
pub mod sys_pll2_lockd_ctrl;
#[doc = "PLL Monitoring Control Register"]
pub struct SYS_PLL2_MNIT_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Monitoring Control Register"]
pub mod sys_pll2_mnit_ctrl;
#[doc = "SYS PLL3 General Function Control Register"]
pub struct SYS_PLL3_GEN_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SYS PLL3 General Function Control Register"]
pub mod sys_pll3_gen_ctrl;
#[doc = "SYS PLL3 Divide and Fraction Data Control 0 Register"]
pub struct SYS_PLL3_FDIV_CTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SYS PLL3 Divide and Fraction Data Control 0 Register"]
pub mod sys_pll3_fdiv_ctl0;
#[doc = "PLL Lock Detector Control Register"]
pub struct SYS_PLL3_LOCKD_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Lock Detector Control Register"]
pub mod sys_pll3_lockd_ctrl;
#[doc = "PLL Monitoring Control Register"]
pub struct SYS_PLL3_MNIT_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Monitoring Control Register"]
pub mod sys_pll3_mnit_ctrl;
#[doc = "Osc Misc Configuration Register"]
pub struct OSC_MISC_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Osc Misc Configuration Register"]
pub mod osc_misc_cfg;
#[doc = "PLL Clock Output for Test Enable and Select Register"]
pub struct ANAMIX_PLL_MNIT_CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL Clock Output for Test Enable and Select Register"]
pub mod anamix_pll_mnit_ctl;
#[doc = "DIGPROG Register"]
pub struct DIGPROG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DIGPROG Register"]
pub mod digprog;
