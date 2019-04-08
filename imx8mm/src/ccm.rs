#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - General Purpose Register"]
    pub gpr0: GPR0,
    #[doc = "0x04 - General Purpose Register"]
    pub gpr0_set: GPR0_SET,
    #[doc = "0x08 - General Purpose Register"]
    pub gpr0_clr: GPR0_CLR,
    #[doc = "0x0c - General Purpose Register"]
    pub gpr0_tog: GPR0_TOG,
    _reserved0: [u8; 2032usize],
    #[doc = "0x800 - CCM PLL Control Register"]
    pub pll_ctrl0: PLL_CTRL,
    #[doc = "0x804 - CCM PLL Control Register"]
    pub pll_ctrl0_set: PLL_CTRL_SET,
    #[doc = "0x808 - CCM PLL Control Register"]
    pub pll_ctrl0_clr: PLL_CTRL_CLR,
    #[doc = "0x80c - CCM PLL Control Register"]
    pub pll_ctrl0_tog: PLL_CTRL_TOG,
    #[doc = "0x810 - CCM PLL Control Register"]
    pub pll_ctrl1: PLL_CTRL,
    #[doc = "0x814 - CCM PLL Control Register"]
    pub pll_ctrl1_set: PLL_CTRL_SET,
    #[doc = "0x818 - CCM PLL Control Register"]
    pub pll_ctrl1_clr: PLL_CTRL_CLR,
    #[doc = "0x81c - CCM PLL Control Register"]
    pub pll_ctrl1_tog: PLL_CTRL_TOG,
    #[doc = "0x820 - CCM PLL Control Register"]
    pub pll_ctrl2: PLL_CTRL,
    #[doc = "0x824 - CCM PLL Control Register"]
    pub pll_ctrl2_set: PLL_CTRL_SET,
    #[doc = "0x828 - CCM PLL Control Register"]
    pub pll_ctrl2_clr: PLL_CTRL_CLR,
    #[doc = "0x82c - CCM PLL Control Register"]
    pub pll_ctrl2_tog: PLL_CTRL_TOG,
    #[doc = "0x830 - CCM PLL Control Register"]
    pub pll_ctrl3: PLL_CTRL,
    #[doc = "0x834 - CCM PLL Control Register"]
    pub pll_ctrl3_set: PLL_CTRL_SET,
    #[doc = "0x838 - CCM PLL Control Register"]
    pub pll_ctrl3_clr: PLL_CTRL_CLR,
    #[doc = "0x83c - CCM PLL Control Register"]
    pub pll_ctrl3_tog: PLL_CTRL_TOG,
    #[doc = "0x840 - CCM PLL Control Register"]
    pub pll_ctrl4: PLL_CTRL,
    #[doc = "0x844 - CCM PLL Control Register"]
    pub pll_ctrl4_set: PLL_CTRL_SET,
    #[doc = "0x848 - CCM PLL Control Register"]
    pub pll_ctrl4_clr: PLL_CTRL_CLR,
    #[doc = "0x84c - CCM PLL Control Register"]
    pub pll_ctrl4_tog: PLL_CTRL_TOG,
    #[doc = "0x850 - CCM PLL Control Register"]
    pub pll_ctrl5: PLL_CTRL,
    #[doc = "0x854 - CCM PLL Control Register"]
    pub pll_ctrl5_set: PLL_CTRL_SET,
    #[doc = "0x858 - CCM PLL Control Register"]
    pub pll_ctrl5_clr: PLL_CTRL_CLR,
    #[doc = "0x85c - CCM PLL Control Register"]
    pub pll_ctrl5_tog: PLL_CTRL_TOG,
    #[doc = "0x860 - CCM PLL Control Register"]
    pub pll_ctrl6: PLL_CTRL,
    #[doc = "0x864 - CCM PLL Control Register"]
    pub pll_ctrl6_set: PLL_CTRL_SET,
    #[doc = "0x868 - CCM PLL Control Register"]
    pub pll_ctrl6_clr: PLL_CTRL_CLR,
    #[doc = "0x86c - CCM PLL Control Register"]
    pub pll_ctrl6_tog: PLL_CTRL_TOG,
    #[doc = "0x870 - CCM PLL Control Register"]
    pub pll_ctrl7: PLL_CTRL,
    #[doc = "0x874 - CCM PLL Control Register"]
    pub pll_ctrl7_set: PLL_CTRL_SET,
    #[doc = "0x878 - CCM PLL Control Register"]
    pub pll_ctrl7_clr: PLL_CTRL_CLR,
    #[doc = "0x87c - CCM PLL Control Register"]
    pub pll_ctrl7_tog: PLL_CTRL_TOG,
    #[doc = "0x880 - CCM PLL Control Register"]
    pub pll_ctrl8: PLL_CTRL,
    #[doc = "0x884 - CCM PLL Control Register"]
    pub pll_ctrl8_set: PLL_CTRL_SET,
    #[doc = "0x888 - CCM PLL Control Register"]
    pub pll_ctrl8_clr: PLL_CTRL_CLR,
    #[doc = "0x88c - CCM PLL Control Register"]
    pub pll_ctrl8_tog: PLL_CTRL_TOG,
    #[doc = "0x890 - CCM PLL Control Register"]
    pub pll_ctrl9: PLL_CTRL,
    #[doc = "0x894 - CCM PLL Control Register"]
    pub pll_ctrl9_set: PLL_CTRL_SET,
    #[doc = "0x898 - CCM PLL Control Register"]
    pub pll_ctrl9_clr: PLL_CTRL_CLR,
    #[doc = "0x89c - CCM PLL Control Register"]
    pub pll_ctrl9_tog: PLL_CTRL_TOG,
    #[doc = "0x8a0 - CCM PLL Control Register"]
    pub pll_ctrl10: PLL_CTRL,
    #[doc = "0x8a4 - CCM PLL Control Register"]
    pub pll_ctrl10_set: PLL_CTRL_SET,
    #[doc = "0x8a8 - CCM PLL Control Register"]
    pub pll_ctrl10_clr: PLL_CTRL_CLR,
    #[doc = "0x8ac - CCM PLL Control Register"]
    pub pll_ctrl10_tog: PLL_CTRL_TOG,
    #[doc = "0x8b0 - CCM PLL Control Register"]
    pub pll_ctrl11: PLL_CTRL,
    #[doc = "0x8b4 - CCM PLL Control Register"]
    pub pll_ctrl11_set: PLL_CTRL_SET,
    #[doc = "0x8b8 - CCM PLL Control Register"]
    pub pll_ctrl11_clr: PLL_CTRL_CLR,
    #[doc = "0x8bc - CCM PLL Control Register"]
    pub pll_ctrl11_tog: PLL_CTRL_TOG,
    #[doc = "0x8c0 - CCM PLL Control Register"]
    pub pll_ctrl12: PLL_CTRL,
    #[doc = "0x8c4 - CCM PLL Control Register"]
    pub pll_ctrl12_set: PLL_CTRL_SET,
    #[doc = "0x8c8 - CCM PLL Control Register"]
    pub pll_ctrl12_clr: PLL_CTRL_CLR,
    #[doc = "0x8cc - CCM PLL Control Register"]
    pub pll_ctrl12_tog: PLL_CTRL_TOG,
    #[doc = "0x8d0 - CCM PLL Control Register"]
    pub pll_ctrl13: PLL_CTRL,
    #[doc = "0x8d4 - CCM PLL Control Register"]
    pub pll_ctrl13_set: PLL_CTRL_SET,
    #[doc = "0x8d8 - CCM PLL Control Register"]
    pub pll_ctrl13_clr: PLL_CTRL_CLR,
    #[doc = "0x8dc - CCM PLL Control Register"]
    pub pll_ctrl13_tog: PLL_CTRL_TOG,
    #[doc = "0x8e0 - CCM PLL Control Register"]
    pub pll_ctrl14: PLL_CTRL,
    #[doc = "0x8e4 - CCM PLL Control Register"]
    pub pll_ctrl14_set: PLL_CTRL_SET,
    #[doc = "0x8e8 - CCM PLL Control Register"]
    pub pll_ctrl14_clr: PLL_CTRL_CLR,
    #[doc = "0x8ec - CCM PLL Control Register"]
    pub pll_ctrl14_tog: PLL_CTRL_TOG,
    #[doc = "0x8f0 - CCM PLL Control Register"]
    pub pll_ctrl15: PLL_CTRL,
    #[doc = "0x8f4 - CCM PLL Control Register"]
    pub pll_ctrl15_set: PLL_CTRL_SET,
    #[doc = "0x8f8 - CCM PLL Control Register"]
    pub pll_ctrl15_clr: PLL_CTRL_CLR,
    #[doc = "0x8fc - CCM PLL Control Register"]
    pub pll_ctrl15_tog: PLL_CTRL_TOG,
    #[doc = "0x900 - CCM PLL Control Register"]
    pub pll_ctrl16: PLL_CTRL,
    #[doc = "0x904 - CCM PLL Control Register"]
    pub pll_ctrl16_set: PLL_CTRL_SET,
    #[doc = "0x908 - CCM PLL Control Register"]
    pub pll_ctrl16_clr: PLL_CTRL_CLR,
    #[doc = "0x90c - CCM PLL Control Register"]
    pub pll_ctrl16_tog: PLL_CTRL_TOG,
    #[doc = "0x910 - CCM PLL Control Register"]
    pub pll_ctrl17: PLL_CTRL,
    #[doc = "0x914 - CCM PLL Control Register"]
    pub pll_ctrl17_set: PLL_CTRL_SET,
    #[doc = "0x918 - CCM PLL Control Register"]
    pub pll_ctrl17_clr: PLL_CTRL_CLR,
    #[doc = "0x91c - CCM PLL Control Register"]
    pub pll_ctrl17_tog: PLL_CTRL_TOG,
    #[doc = "0x920 - CCM PLL Control Register"]
    pub pll_ctrl18: PLL_CTRL,
    #[doc = "0x924 - CCM PLL Control Register"]
    pub pll_ctrl18_set: PLL_CTRL_SET,
    #[doc = "0x928 - CCM PLL Control Register"]
    pub pll_ctrl18_clr: PLL_CTRL_CLR,
    #[doc = "0x92c - CCM PLL Control Register"]
    pub pll_ctrl18_tog: PLL_CTRL_TOG,
    #[doc = "0x930 - CCM PLL Control Register"]
    pub pll_ctrl19: PLL_CTRL,
    #[doc = "0x934 - CCM PLL Control Register"]
    pub pll_ctrl19_set: PLL_CTRL_SET,
    #[doc = "0x938 - CCM PLL Control Register"]
    pub pll_ctrl19_clr: PLL_CTRL_CLR,
    #[doc = "0x93c - CCM PLL Control Register"]
    pub pll_ctrl19_tog: PLL_CTRL_TOG,
    #[doc = "0x940 - CCM PLL Control Register"]
    pub pll_ctrl20: PLL_CTRL,
    #[doc = "0x944 - CCM PLL Control Register"]
    pub pll_ctrl20_set: PLL_CTRL_SET,
    #[doc = "0x948 - CCM PLL Control Register"]
    pub pll_ctrl20_clr: PLL_CTRL_CLR,
    #[doc = "0x94c - CCM PLL Control Register"]
    pub pll_ctrl20_tog: PLL_CTRL_TOG,
    #[doc = "0x950 - CCM PLL Control Register"]
    pub pll_ctrl21: PLL_CTRL,
    #[doc = "0x954 - CCM PLL Control Register"]
    pub pll_ctrl21_set: PLL_CTRL_SET,
    #[doc = "0x958 - CCM PLL Control Register"]
    pub pll_ctrl21_clr: PLL_CTRL_CLR,
    #[doc = "0x95c - CCM PLL Control Register"]
    pub pll_ctrl21_tog: PLL_CTRL_TOG,
    #[doc = "0x960 - CCM PLL Control Register"]
    pub pll_ctrl22: PLL_CTRL,
    #[doc = "0x964 - CCM PLL Control Register"]
    pub pll_ctrl22_set: PLL_CTRL_SET,
    #[doc = "0x968 - CCM PLL Control Register"]
    pub pll_ctrl22_clr: PLL_CTRL_CLR,
    #[doc = "0x96c - CCM PLL Control Register"]
    pub pll_ctrl22_tog: PLL_CTRL_TOG,
    #[doc = "0x970 - CCM PLL Control Register"]
    pub pll_ctrl23: PLL_CTRL,
    #[doc = "0x974 - CCM PLL Control Register"]
    pub pll_ctrl23_set: PLL_CTRL_SET,
    #[doc = "0x978 - CCM PLL Control Register"]
    pub pll_ctrl23_clr: PLL_CTRL_CLR,
    #[doc = "0x97c - CCM PLL Control Register"]
    pub pll_ctrl23_tog: PLL_CTRL_TOG,
    #[doc = "0x980 - CCM PLL Control Register"]
    pub pll_ctrl24: PLL_CTRL,
    #[doc = "0x984 - CCM PLL Control Register"]
    pub pll_ctrl24_set: PLL_CTRL_SET,
    #[doc = "0x988 - CCM PLL Control Register"]
    pub pll_ctrl24_clr: PLL_CTRL_CLR,
    #[doc = "0x98c - CCM PLL Control Register"]
    pub pll_ctrl24_tog: PLL_CTRL_TOG,
    #[doc = "0x990 - CCM PLL Control Register"]
    pub pll_ctrl25: PLL_CTRL,
    #[doc = "0x994 - CCM PLL Control Register"]
    pub pll_ctrl25_set: PLL_CTRL_SET,
    #[doc = "0x998 - CCM PLL Control Register"]
    pub pll_ctrl25_clr: PLL_CTRL_CLR,
    #[doc = "0x99c - CCM PLL Control Register"]
    pub pll_ctrl25_tog: PLL_CTRL_TOG,
    #[doc = "0x9a0 - CCM PLL Control Register"]
    pub pll_ctrl26: PLL_CTRL,
    #[doc = "0x9a4 - CCM PLL Control Register"]
    pub pll_ctrl26_set: PLL_CTRL_SET,
    #[doc = "0x9a8 - CCM PLL Control Register"]
    pub pll_ctrl26_clr: PLL_CTRL_CLR,
    #[doc = "0x9ac - CCM PLL Control Register"]
    pub pll_ctrl26_tog: PLL_CTRL_TOG,
    #[doc = "0x9b0 - CCM PLL Control Register"]
    pub pll_ctrl27: PLL_CTRL,
    #[doc = "0x9b4 - CCM PLL Control Register"]
    pub pll_ctrl27_set: PLL_CTRL_SET,
    #[doc = "0x9b8 - CCM PLL Control Register"]
    pub pll_ctrl27_clr: PLL_CTRL_CLR,
    #[doc = "0x9bc - CCM PLL Control Register"]
    pub pll_ctrl27_tog: PLL_CTRL_TOG,
    #[doc = "0x9c0 - CCM PLL Control Register"]
    pub pll_ctrl28: PLL_CTRL,
    #[doc = "0x9c4 - CCM PLL Control Register"]
    pub pll_ctrl28_set: PLL_CTRL_SET,
    #[doc = "0x9c8 - CCM PLL Control Register"]
    pub pll_ctrl28_clr: PLL_CTRL_CLR,
    #[doc = "0x9cc - CCM PLL Control Register"]
    pub pll_ctrl28_tog: PLL_CTRL_TOG,
    #[doc = "0x9d0 - CCM PLL Control Register"]
    pub pll_ctrl29: PLL_CTRL,
    #[doc = "0x9d4 - CCM PLL Control Register"]
    pub pll_ctrl29_set: PLL_CTRL_SET,
    #[doc = "0x9d8 - CCM PLL Control Register"]
    pub pll_ctrl29_clr: PLL_CTRL_CLR,
    #[doc = "0x9dc - CCM PLL Control Register"]
    pub pll_ctrl29_tog: PLL_CTRL_TOG,
    #[doc = "0x9e0 - CCM PLL Control Register"]
    pub pll_ctrl30: PLL_CTRL,
    #[doc = "0x9e4 - CCM PLL Control Register"]
    pub pll_ctrl30_set: PLL_CTRL_SET,
    #[doc = "0x9e8 - CCM PLL Control Register"]
    pub pll_ctrl30_clr: PLL_CTRL_CLR,
    #[doc = "0x9ec - CCM PLL Control Register"]
    pub pll_ctrl30_tog: PLL_CTRL_TOG,
    #[doc = "0x9f0 - CCM PLL Control Register"]
    pub pll_ctrl31: PLL_CTRL,
    #[doc = "0x9f4 - CCM PLL Control Register"]
    pub pll_ctrl31_set: PLL_CTRL_SET,
    #[doc = "0x9f8 - CCM PLL Control Register"]
    pub pll_ctrl31_clr: PLL_CTRL_CLR,
    #[doc = "0x9fc - CCM PLL Control Register"]
    pub pll_ctrl31_tog: PLL_CTRL_TOG,
    #[doc = "0xa00 - CCM PLL Control Register"]
    pub pll_ctrl32: PLL_CTRL,
    #[doc = "0xa04 - CCM PLL Control Register"]
    pub pll_ctrl32_set: PLL_CTRL_SET,
    #[doc = "0xa08 - CCM PLL Control Register"]
    pub pll_ctrl32_clr: PLL_CTRL_CLR,
    #[doc = "0xa0c - CCM PLL Control Register"]
    pub pll_ctrl32_tog: PLL_CTRL_TOG,
    #[doc = "0xa10 - CCM PLL Control Register"]
    pub pll_ctrl33: PLL_CTRL,
    #[doc = "0xa14 - CCM PLL Control Register"]
    pub pll_ctrl33_set: PLL_CTRL_SET,
    #[doc = "0xa18 - CCM PLL Control Register"]
    pub pll_ctrl33_clr: PLL_CTRL_CLR,
    #[doc = "0xa1c - CCM PLL Control Register"]
    pub pll_ctrl33_tog: PLL_CTRL_TOG,
    #[doc = "0xa20 - CCM PLL Control Register"]
    pub pll_ctrl34: PLL_CTRL,
    #[doc = "0xa24 - CCM PLL Control Register"]
    pub pll_ctrl34_set: PLL_CTRL_SET,
    #[doc = "0xa28 - CCM PLL Control Register"]
    pub pll_ctrl34_clr: PLL_CTRL_CLR,
    #[doc = "0xa2c - CCM PLL Control Register"]
    pub pll_ctrl34_tog: PLL_CTRL_TOG,
    #[doc = "0xa30 - CCM PLL Control Register"]
    pub pll_ctrl35: PLL_CTRL,
    #[doc = "0xa34 - CCM PLL Control Register"]
    pub pll_ctrl35_set: PLL_CTRL_SET,
    #[doc = "0xa38 - CCM PLL Control Register"]
    pub pll_ctrl35_clr: PLL_CTRL_CLR,
    #[doc = "0xa3c - CCM PLL Control Register"]
    pub pll_ctrl35_tog: PLL_CTRL_TOG,
    #[doc = "0xa40 - CCM PLL Control Register"]
    pub pll_ctrl36: PLL_CTRL,
    #[doc = "0xa44 - CCM PLL Control Register"]
    pub pll_ctrl36_set: PLL_CTRL_SET,
    #[doc = "0xa48 - CCM PLL Control Register"]
    pub pll_ctrl36_clr: PLL_CTRL_CLR,
    #[doc = "0xa4c - CCM PLL Control Register"]
    pub pll_ctrl36_tog: PLL_CTRL_TOG,
    #[doc = "0xa50 - CCM PLL Control Register"]
    pub pll_ctrl37: PLL_CTRL,
    #[doc = "0xa54 - CCM PLL Control Register"]
    pub pll_ctrl37_set: PLL_CTRL_SET,
    #[doc = "0xa58 - CCM PLL Control Register"]
    pub pll_ctrl37_clr: PLL_CTRL_CLR,
    #[doc = "0xa5c - CCM PLL Control Register"]
    pub pll_ctrl37_tog: PLL_CTRL_TOG,
    #[doc = "0xa60 - CCM PLL Control Register"]
    pub pll_ctrl38: PLL_CTRL,
    #[doc = "0xa64 - CCM PLL Control Register"]
    pub pll_ctrl38_set: PLL_CTRL_SET,
    #[doc = "0xa68 - CCM PLL Control Register"]
    pub pll_ctrl38_clr: PLL_CTRL_CLR,
    #[doc = "0xa6c - CCM PLL Control Register"]
    pub pll_ctrl38_tog: PLL_CTRL_TOG,
    _reserved1: [u8; 13712usize],
    #[doc = "0x4000 - CCM Clock Gating Register"]
    pub ccgr0: CCGR,
    #[doc = "0x4004 - CCM Clock Gating Register"]
    pub ccgr0_set: CCGR_SET,
    #[doc = "0x4008 - CCM Clock Gating Register"]
    pub ccgr0_clr: CCGR_CLR,
    #[doc = "0x400c - CCM Clock Gating Register"]
    pub ccgr0_tog: CCGR_TOG,
    #[doc = "0x4010 - CCM Clock Gating Register"]
    pub ccgr1: CCGR,
    #[doc = "0x4014 - CCM Clock Gating Register"]
    pub ccgr1_set: CCGR_SET,
    #[doc = "0x4018 - CCM Clock Gating Register"]
    pub ccgr1_clr: CCGR_CLR,
    #[doc = "0x401c - CCM Clock Gating Register"]
    pub ccgr1_tog: CCGR_TOG,
    #[doc = "0x4020 - CCM Clock Gating Register"]
    pub ccgr2: CCGR,
    #[doc = "0x4024 - CCM Clock Gating Register"]
    pub ccgr2_set: CCGR_SET,
    #[doc = "0x4028 - CCM Clock Gating Register"]
    pub ccgr2_clr: CCGR_CLR,
    #[doc = "0x402c - CCM Clock Gating Register"]
    pub ccgr2_tog: CCGR_TOG,
    #[doc = "0x4030 - CCM Clock Gating Register"]
    pub ccgr3: CCGR,
    #[doc = "0x4034 - CCM Clock Gating Register"]
    pub ccgr3_set: CCGR_SET,
    #[doc = "0x4038 - CCM Clock Gating Register"]
    pub ccgr3_clr: CCGR_CLR,
    #[doc = "0x403c - CCM Clock Gating Register"]
    pub ccgr3_tog: CCGR_TOG,
    #[doc = "0x4040 - CCM Clock Gating Register"]
    pub ccgr4: CCGR,
    #[doc = "0x4044 - CCM Clock Gating Register"]
    pub ccgr4_set: CCGR_SET,
    #[doc = "0x4048 - CCM Clock Gating Register"]
    pub ccgr4_clr: CCGR_CLR,
    #[doc = "0x404c - CCM Clock Gating Register"]
    pub ccgr4_tog: CCGR_TOG,
    #[doc = "0x4050 - CCM Clock Gating Register"]
    pub ccgr5: CCGR,
    #[doc = "0x4054 - CCM Clock Gating Register"]
    pub ccgr5_set: CCGR_SET,
    #[doc = "0x4058 - CCM Clock Gating Register"]
    pub ccgr5_clr: CCGR_CLR,
    #[doc = "0x405c - CCM Clock Gating Register"]
    pub ccgr5_tog: CCGR_TOG,
    #[doc = "0x4060 - CCM Clock Gating Register"]
    pub ccgr6: CCGR,
    #[doc = "0x4064 - CCM Clock Gating Register"]
    pub ccgr6_set: CCGR_SET,
    #[doc = "0x4068 - CCM Clock Gating Register"]
    pub ccgr6_clr: CCGR_CLR,
    #[doc = "0x406c - CCM Clock Gating Register"]
    pub ccgr6_tog: CCGR_TOG,
    #[doc = "0x4070 - CCM Clock Gating Register"]
    pub ccgr7: CCGR,
    #[doc = "0x4074 - CCM Clock Gating Register"]
    pub ccgr7_set: CCGR_SET,
    #[doc = "0x4078 - CCM Clock Gating Register"]
    pub ccgr7_clr: CCGR_CLR,
    #[doc = "0x407c - CCM Clock Gating Register"]
    pub ccgr7_tog: CCGR_TOG,
    #[doc = "0x4080 - CCM Clock Gating Register"]
    pub ccgr8: CCGR,
    #[doc = "0x4084 - CCM Clock Gating Register"]
    pub ccgr8_set: CCGR_SET,
    #[doc = "0x4088 - CCM Clock Gating Register"]
    pub ccgr8_clr: CCGR_CLR,
    #[doc = "0x408c - CCM Clock Gating Register"]
    pub ccgr8_tog: CCGR_TOG,
    #[doc = "0x4090 - CCM Clock Gating Register"]
    pub ccgr9: CCGR,
    #[doc = "0x4094 - CCM Clock Gating Register"]
    pub ccgr9_set: CCGR_SET,
    #[doc = "0x4098 - CCM Clock Gating Register"]
    pub ccgr9_clr: CCGR_CLR,
    #[doc = "0x409c - CCM Clock Gating Register"]
    pub ccgr9_tog: CCGR_TOG,
    #[doc = "0x40a0 - CCM Clock Gating Register"]
    pub ccgr10: CCGR,
    #[doc = "0x40a4 - CCM Clock Gating Register"]
    pub ccgr10_set: CCGR_SET,
    #[doc = "0x40a8 - CCM Clock Gating Register"]
    pub ccgr10_clr: CCGR_CLR,
    #[doc = "0x40ac - CCM Clock Gating Register"]
    pub ccgr10_tog: CCGR_TOG,
    #[doc = "0x40b0 - CCM Clock Gating Register"]
    pub ccgr11: CCGR,
    #[doc = "0x40b4 - CCM Clock Gating Register"]
    pub ccgr11_set: CCGR_SET,
    #[doc = "0x40b8 - CCM Clock Gating Register"]
    pub ccgr11_clr: CCGR_CLR,
    #[doc = "0x40bc - CCM Clock Gating Register"]
    pub ccgr11_tog: CCGR_TOG,
    #[doc = "0x40c0 - CCM Clock Gating Register"]
    pub ccgr12: CCGR,
    #[doc = "0x40c4 - CCM Clock Gating Register"]
    pub ccgr12_set: CCGR_SET,
    #[doc = "0x40c8 - CCM Clock Gating Register"]
    pub ccgr12_clr: CCGR_CLR,
    #[doc = "0x40cc - CCM Clock Gating Register"]
    pub ccgr12_tog: CCGR_TOG,
    #[doc = "0x40d0 - CCM Clock Gating Register"]
    pub ccgr13: CCGR,
    #[doc = "0x40d4 - CCM Clock Gating Register"]
    pub ccgr13_set: CCGR_SET,
    #[doc = "0x40d8 - CCM Clock Gating Register"]
    pub ccgr13_clr: CCGR_CLR,
    #[doc = "0x40dc - CCM Clock Gating Register"]
    pub ccgr13_tog: CCGR_TOG,
    #[doc = "0x40e0 - CCM Clock Gating Register"]
    pub ccgr14: CCGR,
    #[doc = "0x40e4 - CCM Clock Gating Register"]
    pub ccgr14_set: CCGR_SET,
    #[doc = "0x40e8 - CCM Clock Gating Register"]
    pub ccgr14_clr: CCGR_CLR,
    #[doc = "0x40ec - CCM Clock Gating Register"]
    pub ccgr14_tog: CCGR_TOG,
    #[doc = "0x40f0 - CCM Clock Gating Register"]
    pub ccgr15: CCGR,
    #[doc = "0x40f4 - CCM Clock Gating Register"]
    pub ccgr15_set: CCGR_SET,
    #[doc = "0x40f8 - CCM Clock Gating Register"]
    pub ccgr15_clr: CCGR_CLR,
    #[doc = "0x40fc - CCM Clock Gating Register"]
    pub ccgr15_tog: CCGR_TOG,
    #[doc = "0x4100 - CCM Clock Gating Register"]
    pub ccgr16: CCGR,
    #[doc = "0x4104 - CCM Clock Gating Register"]
    pub ccgr16_set: CCGR_SET,
    #[doc = "0x4108 - CCM Clock Gating Register"]
    pub ccgr16_clr: CCGR_CLR,
    #[doc = "0x410c - CCM Clock Gating Register"]
    pub ccgr16_tog: CCGR_TOG,
    #[doc = "0x4110 - CCM Clock Gating Register"]
    pub ccgr17: CCGR,
    #[doc = "0x4114 - CCM Clock Gating Register"]
    pub ccgr17_set: CCGR_SET,
    #[doc = "0x4118 - CCM Clock Gating Register"]
    pub ccgr17_clr: CCGR_CLR,
    #[doc = "0x411c - CCM Clock Gating Register"]
    pub ccgr17_tog: CCGR_TOG,
    #[doc = "0x4120 - CCM Clock Gating Register"]
    pub ccgr18: CCGR,
    #[doc = "0x4124 - CCM Clock Gating Register"]
    pub ccgr18_set: CCGR_SET,
    #[doc = "0x4128 - CCM Clock Gating Register"]
    pub ccgr18_clr: CCGR_CLR,
    #[doc = "0x412c - CCM Clock Gating Register"]
    pub ccgr18_tog: CCGR_TOG,
    #[doc = "0x4130 - CCM Clock Gating Register"]
    pub ccgr19: CCGR,
    #[doc = "0x4134 - CCM Clock Gating Register"]
    pub ccgr19_set: CCGR_SET,
    #[doc = "0x4138 - CCM Clock Gating Register"]
    pub ccgr19_clr: CCGR_CLR,
    #[doc = "0x413c - CCM Clock Gating Register"]
    pub ccgr19_tog: CCGR_TOG,
    #[doc = "0x4140 - CCM Clock Gating Register"]
    pub ccgr20: CCGR,
    #[doc = "0x4144 - CCM Clock Gating Register"]
    pub ccgr20_set: CCGR_SET,
    #[doc = "0x4148 - CCM Clock Gating Register"]
    pub ccgr20_clr: CCGR_CLR,
    #[doc = "0x414c - CCM Clock Gating Register"]
    pub ccgr20_tog: CCGR_TOG,
    #[doc = "0x4150 - CCM Clock Gating Register"]
    pub ccgr21: CCGR,
    #[doc = "0x4154 - CCM Clock Gating Register"]
    pub ccgr21_set: CCGR_SET,
    #[doc = "0x4158 - CCM Clock Gating Register"]
    pub ccgr21_clr: CCGR_CLR,
    #[doc = "0x415c - CCM Clock Gating Register"]
    pub ccgr21_tog: CCGR_TOG,
    #[doc = "0x4160 - CCM Clock Gating Register"]
    pub ccgr22: CCGR,
    #[doc = "0x4164 - CCM Clock Gating Register"]
    pub ccgr22_set: CCGR_SET,
    #[doc = "0x4168 - CCM Clock Gating Register"]
    pub ccgr22_clr: CCGR_CLR,
    #[doc = "0x416c - CCM Clock Gating Register"]
    pub ccgr22_tog: CCGR_TOG,
    #[doc = "0x4170 - CCM Clock Gating Register"]
    pub ccgr23: CCGR,
    #[doc = "0x4174 - CCM Clock Gating Register"]
    pub ccgr23_set: CCGR_SET,
    #[doc = "0x4178 - CCM Clock Gating Register"]
    pub ccgr23_clr: CCGR_CLR,
    #[doc = "0x417c - CCM Clock Gating Register"]
    pub ccgr23_tog: CCGR_TOG,
    #[doc = "0x4180 - CCM Clock Gating Register"]
    pub ccgr24: CCGR,
    #[doc = "0x4184 - CCM Clock Gating Register"]
    pub ccgr24_set: CCGR_SET,
    #[doc = "0x4188 - CCM Clock Gating Register"]
    pub ccgr24_clr: CCGR_CLR,
    #[doc = "0x418c - CCM Clock Gating Register"]
    pub ccgr24_tog: CCGR_TOG,
    #[doc = "0x4190 - CCM Clock Gating Register"]
    pub ccgr25: CCGR,
    #[doc = "0x4194 - CCM Clock Gating Register"]
    pub ccgr25_set: CCGR_SET,
    #[doc = "0x4198 - CCM Clock Gating Register"]
    pub ccgr25_clr: CCGR_CLR,
    #[doc = "0x419c - CCM Clock Gating Register"]
    pub ccgr25_tog: CCGR_TOG,
    #[doc = "0x41a0 - CCM Clock Gating Register"]
    pub ccgr26: CCGR,
    #[doc = "0x41a4 - CCM Clock Gating Register"]
    pub ccgr26_set: CCGR_SET,
    #[doc = "0x41a8 - CCM Clock Gating Register"]
    pub ccgr26_clr: CCGR_CLR,
    #[doc = "0x41ac - CCM Clock Gating Register"]
    pub ccgr26_tog: CCGR_TOG,
    #[doc = "0x41b0 - CCM Clock Gating Register"]
    pub ccgr27: CCGR,
    #[doc = "0x41b4 - CCM Clock Gating Register"]
    pub ccgr27_set: CCGR_SET,
    #[doc = "0x41b8 - CCM Clock Gating Register"]
    pub ccgr27_clr: CCGR_CLR,
    #[doc = "0x41bc - CCM Clock Gating Register"]
    pub ccgr27_tog: CCGR_TOG,
    #[doc = "0x41c0 - CCM Clock Gating Register"]
    pub ccgr28: CCGR,
    #[doc = "0x41c4 - CCM Clock Gating Register"]
    pub ccgr28_set: CCGR_SET,
    #[doc = "0x41c8 - CCM Clock Gating Register"]
    pub ccgr28_clr: CCGR_CLR,
    #[doc = "0x41cc - CCM Clock Gating Register"]
    pub ccgr28_tog: CCGR_TOG,
    #[doc = "0x41d0 - CCM Clock Gating Register"]
    pub ccgr29: CCGR,
    #[doc = "0x41d4 - CCM Clock Gating Register"]
    pub ccgr29_set: CCGR_SET,
    #[doc = "0x41d8 - CCM Clock Gating Register"]
    pub ccgr29_clr: CCGR_CLR,
    #[doc = "0x41dc - CCM Clock Gating Register"]
    pub ccgr29_tog: CCGR_TOG,
    #[doc = "0x41e0 - CCM Clock Gating Register"]
    pub ccgr30: CCGR,
    #[doc = "0x41e4 - CCM Clock Gating Register"]
    pub ccgr30_set: CCGR_SET,
    #[doc = "0x41e8 - CCM Clock Gating Register"]
    pub ccgr30_clr: CCGR_CLR,
    #[doc = "0x41ec - CCM Clock Gating Register"]
    pub ccgr30_tog: CCGR_TOG,
    #[doc = "0x41f0 - CCM Clock Gating Register"]
    pub ccgr31: CCGR,
    #[doc = "0x41f4 - CCM Clock Gating Register"]
    pub ccgr31_set: CCGR_SET,
    #[doc = "0x41f8 - CCM Clock Gating Register"]
    pub ccgr31_clr: CCGR_CLR,
    #[doc = "0x41fc - CCM Clock Gating Register"]
    pub ccgr31_tog: CCGR_TOG,
    #[doc = "0x4200 - CCM Clock Gating Register"]
    pub ccgr32: CCGR,
    #[doc = "0x4204 - CCM Clock Gating Register"]
    pub ccgr32_set: CCGR_SET,
    #[doc = "0x4208 - CCM Clock Gating Register"]
    pub ccgr32_clr: CCGR_CLR,
    #[doc = "0x420c - CCM Clock Gating Register"]
    pub ccgr32_tog: CCGR_TOG,
    #[doc = "0x4210 - CCM Clock Gating Register"]
    pub ccgr33: CCGR,
    #[doc = "0x4214 - CCM Clock Gating Register"]
    pub ccgr33_set: CCGR_SET,
    #[doc = "0x4218 - CCM Clock Gating Register"]
    pub ccgr33_clr: CCGR_CLR,
    #[doc = "0x421c - CCM Clock Gating Register"]
    pub ccgr33_tog: CCGR_TOG,
    #[doc = "0x4220 - CCM Clock Gating Register"]
    pub ccgr34: CCGR,
    #[doc = "0x4224 - CCM Clock Gating Register"]
    pub ccgr34_set: CCGR_SET,
    #[doc = "0x4228 - CCM Clock Gating Register"]
    pub ccgr34_clr: CCGR_CLR,
    #[doc = "0x422c - CCM Clock Gating Register"]
    pub ccgr34_tog: CCGR_TOG,
    #[doc = "0x4230 - CCM Clock Gating Register"]
    pub ccgr35: CCGR,
    #[doc = "0x4234 - CCM Clock Gating Register"]
    pub ccgr35_set: CCGR_SET,
    #[doc = "0x4238 - CCM Clock Gating Register"]
    pub ccgr35_clr: CCGR_CLR,
    #[doc = "0x423c - CCM Clock Gating Register"]
    pub ccgr35_tog: CCGR_TOG,
    #[doc = "0x4240 - CCM Clock Gating Register"]
    pub ccgr36: CCGR,
    #[doc = "0x4244 - CCM Clock Gating Register"]
    pub ccgr36_set: CCGR_SET,
    #[doc = "0x4248 - CCM Clock Gating Register"]
    pub ccgr36_clr: CCGR_CLR,
    #[doc = "0x424c - CCM Clock Gating Register"]
    pub ccgr36_tog: CCGR_TOG,
    #[doc = "0x4250 - CCM Clock Gating Register"]
    pub ccgr37: CCGR,
    #[doc = "0x4254 - CCM Clock Gating Register"]
    pub ccgr37_set: CCGR_SET,
    #[doc = "0x4258 - CCM Clock Gating Register"]
    pub ccgr37_clr: CCGR_CLR,
    #[doc = "0x425c - CCM Clock Gating Register"]
    pub ccgr37_tog: CCGR_TOG,
    #[doc = "0x4260 - CCM Clock Gating Register"]
    pub ccgr38: CCGR,
    #[doc = "0x4264 - CCM Clock Gating Register"]
    pub ccgr38_set: CCGR_SET,
    #[doc = "0x4268 - CCM Clock Gating Register"]
    pub ccgr38_clr: CCGR_CLR,
    #[doc = "0x426c - CCM Clock Gating Register"]
    pub ccgr38_tog: CCGR_TOG,
    #[doc = "0x4270 - CCM Clock Gating Register"]
    pub ccgr39: CCGR,
    #[doc = "0x4274 - CCM Clock Gating Register"]
    pub ccgr39_set: CCGR_SET,
    #[doc = "0x4278 - CCM Clock Gating Register"]
    pub ccgr39_clr: CCGR_CLR,
    #[doc = "0x427c - CCM Clock Gating Register"]
    pub ccgr39_tog: CCGR_TOG,
    #[doc = "0x4280 - CCM Clock Gating Register"]
    pub ccgr40: CCGR,
    #[doc = "0x4284 - CCM Clock Gating Register"]
    pub ccgr40_set: CCGR_SET,
    #[doc = "0x4288 - CCM Clock Gating Register"]
    pub ccgr40_clr: CCGR_CLR,
    #[doc = "0x428c - CCM Clock Gating Register"]
    pub ccgr40_tog: CCGR_TOG,
    #[doc = "0x4290 - CCM Clock Gating Register"]
    pub ccgr41: CCGR,
    #[doc = "0x4294 - CCM Clock Gating Register"]
    pub ccgr41_set: CCGR_SET,
    #[doc = "0x4298 - CCM Clock Gating Register"]
    pub ccgr41_clr: CCGR_CLR,
    #[doc = "0x429c - CCM Clock Gating Register"]
    pub ccgr41_tog: CCGR_TOG,
    #[doc = "0x42a0 - CCM Clock Gating Register"]
    pub ccgr42: CCGR,
    #[doc = "0x42a4 - CCM Clock Gating Register"]
    pub ccgr42_set: CCGR_SET,
    #[doc = "0x42a8 - CCM Clock Gating Register"]
    pub ccgr42_clr: CCGR_CLR,
    #[doc = "0x42ac - CCM Clock Gating Register"]
    pub ccgr42_tog: CCGR_TOG,
    #[doc = "0x42b0 - CCM Clock Gating Register"]
    pub ccgr43: CCGR,
    #[doc = "0x42b4 - CCM Clock Gating Register"]
    pub ccgr43_set: CCGR_SET,
    #[doc = "0x42b8 - CCM Clock Gating Register"]
    pub ccgr43_clr: CCGR_CLR,
    #[doc = "0x42bc - CCM Clock Gating Register"]
    pub ccgr43_tog: CCGR_TOG,
    #[doc = "0x42c0 - CCM Clock Gating Register"]
    pub ccgr44: CCGR,
    #[doc = "0x42c4 - CCM Clock Gating Register"]
    pub ccgr44_set: CCGR_SET,
    #[doc = "0x42c8 - CCM Clock Gating Register"]
    pub ccgr44_clr: CCGR_CLR,
    #[doc = "0x42cc - CCM Clock Gating Register"]
    pub ccgr44_tog: CCGR_TOG,
    #[doc = "0x42d0 - CCM Clock Gating Register"]
    pub ccgr45: CCGR,
    #[doc = "0x42d4 - CCM Clock Gating Register"]
    pub ccgr45_set: CCGR_SET,
    #[doc = "0x42d8 - CCM Clock Gating Register"]
    pub ccgr45_clr: CCGR_CLR,
    #[doc = "0x42dc - CCM Clock Gating Register"]
    pub ccgr45_tog: CCGR_TOG,
    #[doc = "0x42e0 - CCM Clock Gating Register"]
    pub ccgr46: CCGR,
    #[doc = "0x42e4 - CCM Clock Gating Register"]
    pub ccgr46_set: CCGR_SET,
    #[doc = "0x42e8 - CCM Clock Gating Register"]
    pub ccgr46_clr: CCGR_CLR,
    #[doc = "0x42ec - CCM Clock Gating Register"]
    pub ccgr46_tog: CCGR_TOG,
    #[doc = "0x42f0 - CCM Clock Gating Register"]
    pub ccgr47: CCGR,
    #[doc = "0x42f4 - CCM Clock Gating Register"]
    pub ccgr47_set: CCGR_SET,
    #[doc = "0x42f8 - CCM Clock Gating Register"]
    pub ccgr47_clr: CCGR_CLR,
    #[doc = "0x42fc - CCM Clock Gating Register"]
    pub ccgr47_tog: CCGR_TOG,
    #[doc = "0x4300 - CCM Clock Gating Register"]
    pub ccgr48: CCGR,
    #[doc = "0x4304 - CCM Clock Gating Register"]
    pub ccgr48_set: CCGR_SET,
    #[doc = "0x4308 - CCM Clock Gating Register"]
    pub ccgr48_clr: CCGR_CLR,
    #[doc = "0x430c - CCM Clock Gating Register"]
    pub ccgr48_tog: CCGR_TOG,
    #[doc = "0x4310 - CCM Clock Gating Register"]
    pub ccgr49: CCGR,
    #[doc = "0x4314 - CCM Clock Gating Register"]
    pub ccgr49_set: CCGR_SET,
    #[doc = "0x4318 - CCM Clock Gating Register"]
    pub ccgr49_clr: CCGR_CLR,
    #[doc = "0x431c - CCM Clock Gating Register"]
    pub ccgr49_tog: CCGR_TOG,
    #[doc = "0x4320 - CCM Clock Gating Register"]
    pub ccgr50: CCGR,
    #[doc = "0x4324 - CCM Clock Gating Register"]
    pub ccgr50_set: CCGR_SET,
    #[doc = "0x4328 - CCM Clock Gating Register"]
    pub ccgr50_clr: CCGR_CLR,
    #[doc = "0x432c - CCM Clock Gating Register"]
    pub ccgr50_tog: CCGR_TOG,
    #[doc = "0x4330 - CCM Clock Gating Register"]
    pub ccgr51: CCGR,
    #[doc = "0x4334 - CCM Clock Gating Register"]
    pub ccgr51_set: CCGR_SET,
    #[doc = "0x4338 - CCM Clock Gating Register"]
    pub ccgr51_clr: CCGR_CLR,
    #[doc = "0x433c - CCM Clock Gating Register"]
    pub ccgr51_tog: CCGR_TOG,
    #[doc = "0x4340 - CCM Clock Gating Register"]
    pub ccgr52: CCGR,
    #[doc = "0x4344 - CCM Clock Gating Register"]
    pub ccgr52_set: CCGR_SET,
    #[doc = "0x4348 - CCM Clock Gating Register"]
    pub ccgr52_clr: CCGR_CLR,
    #[doc = "0x434c - CCM Clock Gating Register"]
    pub ccgr52_tog: CCGR_TOG,
    #[doc = "0x4350 - CCM Clock Gating Register"]
    pub ccgr53: CCGR,
    #[doc = "0x4354 - CCM Clock Gating Register"]
    pub ccgr53_set: CCGR_SET,
    #[doc = "0x4358 - CCM Clock Gating Register"]
    pub ccgr53_clr: CCGR_CLR,
    #[doc = "0x435c - CCM Clock Gating Register"]
    pub ccgr53_tog: CCGR_TOG,
    #[doc = "0x4360 - CCM Clock Gating Register"]
    pub ccgr54: CCGR,
    #[doc = "0x4364 - CCM Clock Gating Register"]
    pub ccgr54_set: CCGR_SET,
    #[doc = "0x4368 - CCM Clock Gating Register"]
    pub ccgr54_clr: CCGR_CLR,
    #[doc = "0x436c - CCM Clock Gating Register"]
    pub ccgr54_tog: CCGR_TOG,
    #[doc = "0x4370 - CCM Clock Gating Register"]
    pub ccgr55: CCGR,
    #[doc = "0x4374 - CCM Clock Gating Register"]
    pub ccgr55_set: CCGR_SET,
    #[doc = "0x4378 - CCM Clock Gating Register"]
    pub ccgr55_clr: CCGR_CLR,
    #[doc = "0x437c - CCM Clock Gating Register"]
    pub ccgr55_tog: CCGR_TOG,
    #[doc = "0x4380 - CCM Clock Gating Register"]
    pub ccgr56: CCGR,
    #[doc = "0x4384 - CCM Clock Gating Register"]
    pub ccgr56_set: CCGR_SET,
    #[doc = "0x4388 - CCM Clock Gating Register"]
    pub ccgr56_clr: CCGR_CLR,
    #[doc = "0x438c - CCM Clock Gating Register"]
    pub ccgr56_tog: CCGR_TOG,
    #[doc = "0x4390 - CCM Clock Gating Register"]
    pub ccgr57: CCGR,
    #[doc = "0x4394 - CCM Clock Gating Register"]
    pub ccgr57_set: CCGR_SET,
    #[doc = "0x4398 - CCM Clock Gating Register"]
    pub ccgr57_clr: CCGR_CLR,
    #[doc = "0x439c - CCM Clock Gating Register"]
    pub ccgr57_tog: CCGR_TOG,
    #[doc = "0x43a0 - CCM Clock Gating Register"]
    pub ccgr58: CCGR,
    #[doc = "0x43a4 - CCM Clock Gating Register"]
    pub ccgr58_set: CCGR_SET,
    #[doc = "0x43a8 - CCM Clock Gating Register"]
    pub ccgr58_clr: CCGR_CLR,
    #[doc = "0x43ac - CCM Clock Gating Register"]
    pub ccgr58_tog: CCGR_TOG,
    #[doc = "0x43b0 - CCM Clock Gating Register"]
    pub ccgr59: CCGR,
    #[doc = "0x43b4 - CCM Clock Gating Register"]
    pub ccgr59_set: CCGR_SET,
    #[doc = "0x43b8 - CCM Clock Gating Register"]
    pub ccgr59_clr: CCGR_CLR,
    #[doc = "0x43bc - CCM Clock Gating Register"]
    pub ccgr59_tog: CCGR_TOG,
    #[doc = "0x43c0 - CCM Clock Gating Register"]
    pub ccgr60: CCGR,
    #[doc = "0x43c4 - CCM Clock Gating Register"]
    pub ccgr60_set: CCGR_SET,
    #[doc = "0x43c8 - CCM Clock Gating Register"]
    pub ccgr60_clr: CCGR_CLR,
    #[doc = "0x43cc - CCM Clock Gating Register"]
    pub ccgr60_tog: CCGR_TOG,
    #[doc = "0x43d0 - CCM Clock Gating Register"]
    pub ccgr61: CCGR,
    #[doc = "0x43d4 - CCM Clock Gating Register"]
    pub ccgr61_set: CCGR_SET,
    #[doc = "0x43d8 - CCM Clock Gating Register"]
    pub ccgr61_clr: CCGR_CLR,
    #[doc = "0x43dc - CCM Clock Gating Register"]
    pub ccgr61_tog: CCGR_TOG,
    #[doc = "0x43e0 - CCM Clock Gating Register"]
    pub ccgr62: CCGR,
    #[doc = "0x43e4 - CCM Clock Gating Register"]
    pub ccgr62_set: CCGR_SET,
    #[doc = "0x43e8 - CCM Clock Gating Register"]
    pub ccgr62_clr: CCGR_CLR,
    #[doc = "0x43ec - CCM Clock Gating Register"]
    pub ccgr62_tog: CCGR_TOG,
    #[doc = "0x43f0 - CCM Clock Gating Register"]
    pub ccgr63: CCGR,
    #[doc = "0x43f4 - CCM Clock Gating Register"]
    pub ccgr63_set: CCGR_SET,
    #[doc = "0x43f8 - CCM Clock Gating Register"]
    pub ccgr63_clr: CCGR_CLR,
    #[doc = "0x43fc - CCM Clock Gating Register"]
    pub ccgr63_tog: CCGR_TOG,
    #[doc = "0x4400 - CCM Clock Gating Register"]
    pub ccgr64: CCGR,
    #[doc = "0x4404 - CCM Clock Gating Register"]
    pub ccgr64_set: CCGR_SET,
    #[doc = "0x4408 - CCM Clock Gating Register"]
    pub ccgr64_clr: CCGR_CLR,
    #[doc = "0x440c - CCM Clock Gating Register"]
    pub ccgr64_tog: CCGR_TOG,
    #[doc = "0x4410 - CCM Clock Gating Register"]
    pub ccgr65: CCGR,
    #[doc = "0x4414 - CCM Clock Gating Register"]
    pub ccgr65_set: CCGR_SET,
    #[doc = "0x4418 - CCM Clock Gating Register"]
    pub ccgr65_clr: CCGR_CLR,
    #[doc = "0x441c - CCM Clock Gating Register"]
    pub ccgr65_tog: CCGR_TOG,
    #[doc = "0x4420 - CCM Clock Gating Register"]
    pub ccgr66: CCGR,
    #[doc = "0x4424 - CCM Clock Gating Register"]
    pub ccgr66_set: CCGR_SET,
    #[doc = "0x4428 - CCM Clock Gating Register"]
    pub ccgr66_clr: CCGR_CLR,
    #[doc = "0x442c - CCM Clock Gating Register"]
    pub ccgr66_tog: CCGR_TOG,
    #[doc = "0x4430 - CCM Clock Gating Register"]
    pub ccgr67: CCGR,
    #[doc = "0x4434 - CCM Clock Gating Register"]
    pub ccgr67_set: CCGR_SET,
    #[doc = "0x4438 - CCM Clock Gating Register"]
    pub ccgr67_clr: CCGR_CLR,
    #[doc = "0x443c - CCM Clock Gating Register"]
    pub ccgr67_tog: CCGR_TOG,
    #[doc = "0x4440 - CCM Clock Gating Register"]
    pub ccgr68: CCGR,
    #[doc = "0x4444 - CCM Clock Gating Register"]
    pub ccgr68_set: CCGR_SET,
    #[doc = "0x4448 - CCM Clock Gating Register"]
    pub ccgr68_clr: CCGR_CLR,
    #[doc = "0x444c - CCM Clock Gating Register"]
    pub ccgr68_tog: CCGR_TOG,
    #[doc = "0x4450 - CCM Clock Gating Register"]
    pub ccgr69: CCGR,
    #[doc = "0x4454 - CCM Clock Gating Register"]
    pub ccgr69_set: CCGR_SET,
    #[doc = "0x4458 - CCM Clock Gating Register"]
    pub ccgr69_clr: CCGR_CLR,
    #[doc = "0x445c - CCM Clock Gating Register"]
    pub ccgr69_tog: CCGR_TOG,
    #[doc = "0x4460 - CCM Clock Gating Register"]
    pub ccgr70: CCGR,
    #[doc = "0x4464 - CCM Clock Gating Register"]
    pub ccgr70_set: CCGR_SET,
    #[doc = "0x4468 - CCM Clock Gating Register"]
    pub ccgr70_clr: CCGR_CLR,
    #[doc = "0x446c - CCM Clock Gating Register"]
    pub ccgr70_tog: CCGR_TOG,
    #[doc = "0x4470 - CCM Clock Gating Register"]
    pub ccgr71: CCGR,
    #[doc = "0x4474 - CCM Clock Gating Register"]
    pub ccgr71_set: CCGR_SET,
    #[doc = "0x4478 - CCM Clock Gating Register"]
    pub ccgr71_clr: CCGR_CLR,
    #[doc = "0x447c - CCM Clock Gating Register"]
    pub ccgr71_tog: CCGR_TOG,
    #[doc = "0x4480 - CCM Clock Gating Register"]
    pub ccgr72: CCGR,
    #[doc = "0x4484 - CCM Clock Gating Register"]
    pub ccgr72_set: CCGR_SET,
    #[doc = "0x4488 - CCM Clock Gating Register"]
    pub ccgr72_clr: CCGR_CLR,
    #[doc = "0x448c - CCM Clock Gating Register"]
    pub ccgr72_tog: CCGR_TOG,
    #[doc = "0x4490 - CCM Clock Gating Register"]
    pub ccgr73: CCGR,
    #[doc = "0x4494 - CCM Clock Gating Register"]
    pub ccgr73_set: CCGR_SET,
    #[doc = "0x4498 - CCM Clock Gating Register"]
    pub ccgr73_clr: CCGR_CLR,
    #[doc = "0x449c - CCM Clock Gating Register"]
    pub ccgr73_tog: CCGR_TOG,
    #[doc = "0x44a0 - CCM Clock Gating Register"]
    pub ccgr74: CCGR,
    #[doc = "0x44a4 - CCM Clock Gating Register"]
    pub ccgr74_set: CCGR_SET,
    #[doc = "0x44a8 - CCM Clock Gating Register"]
    pub ccgr74_clr: CCGR_CLR,
    #[doc = "0x44ac - CCM Clock Gating Register"]
    pub ccgr74_tog: CCGR_TOG,
    #[doc = "0x44b0 - CCM Clock Gating Register"]
    pub ccgr75: CCGR,
    #[doc = "0x44b4 - CCM Clock Gating Register"]
    pub ccgr75_set: CCGR_SET,
    #[doc = "0x44b8 - CCM Clock Gating Register"]
    pub ccgr75_clr: CCGR_CLR,
    #[doc = "0x44bc - CCM Clock Gating Register"]
    pub ccgr75_tog: CCGR_TOG,
    #[doc = "0x44c0 - CCM Clock Gating Register"]
    pub ccgr76: CCGR,
    #[doc = "0x44c4 - CCM Clock Gating Register"]
    pub ccgr76_set: CCGR_SET,
    #[doc = "0x44c8 - CCM Clock Gating Register"]
    pub ccgr76_clr: CCGR_CLR,
    #[doc = "0x44cc - CCM Clock Gating Register"]
    pub ccgr76_tog: CCGR_TOG,
    #[doc = "0x44d0 - CCM Clock Gating Register"]
    pub ccgr77: CCGR,
    #[doc = "0x44d4 - CCM Clock Gating Register"]
    pub ccgr77_set: CCGR_SET,
    #[doc = "0x44d8 - CCM Clock Gating Register"]
    pub ccgr77_clr: CCGR_CLR,
    #[doc = "0x44dc - CCM Clock Gating Register"]
    pub ccgr77_tog: CCGR_TOG,
    #[doc = "0x44e0 - CCM Clock Gating Register"]
    pub ccgr78: CCGR,
    #[doc = "0x44e4 - CCM Clock Gating Register"]
    pub ccgr78_set: CCGR_SET,
    #[doc = "0x44e8 - CCM Clock Gating Register"]
    pub ccgr78_clr: CCGR_CLR,
    #[doc = "0x44ec - CCM Clock Gating Register"]
    pub ccgr78_tog: CCGR_TOG,
    #[doc = "0x44f0 - CCM Clock Gating Register"]
    pub ccgr79: CCGR,
    #[doc = "0x44f4 - CCM Clock Gating Register"]
    pub ccgr79_set: CCGR_SET,
    #[doc = "0x44f8 - CCM Clock Gating Register"]
    pub ccgr79_clr: CCGR_CLR,
    #[doc = "0x44fc - CCM Clock Gating Register"]
    pub ccgr79_tog: CCGR_TOG,
    #[doc = "0x4500 - CCM Clock Gating Register"]
    pub ccgr80: CCGR,
    #[doc = "0x4504 - CCM Clock Gating Register"]
    pub ccgr80_set: CCGR_SET,
    #[doc = "0x4508 - CCM Clock Gating Register"]
    pub ccgr80_clr: CCGR_CLR,
    #[doc = "0x450c - CCM Clock Gating Register"]
    pub ccgr80_tog: CCGR_TOG,
    #[doc = "0x4510 - CCM Clock Gating Register"]
    pub ccgr81: CCGR,
    #[doc = "0x4514 - CCM Clock Gating Register"]
    pub ccgr81_set: CCGR_SET,
    #[doc = "0x4518 - CCM Clock Gating Register"]
    pub ccgr81_clr: CCGR_CLR,
    #[doc = "0x451c - CCM Clock Gating Register"]
    pub ccgr81_tog: CCGR_TOG,
    #[doc = "0x4520 - CCM Clock Gating Register"]
    pub ccgr82: CCGR,
    #[doc = "0x4524 - CCM Clock Gating Register"]
    pub ccgr82_set: CCGR_SET,
    #[doc = "0x4528 - CCM Clock Gating Register"]
    pub ccgr82_clr: CCGR_CLR,
    #[doc = "0x452c - CCM Clock Gating Register"]
    pub ccgr82_tog: CCGR_TOG,
    #[doc = "0x4530 - CCM Clock Gating Register"]
    pub ccgr83: CCGR,
    #[doc = "0x4534 - CCM Clock Gating Register"]
    pub ccgr83_set: CCGR_SET,
    #[doc = "0x4538 - CCM Clock Gating Register"]
    pub ccgr83_clr: CCGR_CLR,
    #[doc = "0x453c - CCM Clock Gating Register"]
    pub ccgr83_tog: CCGR_TOG,
    #[doc = "0x4540 - CCM Clock Gating Register"]
    pub ccgr84: CCGR,
    #[doc = "0x4544 - CCM Clock Gating Register"]
    pub ccgr84_set: CCGR_SET,
    #[doc = "0x4548 - CCM Clock Gating Register"]
    pub ccgr84_clr: CCGR_CLR,
    #[doc = "0x454c - CCM Clock Gating Register"]
    pub ccgr84_tog: CCGR_TOG,
    #[doc = "0x4550 - CCM Clock Gating Register"]
    pub ccgr85: CCGR,
    #[doc = "0x4554 - CCM Clock Gating Register"]
    pub ccgr85_set: CCGR_SET,
    #[doc = "0x4558 - CCM Clock Gating Register"]
    pub ccgr85_clr: CCGR_CLR,
    #[doc = "0x455c - CCM Clock Gating Register"]
    pub ccgr85_tog: CCGR_TOG,
    #[doc = "0x4560 - CCM Clock Gating Register"]
    pub ccgr86: CCGR,
    #[doc = "0x4564 - CCM Clock Gating Register"]
    pub ccgr86_set: CCGR_SET,
    #[doc = "0x4568 - CCM Clock Gating Register"]
    pub ccgr86_clr: CCGR_CLR,
    #[doc = "0x456c - CCM Clock Gating Register"]
    pub ccgr86_tog: CCGR_TOG,
    #[doc = "0x4570 - CCM Clock Gating Register"]
    pub ccgr87: CCGR,
    #[doc = "0x4574 - CCM Clock Gating Register"]
    pub ccgr87_set: CCGR_SET,
    #[doc = "0x4578 - CCM Clock Gating Register"]
    pub ccgr87_clr: CCGR_CLR,
    #[doc = "0x457c - CCM Clock Gating Register"]
    pub ccgr87_tog: CCGR_TOG,
    #[doc = "0x4580 - CCM Clock Gating Register"]
    pub ccgr88: CCGR,
    #[doc = "0x4584 - CCM Clock Gating Register"]
    pub ccgr88_set: CCGR_SET,
    #[doc = "0x4588 - CCM Clock Gating Register"]
    pub ccgr88_clr: CCGR_CLR,
    #[doc = "0x458c - CCM Clock Gating Register"]
    pub ccgr88_tog: CCGR_TOG,
    #[doc = "0x4590 - CCM Clock Gating Register"]
    pub ccgr89: CCGR,
    #[doc = "0x4594 - CCM Clock Gating Register"]
    pub ccgr89_set: CCGR_SET,
    #[doc = "0x4598 - CCM Clock Gating Register"]
    pub ccgr89_clr: CCGR_CLR,
    #[doc = "0x459c - CCM Clock Gating Register"]
    pub ccgr89_tog: CCGR_TOG,
    #[doc = "0x45a0 - CCM Clock Gating Register"]
    pub ccgr90: CCGR,
    #[doc = "0x45a4 - CCM Clock Gating Register"]
    pub ccgr90_set: CCGR_SET,
    #[doc = "0x45a8 - CCM Clock Gating Register"]
    pub ccgr90_clr: CCGR_CLR,
    #[doc = "0x45ac - CCM Clock Gating Register"]
    pub ccgr90_tog: CCGR_TOG,
    #[doc = "0x45b0 - CCM Clock Gating Register"]
    pub ccgr91: CCGR,
    #[doc = "0x45b4 - CCM Clock Gating Register"]
    pub ccgr91_set: CCGR_SET,
    #[doc = "0x45b8 - CCM Clock Gating Register"]
    pub ccgr91_clr: CCGR_CLR,
    #[doc = "0x45bc - CCM Clock Gating Register"]
    pub ccgr91_tog: CCGR_TOG,
    #[doc = "0x45c0 - CCM Clock Gating Register"]
    pub ccgr92: CCGR,
    #[doc = "0x45c4 - CCM Clock Gating Register"]
    pub ccgr92_set: CCGR_SET,
    #[doc = "0x45c8 - CCM Clock Gating Register"]
    pub ccgr92_clr: CCGR_CLR,
    #[doc = "0x45cc - CCM Clock Gating Register"]
    pub ccgr92_tog: CCGR_TOG,
    #[doc = "0x45d0 - CCM Clock Gating Register"]
    pub ccgr93: CCGR,
    #[doc = "0x45d4 - CCM Clock Gating Register"]
    pub ccgr93_set: CCGR_SET,
    #[doc = "0x45d8 - CCM Clock Gating Register"]
    pub ccgr93_clr: CCGR_CLR,
    #[doc = "0x45dc - CCM Clock Gating Register"]
    pub ccgr93_tog: CCGR_TOG,
    #[doc = "0x45e0 - CCM Clock Gating Register"]
    pub ccgr94: CCGR,
    #[doc = "0x45e4 - CCM Clock Gating Register"]
    pub ccgr94_set: CCGR_SET,
    #[doc = "0x45e8 - CCM Clock Gating Register"]
    pub ccgr94_clr: CCGR_CLR,
    #[doc = "0x45ec - CCM Clock Gating Register"]
    pub ccgr94_tog: CCGR_TOG,
    #[doc = "0x45f0 - CCM Clock Gating Register"]
    pub ccgr95: CCGR,
    #[doc = "0x45f4 - CCM Clock Gating Register"]
    pub ccgr95_set: CCGR_SET,
    #[doc = "0x45f8 - CCM Clock Gating Register"]
    pub ccgr95_clr: CCGR_CLR,
    #[doc = "0x45fc - CCM Clock Gating Register"]
    pub ccgr95_tog: CCGR_TOG,
    #[doc = "0x4600 - CCM Clock Gating Register"]
    pub ccgr96: CCGR,
    #[doc = "0x4604 - CCM Clock Gating Register"]
    pub ccgr96_set: CCGR_SET,
    #[doc = "0x4608 - CCM Clock Gating Register"]
    pub ccgr96_clr: CCGR_CLR,
    #[doc = "0x460c - CCM Clock Gating Register"]
    pub ccgr96_tog: CCGR_TOG,
    #[doc = "0x4610 - CCM Clock Gating Register"]
    pub ccgr97: CCGR,
    #[doc = "0x4614 - CCM Clock Gating Register"]
    pub ccgr97_set: CCGR_SET,
    #[doc = "0x4618 - CCM Clock Gating Register"]
    pub ccgr97_clr: CCGR_CLR,
    #[doc = "0x461c - CCM Clock Gating Register"]
    pub ccgr97_tog: CCGR_TOG,
    #[doc = "0x4620 - CCM Clock Gating Register"]
    pub ccgr98: CCGR,
    #[doc = "0x4624 - CCM Clock Gating Register"]
    pub ccgr98_set: CCGR_SET,
    #[doc = "0x4628 - CCM Clock Gating Register"]
    pub ccgr98_clr: CCGR_CLR,
    #[doc = "0x462c - CCM Clock Gating Register"]
    pub ccgr98_tog: CCGR_TOG,
    #[doc = "0x4630 - CCM Clock Gating Register"]
    pub ccgr99: CCGR,
    #[doc = "0x4634 - CCM Clock Gating Register"]
    pub ccgr99_set: CCGR_SET,
    #[doc = "0x4638 - CCM Clock Gating Register"]
    pub ccgr99_clr: CCGR_CLR,
    #[doc = "0x463c - CCM Clock Gating Register"]
    pub ccgr99_tog: CCGR_TOG,
    #[doc = "0x4640 - CCM Clock Gating Register"]
    pub ccgr100: CCGR,
    #[doc = "0x4644 - CCM Clock Gating Register"]
    pub ccgr100_set: CCGR_SET,
    #[doc = "0x4648 - CCM Clock Gating Register"]
    pub ccgr100_clr: CCGR_CLR,
    #[doc = "0x464c - CCM Clock Gating Register"]
    pub ccgr100_tog: CCGR_TOG,
    #[doc = "0x4650 - CCM Clock Gating Register"]
    pub ccgr101: CCGR,
    #[doc = "0x4654 - CCM Clock Gating Register"]
    pub ccgr101_set: CCGR_SET,
    #[doc = "0x4658 - CCM Clock Gating Register"]
    pub ccgr101_clr: CCGR_CLR,
    #[doc = "0x465c - CCM Clock Gating Register"]
    pub ccgr101_tog: CCGR_TOG,
    #[doc = "0x4660 - CCM Clock Gating Register"]
    pub ccgr102: CCGR,
    #[doc = "0x4664 - CCM Clock Gating Register"]
    pub ccgr102_set: CCGR_SET,
    #[doc = "0x4668 - CCM Clock Gating Register"]
    pub ccgr102_clr: CCGR_CLR,
    #[doc = "0x466c - CCM Clock Gating Register"]
    pub ccgr102_tog: CCGR_TOG,
    #[doc = "0x4670 - CCM Clock Gating Register"]
    pub ccgr103: CCGR,
    #[doc = "0x4674 - CCM Clock Gating Register"]
    pub ccgr103_set: CCGR_SET,
    #[doc = "0x4678 - CCM Clock Gating Register"]
    pub ccgr103_clr: CCGR_CLR,
    #[doc = "0x467c - CCM Clock Gating Register"]
    pub ccgr103_tog: CCGR_TOG,
    #[doc = "0x4680 - CCM Clock Gating Register"]
    pub ccgr104: CCGR,
    #[doc = "0x4684 - CCM Clock Gating Register"]
    pub ccgr104_set: CCGR_SET,
    #[doc = "0x4688 - CCM Clock Gating Register"]
    pub ccgr104_clr: CCGR_CLR,
    #[doc = "0x468c - CCM Clock Gating Register"]
    pub ccgr104_tog: CCGR_TOG,
    #[doc = "0x4690 - CCM Clock Gating Register"]
    pub ccgr105: CCGR,
    #[doc = "0x4694 - CCM Clock Gating Register"]
    pub ccgr105_set: CCGR_SET,
    #[doc = "0x4698 - CCM Clock Gating Register"]
    pub ccgr105_clr: CCGR_CLR,
    #[doc = "0x469c - CCM Clock Gating Register"]
    pub ccgr105_tog: CCGR_TOG,
    #[doc = "0x46a0 - CCM Clock Gating Register"]
    pub ccgr106: CCGR,
    #[doc = "0x46a4 - CCM Clock Gating Register"]
    pub ccgr106_set: CCGR_SET,
    #[doc = "0x46a8 - CCM Clock Gating Register"]
    pub ccgr106_clr: CCGR_CLR,
    #[doc = "0x46ac - CCM Clock Gating Register"]
    pub ccgr106_tog: CCGR_TOG,
    #[doc = "0x46b0 - CCM Clock Gating Register"]
    pub ccgr107: CCGR,
    #[doc = "0x46b4 - CCM Clock Gating Register"]
    pub ccgr107_set: CCGR_SET,
    #[doc = "0x46b8 - CCM Clock Gating Register"]
    pub ccgr107_clr: CCGR_CLR,
    #[doc = "0x46bc - CCM Clock Gating Register"]
    pub ccgr107_tog: CCGR_TOG,
    #[doc = "0x46c0 - CCM Clock Gating Register"]
    pub ccgr108: CCGR,
    #[doc = "0x46c4 - CCM Clock Gating Register"]
    pub ccgr108_set: CCGR_SET,
    #[doc = "0x46c8 - CCM Clock Gating Register"]
    pub ccgr108_clr: CCGR_CLR,
    #[doc = "0x46cc - CCM Clock Gating Register"]
    pub ccgr108_tog: CCGR_TOG,
    #[doc = "0x46d0 - CCM Clock Gating Register"]
    pub ccgr109: CCGR,
    #[doc = "0x46d4 - CCM Clock Gating Register"]
    pub ccgr109_set: CCGR_SET,
    #[doc = "0x46d8 - CCM Clock Gating Register"]
    pub ccgr109_clr: CCGR_CLR,
    #[doc = "0x46dc - CCM Clock Gating Register"]
    pub ccgr109_tog: CCGR_TOG,
    #[doc = "0x46e0 - CCM Clock Gating Register"]
    pub ccgr110: CCGR,
    #[doc = "0x46e4 - CCM Clock Gating Register"]
    pub ccgr110_set: CCGR_SET,
    #[doc = "0x46e8 - CCM Clock Gating Register"]
    pub ccgr110_clr: CCGR_CLR,
    #[doc = "0x46ec - CCM Clock Gating Register"]
    pub ccgr110_tog: CCGR_TOG,
    #[doc = "0x46f0 - CCM Clock Gating Register"]
    pub ccgr111: CCGR,
    #[doc = "0x46f4 - CCM Clock Gating Register"]
    pub ccgr111_set: CCGR_SET,
    #[doc = "0x46f8 - CCM Clock Gating Register"]
    pub ccgr111_clr: CCGR_CLR,
    #[doc = "0x46fc - CCM Clock Gating Register"]
    pub ccgr111_tog: CCGR_TOG,
    #[doc = "0x4700 - CCM Clock Gating Register"]
    pub ccgr112: CCGR,
    #[doc = "0x4704 - CCM Clock Gating Register"]
    pub ccgr112_set: CCGR_SET,
    #[doc = "0x4708 - CCM Clock Gating Register"]
    pub ccgr112_clr: CCGR_CLR,
    #[doc = "0x470c - CCM Clock Gating Register"]
    pub ccgr112_tog: CCGR_TOG,
    #[doc = "0x4710 - CCM Clock Gating Register"]
    pub ccgr113: CCGR,
    #[doc = "0x4714 - CCM Clock Gating Register"]
    pub ccgr113_set: CCGR_SET,
    #[doc = "0x4718 - CCM Clock Gating Register"]
    pub ccgr113_clr: CCGR_CLR,
    #[doc = "0x471c - CCM Clock Gating Register"]
    pub ccgr113_tog: CCGR_TOG,
    #[doc = "0x4720 - CCM Clock Gating Register"]
    pub ccgr114: CCGR,
    #[doc = "0x4724 - CCM Clock Gating Register"]
    pub ccgr114_set: CCGR_SET,
    #[doc = "0x4728 - CCM Clock Gating Register"]
    pub ccgr114_clr: CCGR_CLR,
    #[doc = "0x472c - CCM Clock Gating Register"]
    pub ccgr114_tog: CCGR_TOG,
    #[doc = "0x4730 - CCM Clock Gating Register"]
    pub ccgr115: CCGR,
    #[doc = "0x4734 - CCM Clock Gating Register"]
    pub ccgr115_set: CCGR_SET,
    #[doc = "0x4738 - CCM Clock Gating Register"]
    pub ccgr115_clr: CCGR_CLR,
    #[doc = "0x473c - CCM Clock Gating Register"]
    pub ccgr115_tog: CCGR_TOG,
    #[doc = "0x4740 - CCM Clock Gating Register"]
    pub ccgr116: CCGR,
    #[doc = "0x4744 - CCM Clock Gating Register"]
    pub ccgr116_set: CCGR_SET,
    #[doc = "0x4748 - CCM Clock Gating Register"]
    pub ccgr116_clr: CCGR_CLR,
    #[doc = "0x474c - CCM Clock Gating Register"]
    pub ccgr116_tog: CCGR_TOG,
    #[doc = "0x4750 - CCM Clock Gating Register"]
    pub ccgr117: CCGR,
    #[doc = "0x4754 - CCM Clock Gating Register"]
    pub ccgr117_set: CCGR_SET,
    #[doc = "0x4758 - CCM Clock Gating Register"]
    pub ccgr117_clr: CCGR_CLR,
    #[doc = "0x475c - CCM Clock Gating Register"]
    pub ccgr117_tog: CCGR_TOG,
    #[doc = "0x4760 - CCM Clock Gating Register"]
    pub ccgr118: CCGR,
    #[doc = "0x4764 - CCM Clock Gating Register"]
    pub ccgr118_set: CCGR_SET,
    #[doc = "0x4768 - CCM Clock Gating Register"]
    pub ccgr118_clr: CCGR_CLR,
    #[doc = "0x476c - CCM Clock Gating Register"]
    pub ccgr118_tog: CCGR_TOG,
    #[doc = "0x4770 - CCM Clock Gating Register"]
    pub ccgr119: CCGR,
    #[doc = "0x4774 - CCM Clock Gating Register"]
    pub ccgr119_set: CCGR_SET,
    #[doc = "0x4778 - CCM Clock Gating Register"]
    pub ccgr119_clr: CCGR_CLR,
    #[doc = "0x477c - CCM Clock Gating Register"]
    pub ccgr119_tog: CCGR_TOG,
    #[doc = "0x4780 - CCM Clock Gating Register"]
    pub ccgr120: CCGR,
    #[doc = "0x4784 - CCM Clock Gating Register"]
    pub ccgr120_set: CCGR_SET,
    #[doc = "0x4788 - CCM Clock Gating Register"]
    pub ccgr120_clr: CCGR_CLR,
    #[doc = "0x478c - CCM Clock Gating Register"]
    pub ccgr120_tog: CCGR_TOG,
    #[doc = "0x4790 - CCM Clock Gating Register"]
    pub ccgr121: CCGR,
    #[doc = "0x4794 - CCM Clock Gating Register"]
    pub ccgr121_set: CCGR_SET,
    #[doc = "0x4798 - CCM Clock Gating Register"]
    pub ccgr121_clr: CCGR_CLR,
    #[doc = "0x479c - CCM Clock Gating Register"]
    pub ccgr121_tog: CCGR_TOG,
    #[doc = "0x47a0 - CCM Clock Gating Register"]
    pub ccgr122: CCGR,
    #[doc = "0x47a4 - CCM Clock Gating Register"]
    pub ccgr122_set: CCGR_SET,
    #[doc = "0x47a8 - CCM Clock Gating Register"]
    pub ccgr122_clr: CCGR_CLR,
    #[doc = "0x47ac - CCM Clock Gating Register"]
    pub ccgr122_tog: CCGR_TOG,
    #[doc = "0x47b0 - CCM Clock Gating Register"]
    pub ccgr123: CCGR,
    #[doc = "0x47b4 - CCM Clock Gating Register"]
    pub ccgr123_set: CCGR_SET,
    #[doc = "0x47b8 - CCM Clock Gating Register"]
    pub ccgr123_clr: CCGR_CLR,
    #[doc = "0x47bc - CCM Clock Gating Register"]
    pub ccgr123_tog: CCGR_TOG,
    #[doc = "0x47c0 - CCM Clock Gating Register"]
    pub ccgr124: CCGR,
    #[doc = "0x47c4 - CCM Clock Gating Register"]
    pub ccgr124_set: CCGR_SET,
    #[doc = "0x47c8 - CCM Clock Gating Register"]
    pub ccgr124_clr: CCGR_CLR,
    #[doc = "0x47cc - CCM Clock Gating Register"]
    pub ccgr124_tog: CCGR_TOG,
    #[doc = "0x47d0 - CCM Clock Gating Register"]
    pub ccgr125: CCGR,
    #[doc = "0x47d4 - CCM Clock Gating Register"]
    pub ccgr125_set: CCGR_SET,
    #[doc = "0x47d8 - CCM Clock Gating Register"]
    pub ccgr125_clr: CCGR_CLR,
    #[doc = "0x47dc - CCM Clock Gating Register"]
    pub ccgr125_tog: CCGR_TOG,
    #[doc = "0x47e0 - CCM Clock Gating Register"]
    pub ccgr126: CCGR,
    #[doc = "0x47e4 - CCM Clock Gating Register"]
    pub ccgr126_set: CCGR_SET,
    #[doc = "0x47e8 - CCM Clock Gating Register"]
    pub ccgr126_clr: CCGR_CLR,
    #[doc = "0x47ec - CCM Clock Gating Register"]
    pub ccgr126_tog: CCGR_TOG,
    #[doc = "0x47f0 - CCM Clock Gating Register"]
    pub ccgr127: CCGR,
    #[doc = "0x47f4 - CCM Clock Gating Register"]
    pub ccgr127_set: CCGR_SET,
    #[doc = "0x47f8 - CCM Clock Gating Register"]
    pub ccgr127_clr: CCGR_CLR,
    #[doc = "0x47fc - CCM Clock Gating Register"]
    pub ccgr127_tog: CCGR_TOG,
    #[doc = "0x4800 - CCM Clock Gating Register"]
    pub ccgr128: CCGR,
    #[doc = "0x4804 - CCM Clock Gating Register"]
    pub ccgr128_set: CCGR_SET,
    #[doc = "0x4808 - CCM Clock Gating Register"]
    pub ccgr128_clr: CCGR_CLR,
    #[doc = "0x480c - CCM Clock Gating Register"]
    pub ccgr128_tog: CCGR_TOG,
    #[doc = "0x4810 - CCM Clock Gating Register"]
    pub ccgr129: CCGR,
    #[doc = "0x4814 - CCM Clock Gating Register"]
    pub ccgr129_set: CCGR_SET,
    #[doc = "0x4818 - CCM Clock Gating Register"]
    pub ccgr129_clr: CCGR_CLR,
    #[doc = "0x481c - CCM Clock Gating Register"]
    pub ccgr129_tog: CCGR_TOG,
    #[doc = "0x4820 - CCM Clock Gating Register"]
    pub ccgr130: CCGR,
    #[doc = "0x4824 - CCM Clock Gating Register"]
    pub ccgr130_set: CCGR_SET,
    #[doc = "0x4828 - CCM Clock Gating Register"]
    pub ccgr130_clr: CCGR_CLR,
    #[doc = "0x482c - CCM Clock Gating Register"]
    pub ccgr130_tog: CCGR_TOG,
    #[doc = "0x4830 - CCM Clock Gating Register"]
    pub ccgr131: CCGR,
    #[doc = "0x4834 - CCM Clock Gating Register"]
    pub ccgr131_set: CCGR_SET,
    #[doc = "0x4838 - CCM Clock Gating Register"]
    pub ccgr131_clr: CCGR_CLR,
    #[doc = "0x483c - CCM Clock Gating Register"]
    pub ccgr131_tog: CCGR_TOG,
    #[doc = "0x4840 - CCM Clock Gating Register"]
    pub ccgr132: CCGR,
    #[doc = "0x4844 - CCM Clock Gating Register"]
    pub ccgr132_set: CCGR_SET,
    #[doc = "0x4848 - CCM Clock Gating Register"]
    pub ccgr132_clr: CCGR_CLR,
    #[doc = "0x484c - CCM Clock Gating Register"]
    pub ccgr132_tog: CCGR_TOG,
    #[doc = "0x4850 - CCM Clock Gating Register"]
    pub ccgr133: CCGR,
    #[doc = "0x4854 - CCM Clock Gating Register"]
    pub ccgr133_set: CCGR_SET,
    #[doc = "0x4858 - CCM Clock Gating Register"]
    pub ccgr133_clr: CCGR_CLR,
    #[doc = "0x485c - CCM Clock Gating Register"]
    pub ccgr133_tog: CCGR_TOG,
    #[doc = "0x4860 - CCM Clock Gating Register"]
    pub ccgr134: CCGR,
    #[doc = "0x4864 - CCM Clock Gating Register"]
    pub ccgr134_set: CCGR_SET,
    #[doc = "0x4868 - CCM Clock Gating Register"]
    pub ccgr134_clr: CCGR_CLR,
    #[doc = "0x486c - CCM Clock Gating Register"]
    pub ccgr134_tog: CCGR_TOG,
    #[doc = "0x4870 - CCM Clock Gating Register"]
    pub ccgr135: CCGR,
    #[doc = "0x4874 - CCM Clock Gating Register"]
    pub ccgr135_set: CCGR_SET,
    #[doc = "0x4878 - CCM Clock Gating Register"]
    pub ccgr135_clr: CCGR_CLR,
    #[doc = "0x487c - CCM Clock Gating Register"]
    pub ccgr135_tog: CCGR_TOG,
    #[doc = "0x4880 - CCM Clock Gating Register"]
    pub ccgr136: CCGR,
    #[doc = "0x4884 - CCM Clock Gating Register"]
    pub ccgr136_set: CCGR_SET,
    #[doc = "0x4888 - CCM Clock Gating Register"]
    pub ccgr136_clr: CCGR_CLR,
    #[doc = "0x488c - CCM Clock Gating Register"]
    pub ccgr136_tog: CCGR_TOG,
    #[doc = "0x4890 - CCM Clock Gating Register"]
    pub ccgr137: CCGR,
    #[doc = "0x4894 - CCM Clock Gating Register"]
    pub ccgr137_set: CCGR_SET,
    #[doc = "0x4898 - CCM Clock Gating Register"]
    pub ccgr137_clr: CCGR_CLR,
    #[doc = "0x489c - CCM Clock Gating Register"]
    pub ccgr137_tog: CCGR_TOG,
    #[doc = "0x48a0 - CCM Clock Gating Register"]
    pub ccgr138: CCGR,
    #[doc = "0x48a4 - CCM Clock Gating Register"]
    pub ccgr138_set: CCGR_SET,
    #[doc = "0x48a8 - CCM Clock Gating Register"]
    pub ccgr138_clr: CCGR_CLR,
    #[doc = "0x48ac - CCM Clock Gating Register"]
    pub ccgr138_tog: CCGR_TOG,
    #[doc = "0x48b0 - CCM Clock Gating Register"]
    pub ccgr139: CCGR,
    #[doc = "0x48b4 - CCM Clock Gating Register"]
    pub ccgr139_set: CCGR_SET,
    #[doc = "0x48b8 - CCM Clock Gating Register"]
    pub ccgr139_clr: CCGR_CLR,
    #[doc = "0x48bc - CCM Clock Gating Register"]
    pub ccgr139_tog: CCGR_TOG,
    #[doc = "0x48c0 - CCM Clock Gating Register"]
    pub ccgr140: CCGR,
    #[doc = "0x48c4 - CCM Clock Gating Register"]
    pub ccgr140_set: CCGR_SET,
    #[doc = "0x48c8 - CCM Clock Gating Register"]
    pub ccgr140_clr: CCGR_CLR,
    #[doc = "0x48cc - CCM Clock Gating Register"]
    pub ccgr140_tog: CCGR_TOG,
    #[doc = "0x48d0 - CCM Clock Gating Register"]
    pub ccgr141: CCGR,
    #[doc = "0x48d4 - CCM Clock Gating Register"]
    pub ccgr141_set: CCGR_SET,
    #[doc = "0x48d8 - CCM Clock Gating Register"]
    pub ccgr141_clr: CCGR_CLR,
    #[doc = "0x48dc - CCM Clock Gating Register"]
    pub ccgr141_tog: CCGR_TOG,
    #[doc = "0x48e0 - CCM Clock Gating Register"]
    pub ccgr142: CCGR,
    #[doc = "0x48e4 - CCM Clock Gating Register"]
    pub ccgr142_set: CCGR_SET,
    #[doc = "0x48e8 - CCM Clock Gating Register"]
    pub ccgr142_clr: CCGR_CLR,
    #[doc = "0x48ec - CCM Clock Gating Register"]
    pub ccgr142_tog: CCGR_TOG,
    #[doc = "0x48f0 - CCM Clock Gating Register"]
    pub ccgr143: CCGR,
    #[doc = "0x48f4 - CCM Clock Gating Register"]
    pub ccgr143_set: CCGR_SET,
    #[doc = "0x48f8 - CCM Clock Gating Register"]
    pub ccgr143_clr: CCGR_CLR,
    #[doc = "0x48fc - CCM Clock Gating Register"]
    pub ccgr143_tog: CCGR_TOG,
    #[doc = "0x4900 - CCM Clock Gating Register"]
    pub ccgr144: CCGR,
    #[doc = "0x4904 - CCM Clock Gating Register"]
    pub ccgr144_set: CCGR_SET,
    #[doc = "0x4908 - CCM Clock Gating Register"]
    pub ccgr144_clr: CCGR_CLR,
    #[doc = "0x490c - CCM Clock Gating Register"]
    pub ccgr144_tog: CCGR_TOG,
    #[doc = "0x4910 - CCM Clock Gating Register"]
    pub ccgr145: CCGR,
    #[doc = "0x4914 - CCM Clock Gating Register"]
    pub ccgr145_set: CCGR_SET,
    #[doc = "0x4918 - CCM Clock Gating Register"]
    pub ccgr145_clr: CCGR_CLR,
    #[doc = "0x491c - CCM Clock Gating Register"]
    pub ccgr145_tog: CCGR_TOG,
    #[doc = "0x4920 - CCM Clock Gating Register"]
    pub ccgr146: CCGR,
    #[doc = "0x4924 - CCM Clock Gating Register"]
    pub ccgr146_set: CCGR_SET,
    #[doc = "0x4928 - CCM Clock Gating Register"]
    pub ccgr146_clr: CCGR_CLR,
    #[doc = "0x492c - CCM Clock Gating Register"]
    pub ccgr146_tog: CCGR_TOG,
    #[doc = "0x4930 - CCM Clock Gating Register"]
    pub ccgr147: CCGR,
    #[doc = "0x4934 - CCM Clock Gating Register"]
    pub ccgr147_set: CCGR_SET,
    #[doc = "0x4938 - CCM Clock Gating Register"]
    pub ccgr147_clr: CCGR_CLR,
    #[doc = "0x493c - CCM Clock Gating Register"]
    pub ccgr147_tog: CCGR_TOG,
    #[doc = "0x4940 - CCM Clock Gating Register"]
    pub ccgr148: CCGR,
    #[doc = "0x4944 - CCM Clock Gating Register"]
    pub ccgr148_set: CCGR_SET,
    #[doc = "0x4948 - CCM Clock Gating Register"]
    pub ccgr148_clr: CCGR_CLR,
    #[doc = "0x494c - CCM Clock Gating Register"]
    pub ccgr148_tog: CCGR_TOG,
    #[doc = "0x4950 - CCM Clock Gating Register"]
    pub ccgr149: CCGR,
    #[doc = "0x4954 - CCM Clock Gating Register"]
    pub ccgr149_set: CCGR_SET,
    #[doc = "0x4958 - CCM Clock Gating Register"]
    pub ccgr149_clr: CCGR_CLR,
    #[doc = "0x495c - CCM Clock Gating Register"]
    pub ccgr149_tog: CCGR_TOG,
    #[doc = "0x4960 - CCM Clock Gating Register"]
    pub ccgr150: CCGR,
    #[doc = "0x4964 - CCM Clock Gating Register"]
    pub ccgr150_set: CCGR_SET,
    #[doc = "0x4968 - CCM Clock Gating Register"]
    pub ccgr150_clr: CCGR_CLR,
    #[doc = "0x496c - CCM Clock Gating Register"]
    pub ccgr150_tog: CCGR_TOG,
    #[doc = "0x4970 - CCM Clock Gating Register"]
    pub ccgr151: CCGR,
    #[doc = "0x4974 - CCM Clock Gating Register"]
    pub ccgr151_set: CCGR_SET,
    #[doc = "0x4978 - CCM Clock Gating Register"]
    pub ccgr151_clr: CCGR_CLR,
    #[doc = "0x497c - CCM Clock Gating Register"]
    pub ccgr151_tog: CCGR_TOG,
    #[doc = "0x4980 - CCM Clock Gating Register"]
    pub ccgr152: CCGR,
    #[doc = "0x4984 - CCM Clock Gating Register"]
    pub ccgr152_set: CCGR_SET,
    #[doc = "0x4988 - CCM Clock Gating Register"]
    pub ccgr152_clr: CCGR_CLR,
    #[doc = "0x498c - CCM Clock Gating Register"]
    pub ccgr152_tog: CCGR_TOG,
    #[doc = "0x4990 - CCM Clock Gating Register"]
    pub ccgr153: CCGR,
    #[doc = "0x4994 - CCM Clock Gating Register"]
    pub ccgr153_set: CCGR_SET,
    #[doc = "0x4998 - CCM Clock Gating Register"]
    pub ccgr153_clr: CCGR_CLR,
    #[doc = "0x499c - CCM Clock Gating Register"]
    pub ccgr153_tog: CCGR_TOG,
    #[doc = "0x49a0 - CCM Clock Gating Register"]
    pub ccgr154: CCGR,
    #[doc = "0x49a4 - CCM Clock Gating Register"]
    pub ccgr154_set: CCGR_SET,
    #[doc = "0x49a8 - CCM Clock Gating Register"]
    pub ccgr154_clr: CCGR_CLR,
    #[doc = "0x49ac - CCM Clock Gating Register"]
    pub ccgr154_tog: CCGR_TOG,
    #[doc = "0x49b0 - CCM Clock Gating Register"]
    pub ccgr155: CCGR,
    #[doc = "0x49b4 - CCM Clock Gating Register"]
    pub ccgr155_set: CCGR_SET,
    #[doc = "0x49b8 - CCM Clock Gating Register"]
    pub ccgr155_clr: CCGR_CLR,
    #[doc = "0x49bc - CCM Clock Gating Register"]
    pub ccgr155_tog: CCGR_TOG,
    #[doc = "0x49c0 - CCM Clock Gating Register"]
    pub ccgr156: CCGR,
    #[doc = "0x49c4 - CCM Clock Gating Register"]
    pub ccgr156_set: CCGR_SET,
    #[doc = "0x49c8 - CCM Clock Gating Register"]
    pub ccgr156_clr: CCGR_CLR,
    #[doc = "0x49cc - CCM Clock Gating Register"]
    pub ccgr156_tog: CCGR_TOG,
    #[doc = "0x49d0 - CCM Clock Gating Register"]
    pub ccgr157: CCGR,
    #[doc = "0x49d4 - CCM Clock Gating Register"]
    pub ccgr157_set: CCGR_SET,
    #[doc = "0x49d8 - CCM Clock Gating Register"]
    pub ccgr157_clr: CCGR_CLR,
    #[doc = "0x49dc - CCM Clock Gating Register"]
    pub ccgr157_tog: CCGR_TOG,
    #[doc = "0x49e0 - CCM Clock Gating Register"]
    pub ccgr158: CCGR,
    #[doc = "0x49e4 - CCM Clock Gating Register"]
    pub ccgr158_set: CCGR_SET,
    #[doc = "0x49e8 - CCM Clock Gating Register"]
    pub ccgr158_clr: CCGR_CLR,
    #[doc = "0x49ec - CCM Clock Gating Register"]
    pub ccgr158_tog: CCGR_TOG,
    #[doc = "0x49f0 - CCM Clock Gating Register"]
    pub ccgr159: CCGR,
    #[doc = "0x49f4 - CCM Clock Gating Register"]
    pub ccgr159_set: CCGR_SET,
    #[doc = "0x49f8 - CCM Clock Gating Register"]
    pub ccgr159_clr: CCGR_CLR,
    #[doc = "0x49fc - CCM Clock Gating Register"]
    pub ccgr159_tog: CCGR_TOG,
    #[doc = "0x4a00 - CCM Clock Gating Register"]
    pub ccgr160: CCGR,
    #[doc = "0x4a04 - CCM Clock Gating Register"]
    pub ccgr160_set: CCGR_SET,
    #[doc = "0x4a08 - CCM Clock Gating Register"]
    pub ccgr160_clr: CCGR_CLR,
    #[doc = "0x4a0c - CCM Clock Gating Register"]
    pub ccgr160_tog: CCGR_TOG,
    #[doc = "0x4a10 - CCM Clock Gating Register"]
    pub ccgr161: CCGR,
    #[doc = "0x4a14 - CCM Clock Gating Register"]
    pub ccgr161_set: CCGR_SET,
    #[doc = "0x4a18 - CCM Clock Gating Register"]
    pub ccgr161_clr: CCGR_CLR,
    #[doc = "0x4a1c - CCM Clock Gating Register"]
    pub ccgr161_tog: CCGR_TOG,
    #[doc = "0x4a20 - CCM Clock Gating Register"]
    pub ccgr162: CCGR,
    #[doc = "0x4a24 - CCM Clock Gating Register"]
    pub ccgr162_set: CCGR_SET,
    #[doc = "0x4a28 - CCM Clock Gating Register"]
    pub ccgr162_clr: CCGR_CLR,
    #[doc = "0x4a2c - CCM Clock Gating Register"]
    pub ccgr162_tog: CCGR_TOG,
    #[doc = "0x4a30 - CCM Clock Gating Register"]
    pub ccgr163: CCGR,
    #[doc = "0x4a34 - CCM Clock Gating Register"]
    pub ccgr163_set: CCGR_SET,
    #[doc = "0x4a38 - CCM Clock Gating Register"]
    pub ccgr163_clr: CCGR_CLR,
    #[doc = "0x4a3c - CCM Clock Gating Register"]
    pub ccgr163_tog: CCGR_TOG,
    #[doc = "0x4a40 - CCM Clock Gating Register"]
    pub ccgr164: CCGR,
    #[doc = "0x4a44 - CCM Clock Gating Register"]
    pub ccgr164_set: CCGR_SET,
    #[doc = "0x4a48 - CCM Clock Gating Register"]
    pub ccgr164_clr: CCGR_CLR,
    #[doc = "0x4a4c - CCM Clock Gating Register"]
    pub ccgr164_tog: CCGR_TOG,
    #[doc = "0x4a50 - CCM Clock Gating Register"]
    pub ccgr165: CCGR,
    #[doc = "0x4a54 - CCM Clock Gating Register"]
    pub ccgr165_set: CCGR_SET,
    #[doc = "0x4a58 - CCM Clock Gating Register"]
    pub ccgr165_clr: CCGR_CLR,
    #[doc = "0x4a5c - CCM Clock Gating Register"]
    pub ccgr165_tog: CCGR_TOG,
    #[doc = "0x4a60 - CCM Clock Gating Register"]
    pub ccgr166: CCGR,
    #[doc = "0x4a64 - CCM Clock Gating Register"]
    pub ccgr166_set: CCGR_SET,
    #[doc = "0x4a68 - CCM Clock Gating Register"]
    pub ccgr166_clr: CCGR_CLR,
    #[doc = "0x4a6c - CCM Clock Gating Register"]
    pub ccgr166_tog: CCGR_TOG,
    #[doc = "0x4a70 - CCM Clock Gating Register"]
    pub ccgr167: CCGR,
    #[doc = "0x4a74 - CCM Clock Gating Register"]
    pub ccgr167_set: CCGR_SET,
    #[doc = "0x4a78 - CCM Clock Gating Register"]
    pub ccgr167_clr: CCGR_CLR,
    #[doc = "0x4a7c - CCM Clock Gating Register"]
    pub ccgr167_tog: CCGR_TOG,
    #[doc = "0x4a80 - CCM Clock Gating Register"]
    pub ccgr168: CCGR,
    #[doc = "0x4a84 - CCM Clock Gating Register"]
    pub ccgr168_set: CCGR_SET,
    #[doc = "0x4a88 - CCM Clock Gating Register"]
    pub ccgr168_clr: CCGR_CLR,
    #[doc = "0x4a8c - CCM Clock Gating Register"]
    pub ccgr168_tog: CCGR_TOG,
    #[doc = "0x4a90 - CCM Clock Gating Register"]
    pub ccgr169: CCGR,
    #[doc = "0x4a94 - CCM Clock Gating Register"]
    pub ccgr169_set: CCGR_SET,
    #[doc = "0x4a98 - CCM Clock Gating Register"]
    pub ccgr169_clr: CCGR_CLR,
    #[doc = "0x4a9c - CCM Clock Gating Register"]
    pub ccgr169_tog: CCGR_TOG,
    #[doc = "0x4aa0 - CCM Clock Gating Register"]
    pub ccgr170: CCGR,
    #[doc = "0x4aa4 - CCM Clock Gating Register"]
    pub ccgr170_set: CCGR_SET,
    #[doc = "0x4aa8 - CCM Clock Gating Register"]
    pub ccgr170_clr: CCGR_CLR,
    #[doc = "0x4aac - CCM Clock Gating Register"]
    pub ccgr170_tog: CCGR_TOG,
    #[doc = "0x4ab0 - CCM Clock Gating Register"]
    pub ccgr171: CCGR,
    #[doc = "0x4ab4 - CCM Clock Gating Register"]
    pub ccgr171_set: CCGR_SET,
    #[doc = "0x4ab8 - CCM Clock Gating Register"]
    pub ccgr171_clr: CCGR_CLR,
    #[doc = "0x4abc - CCM Clock Gating Register"]
    pub ccgr171_tog: CCGR_TOG,
    #[doc = "0x4ac0 - CCM Clock Gating Register"]
    pub ccgr172: CCGR,
    #[doc = "0x4ac4 - CCM Clock Gating Register"]
    pub ccgr172_set: CCGR_SET,
    #[doc = "0x4ac8 - CCM Clock Gating Register"]
    pub ccgr172_clr: CCGR_CLR,
    #[doc = "0x4acc - CCM Clock Gating Register"]
    pub ccgr172_tog: CCGR_TOG,
    #[doc = "0x4ad0 - CCM Clock Gating Register"]
    pub ccgr173: CCGR,
    #[doc = "0x4ad4 - CCM Clock Gating Register"]
    pub ccgr173_set: CCGR_SET,
    #[doc = "0x4ad8 - CCM Clock Gating Register"]
    pub ccgr173_clr: CCGR_CLR,
    #[doc = "0x4adc - CCM Clock Gating Register"]
    pub ccgr173_tog: CCGR_TOG,
    #[doc = "0x4ae0 - CCM Clock Gating Register"]
    pub ccgr174: CCGR,
    #[doc = "0x4ae4 - CCM Clock Gating Register"]
    pub ccgr174_set: CCGR_SET,
    #[doc = "0x4ae8 - CCM Clock Gating Register"]
    pub ccgr174_clr: CCGR_CLR,
    #[doc = "0x4aec - CCM Clock Gating Register"]
    pub ccgr174_tog: CCGR_TOG,
    #[doc = "0x4af0 - CCM Clock Gating Register"]
    pub ccgr175: CCGR,
    #[doc = "0x4af4 - CCM Clock Gating Register"]
    pub ccgr175_set: CCGR_SET,
    #[doc = "0x4af8 - CCM Clock Gating Register"]
    pub ccgr175_clr: CCGR_CLR,
    #[doc = "0x4afc - CCM Clock Gating Register"]
    pub ccgr175_tog: CCGR_TOG,
    #[doc = "0x4b00 - CCM Clock Gating Register"]
    pub ccgr176: CCGR,
    #[doc = "0x4b04 - CCM Clock Gating Register"]
    pub ccgr176_set: CCGR_SET,
    #[doc = "0x4b08 - CCM Clock Gating Register"]
    pub ccgr176_clr: CCGR_CLR,
    #[doc = "0x4b0c - CCM Clock Gating Register"]
    pub ccgr176_tog: CCGR_TOG,
    #[doc = "0x4b10 - CCM Clock Gating Register"]
    pub ccgr177: CCGR,
    #[doc = "0x4b14 - CCM Clock Gating Register"]
    pub ccgr177_set: CCGR_SET,
    #[doc = "0x4b18 - CCM Clock Gating Register"]
    pub ccgr177_clr: CCGR_CLR,
    #[doc = "0x4b1c - CCM Clock Gating Register"]
    pub ccgr177_tog: CCGR_TOG,
    #[doc = "0x4b20 - CCM Clock Gating Register"]
    pub ccgr178: CCGR,
    #[doc = "0x4b24 - CCM Clock Gating Register"]
    pub ccgr178_set: CCGR_SET,
    #[doc = "0x4b28 - CCM Clock Gating Register"]
    pub ccgr178_clr: CCGR_CLR,
    #[doc = "0x4b2c - CCM Clock Gating Register"]
    pub ccgr178_tog: CCGR_TOG,
    #[doc = "0x4b30 - CCM Clock Gating Register"]
    pub ccgr179: CCGR,
    #[doc = "0x4b34 - CCM Clock Gating Register"]
    pub ccgr179_set: CCGR_SET,
    #[doc = "0x4b38 - CCM Clock Gating Register"]
    pub ccgr179_clr: CCGR_CLR,
    #[doc = "0x4b3c - CCM Clock Gating Register"]
    pub ccgr179_tog: CCGR_TOG,
    #[doc = "0x4b40 - CCM Clock Gating Register"]
    pub ccgr180: CCGR,
    #[doc = "0x4b44 - CCM Clock Gating Register"]
    pub ccgr180_set: CCGR_SET,
    #[doc = "0x4b48 - CCM Clock Gating Register"]
    pub ccgr180_clr: CCGR_CLR,
    #[doc = "0x4b4c - CCM Clock Gating Register"]
    pub ccgr180_tog: CCGR_TOG,
    #[doc = "0x4b50 - CCM Clock Gating Register"]
    pub ccgr181: CCGR,
    #[doc = "0x4b54 - CCM Clock Gating Register"]
    pub ccgr181_set: CCGR_SET,
    #[doc = "0x4b58 - CCM Clock Gating Register"]
    pub ccgr181_clr: CCGR_CLR,
    #[doc = "0x4b5c - CCM Clock Gating Register"]
    pub ccgr181_tog: CCGR_TOG,
    #[doc = "0x4b60 - CCM Clock Gating Register"]
    pub ccgr182: CCGR,
    #[doc = "0x4b64 - CCM Clock Gating Register"]
    pub ccgr182_set: CCGR_SET,
    #[doc = "0x4b68 - CCM Clock Gating Register"]
    pub ccgr182_clr: CCGR_CLR,
    #[doc = "0x4b6c - CCM Clock Gating Register"]
    pub ccgr182_tog: CCGR_TOG,
    #[doc = "0x4b70 - CCM Clock Gating Register"]
    pub ccgr183: CCGR,
    #[doc = "0x4b74 - CCM Clock Gating Register"]
    pub ccgr183_set: CCGR_SET,
    #[doc = "0x4b78 - CCM Clock Gating Register"]
    pub ccgr183_clr: CCGR_CLR,
    #[doc = "0x4b7c - CCM Clock Gating Register"]
    pub ccgr183_tog: CCGR_TOG,
    #[doc = "0x4b80 - CCM Clock Gating Register"]
    pub ccgr184: CCGR,
    #[doc = "0x4b84 - CCM Clock Gating Register"]
    pub ccgr184_set: CCGR_SET,
    #[doc = "0x4b88 - CCM Clock Gating Register"]
    pub ccgr184_clr: CCGR_CLR,
    #[doc = "0x4b8c - CCM Clock Gating Register"]
    pub ccgr184_tog: CCGR_TOG,
    #[doc = "0x4b90 - CCM Clock Gating Register"]
    pub ccgr185: CCGR,
    #[doc = "0x4b94 - CCM Clock Gating Register"]
    pub ccgr185_set: CCGR_SET,
    #[doc = "0x4b98 - CCM Clock Gating Register"]
    pub ccgr185_clr: CCGR_CLR,
    #[doc = "0x4b9c - CCM Clock Gating Register"]
    pub ccgr185_tog: CCGR_TOG,
    #[doc = "0x4ba0 - CCM Clock Gating Register"]
    pub ccgr186: CCGR,
    #[doc = "0x4ba4 - CCM Clock Gating Register"]
    pub ccgr186_set: CCGR_SET,
    #[doc = "0x4ba8 - CCM Clock Gating Register"]
    pub ccgr186_clr: CCGR_CLR,
    #[doc = "0x4bac - CCM Clock Gating Register"]
    pub ccgr186_tog: CCGR_TOG,
    #[doc = "0x4bb0 - CCM Clock Gating Register"]
    pub ccgr187: CCGR,
    #[doc = "0x4bb4 - CCM Clock Gating Register"]
    pub ccgr187_set: CCGR_SET,
    #[doc = "0x4bb8 - CCM Clock Gating Register"]
    pub ccgr187_clr: CCGR_CLR,
    #[doc = "0x4bbc - CCM Clock Gating Register"]
    pub ccgr187_tog: CCGR_TOG,
    #[doc = "0x4bc0 - CCM Clock Gating Register"]
    pub ccgr188: CCGR,
    #[doc = "0x4bc4 - CCM Clock Gating Register"]
    pub ccgr188_set: CCGR_SET,
    #[doc = "0x4bc8 - CCM Clock Gating Register"]
    pub ccgr188_clr: CCGR_CLR,
    #[doc = "0x4bcc - CCM Clock Gating Register"]
    pub ccgr188_tog: CCGR_TOG,
    #[doc = "0x4bd0 - CCM Clock Gating Register"]
    pub ccgr189: CCGR,
    #[doc = "0x4bd4 - CCM Clock Gating Register"]
    pub ccgr189_set: CCGR_SET,
    #[doc = "0x4bd8 - CCM Clock Gating Register"]
    pub ccgr189_clr: CCGR_CLR,
    #[doc = "0x4bdc - CCM Clock Gating Register"]
    pub ccgr189_tog: CCGR_TOG,
    #[doc = "0x4be0 - CCM Clock Gating Register"]
    pub ccgr190: CCGR,
    #[doc = "0x4be4 - CCM Clock Gating Register"]
    pub ccgr190_set: CCGR_SET,
    #[doc = "0x4be8 - CCM Clock Gating Register"]
    pub ccgr190_clr: CCGR_CLR,
    #[doc = "0x4bec - CCM Clock Gating Register"]
    pub ccgr190_tog: CCGR_TOG,
    _reserved2: [u8; 13328usize],
    #[doc = "0x8000 - Target Register"]
    pub target_root0: TARGET_ROOT0,
    #[doc = "0x8004 - Target Register"]
    pub target_root0_set: TARGET_ROOT0_SET,
    #[doc = "0x8008 - Target Register"]
    pub target_root0_clr: TARGET_ROOT0_CLR,
    #[doc = "0x800c - Target Register"]
    pub target_root0_tog: TARGET_ROOT0_TOG,
    #[doc = "0x8010 - Miscellaneous Register"]
    pub misc0: MISC0,
    #[doc = "0x8014 - Miscellaneous Register"]
    pub misc_root0_set: MISC_ROOT0_SET,
    #[doc = "0x8018 - Miscellaneous Register"]
    pub misc_root0_clr: MISC_ROOT0_CLR,
    #[doc = "0x801c - Miscellaneous Register"]
    pub misc_root0_tog: MISC_ROOT0_TOG,
    #[doc = "0x8020 - Post Divider Register"]
    pub post0: POST0,
    #[doc = "0x8024 - Post Divider Register"]
    pub post_root0_set: POST_ROOT0_SET,
    #[doc = "0x8028 - Post Divider Register"]
    pub post_root0_clr: POST_ROOT0_CLR,
    #[doc = "0x802c - Post Divider Register"]
    pub post_root0_tog: POST_ROOT0_TOG,
    #[doc = "0x8030 - Pre Divider Register"]
    pub pre0: PRE0,
    #[doc = "0x8034 - Pre Divider Register"]
    pub pre_root0_set: PRE_ROOT0_SET,
    #[doc = "0x8038 - Pre Divider Register"]
    pub pre_root0_clr: PRE_ROOT0_CLR,
    #[doc = "0x803c - Pre Divider Register"]
    pub pre_root0_tog: PRE_ROOT0_TOG,
    _reserved3: [u8; 48usize],
    #[doc = "0x8070 - Access Control Register"]
    pub access_ctrl0: ACCESS_CTRL0,
    #[doc = "0x8074 - Access Control Register"]
    pub access_ctrl_root0_set: ACCESS_CTRL_ROOT0_SET,
    #[doc = "0x8078 - Access Control Register"]
    pub access_ctrl_root0_clr: ACCESS_CTRL_ROOT0_CLR,
    #[doc = "0x807c - Access Control Register"]
    pub access_ctrl_root0_tog: ACCESS_CTRL_ROOT0_TOG,
    #[doc = "0x8080 - Target Register"]
    pub target_root1: TARGET_ROOT1,
    #[doc = "0x8084 - Target Register"]
    pub target_root1_set: TARGET_ROOT1_SET,
    #[doc = "0x8088 - Target Register"]
    pub target_root1_clr: TARGET_ROOT1_CLR,
    #[doc = "0x808c - Target Register"]
    pub target_root1_tog: TARGET_ROOT1_TOG,
    #[doc = "0x8090 - Miscellaneous Register"]
    pub misc1: MISC1,
    #[doc = "0x8094 - Miscellaneous Register"]
    pub misc_root1_set: MISC_ROOT1_SET,
    #[doc = "0x8098 - Miscellaneous Register"]
    pub misc_root1_clr: MISC_ROOT1_CLR,
    #[doc = "0x809c - Miscellaneous Register"]
    pub misc_root1_tog: MISC_ROOT1_TOG,
    #[doc = "0x80a0 - Post Divider Register"]
    pub post1: POST1,
    #[doc = "0x80a4 - Post Divider Register"]
    pub post_root1_set: POST_ROOT1_SET,
    #[doc = "0x80a8 - Post Divider Register"]
    pub post_root1_clr: POST_ROOT1_CLR,
    #[doc = "0x80ac - Post Divider Register"]
    pub post_root1_tog: POST_ROOT1_TOG,
    #[doc = "0x80b0 - Pre Divider Register"]
    pub pre1: PRE1,
    #[doc = "0x80b4 - Pre Divider Register"]
    pub pre_root1_set: PRE_ROOT1_SET,
    #[doc = "0x80b8 - Pre Divider Register"]
    pub pre_root1_clr: PRE_ROOT1_CLR,
    #[doc = "0x80bc - Pre Divider Register"]
    pub pre_root1_tog: PRE_ROOT1_TOG,
    _reserved4: [u8; 48usize],
    #[doc = "0x80f0 - Access Control Register"]
    pub access_ctrl1: ACCESS_CTRL1,
    #[doc = "0x80f4 - Access Control Register"]
    pub access_ctrl_root1_set: ACCESS_CTRL_ROOT1_SET,
    #[doc = "0x80f8 - Access Control Register"]
    pub access_ctrl_root1_clr: ACCESS_CTRL_ROOT1_CLR,
    #[doc = "0x80fc - Access Control Register"]
    pub access_ctrl_root1_tog: ACCESS_CTRL_ROOT1_TOG,
    #[doc = "0x8100 - Target Register"]
    pub target_root2: TARGET_ROOT2,
    #[doc = "0x8104 - Target Register"]
    pub target_root2_set: TARGET_ROOT2_SET,
    #[doc = "0x8108 - Target Register"]
    pub target_root2_clr: TARGET_ROOT2_CLR,
    #[doc = "0x810c - Target Register"]
    pub target_root2_tog: TARGET_ROOT2_TOG,
    #[doc = "0x8110 - Miscellaneous Register"]
    pub misc2: MISC2,
    #[doc = "0x8114 - Miscellaneous Register"]
    pub misc_root2_set: MISC_ROOT2_SET,
    #[doc = "0x8118 - Miscellaneous Register"]
    pub misc_root2_clr: MISC_ROOT2_CLR,
    #[doc = "0x811c - Miscellaneous Register"]
    pub misc_root2_tog: MISC_ROOT2_TOG,
    #[doc = "0x8120 - Post Divider Register"]
    pub post2: POST2,
    #[doc = "0x8124 - Post Divider Register"]
    pub post_root2_set: POST_ROOT2_SET,
    #[doc = "0x8128 - Post Divider Register"]
    pub post_root2_clr: POST_ROOT2_CLR,
    #[doc = "0x812c - Post Divider Register"]
    pub post_root2_tog: POST_ROOT2_TOG,
    #[doc = "0x8130 - Pre Divider Register"]
    pub pre2: PRE2,
    #[doc = "0x8134 - Pre Divider Register"]
    pub pre_root2_set: PRE_ROOT2_SET,
    #[doc = "0x8138 - Pre Divider Register"]
    pub pre_root2_clr: PRE_ROOT2_CLR,
    #[doc = "0x813c - Pre Divider Register"]
    pub pre_root2_tog: PRE_ROOT2_TOG,
    _reserved5: [u8; 48usize],
    #[doc = "0x8170 - Access Control Register"]
    pub access_ctrl2: ACCESS_CTRL2,
    #[doc = "0x8174 - Access Control Register"]
    pub access_ctrl_root2_set: ACCESS_CTRL_ROOT2_SET,
    #[doc = "0x8178 - Access Control Register"]
    pub access_ctrl_root2_clr: ACCESS_CTRL_ROOT2_CLR,
    #[doc = "0x817c - Access Control Register"]
    pub access_ctrl_root2_tog: ACCESS_CTRL_ROOT2_TOG,
    #[doc = "0x8180 - Target Register"]
    pub target_root3: TARGET_ROOT3,
    #[doc = "0x8184 - Target Register"]
    pub target_root3_set: TARGET_ROOT3_SET,
    #[doc = "0x8188 - Target Register"]
    pub target_root3_clr: TARGET_ROOT3_CLR,
    #[doc = "0x818c - Target Register"]
    pub target_root3_tog: TARGET_ROOT3_TOG,
    #[doc = "0x8190 - Miscellaneous Register"]
    pub misc3: MISC3,
    #[doc = "0x8194 - Miscellaneous Register"]
    pub misc_root3_set: MISC_ROOT3_SET,
    #[doc = "0x8198 - Miscellaneous Register"]
    pub misc_root3_clr: MISC_ROOT3_CLR,
    #[doc = "0x819c - Miscellaneous Register"]
    pub misc_root3_tog: MISC_ROOT3_TOG,
    #[doc = "0x81a0 - Post Divider Register"]
    pub post3: POST3,
    #[doc = "0x81a4 - Post Divider Register"]
    pub post_root3_set: POST_ROOT3_SET,
    #[doc = "0x81a8 - Post Divider Register"]
    pub post_root3_clr: POST_ROOT3_CLR,
    #[doc = "0x81ac - Post Divider Register"]
    pub post_root3_tog: POST_ROOT3_TOG,
    #[doc = "0x81b0 - Pre Divider Register"]
    pub pre3: PRE3,
    #[doc = "0x81b4 - Pre Divider Register"]
    pub pre_root3_set: PRE_ROOT3_SET,
    #[doc = "0x81b8 - Pre Divider Register"]
    pub pre_root3_clr: PRE_ROOT3_CLR,
    #[doc = "0x81bc - Pre Divider Register"]
    pub pre_root3_tog: PRE_ROOT3_TOG,
    _reserved6: [u8; 48usize],
    #[doc = "0x81f0 - Access Control Register"]
    pub access_ctrl3: ACCESS_CTRL3,
    #[doc = "0x81f4 - Access Control Register"]
    pub access_ctrl_root3_set: ACCESS_CTRL_ROOT3_SET,
    #[doc = "0x81f8 - Access Control Register"]
    pub access_ctrl_root3_clr: ACCESS_CTRL_ROOT3_CLR,
    #[doc = "0x81fc - Access Control Register"]
    pub access_ctrl_root3_tog: ACCESS_CTRL_ROOT3_TOG,
    #[doc = "0x8200 - Target Register"]
    pub target_root4: TARGET_ROOT4,
    #[doc = "0x8204 - Target Register"]
    pub target_root4_set: TARGET_ROOT4_SET,
    #[doc = "0x8208 - Target Register"]
    pub target_root4_clr: TARGET_ROOT4_CLR,
    #[doc = "0x820c - Target Register"]
    pub target_root4_tog: TARGET_ROOT4_TOG,
    #[doc = "0x8210 - Miscellaneous Register"]
    pub misc4: MISC4,
    #[doc = "0x8214 - Miscellaneous Register"]
    pub misc_root4_set: MISC_ROOT4_SET,
    #[doc = "0x8218 - Miscellaneous Register"]
    pub misc_root4_clr: MISC_ROOT4_CLR,
    #[doc = "0x821c - Miscellaneous Register"]
    pub misc_root4_tog: MISC_ROOT4_TOG,
    #[doc = "0x8220 - Post Divider Register"]
    pub post4: POST4,
    #[doc = "0x8224 - Post Divider Register"]
    pub post_root4_set: POST_ROOT4_SET,
    #[doc = "0x8228 - Post Divider Register"]
    pub post_root4_clr: POST_ROOT4_CLR,
    #[doc = "0x822c - Post Divider Register"]
    pub post_root4_tog: POST_ROOT4_TOG,
    #[doc = "0x8230 - Pre Divider Register"]
    pub pre4: PRE4,
    #[doc = "0x8234 - Pre Divider Register"]
    pub pre_root4_set: PRE_ROOT4_SET,
    #[doc = "0x8238 - Pre Divider Register"]
    pub pre_root4_clr: PRE_ROOT4_CLR,
    #[doc = "0x823c - Pre Divider Register"]
    pub pre_root4_tog: PRE_ROOT4_TOG,
    _reserved7: [u8; 48usize],
    #[doc = "0x8270 - Access Control Register"]
    pub access_ctrl4: ACCESS_CTRL4,
    #[doc = "0x8274 - Access Control Register"]
    pub access_ctrl_root4_set: ACCESS_CTRL_ROOT4_SET,
    #[doc = "0x8278 - Access Control Register"]
    pub access_ctrl_root4_clr: ACCESS_CTRL_ROOT4_CLR,
    #[doc = "0x827c - Access Control Register"]
    pub access_ctrl_root4_tog: ACCESS_CTRL_ROOT4_TOG,
    _reserved8: [u8; 1408usize],
    #[doc = "0x8800 - Target Register"]
    pub target_root16: TARGET_ROOT16,
    #[doc = "0x8804 - Target Register"]
    pub target_root16_set: TARGET_ROOT16_SET,
    #[doc = "0x8808 - Target Register"]
    pub target_root16_clr: TARGET_ROOT16_CLR,
    #[doc = "0x880c - Target Register"]
    pub target_root16_tog: TARGET_ROOT16_TOG,
    #[doc = "0x8810 - Miscellaneous Register"]
    pub misc16: MISC16,
    #[doc = "0x8814 - Miscellaneous Register"]
    pub misc_root16_set: MISC_ROOT16_SET,
    #[doc = "0x8818 - Miscellaneous Register"]
    pub misc_root16_clr: MISC_ROOT16_CLR,
    #[doc = "0x881c - Miscellaneous Register"]
    pub misc_root16_tog: MISC_ROOT16_TOG,
    #[doc = "0x8820 - Post Divider Register"]
    pub post16: POST16,
    #[doc = "0x8824 - Post Divider Register"]
    pub post_root16_set: POST_ROOT16_SET,
    #[doc = "0x8828 - Post Divider Register"]
    pub post_root16_clr: POST_ROOT16_CLR,
    #[doc = "0x882c - Post Divider Register"]
    pub post_root16_tog: POST_ROOT16_TOG,
    #[doc = "0x8830 - Pre Divider Register"]
    pub pre16: PRE16,
    #[doc = "0x8834 - Pre Divider Register"]
    pub pre_root16_set: PRE_ROOT16_SET,
    #[doc = "0x8838 - Pre Divider Register"]
    pub pre_root16_clr: PRE_ROOT16_CLR,
    #[doc = "0x883c - Pre Divider Register"]
    pub pre_root16_tog: PRE_ROOT16_TOG,
    _reserved9: [u8; 48usize],
    #[doc = "0x8870 - Access Control Register"]
    pub access_ctrl16: ACCESS_CTRL16,
    #[doc = "0x8874 - Access Control Register"]
    pub access_ctrl_root16_set: ACCESS_CTRL_ROOT16_SET,
    #[doc = "0x8878 - Access Control Register"]
    pub access_ctrl_root16_clr: ACCESS_CTRL_ROOT16_CLR,
    #[doc = "0x887c - Access Control Register"]
    pub access_ctrl_root16_tog: ACCESS_CTRL_ROOT16_TOG,
    #[doc = "0x8880 - Target Register"]
    pub target_root17: TARGET_ROOT17,
    #[doc = "0x8884 - Target Register"]
    pub target_root17_set: TARGET_ROOT17_SET,
    #[doc = "0x8888 - Target Register"]
    pub target_root17_clr: TARGET_ROOT17_CLR,
    #[doc = "0x888c - Target Register"]
    pub target_root17_tog: TARGET_ROOT17_TOG,
    #[doc = "0x8890 - Miscellaneous Register"]
    pub misc17: MISC17,
    #[doc = "0x8894 - Miscellaneous Register"]
    pub misc_root17_set: MISC_ROOT17_SET,
    #[doc = "0x8898 - Miscellaneous Register"]
    pub misc_root17_clr: MISC_ROOT17_CLR,
    #[doc = "0x889c - Miscellaneous Register"]
    pub misc_root17_tog: MISC_ROOT17_TOG,
    #[doc = "0x88a0 - Post Divider Register"]
    pub post17: POST17,
    #[doc = "0x88a4 - Post Divider Register"]
    pub post_root17_set: POST_ROOT17_SET,
    #[doc = "0x88a8 - Post Divider Register"]
    pub post_root17_clr: POST_ROOT17_CLR,
    #[doc = "0x88ac - Post Divider Register"]
    pub post_root17_tog: POST_ROOT17_TOG,
    #[doc = "0x88b0 - Pre Divider Register"]
    pub pre17: PRE17,
    #[doc = "0x88b4 - Pre Divider Register"]
    pub pre_root17_set: PRE_ROOT17_SET,
    #[doc = "0x88b8 - Pre Divider Register"]
    pub pre_root17_clr: PRE_ROOT17_CLR,
    #[doc = "0x88bc - Pre Divider Register"]
    pub pre_root17_tog: PRE_ROOT17_TOG,
    _reserved10: [u8; 48usize],
    #[doc = "0x88f0 - Access Control Register"]
    pub access_ctrl17: ACCESS_CTRL17,
    #[doc = "0x88f4 - Access Control Register"]
    pub access_ctrl_root17_set: ACCESS_CTRL_ROOT17_SET,
    #[doc = "0x88f8 - Access Control Register"]
    pub access_ctrl_root17_clr: ACCESS_CTRL_ROOT17_CLR,
    #[doc = "0x88fc - Access Control Register"]
    pub access_ctrl_root17_tog: ACCESS_CTRL_ROOT17_TOG,
    #[doc = "0x8900 - Target Register"]
    pub target_root18: TARGET_ROOT18,
    #[doc = "0x8904 - Target Register"]
    pub target_root18_set: TARGET_ROOT18_SET,
    #[doc = "0x8908 - Target Register"]
    pub target_root18_clr: TARGET_ROOT18_CLR,
    #[doc = "0x890c - Target Register"]
    pub target_root18_tog: TARGET_ROOT18_TOG,
    #[doc = "0x8910 - Miscellaneous Register"]
    pub misc18: MISC18,
    #[doc = "0x8914 - Miscellaneous Register"]
    pub misc_root18_set: MISC_ROOT18_SET,
    #[doc = "0x8918 - Miscellaneous Register"]
    pub misc_root18_clr: MISC_ROOT18_CLR,
    #[doc = "0x891c - Miscellaneous Register"]
    pub misc_root18_tog: MISC_ROOT18_TOG,
    #[doc = "0x8920 - Post Divider Register"]
    pub post18: POST18,
    #[doc = "0x8924 - Post Divider Register"]
    pub post_root18_set: POST_ROOT18_SET,
    #[doc = "0x8928 - Post Divider Register"]
    pub post_root18_clr: POST_ROOT18_CLR,
    #[doc = "0x892c - Post Divider Register"]
    pub post_root18_tog: POST_ROOT18_TOG,
    #[doc = "0x8930 - Pre Divider Register"]
    pub pre18: PRE18,
    #[doc = "0x8934 - Pre Divider Register"]
    pub pre_root18_set: PRE_ROOT18_SET,
    #[doc = "0x8938 - Pre Divider Register"]
    pub pre_root18_clr: PRE_ROOT18_CLR,
    #[doc = "0x893c - Pre Divider Register"]
    pub pre_root18_tog: PRE_ROOT18_TOG,
    _reserved11: [u8; 48usize],
    #[doc = "0x8970 - Access Control Register"]
    pub access_ctrl18: ACCESS_CTRL18,
    #[doc = "0x8974 - Access Control Register"]
    pub access_ctrl_root18_set: ACCESS_CTRL_ROOT18_SET,
    #[doc = "0x8978 - Access Control Register"]
    pub access_ctrl_root18_clr: ACCESS_CTRL_ROOT18_CLR,
    #[doc = "0x897c - Access Control Register"]
    pub access_ctrl_root18_tog: ACCESS_CTRL_ROOT18_TOG,
    #[doc = "0x8980 - Target Register"]
    pub target_root19: TARGET_ROOT19,
    #[doc = "0x8984 - Target Register"]
    pub target_root19_set: TARGET_ROOT19_SET,
    #[doc = "0x8988 - Target Register"]
    pub target_root19_clr: TARGET_ROOT19_CLR,
    #[doc = "0x898c - Target Register"]
    pub target_root19_tog: TARGET_ROOT19_TOG,
    #[doc = "0x8990 - Miscellaneous Register"]
    pub misc19: MISC19,
    #[doc = "0x8994 - Miscellaneous Register"]
    pub misc_root19_set: MISC_ROOT19_SET,
    #[doc = "0x8998 - Miscellaneous Register"]
    pub misc_root19_clr: MISC_ROOT19_CLR,
    #[doc = "0x899c - Miscellaneous Register"]
    pub misc_root19_tog: MISC_ROOT19_TOG,
    #[doc = "0x89a0 - Post Divider Register"]
    pub post19: POST19,
    #[doc = "0x89a4 - Post Divider Register"]
    pub post_root19_set: POST_ROOT19_SET,
    #[doc = "0x89a8 - Post Divider Register"]
    pub post_root19_clr: POST_ROOT19_CLR,
    #[doc = "0x89ac - Post Divider Register"]
    pub post_root19_tog: POST_ROOT19_TOG,
    #[doc = "0x89b0 - Pre Divider Register"]
    pub pre19: PRE19,
    #[doc = "0x89b4 - Pre Divider Register"]
    pub pre_root19_set: PRE_ROOT19_SET,
    #[doc = "0x89b8 - Pre Divider Register"]
    pub pre_root19_clr: PRE_ROOT19_CLR,
    #[doc = "0x89bc - Pre Divider Register"]
    pub pre_root19_tog: PRE_ROOT19_TOG,
    _reserved12: [u8; 48usize],
    #[doc = "0x89f0 - Access Control Register"]
    pub access_ctrl19: ACCESS_CTRL19,
    #[doc = "0x89f4 - Access Control Register"]
    pub access_ctrl_root19_set: ACCESS_CTRL_ROOT19_SET,
    #[doc = "0x89f8 - Access Control Register"]
    pub access_ctrl_root19_clr: ACCESS_CTRL_ROOT19_CLR,
    #[doc = "0x89fc - Access Control Register"]
    pub access_ctrl_root19_tog: ACCESS_CTRL_ROOT19_TOG,
    #[doc = "0x8a00 - Target Register"]
    pub target_root20: TARGET_ROOT20,
    #[doc = "0x8a04 - Target Register"]
    pub target_root20_set: TARGET_ROOT20_SET,
    #[doc = "0x8a08 - Target Register"]
    pub target_root20_clr: TARGET_ROOT20_CLR,
    #[doc = "0x8a0c - Target Register"]
    pub target_root20_tog: TARGET_ROOT20_TOG,
    #[doc = "0x8a10 - Miscellaneous Register"]
    pub misc20: MISC20,
    #[doc = "0x8a14 - Miscellaneous Register"]
    pub misc_root20_set: MISC_ROOT20_SET,
    #[doc = "0x8a18 - Miscellaneous Register"]
    pub misc_root20_clr: MISC_ROOT20_CLR,
    #[doc = "0x8a1c - Miscellaneous Register"]
    pub misc_root20_tog: MISC_ROOT20_TOG,
    #[doc = "0x8a20 - Post Divider Register"]
    pub post20: POST20,
    #[doc = "0x8a24 - Post Divider Register"]
    pub post_root20_set: POST_ROOT20_SET,
    #[doc = "0x8a28 - Post Divider Register"]
    pub post_root20_clr: POST_ROOT20_CLR,
    #[doc = "0x8a2c - Post Divider Register"]
    pub post_root20_tog: POST_ROOT20_TOG,
    #[doc = "0x8a30 - Pre Divider Register"]
    pub pre20: PRE20,
    #[doc = "0x8a34 - Pre Divider Register"]
    pub pre_root20_set: PRE_ROOT20_SET,
    #[doc = "0x8a38 - Pre Divider Register"]
    pub pre_root20_clr: PRE_ROOT20_CLR,
    #[doc = "0x8a3c - Pre Divider Register"]
    pub pre_root20_tog: PRE_ROOT20_TOG,
    _reserved13: [u8; 48usize],
    #[doc = "0x8a70 - Access Control Register"]
    pub access_ctrl20: ACCESS_CTRL20,
    #[doc = "0x8a74 - Access Control Register"]
    pub access_ctrl_root20_set: ACCESS_CTRL_ROOT20_SET,
    #[doc = "0x8a78 - Access Control Register"]
    pub access_ctrl_root20_clr: ACCESS_CTRL_ROOT20_CLR,
    #[doc = "0x8a7c - Access Control Register"]
    pub access_ctrl_root20_tog: ACCESS_CTRL_ROOT20_TOG,
    #[doc = "0x8a80 - Target Register"]
    pub target_root21: TARGET_ROOT21,
    #[doc = "0x8a84 - Target Register"]
    pub target_root21_set: TARGET_ROOT21_SET,
    #[doc = "0x8a88 - Target Register"]
    pub target_root21_clr: TARGET_ROOT21_CLR,
    #[doc = "0x8a8c - Target Register"]
    pub target_root21_tog: TARGET_ROOT21_TOG,
    #[doc = "0x8a90 - Miscellaneous Register"]
    pub misc21: MISC21,
    #[doc = "0x8a94 - Miscellaneous Register"]
    pub misc_root21_set: MISC_ROOT21_SET,
    #[doc = "0x8a98 - Miscellaneous Register"]
    pub misc_root21_clr: MISC_ROOT21_CLR,
    #[doc = "0x8a9c - Miscellaneous Register"]
    pub misc_root21_tog: MISC_ROOT21_TOG,
    #[doc = "0x8aa0 - Post Divider Register"]
    pub post21: POST21,
    #[doc = "0x8aa4 - Post Divider Register"]
    pub post_root21_set: POST_ROOT21_SET,
    #[doc = "0x8aa8 - Post Divider Register"]
    pub post_root21_clr: POST_ROOT21_CLR,
    #[doc = "0x8aac - Post Divider Register"]
    pub post_root21_tog: POST_ROOT21_TOG,
    #[doc = "0x8ab0 - Pre Divider Register"]
    pub pre21: PRE21,
    #[doc = "0x8ab4 - Pre Divider Register"]
    pub pre_root21_set: PRE_ROOT21_SET,
    #[doc = "0x8ab8 - Pre Divider Register"]
    pub pre_root21_clr: PRE_ROOT21_CLR,
    #[doc = "0x8abc - Pre Divider Register"]
    pub pre_root21_tog: PRE_ROOT21_TOG,
    _reserved14: [u8; 48usize],
    #[doc = "0x8af0 - Access Control Register"]
    pub access_ctrl21: ACCESS_CTRL21,
    #[doc = "0x8af4 - Access Control Register"]
    pub access_ctrl_root21_set: ACCESS_CTRL_ROOT21_SET,
    #[doc = "0x8af8 - Access Control Register"]
    pub access_ctrl_root21_clr: ACCESS_CTRL_ROOT21_CLR,
    #[doc = "0x8afc - Access Control Register"]
    pub access_ctrl_root21_tog: ACCESS_CTRL_ROOT21_TOG,
    #[doc = "0x8b00 - Target Register"]
    pub target_root22: TARGET_ROOT22,
    #[doc = "0x8b04 - Target Register"]
    pub target_root22_set: TARGET_ROOT22_SET,
    #[doc = "0x8b08 - Target Register"]
    pub target_root22_clr: TARGET_ROOT22_CLR,
    #[doc = "0x8b0c - Target Register"]
    pub target_root22_tog: TARGET_ROOT22_TOG,
    #[doc = "0x8b10 - Miscellaneous Register"]
    pub misc22: MISC22,
    #[doc = "0x8b14 - Miscellaneous Register"]
    pub misc_root22_set: MISC_ROOT22_SET,
    #[doc = "0x8b18 - Miscellaneous Register"]
    pub misc_root22_clr: MISC_ROOT22_CLR,
    #[doc = "0x8b1c - Miscellaneous Register"]
    pub misc_root22_tog: MISC_ROOT22_TOG,
    #[doc = "0x8b20 - Post Divider Register"]
    pub post22: POST22,
    #[doc = "0x8b24 - Post Divider Register"]
    pub post_root22_set: POST_ROOT22_SET,
    #[doc = "0x8b28 - Post Divider Register"]
    pub post_root22_clr: POST_ROOT22_CLR,
    #[doc = "0x8b2c - Post Divider Register"]
    pub post_root22_tog: POST_ROOT22_TOG,
    #[doc = "0x8b30 - Pre Divider Register"]
    pub pre22: PRE22,
    #[doc = "0x8b34 - Pre Divider Register"]
    pub pre_root22_set: PRE_ROOT22_SET,
    #[doc = "0x8b38 - Pre Divider Register"]
    pub pre_root22_clr: PRE_ROOT22_CLR,
    #[doc = "0x8b3c - Pre Divider Register"]
    pub pre_root22_tog: PRE_ROOT22_TOG,
    _reserved15: [u8; 48usize],
    #[doc = "0x8b70 - Access Control Register"]
    pub access_ctrl22: ACCESS_CTRL22,
    #[doc = "0x8b74 - Access Control Register"]
    pub access_ctrl_root22_set: ACCESS_CTRL_ROOT22_SET,
    #[doc = "0x8b78 - Access Control Register"]
    pub access_ctrl_root22_clr: ACCESS_CTRL_ROOT22_CLR,
    #[doc = "0x8b7c - Access Control Register"]
    pub access_ctrl_root22_tog: ACCESS_CTRL_ROOT22_TOG,
    #[doc = "0x8b80 - Target Register"]
    pub target_root23: TARGET_ROOT23,
    #[doc = "0x8b84 - Target Register"]
    pub target_root23_set: TARGET_ROOT23_SET,
    #[doc = "0x8b88 - Target Register"]
    pub target_root23_clr: TARGET_ROOT23_CLR,
    #[doc = "0x8b8c - Target Register"]
    pub target_root23_tog: TARGET_ROOT23_TOG,
    #[doc = "0x8b90 - Miscellaneous Register"]
    pub misc23: MISC23,
    #[doc = "0x8b94 - Miscellaneous Register"]
    pub misc_root23_set: MISC_ROOT23_SET,
    #[doc = "0x8b98 - Miscellaneous Register"]
    pub misc_root23_clr: MISC_ROOT23_CLR,
    #[doc = "0x8b9c - Miscellaneous Register"]
    pub misc_root23_tog: MISC_ROOT23_TOG,
    #[doc = "0x8ba0 - Post Divider Register"]
    pub post23: POST23,
    #[doc = "0x8ba4 - Post Divider Register"]
    pub post_root23_set: POST_ROOT23_SET,
    #[doc = "0x8ba8 - Post Divider Register"]
    pub post_root23_clr: POST_ROOT23_CLR,
    #[doc = "0x8bac - Post Divider Register"]
    pub post_root23_tog: POST_ROOT23_TOG,
    #[doc = "0x8bb0 - Pre Divider Register"]
    pub pre23: PRE23,
    #[doc = "0x8bb4 - Pre Divider Register"]
    pub pre_root23_set: PRE_ROOT23_SET,
    #[doc = "0x8bb8 - Pre Divider Register"]
    pub pre_root23_clr: PRE_ROOT23_CLR,
    #[doc = "0x8bbc - Pre Divider Register"]
    pub pre_root23_tog: PRE_ROOT23_TOG,
    _reserved16: [u8; 48usize],
    #[doc = "0x8bf0 - Access Control Register"]
    pub access_ctrl23: ACCESS_CTRL23,
    #[doc = "0x8bf4 - Access Control Register"]
    pub access_ctrl_root23_set: ACCESS_CTRL_ROOT23_SET,
    #[doc = "0x8bf8 - Access Control Register"]
    pub access_ctrl_root23_clr: ACCESS_CTRL_ROOT23_CLR,
    #[doc = "0x8bfc - Access Control Register"]
    pub access_ctrl_root23_tog: ACCESS_CTRL_ROOT23_TOG,
    #[doc = "0x8c00 - Target Register"]
    pub target_root24: TARGET_ROOT24,
    #[doc = "0x8c04 - Target Register"]
    pub target_root24_set: TARGET_ROOT24_SET,
    #[doc = "0x8c08 - Target Register"]
    pub target_root24_clr: TARGET_ROOT24_CLR,
    #[doc = "0x8c0c - Target Register"]
    pub target_root24_tog: TARGET_ROOT24_TOG,
    #[doc = "0x8c10 - Miscellaneous Register"]
    pub misc24: MISC24,
    #[doc = "0x8c14 - Miscellaneous Register"]
    pub misc_root24_set: MISC_ROOT24_SET,
    #[doc = "0x8c18 - Miscellaneous Register"]
    pub misc_root24_clr: MISC_ROOT24_CLR,
    #[doc = "0x8c1c - Miscellaneous Register"]
    pub misc_root24_tog: MISC_ROOT24_TOG,
    #[doc = "0x8c20 - Post Divider Register"]
    pub post24: POST24,
    #[doc = "0x8c24 - Post Divider Register"]
    pub post_root24_set: POST_ROOT24_SET,
    #[doc = "0x8c28 - Post Divider Register"]
    pub post_root24_clr: POST_ROOT24_CLR,
    #[doc = "0x8c2c - Post Divider Register"]
    pub post_root24_tog: POST_ROOT24_TOG,
    #[doc = "0x8c30 - Pre Divider Register"]
    pub pre24: PRE24,
    #[doc = "0x8c34 - Pre Divider Register"]
    pub pre_root24_set: PRE_ROOT24_SET,
    #[doc = "0x8c38 - Pre Divider Register"]
    pub pre_root24_clr: PRE_ROOT24_CLR,
    #[doc = "0x8c3c - Pre Divider Register"]
    pub pre_root24_tog: PRE_ROOT24_TOG,
    _reserved17: [u8; 48usize],
    #[doc = "0x8c70 - Access Control Register"]
    pub access_ctrl24: ACCESS_CTRL24,
    #[doc = "0x8c74 - Access Control Register"]
    pub access_ctrl_root24_set: ACCESS_CTRL_ROOT24_SET,
    #[doc = "0x8c78 - Access Control Register"]
    pub access_ctrl_root24_clr: ACCESS_CTRL_ROOT24_CLR,
    #[doc = "0x8c7c - Access Control Register"]
    pub access_ctrl_root24_tog: ACCESS_CTRL_ROOT24_TOG,
    #[doc = "0x8c80 - Target Register"]
    pub target_root25: TARGET_ROOT25,
    #[doc = "0x8c84 - Target Register"]
    pub target_root25_set: TARGET_ROOT25_SET,
    #[doc = "0x8c88 - Target Register"]
    pub target_root25_clr: TARGET_ROOT25_CLR,
    #[doc = "0x8c8c - Target Register"]
    pub target_root25_tog: TARGET_ROOT25_TOG,
    #[doc = "0x8c90 - Miscellaneous Register"]
    pub misc25: MISC25,
    #[doc = "0x8c94 - Miscellaneous Register"]
    pub misc_root25_set: MISC_ROOT25_SET,
    #[doc = "0x8c98 - Miscellaneous Register"]
    pub misc_root25_clr: MISC_ROOT25_CLR,
    #[doc = "0x8c9c - Miscellaneous Register"]
    pub misc_root25_tog: MISC_ROOT25_TOG,
    #[doc = "0x8ca0 - Post Divider Register"]
    pub post25: POST25,
    #[doc = "0x8ca4 - Post Divider Register"]
    pub post_root25_set: POST_ROOT25_SET,
    #[doc = "0x8ca8 - Post Divider Register"]
    pub post_root25_clr: POST_ROOT25_CLR,
    #[doc = "0x8cac - Post Divider Register"]
    pub post_root25_tog: POST_ROOT25_TOG,
    #[doc = "0x8cb0 - Pre Divider Register"]
    pub pre25: PRE25,
    #[doc = "0x8cb4 - Pre Divider Register"]
    pub pre_root25_set: PRE_ROOT25_SET,
    #[doc = "0x8cb8 - Pre Divider Register"]
    pub pre_root25_clr: PRE_ROOT25_CLR,
    #[doc = "0x8cbc - Pre Divider Register"]
    pub pre_root25_tog: PRE_ROOT25_TOG,
    _reserved18: [u8; 48usize],
    #[doc = "0x8cf0 - Access Control Register"]
    pub access_ctrl25: ACCESS_CTRL25,
    #[doc = "0x8cf4 - Access Control Register"]
    pub access_ctrl_root25_set: ACCESS_CTRL_ROOT25_SET,
    #[doc = "0x8cf8 - Access Control Register"]
    pub access_ctrl_root25_clr: ACCESS_CTRL_ROOT25_CLR,
    #[doc = "0x8cfc - Access Control Register"]
    pub access_ctrl_root25_tog: ACCESS_CTRL_ROOT25_TOG,
    #[doc = "0x8d00 - Target Register"]
    pub target_root26: TARGET_ROOT26,
    #[doc = "0x8d04 - Target Register"]
    pub target_root26_set: TARGET_ROOT26_SET,
    #[doc = "0x8d08 - Target Register"]
    pub target_root26_clr: TARGET_ROOT26_CLR,
    #[doc = "0x8d0c - Target Register"]
    pub target_root26_tog: TARGET_ROOT26_TOG,
    #[doc = "0x8d10 - Miscellaneous Register"]
    pub misc26: MISC26,
    #[doc = "0x8d14 - Miscellaneous Register"]
    pub misc_root26_set: MISC_ROOT26_SET,
    #[doc = "0x8d18 - Miscellaneous Register"]
    pub misc_root26_clr: MISC_ROOT26_CLR,
    #[doc = "0x8d1c - Miscellaneous Register"]
    pub misc_root26_tog: MISC_ROOT26_TOG,
    #[doc = "0x8d20 - Post Divider Register"]
    pub post26: POST26,
    #[doc = "0x8d24 - Post Divider Register"]
    pub post_root26_set: POST_ROOT26_SET,
    #[doc = "0x8d28 - Post Divider Register"]
    pub post_root26_clr: POST_ROOT26_CLR,
    #[doc = "0x8d2c - Post Divider Register"]
    pub post_root26_tog: POST_ROOT26_TOG,
    #[doc = "0x8d30 - Pre Divider Register"]
    pub pre26: PRE26,
    #[doc = "0x8d34 - Pre Divider Register"]
    pub pre_root26_set: PRE_ROOT26_SET,
    #[doc = "0x8d38 - Pre Divider Register"]
    pub pre_root26_clr: PRE_ROOT26_CLR,
    #[doc = "0x8d3c - Pre Divider Register"]
    pub pre_root26_tog: PRE_ROOT26_TOG,
    _reserved19: [u8; 48usize],
    #[doc = "0x8d70 - Access Control Register"]
    pub access_ctrl26: ACCESS_CTRL26,
    #[doc = "0x8d74 - Access Control Register"]
    pub access_ctrl_root26_set: ACCESS_CTRL_ROOT26_SET,
    #[doc = "0x8d78 - Access Control Register"]
    pub access_ctrl_root26_clr: ACCESS_CTRL_ROOT26_CLR,
    #[doc = "0x8d7c - Access Control Register"]
    pub access_ctrl_root26_tog: ACCESS_CTRL_ROOT26_TOG,
    #[doc = "0x8d80 - Target Register"]
    pub target_root27: TARGET_ROOT27,
    #[doc = "0x8d84 - Target Register"]
    pub target_root27_set: TARGET_ROOT27_SET,
    #[doc = "0x8d88 - Target Register"]
    pub target_root27_clr: TARGET_ROOT27_CLR,
    #[doc = "0x8d8c - Target Register"]
    pub target_root27_tog: TARGET_ROOT27_TOG,
    #[doc = "0x8d90 - Miscellaneous Register"]
    pub misc27: MISC27,
    #[doc = "0x8d94 - Miscellaneous Register"]
    pub misc_root27_set: MISC_ROOT27_SET,
    #[doc = "0x8d98 - Miscellaneous Register"]
    pub misc_root27_clr: MISC_ROOT27_CLR,
    #[doc = "0x8d9c - Miscellaneous Register"]
    pub misc_root27_tog: MISC_ROOT27_TOG,
    #[doc = "0x8da0 - Post Divider Register"]
    pub post27: POST27,
    #[doc = "0x8da4 - Post Divider Register"]
    pub post_root27_set: POST_ROOT27_SET,
    #[doc = "0x8da8 - Post Divider Register"]
    pub post_root27_clr: POST_ROOT27_CLR,
    #[doc = "0x8dac - Post Divider Register"]
    pub post_root27_tog: POST_ROOT27_TOG,
    #[doc = "0x8db0 - Pre Divider Register"]
    pub pre27: PRE27,
    #[doc = "0x8db4 - Pre Divider Register"]
    pub pre_root27_set: PRE_ROOT27_SET,
    #[doc = "0x8db8 - Pre Divider Register"]
    pub pre_root27_clr: PRE_ROOT27_CLR,
    #[doc = "0x8dbc - Pre Divider Register"]
    pub pre_root27_tog: PRE_ROOT27_TOG,
    _reserved20: [u8; 48usize],
    #[doc = "0x8df0 - Access Control Register"]
    pub access_ctrl27: ACCESS_CTRL27,
    #[doc = "0x8df4 - Access Control Register"]
    pub access_ctrl_root27_set: ACCESS_CTRL_ROOT27_SET,
    #[doc = "0x8df8 - Access Control Register"]
    pub access_ctrl_root27_clr: ACCESS_CTRL_ROOT27_CLR,
    #[doc = "0x8dfc - Access Control Register"]
    pub access_ctrl_root27_tog: ACCESS_CTRL_ROOT27_TOG,
    _reserved21: [u8; 512usize],
    #[doc = "0x9000 - Target Register"]
    pub target_root32: TARGET_ROOT32,
    #[doc = "0x9004 - Target Register"]
    pub target_root32_set: TARGET_ROOT32_SET,
    #[doc = "0x9008 - Target Register"]
    pub target_root32_clr: TARGET_ROOT32_CLR,
    #[doc = "0x900c - Target Register"]
    pub target_root32_tog: TARGET_ROOT32_TOG,
    #[doc = "0x9010 - Miscellaneous Register"]
    pub misc32: MISC32,
    #[doc = "0x9014 - Miscellaneous Register"]
    pub misc_root32_set: MISC_ROOT32_SET,
    #[doc = "0x9018 - Miscellaneous Register"]
    pub misc_root32_clr: MISC_ROOT32_CLR,
    #[doc = "0x901c - Miscellaneous Register"]
    pub misc_root32_tog: MISC_ROOT32_TOG,
    #[doc = "0x9020 - Post Divider Register"]
    pub post32: POST32,
    #[doc = "0x9024 - Post Divider Register"]
    pub post_root32_set: POST_ROOT32_SET,
    #[doc = "0x9028 - Post Divider Register"]
    pub post_root32_clr: POST_ROOT32_CLR,
    #[doc = "0x902c - Post Divider Register"]
    pub post_root32_tog: POST_ROOT32_TOG,
    #[doc = "0x9030 - Pre Divider Register"]
    pub pre32: PRE32,
    #[doc = "0x9034 - Pre Divider Register"]
    pub pre_root32_set: PRE_ROOT32_SET,
    #[doc = "0x9038 - Pre Divider Register"]
    pub pre_root32_clr: PRE_ROOT32_CLR,
    #[doc = "0x903c - Pre Divider Register"]
    pub pre_root32_tog: PRE_ROOT32_TOG,
    _reserved22: [u8; 48usize],
    #[doc = "0x9070 - Access Control Register"]
    pub access_ctrl32: ACCESS_CTRL32,
    #[doc = "0x9074 - Access Control Register"]
    pub access_ctrl_root32_set: ACCESS_CTRL_ROOT32_SET,
    #[doc = "0x9078 - Access Control Register"]
    pub access_ctrl_root32_clr: ACCESS_CTRL_ROOT32_CLR,
    #[doc = "0x907c - Access Control Register"]
    pub access_ctrl_root32_tog: ACCESS_CTRL_ROOT32_TOG,
    #[doc = "0x9080 - Target Register"]
    pub target_root33: TARGET_ROOT33,
    #[doc = "0x9084 - Target Register"]
    pub target_root33_set: TARGET_ROOT33_SET,
    #[doc = "0x9088 - Target Register"]
    pub target_root33_clr: TARGET_ROOT33_CLR,
    #[doc = "0x908c - Target Register"]
    pub target_root33_tog: TARGET_ROOT33_TOG,
    #[doc = "0x9090 - Miscellaneous Register"]
    pub misc33: MISC33,
    #[doc = "0x9094 - Miscellaneous Register"]
    pub misc_root33_set: MISC_ROOT33_SET,
    #[doc = "0x9098 - Miscellaneous Register"]
    pub misc_root33_clr: MISC_ROOT33_CLR,
    #[doc = "0x909c - Miscellaneous Register"]
    pub misc_root33_tog: MISC_ROOT33_TOG,
    #[doc = "0x90a0 - Post Divider Register"]
    pub post33: POST33,
    #[doc = "0x90a4 - Post Divider Register"]
    pub post_root33_set: POST_ROOT33_SET,
    #[doc = "0x90a8 - Post Divider Register"]
    pub post_root33_clr: POST_ROOT33_CLR,
    #[doc = "0x90ac - Post Divider Register"]
    pub post_root33_tog: POST_ROOT33_TOG,
    #[doc = "0x90b0 - Pre Divider Register"]
    pub pre33: PRE33,
    #[doc = "0x90b4 - Pre Divider Register"]
    pub pre_root33_set: PRE_ROOT33_SET,
    #[doc = "0x90b8 - Pre Divider Register"]
    pub pre_root33_clr: PRE_ROOT33_CLR,
    #[doc = "0x90bc - Pre Divider Register"]
    pub pre_root33_tog: PRE_ROOT33_TOG,
    _reserved23: [u8; 48usize],
    #[doc = "0x90f0 - Access Control Register"]
    pub access_ctrl33: ACCESS_CTRL33,
    #[doc = "0x90f4 - Access Control Register"]
    pub access_ctrl_root33_set: ACCESS_CTRL_ROOT33_SET,
    #[doc = "0x90f8 - Access Control Register"]
    pub access_ctrl_root33_clr: ACCESS_CTRL_ROOT33_CLR,
    #[doc = "0x90fc - Access Control Register"]
    pub access_ctrl_root33_tog: ACCESS_CTRL_ROOT33_TOG,
    #[doc = "0x9100 - Target Register"]
    pub target_root34: TARGET_ROOT34,
    #[doc = "0x9104 - Target Register"]
    pub target_root34_set: TARGET_ROOT34_SET,
    #[doc = "0x9108 - Target Register"]
    pub target_root34_clr: TARGET_ROOT34_CLR,
    #[doc = "0x910c - Target Register"]
    pub target_root34_tog: TARGET_ROOT34_TOG,
    #[doc = "0x9110 - Miscellaneous Register"]
    pub misc34: MISC34,
    #[doc = "0x9114 - Miscellaneous Register"]
    pub misc_root34_set: MISC_ROOT34_SET,
    #[doc = "0x9118 - Miscellaneous Register"]
    pub misc_root34_clr: MISC_ROOT34_CLR,
    #[doc = "0x911c - Miscellaneous Register"]
    pub misc_root34_tog: MISC_ROOT34_TOG,
    #[doc = "0x9120 - Post Divider Register"]
    pub post34: POST34,
    #[doc = "0x9124 - Post Divider Register"]
    pub post_root34_set: POST_ROOT34_SET,
    #[doc = "0x9128 - Post Divider Register"]
    pub post_root34_clr: POST_ROOT34_CLR,
    #[doc = "0x912c - Post Divider Register"]
    pub post_root34_tog: POST_ROOT34_TOG,
    #[doc = "0x9130 - Pre Divider Register"]
    pub pre34: PRE34,
    #[doc = "0x9134 - Pre Divider Register"]
    pub pre_root34_set: PRE_ROOT34_SET,
    #[doc = "0x9138 - Pre Divider Register"]
    pub pre_root34_clr: PRE_ROOT34_CLR,
    #[doc = "0x913c - Pre Divider Register"]
    pub pre_root34_tog: PRE_ROOT34_TOG,
    _reserved24: [u8; 48usize],
    #[doc = "0x9170 - Access Control Register"]
    pub access_ctrl34: ACCESS_CTRL34,
    #[doc = "0x9174 - Access Control Register"]
    pub access_ctrl_root34_set: ACCESS_CTRL_ROOT34_SET,
    #[doc = "0x9178 - Access Control Register"]
    pub access_ctrl_root34_clr: ACCESS_CTRL_ROOT34_CLR,
    #[doc = "0x917c - Access Control Register"]
    pub access_ctrl_root34_tog: ACCESS_CTRL_ROOT34_TOG,
    #[doc = "0x9180 - Target Register"]
    pub target_root35: TARGET_ROOT35,
    #[doc = "0x9184 - Target Register"]
    pub target_root35_set: TARGET_ROOT35_SET,
    #[doc = "0x9188 - Target Register"]
    pub target_root35_clr: TARGET_ROOT35_CLR,
    #[doc = "0x918c - Target Register"]
    pub target_root35_tog: TARGET_ROOT35_TOG,
    #[doc = "0x9190 - Miscellaneous Register"]
    pub misc35: MISC35,
    #[doc = "0x9194 - Miscellaneous Register"]
    pub misc_root35_set: MISC_ROOT35_SET,
    #[doc = "0x9198 - Miscellaneous Register"]
    pub misc_root35_clr: MISC_ROOT35_CLR,
    #[doc = "0x919c - Miscellaneous Register"]
    pub misc_root35_tog: MISC_ROOT35_TOG,
    #[doc = "0x91a0 - Post Divider Register"]
    pub post35: POST35,
    #[doc = "0x91a4 - Post Divider Register"]
    pub post_root35_set: POST_ROOT35_SET,
    #[doc = "0x91a8 - Post Divider Register"]
    pub post_root35_clr: POST_ROOT35_CLR,
    #[doc = "0x91ac - Post Divider Register"]
    pub post_root35_tog: POST_ROOT35_TOG,
    #[doc = "0x91b0 - Pre Divider Register"]
    pub pre35: PRE35,
    #[doc = "0x91b4 - Pre Divider Register"]
    pub pre_root35_set: PRE_ROOT35_SET,
    #[doc = "0x91b8 - Pre Divider Register"]
    pub pre_root35_clr: PRE_ROOT35_CLR,
    #[doc = "0x91bc - Pre Divider Register"]
    pub pre_root35_tog: PRE_ROOT35_TOG,
    _reserved25: [u8; 48usize],
    #[doc = "0x91f0 - Access Control Register"]
    pub access_ctrl35: ACCESS_CTRL35,
    #[doc = "0x91f4 - Access Control Register"]
    pub access_ctrl_root35_set: ACCESS_CTRL_ROOT35_SET,
    #[doc = "0x91f8 - Access Control Register"]
    pub access_ctrl_root35_clr: ACCESS_CTRL_ROOT35_CLR,
    #[doc = "0x91fc - Access Control Register"]
    pub access_ctrl_root35_tog: ACCESS_CTRL_ROOT35_TOG,
    #[doc = "0x9200 - Target Register"]
    pub target_root36: TARGET_ROOT36,
    #[doc = "0x9204 - Target Register"]
    pub target_root36_set: TARGET_ROOT36_SET,
    #[doc = "0x9208 - Target Register"]
    pub target_root36_clr: TARGET_ROOT36_CLR,
    #[doc = "0x920c - Target Register"]
    pub target_root36_tog: TARGET_ROOT36_TOG,
    #[doc = "0x9210 - Miscellaneous Register"]
    pub misc36: MISC36,
    #[doc = "0x9214 - Miscellaneous Register"]
    pub misc_root36_set: MISC_ROOT36_SET,
    #[doc = "0x9218 - Miscellaneous Register"]
    pub misc_root36_clr: MISC_ROOT36_CLR,
    #[doc = "0x921c - Miscellaneous Register"]
    pub misc_root36_tog: MISC_ROOT36_TOG,
    #[doc = "0x9220 - Post Divider Register"]
    pub post36: POST36,
    #[doc = "0x9224 - Post Divider Register"]
    pub post_root36_set: POST_ROOT36_SET,
    #[doc = "0x9228 - Post Divider Register"]
    pub post_root36_clr: POST_ROOT36_CLR,
    #[doc = "0x922c - Post Divider Register"]
    pub post_root36_tog: POST_ROOT36_TOG,
    #[doc = "0x9230 - Pre Divider Register"]
    pub pre36: PRE36,
    #[doc = "0x9234 - Pre Divider Register"]
    pub pre_root36_set: PRE_ROOT36_SET,
    #[doc = "0x9238 - Pre Divider Register"]
    pub pre_root36_clr: PRE_ROOT36_CLR,
    #[doc = "0x923c - Pre Divider Register"]
    pub pre_root36_tog: PRE_ROOT36_TOG,
    _reserved26: [u8; 48usize],
    #[doc = "0x9270 - Access Control Register"]
    pub access_ctrl36: ACCESS_CTRL36,
    #[doc = "0x9274 - Access Control Register"]
    pub access_ctrl_root36_set: ACCESS_CTRL_ROOT36_SET,
    #[doc = "0x9278 - Access Control Register"]
    pub access_ctrl_root36_clr: ACCESS_CTRL_ROOT36_CLR,
    #[doc = "0x927c - Access Control Register"]
    pub access_ctrl_root36_tog: ACCESS_CTRL_ROOT36_TOG,
    #[doc = "0x9280 - Target Register"]
    pub target_root37: TARGET_ROOT37,
    #[doc = "0x9284 - Target Register"]
    pub target_root37_set: TARGET_ROOT37_SET,
    #[doc = "0x9288 - Target Register"]
    pub target_root37_clr: TARGET_ROOT37_CLR,
    #[doc = "0x928c - Target Register"]
    pub target_root37_tog: TARGET_ROOT37_TOG,
    #[doc = "0x9290 - Miscellaneous Register"]
    pub misc37: MISC37,
    #[doc = "0x9294 - Miscellaneous Register"]
    pub misc_root37_set: MISC_ROOT37_SET,
    #[doc = "0x9298 - Miscellaneous Register"]
    pub misc_root37_clr: MISC_ROOT37_CLR,
    #[doc = "0x929c - Miscellaneous Register"]
    pub misc_root37_tog: MISC_ROOT37_TOG,
    #[doc = "0x92a0 - Post Divider Register"]
    pub post37: POST37,
    #[doc = "0x92a4 - Post Divider Register"]
    pub post_root37_set: POST_ROOT37_SET,
    #[doc = "0x92a8 - Post Divider Register"]
    pub post_root37_clr: POST_ROOT37_CLR,
    #[doc = "0x92ac - Post Divider Register"]
    pub post_root37_tog: POST_ROOT37_TOG,
    #[doc = "0x92b0 - Pre Divider Register"]
    pub pre37: PRE37,
    #[doc = "0x92b4 - Pre Divider Register"]
    pub pre_root37_set: PRE_ROOT37_SET,
    #[doc = "0x92b8 - Pre Divider Register"]
    pub pre_root37_clr: PRE_ROOT37_CLR,
    #[doc = "0x92bc - Pre Divider Register"]
    pub pre_root37_tog: PRE_ROOT37_TOG,
    _reserved27: [u8; 48usize],
    #[doc = "0x92f0 - Access Control Register"]
    pub access_ctrl37: ACCESS_CTRL37,
    #[doc = "0x92f4 - Access Control Register"]
    pub access_ctrl_root37_set: ACCESS_CTRL_ROOT37_SET,
    #[doc = "0x92f8 - Access Control Register"]
    pub access_ctrl_root37_clr: ACCESS_CTRL_ROOT37_CLR,
    #[doc = "0x92fc - Access Control Register"]
    pub access_ctrl_root37_tog: ACCESS_CTRL_ROOT37_TOG,
    _reserved28: [u8; 1280usize],
    #[doc = "0x9800 - Target Register"]
    pub target_root48: TARGET_ROOT48,
    #[doc = "0x9804 - Target Register"]
    pub target_root48_set: TARGET_ROOT48_SET,
    #[doc = "0x9808 - Target Register"]
    pub target_root48_clr: TARGET_ROOT48_CLR,
    #[doc = "0x980c - Target Register"]
    pub target_root48_tog: TARGET_ROOT48_TOG,
    #[doc = "0x9810 - Miscellaneous Register"]
    pub misc48: MISC48,
    #[doc = "0x9814 - Miscellaneous Register"]
    pub misc_root48_set: MISC_ROOT48_SET,
    #[doc = "0x9818 - Miscellaneous Register"]
    pub misc_root48_clr: MISC_ROOT48_CLR,
    #[doc = "0x981c - Miscellaneous Register"]
    pub misc_root48_tog: MISC_ROOT48_TOG,
    #[doc = "0x9820 - Post Divider Register"]
    pub post48: POST48,
    #[doc = "0x9824 - Post Divider Register"]
    pub post_root48_set: POST_ROOT48_SET,
    #[doc = "0x9828 - Post Divider Register"]
    pub post_root48_clr: POST_ROOT48_CLR,
    #[doc = "0x982c - Post Divider Register"]
    pub post_root48_tog: POST_ROOT48_TOG,
    #[doc = "0x9830 - Pre Divider Register"]
    pub pre48: PRE48,
    #[doc = "0x9834 - Pre Divider Register"]
    pub pre_root48_set: PRE_ROOT48_SET,
    #[doc = "0x9838 - Pre Divider Register"]
    pub pre_root48_clr: PRE_ROOT48_CLR,
    #[doc = "0x983c - Pre Divider Register"]
    pub pre_root48_tog: PRE_ROOT48_TOG,
    _reserved29: [u8; 48usize],
    #[doc = "0x9870 - Access Control Register"]
    pub access_ctrl48: ACCESS_CTRL48,
    #[doc = "0x9874 - Access Control Register"]
    pub access_ctrl_root48_set: ACCESS_CTRL_ROOT48_SET,
    #[doc = "0x9878 - Access Control Register"]
    pub access_ctrl_root48_clr: ACCESS_CTRL_ROOT48_CLR,
    #[doc = "0x987c - Access Control Register"]
    pub access_ctrl_root48_tog: ACCESS_CTRL_ROOT48_TOG,
    #[doc = "0x9880 - Target Register"]
    pub target_root49: TARGET_ROOT49,
    #[doc = "0x9884 - Target Register"]
    pub target_root49_set: TARGET_ROOT49_SET,
    #[doc = "0x9888 - Target Register"]
    pub target_root49_clr: TARGET_ROOT49_CLR,
    #[doc = "0x988c - Target Register"]
    pub target_root49_tog: TARGET_ROOT49_TOG,
    #[doc = "0x9890 - Miscellaneous Register"]
    pub misc49: MISC49,
    #[doc = "0x9894 - Miscellaneous Register"]
    pub misc_root49_set: MISC_ROOT49_SET,
    #[doc = "0x9898 - Miscellaneous Register"]
    pub misc_root49_clr: MISC_ROOT49_CLR,
    #[doc = "0x989c - Miscellaneous Register"]
    pub misc_root49_tog: MISC_ROOT49_TOG,
    #[doc = "0x98a0 - Post Divider Register"]
    pub post49: POST49,
    #[doc = "0x98a4 - Post Divider Register"]
    pub post_root49_set: POST_ROOT49_SET,
    #[doc = "0x98a8 - Post Divider Register"]
    pub post_root49_clr: POST_ROOT49_CLR,
    #[doc = "0x98ac - Post Divider Register"]
    pub post_root49_tog: POST_ROOT49_TOG,
    #[doc = "0x98b0 - Pre Divider Register"]
    pub pre49: PRE49,
    #[doc = "0x98b4 - Pre Divider Register"]
    pub pre_root49_set: PRE_ROOT49_SET,
    #[doc = "0x98b8 - Pre Divider Register"]
    pub pre_root49_clr: PRE_ROOT49_CLR,
    #[doc = "0x98bc - Pre Divider Register"]
    pub pre_root49_tog: PRE_ROOT49_TOG,
    _reserved30: [u8; 48usize],
    #[doc = "0x98f0 - Access Control Register"]
    pub access_ctrl49: ACCESS_CTRL49,
    #[doc = "0x98f4 - Access Control Register"]
    pub access_ctrl_root49_set: ACCESS_CTRL_ROOT49_SET,
    #[doc = "0x98f8 - Access Control Register"]
    pub access_ctrl_root49_clr: ACCESS_CTRL_ROOT49_CLR,
    #[doc = "0x98fc - Access Control Register"]
    pub access_ctrl_root49_tog: ACCESS_CTRL_ROOT49_TOG,
    _reserved31: [u8; 1792usize],
    #[doc = "0xa000 - Target Register"]
    pub target_root64: TARGET_ROOT64,
    #[doc = "0xa004 - Target Register"]
    pub target_root64_set: TARGET_ROOT64_SET,
    #[doc = "0xa008 - Target Register"]
    pub target_root64_clr: TARGET_ROOT64_CLR,
    #[doc = "0xa00c - Target Register"]
    pub target_root64_tog: TARGET_ROOT64_TOG,
    #[doc = "0xa010 - Miscellaneous Register"]
    pub misc64: MISC64,
    #[doc = "0xa014 - Miscellaneous Register"]
    pub misc_root64_set: MISC_ROOT64_SET,
    #[doc = "0xa018 - Miscellaneous Register"]
    pub misc_root64_clr: MISC_ROOT64_CLR,
    #[doc = "0xa01c - Miscellaneous Register"]
    pub misc_root64_tog: MISC_ROOT64_TOG,
    #[doc = "0xa020 - Post Divider Register"]
    pub post64: POST64,
    #[doc = "0xa024 - Post Divider Register"]
    pub post_root64_set: POST_ROOT64_SET,
    #[doc = "0xa028 - Post Divider Register"]
    pub post_root64_clr: POST_ROOT64_CLR,
    #[doc = "0xa02c - Post Divider Register"]
    pub post_root64_tog: POST_ROOT64_TOG,
    #[doc = "0xa030 - Pre Divider Register"]
    pub pre64: PRE64,
    #[doc = "0xa034 - Pre Divider Register"]
    pub pre_root64_set: PRE_ROOT64_SET,
    #[doc = "0xa038 - Pre Divider Register"]
    pub pre_root64_clr: PRE_ROOT64_CLR,
    #[doc = "0xa03c - Pre Divider Register"]
    pub pre_root64_tog: PRE_ROOT64_TOG,
    _reserved32: [u8; 48usize],
    #[doc = "0xa070 - Access Control Register"]
    pub access_ctrl64: ACCESS_CTRL64,
    #[doc = "0xa074 - Access Control Register"]
    pub access_ctrl_root64_set: ACCESS_CTRL_ROOT64_SET,
    #[doc = "0xa078 - Access Control Register"]
    pub access_ctrl_root64_clr: ACCESS_CTRL_ROOT64_CLR,
    #[doc = "0xa07c - Access Control Register"]
    pub access_ctrl_root64_tog: ACCESS_CTRL_ROOT64_TOG,
    #[doc = "0xa080 - Target Register"]
    pub target_root65: TARGET_ROOT65,
    #[doc = "0xa084 - Target Register"]
    pub target_root65_set: TARGET_ROOT65_SET,
    #[doc = "0xa088 - Target Register"]
    pub target_root65_clr: TARGET_ROOT65_CLR,
    #[doc = "0xa08c - Target Register"]
    pub target_root65_tog: TARGET_ROOT65_TOG,
    #[doc = "0xa090 - Miscellaneous Register"]
    pub misc65: MISC65,
    #[doc = "0xa094 - Miscellaneous Register"]
    pub misc_root65_set: MISC_ROOT65_SET,
    #[doc = "0xa098 - Miscellaneous Register"]
    pub misc_root65_clr: MISC_ROOT65_CLR,
    #[doc = "0xa09c - Miscellaneous Register"]
    pub misc_root65_tog: MISC_ROOT65_TOG,
    #[doc = "0xa0a0 - Post Divider Register"]
    pub post65: POST65,
    #[doc = "0xa0a4 - Post Divider Register"]
    pub post_root65_set: POST_ROOT65_SET,
    #[doc = "0xa0a8 - Post Divider Register"]
    pub post_root65_clr: POST_ROOT65_CLR,
    #[doc = "0xa0ac - Post Divider Register"]
    pub post_root65_tog: POST_ROOT65_TOG,
    #[doc = "0xa0b0 - Pre Divider Register"]
    pub pre65: PRE65,
    #[doc = "0xa0b4 - Pre Divider Register"]
    pub pre_root65_set: PRE_ROOT65_SET,
    #[doc = "0xa0b8 - Pre Divider Register"]
    pub pre_root65_clr: PRE_ROOT65_CLR,
    #[doc = "0xa0bc - Pre Divider Register"]
    pub pre_root65_tog: PRE_ROOT65_TOG,
    _reserved33: [u8; 48usize],
    #[doc = "0xa0f0 - Access Control Register"]
    pub access_ctrl65: ACCESS_CTRL65,
    #[doc = "0xa0f4 - Access Control Register"]
    pub access_ctrl_root65_set: ACCESS_CTRL_ROOT65_SET,
    #[doc = "0xa0f8 - Access Control Register"]
    pub access_ctrl_root65_clr: ACCESS_CTRL_ROOT65_CLR,
    #[doc = "0xa0fc - Access Control Register"]
    pub access_ctrl_root65_tog: ACCESS_CTRL_ROOT65_TOG,
    #[doc = "0xa100 - Target Register"]
    pub target_root66: TARGET_ROOT66,
    #[doc = "0xa104 - Target Register"]
    pub target_root66_set: TARGET_ROOT66_SET,
    #[doc = "0xa108 - Target Register"]
    pub target_root66_clr: TARGET_ROOT66_CLR,
    #[doc = "0xa10c - Target Register"]
    pub target_root66_tog: TARGET_ROOT66_TOG,
    #[doc = "0xa110 - Miscellaneous Register"]
    pub misc66: MISC66,
    #[doc = "0xa114 - Miscellaneous Register"]
    pub misc_root66_set: MISC_ROOT66_SET,
    #[doc = "0xa118 - Miscellaneous Register"]
    pub misc_root66_clr: MISC_ROOT66_CLR,
    #[doc = "0xa11c - Miscellaneous Register"]
    pub misc_root66_tog: MISC_ROOT66_TOG,
    #[doc = "0xa120 - Post Divider Register"]
    pub post66: POST66,
    #[doc = "0xa124 - Post Divider Register"]
    pub post_root66_set: POST_ROOT66_SET,
    #[doc = "0xa128 - Post Divider Register"]
    pub post_root66_clr: POST_ROOT66_CLR,
    #[doc = "0xa12c - Post Divider Register"]
    pub post_root66_tog: POST_ROOT66_TOG,
    #[doc = "0xa130 - Pre Divider Register"]
    pub pre66: PRE66,
    #[doc = "0xa134 - Pre Divider Register"]
    pub pre_root66_set: PRE_ROOT66_SET,
    #[doc = "0xa138 - Pre Divider Register"]
    pub pre_root66_clr: PRE_ROOT66_CLR,
    #[doc = "0xa13c - Pre Divider Register"]
    pub pre_root66_tog: PRE_ROOT66_TOG,
    _reserved34: [u8; 48usize],
    #[doc = "0xa170 - Access Control Register"]
    pub access_ctrl66: ACCESS_CTRL66,
    #[doc = "0xa174 - Access Control Register"]
    pub access_ctrl_root66_set: ACCESS_CTRL_ROOT66_SET,
    #[doc = "0xa178 - Access Control Register"]
    pub access_ctrl_root66_clr: ACCESS_CTRL_ROOT66_CLR,
    #[doc = "0xa17c - Access Control Register"]
    pub access_ctrl_root66_tog: ACCESS_CTRL_ROOT66_TOG,
    #[doc = "0xa180 - Target Register"]
    pub target_root67: TARGET_ROOT67,
    #[doc = "0xa184 - Target Register"]
    pub target_root67_set: TARGET_ROOT67_SET,
    #[doc = "0xa188 - Target Register"]
    pub target_root67_clr: TARGET_ROOT67_CLR,
    #[doc = "0xa18c - Target Register"]
    pub target_root67_tog: TARGET_ROOT67_TOG,
    #[doc = "0xa190 - Miscellaneous Register"]
    pub misc67: MISC67,
    #[doc = "0xa194 - Miscellaneous Register"]
    pub misc_root67_set: MISC_ROOT67_SET,
    #[doc = "0xa198 - Miscellaneous Register"]
    pub misc_root67_clr: MISC_ROOT67_CLR,
    #[doc = "0xa19c - Miscellaneous Register"]
    pub misc_root67_tog: MISC_ROOT67_TOG,
    #[doc = "0xa1a0 - Post Divider Register"]
    pub post67: POST67,
    #[doc = "0xa1a4 - Post Divider Register"]
    pub post_root67_set: POST_ROOT67_SET,
    #[doc = "0xa1a8 - Post Divider Register"]
    pub post_root67_clr: POST_ROOT67_CLR,
    #[doc = "0xa1ac - Post Divider Register"]
    pub post_root67_tog: POST_ROOT67_TOG,
    #[doc = "0xa1b0 - Pre Divider Register"]
    pub pre67: PRE67,
    #[doc = "0xa1b4 - Pre Divider Register"]
    pub pre_root67_set: PRE_ROOT67_SET,
    #[doc = "0xa1b8 - Pre Divider Register"]
    pub pre_root67_clr: PRE_ROOT67_CLR,
    #[doc = "0xa1bc - Pre Divider Register"]
    pub pre_root67_tog: PRE_ROOT67_TOG,
    _reserved35: [u8; 48usize],
    #[doc = "0xa1f0 - Access Control Register"]
    pub access_ctrl67: ACCESS_CTRL67,
    #[doc = "0xa1f4 - Access Control Register"]
    pub access_ctrl_root67_set: ACCESS_CTRL_ROOT67_SET,
    #[doc = "0xa1f8 - Access Control Register"]
    pub access_ctrl_root67_clr: ACCESS_CTRL_ROOT67_CLR,
    #[doc = "0xa1fc - Access Control Register"]
    pub access_ctrl_root67_tog: ACCESS_CTRL_ROOT67_TOG,
    #[doc = "0xa200 - Target Register"]
    pub target_root68: TARGET_ROOT68,
    #[doc = "0xa204 - Target Register"]
    pub target_root68_set: TARGET_ROOT68_SET,
    #[doc = "0xa208 - Target Register"]
    pub target_root68_clr: TARGET_ROOT68_CLR,
    #[doc = "0xa20c - Target Register"]
    pub target_root68_tog: TARGET_ROOT68_TOG,
    #[doc = "0xa210 - Miscellaneous Register"]
    pub misc68: MISC68,
    #[doc = "0xa214 - Miscellaneous Register"]
    pub misc_root68_set: MISC_ROOT68_SET,
    #[doc = "0xa218 - Miscellaneous Register"]
    pub misc_root68_clr: MISC_ROOT68_CLR,
    #[doc = "0xa21c - Miscellaneous Register"]
    pub misc_root68_tog: MISC_ROOT68_TOG,
    #[doc = "0xa220 - Post Divider Register"]
    pub post68: POST68,
    #[doc = "0xa224 - Post Divider Register"]
    pub post_root68_set: POST_ROOT68_SET,
    #[doc = "0xa228 - Post Divider Register"]
    pub post_root68_clr: POST_ROOT68_CLR,
    #[doc = "0xa22c - Post Divider Register"]
    pub post_root68_tog: POST_ROOT68_TOG,
    #[doc = "0xa230 - Pre Divider Register"]
    pub pre68: PRE68,
    #[doc = "0xa234 - Pre Divider Register"]
    pub pre_root68_set: PRE_ROOT68_SET,
    #[doc = "0xa238 - Pre Divider Register"]
    pub pre_root68_clr: PRE_ROOT68_CLR,
    #[doc = "0xa23c - Pre Divider Register"]
    pub pre_root68_tog: PRE_ROOT68_TOG,
    _reserved36: [u8; 48usize],
    #[doc = "0xa270 - Access Control Register"]
    pub access_ctrl68: ACCESS_CTRL68,
    #[doc = "0xa274 - Access Control Register"]
    pub access_ctrl_root68_set: ACCESS_CTRL_ROOT68_SET,
    #[doc = "0xa278 - Access Control Register"]
    pub access_ctrl_root68_clr: ACCESS_CTRL_ROOT68_CLR,
    #[doc = "0xa27c - Access Control Register"]
    pub access_ctrl_root68_tog: ACCESS_CTRL_ROOT68_TOG,
    #[doc = "0xa280 - Target Register"]
    pub target_root69: TARGET_ROOT69,
    #[doc = "0xa284 - Target Register"]
    pub target_root69_set: TARGET_ROOT69_SET,
    #[doc = "0xa288 - Target Register"]
    pub target_root69_clr: TARGET_ROOT69_CLR,
    #[doc = "0xa28c - Target Register"]
    pub target_root69_tog: TARGET_ROOT69_TOG,
    #[doc = "0xa290 - Miscellaneous Register"]
    pub misc69: MISC69,
    #[doc = "0xa294 - Miscellaneous Register"]
    pub misc_root69_set: MISC_ROOT69_SET,
    #[doc = "0xa298 - Miscellaneous Register"]
    pub misc_root69_clr: MISC_ROOT69_CLR,
    #[doc = "0xa29c - Miscellaneous Register"]
    pub misc_root69_tog: MISC_ROOT69_TOG,
    #[doc = "0xa2a0 - Post Divider Register"]
    pub post69: POST69,
    #[doc = "0xa2a4 - Post Divider Register"]
    pub post_root69_set: POST_ROOT69_SET,
    #[doc = "0xa2a8 - Post Divider Register"]
    pub post_root69_clr: POST_ROOT69_CLR,
    #[doc = "0xa2ac - Post Divider Register"]
    pub post_root69_tog: POST_ROOT69_TOG,
    #[doc = "0xa2b0 - Pre Divider Register"]
    pub pre69: PRE69,
    #[doc = "0xa2b4 - Pre Divider Register"]
    pub pre_root69_set: PRE_ROOT69_SET,
    #[doc = "0xa2b8 - Pre Divider Register"]
    pub pre_root69_clr: PRE_ROOT69_CLR,
    #[doc = "0xa2bc - Pre Divider Register"]
    pub pre_root69_tog: PRE_ROOT69_TOG,
    _reserved37: [u8; 48usize],
    #[doc = "0xa2f0 - Access Control Register"]
    pub access_ctrl69: ACCESS_CTRL69,
    #[doc = "0xa2f4 - Access Control Register"]
    pub access_ctrl_root69_set: ACCESS_CTRL_ROOT69_SET,
    #[doc = "0xa2f8 - Access Control Register"]
    pub access_ctrl_root69_clr: ACCESS_CTRL_ROOT69_CLR,
    #[doc = "0xa2fc - Access Control Register"]
    pub access_ctrl_root69_tog: ACCESS_CTRL_ROOT69_TOG,
    #[doc = "0xa300 - Target Register"]
    pub target_root70: TARGET_ROOT70,
    #[doc = "0xa304 - Target Register"]
    pub target_root70_set: TARGET_ROOT70_SET,
    #[doc = "0xa308 - Target Register"]
    pub target_root70_clr: TARGET_ROOT70_CLR,
    #[doc = "0xa30c - Target Register"]
    pub target_root70_tog: TARGET_ROOT70_TOG,
    #[doc = "0xa310 - Miscellaneous Register"]
    pub misc70: MISC70,
    #[doc = "0xa314 - Miscellaneous Register"]
    pub misc_root70_set: MISC_ROOT70_SET,
    #[doc = "0xa318 - Miscellaneous Register"]
    pub misc_root70_clr: MISC_ROOT70_CLR,
    #[doc = "0xa31c - Miscellaneous Register"]
    pub misc_root70_tog: MISC_ROOT70_TOG,
    #[doc = "0xa320 - Post Divider Register"]
    pub post70: POST70,
    #[doc = "0xa324 - Post Divider Register"]
    pub post_root70_set: POST_ROOT70_SET,
    #[doc = "0xa328 - Post Divider Register"]
    pub post_root70_clr: POST_ROOT70_CLR,
    #[doc = "0xa32c - Post Divider Register"]
    pub post_root70_tog: POST_ROOT70_TOG,
    #[doc = "0xa330 - Pre Divider Register"]
    pub pre70: PRE70,
    #[doc = "0xa334 - Pre Divider Register"]
    pub pre_root70_set: PRE_ROOT70_SET,
    #[doc = "0xa338 - Pre Divider Register"]
    pub pre_root70_clr: PRE_ROOT70_CLR,
    #[doc = "0xa33c - Pre Divider Register"]
    pub pre_root70_tog: PRE_ROOT70_TOG,
    _reserved38: [u8; 48usize],
    #[doc = "0xa370 - Access Control Register"]
    pub access_ctrl70: ACCESS_CTRL70,
    #[doc = "0xa374 - Access Control Register"]
    pub access_ctrl_root70_set: ACCESS_CTRL_ROOT70_SET,
    #[doc = "0xa378 - Access Control Register"]
    pub access_ctrl_root70_clr: ACCESS_CTRL_ROOT70_CLR,
    #[doc = "0xa37c - Access Control Register"]
    pub access_ctrl_root70_tog: ACCESS_CTRL_ROOT70_TOG,
    #[doc = "0xa380 - Target Register"]
    pub target_root71: TARGET_ROOT71,
    #[doc = "0xa384 - Target Register"]
    pub target_root71_set: TARGET_ROOT71_SET,
    #[doc = "0xa388 - Target Register"]
    pub target_root71_clr: TARGET_ROOT71_CLR,
    #[doc = "0xa38c - Target Register"]
    pub target_root71_tog: TARGET_ROOT71_TOG,
    #[doc = "0xa390 - Miscellaneous Register"]
    pub misc71: MISC71,
    #[doc = "0xa394 - Miscellaneous Register"]
    pub misc_root71_set: MISC_ROOT71_SET,
    #[doc = "0xa398 - Miscellaneous Register"]
    pub misc_root71_clr: MISC_ROOT71_CLR,
    #[doc = "0xa39c - Miscellaneous Register"]
    pub misc_root71_tog: MISC_ROOT71_TOG,
    #[doc = "0xa3a0 - Post Divider Register"]
    pub post71: POST71,
    #[doc = "0xa3a4 - Post Divider Register"]
    pub post_root71_set: POST_ROOT71_SET,
    #[doc = "0xa3a8 - Post Divider Register"]
    pub post_root71_clr: POST_ROOT71_CLR,
    #[doc = "0xa3ac - Post Divider Register"]
    pub post_root71_tog: POST_ROOT71_TOG,
    #[doc = "0xa3b0 - Pre Divider Register"]
    pub pre71: PRE71,
    #[doc = "0xa3b4 - Pre Divider Register"]
    pub pre_root71_set: PRE_ROOT71_SET,
    #[doc = "0xa3b8 - Pre Divider Register"]
    pub pre_root71_clr: PRE_ROOT71_CLR,
    #[doc = "0xa3bc - Pre Divider Register"]
    pub pre_root71_tog: PRE_ROOT71_TOG,
    _reserved39: [u8; 48usize],
    #[doc = "0xa3f0 - Access Control Register"]
    pub access_ctrl71: ACCESS_CTRL71,
    #[doc = "0xa3f4 - Access Control Register"]
    pub access_ctrl_root71_set: ACCESS_CTRL_ROOT71_SET,
    #[doc = "0xa3f8 - Access Control Register"]
    pub access_ctrl_root71_clr: ACCESS_CTRL_ROOT71_CLR,
    #[doc = "0xa3fc - Access Control Register"]
    pub access_ctrl_root71_tog: ACCESS_CTRL_ROOT71_TOG,
    #[doc = "0xa400 - Target Register"]
    pub target_root72: TARGET_ROOT72,
    #[doc = "0xa404 - Target Register"]
    pub target_root72_set: TARGET_ROOT72_SET,
    #[doc = "0xa408 - Target Register"]
    pub target_root72_clr: TARGET_ROOT72_CLR,
    #[doc = "0xa40c - Target Register"]
    pub target_root72_tog: TARGET_ROOT72_TOG,
    #[doc = "0xa410 - Miscellaneous Register"]
    pub misc72: MISC72,
    #[doc = "0xa414 - Miscellaneous Register"]
    pub misc_root72_set: MISC_ROOT72_SET,
    #[doc = "0xa418 - Miscellaneous Register"]
    pub misc_root72_clr: MISC_ROOT72_CLR,
    #[doc = "0xa41c - Miscellaneous Register"]
    pub misc_root72_tog: MISC_ROOT72_TOG,
    #[doc = "0xa420 - Post Divider Register"]
    pub post72: POST72,
    #[doc = "0xa424 - Post Divider Register"]
    pub post_root72_set: POST_ROOT72_SET,
    #[doc = "0xa428 - Post Divider Register"]
    pub post_root72_clr: POST_ROOT72_CLR,
    #[doc = "0xa42c - Post Divider Register"]
    pub post_root72_tog: POST_ROOT72_TOG,
    #[doc = "0xa430 - Pre Divider Register"]
    pub pre72: PRE72,
    #[doc = "0xa434 - Pre Divider Register"]
    pub pre_root72_set: PRE_ROOT72_SET,
    #[doc = "0xa438 - Pre Divider Register"]
    pub pre_root72_clr: PRE_ROOT72_CLR,
    #[doc = "0xa43c - Pre Divider Register"]
    pub pre_root72_tog: PRE_ROOT72_TOG,
    _reserved40: [u8; 48usize],
    #[doc = "0xa470 - Access Control Register"]
    pub access_ctrl72: ACCESS_CTRL72,
    #[doc = "0xa474 - Access Control Register"]
    pub access_ctrl_root72_set: ACCESS_CTRL_ROOT72_SET,
    #[doc = "0xa478 - Access Control Register"]
    pub access_ctrl_root72_clr: ACCESS_CTRL_ROOT72_CLR,
    #[doc = "0xa47c - Access Control Register"]
    pub access_ctrl_root72_tog: ACCESS_CTRL_ROOT72_TOG,
    #[doc = "0xa480 - Target Register"]
    pub target_root73: TARGET_ROOT73,
    #[doc = "0xa484 - Target Register"]
    pub target_root73_set: TARGET_ROOT73_SET,
    #[doc = "0xa488 - Target Register"]
    pub target_root73_clr: TARGET_ROOT73_CLR,
    #[doc = "0xa48c - Target Register"]
    pub target_root73_tog: TARGET_ROOT73_TOG,
    #[doc = "0xa490 - Miscellaneous Register"]
    pub misc73: MISC73,
    #[doc = "0xa494 - Miscellaneous Register"]
    pub misc_root73_set: MISC_ROOT73_SET,
    #[doc = "0xa498 - Miscellaneous Register"]
    pub misc_root73_clr: MISC_ROOT73_CLR,
    #[doc = "0xa49c - Miscellaneous Register"]
    pub misc_root73_tog: MISC_ROOT73_TOG,
    #[doc = "0xa4a0 - Post Divider Register"]
    pub post73: POST73,
    #[doc = "0xa4a4 - Post Divider Register"]
    pub post_root73_set: POST_ROOT73_SET,
    #[doc = "0xa4a8 - Post Divider Register"]
    pub post_root73_clr: POST_ROOT73_CLR,
    #[doc = "0xa4ac - Post Divider Register"]
    pub post_root73_tog: POST_ROOT73_TOG,
    #[doc = "0xa4b0 - Pre Divider Register"]
    pub pre73: PRE73,
    #[doc = "0xa4b4 - Pre Divider Register"]
    pub pre_root73_set: PRE_ROOT73_SET,
    #[doc = "0xa4b8 - Pre Divider Register"]
    pub pre_root73_clr: PRE_ROOT73_CLR,
    #[doc = "0xa4bc - Pre Divider Register"]
    pub pre_root73_tog: PRE_ROOT73_TOG,
    _reserved41: [u8; 48usize],
    #[doc = "0xa4f0 - Access Control Register"]
    pub access_ctrl73: ACCESS_CTRL73,
    #[doc = "0xa4f4 - Access Control Register"]
    pub access_ctrl_root73_set: ACCESS_CTRL_ROOT73_SET,
    #[doc = "0xa4f8 - Access Control Register"]
    pub access_ctrl_root73_clr: ACCESS_CTRL_ROOT73_CLR,
    #[doc = "0xa4fc - Access Control Register"]
    pub access_ctrl_root73_tog: ACCESS_CTRL_ROOT73_TOG,
    #[doc = "0xa500 - Target Register"]
    pub target_root74: TARGET_ROOT74,
    #[doc = "0xa504 - Target Register"]
    pub target_root74_set: TARGET_ROOT74_SET,
    #[doc = "0xa508 - Target Register"]
    pub target_root74_clr: TARGET_ROOT74_CLR,
    #[doc = "0xa50c - Target Register"]
    pub target_root74_tog: TARGET_ROOT74_TOG,
    #[doc = "0xa510 - Miscellaneous Register"]
    pub misc74: MISC74,
    #[doc = "0xa514 - Miscellaneous Register"]
    pub misc_root74_set: MISC_ROOT74_SET,
    #[doc = "0xa518 - Miscellaneous Register"]
    pub misc_root74_clr: MISC_ROOT74_CLR,
    #[doc = "0xa51c - Miscellaneous Register"]
    pub misc_root74_tog: MISC_ROOT74_TOG,
    #[doc = "0xa520 - Post Divider Register"]
    pub post74: POST74,
    #[doc = "0xa524 - Post Divider Register"]
    pub post_root74_set: POST_ROOT74_SET,
    #[doc = "0xa528 - Post Divider Register"]
    pub post_root74_clr: POST_ROOT74_CLR,
    #[doc = "0xa52c - Post Divider Register"]
    pub post_root74_tog: POST_ROOT74_TOG,
    #[doc = "0xa530 - Pre Divider Register"]
    pub pre74: PRE74,
    #[doc = "0xa534 - Pre Divider Register"]
    pub pre_root74_set: PRE_ROOT74_SET,
    #[doc = "0xa538 - Pre Divider Register"]
    pub pre_root74_clr: PRE_ROOT74_CLR,
    #[doc = "0xa53c - Pre Divider Register"]
    pub pre_root74_tog: PRE_ROOT74_TOG,
    _reserved42: [u8; 48usize],
    #[doc = "0xa570 - Access Control Register"]
    pub access_ctrl74: ACCESS_CTRL74,
    #[doc = "0xa574 - Access Control Register"]
    pub access_ctrl_root74_set: ACCESS_CTRL_ROOT74_SET,
    #[doc = "0xa578 - Access Control Register"]
    pub access_ctrl_root74_clr: ACCESS_CTRL_ROOT74_CLR,
    #[doc = "0xa57c - Access Control Register"]
    pub access_ctrl_root74_tog: ACCESS_CTRL_ROOT74_TOG,
    #[doc = "0xa580 - Target Register"]
    pub target_root75: TARGET_ROOT75,
    #[doc = "0xa584 - Target Register"]
    pub target_root75_set: TARGET_ROOT75_SET,
    #[doc = "0xa588 - Target Register"]
    pub target_root75_clr: TARGET_ROOT75_CLR,
    #[doc = "0xa58c - Target Register"]
    pub target_root75_tog: TARGET_ROOT75_TOG,
    #[doc = "0xa590 - Miscellaneous Register"]
    pub misc75: MISC75,
    #[doc = "0xa594 - Miscellaneous Register"]
    pub misc_root75_set: MISC_ROOT75_SET,
    #[doc = "0xa598 - Miscellaneous Register"]
    pub misc_root75_clr: MISC_ROOT75_CLR,
    #[doc = "0xa59c - Miscellaneous Register"]
    pub misc_root75_tog: MISC_ROOT75_TOG,
    #[doc = "0xa5a0 - Post Divider Register"]
    pub post75: POST75,
    #[doc = "0xa5a4 - Post Divider Register"]
    pub post_root75_set: POST_ROOT75_SET,
    #[doc = "0xa5a8 - Post Divider Register"]
    pub post_root75_clr: POST_ROOT75_CLR,
    #[doc = "0xa5ac - Post Divider Register"]
    pub post_root75_tog: POST_ROOT75_TOG,
    #[doc = "0xa5b0 - Pre Divider Register"]
    pub pre75: PRE75,
    #[doc = "0xa5b4 - Pre Divider Register"]
    pub pre_root75_set: PRE_ROOT75_SET,
    #[doc = "0xa5b8 - Pre Divider Register"]
    pub pre_root75_clr: PRE_ROOT75_CLR,
    #[doc = "0xa5bc - Pre Divider Register"]
    pub pre_root75_tog: PRE_ROOT75_TOG,
    _reserved43: [u8; 48usize],
    #[doc = "0xa5f0 - Access Control Register"]
    pub access_ctrl75: ACCESS_CTRL75,
    #[doc = "0xa5f4 - Access Control Register"]
    pub access_ctrl_root75_set: ACCESS_CTRL_ROOT75_SET,
    #[doc = "0xa5f8 - Access Control Register"]
    pub access_ctrl_root75_clr: ACCESS_CTRL_ROOT75_CLR,
    #[doc = "0xa5fc - Access Control Register"]
    pub access_ctrl_root75_tog: ACCESS_CTRL_ROOT75_TOG,
    #[doc = "0xa600 - Target Register"]
    pub target_root76: TARGET_ROOT76,
    #[doc = "0xa604 - Target Register"]
    pub target_root76_set: TARGET_ROOT76_SET,
    #[doc = "0xa608 - Target Register"]
    pub target_root76_clr: TARGET_ROOT76_CLR,
    #[doc = "0xa60c - Target Register"]
    pub target_root76_tog: TARGET_ROOT76_TOG,
    #[doc = "0xa610 - Miscellaneous Register"]
    pub misc76: MISC76,
    #[doc = "0xa614 - Miscellaneous Register"]
    pub misc_root76_set: MISC_ROOT76_SET,
    #[doc = "0xa618 - Miscellaneous Register"]
    pub misc_root76_clr: MISC_ROOT76_CLR,
    #[doc = "0xa61c - Miscellaneous Register"]
    pub misc_root76_tog: MISC_ROOT76_TOG,
    #[doc = "0xa620 - Post Divider Register"]
    pub post76: POST76,
    #[doc = "0xa624 - Post Divider Register"]
    pub post_root76_set: POST_ROOT76_SET,
    #[doc = "0xa628 - Post Divider Register"]
    pub post_root76_clr: POST_ROOT76_CLR,
    #[doc = "0xa62c - Post Divider Register"]
    pub post_root76_tog: POST_ROOT76_TOG,
    #[doc = "0xa630 - Pre Divider Register"]
    pub pre76: PRE76,
    #[doc = "0xa634 - Pre Divider Register"]
    pub pre_root76_set: PRE_ROOT76_SET,
    #[doc = "0xa638 - Pre Divider Register"]
    pub pre_root76_clr: PRE_ROOT76_CLR,
    #[doc = "0xa63c - Pre Divider Register"]
    pub pre_root76_tog: PRE_ROOT76_TOG,
    _reserved44: [u8; 48usize],
    #[doc = "0xa670 - Access Control Register"]
    pub access_ctrl76: ACCESS_CTRL76,
    #[doc = "0xa674 - Access Control Register"]
    pub access_ctrl_root76_set: ACCESS_CTRL_ROOT76_SET,
    #[doc = "0xa678 - Access Control Register"]
    pub access_ctrl_root76_clr: ACCESS_CTRL_ROOT76_CLR,
    #[doc = "0xa67c - Access Control Register"]
    pub access_ctrl_root76_tog: ACCESS_CTRL_ROOT76_TOG,
    #[doc = "0xa680 - Target Register"]
    pub target_root77: TARGET_ROOT77,
    #[doc = "0xa684 - Target Register"]
    pub target_root77_set: TARGET_ROOT77_SET,
    #[doc = "0xa688 - Target Register"]
    pub target_root77_clr: TARGET_ROOT77_CLR,
    #[doc = "0xa68c - Target Register"]
    pub target_root77_tog: TARGET_ROOT77_TOG,
    #[doc = "0xa690 - Miscellaneous Register"]
    pub misc77: MISC77,
    #[doc = "0xa694 - Miscellaneous Register"]
    pub misc_root77_set: MISC_ROOT77_SET,
    #[doc = "0xa698 - Miscellaneous Register"]
    pub misc_root77_clr: MISC_ROOT77_CLR,
    #[doc = "0xa69c - Miscellaneous Register"]
    pub misc_root77_tog: MISC_ROOT77_TOG,
    #[doc = "0xa6a0 - Post Divider Register"]
    pub post77: POST77,
    #[doc = "0xa6a4 - Post Divider Register"]
    pub post_root77_set: POST_ROOT77_SET,
    #[doc = "0xa6a8 - Post Divider Register"]
    pub post_root77_clr: POST_ROOT77_CLR,
    #[doc = "0xa6ac - Post Divider Register"]
    pub post_root77_tog: POST_ROOT77_TOG,
    #[doc = "0xa6b0 - Pre Divider Register"]
    pub pre77: PRE77,
    #[doc = "0xa6b4 - Pre Divider Register"]
    pub pre_root77_set: PRE_ROOT77_SET,
    #[doc = "0xa6b8 - Pre Divider Register"]
    pub pre_root77_clr: PRE_ROOT77_CLR,
    #[doc = "0xa6bc - Pre Divider Register"]
    pub pre_root77_tog: PRE_ROOT77_TOG,
    _reserved45: [u8; 48usize],
    #[doc = "0xa6f0 - Access Control Register"]
    pub access_ctrl77: ACCESS_CTRL77,
    #[doc = "0xa6f4 - Access Control Register"]
    pub access_ctrl_root77_set: ACCESS_CTRL_ROOT77_SET,
    #[doc = "0xa6f8 - Access Control Register"]
    pub access_ctrl_root77_clr: ACCESS_CTRL_ROOT77_CLR,
    #[doc = "0xa6fc - Access Control Register"]
    pub access_ctrl_root77_tog: ACCESS_CTRL_ROOT77_TOG,
    #[doc = "0xa700 - Target Register"]
    pub target_root78: TARGET_ROOT78,
    #[doc = "0xa704 - Target Register"]
    pub target_root78_set: TARGET_ROOT78_SET,
    #[doc = "0xa708 - Target Register"]
    pub target_root78_clr: TARGET_ROOT78_CLR,
    #[doc = "0xa70c - Target Register"]
    pub target_root78_tog: TARGET_ROOT78_TOG,
    #[doc = "0xa710 - Miscellaneous Register"]
    pub misc78: MISC78,
    #[doc = "0xa714 - Miscellaneous Register"]
    pub misc_root78_set: MISC_ROOT78_SET,
    #[doc = "0xa718 - Miscellaneous Register"]
    pub misc_root78_clr: MISC_ROOT78_CLR,
    #[doc = "0xa71c - Miscellaneous Register"]
    pub misc_root78_tog: MISC_ROOT78_TOG,
    #[doc = "0xa720 - Post Divider Register"]
    pub post78: POST78,
    #[doc = "0xa724 - Post Divider Register"]
    pub post_root78_set: POST_ROOT78_SET,
    #[doc = "0xa728 - Post Divider Register"]
    pub post_root78_clr: POST_ROOT78_CLR,
    #[doc = "0xa72c - Post Divider Register"]
    pub post_root78_tog: POST_ROOT78_TOG,
    #[doc = "0xa730 - Pre Divider Register"]
    pub pre78: PRE78,
    #[doc = "0xa734 - Pre Divider Register"]
    pub pre_root78_set: PRE_ROOT78_SET,
    #[doc = "0xa738 - Pre Divider Register"]
    pub pre_root78_clr: PRE_ROOT78_CLR,
    #[doc = "0xa73c - Pre Divider Register"]
    pub pre_root78_tog: PRE_ROOT78_TOG,
    _reserved46: [u8; 48usize],
    #[doc = "0xa770 - Access Control Register"]
    pub access_ctrl78: ACCESS_CTRL78,
    #[doc = "0xa774 - Access Control Register"]
    pub access_ctrl_root78_set: ACCESS_CTRL_ROOT78_SET,
    #[doc = "0xa778 - Access Control Register"]
    pub access_ctrl_root78_clr: ACCESS_CTRL_ROOT78_CLR,
    #[doc = "0xa77c - Access Control Register"]
    pub access_ctrl_root78_tog: ACCESS_CTRL_ROOT78_TOG,
    #[doc = "0xa780 - Target Register"]
    pub target_root79: TARGET_ROOT79,
    #[doc = "0xa784 - Target Register"]
    pub target_root79_set: TARGET_ROOT79_SET,
    #[doc = "0xa788 - Target Register"]
    pub target_root79_clr: TARGET_ROOT79_CLR,
    #[doc = "0xa78c - Target Register"]
    pub target_root79_tog: TARGET_ROOT79_TOG,
    #[doc = "0xa790 - Miscellaneous Register"]
    pub misc79: MISC79,
    #[doc = "0xa794 - Miscellaneous Register"]
    pub misc_root79_set: MISC_ROOT79_SET,
    #[doc = "0xa798 - Miscellaneous Register"]
    pub misc_root79_clr: MISC_ROOT79_CLR,
    #[doc = "0xa79c - Miscellaneous Register"]
    pub misc_root79_tog: MISC_ROOT79_TOG,
    #[doc = "0xa7a0 - Post Divider Register"]
    pub post79: POST79,
    #[doc = "0xa7a4 - Post Divider Register"]
    pub post_root79_set: POST_ROOT79_SET,
    #[doc = "0xa7a8 - Post Divider Register"]
    pub post_root79_clr: POST_ROOT79_CLR,
    #[doc = "0xa7ac - Post Divider Register"]
    pub post_root79_tog: POST_ROOT79_TOG,
    #[doc = "0xa7b0 - Pre Divider Register"]
    pub pre79: PRE79,
    #[doc = "0xa7b4 - Pre Divider Register"]
    pub pre_root79_set: PRE_ROOT79_SET,
    #[doc = "0xa7b8 - Pre Divider Register"]
    pub pre_root79_clr: PRE_ROOT79_CLR,
    #[doc = "0xa7bc - Pre Divider Register"]
    pub pre_root79_tog: PRE_ROOT79_TOG,
    _reserved47: [u8; 48usize],
    #[doc = "0xa7f0 - Access Control Register"]
    pub access_ctrl79: ACCESS_CTRL79,
    #[doc = "0xa7f4 - Access Control Register"]
    pub access_ctrl_root79_set: ACCESS_CTRL_ROOT79_SET,
    #[doc = "0xa7f8 - Access Control Register"]
    pub access_ctrl_root79_clr: ACCESS_CTRL_ROOT79_CLR,
    #[doc = "0xa7fc - Access Control Register"]
    pub access_ctrl_root79_tog: ACCESS_CTRL_ROOT79_TOG,
    #[doc = "0xa800 - Target Register"]
    pub target_root80: TARGET_ROOT80,
    #[doc = "0xa804 - Target Register"]
    pub target_root80_set: TARGET_ROOT80_SET,
    #[doc = "0xa808 - Target Register"]
    pub target_root80_clr: TARGET_ROOT80_CLR,
    #[doc = "0xa80c - Target Register"]
    pub target_root80_tog: TARGET_ROOT80_TOG,
    #[doc = "0xa810 - Miscellaneous Register"]
    pub misc80: MISC80,
    #[doc = "0xa814 - Miscellaneous Register"]
    pub misc_root80_set: MISC_ROOT80_SET,
    #[doc = "0xa818 - Miscellaneous Register"]
    pub misc_root80_clr: MISC_ROOT80_CLR,
    #[doc = "0xa81c - Miscellaneous Register"]
    pub misc_root80_tog: MISC_ROOT80_TOG,
    #[doc = "0xa820 - Post Divider Register"]
    pub post80: POST80,
    #[doc = "0xa824 - Post Divider Register"]
    pub post_root80_set: POST_ROOT80_SET,
    #[doc = "0xa828 - Post Divider Register"]
    pub post_root80_clr: POST_ROOT80_CLR,
    #[doc = "0xa82c - Post Divider Register"]
    pub post_root80_tog: POST_ROOT80_TOG,
    #[doc = "0xa830 - Pre Divider Register"]
    pub pre80: PRE80,
    #[doc = "0xa834 - Pre Divider Register"]
    pub pre_root80_set: PRE_ROOT80_SET,
    #[doc = "0xa838 - Pre Divider Register"]
    pub pre_root80_clr: PRE_ROOT80_CLR,
    #[doc = "0xa83c - Pre Divider Register"]
    pub pre_root80_tog: PRE_ROOT80_TOG,
    _reserved48: [u8; 48usize],
    #[doc = "0xa870 - Access Control Register"]
    pub access_ctrl80: ACCESS_CTRL80,
    #[doc = "0xa874 - Access Control Register"]
    pub access_ctrl_root80_set: ACCESS_CTRL_ROOT80_SET,
    #[doc = "0xa878 - Access Control Register"]
    pub access_ctrl_root80_clr: ACCESS_CTRL_ROOT80_CLR,
    #[doc = "0xa87c - Access Control Register"]
    pub access_ctrl_root80_tog: ACCESS_CTRL_ROOT80_TOG,
    #[doc = "0xa880 - Target Register"]
    pub target_root81: TARGET_ROOT81,
    #[doc = "0xa884 - Target Register"]
    pub target_root81_set: TARGET_ROOT81_SET,
    #[doc = "0xa888 - Target Register"]
    pub target_root81_clr: TARGET_ROOT81_CLR,
    #[doc = "0xa88c - Target Register"]
    pub target_root81_tog: TARGET_ROOT81_TOG,
    #[doc = "0xa890 - Miscellaneous Register"]
    pub misc81: MISC81,
    #[doc = "0xa894 - Miscellaneous Register"]
    pub misc_root81_set: MISC_ROOT81_SET,
    #[doc = "0xa898 - Miscellaneous Register"]
    pub misc_root81_clr: MISC_ROOT81_CLR,
    #[doc = "0xa89c - Miscellaneous Register"]
    pub misc_root81_tog: MISC_ROOT81_TOG,
    #[doc = "0xa8a0 - Post Divider Register"]
    pub post81: POST81,
    #[doc = "0xa8a4 - Post Divider Register"]
    pub post_root81_set: POST_ROOT81_SET,
    #[doc = "0xa8a8 - Post Divider Register"]
    pub post_root81_clr: POST_ROOT81_CLR,
    #[doc = "0xa8ac - Post Divider Register"]
    pub post_root81_tog: POST_ROOT81_TOG,
    #[doc = "0xa8b0 - Pre Divider Register"]
    pub pre81: PRE81,
    #[doc = "0xa8b4 - Pre Divider Register"]
    pub pre_root81_set: PRE_ROOT81_SET,
    #[doc = "0xa8b8 - Pre Divider Register"]
    pub pre_root81_clr: PRE_ROOT81_CLR,
    #[doc = "0xa8bc - Pre Divider Register"]
    pub pre_root81_tog: PRE_ROOT81_TOG,
    _reserved49: [u8; 48usize],
    #[doc = "0xa8f0 - Access Control Register"]
    pub access_ctrl81: ACCESS_CTRL81,
    #[doc = "0xa8f4 - Access Control Register"]
    pub access_ctrl_root81_set: ACCESS_CTRL_ROOT81_SET,
    #[doc = "0xa8f8 - Access Control Register"]
    pub access_ctrl_root81_clr: ACCESS_CTRL_ROOT81_CLR,
    #[doc = "0xa8fc - Access Control Register"]
    pub access_ctrl_root81_tog: ACCESS_CTRL_ROOT81_TOG,
    #[doc = "0xa900 - Target Register"]
    pub target_root82: TARGET_ROOT82,
    #[doc = "0xa904 - Target Register"]
    pub target_root82_set: TARGET_ROOT82_SET,
    #[doc = "0xa908 - Target Register"]
    pub target_root82_clr: TARGET_ROOT82_CLR,
    #[doc = "0xa90c - Target Register"]
    pub target_root82_tog: TARGET_ROOT82_TOG,
    #[doc = "0xa910 - Miscellaneous Register"]
    pub misc82: MISC82,
    #[doc = "0xa914 - Miscellaneous Register"]
    pub misc_root82_set: MISC_ROOT82_SET,
    #[doc = "0xa918 - Miscellaneous Register"]
    pub misc_root82_clr: MISC_ROOT82_CLR,
    #[doc = "0xa91c - Miscellaneous Register"]
    pub misc_root82_tog: MISC_ROOT82_TOG,
    #[doc = "0xa920 - Post Divider Register"]
    pub post82: POST82,
    #[doc = "0xa924 - Post Divider Register"]
    pub post_root82_set: POST_ROOT82_SET,
    #[doc = "0xa928 - Post Divider Register"]
    pub post_root82_clr: POST_ROOT82_CLR,
    #[doc = "0xa92c - Post Divider Register"]
    pub post_root82_tog: POST_ROOT82_TOG,
    #[doc = "0xa930 - Pre Divider Register"]
    pub pre82: PRE82,
    #[doc = "0xa934 - Pre Divider Register"]
    pub pre_root82_set: PRE_ROOT82_SET,
    #[doc = "0xa938 - Pre Divider Register"]
    pub pre_root82_clr: PRE_ROOT82_CLR,
    #[doc = "0xa93c - Pre Divider Register"]
    pub pre_root82_tog: PRE_ROOT82_TOG,
    _reserved50: [u8; 48usize],
    #[doc = "0xa970 - Access Control Register"]
    pub access_ctrl82: ACCESS_CTRL82,
    #[doc = "0xa974 - Access Control Register"]
    pub access_ctrl_root82_set: ACCESS_CTRL_ROOT82_SET,
    #[doc = "0xa978 - Access Control Register"]
    pub access_ctrl_root82_clr: ACCESS_CTRL_ROOT82_CLR,
    #[doc = "0xa97c - Access Control Register"]
    pub access_ctrl_root82_tog: ACCESS_CTRL_ROOT82_TOG,
    #[doc = "0xa980 - Target Register"]
    pub target_root83: TARGET_ROOT83,
    #[doc = "0xa984 - Target Register"]
    pub target_root83_set: TARGET_ROOT83_SET,
    #[doc = "0xa988 - Target Register"]
    pub target_root83_clr: TARGET_ROOT83_CLR,
    #[doc = "0xa98c - Target Register"]
    pub target_root83_tog: TARGET_ROOT83_TOG,
    #[doc = "0xa990 - Miscellaneous Register"]
    pub misc83: MISC83,
    #[doc = "0xa994 - Miscellaneous Register"]
    pub misc_root83_set: MISC_ROOT83_SET,
    #[doc = "0xa998 - Miscellaneous Register"]
    pub misc_root83_clr: MISC_ROOT83_CLR,
    #[doc = "0xa99c - Miscellaneous Register"]
    pub misc_root83_tog: MISC_ROOT83_TOG,
    #[doc = "0xa9a0 - Post Divider Register"]
    pub post83: POST83,
    #[doc = "0xa9a4 - Post Divider Register"]
    pub post_root83_set: POST_ROOT83_SET,
    #[doc = "0xa9a8 - Post Divider Register"]
    pub post_root83_clr: POST_ROOT83_CLR,
    #[doc = "0xa9ac - Post Divider Register"]
    pub post_root83_tog: POST_ROOT83_TOG,
    #[doc = "0xa9b0 - Pre Divider Register"]
    pub pre83: PRE83,
    #[doc = "0xa9b4 - Pre Divider Register"]
    pub pre_root83_set: PRE_ROOT83_SET,
    #[doc = "0xa9b8 - Pre Divider Register"]
    pub pre_root83_clr: PRE_ROOT83_CLR,
    #[doc = "0xa9bc - Pre Divider Register"]
    pub pre_root83_tog: PRE_ROOT83_TOG,
    _reserved51: [u8; 48usize],
    #[doc = "0xa9f0 - Access Control Register"]
    pub access_ctrl83: ACCESS_CTRL83,
    #[doc = "0xa9f4 - Access Control Register"]
    pub access_ctrl_root83_set: ACCESS_CTRL_ROOT83_SET,
    #[doc = "0xa9f8 - Access Control Register"]
    pub access_ctrl_root83_clr: ACCESS_CTRL_ROOT83_CLR,
    #[doc = "0xa9fc - Access Control Register"]
    pub access_ctrl_root83_tog: ACCESS_CTRL_ROOT83_TOG,
    #[doc = "0xaa00 - Target Register"]
    pub target_root84: TARGET_ROOT84,
    #[doc = "0xaa04 - Target Register"]
    pub target_root84_set: TARGET_ROOT84_SET,
    #[doc = "0xaa08 - Target Register"]
    pub target_root84_clr: TARGET_ROOT84_CLR,
    #[doc = "0xaa0c - Target Register"]
    pub target_root84_tog: TARGET_ROOT84_TOG,
    #[doc = "0xaa10 - Miscellaneous Register"]
    pub misc84: MISC84,
    #[doc = "0xaa14 - Miscellaneous Register"]
    pub misc_root84_set: MISC_ROOT84_SET,
    #[doc = "0xaa18 - Miscellaneous Register"]
    pub misc_root84_clr: MISC_ROOT84_CLR,
    #[doc = "0xaa1c - Miscellaneous Register"]
    pub misc_root84_tog: MISC_ROOT84_TOG,
    #[doc = "0xaa20 - Post Divider Register"]
    pub post84: POST84,
    #[doc = "0xaa24 - Post Divider Register"]
    pub post_root84_set: POST_ROOT84_SET,
    #[doc = "0xaa28 - Post Divider Register"]
    pub post_root84_clr: POST_ROOT84_CLR,
    #[doc = "0xaa2c - Post Divider Register"]
    pub post_root84_tog: POST_ROOT84_TOG,
    #[doc = "0xaa30 - Pre Divider Register"]
    pub pre84: PRE84,
    #[doc = "0xaa34 - Pre Divider Register"]
    pub pre_root84_set: PRE_ROOT84_SET,
    #[doc = "0xaa38 - Pre Divider Register"]
    pub pre_root84_clr: PRE_ROOT84_CLR,
    #[doc = "0xaa3c - Pre Divider Register"]
    pub pre_root84_tog: PRE_ROOT84_TOG,
    _reserved52: [u8; 48usize],
    #[doc = "0xaa70 - Access Control Register"]
    pub access_ctrl84: ACCESS_CTRL84,
    #[doc = "0xaa74 - Access Control Register"]
    pub access_ctrl_root84_set: ACCESS_CTRL_ROOT84_SET,
    #[doc = "0xaa78 - Access Control Register"]
    pub access_ctrl_root84_clr: ACCESS_CTRL_ROOT84_CLR,
    #[doc = "0xaa7c - Access Control Register"]
    pub access_ctrl_root84_tog: ACCESS_CTRL_ROOT84_TOG,
    #[doc = "0xaa80 - Target Register"]
    pub target_root85: TARGET_ROOT85,
    #[doc = "0xaa84 - Target Register"]
    pub target_root85_set: TARGET_ROOT85_SET,
    #[doc = "0xaa88 - Target Register"]
    pub target_root85_clr: TARGET_ROOT85_CLR,
    #[doc = "0xaa8c - Target Register"]
    pub target_root85_tog: TARGET_ROOT85_TOG,
    #[doc = "0xaa90 - Miscellaneous Register"]
    pub misc85: MISC85,
    #[doc = "0xaa94 - Miscellaneous Register"]
    pub misc_root85_set: MISC_ROOT85_SET,
    #[doc = "0xaa98 - Miscellaneous Register"]
    pub misc_root85_clr: MISC_ROOT85_CLR,
    #[doc = "0xaa9c - Miscellaneous Register"]
    pub misc_root85_tog: MISC_ROOT85_TOG,
    #[doc = "0xaaa0 - Post Divider Register"]
    pub post85: POST85,
    #[doc = "0xaaa4 - Post Divider Register"]
    pub post_root85_set: POST_ROOT85_SET,
    #[doc = "0xaaa8 - Post Divider Register"]
    pub post_root85_clr: POST_ROOT85_CLR,
    #[doc = "0xaaac - Post Divider Register"]
    pub post_root85_tog: POST_ROOT85_TOG,
    #[doc = "0xaab0 - Pre Divider Register"]
    pub pre85: PRE85,
    #[doc = "0xaab4 - Pre Divider Register"]
    pub pre_root85_set: PRE_ROOT85_SET,
    #[doc = "0xaab8 - Pre Divider Register"]
    pub pre_root85_clr: PRE_ROOT85_CLR,
    #[doc = "0xaabc - Pre Divider Register"]
    pub pre_root85_tog: PRE_ROOT85_TOG,
    _reserved53: [u8; 48usize],
    #[doc = "0xaaf0 - Access Control Register"]
    pub access_ctrl85: ACCESS_CTRL85,
    #[doc = "0xaaf4 - Access Control Register"]
    pub access_ctrl_root85_set: ACCESS_CTRL_ROOT85_SET,
    #[doc = "0xaaf8 - Access Control Register"]
    pub access_ctrl_root85_clr: ACCESS_CTRL_ROOT85_CLR,
    #[doc = "0xaafc - Access Control Register"]
    pub access_ctrl_root85_tog: ACCESS_CTRL_ROOT85_TOG,
    #[doc = "0xab00 - Target Register"]
    pub target_root86: TARGET_ROOT86,
    #[doc = "0xab04 - Target Register"]
    pub target_root86_set: TARGET_ROOT86_SET,
    #[doc = "0xab08 - Target Register"]
    pub target_root86_clr: TARGET_ROOT86_CLR,
    #[doc = "0xab0c - Target Register"]
    pub target_root86_tog: TARGET_ROOT86_TOG,
    #[doc = "0xab10 - Miscellaneous Register"]
    pub misc86: MISC86,
    #[doc = "0xab14 - Miscellaneous Register"]
    pub misc_root86_set: MISC_ROOT86_SET,
    #[doc = "0xab18 - Miscellaneous Register"]
    pub misc_root86_clr: MISC_ROOT86_CLR,
    #[doc = "0xab1c - Miscellaneous Register"]
    pub misc_root86_tog: MISC_ROOT86_TOG,
    #[doc = "0xab20 - Post Divider Register"]
    pub post86: POST86,
    #[doc = "0xab24 - Post Divider Register"]
    pub post_root86_set: POST_ROOT86_SET,
    #[doc = "0xab28 - Post Divider Register"]
    pub post_root86_clr: POST_ROOT86_CLR,
    #[doc = "0xab2c - Post Divider Register"]
    pub post_root86_tog: POST_ROOT86_TOG,
    #[doc = "0xab30 - Pre Divider Register"]
    pub pre86: PRE86,
    #[doc = "0xab34 - Pre Divider Register"]
    pub pre_root86_set: PRE_ROOT86_SET,
    #[doc = "0xab38 - Pre Divider Register"]
    pub pre_root86_clr: PRE_ROOT86_CLR,
    #[doc = "0xab3c - Pre Divider Register"]
    pub pre_root86_tog: PRE_ROOT86_TOG,
    _reserved54: [u8; 48usize],
    #[doc = "0xab70 - Access Control Register"]
    pub access_ctrl86: ACCESS_CTRL86,
    #[doc = "0xab74 - Access Control Register"]
    pub access_ctrl_root86_set: ACCESS_CTRL_ROOT86_SET,
    #[doc = "0xab78 - Access Control Register"]
    pub access_ctrl_root86_clr: ACCESS_CTRL_ROOT86_CLR,
    #[doc = "0xab7c - Access Control Register"]
    pub access_ctrl_root86_tog: ACCESS_CTRL_ROOT86_TOG,
    #[doc = "0xab80 - Target Register"]
    pub target_root87: TARGET_ROOT87,
    #[doc = "0xab84 - Target Register"]
    pub target_root87_set: TARGET_ROOT87_SET,
    #[doc = "0xab88 - Target Register"]
    pub target_root87_clr: TARGET_ROOT87_CLR,
    #[doc = "0xab8c - Target Register"]
    pub target_root87_tog: TARGET_ROOT87_TOG,
    #[doc = "0xab90 - Miscellaneous Register"]
    pub misc87: MISC87,
    #[doc = "0xab94 - Miscellaneous Register"]
    pub misc_root87_set: MISC_ROOT87_SET,
    #[doc = "0xab98 - Miscellaneous Register"]
    pub misc_root87_clr: MISC_ROOT87_CLR,
    #[doc = "0xab9c - Miscellaneous Register"]
    pub misc_root87_tog: MISC_ROOT87_TOG,
    #[doc = "0xaba0 - Post Divider Register"]
    pub post87: POST87,
    #[doc = "0xaba4 - Post Divider Register"]
    pub post_root87_set: POST_ROOT87_SET,
    #[doc = "0xaba8 - Post Divider Register"]
    pub post_root87_clr: POST_ROOT87_CLR,
    #[doc = "0xabac - Post Divider Register"]
    pub post_root87_tog: POST_ROOT87_TOG,
    #[doc = "0xabb0 - Pre Divider Register"]
    pub pre87: PRE87,
    #[doc = "0xabb4 - Pre Divider Register"]
    pub pre_root87_set: PRE_ROOT87_SET,
    #[doc = "0xabb8 - Pre Divider Register"]
    pub pre_root87_clr: PRE_ROOT87_CLR,
    #[doc = "0xabbc - Pre Divider Register"]
    pub pre_root87_tog: PRE_ROOT87_TOG,
    _reserved55: [u8; 48usize],
    #[doc = "0xabf0 - Access Control Register"]
    pub access_ctrl87: ACCESS_CTRL87,
    #[doc = "0xabf4 - Access Control Register"]
    pub access_ctrl_root87_set: ACCESS_CTRL_ROOT87_SET,
    #[doc = "0xabf8 - Access Control Register"]
    pub access_ctrl_root87_clr: ACCESS_CTRL_ROOT87_CLR,
    #[doc = "0xabfc - Access Control Register"]
    pub access_ctrl_root87_tog: ACCESS_CTRL_ROOT87_TOG,
    #[doc = "0xac00 - Target Register"]
    pub target_root88: TARGET_ROOT88,
    #[doc = "0xac04 - Target Register"]
    pub target_root88_set: TARGET_ROOT88_SET,
    #[doc = "0xac08 - Target Register"]
    pub target_root88_clr: TARGET_ROOT88_CLR,
    #[doc = "0xac0c - Target Register"]
    pub target_root88_tog: TARGET_ROOT88_TOG,
    #[doc = "0xac10 - Miscellaneous Register"]
    pub misc88: MISC88,
    #[doc = "0xac14 - Miscellaneous Register"]
    pub misc_root88_set: MISC_ROOT88_SET,
    #[doc = "0xac18 - Miscellaneous Register"]
    pub misc_root88_clr: MISC_ROOT88_CLR,
    #[doc = "0xac1c - Miscellaneous Register"]
    pub misc_root88_tog: MISC_ROOT88_TOG,
    #[doc = "0xac20 - Post Divider Register"]
    pub post88: POST88,
    #[doc = "0xac24 - Post Divider Register"]
    pub post_root88_set: POST_ROOT88_SET,
    #[doc = "0xac28 - Post Divider Register"]
    pub post_root88_clr: POST_ROOT88_CLR,
    #[doc = "0xac2c - Post Divider Register"]
    pub post_root88_tog: POST_ROOT88_TOG,
    #[doc = "0xac30 - Pre Divider Register"]
    pub pre88: PRE88,
    #[doc = "0xac34 - Pre Divider Register"]
    pub pre_root88_set: PRE_ROOT88_SET,
    #[doc = "0xac38 - Pre Divider Register"]
    pub pre_root88_clr: PRE_ROOT88_CLR,
    #[doc = "0xac3c - Pre Divider Register"]
    pub pre_root88_tog: PRE_ROOT88_TOG,
    _reserved56: [u8; 48usize],
    #[doc = "0xac70 - Access Control Register"]
    pub access_ctrl88: ACCESS_CTRL88,
    #[doc = "0xac74 - Access Control Register"]
    pub access_ctrl_root88_set: ACCESS_CTRL_ROOT88_SET,
    #[doc = "0xac78 - Access Control Register"]
    pub access_ctrl_root88_clr: ACCESS_CTRL_ROOT88_CLR,
    #[doc = "0xac7c - Access Control Register"]
    pub access_ctrl_root88_tog: ACCESS_CTRL_ROOT88_TOG,
    #[doc = "0xac80 - Target Register"]
    pub target_root89: TARGET_ROOT89,
    #[doc = "0xac84 - Target Register"]
    pub target_root89_set: TARGET_ROOT89_SET,
    #[doc = "0xac88 - Target Register"]
    pub target_root89_clr: TARGET_ROOT89_CLR,
    #[doc = "0xac8c - Target Register"]
    pub target_root89_tog: TARGET_ROOT89_TOG,
    #[doc = "0xac90 - Miscellaneous Register"]
    pub misc89: MISC89,
    #[doc = "0xac94 - Miscellaneous Register"]
    pub misc_root89_set: MISC_ROOT89_SET,
    #[doc = "0xac98 - Miscellaneous Register"]
    pub misc_root89_clr: MISC_ROOT89_CLR,
    #[doc = "0xac9c - Miscellaneous Register"]
    pub misc_root89_tog: MISC_ROOT89_TOG,
    #[doc = "0xaca0 - Post Divider Register"]
    pub post89: POST89,
    #[doc = "0xaca4 - Post Divider Register"]
    pub post_root89_set: POST_ROOT89_SET,
    #[doc = "0xaca8 - Post Divider Register"]
    pub post_root89_clr: POST_ROOT89_CLR,
    #[doc = "0xacac - Post Divider Register"]
    pub post_root89_tog: POST_ROOT89_TOG,
    #[doc = "0xacb0 - Pre Divider Register"]
    pub pre89: PRE89,
    #[doc = "0xacb4 - Pre Divider Register"]
    pub pre_root89_set: PRE_ROOT89_SET,
    #[doc = "0xacb8 - Pre Divider Register"]
    pub pre_root89_clr: PRE_ROOT89_CLR,
    #[doc = "0xacbc - Pre Divider Register"]
    pub pre_root89_tog: PRE_ROOT89_TOG,
    _reserved57: [u8; 48usize],
    #[doc = "0xacf0 - Access Control Register"]
    pub access_ctrl89: ACCESS_CTRL89,
    #[doc = "0xacf4 - Access Control Register"]
    pub access_ctrl_root89_set: ACCESS_CTRL_ROOT89_SET,
    #[doc = "0xacf8 - Access Control Register"]
    pub access_ctrl_root89_clr: ACCESS_CTRL_ROOT89_CLR,
    #[doc = "0xacfc - Access Control Register"]
    pub access_ctrl_root89_tog: ACCESS_CTRL_ROOT89_TOG,
    #[doc = "0xad00 - Target Register"]
    pub target_root90: TARGET_ROOT90,
    #[doc = "0xad04 - Target Register"]
    pub target_root90_set: TARGET_ROOT90_SET,
    #[doc = "0xad08 - Target Register"]
    pub target_root90_clr: TARGET_ROOT90_CLR,
    #[doc = "0xad0c - Target Register"]
    pub target_root90_tog: TARGET_ROOT90_TOG,
    #[doc = "0xad10 - Miscellaneous Register"]
    pub misc90: MISC90,
    #[doc = "0xad14 - Miscellaneous Register"]
    pub misc_root90_set: MISC_ROOT90_SET,
    #[doc = "0xad18 - Miscellaneous Register"]
    pub misc_root90_clr: MISC_ROOT90_CLR,
    #[doc = "0xad1c - Miscellaneous Register"]
    pub misc_root90_tog: MISC_ROOT90_TOG,
    #[doc = "0xad20 - Post Divider Register"]
    pub post90: POST90,
    #[doc = "0xad24 - Post Divider Register"]
    pub post_root90_set: POST_ROOT90_SET,
    #[doc = "0xad28 - Post Divider Register"]
    pub post_root90_clr: POST_ROOT90_CLR,
    #[doc = "0xad2c - Post Divider Register"]
    pub post_root90_tog: POST_ROOT90_TOG,
    #[doc = "0xad30 - Pre Divider Register"]
    pub pre90: PRE90,
    #[doc = "0xad34 - Pre Divider Register"]
    pub pre_root90_set: PRE_ROOT90_SET,
    #[doc = "0xad38 - Pre Divider Register"]
    pub pre_root90_clr: PRE_ROOT90_CLR,
    #[doc = "0xad3c - Pre Divider Register"]
    pub pre_root90_tog: PRE_ROOT90_TOG,
    _reserved58: [u8; 48usize],
    #[doc = "0xad70 - Access Control Register"]
    pub access_ctrl90: ACCESS_CTRL90,
    #[doc = "0xad74 - Access Control Register"]
    pub access_ctrl_root90_set: ACCESS_CTRL_ROOT90_SET,
    #[doc = "0xad78 - Access Control Register"]
    pub access_ctrl_root90_clr: ACCESS_CTRL_ROOT90_CLR,
    #[doc = "0xad7c - Access Control Register"]
    pub access_ctrl_root90_tog: ACCESS_CTRL_ROOT90_TOG,
    #[doc = "0xad80 - Target Register"]
    pub target_root91: TARGET_ROOT91,
    #[doc = "0xad84 - Target Register"]
    pub target_root91_set: TARGET_ROOT91_SET,
    #[doc = "0xad88 - Target Register"]
    pub target_root91_clr: TARGET_ROOT91_CLR,
    #[doc = "0xad8c - Target Register"]
    pub target_root91_tog: TARGET_ROOT91_TOG,
    #[doc = "0xad90 - Miscellaneous Register"]
    pub misc91: MISC91,
    #[doc = "0xad94 - Miscellaneous Register"]
    pub misc_root91_set: MISC_ROOT91_SET,
    #[doc = "0xad98 - Miscellaneous Register"]
    pub misc_root91_clr: MISC_ROOT91_CLR,
    #[doc = "0xad9c - Miscellaneous Register"]
    pub misc_root91_tog: MISC_ROOT91_TOG,
    #[doc = "0xada0 - Post Divider Register"]
    pub post91: POST91,
    #[doc = "0xada4 - Post Divider Register"]
    pub post_root91_set: POST_ROOT91_SET,
    #[doc = "0xada8 - Post Divider Register"]
    pub post_root91_clr: POST_ROOT91_CLR,
    #[doc = "0xadac - Post Divider Register"]
    pub post_root91_tog: POST_ROOT91_TOG,
    #[doc = "0xadb0 - Pre Divider Register"]
    pub pre91: PRE91,
    #[doc = "0xadb4 - Pre Divider Register"]
    pub pre_root91_set: PRE_ROOT91_SET,
    #[doc = "0xadb8 - Pre Divider Register"]
    pub pre_root91_clr: PRE_ROOT91_CLR,
    #[doc = "0xadbc - Pre Divider Register"]
    pub pre_root91_tog: PRE_ROOT91_TOG,
    _reserved59: [u8; 48usize],
    #[doc = "0xadf0 - Access Control Register"]
    pub access_ctrl91: ACCESS_CTRL91,
    #[doc = "0xadf4 - Access Control Register"]
    pub access_ctrl_root91_set: ACCESS_CTRL_ROOT91_SET,
    #[doc = "0xadf8 - Access Control Register"]
    pub access_ctrl_root91_clr: ACCESS_CTRL_ROOT91_CLR,
    #[doc = "0xadfc - Access Control Register"]
    pub access_ctrl_root91_tog: ACCESS_CTRL_ROOT91_TOG,
    #[doc = "0xae00 - Target Register"]
    pub target_root92: TARGET_ROOT92,
    #[doc = "0xae04 - Target Register"]
    pub target_root92_set: TARGET_ROOT92_SET,
    #[doc = "0xae08 - Target Register"]
    pub target_root92_clr: TARGET_ROOT92_CLR,
    #[doc = "0xae0c - Target Register"]
    pub target_root92_tog: TARGET_ROOT92_TOG,
    #[doc = "0xae10 - Miscellaneous Register"]
    pub misc92: MISC92,
    #[doc = "0xae14 - Miscellaneous Register"]
    pub misc_root92_set: MISC_ROOT92_SET,
    #[doc = "0xae18 - Miscellaneous Register"]
    pub misc_root92_clr: MISC_ROOT92_CLR,
    #[doc = "0xae1c - Miscellaneous Register"]
    pub misc_root92_tog: MISC_ROOT92_TOG,
    #[doc = "0xae20 - Post Divider Register"]
    pub post92: POST92,
    #[doc = "0xae24 - Post Divider Register"]
    pub post_root92_set: POST_ROOT92_SET,
    #[doc = "0xae28 - Post Divider Register"]
    pub post_root92_clr: POST_ROOT92_CLR,
    #[doc = "0xae2c - Post Divider Register"]
    pub post_root92_tog: POST_ROOT92_TOG,
    #[doc = "0xae30 - Pre Divider Register"]
    pub pre92: PRE92,
    #[doc = "0xae34 - Pre Divider Register"]
    pub pre_root92_set: PRE_ROOT92_SET,
    #[doc = "0xae38 - Pre Divider Register"]
    pub pre_root92_clr: PRE_ROOT92_CLR,
    #[doc = "0xae3c - Pre Divider Register"]
    pub pre_root92_tog: PRE_ROOT92_TOG,
    _reserved60: [u8; 48usize],
    #[doc = "0xae70 - Access Control Register"]
    pub access_ctrl92: ACCESS_CTRL92,
    #[doc = "0xae74 - Access Control Register"]
    pub access_ctrl_root92_set: ACCESS_CTRL_ROOT92_SET,
    #[doc = "0xae78 - Access Control Register"]
    pub access_ctrl_root92_clr: ACCESS_CTRL_ROOT92_CLR,
    #[doc = "0xae7c - Access Control Register"]
    pub access_ctrl_root92_tog: ACCESS_CTRL_ROOT92_TOG,
    #[doc = "0xae80 - Target Register"]
    pub target_root93: TARGET_ROOT93,
    #[doc = "0xae84 - Target Register"]
    pub target_root93_set: TARGET_ROOT93_SET,
    #[doc = "0xae88 - Target Register"]
    pub target_root93_clr: TARGET_ROOT93_CLR,
    #[doc = "0xae8c - Target Register"]
    pub target_root93_tog: TARGET_ROOT93_TOG,
    #[doc = "0xae90 - Miscellaneous Register"]
    pub misc93: MISC93,
    #[doc = "0xae94 - Miscellaneous Register"]
    pub misc_root93_set: MISC_ROOT93_SET,
    #[doc = "0xae98 - Miscellaneous Register"]
    pub misc_root93_clr: MISC_ROOT93_CLR,
    #[doc = "0xae9c - Miscellaneous Register"]
    pub misc_root93_tog: MISC_ROOT93_TOG,
    #[doc = "0xaea0 - Post Divider Register"]
    pub post93: POST93,
    #[doc = "0xaea4 - Post Divider Register"]
    pub post_root93_set: POST_ROOT93_SET,
    #[doc = "0xaea8 - Post Divider Register"]
    pub post_root93_clr: POST_ROOT93_CLR,
    #[doc = "0xaeac - Post Divider Register"]
    pub post_root93_tog: POST_ROOT93_TOG,
    #[doc = "0xaeb0 - Pre Divider Register"]
    pub pre93: PRE93,
    #[doc = "0xaeb4 - Pre Divider Register"]
    pub pre_root93_set: PRE_ROOT93_SET,
    #[doc = "0xaeb8 - Pre Divider Register"]
    pub pre_root93_clr: PRE_ROOT93_CLR,
    #[doc = "0xaebc - Pre Divider Register"]
    pub pre_root93_tog: PRE_ROOT93_TOG,
    _reserved61: [u8; 48usize],
    #[doc = "0xaef0 - Access Control Register"]
    pub access_ctrl93: ACCESS_CTRL93,
    #[doc = "0xaef4 - Access Control Register"]
    pub access_ctrl_root93_set: ACCESS_CTRL_ROOT93_SET,
    #[doc = "0xaef8 - Access Control Register"]
    pub access_ctrl_root93_clr: ACCESS_CTRL_ROOT93_CLR,
    #[doc = "0xaefc - Access Control Register"]
    pub access_ctrl_root93_tog: ACCESS_CTRL_ROOT93_TOG,
    #[doc = "0xaf00 - Target Register"]
    pub target_root94: TARGET_ROOT94,
    #[doc = "0xaf04 - Target Register"]
    pub target_root94_set: TARGET_ROOT94_SET,
    #[doc = "0xaf08 - Target Register"]
    pub target_root94_clr: TARGET_ROOT94_CLR,
    #[doc = "0xaf0c - Target Register"]
    pub target_root94_tog: TARGET_ROOT94_TOG,
    #[doc = "0xaf10 - Miscellaneous Register"]
    pub misc94: MISC94,
    #[doc = "0xaf14 - Miscellaneous Register"]
    pub misc_root94_set: MISC_ROOT94_SET,
    #[doc = "0xaf18 - Miscellaneous Register"]
    pub misc_root94_clr: MISC_ROOT94_CLR,
    #[doc = "0xaf1c - Miscellaneous Register"]
    pub misc_root94_tog: MISC_ROOT94_TOG,
    #[doc = "0xaf20 - Post Divider Register"]
    pub post94: POST94,
    #[doc = "0xaf24 - Post Divider Register"]
    pub post_root94_set: POST_ROOT94_SET,
    #[doc = "0xaf28 - Post Divider Register"]
    pub post_root94_clr: POST_ROOT94_CLR,
    #[doc = "0xaf2c - Post Divider Register"]
    pub post_root94_tog: POST_ROOT94_TOG,
    #[doc = "0xaf30 - Pre Divider Register"]
    pub pre94: PRE94,
    #[doc = "0xaf34 - Pre Divider Register"]
    pub pre_root94_set: PRE_ROOT94_SET,
    #[doc = "0xaf38 - Pre Divider Register"]
    pub pre_root94_clr: PRE_ROOT94_CLR,
    #[doc = "0xaf3c - Pre Divider Register"]
    pub pre_root94_tog: PRE_ROOT94_TOG,
    _reserved62: [u8; 48usize],
    #[doc = "0xaf70 - Access Control Register"]
    pub access_ctrl94: ACCESS_CTRL94,
    #[doc = "0xaf74 - Access Control Register"]
    pub access_ctrl_root94_set: ACCESS_CTRL_ROOT94_SET,
    #[doc = "0xaf78 - Access Control Register"]
    pub access_ctrl_root94_clr: ACCESS_CTRL_ROOT94_CLR,
    #[doc = "0xaf7c - Access Control Register"]
    pub access_ctrl_root94_tog: ACCESS_CTRL_ROOT94_TOG,
    #[doc = "0xaf80 - Target Register"]
    pub target_root95: TARGET_ROOT95,
    #[doc = "0xaf84 - Target Register"]
    pub target_root95_set: TARGET_ROOT95_SET,
    #[doc = "0xaf88 - Target Register"]
    pub target_root95_clr: TARGET_ROOT95_CLR,
    #[doc = "0xaf8c - Target Register"]
    pub target_root95_tog: TARGET_ROOT95_TOG,
    #[doc = "0xaf90 - Miscellaneous Register"]
    pub misc95: MISC95,
    #[doc = "0xaf94 - Miscellaneous Register"]
    pub misc_root95_set: MISC_ROOT95_SET,
    #[doc = "0xaf98 - Miscellaneous Register"]
    pub misc_root95_clr: MISC_ROOT95_CLR,
    #[doc = "0xaf9c - Miscellaneous Register"]
    pub misc_root95_tog: MISC_ROOT95_TOG,
    #[doc = "0xafa0 - Post Divider Register"]
    pub post95: POST95,
    #[doc = "0xafa4 - Post Divider Register"]
    pub post_root95_set: POST_ROOT95_SET,
    #[doc = "0xafa8 - Post Divider Register"]
    pub post_root95_clr: POST_ROOT95_CLR,
    #[doc = "0xafac - Post Divider Register"]
    pub post_root95_tog: POST_ROOT95_TOG,
    #[doc = "0xafb0 - Pre Divider Register"]
    pub pre95: PRE95,
    #[doc = "0xafb4 - Pre Divider Register"]
    pub pre_root95_set: PRE_ROOT95_SET,
    #[doc = "0xafb8 - Pre Divider Register"]
    pub pre_root95_clr: PRE_ROOT95_CLR,
    #[doc = "0xafbc - Pre Divider Register"]
    pub pre_root95_tog: PRE_ROOT95_TOG,
    _reserved63: [u8; 48usize],
    #[doc = "0xaff0 - Access Control Register"]
    pub access_ctrl95: ACCESS_CTRL95,
    #[doc = "0xaff4 - Access Control Register"]
    pub access_ctrl_root95_set: ACCESS_CTRL_ROOT95_SET,
    #[doc = "0xaff8 - Access Control Register"]
    pub access_ctrl_root95_clr: ACCESS_CTRL_ROOT95_CLR,
    #[doc = "0xaffc - Access Control Register"]
    pub access_ctrl_root95_tog: ACCESS_CTRL_ROOT95_TOG,
    #[doc = "0xb000 - Target Register"]
    pub target_root96: TARGET_ROOT96,
    #[doc = "0xb004 - Target Register"]
    pub target_root96_set: TARGET_ROOT96_SET,
    #[doc = "0xb008 - Target Register"]
    pub target_root96_clr: TARGET_ROOT96_CLR,
    #[doc = "0xb00c - Target Register"]
    pub target_root96_tog: TARGET_ROOT96_TOG,
    #[doc = "0xb010 - Miscellaneous Register"]
    pub misc96: MISC96,
    #[doc = "0xb014 - Miscellaneous Register"]
    pub misc_root96_set: MISC_ROOT96_SET,
    #[doc = "0xb018 - Miscellaneous Register"]
    pub misc_root96_clr: MISC_ROOT96_CLR,
    #[doc = "0xb01c - Miscellaneous Register"]
    pub misc_root96_tog: MISC_ROOT96_TOG,
    #[doc = "0xb020 - Post Divider Register"]
    pub post96: POST96,
    #[doc = "0xb024 - Post Divider Register"]
    pub post_root96_set: POST_ROOT96_SET,
    #[doc = "0xb028 - Post Divider Register"]
    pub post_root96_clr: POST_ROOT96_CLR,
    #[doc = "0xb02c - Post Divider Register"]
    pub post_root96_tog: POST_ROOT96_TOG,
    #[doc = "0xb030 - Pre Divider Register"]
    pub pre96: PRE96,
    #[doc = "0xb034 - Pre Divider Register"]
    pub pre_root96_set: PRE_ROOT96_SET,
    #[doc = "0xb038 - Pre Divider Register"]
    pub pre_root96_clr: PRE_ROOT96_CLR,
    #[doc = "0xb03c - Pre Divider Register"]
    pub pre_root96_tog: PRE_ROOT96_TOG,
    _reserved64: [u8; 48usize],
    #[doc = "0xb070 - Access Control Register"]
    pub access_ctrl96: ACCESS_CTRL96,
    #[doc = "0xb074 - Access Control Register"]
    pub access_ctrl_root96_set: ACCESS_CTRL_ROOT96_SET,
    #[doc = "0xb078 - Access Control Register"]
    pub access_ctrl_root96_clr: ACCESS_CTRL_ROOT96_CLR,
    #[doc = "0xb07c - Access Control Register"]
    pub access_ctrl_root96_tog: ACCESS_CTRL_ROOT96_TOG,
    #[doc = "0xb080 - Target Register"]
    pub target_root97: TARGET_ROOT97,
    #[doc = "0xb084 - Target Register"]
    pub target_root97_set: TARGET_ROOT97_SET,
    #[doc = "0xb088 - Target Register"]
    pub target_root97_clr: TARGET_ROOT97_CLR,
    #[doc = "0xb08c - Target Register"]
    pub target_root97_tog: TARGET_ROOT97_TOG,
    #[doc = "0xb090 - Miscellaneous Register"]
    pub misc97: MISC97,
    #[doc = "0xb094 - Miscellaneous Register"]
    pub misc_root97_set: MISC_ROOT97_SET,
    #[doc = "0xb098 - Miscellaneous Register"]
    pub misc_root97_clr: MISC_ROOT97_CLR,
    #[doc = "0xb09c - Miscellaneous Register"]
    pub misc_root97_tog: MISC_ROOT97_TOG,
    #[doc = "0xb0a0 - Post Divider Register"]
    pub post97: POST97,
    #[doc = "0xb0a4 - Post Divider Register"]
    pub post_root97_set: POST_ROOT97_SET,
    #[doc = "0xb0a8 - Post Divider Register"]
    pub post_root97_clr: POST_ROOT97_CLR,
    #[doc = "0xb0ac - Post Divider Register"]
    pub post_root97_tog: POST_ROOT97_TOG,
    #[doc = "0xb0b0 - Pre Divider Register"]
    pub pre97: PRE97,
    #[doc = "0xb0b4 - Pre Divider Register"]
    pub pre_root97_set: PRE_ROOT97_SET,
    #[doc = "0xb0b8 - Pre Divider Register"]
    pub pre_root97_clr: PRE_ROOT97_CLR,
    #[doc = "0xb0bc - Pre Divider Register"]
    pub pre_root97_tog: PRE_ROOT97_TOG,
    _reserved65: [u8; 48usize],
    #[doc = "0xb0f0 - Access Control Register"]
    pub access_ctrl97: ACCESS_CTRL97,
    #[doc = "0xb0f4 - Access Control Register"]
    pub access_ctrl_root97_set: ACCESS_CTRL_ROOT97_SET,
    #[doc = "0xb0f8 - Access Control Register"]
    pub access_ctrl_root97_clr: ACCESS_CTRL_ROOT97_CLR,
    #[doc = "0xb0fc - Access Control Register"]
    pub access_ctrl_root97_tog: ACCESS_CTRL_ROOT97_TOG,
    #[doc = "0xb100 - Target Register"]
    pub target_root98: TARGET_ROOT98,
    #[doc = "0xb104 - Target Register"]
    pub target_root98_set: TARGET_ROOT98_SET,
    #[doc = "0xb108 - Target Register"]
    pub target_root98_clr: TARGET_ROOT98_CLR,
    #[doc = "0xb10c - Target Register"]
    pub target_root98_tog: TARGET_ROOT98_TOG,
    #[doc = "0xb110 - Miscellaneous Register"]
    pub misc98: MISC98,
    #[doc = "0xb114 - Miscellaneous Register"]
    pub misc_root98_set: MISC_ROOT98_SET,
    #[doc = "0xb118 - Miscellaneous Register"]
    pub misc_root98_clr: MISC_ROOT98_CLR,
    #[doc = "0xb11c - Miscellaneous Register"]
    pub misc_root98_tog: MISC_ROOT98_TOG,
    #[doc = "0xb120 - Post Divider Register"]
    pub post98: POST98,
    #[doc = "0xb124 - Post Divider Register"]
    pub post_root98_set: POST_ROOT98_SET,
    #[doc = "0xb128 - Post Divider Register"]
    pub post_root98_clr: POST_ROOT98_CLR,
    #[doc = "0xb12c - Post Divider Register"]
    pub post_root98_tog: POST_ROOT98_TOG,
    #[doc = "0xb130 - Pre Divider Register"]
    pub pre98: PRE98,
    #[doc = "0xb134 - Pre Divider Register"]
    pub pre_root98_set: PRE_ROOT98_SET,
    #[doc = "0xb138 - Pre Divider Register"]
    pub pre_root98_clr: PRE_ROOT98_CLR,
    #[doc = "0xb13c - Pre Divider Register"]
    pub pre_root98_tog: PRE_ROOT98_TOG,
    _reserved66: [u8; 48usize],
    #[doc = "0xb170 - Access Control Register"]
    pub access_ctrl98: ACCESS_CTRL98,
    #[doc = "0xb174 - Access Control Register"]
    pub access_ctrl_root98_set: ACCESS_CTRL_ROOT98_SET,
    #[doc = "0xb178 - Access Control Register"]
    pub access_ctrl_root98_clr: ACCESS_CTRL_ROOT98_CLR,
    #[doc = "0xb17c - Access Control Register"]
    pub access_ctrl_root98_tog: ACCESS_CTRL_ROOT98_TOG,
    #[doc = "0xb180 - Target Register"]
    pub target_root99: TARGET_ROOT99,
    #[doc = "0xb184 - Target Register"]
    pub target_root99_set: TARGET_ROOT99_SET,
    #[doc = "0xb188 - Target Register"]
    pub target_root99_clr: TARGET_ROOT99_CLR,
    #[doc = "0xb18c - Target Register"]
    pub target_root99_tog: TARGET_ROOT99_TOG,
    #[doc = "0xb190 - Miscellaneous Register"]
    pub misc99: MISC99,
    #[doc = "0xb194 - Miscellaneous Register"]
    pub misc_root99_set: MISC_ROOT99_SET,
    #[doc = "0xb198 - Miscellaneous Register"]
    pub misc_root99_clr: MISC_ROOT99_CLR,
    #[doc = "0xb19c - Miscellaneous Register"]
    pub misc_root99_tog: MISC_ROOT99_TOG,
    #[doc = "0xb1a0 - Post Divider Register"]
    pub post99: POST99,
    #[doc = "0xb1a4 - Post Divider Register"]
    pub post_root99_set: POST_ROOT99_SET,
    #[doc = "0xb1a8 - Post Divider Register"]
    pub post_root99_clr: POST_ROOT99_CLR,
    #[doc = "0xb1ac - Post Divider Register"]
    pub post_root99_tog: POST_ROOT99_TOG,
    #[doc = "0xb1b0 - Pre Divider Register"]
    pub pre99: PRE99,
    #[doc = "0xb1b4 - Pre Divider Register"]
    pub pre_root99_set: PRE_ROOT99_SET,
    #[doc = "0xb1b8 - Pre Divider Register"]
    pub pre_root99_clr: PRE_ROOT99_CLR,
    #[doc = "0xb1bc - Pre Divider Register"]
    pub pre_root99_tog: PRE_ROOT99_TOG,
    _reserved67: [u8; 48usize],
    #[doc = "0xb1f0 - Access Control Register"]
    pub access_ctrl99: ACCESS_CTRL99,
    #[doc = "0xb1f4 - Access Control Register"]
    pub access_ctrl_root99_set: ACCESS_CTRL_ROOT99_SET,
    #[doc = "0xb1f8 - Access Control Register"]
    pub access_ctrl_root99_clr: ACCESS_CTRL_ROOT99_CLR,
    #[doc = "0xb1fc - Access Control Register"]
    pub access_ctrl_root99_tog: ACCESS_CTRL_ROOT99_TOG,
    #[doc = "0xb200 - Target Register"]
    pub target_root100: TARGET_ROOT100,
    #[doc = "0xb204 - Target Register"]
    pub target_root100_set: TARGET_ROOT100_SET,
    #[doc = "0xb208 - Target Register"]
    pub target_root100_clr: TARGET_ROOT100_CLR,
    #[doc = "0xb20c - Target Register"]
    pub target_root100_tog: TARGET_ROOT100_TOG,
    #[doc = "0xb210 - Miscellaneous Register"]
    pub misc100: MISC100,
    #[doc = "0xb214 - Miscellaneous Register"]
    pub misc_root100_set: MISC_ROOT100_SET,
    #[doc = "0xb218 - Miscellaneous Register"]
    pub misc_root100_clr: MISC_ROOT100_CLR,
    #[doc = "0xb21c - Miscellaneous Register"]
    pub misc_root100_tog: MISC_ROOT100_TOG,
    #[doc = "0xb220 - Post Divider Register"]
    pub post100: POST100,
    #[doc = "0xb224 - Post Divider Register"]
    pub post_root100_set: POST_ROOT100_SET,
    #[doc = "0xb228 - Post Divider Register"]
    pub post_root100_clr: POST_ROOT100_CLR,
    #[doc = "0xb22c - Post Divider Register"]
    pub post_root100_tog: POST_ROOT100_TOG,
    #[doc = "0xb230 - Pre Divider Register"]
    pub pre100: PRE100,
    #[doc = "0xb234 - Pre Divider Register"]
    pub pre_root100_set: PRE_ROOT100_SET,
    #[doc = "0xb238 - Pre Divider Register"]
    pub pre_root100_clr: PRE_ROOT100_CLR,
    #[doc = "0xb23c - Pre Divider Register"]
    pub pre_root100_tog: PRE_ROOT100_TOG,
    _reserved68: [u8; 48usize],
    #[doc = "0xb270 - Access Control Register"]
    pub access_ctrl100: ACCESS_CTRL100,
    #[doc = "0xb274 - Access Control Register"]
    pub access_ctrl_root100_set: ACCESS_CTRL_ROOT100_SET,
    #[doc = "0xb278 - Access Control Register"]
    pub access_ctrl_root100_clr: ACCESS_CTRL_ROOT100_CLR,
    #[doc = "0xb27c - Access Control Register"]
    pub access_ctrl_root100_tog: ACCESS_CTRL_ROOT100_TOG,
    #[doc = "0xb280 - Target Register"]
    pub target_root101: TARGET_ROOT101,
    #[doc = "0xb284 - Target Register"]
    pub target_root101_set: TARGET_ROOT101_SET,
    #[doc = "0xb288 - Target Register"]
    pub target_root101_clr: TARGET_ROOT101_CLR,
    #[doc = "0xb28c - Target Register"]
    pub target_root101_tog: TARGET_ROOT101_TOG,
    #[doc = "0xb290 - Miscellaneous Register"]
    pub misc101: MISC101,
    #[doc = "0xb294 - Miscellaneous Register"]
    pub misc_root101_set: MISC_ROOT101_SET,
    #[doc = "0xb298 - Miscellaneous Register"]
    pub misc_root101_clr: MISC_ROOT101_CLR,
    #[doc = "0xb29c - Miscellaneous Register"]
    pub misc_root101_tog: MISC_ROOT101_TOG,
    #[doc = "0xb2a0 - Post Divider Register"]
    pub post101: POST101,
    #[doc = "0xb2a4 - Post Divider Register"]
    pub post_root101_set: POST_ROOT101_SET,
    #[doc = "0xb2a8 - Post Divider Register"]
    pub post_root101_clr: POST_ROOT101_CLR,
    #[doc = "0xb2ac - Post Divider Register"]
    pub post_root101_tog: POST_ROOT101_TOG,
    #[doc = "0xb2b0 - Pre Divider Register"]
    pub pre101: PRE101,
    #[doc = "0xb2b4 - Pre Divider Register"]
    pub pre_root101_set: PRE_ROOT101_SET,
    #[doc = "0xb2b8 - Pre Divider Register"]
    pub pre_root101_clr: PRE_ROOT101_CLR,
    #[doc = "0xb2bc - Pre Divider Register"]
    pub pre_root101_tog: PRE_ROOT101_TOG,
    _reserved69: [u8; 48usize],
    #[doc = "0xb2f0 - Access Control Register"]
    pub access_ctrl101: ACCESS_CTRL101,
    #[doc = "0xb2f4 - Access Control Register"]
    pub access_ctrl_root101_set: ACCESS_CTRL_ROOT101_SET,
    #[doc = "0xb2f8 - Access Control Register"]
    pub access_ctrl_root101_clr: ACCESS_CTRL_ROOT101_CLR,
    #[doc = "0xb2fc - Access Control Register"]
    pub access_ctrl_root101_tog: ACCESS_CTRL_ROOT101_TOG,
    #[doc = "0xb300 - Target Register"]
    pub target_root102: TARGET_ROOT102,
    #[doc = "0xb304 - Target Register"]
    pub target_root102_set: TARGET_ROOT102_SET,
    #[doc = "0xb308 - Target Register"]
    pub target_root102_clr: TARGET_ROOT102_CLR,
    #[doc = "0xb30c - Target Register"]
    pub target_root102_tog: TARGET_ROOT102_TOG,
    #[doc = "0xb310 - Miscellaneous Register"]
    pub misc102: MISC102,
    #[doc = "0xb314 - Miscellaneous Register"]
    pub misc_root102_set: MISC_ROOT102_SET,
    #[doc = "0xb318 - Miscellaneous Register"]
    pub misc_root102_clr: MISC_ROOT102_CLR,
    #[doc = "0xb31c - Miscellaneous Register"]
    pub misc_root102_tog: MISC_ROOT102_TOG,
    #[doc = "0xb320 - Post Divider Register"]
    pub post102: POST102,
    #[doc = "0xb324 - Post Divider Register"]
    pub post_root102_set: POST_ROOT102_SET,
    #[doc = "0xb328 - Post Divider Register"]
    pub post_root102_clr: POST_ROOT102_CLR,
    #[doc = "0xb32c - Post Divider Register"]
    pub post_root102_tog: POST_ROOT102_TOG,
    #[doc = "0xb330 - Pre Divider Register"]
    pub pre102: PRE102,
    #[doc = "0xb334 - Pre Divider Register"]
    pub pre_root102_set: PRE_ROOT102_SET,
    #[doc = "0xb338 - Pre Divider Register"]
    pub pre_root102_clr: PRE_ROOT102_CLR,
    #[doc = "0xb33c - Pre Divider Register"]
    pub pre_root102_tog: PRE_ROOT102_TOG,
    _reserved70: [u8; 48usize],
    #[doc = "0xb370 - Access Control Register"]
    pub access_ctrl102: ACCESS_CTRL102,
    #[doc = "0xb374 - Access Control Register"]
    pub access_ctrl_root102_set: ACCESS_CTRL_ROOT102_SET,
    #[doc = "0xb378 - Access Control Register"]
    pub access_ctrl_root102_clr: ACCESS_CTRL_ROOT102_CLR,
    #[doc = "0xb37c - Access Control Register"]
    pub access_ctrl_root102_tog: ACCESS_CTRL_ROOT102_TOG,
    #[doc = "0xb380 - Target Register"]
    pub target_root103: TARGET_ROOT103,
    #[doc = "0xb384 - Target Register"]
    pub target_root103_set: TARGET_ROOT103_SET,
    #[doc = "0xb388 - Target Register"]
    pub target_root103_clr: TARGET_ROOT103_CLR,
    #[doc = "0xb38c - Target Register"]
    pub target_root103_tog: TARGET_ROOT103_TOG,
    #[doc = "0xb390 - Miscellaneous Register"]
    pub misc103: MISC103,
    #[doc = "0xb394 - Miscellaneous Register"]
    pub misc_root103_set: MISC_ROOT103_SET,
    #[doc = "0xb398 - Miscellaneous Register"]
    pub misc_root103_clr: MISC_ROOT103_CLR,
    #[doc = "0xb39c - Miscellaneous Register"]
    pub misc_root103_tog: MISC_ROOT103_TOG,
    #[doc = "0xb3a0 - Post Divider Register"]
    pub post103: POST103,
    #[doc = "0xb3a4 - Post Divider Register"]
    pub post_root103_set: POST_ROOT103_SET,
    #[doc = "0xb3a8 - Post Divider Register"]
    pub post_root103_clr: POST_ROOT103_CLR,
    #[doc = "0xb3ac - Post Divider Register"]
    pub post_root103_tog: POST_ROOT103_TOG,
    #[doc = "0xb3b0 - Pre Divider Register"]
    pub pre103: PRE103,
    #[doc = "0xb3b4 - Pre Divider Register"]
    pub pre_root103_set: PRE_ROOT103_SET,
    #[doc = "0xb3b8 - Pre Divider Register"]
    pub pre_root103_clr: PRE_ROOT103_CLR,
    #[doc = "0xb3bc - Pre Divider Register"]
    pub pre_root103_tog: PRE_ROOT103_TOG,
    _reserved71: [u8; 48usize],
    #[doc = "0xb3f0 - Access Control Register"]
    pub access_ctrl103: ACCESS_CTRL103,
    #[doc = "0xb3f4 - Access Control Register"]
    pub access_ctrl_root103_set: ACCESS_CTRL_ROOT103_SET,
    #[doc = "0xb3f8 - Access Control Register"]
    pub access_ctrl_root103_clr: ACCESS_CTRL_ROOT103_CLR,
    #[doc = "0xb3fc - Access Control Register"]
    pub access_ctrl_root103_tog: ACCESS_CTRL_ROOT103_TOG,
    #[doc = "0xb400 - Target Register"]
    pub target_root104: TARGET_ROOT104,
    #[doc = "0xb404 - Target Register"]
    pub target_root104_set: TARGET_ROOT104_SET,
    #[doc = "0xb408 - Target Register"]
    pub target_root104_clr: TARGET_ROOT104_CLR,
    #[doc = "0xb40c - Target Register"]
    pub target_root104_tog: TARGET_ROOT104_TOG,
    #[doc = "0xb410 - Miscellaneous Register"]
    pub misc104: MISC104,
    #[doc = "0xb414 - Miscellaneous Register"]
    pub misc_root104_set: MISC_ROOT104_SET,
    #[doc = "0xb418 - Miscellaneous Register"]
    pub misc_root104_clr: MISC_ROOT104_CLR,
    #[doc = "0xb41c - Miscellaneous Register"]
    pub misc_root104_tog: MISC_ROOT104_TOG,
    #[doc = "0xb420 - Post Divider Register"]
    pub post104: POST104,
    #[doc = "0xb424 - Post Divider Register"]
    pub post_root104_set: POST_ROOT104_SET,
    #[doc = "0xb428 - Post Divider Register"]
    pub post_root104_clr: POST_ROOT104_CLR,
    #[doc = "0xb42c - Post Divider Register"]
    pub post_root104_tog: POST_ROOT104_TOG,
    #[doc = "0xb430 - Pre Divider Register"]
    pub pre104: PRE104,
    #[doc = "0xb434 - Pre Divider Register"]
    pub pre_root104_set: PRE_ROOT104_SET,
    #[doc = "0xb438 - Pre Divider Register"]
    pub pre_root104_clr: PRE_ROOT104_CLR,
    #[doc = "0xb43c - Pre Divider Register"]
    pub pre_root104_tog: PRE_ROOT104_TOG,
    _reserved72: [u8; 48usize],
    #[doc = "0xb470 - Access Control Register"]
    pub access_ctrl104: ACCESS_CTRL104,
    #[doc = "0xb474 - Access Control Register"]
    pub access_ctrl_root104_set: ACCESS_CTRL_ROOT104_SET,
    #[doc = "0xb478 - Access Control Register"]
    pub access_ctrl_root104_clr: ACCESS_CTRL_ROOT104_CLR,
    #[doc = "0xb47c - Access Control Register"]
    pub access_ctrl_root104_tog: ACCESS_CTRL_ROOT104_TOG,
    #[doc = "0xb480 - Target Register"]
    pub target_root105: TARGET_ROOT105,
    #[doc = "0xb484 - Target Register"]
    pub target_root105_set: TARGET_ROOT105_SET,
    #[doc = "0xb488 - Target Register"]
    pub target_root105_clr: TARGET_ROOT105_CLR,
    #[doc = "0xb48c - Target Register"]
    pub target_root105_tog: TARGET_ROOT105_TOG,
    #[doc = "0xb490 - Miscellaneous Register"]
    pub misc105: MISC105,
    #[doc = "0xb494 - Miscellaneous Register"]
    pub misc_root105_set: MISC_ROOT105_SET,
    #[doc = "0xb498 - Miscellaneous Register"]
    pub misc_root105_clr: MISC_ROOT105_CLR,
    #[doc = "0xb49c - Miscellaneous Register"]
    pub misc_root105_tog: MISC_ROOT105_TOG,
    #[doc = "0xb4a0 - Post Divider Register"]
    pub post105: POST105,
    #[doc = "0xb4a4 - Post Divider Register"]
    pub post_root105_set: POST_ROOT105_SET,
    #[doc = "0xb4a8 - Post Divider Register"]
    pub post_root105_clr: POST_ROOT105_CLR,
    #[doc = "0xb4ac - Post Divider Register"]
    pub post_root105_tog: POST_ROOT105_TOG,
    #[doc = "0xb4b0 - Pre Divider Register"]
    pub pre105: PRE105,
    #[doc = "0xb4b4 - Pre Divider Register"]
    pub pre_root105_set: PRE_ROOT105_SET,
    #[doc = "0xb4b8 - Pre Divider Register"]
    pub pre_root105_clr: PRE_ROOT105_CLR,
    #[doc = "0xb4bc - Pre Divider Register"]
    pub pre_root105_tog: PRE_ROOT105_TOG,
    _reserved73: [u8; 48usize],
    #[doc = "0xb4f0 - Access Control Register"]
    pub access_ctrl105: ACCESS_CTRL105,
    #[doc = "0xb4f4 - Access Control Register"]
    pub access_ctrl_root105_set: ACCESS_CTRL_ROOT105_SET,
    #[doc = "0xb4f8 - Access Control Register"]
    pub access_ctrl_root105_clr: ACCESS_CTRL_ROOT105_CLR,
    #[doc = "0xb4fc - Access Control Register"]
    pub access_ctrl_root105_tog: ACCESS_CTRL_ROOT105_TOG,
    #[doc = "0xb500 - Target Register"]
    pub target_root106: TARGET_ROOT106,
    #[doc = "0xb504 - Target Register"]
    pub target_root106_set: TARGET_ROOT106_SET,
    #[doc = "0xb508 - Target Register"]
    pub target_root106_clr: TARGET_ROOT106_CLR,
    #[doc = "0xb50c - Target Register"]
    pub target_root106_tog: TARGET_ROOT106_TOG,
    #[doc = "0xb510 - Miscellaneous Register"]
    pub misc106: MISC106,
    #[doc = "0xb514 - Miscellaneous Register"]
    pub misc_root106_set: MISC_ROOT106_SET,
    #[doc = "0xb518 - Miscellaneous Register"]
    pub misc_root106_clr: MISC_ROOT106_CLR,
    #[doc = "0xb51c - Miscellaneous Register"]
    pub misc_root106_tog: MISC_ROOT106_TOG,
    #[doc = "0xb520 - Post Divider Register"]
    pub post106: POST106,
    #[doc = "0xb524 - Post Divider Register"]
    pub post_root106_set: POST_ROOT106_SET,
    #[doc = "0xb528 - Post Divider Register"]
    pub post_root106_clr: POST_ROOT106_CLR,
    #[doc = "0xb52c - Post Divider Register"]
    pub post_root106_tog: POST_ROOT106_TOG,
    #[doc = "0xb530 - Pre Divider Register"]
    pub pre106: PRE106,
    #[doc = "0xb534 - Pre Divider Register"]
    pub pre_root106_set: PRE_ROOT106_SET,
    #[doc = "0xb538 - Pre Divider Register"]
    pub pre_root106_clr: PRE_ROOT106_CLR,
    #[doc = "0xb53c - Pre Divider Register"]
    pub pre_root106_tog: PRE_ROOT106_TOG,
    _reserved74: [u8; 48usize],
    #[doc = "0xb570 - Access Control Register"]
    pub access_ctrl106: ACCESS_CTRL106,
    #[doc = "0xb574 - Access Control Register"]
    pub access_ctrl_root106_set: ACCESS_CTRL_ROOT106_SET,
    #[doc = "0xb578 - Access Control Register"]
    pub access_ctrl_root106_clr: ACCESS_CTRL_ROOT106_CLR,
    #[doc = "0xb57c - Access Control Register"]
    pub access_ctrl_root106_tog: ACCESS_CTRL_ROOT106_TOG,
    #[doc = "0xb580 - Target Register"]
    pub target_root107: TARGET_ROOT107,
    #[doc = "0xb584 - Target Register"]
    pub target_root107_set: TARGET_ROOT107_SET,
    #[doc = "0xb588 - Target Register"]
    pub target_root107_clr: TARGET_ROOT107_CLR,
    #[doc = "0xb58c - Target Register"]
    pub target_root107_tog: TARGET_ROOT107_TOG,
    #[doc = "0xb590 - Miscellaneous Register"]
    pub misc107: MISC107,
    #[doc = "0xb594 - Miscellaneous Register"]
    pub misc_root107_set: MISC_ROOT107_SET,
    #[doc = "0xb598 - Miscellaneous Register"]
    pub misc_root107_clr: MISC_ROOT107_CLR,
    #[doc = "0xb59c - Miscellaneous Register"]
    pub misc_root107_tog: MISC_ROOT107_TOG,
    #[doc = "0xb5a0 - Post Divider Register"]
    pub post107: POST107,
    #[doc = "0xb5a4 - Post Divider Register"]
    pub post_root107_set: POST_ROOT107_SET,
    #[doc = "0xb5a8 - Post Divider Register"]
    pub post_root107_clr: POST_ROOT107_CLR,
    #[doc = "0xb5ac - Post Divider Register"]
    pub post_root107_tog: POST_ROOT107_TOG,
    #[doc = "0xb5b0 - Pre Divider Register"]
    pub pre107: PRE107,
    #[doc = "0xb5b4 - Pre Divider Register"]
    pub pre_root107_set: PRE_ROOT107_SET,
    #[doc = "0xb5b8 - Pre Divider Register"]
    pub pre_root107_clr: PRE_ROOT107_CLR,
    #[doc = "0xb5bc - Pre Divider Register"]
    pub pre_root107_tog: PRE_ROOT107_TOG,
    _reserved75: [u8; 48usize],
    #[doc = "0xb5f0 - Access Control Register"]
    pub access_ctrl107: ACCESS_CTRL107,
    #[doc = "0xb5f4 - Access Control Register"]
    pub access_ctrl_root107_set: ACCESS_CTRL_ROOT107_SET,
    #[doc = "0xb5f8 - Access Control Register"]
    pub access_ctrl_root107_clr: ACCESS_CTRL_ROOT107_CLR,
    #[doc = "0xb5fc - Access Control Register"]
    pub access_ctrl_root107_tog: ACCESS_CTRL_ROOT107_TOG,
    #[doc = "0xb600 - Target Register"]
    pub target_root108: TARGET_ROOT108,
    #[doc = "0xb604 - Target Register"]
    pub target_root108_set: TARGET_ROOT108_SET,
    #[doc = "0xb608 - Target Register"]
    pub target_root108_clr: TARGET_ROOT108_CLR,
    #[doc = "0xb60c - Target Register"]
    pub target_root108_tog: TARGET_ROOT108_TOG,
    #[doc = "0xb610 - Miscellaneous Register"]
    pub misc108: MISC108,
    #[doc = "0xb614 - Miscellaneous Register"]
    pub misc_root108_set: MISC_ROOT108_SET,
    #[doc = "0xb618 - Miscellaneous Register"]
    pub misc_root108_clr: MISC_ROOT108_CLR,
    #[doc = "0xb61c - Miscellaneous Register"]
    pub misc_root108_tog: MISC_ROOT108_TOG,
    #[doc = "0xb620 - Post Divider Register"]
    pub post108: POST108,
    #[doc = "0xb624 - Post Divider Register"]
    pub post_root108_set: POST_ROOT108_SET,
    #[doc = "0xb628 - Post Divider Register"]
    pub post_root108_clr: POST_ROOT108_CLR,
    #[doc = "0xb62c - Post Divider Register"]
    pub post_root108_tog: POST_ROOT108_TOG,
    #[doc = "0xb630 - Pre Divider Register"]
    pub pre108: PRE108,
    #[doc = "0xb634 - Pre Divider Register"]
    pub pre_root108_set: PRE_ROOT108_SET,
    #[doc = "0xb638 - Pre Divider Register"]
    pub pre_root108_clr: PRE_ROOT108_CLR,
    #[doc = "0xb63c - Pre Divider Register"]
    pub pre_root108_tog: PRE_ROOT108_TOG,
    _reserved76: [u8; 48usize],
    #[doc = "0xb670 - Access Control Register"]
    pub access_ctrl108: ACCESS_CTRL108,
    #[doc = "0xb674 - Access Control Register"]
    pub access_ctrl_root108_set: ACCESS_CTRL_ROOT108_SET,
    #[doc = "0xb678 - Access Control Register"]
    pub access_ctrl_root108_clr: ACCESS_CTRL_ROOT108_CLR,
    #[doc = "0xb67c - Access Control Register"]
    pub access_ctrl_root108_tog: ACCESS_CTRL_ROOT108_TOG,
    #[doc = "0xb680 - Target Register"]
    pub target_root109: TARGET_ROOT109,
    #[doc = "0xb684 - Target Register"]
    pub target_root109_set: TARGET_ROOT109_SET,
    #[doc = "0xb688 - Target Register"]
    pub target_root109_clr: TARGET_ROOT109_CLR,
    #[doc = "0xb68c - Target Register"]
    pub target_root109_tog: TARGET_ROOT109_TOG,
    #[doc = "0xb690 - Miscellaneous Register"]
    pub misc109: MISC109,
    #[doc = "0xb694 - Miscellaneous Register"]
    pub misc_root109_set: MISC_ROOT109_SET,
    #[doc = "0xb698 - Miscellaneous Register"]
    pub misc_root109_clr: MISC_ROOT109_CLR,
    #[doc = "0xb69c - Miscellaneous Register"]
    pub misc_root109_tog: MISC_ROOT109_TOG,
    #[doc = "0xb6a0 - Post Divider Register"]
    pub post109: POST109,
    #[doc = "0xb6a4 - Post Divider Register"]
    pub post_root109_set: POST_ROOT109_SET,
    #[doc = "0xb6a8 - Post Divider Register"]
    pub post_root109_clr: POST_ROOT109_CLR,
    #[doc = "0xb6ac - Post Divider Register"]
    pub post_root109_tog: POST_ROOT109_TOG,
    #[doc = "0xb6b0 - Pre Divider Register"]
    pub pre109: PRE109,
    #[doc = "0xb6b4 - Pre Divider Register"]
    pub pre_root109_set: PRE_ROOT109_SET,
    #[doc = "0xb6b8 - Pre Divider Register"]
    pub pre_root109_clr: PRE_ROOT109_CLR,
    #[doc = "0xb6bc - Pre Divider Register"]
    pub pre_root109_tog: PRE_ROOT109_TOG,
    _reserved77: [u8; 48usize],
    #[doc = "0xb6f0 - Access Control Register"]
    pub access_ctrl109: ACCESS_CTRL109,
    #[doc = "0xb6f4 - Access Control Register"]
    pub access_ctrl_root109_set: ACCESS_CTRL_ROOT109_SET,
    #[doc = "0xb6f8 - Access Control Register"]
    pub access_ctrl_root109_clr: ACCESS_CTRL_ROOT109_CLR,
    #[doc = "0xb6fc - Access Control Register"]
    pub access_ctrl_root109_tog: ACCESS_CTRL_ROOT109_TOG,
    #[doc = "0xb700 - Target Register"]
    pub target_root110: TARGET_ROOT110,
    #[doc = "0xb704 - Target Register"]
    pub target_root110_set: TARGET_ROOT110_SET,
    #[doc = "0xb708 - Target Register"]
    pub target_root110_clr: TARGET_ROOT110_CLR,
    #[doc = "0xb70c - Target Register"]
    pub target_root110_tog: TARGET_ROOT110_TOG,
    #[doc = "0xb710 - Miscellaneous Register"]
    pub misc110: MISC110,
    #[doc = "0xb714 - Miscellaneous Register"]
    pub misc_root110_set: MISC_ROOT110_SET,
    #[doc = "0xb718 - Miscellaneous Register"]
    pub misc_root110_clr: MISC_ROOT110_CLR,
    #[doc = "0xb71c - Miscellaneous Register"]
    pub misc_root110_tog: MISC_ROOT110_TOG,
    #[doc = "0xb720 - Post Divider Register"]
    pub post110: POST110,
    #[doc = "0xb724 - Post Divider Register"]
    pub post_root110_set: POST_ROOT110_SET,
    #[doc = "0xb728 - Post Divider Register"]
    pub post_root110_clr: POST_ROOT110_CLR,
    #[doc = "0xb72c - Post Divider Register"]
    pub post_root110_tog: POST_ROOT110_TOG,
    #[doc = "0xb730 - Pre Divider Register"]
    pub pre110: PRE110,
    #[doc = "0xb734 - Pre Divider Register"]
    pub pre_root110_set: PRE_ROOT110_SET,
    #[doc = "0xb738 - Pre Divider Register"]
    pub pre_root110_clr: PRE_ROOT110_CLR,
    #[doc = "0xb73c - Pre Divider Register"]
    pub pre_root110_tog: PRE_ROOT110_TOG,
    _reserved78: [u8; 48usize],
    #[doc = "0xb770 - Access Control Register"]
    pub access_ctrl110: ACCESS_CTRL110,
    #[doc = "0xb774 - Access Control Register"]
    pub access_ctrl_root110_set: ACCESS_CTRL_ROOT110_SET,
    #[doc = "0xb778 - Access Control Register"]
    pub access_ctrl_root110_clr: ACCESS_CTRL_ROOT110_CLR,
    #[doc = "0xb77c - Access Control Register"]
    pub access_ctrl_root110_tog: ACCESS_CTRL_ROOT110_TOG,
    #[doc = "0xb780 - Target Register"]
    pub target_root111: TARGET_ROOT111,
    #[doc = "0xb784 - Target Register"]
    pub target_root111_set: TARGET_ROOT111_SET,
    #[doc = "0xb788 - Target Register"]
    pub target_root111_clr: TARGET_ROOT111_CLR,
    #[doc = "0xb78c - Target Register"]
    pub target_root111_tog: TARGET_ROOT111_TOG,
    #[doc = "0xb790 - Miscellaneous Register"]
    pub misc111: MISC111,
    #[doc = "0xb794 - Miscellaneous Register"]
    pub misc_root111_set: MISC_ROOT111_SET,
    #[doc = "0xb798 - Miscellaneous Register"]
    pub misc_root111_clr: MISC_ROOT111_CLR,
    #[doc = "0xb79c - Miscellaneous Register"]
    pub misc_root111_tog: MISC_ROOT111_TOG,
    #[doc = "0xb7a0 - Post Divider Register"]
    pub post111: POST111,
    #[doc = "0xb7a4 - Post Divider Register"]
    pub post_root111_set: POST_ROOT111_SET,
    #[doc = "0xb7a8 - Post Divider Register"]
    pub post_root111_clr: POST_ROOT111_CLR,
    #[doc = "0xb7ac - Post Divider Register"]
    pub post_root111_tog: POST_ROOT111_TOG,
    #[doc = "0xb7b0 - Pre Divider Register"]
    pub pre111: PRE111,
    #[doc = "0xb7b4 - Pre Divider Register"]
    pub pre_root111_set: PRE_ROOT111_SET,
    #[doc = "0xb7b8 - Pre Divider Register"]
    pub pre_root111_clr: PRE_ROOT111_CLR,
    #[doc = "0xb7bc - Pre Divider Register"]
    pub pre_root111_tog: PRE_ROOT111_TOG,
    _reserved79: [u8; 48usize],
    #[doc = "0xb7f0 - Access Control Register"]
    pub access_ctrl111: ACCESS_CTRL111,
    #[doc = "0xb7f4 - Access Control Register"]
    pub access_ctrl_root111_set: ACCESS_CTRL_ROOT111_SET,
    #[doc = "0xb7f8 - Access Control Register"]
    pub access_ctrl_root111_clr: ACCESS_CTRL_ROOT111_CLR,
    #[doc = "0xb7fc - Access Control Register"]
    pub access_ctrl_root111_tog: ACCESS_CTRL_ROOT111_TOG,
    #[doc = "0xb800 - Target Register"]
    pub target_root112: TARGET_ROOT112,
    #[doc = "0xb804 - Target Register"]
    pub target_root112_set: TARGET_ROOT112_SET,
    #[doc = "0xb808 - Target Register"]
    pub target_root112_clr: TARGET_ROOT112_CLR,
    #[doc = "0xb80c - Target Register"]
    pub target_root112_tog: TARGET_ROOT112_TOG,
    #[doc = "0xb810 - Miscellaneous Register"]
    pub misc112: MISC112,
    #[doc = "0xb814 - Miscellaneous Register"]
    pub misc_root112_set: MISC_ROOT112_SET,
    #[doc = "0xb818 - Miscellaneous Register"]
    pub misc_root112_clr: MISC_ROOT112_CLR,
    #[doc = "0xb81c - Miscellaneous Register"]
    pub misc_root112_tog: MISC_ROOT112_TOG,
    #[doc = "0xb820 - Post Divider Register"]
    pub post112: POST112,
    #[doc = "0xb824 - Post Divider Register"]
    pub post_root112_set: POST_ROOT112_SET,
    #[doc = "0xb828 - Post Divider Register"]
    pub post_root112_clr: POST_ROOT112_CLR,
    #[doc = "0xb82c - Post Divider Register"]
    pub post_root112_tog: POST_ROOT112_TOG,
    #[doc = "0xb830 - Pre Divider Register"]
    pub pre112: PRE112,
    #[doc = "0xb834 - Pre Divider Register"]
    pub pre_root112_set: PRE_ROOT112_SET,
    #[doc = "0xb838 - Pre Divider Register"]
    pub pre_root112_clr: PRE_ROOT112_CLR,
    #[doc = "0xb83c - Pre Divider Register"]
    pub pre_root112_tog: PRE_ROOT112_TOG,
    _reserved80: [u8; 48usize],
    #[doc = "0xb870 - Access Control Register"]
    pub access_ctrl112: ACCESS_CTRL112,
    #[doc = "0xb874 - Access Control Register"]
    pub access_ctrl_root112_set: ACCESS_CTRL_ROOT112_SET,
    #[doc = "0xb878 - Access Control Register"]
    pub access_ctrl_root112_clr: ACCESS_CTRL_ROOT112_CLR,
    #[doc = "0xb87c - Access Control Register"]
    pub access_ctrl_root112_tog: ACCESS_CTRL_ROOT112_TOG,
    #[doc = "0xb880 - Target Register"]
    pub target_root113: TARGET_ROOT113,
    #[doc = "0xb884 - Target Register"]
    pub target_root113_set: TARGET_ROOT113_SET,
    #[doc = "0xb888 - Target Register"]
    pub target_root113_clr: TARGET_ROOT113_CLR,
    #[doc = "0xb88c - Target Register"]
    pub target_root113_tog: TARGET_ROOT113_TOG,
    #[doc = "0xb890 - Miscellaneous Register"]
    pub misc113: MISC113,
    #[doc = "0xb894 - Miscellaneous Register"]
    pub misc_root113_set: MISC_ROOT113_SET,
    #[doc = "0xb898 - Miscellaneous Register"]
    pub misc_root113_clr: MISC_ROOT113_CLR,
    #[doc = "0xb89c - Miscellaneous Register"]
    pub misc_root113_tog: MISC_ROOT113_TOG,
    #[doc = "0xb8a0 - Post Divider Register"]
    pub post113: POST113,
    #[doc = "0xb8a4 - Post Divider Register"]
    pub post_root113_set: POST_ROOT113_SET,
    #[doc = "0xb8a8 - Post Divider Register"]
    pub post_root113_clr: POST_ROOT113_CLR,
    #[doc = "0xb8ac - Post Divider Register"]
    pub post_root113_tog: POST_ROOT113_TOG,
    #[doc = "0xb8b0 - Pre Divider Register"]
    pub pre113: PRE113,
    #[doc = "0xb8b4 - Pre Divider Register"]
    pub pre_root113_set: PRE_ROOT113_SET,
    #[doc = "0xb8b8 - Pre Divider Register"]
    pub pre_root113_clr: PRE_ROOT113_CLR,
    #[doc = "0xb8bc - Pre Divider Register"]
    pub pre_root113_tog: PRE_ROOT113_TOG,
    _reserved81: [u8; 48usize],
    #[doc = "0xb8f0 - Access Control Register"]
    pub access_ctrl113: ACCESS_CTRL113,
    #[doc = "0xb8f4 - Access Control Register"]
    pub access_ctrl_root113_set: ACCESS_CTRL_ROOT113_SET,
    #[doc = "0xb8f8 - Access Control Register"]
    pub access_ctrl_root113_clr: ACCESS_CTRL_ROOT113_CLR,
    #[doc = "0xb8fc - Access Control Register"]
    pub access_ctrl_root113_tog: ACCESS_CTRL_ROOT113_TOG,
    #[doc = "0xb900 - Target Register"]
    pub target_root114: TARGET_ROOT114,
    #[doc = "0xb904 - Target Register"]
    pub target_root114_set: TARGET_ROOT114_SET,
    #[doc = "0xb908 - Target Register"]
    pub target_root114_clr: TARGET_ROOT114_CLR,
    #[doc = "0xb90c - Target Register"]
    pub target_root114_tog: TARGET_ROOT114_TOG,
    #[doc = "0xb910 - Miscellaneous Register"]
    pub misc114: MISC114,
    #[doc = "0xb914 - Miscellaneous Register"]
    pub misc_root114_set: MISC_ROOT114_SET,
    #[doc = "0xb918 - Miscellaneous Register"]
    pub misc_root114_clr: MISC_ROOT114_CLR,
    #[doc = "0xb91c - Miscellaneous Register"]
    pub misc_root114_tog: MISC_ROOT114_TOG,
    #[doc = "0xb920 - Post Divider Register"]
    pub post114: POST114,
    #[doc = "0xb924 - Post Divider Register"]
    pub post_root114_set: POST_ROOT114_SET,
    #[doc = "0xb928 - Post Divider Register"]
    pub post_root114_clr: POST_ROOT114_CLR,
    #[doc = "0xb92c - Post Divider Register"]
    pub post_root114_tog: POST_ROOT114_TOG,
    #[doc = "0xb930 - Pre Divider Register"]
    pub pre114: PRE114,
    #[doc = "0xb934 - Pre Divider Register"]
    pub pre_root114_set: PRE_ROOT114_SET,
    #[doc = "0xb938 - Pre Divider Register"]
    pub pre_root114_clr: PRE_ROOT114_CLR,
    #[doc = "0xb93c - Pre Divider Register"]
    pub pre_root114_tog: PRE_ROOT114_TOG,
    _reserved82: [u8; 48usize],
    #[doc = "0xb970 - Access Control Register"]
    pub access_ctrl114: ACCESS_CTRL114,
    #[doc = "0xb974 - Access Control Register"]
    pub access_ctrl_root114_set: ACCESS_CTRL_ROOT114_SET,
    #[doc = "0xb978 - Access Control Register"]
    pub access_ctrl_root114_clr: ACCESS_CTRL_ROOT114_CLR,
    #[doc = "0xb97c - Access Control Register"]
    pub access_ctrl_root114_tog: ACCESS_CTRL_ROOT114_TOG,
    #[doc = "0xb980 - Target Register"]
    pub target_root115: TARGET_ROOT115,
    #[doc = "0xb984 - Target Register"]
    pub target_root115_set: TARGET_ROOT115_SET,
    #[doc = "0xb988 - Target Register"]
    pub target_root115_clr: TARGET_ROOT115_CLR,
    #[doc = "0xb98c - Target Register"]
    pub target_root115_tog: TARGET_ROOT115_TOG,
    #[doc = "0xb990 - Miscellaneous Register"]
    pub misc115: MISC115,
    #[doc = "0xb994 - Miscellaneous Register"]
    pub misc_root115_set: MISC_ROOT115_SET,
    #[doc = "0xb998 - Miscellaneous Register"]
    pub misc_root115_clr: MISC_ROOT115_CLR,
    #[doc = "0xb99c - Miscellaneous Register"]
    pub misc_root115_tog: MISC_ROOT115_TOG,
    #[doc = "0xb9a0 - Post Divider Register"]
    pub post115: POST115,
    #[doc = "0xb9a4 - Post Divider Register"]
    pub post_root115_set: POST_ROOT115_SET,
    #[doc = "0xb9a8 - Post Divider Register"]
    pub post_root115_clr: POST_ROOT115_CLR,
    #[doc = "0xb9ac - Post Divider Register"]
    pub post_root115_tog: POST_ROOT115_TOG,
    #[doc = "0xb9b0 - Pre Divider Register"]
    pub pre115: PRE115,
    #[doc = "0xb9b4 - Pre Divider Register"]
    pub pre_root115_set: PRE_ROOT115_SET,
    #[doc = "0xb9b8 - Pre Divider Register"]
    pub pre_root115_clr: PRE_ROOT115_CLR,
    #[doc = "0xb9bc - Pre Divider Register"]
    pub pre_root115_tog: PRE_ROOT115_TOG,
    _reserved83: [u8; 48usize],
    #[doc = "0xb9f0 - Access Control Register"]
    pub access_ctrl115: ACCESS_CTRL115,
    #[doc = "0xb9f4 - Access Control Register"]
    pub access_ctrl_root115_set: ACCESS_CTRL_ROOT115_SET,
    #[doc = "0xb9f8 - Access Control Register"]
    pub access_ctrl_root115_clr: ACCESS_CTRL_ROOT115_CLR,
    #[doc = "0xb9fc - Access Control Register"]
    pub access_ctrl_root115_tog: ACCESS_CTRL_ROOT115_TOG,
    #[doc = "0xba00 - Target Register"]
    pub target_root116: TARGET_ROOT116,
    #[doc = "0xba04 - Target Register"]
    pub target_root116_set: TARGET_ROOT116_SET,
    #[doc = "0xba08 - Target Register"]
    pub target_root116_clr: TARGET_ROOT116_CLR,
    #[doc = "0xba0c - Target Register"]
    pub target_root116_tog: TARGET_ROOT116_TOG,
    #[doc = "0xba10 - Miscellaneous Register"]
    pub misc116: MISC116,
    #[doc = "0xba14 - Miscellaneous Register"]
    pub misc_root116_set: MISC_ROOT116_SET,
    #[doc = "0xba18 - Miscellaneous Register"]
    pub misc_root116_clr: MISC_ROOT116_CLR,
    #[doc = "0xba1c - Miscellaneous Register"]
    pub misc_root116_tog: MISC_ROOT116_TOG,
    #[doc = "0xba20 - Post Divider Register"]
    pub post116: POST116,
    #[doc = "0xba24 - Post Divider Register"]
    pub post_root116_set: POST_ROOT116_SET,
    #[doc = "0xba28 - Post Divider Register"]
    pub post_root116_clr: POST_ROOT116_CLR,
    #[doc = "0xba2c - Post Divider Register"]
    pub post_root116_tog: POST_ROOT116_TOG,
    #[doc = "0xba30 - Pre Divider Register"]
    pub pre116: PRE116,
    #[doc = "0xba34 - Pre Divider Register"]
    pub pre_root116_set: PRE_ROOT116_SET,
    #[doc = "0xba38 - Pre Divider Register"]
    pub pre_root116_clr: PRE_ROOT116_CLR,
    #[doc = "0xba3c - Pre Divider Register"]
    pub pre_root116_tog: PRE_ROOT116_TOG,
    _reserved84: [u8; 48usize],
    #[doc = "0xba70 - Access Control Register"]
    pub access_ctrl116: ACCESS_CTRL116,
    #[doc = "0xba74 - Access Control Register"]
    pub access_ctrl_root116_set: ACCESS_CTRL_ROOT116_SET,
    #[doc = "0xba78 - Access Control Register"]
    pub access_ctrl_root116_clr: ACCESS_CTRL_ROOT116_CLR,
    #[doc = "0xba7c - Access Control Register"]
    pub access_ctrl_root116_tog: ACCESS_CTRL_ROOT116_TOG,
    #[doc = "0xba80 - Target Register"]
    pub target_root117: TARGET_ROOT117,
    #[doc = "0xba84 - Target Register"]
    pub target_root117_set: TARGET_ROOT117_SET,
    #[doc = "0xba88 - Target Register"]
    pub target_root117_clr: TARGET_ROOT117_CLR,
    #[doc = "0xba8c - Target Register"]
    pub target_root117_tog: TARGET_ROOT117_TOG,
    #[doc = "0xba90 - Miscellaneous Register"]
    pub misc117: MISC117,
    #[doc = "0xba94 - Miscellaneous Register"]
    pub misc_root117_set: MISC_ROOT117_SET,
    #[doc = "0xba98 - Miscellaneous Register"]
    pub misc_root117_clr: MISC_ROOT117_CLR,
    #[doc = "0xba9c - Miscellaneous Register"]
    pub misc_root117_tog: MISC_ROOT117_TOG,
    #[doc = "0xbaa0 - Post Divider Register"]
    pub post117: POST117,
    #[doc = "0xbaa4 - Post Divider Register"]
    pub post_root117_set: POST_ROOT117_SET,
    #[doc = "0xbaa8 - Post Divider Register"]
    pub post_root117_clr: POST_ROOT117_CLR,
    #[doc = "0xbaac - Post Divider Register"]
    pub post_root117_tog: POST_ROOT117_TOG,
    #[doc = "0xbab0 - Pre Divider Register"]
    pub pre117: PRE117,
    #[doc = "0xbab4 - Pre Divider Register"]
    pub pre_root117_set: PRE_ROOT117_SET,
    #[doc = "0xbab8 - Pre Divider Register"]
    pub pre_root117_clr: PRE_ROOT117_CLR,
    #[doc = "0xbabc - Pre Divider Register"]
    pub pre_root117_tog: PRE_ROOT117_TOG,
    _reserved85: [u8; 48usize],
    #[doc = "0xbaf0 - Access Control Register"]
    pub access_ctrl117: ACCESS_CTRL117,
    #[doc = "0xbaf4 - Access Control Register"]
    pub access_ctrl_root117_set: ACCESS_CTRL_ROOT117_SET,
    #[doc = "0xbaf8 - Access Control Register"]
    pub access_ctrl_root117_clr: ACCESS_CTRL_ROOT117_CLR,
    #[doc = "0xbafc - Access Control Register"]
    pub access_ctrl_root117_tog: ACCESS_CTRL_ROOT117_TOG,
    #[doc = "0xbb00 - Target Register"]
    pub target_root118: TARGET_ROOT118,
    #[doc = "0xbb04 - Target Register"]
    pub target_root118_set: TARGET_ROOT118_SET,
    #[doc = "0xbb08 - Target Register"]
    pub target_root118_clr: TARGET_ROOT118_CLR,
    #[doc = "0xbb0c - Target Register"]
    pub target_root118_tog: TARGET_ROOT118_TOG,
    #[doc = "0xbb10 - Miscellaneous Register"]
    pub misc118: MISC118,
    #[doc = "0xbb14 - Miscellaneous Register"]
    pub misc_root118_set: MISC_ROOT118_SET,
    #[doc = "0xbb18 - Miscellaneous Register"]
    pub misc_root118_clr: MISC_ROOT118_CLR,
    #[doc = "0xbb1c - Miscellaneous Register"]
    pub misc_root118_tog: MISC_ROOT118_TOG,
    #[doc = "0xbb20 - Post Divider Register"]
    pub post118: POST118,
    #[doc = "0xbb24 - Post Divider Register"]
    pub post_root118_set: POST_ROOT118_SET,
    #[doc = "0xbb28 - Post Divider Register"]
    pub post_root118_clr: POST_ROOT118_CLR,
    #[doc = "0xbb2c - Post Divider Register"]
    pub post_root118_tog: POST_ROOT118_TOG,
    #[doc = "0xbb30 - Pre Divider Register"]
    pub pre118: PRE118,
    #[doc = "0xbb34 - Pre Divider Register"]
    pub pre_root118_set: PRE_ROOT118_SET,
    #[doc = "0xbb38 - Pre Divider Register"]
    pub pre_root118_clr: PRE_ROOT118_CLR,
    #[doc = "0xbb3c - Pre Divider Register"]
    pub pre_root118_tog: PRE_ROOT118_TOG,
    _reserved86: [u8; 48usize],
    #[doc = "0xbb70 - Access Control Register"]
    pub access_ctrl118: ACCESS_CTRL118,
    #[doc = "0xbb74 - Access Control Register"]
    pub access_ctrl_root118_set: ACCESS_CTRL_ROOT118_SET,
    #[doc = "0xbb78 - Access Control Register"]
    pub access_ctrl_root118_clr: ACCESS_CTRL_ROOT118_CLR,
    #[doc = "0xbb7c - Access Control Register"]
    pub access_ctrl_root118_tog: ACCESS_CTRL_ROOT118_TOG,
    #[doc = "0xbb80 - Target Register"]
    pub target_root119: TARGET_ROOT119,
    #[doc = "0xbb84 - Target Register"]
    pub target_root119_set: TARGET_ROOT119_SET,
    #[doc = "0xbb88 - Target Register"]
    pub target_root119_clr: TARGET_ROOT119_CLR,
    #[doc = "0xbb8c - Target Register"]
    pub target_root119_tog: TARGET_ROOT119_TOG,
    #[doc = "0xbb90 - Miscellaneous Register"]
    pub misc119: MISC119,
    #[doc = "0xbb94 - Miscellaneous Register"]
    pub misc_root119_set: MISC_ROOT119_SET,
    #[doc = "0xbb98 - Miscellaneous Register"]
    pub misc_root119_clr: MISC_ROOT119_CLR,
    #[doc = "0xbb9c - Miscellaneous Register"]
    pub misc_root119_tog: MISC_ROOT119_TOG,
    #[doc = "0xbba0 - Post Divider Register"]
    pub post119: POST119,
    #[doc = "0xbba4 - Post Divider Register"]
    pub post_root119_set: POST_ROOT119_SET,
    #[doc = "0xbba8 - Post Divider Register"]
    pub post_root119_clr: POST_ROOT119_CLR,
    #[doc = "0xbbac - Post Divider Register"]
    pub post_root119_tog: POST_ROOT119_TOG,
    #[doc = "0xbbb0 - Pre Divider Register"]
    pub pre119: PRE119,
    #[doc = "0xbbb4 - Pre Divider Register"]
    pub pre_root119_set: PRE_ROOT119_SET,
    #[doc = "0xbbb8 - Pre Divider Register"]
    pub pre_root119_clr: PRE_ROOT119_CLR,
    #[doc = "0xbbbc - Pre Divider Register"]
    pub pre_root119_tog: PRE_ROOT119_TOG,
    _reserved87: [u8; 48usize],
    #[doc = "0xbbf0 - Access Control Register"]
    pub access_ctrl119: ACCESS_CTRL119,
    #[doc = "0xbbf4 - Access Control Register"]
    pub access_ctrl_root119_set: ACCESS_CTRL_ROOT119_SET,
    #[doc = "0xbbf8 - Access Control Register"]
    pub access_ctrl_root119_clr: ACCESS_CTRL_ROOT119_CLR,
    #[doc = "0xbbfc - Access Control Register"]
    pub access_ctrl_root119_tog: ACCESS_CTRL_ROOT119_TOG,
    #[doc = "0xbc00 - Target Register"]
    pub target_root120: TARGET_ROOT120,
    #[doc = "0xbc04 - Target Register"]
    pub target_root120_set: TARGET_ROOT120_SET,
    #[doc = "0xbc08 - Target Register"]
    pub target_root120_clr: TARGET_ROOT120_CLR,
    #[doc = "0xbc0c - Target Register"]
    pub target_root120_tog: TARGET_ROOT120_TOG,
    #[doc = "0xbc10 - Miscellaneous Register"]
    pub misc120: MISC120,
    #[doc = "0xbc14 - Miscellaneous Register"]
    pub misc_root120_set: MISC_ROOT120_SET,
    #[doc = "0xbc18 - Miscellaneous Register"]
    pub misc_root120_clr: MISC_ROOT120_CLR,
    #[doc = "0xbc1c - Miscellaneous Register"]
    pub misc_root120_tog: MISC_ROOT120_TOG,
    #[doc = "0xbc20 - Post Divider Register"]
    pub post120: POST120,
    #[doc = "0xbc24 - Post Divider Register"]
    pub post_root120_set: POST_ROOT120_SET,
    #[doc = "0xbc28 - Post Divider Register"]
    pub post_root120_clr: POST_ROOT120_CLR,
    #[doc = "0xbc2c - Post Divider Register"]
    pub post_root120_tog: POST_ROOT120_TOG,
    #[doc = "0xbc30 - Pre Divider Register"]
    pub pre120: PRE120,
    #[doc = "0xbc34 - Pre Divider Register"]
    pub pre_root120_set: PRE_ROOT120_SET,
    #[doc = "0xbc38 - Pre Divider Register"]
    pub pre_root120_clr: PRE_ROOT120_CLR,
    #[doc = "0xbc3c - Pre Divider Register"]
    pub pre_root120_tog: PRE_ROOT120_TOG,
    _reserved88: [u8; 48usize],
    #[doc = "0xbc70 - Access Control Register"]
    pub access_ctrl120: ACCESS_CTRL120,
    #[doc = "0xbc74 - Access Control Register"]
    pub access_ctrl_root120_set: ACCESS_CTRL_ROOT120_SET,
    #[doc = "0xbc78 - Access Control Register"]
    pub access_ctrl_root120_clr: ACCESS_CTRL_ROOT120_CLR,
    #[doc = "0xbc7c - Access Control Register"]
    pub access_ctrl_root120_tog: ACCESS_CTRL_ROOT120_TOG,
    #[doc = "0xbc80 - Target Register"]
    pub target_root121: TARGET_ROOT121,
    #[doc = "0xbc84 - Target Register"]
    pub target_root121_set: TARGET_ROOT121_SET,
    #[doc = "0xbc88 - Target Register"]
    pub target_root121_clr: TARGET_ROOT121_CLR,
    #[doc = "0xbc8c - Target Register"]
    pub target_root121_tog: TARGET_ROOT121_TOG,
    #[doc = "0xbc90 - Miscellaneous Register"]
    pub misc121: MISC121,
    #[doc = "0xbc94 - Miscellaneous Register"]
    pub misc_root121_set: MISC_ROOT121_SET,
    #[doc = "0xbc98 - Miscellaneous Register"]
    pub misc_root121_clr: MISC_ROOT121_CLR,
    #[doc = "0xbc9c - Miscellaneous Register"]
    pub misc_root121_tog: MISC_ROOT121_TOG,
    #[doc = "0xbca0 - Post Divider Register"]
    pub post121: POST121,
    #[doc = "0xbca4 - Post Divider Register"]
    pub post_root121_set: POST_ROOT121_SET,
    #[doc = "0xbca8 - Post Divider Register"]
    pub post_root121_clr: POST_ROOT121_CLR,
    #[doc = "0xbcac - Post Divider Register"]
    pub post_root121_tog: POST_ROOT121_TOG,
    #[doc = "0xbcb0 - Pre Divider Register"]
    pub pre121: PRE121,
    #[doc = "0xbcb4 - Pre Divider Register"]
    pub pre_root121_set: PRE_ROOT121_SET,
    #[doc = "0xbcb8 - Pre Divider Register"]
    pub pre_root121_clr: PRE_ROOT121_CLR,
    #[doc = "0xbcbc - Pre Divider Register"]
    pub pre_root121_tog: PRE_ROOT121_TOG,
    _reserved89: [u8; 48usize],
    #[doc = "0xbcf0 - Access Control Register"]
    pub access_ctrl121: ACCESS_CTRL121,
    #[doc = "0xbcf4 - Access Control Register"]
    pub access_ctrl_root121_set: ACCESS_CTRL_ROOT121_SET,
    #[doc = "0xbcf8 - Access Control Register"]
    pub access_ctrl_root121_clr: ACCESS_CTRL_ROOT121_CLR,
    #[doc = "0xbcfc - Access Control Register"]
    pub access_ctrl_root121_tog: ACCESS_CTRL_ROOT121_TOG,
    #[doc = "0xbd00 - Target Register"]
    pub target_root122: TARGET_ROOT122,
    #[doc = "0xbd04 - Target Register"]
    pub target_root122_set: TARGET_ROOT122_SET,
    #[doc = "0xbd08 - Target Register"]
    pub target_root122_clr: TARGET_ROOT122_CLR,
    #[doc = "0xbd0c - Target Register"]
    pub target_root122_tog: TARGET_ROOT122_TOG,
    #[doc = "0xbd10 - Miscellaneous Register"]
    pub misc122: MISC122,
    #[doc = "0xbd14 - Miscellaneous Register"]
    pub misc_root122_set: MISC_ROOT122_SET,
    #[doc = "0xbd18 - Miscellaneous Register"]
    pub misc_root122_clr: MISC_ROOT122_CLR,
    #[doc = "0xbd1c - Miscellaneous Register"]
    pub misc_root122_tog: MISC_ROOT122_TOG,
    #[doc = "0xbd20 - Post Divider Register"]
    pub post122: POST122,
    #[doc = "0xbd24 - Post Divider Register"]
    pub post_root122_set: POST_ROOT122_SET,
    #[doc = "0xbd28 - Post Divider Register"]
    pub post_root122_clr: POST_ROOT122_CLR,
    #[doc = "0xbd2c - Post Divider Register"]
    pub post_root122_tog: POST_ROOT122_TOG,
    #[doc = "0xbd30 - Pre Divider Register"]
    pub pre122: PRE122,
    #[doc = "0xbd34 - Pre Divider Register"]
    pub pre_root122_set: PRE_ROOT122_SET,
    #[doc = "0xbd38 - Pre Divider Register"]
    pub pre_root122_clr: PRE_ROOT122_CLR,
    #[doc = "0xbd3c - Pre Divider Register"]
    pub pre_root122_tog: PRE_ROOT122_TOG,
    _reserved90: [u8; 48usize],
    #[doc = "0xbd70 - Access Control Register"]
    pub access_ctrl122: ACCESS_CTRL122,
    #[doc = "0xbd74 - Access Control Register"]
    pub access_ctrl_root122_set: ACCESS_CTRL_ROOT122_SET,
    #[doc = "0xbd78 - Access Control Register"]
    pub access_ctrl_root122_clr: ACCESS_CTRL_ROOT122_CLR,
    #[doc = "0xbd7c - Access Control Register"]
    pub access_ctrl_root122_tog: ACCESS_CTRL_ROOT122_TOG,
    #[doc = "0xbd80 - Target Register"]
    pub target_root123: TARGET_ROOT123,
    #[doc = "0xbd84 - Target Register"]
    pub target_root123_set: TARGET_ROOT123_SET,
    #[doc = "0xbd88 - Target Register"]
    pub target_root123_clr: TARGET_ROOT123_CLR,
    #[doc = "0xbd8c - Target Register"]
    pub target_root123_tog: TARGET_ROOT123_TOG,
    #[doc = "0xbd90 - Miscellaneous Register"]
    pub misc123: MISC123,
    #[doc = "0xbd94 - Miscellaneous Register"]
    pub misc_root123_set: MISC_ROOT123_SET,
    #[doc = "0xbd98 - Miscellaneous Register"]
    pub misc_root123_clr: MISC_ROOT123_CLR,
    #[doc = "0xbd9c - Miscellaneous Register"]
    pub misc_root123_tog: MISC_ROOT123_TOG,
    #[doc = "0xbda0 - Post Divider Register"]
    pub post123: POST123,
    #[doc = "0xbda4 - Post Divider Register"]
    pub post_root123_set: POST_ROOT123_SET,
    #[doc = "0xbda8 - Post Divider Register"]
    pub post_root123_clr: POST_ROOT123_CLR,
    #[doc = "0xbdac - Post Divider Register"]
    pub post_root123_tog: POST_ROOT123_TOG,
    #[doc = "0xbdb0 - Pre Divider Register"]
    pub pre123: PRE123,
    #[doc = "0xbdb4 - Pre Divider Register"]
    pub pre_root123_set: PRE_ROOT123_SET,
    #[doc = "0xbdb8 - Pre Divider Register"]
    pub pre_root123_clr: PRE_ROOT123_CLR,
    #[doc = "0xbdbc - Pre Divider Register"]
    pub pre_root123_tog: PRE_ROOT123_TOG,
    _reserved91: [u8; 48usize],
    #[doc = "0xbdf0 - Access Control Register"]
    pub access_ctrl123: ACCESS_CTRL123,
    #[doc = "0xbdf4 - Access Control Register"]
    pub access_ctrl_root123_set: ACCESS_CTRL_ROOT123_SET,
    #[doc = "0xbdf8 - Access Control Register"]
    pub access_ctrl_root123_clr: ACCESS_CTRL_ROOT123_CLR,
    #[doc = "0xbdfc - Access Control Register"]
    pub access_ctrl_root123_tog: ACCESS_CTRL_ROOT123_TOG,
    #[doc = "0xbe00 - Target Register"]
    pub target_root124: TARGET_ROOT124,
    #[doc = "0xbe04 - Target Register"]
    pub target_root124_set: TARGET_ROOT124_SET,
    #[doc = "0xbe08 - Target Register"]
    pub target_root124_clr: TARGET_ROOT124_CLR,
    #[doc = "0xbe0c - Target Register"]
    pub target_root124_tog: TARGET_ROOT124_TOG,
    #[doc = "0xbe10 - Miscellaneous Register"]
    pub misc124: MISC124,
    #[doc = "0xbe14 - Miscellaneous Register"]
    pub misc_root124_set: MISC_ROOT124_SET,
    #[doc = "0xbe18 - Miscellaneous Register"]
    pub misc_root124_clr: MISC_ROOT124_CLR,
    #[doc = "0xbe1c - Miscellaneous Register"]
    pub misc_root124_tog: MISC_ROOT124_TOG,
    #[doc = "0xbe20 - Post Divider Register"]
    pub post124: POST124,
    #[doc = "0xbe24 - Post Divider Register"]
    pub post_root124_set: POST_ROOT124_SET,
    #[doc = "0xbe28 - Post Divider Register"]
    pub post_root124_clr: POST_ROOT124_CLR,
    #[doc = "0xbe2c - Post Divider Register"]
    pub post_root124_tog: POST_ROOT124_TOG,
    #[doc = "0xbe30 - Pre Divider Register"]
    pub pre124: PRE124,
    #[doc = "0xbe34 - Pre Divider Register"]
    pub pre_root124_set: PRE_ROOT124_SET,
    #[doc = "0xbe38 - Pre Divider Register"]
    pub pre_root124_clr: PRE_ROOT124_CLR,
    #[doc = "0xbe3c - Pre Divider Register"]
    pub pre_root124_tog: PRE_ROOT124_TOG,
    _reserved92: [u8; 48usize],
    #[doc = "0xbe70 - Access Control Register"]
    pub access_ctrl124: ACCESS_CTRL124,
    #[doc = "0xbe74 - Access Control Register"]
    pub access_ctrl_root124_set: ACCESS_CTRL_ROOT124_SET,
    #[doc = "0xbe78 - Access Control Register"]
    pub access_ctrl_root124_clr: ACCESS_CTRL_ROOT124_CLR,
    #[doc = "0xbe7c - Access Control Register"]
    pub access_ctrl_root124_tog: ACCESS_CTRL_ROOT124_TOG,
    #[doc = "0xbe80 - Target Register"]
    pub target_root125: TARGET_ROOT125,
    #[doc = "0xbe84 - Target Register"]
    pub target_root125_set: TARGET_ROOT125_SET,
    #[doc = "0xbe88 - Target Register"]
    pub target_root125_clr: TARGET_ROOT125_CLR,
    #[doc = "0xbe8c - Target Register"]
    pub target_root125_tog: TARGET_ROOT125_TOG,
    #[doc = "0xbe90 - Miscellaneous Register"]
    pub misc125: MISC125,
    #[doc = "0xbe94 - Miscellaneous Register"]
    pub misc_root125_set: MISC_ROOT125_SET,
    #[doc = "0xbe98 - Miscellaneous Register"]
    pub misc_root125_clr: MISC_ROOT125_CLR,
    #[doc = "0xbe9c - Miscellaneous Register"]
    pub misc_root125_tog: MISC_ROOT125_TOG,
    #[doc = "0xbea0 - Post Divider Register"]
    pub post125: POST125,
    #[doc = "0xbea4 - Post Divider Register"]
    pub post_root125_set: POST_ROOT125_SET,
    #[doc = "0xbea8 - Post Divider Register"]
    pub post_root125_clr: POST_ROOT125_CLR,
    #[doc = "0xbeac - Post Divider Register"]
    pub post_root125_tog: POST_ROOT125_TOG,
    #[doc = "0xbeb0 - Pre Divider Register"]
    pub pre125: PRE125,
    #[doc = "0xbeb4 - Pre Divider Register"]
    pub pre_root125_set: PRE_ROOT125_SET,
    #[doc = "0xbeb8 - Pre Divider Register"]
    pub pre_root125_clr: PRE_ROOT125_CLR,
    #[doc = "0xbebc - Pre Divider Register"]
    pub pre_root125_tog: PRE_ROOT125_TOG,
    _reserved93: [u8; 48usize],
    #[doc = "0xbef0 - Access Control Register"]
    pub access_ctrl125: ACCESS_CTRL125,
    #[doc = "0xbef4 - Access Control Register"]
    pub access_ctrl_root125_set: ACCESS_CTRL_ROOT125_SET,
    #[doc = "0xbef8 - Access Control Register"]
    pub access_ctrl_root125_clr: ACCESS_CTRL_ROOT125_CLR,
    #[doc = "0xbefc - Access Control Register"]
    pub access_ctrl_root125_tog: ACCESS_CTRL_ROOT125_TOG,
    #[doc = "0xbf00 - Target Register"]
    pub target_root126: TARGET_ROOT126,
    #[doc = "0xbf04 - Target Register"]
    pub target_root126_set: TARGET_ROOT126_SET,
    #[doc = "0xbf08 - Target Register"]
    pub target_root126_clr: TARGET_ROOT126_CLR,
    #[doc = "0xbf0c - Target Register"]
    pub target_root126_tog: TARGET_ROOT126_TOG,
    #[doc = "0xbf10 - Miscellaneous Register"]
    pub misc126: MISC126,
    #[doc = "0xbf14 - Miscellaneous Register"]
    pub misc_root126_set: MISC_ROOT126_SET,
    #[doc = "0xbf18 - Miscellaneous Register"]
    pub misc_root126_clr: MISC_ROOT126_CLR,
    #[doc = "0xbf1c - Miscellaneous Register"]
    pub misc_root126_tog: MISC_ROOT126_TOG,
    #[doc = "0xbf20 - Post Divider Register"]
    pub post126: POST126,
    #[doc = "0xbf24 - Post Divider Register"]
    pub post_root126_set: POST_ROOT126_SET,
    #[doc = "0xbf28 - Post Divider Register"]
    pub post_root126_clr: POST_ROOT126_CLR,
    #[doc = "0xbf2c - Post Divider Register"]
    pub post_root126_tog: POST_ROOT126_TOG,
    #[doc = "0xbf30 - Pre Divider Register"]
    pub pre126: PRE126,
    #[doc = "0xbf34 - Pre Divider Register"]
    pub pre_root126_set: PRE_ROOT126_SET,
    #[doc = "0xbf38 - Pre Divider Register"]
    pub pre_root126_clr: PRE_ROOT126_CLR,
    #[doc = "0xbf3c - Pre Divider Register"]
    pub pre_root126_tog: PRE_ROOT126_TOG,
    _reserved94: [u8; 48usize],
    #[doc = "0xbf70 - Access Control Register"]
    pub access_ctrl126: ACCESS_CTRL126,
    #[doc = "0xbf74 - Access Control Register"]
    pub access_ctrl_root126_set: ACCESS_CTRL_ROOT126_SET,
    #[doc = "0xbf78 - Access Control Register"]
    pub access_ctrl_root126_clr: ACCESS_CTRL_ROOT126_CLR,
    #[doc = "0xbf7c - Access Control Register"]
    pub access_ctrl_root126_tog: ACCESS_CTRL_ROOT126_TOG,
    #[doc = "0xbf80 - Target Register"]
    pub target_root127: TARGET_ROOT127,
    #[doc = "0xbf84 - Target Register"]
    pub target_root127_set: TARGET_ROOT127_SET,
    #[doc = "0xbf88 - Target Register"]
    pub target_root127_clr: TARGET_ROOT127_CLR,
    #[doc = "0xbf8c - Target Register"]
    pub target_root127_tog: TARGET_ROOT127_TOG,
    #[doc = "0xbf90 - Miscellaneous Register"]
    pub misc127: MISC127,
    #[doc = "0xbf94 - Miscellaneous Register"]
    pub misc_root127_set: MISC_ROOT127_SET,
    #[doc = "0xbf98 - Miscellaneous Register"]
    pub misc_root127_clr: MISC_ROOT127_CLR,
    #[doc = "0xbf9c - Miscellaneous Register"]
    pub misc_root127_tog: MISC_ROOT127_TOG,
    #[doc = "0xbfa0 - Post Divider Register"]
    pub post127: POST127,
    #[doc = "0xbfa4 - Post Divider Register"]
    pub post_root127_set: POST_ROOT127_SET,
    #[doc = "0xbfa8 - Post Divider Register"]
    pub post_root127_clr: POST_ROOT127_CLR,
    #[doc = "0xbfac - Post Divider Register"]
    pub post_root127_tog: POST_ROOT127_TOG,
    #[doc = "0xbfb0 - Pre Divider Register"]
    pub pre127: PRE127,
    #[doc = "0xbfb4 - Pre Divider Register"]
    pub pre_root127_set: PRE_ROOT127_SET,
    #[doc = "0xbfb8 - Pre Divider Register"]
    pub pre_root127_clr: PRE_ROOT127_CLR,
    #[doc = "0xbfbc - Pre Divider Register"]
    pub pre_root127_tog: PRE_ROOT127_TOG,
    _reserved95: [u8; 48usize],
    #[doc = "0xbff0 - Access Control Register"]
    pub access_ctrl127: ACCESS_CTRL127,
    #[doc = "0xbff4 - Access Control Register"]
    pub access_ctrl_root127_set: ACCESS_CTRL_ROOT127_SET,
    #[doc = "0xbff8 - Access Control Register"]
    pub access_ctrl_root127_clr: ACCESS_CTRL_ROOT127_CLR,
    #[doc = "0xbffc - Access Control Register"]
    pub access_ctrl_root127_tog: ACCESS_CTRL_ROOT127_TOG,
    #[doc = "0xc000 - Target Register"]
    pub target_root128: TARGET_ROOT128,
    #[doc = "0xc004 - Target Register"]
    pub target_root128_set: TARGET_ROOT128_SET,
    #[doc = "0xc008 - Target Register"]
    pub target_root128_clr: TARGET_ROOT128_CLR,
    #[doc = "0xc00c - Target Register"]
    pub target_root128_tog: TARGET_ROOT128_TOG,
    #[doc = "0xc010 - Miscellaneous Register"]
    pub misc128: MISC128,
    #[doc = "0xc014 - Miscellaneous Register"]
    pub misc_root128_set: MISC_ROOT128_SET,
    #[doc = "0xc018 - Miscellaneous Register"]
    pub misc_root128_clr: MISC_ROOT128_CLR,
    #[doc = "0xc01c - Miscellaneous Register"]
    pub misc_root128_tog: MISC_ROOT128_TOG,
    #[doc = "0xc020 - Post Divider Register"]
    pub post128: POST128,
    #[doc = "0xc024 - Post Divider Register"]
    pub post_root128_set: POST_ROOT128_SET,
    #[doc = "0xc028 - Post Divider Register"]
    pub post_root128_clr: POST_ROOT128_CLR,
    #[doc = "0xc02c - Post Divider Register"]
    pub post_root128_tog: POST_ROOT128_TOG,
    #[doc = "0xc030 - Pre Divider Register"]
    pub pre128: PRE128,
    #[doc = "0xc034 - Pre Divider Register"]
    pub pre_root128_set: PRE_ROOT128_SET,
    #[doc = "0xc038 - Pre Divider Register"]
    pub pre_root128_clr: PRE_ROOT128_CLR,
    #[doc = "0xc03c - Pre Divider Register"]
    pub pre_root128_tog: PRE_ROOT128_TOG,
    _reserved96: [u8; 48usize],
    #[doc = "0xc070 - Access Control Register"]
    pub access_ctrl128: ACCESS_CTRL128,
    #[doc = "0xc074 - Access Control Register"]
    pub access_ctrl_root128_set: ACCESS_CTRL_ROOT128_SET,
    #[doc = "0xc078 - Access Control Register"]
    pub access_ctrl_root128_clr: ACCESS_CTRL_ROOT128_CLR,
    #[doc = "0xc07c - Access Control Register"]
    pub access_ctrl_root128_tog: ACCESS_CTRL_ROOT128_TOG,
    #[doc = "0xc080 - Target Register"]
    pub target_root129: TARGET_ROOT129,
    #[doc = "0xc084 - Target Register"]
    pub target_root129_set: TARGET_ROOT129_SET,
    #[doc = "0xc088 - Target Register"]
    pub target_root129_clr: TARGET_ROOT129_CLR,
    #[doc = "0xc08c - Target Register"]
    pub target_root129_tog: TARGET_ROOT129_TOG,
    #[doc = "0xc090 - Miscellaneous Register"]
    pub misc129: MISC129,
    #[doc = "0xc094 - Miscellaneous Register"]
    pub misc_root129_set: MISC_ROOT129_SET,
    #[doc = "0xc098 - Miscellaneous Register"]
    pub misc_root129_clr: MISC_ROOT129_CLR,
    #[doc = "0xc09c - Miscellaneous Register"]
    pub misc_root129_tog: MISC_ROOT129_TOG,
    #[doc = "0xc0a0 - Post Divider Register"]
    pub post129: POST129,
    #[doc = "0xc0a4 - Post Divider Register"]
    pub post_root129_set: POST_ROOT129_SET,
    #[doc = "0xc0a8 - Post Divider Register"]
    pub post_root129_clr: POST_ROOT129_CLR,
    #[doc = "0xc0ac - Post Divider Register"]
    pub post_root129_tog: POST_ROOT129_TOG,
    #[doc = "0xc0b0 - Pre Divider Register"]
    pub pre129: PRE129,
    #[doc = "0xc0b4 - Pre Divider Register"]
    pub pre_root129_set: PRE_ROOT129_SET,
    #[doc = "0xc0b8 - Pre Divider Register"]
    pub pre_root129_clr: PRE_ROOT129_CLR,
    #[doc = "0xc0bc - Pre Divider Register"]
    pub pre_root129_tog: PRE_ROOT129_TOG,
    _reserved97: [u8; 48usize],
    #[doc = "0xc0f0 - Access Control Register"]
    pub access_ctrl129: ACCESS_CTRL129,
    #[doc = "0xc0f4 - Access Control Register"]
    pub access_ctrl_root129_set: ACCESS_CTRL_ROOT129_SET,
    #[doc = "0xc0f8 - Access Control Register"]
    pub access_ctrl_root129_clr: ACCESS_CTRL_ROOT129_CLR,
    #[doc = "0xc0fc - Access Control Register"]
    pub access_ctrl_root129_tog: ACCESS_CTRL_ROOT129_TOG,
    #[doc = "0xc100 - Target Register"]
    pub target_root130: TARGET_ROOT130,
    #[doc = "0xc104 - Target Register"]
    pub target_root130_set: TARGET_ROOT130_SET,
    #[doc = "0xc108 - Target Register"]
    pub target_root130_clr: TARGET_ROOT130_CLR,
    #[doc = "0xc10c - Target Register"]
    pub target_root130_tog: TARGET_ROOT130_TOG,
    #[doc = "0xc110 - Miscellaneous Register"]
    pub misc130: MISC130,
    #[doc = "0xc114 - Miscellaneous Register"]
    pub misc_root130_set: MISC_ROOT130_SET,
    #[doc = "0xc118 - Miscellaneous Register"]
    pub misc_root130_clr: MISC_ROOT130_CLR,
    #[doc = "0xc11c - Miscellaneous Register"]
    pub misc_root130_tog: MISC_ROOT130_TOG,
    #[doc = "0xc120 - Post Divider Register"]
    pub post130: POST130,
    #[doc = "0xc124 - Post Divider Register"]
    pub post_root130_set: POST_ROOT130_SET,
    #[doc = "0xc128 - Post Divider Register"]
    pub post_root130_clr: POST_ROOT130_CLR,
    #[doc = "0xc12c - Post Divider Register"]
    pub post_root130_tog: POST_ROOT130_TOG,
    #[doc = "0xc130 - Pre Divider Register"]
    pub pre130: PRE130,
    #[doc = "0xc134 - Pre Divider Register"]
    pub pre_root130_set: PRE_ROOT130_SET,
    #[doc = "0xc138 - Pre Divider Register"]
    pub pre_root130_clr: PRE_ROOT130_CLR,
    #[doc = "0xc13c - Pre Divider Register"]
    pub pre_root130_tog: PRE_ROOT130_TOG,
    _reserved98: [u8; 48usize],
    #[doc = "0xc170 - Access Control Register"]
    pub access_ctrl130: ACCESS_CTRL130,
    #[doc = "0xc174 - Access Control Register"]
    pub access_ctrl_root130_set: ACCESS_CTRL_ROOT130_SET,
    #[doc = "0xc178 - Access Control Register"]
    pub access_ctrl_root130_clr: ACCESS_CTRL_ROOT130_CLR,
    #[doc = "0xc17c - Access Control Register"]
    pub access_ctrl_root130_tog: ACCESS_CTRL_ROOT130_TOG,
    #[doc = "0xc180 - Target Register"]
    pub target_root131: TARGET_ROOT131,
    #[doc = "0xc184 - Target Register"]
    pub target_root131_set: TARGET_ROOT131_SET,
    #[doc = "0xc188 - Target Register"]
    pub target_root131_clr: TARGET_ROOT131_CLR,
    #[doc = "0xc18c - Target Register"]
    pub target_root131_tog: TARGET_ROOT131_TOG,
    #[doc = "0xc190 - Miscellaneous Register"]
    pub misc131: MISC131,
    #[doc = "0xc194 - Miscellaneous Register"]
    pub misc_root131_set: MISC_ROOT131_SET,
    #[doc = "0xc198 - Miscellaneous Register"]
    pub misc_root131_clr: MISC_ROOT131_CLR,
    #[doc = "0xc19c - Miscellaneous Register"]
    pub misc_root131_tog: MISC_ROOT131_TOG,
    #[doc = "0xc1a0 - Post Divider Register"]
    pub post131: POST131,
    #[doc = "0xc1a4 - Post Divider Register"]
    pub post_root131_set: POST_ROOT131_SET,
    #[doc = "0xc1a8 - Post Divider Register"]
    pub post_root131_clr: POST_ROOT131_CLR,
    #[doc = "0xc1ac - Post Divider Register"]
    pub post_root131_tog: POST_ROOT131_TOG,
    #[doc = "0xc1b0 - Pre Divider Register"]
    pub pre131: PRE131,
    #[doc = "0xc1b4 - Pre Divider Register"]
    pub pre_root131_set: PRE_ROOT131_SET,
    #[doc = "0xc1b8 - Pre Divider Register"]
    pub pre_root131_clr: PRE_ROOT131_CLR,
    #[doc = "0xc1bc - Pre Divider Register"]
    pub pre_root131_tog: PRE_ROOT131_TOG,
    _reserved99: [u8; 48usize],
    #[doc = "0xc1f0 - Access Control Register"]
    pub access_ctrl131: ACCESS_CTRL131,
    #[doc = "0xc1f4 - Access Control Register"]
    pub access_ctrl_root131_set: ACCESS_CTRL_ROOT131_SET,
    #[doc = "0xc1f8 - Access Control Register"]
    pub access_ctrl_root131_clr: ACCESS_CTRL_ROOT131_CLR,
    #[doc = "0xc1fc - Access Control Register"]
    pub access_ctrl_root131_tog: ACCESS_CTRL_ROOT131_TOG,
    #[doc = "0xc200 - Target Register"]
    pub target_root132: TARGET_ROOT132,
    #[doc = "0xc204 - Target Register"]
    pub target_root132_set: TARGET_ROOT132_SET,
    #[doc = "0xc208 - Target Register"]
    pub target_root132_clr: TARGET_ROOT132_CLR,
    #[doc = "0xc20c - Target Register"]
    pub target_root132_tog: TARGET_ROOT132_TOG,
    #[doc = "0xc210 - Miscellaneous Register"]
    pub misc132: MISC132,
    #[doc = "0xc214 - Miscellaneous Register"]
    pub misc_root132_set: MISC_ROOT132_SET,
    #[doc = "0xc218 - Miscellaneous Register"]
    pub misc_root132_clr: MISC_ROOT132_CLR,
    #[doc = "0xc21c - Miscellaneous Register"]
    pub misc_root132_tog: MISC_ROOT132_TOG,
    #[doc = "0xc220 - Post Divider Register"]
    pub post132: POST132,
    #[doc = "0xc224 - Post Divider Register"]
    pub post_root132_set: POST_ROOT132_SET,
    #[doc = "0xc228 - Post Divider Register"]
    pub post_root132_clr: POST_ROOT132_CLR,
    #[doc = "0xc22c - Post Divider Register"]
    pub post_root132_tog: POST_ROOT132_TOG,
    #[doc = "0xc230 - Pre Divider Register"]
    pub pre132: PRE132,
    #[doc = "0xc234 - Pre Divider Register"]
    pub pre_root132_set: PRE_ROOT132_SET,
    #[doc = "0xc238 - Pre Divider Register"]
    pub pre_root132_clr: PRE_ROOT132_CLR,
    #[doc = "0xc23c - Pre Divider Register"]
    pub pre_root132_tog: PRE_ROOT132_TOG,
    _reserved100: [u8; 48usize],
    #[doc = "0xc270 - Access Control Register"]
    pub access_ctrl132: ACCESS_CTRL132,
    #[doc = "0xc274 - Access Control Register"]
    pub access_ctrl_root132_set: ACCESS_CTRL_ROOT132_SET,
    #[doc = "0xc278 - Access Control Register"]
    pub access_ctrl_root132_clr: ACCESS_CTRL_ROOT132_CLR,
    #[doc = "0xc27c - Access Control Register"]
    pub access_ctrl_root132_tog: ACCESS_CTRL_ROOT132_TOG,
    #[doc = "0xc280 - Target Register"]
    pub target_root133: TARGET_ROOT133,
    #[doc = "0xc284 - Target Register"]
    pub target_root133_set: TARGET_ROOT133_SET,
    #[doc = "0xc288 - Target Register"]
    pub target_root133_clr: TARGET_ROOT133_CLR,
    #[doc = "0xc28c - Target Register"]
    pub target_root133_tog: TARGET_ROOT133_TOG,
    #[doc = "0xc290 - Miscellaneous Register"]
    pub misc133: MISC133,
    #[doc = "0xc294 - Miscellaneous Register"]
    pub misc_root133_set: MISC_ROOT133_SET,
    #[doc = "0xc298 - Miscellaneous Register"]
    pub misc_root133_clr: MISC_ROOT133_CLR,
    #[doc = "0xc29c - Miscellaneous Register"]
    pub misc_root133_tog: MISC_ROOT133_TOG,
    #[doc = "0xc2a0 - Post Divider Register"]
    pub post133: POST133,
    #[doc = "0xc2a4 - Post Divider Register"]
    pub post_root133_set: POST_ROOT133_SET,
    #[doc = "0xc2a8 - Post Divider Register"]
    pub post_root133_clr: POST_ROOT133_CLR,
    #[doc = "0xc2ac - Post Divider Register"]
    pub post_root133_tog: POST_ROOT133_TOG,
    #[doc = "0xc2b0 - Pre Divider Register"]
    pub pre133: PRE133,
    #[doc = "0xc2b4 - Pre Divider Register"]
    pub pre_root133_set: PRE_ROOT133_SET,
    #[doc = "0xc2b8 - Pre Divider Register"]
    pub pre_root133_clr: PRE_ROOT133_CLR,
    #[doc = "0xc2bc - Pre Divider Register"]
    pub pre_root133_tog: PRE_ROOT133_TOG,
    _reserved101: [u8; 48usize],
    #[doc = "0xc2f0 - Access Control Register"]
    pub access_ctrl133: ACCESS_CTRL133,
    #[doc = "0xc2f4 - Access Control Register"]
    pub access_ctrl_root133_set: ACCESS_CTRL_ROOT133_SET,
    #[doc = "0xc2f8 - Access Control Register"]
    pub access_ctrl_root133_clr: ACCESS_CTRL_ROOT133_CLR,
    #[doc = "0xc2fc - Access Control Register"]
    pub access_ctrl_root133_tog: ACCESS_CTRL_ROOT133_TOG,
    #[doc = "0xc300 - Target Register"]
    pub target_root134: TARGET_ROOT134,
    #[doc = "0xc304 - Target Register"]
    pub target_root134_set: TARGET_ROOT134_SET,
    #[doc = "0xc308 - Target Register"]
    pub target_root134_clr: TARGET_ROOT134_CLR,
    #[doc = "0xc30c - Target Register"]
    pub target_root134_tog: TARGET_ROOT134_TOG,
    #[doc = "0xc310 - Miscellaneous Register"]
    pub misc134: MISC134,
    #[doc = "0xc314 - Miscellaneous Register"]
    pub misc_root134_set: MISC_ROOT134_SET,
    #[doc = "0xc318 - Miscellaneous Register"]
    pub misc_root134_clr: MISC_ROOT134_CLR,
    #[doc = "0xc31c - Miscellaneous Register"]
    pub misc_root134_tog: MISC_ROOT134_TOG,
    #[doc = "0xc320 - Post Divider Register"]
    pub post134: POST134,
    #[doc = "0xc324 - Post Divider Register"]
    pub post_root134_set: POST_ROOT134_SET,
    #[doc = "0xc328 - Post Divider Register"]
    pub post_root134_clr: POST_ROOT134_CLR,
    #[doc = "0xc32c - Post Divider Register"]
    pub post_root134_tog: POST_ROOT134_TOG,
    #[doc = "0xc330 - Pre Divider Register"]
    pub pre134: PRE134,
    #[doc = "0xc334 - Pre Divider Register"]
    pub pre_root134_set: PRE_ROOT134_SET,
    #[doc = "0xc338 - Pre Divider Register"]
    pub pre_root134_clr: PRE_ROOT134_CLR,
    #[doc = "0xc33c - Pre Divider Register"]
    pub pre_root134_tog: PRE_ROOT134_TOG,
    _reserved102: [u8; 48usize],
    #[doc = "0xc370 - Access Control Register"]
    pub access_ctrl134: ACCESS_CTRL134,
    #[doc = "0xc374 - Access Control Register"]
    pub access_ctrl_root134_set: ACCESS_CTRL_ROOT134_SET,
    #[doc = "0xc378 - Access Control Register"]
    pub access_ctrl_root134_clr: ACCESS_CTRL_ROOT134_CLR,
    #[doc = "0xc37c - Access Control Register"]
    pub access_ctrl_root134_tog: ACCESS_CTRL_ROOT134_TOG,
    #[doc = "0xc380 - Target Register"]
    pub target_root135: TARGET_ROOT135,
    #[doc = "0xc384 - Target Register"]
    pub target_root135_set: TARGET_ROOT135_SET,
    #[doc = "0xc388 - Target Register"]
    pub target_root135_clr: TARGET_ROOT135_CLR,
    #[doc = "0xc38c - Target Register"]
    pub target_root135_tog: TARGET_ROOT135_TOG,
    #[doc = "0xc390 - Miscellaneous Register"]
    pub misc135: MISC135,
    #[doc = "0xc394 - Miscellaneous Register"]
    pub misc_root135_set: MISC_ROOT135_SET,
    #[doc = "0xc398 - Miscellaneous Register"]
    pub misc_root135_clr: MISC_ROOT135_CLR,
    #[doc = "0xc39c - Miscellaneous Register"]
    pub misc_root135_tog: MISC_ROOT135_TOG,
    #[doc = "0xc3a0 - Post Divider Register"]
    pub post135: POST135,
    #[doc = "0xc3a4 - Post Divider Register"]
    pub post_root135_set: POST_ROOT135_SET,
    #[doc = "0xc3a8 - Post Divider Register"]
    pub post_root135_clr: POST_ROOT135_CLR,
    #[doc = "0xc3ac - Post Divider Register"]
    pub post_root135_tog: POST_ROOT135_TOG,
    #[doc = "0xc3b0 - Pre Divider Register"]
    pub pre135: PRE135,
    #[doc = "0xc3b4 - Pre Divider Register"]
    pub pre_root135_set: PRE_ROOT135_SET,
    #[doc = "0xc3b8 - Pre Divider Register"]
    pub pre_root135_clr: PRE_ROOT135_CLR,
    #[doc = "0xc3bc - Pre Divider Register"]
    pub pre_root135_tog: PRE_ROOT135_TOG,
    _reserved103: [u8; 48usize],
    #[doc = "0xc3f0 - Access Control Register"]
    pub access_ctrl135: ACCESS_CTRL135,
    #[doc = "0xc3f4 - Access Control Register"]
    pub access_ctrl_root135_set: ACCESS_CTRL_ROOT135_SET,
    #[doc = "0xc3f8 - Access Control Register"]
    pub access_ctrl_root135_clr: ACCESS_CTRL_ROOT135_CLR,
    #[doc = "0xc3fc - Access Control Register"]
    pub access_ctrl_root135_tog: ACCESS_CTRL_ROOT135_TOG,
    #[doc = "0xc400 - Target Register"]
    pub target_root136: TARGET_ROOT136,
    #[doc = "0xc404 - Target Register"]
    pub target_root136_set: TARGET_ROOT136_SET,
    #[doc = "0xc408 - Target Register"]
    pub target_root136_clr: TARGET_ROOT136_CLR,
    #[doc = "0xc40c - Target Register"]
    pub target_root136_tog: TARGET_ROOT136_TOG,
    #[doc = "0xc410 - Miscellaneous Register"]
    pub misc136: MISC136,
    #[doc = "0xc414 - Miscellaneous Register"]
    pub misc_root136_set: MISC_ROOT136_SET,
    #[doc = "0xc418 - Miscellaneous Register"]
    pub misc_root136_clr: MISC_ROOT136_CLR,
    #[doc = "0xc41c - Miscellaneous Register"]
    pub misc_root136_tog: MISC_ROOT136_TOG,
    #[doc = "0xc420 - Post Divider Register"]
    pub post136: POST136,
    #[doc = "0xc424 - Post Divider Register"]
    pub post_root136_set: POST_ROOT136_SET,
    #[doc = "0xc428 - Post Divider Register"]
    pub post_root136_clr: POST_ROOT136_CLR,
    #[doc = "0xc42c - Post Divider Register"]
    pub post_root136_tog: POST_ROOT136_TOG,
    #[doc = "0xc430 - Pre Divider Register"]
    pub pre136: PRE136,
    #[doc = "0xc434 - Pre Divider Register"]
    pub pre_root136_set: PRE_ROOT136_SET,
    #[doc = "0xc438 - Pre Divider Register"]
    pub pre_root136_clr: PRE_ROOT136_CLR,
    #[doc = "0xc43c - Pre Divider Register"]
    pub pre_root136_tog: PRE_ROOT136_TOG,
    _reserved104: [u8; 48usize],
    #[doc = "0xc470 - Access Control Register"]
    pub access_ctrl136: ACCESS_CTRL136,
    #[doc = "0xc474 - Access Control Register"]
    pub access_ctrl_root136_set: ACCESS_CTRL_ROOT136_SET,
    #[doc = "0xc478 - Access Control Register"]
    pub access_ctrl_root136_clr: ACCESS_CTRL_ROOT136_CLR,
    #[doc = "0xc47c - Access Control Register"]
    pub access_ctrl_root136_tog: ACCESS_CTRL_ROOT136_TOG,
    #[doc = "0xc480 - Target Register"]
    pub target_root137: TARGET_ROOT137,
    #[doc = "0xc484 - Target Register"]
    pub target_root137_set: TARGET_ROOT137_SET,
    #[doc = "0xc488 - Target Register"]
    pub target_root137_clr: TARGET_ROOT137_CLR,
    #[doc = "0xc48c - Target Register"]
    pub target_root137_tog: TARGET_ROOT137_TOG,
    #[doc = "0xc490 - Miscellaneous Register"]
    pub misc137: MISC137,
    #[doc = "0xc494 - Miscellaneous Register"]
    pub misc_root137_set: MISC_ROOT137_SET,
    #[doc = "0xc498 - Miscellaneous Register"]
    pub misc_root137_clr: MISC_ROOT137_CLR,
    #[doc = "0xc49c - Miscellaneous Register"]
    pub misc_root137_tog: MISC_ROOT137_TOG,
    #[doc = "0xc4a0 - Post Divider Register"]
    pub post137: POST137,
    #[doc = "0xc4a4 - Post Divider Register"]
    pub post_root137_set: POST_ROOT137_SET,
    #[doc = "0xc4a8 - Post Divider Register"]
    pub post_root137_clr: POST_ROOT137_CLR,
    #[doc = "0xc4ac - Post Divider Register"]
    pub post_root137_tog: POST_ROOT137_TOG,
    #[doc = "0xc4b0 - Pre Divider Register"]
    pub pre137: PRE137,
    #[doc = "0xc4b4 - Pre Divider Register"]
    pub pre_root137_set: PRE_ROOT137_SET,
    #[doc = "0xc4b8 - Pre Divider Register"]
    pub pre_root137_clr: PRE_ROOT137_CLR,
    #[doc = "0xc4bc - Pre Divider Register"]
    pub pre_root137_tog: PRE_ROOT137_TOG,
    _reserved105: [u8; 48usize],
    #[doc = "0xc4f0 - Access Control Register"]
    pub access_ctrl137: ACCESS_CTRL137,
    #[doc = "0xc4f4 - Access Control Register"]
    pub access_ctrl_root137_set: ACCESS_CTRL_ROOT137_SET,
    #[doc = "0xc4f8 - Access Control Register"]
    pub access_ctrl_root137_clr: ACCESS_CTRL_ROOT137_CLR,
    #[doc = "0xc4fc - Access Control Register"]
    pub access_ctrl_root137_tog: ACCESS_CTRL_ROOT137_TOG,
    #[doc = "0xc500 - Target Register"]
    pub target_root138: TARGET_ROOT138,
    #[doc = "0xc504 - Target Register"]
    pub target_root138_set: TARGET_ROOT138_SET,
    #[doc = "0xc508 - Target Register"]
    pub target_root138_clr: TARGET_ROOT138_CLR,
    #[doc = "0xc50c - Target Register"]
    pub target_root138_tog: TARGET_ROOT138_TOG,
    #[doc = "0xc510 - Miscellaneous Register"]
    pub misc138: MISC138,
    #[doc = "0xc514 - Miscellaneous Register"]
    pub misc_root138_set: MISC_ROOT138_SET,
    #[doc = "0xc518 - Miscellaneous Register"]
    pub misc_root138_clr: MISC_ROOT138_CLR,
    #[doc = "0xc51c - Miscellaneous Register"]
    pub misc_root138_tog: MISC_ROOT138_TOG,
    #[doc = "0xc520 - Post Divider Register"]
    pub post138: POST138,
    #[doc = "0xc524 - Post Divider Register"]
    pub post_root138_set: POST_ROOT138_SET,
    #[doc = "0xc528 - Post Divider Register"]
    pub post_root138_clr: POST_ROOT138_CLR,
    #[doc = "0xc52c - Post Divider Register"]
    pub post_root138_tog: POST_ROOT138_TOG,
    #[doc = "0xc530 - Pre Divider Register"]
    pub pre138: PRE138,
    #[doc = "0xc534 - Pre Divider Register"]
    pub pre_root138_set: PRE_ROOT138_SET,
    #[doc = "0xc538 - Pre Divider Register"]
    pub pre_root138_clr: PRE_ROOT138_CLR,
    #[doc = "0xc53c - Pre Divider Register"]
    pub pre_root138_tog: PRE_ROOT138_TOG,
    _reserved106: [u8; 48usize],
    #[doc = "0xc570 - Access Control Register"]
    pub access_ctrl138: ACCESS_CTRL138,
    #[doc = "0xc574 - Access Control Register"]
    pub access_ctrl_root138_set: ACCESS_CTRL_ROOT138_SET,
    #[doc = "0xc578 - Access Control Register"]
    pub access_ctrl_root138_clr: ACCESS_CTRL_ROOT138_CLR,
    #[doc = "0xc57c - Access Control Register"]
    pub access_ctrl_root138_tog: ACCESS_CTRL_ROOT138_TOG,
    #[doc = "0xc580 - Target Register"]
    pub target_root139: TARGET_ROOT139,
    #[doc = "0xc584 - Target Register"]
    pub target_root139_set: TARGET_ROOT139_SET,
    #[doc = "0xc588 - Target Register"]
    pub target_root139_clr: TARGET_ROOT139_CLR,
    #[doc = "0xc58c - Target Register"]
    pub target_root139_tog: TARGET_ROOT139_TOG,
    #[doc = "0xc590 - Miscellaneous Register"]
    pub misc139: MISC139,
    #[doc = "0xc594 - Miscellaneous Register"]
    pub misc_root139_set: MISC_ROOT139_SET,
    #[doc = "0xc598 - Miscellaneous Register"]
    pub misc_root139_clr: MISC_ROOT139_CLR,
    #[doc = "0xc59c - Miscellaneous Register"]
    pub misc_root139_tog: MISC_ROOT139_TOG,
    #[doc = "0xc5a0 - Post Divider Register"]
    pub post139: POST139,
    #[doc = "0xc5a4 - Post Divider Register"]
    pub post_root139_set: POST_ROOT139_SET,
    #[doc = "0xc5a8 - Post Divider Register"]
    pub post_root139_clr: POST_ROOT139_CLR,
    #[doc = "0xc5ac - Post Divider Register"]
    pub post_root139_tog: POST_ROOT139_TOG,
    #[doc = "0xc5b0 - Pre Divider Register"]
    pub pre139: PRE139,
    #[doc = "0xc5b4 - Pre Divider Register"]
    pub pre_root139_set: PRE_ROOT139_SET,
    #[doc = "0xc5b8 - Pre Divider Register"]
    pub pre_root139_clr: PRE_ROOT139_CLR,
    #[doc = "0xc5bc - Pre Divider Register"]
    pub pre_root139_tog: PRE_ROOT139_TOG,
    _reserved107: [u8; 48usize],
    #[doc = "0xc5f0 - Access Control Register"]
    pub access_ctrl139: ACCESS_CTRL139,
    #[doc = "0xc5f4 - Access Control Register"]
    pub access_ctrl_root139_set: ACCESS_CTRL_ROOT139_SET,
    #[doc = "0xc5f8 - Access Control Register"]
    pub access_ctrl_root139_clr: ACCESS_CTRL_ROOT139_CLR,
    #[doc = "0xc5fc - Access Control Register"]
    pub access_ctrl_root139_tog: ACCESS_CTRL_ROOT139_TOG,
    #[doc = "0xc600 - Target Register"]
    pub target_root140: TARGET_ROOT140,
    #[doc = "0xc604 - Target Register"]
    pub target_root140_set: TARGET_ROOT140_SET,
    #[doc = "0xc608 - Target Register"]
    pub target_root140_clr: TARGET_ROOT140_CLR,
    #[doc = "0xc60c - Target Register"]
    pub target_root140_tog: TARGET_ROOT140_TOG,
    #[doc = "0xc610 - Miscellaneous Register"]
    pub misc140: MISC140,
    #[doc = "0xc614 - Miscellaneous Register"]
    pub misc_root140_set: MISC_ROOT140_SET,
    #[doc = "0xc618 - Miscellaneous Register"]
    pub misc_root140_clr: MISC_ROOT140_CLR,
    #[doc = "0xc61c - Miscellaneous Register"]
    pub misc_root140_tog: MISC_ROOT140_TOG,
    #[doc = "0xc620 - Post Divider Register"]
    pub post140: POST140,
    #[doc = "0xc624 - Post Divider Register"]
    pub post_root140_set: POST_ROOT140_SET,
    #[doc = "0xc628 - Post Divider Register"]
    pub post_root140_clr: POST_ROOT140_CLR,
    #[doc = "0xc62c - Post Divider Register"]
    pub post_root140_tog: POST_ROOT140_TOG,
    #[doc = "0xc630 - Pre Divider Register"]
    pub pre140: PRE140,
    #[doc = "0xc634 - Pre Divider Register"]
    pub pre_root140_set: PRE_ROOT140_SET,
    #[doc = "0xc638 - Pre Divider Register"]
    pub pre_root140_clr: PRE_ROOT140_CLR,
    #[doc = "0xc63c - Pre Divider Register"]
    pub pre_root140_tog: PRE_ROOT140_TOG,
    _reserved108: [u8; 48usize],
    #[doc = "0xc670 - Access Control Register"]
    pub access_ctrl140: ACCESS_CTRL140,
    #[doc = "0xc674 - Access Control Register"]
    pub access_ctrl_root140_set: ACCESS_CTRL_ROOT140_SET,
    #[doc = "0xc678 - Access Control Register"]
    pub access_ctrl_root140_clr: ACCESS_CTRL_ROOT140_CLR,
    #[doc = "0xc67c - Access Control Register"]
    pub access_ctrl_root140_tog: ACCESS_CTRL_ROOT140_TOG,
    #[doc = "0xc680 - Target Register"]
    pub target_root141: TARGET_ROOT141,
    #[doc = "0xc684 - Target Register"]
    pub target_root141_set: TARGET_ROOT141_SET,
    #[doc = "0xc688 - Target Register"]
    pub target_root141_clr: TARGET_ROOT141_CLR,
    #[doc = "0xc68c - Target Register"]
    pub target_root141_tog: TARGET_ROOT141_TOG,
    #[doc = "0xc690 - Miscellaneous Register"]
    pub misc141: MISC141,
    #[doc = "0xc694 - Miscellaneous Register"]
    pub misc_root141_set: MISC_ROOT141_SET,
    #[doc = "0xc698 - Miscellaneous Register"]
    pub misc_root141_clr: MISC_ROOT141_CLR,
    #[doc = "0xc69c - Miscellaneous Register"]
    pub misc_root141_tog: MISC_ROOT141_TOG,
    #[doc = "0xc6a0 - Post Divider Register"]
    pub post141: POST141,
    #[doc = "0xc6a4 - Post Divider Register"]
    pub post_root141_set: POST_ROOT141_SET,
    #[doc = "0xc6a8 - Post Divider Register"]
    pub post_root141_clr: POST_ROOT141_CLR,
    #[doc = "0xc6ac - Post Divider Register"]
    pub post_root141_tog: POST_ROOT141_TOG,
    #[doc = "0xc6b0 - Pre Divider Register"]
    pub pre141: PRE141,
    #[doc = "0xc6b4 - Pre Divider Register"]
    pub pre_root141_set: PRE_ROOT141_SET,
    #[doc = "0xc6b8 - Pre Divider Register"]
    pub pre_root141_clr: PRE_ROOT141_CLR,
    #[doc = "0xc6bc - Pre Divider Register"]
    pub pre_root141_tog: PRE_ROOT141_TOG,
    _reserved109: [u8; 48usize],
    #[doc = "0xc6f0 - Access Control Register"]
    pub access_ctrl141: ACCESS_CTRL141,
    #[doc = "0xc6f4 - Access Control Register"]
    pub access_ctrl_root141_set: ACCESS_CTRL_ROOT141_SET,
    #[doc = "0xc6f8 - Access Control Register"]
    pub access_ctrl_root141_clr: ACCESS_CTRL_ROOT141_CLR,
    #[doc = "0xc6fc - Access Control Register"]
    pub access_ctrl_root141_tog: ACCESS_CTRL_ROOT141_TOG,
}
#[doc = "General Purpose Register"]
pub struct GPR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register"]
pub mod gpr0;
#[doc = "General Purpose Register"]
pub struct GPR0_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register"]
pub mod gpr0_set;
#[doc = "General Purpose Register"]
pub struct GPR0_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register"]
pub mod gpr0_clr;
#[doc = "General Purpose Register"]
pub struct GPR0_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register"]
pub mod gpr0_tog;
#[doc = "CCM PLL Control Register"]
pub struct PLL_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM PLL Control Register"]
pub mod pll_ctrl;
#[doc = "CCM PLL Control Register"]
pub struct PLL_CTRL_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM PLL Control Register"]
pub mod pll_ctrl_set;
#[doc = "CCM PLL Control Register"]
pub struct PLL_CTRL_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM PLL Control Register"]
pub mod pll_ctrl_clr;
#[doc = "CCM PLL Control Register"]
pub struct PLL_CTRL_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM PLL Control Register"]
pub mod pll_ctrl_tog;
#[doc = "CCM Clock Gating Register"]
pub struct CCGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Clock Gating Register"]
pub mod ccgr;
#[doc = "CCM Clock Gating Register"]
pub struct CCGR_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Clock Gating Register"]
pub mod ccgr_set;
#[doc = "CCM Clock Gating Register"]
pub struct CCGR_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Clock Gating Register"]
pub mod ccgr_clr;
#[doc = "CCM Clock Gating Register"]
pub struct CCGR_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM Clock Gating Register"]
pub mod ccgr_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root0;
#[doc = "Target Register"]
pub struct TARGET_ROOT0_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root0_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT0_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root0_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT0_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root0_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc0;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT0_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root0_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT0_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root0_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT0_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root0_tog;
#[doc = "Post Divider Register"]
pub struct POST0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post0;
#[doc = "Post Divider Register"]
pub struct POST_ROOT0_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root0_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT0_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root0_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT0_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root0_tog;
#[doc = "Pre Divider Register"]
pub struct PRE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre0;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT0_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root0_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT0_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root0_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT0_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root0_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl0;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT0_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root0_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT0_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root0_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT0_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root0_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root1;
#[doc = "Target Register"]
pub struct TARGET_ROOT1_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root1_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT1_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root1_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT1_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root1_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc1;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT1_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root1_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT1_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root1_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT1_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root1_tog;
#[doc = "Post Divider Register"]
pub struct POST1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post1;
#[doc = "Post Divider Register"]
pub struct POST_ROOT1_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root1_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT1_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root1_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT1_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root1_tog;
#[doc = "Pre Divider Register"]
pub struct PRE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre1;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT1_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root1_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT1_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root1_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT1_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root1_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl1;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT1_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root1_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT1_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root1_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT1_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root1_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root2;
#[doc = "Target Register"]
pub struct TARGET_ROOT2_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root2_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT2_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root2_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT2_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root2_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc2;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT2_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root2_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT2_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root2_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT2_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root2_tog;
#[doc = "Post Divider Register"]
pub struct POST2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post2;
#[doc = "Post Divider Register"]
pub struct POST_ROOT2_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root2_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT2_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root2_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT2_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root2_tog;
#[doc = "Pre Divider Register"]
pub struct PRE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre2;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT2_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root2_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT2_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root2_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT2_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root2_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl2;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT2_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root2_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT2_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root2_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT2_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root2_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root3;
#[doc = "Target Register"]
pub struct TARGET_ROOT3_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root3_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT3_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root3_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT3_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root3_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc3;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT3_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root3_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT3_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root3_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT3_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root3_tog;
#[doc = "Post Divider Register"]
pub struct POST3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post3;
#[doc = "Post Divider Register"]
pub struct POST_ROOT3_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root3_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT3_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root3_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT3_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root3_tog;
#[doc = "Pre Divider Register"]
pub struct PRE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre3;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT3_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root3_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT3_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root3_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT3_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root3_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl3;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT3_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root3_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT3_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root3_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT3_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root3_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root4;
#[doc = "Target Register"]
pub struct TARGET_ROOT4_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root4_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT4_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root4_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT4_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root4_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc4;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT4_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root4_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT4_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root4_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT4_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root4_tog;
#[doc = "Post Divider Register"]
pub struct POST4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post4;
#[doc = "Post Divider Register"]
pub struct POST_ROOT4_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root4_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT4_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root4_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT4_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root4_tog;
#[doc = "Pre Divider Register"]
pub struct PRE4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre4;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT4_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root4_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT4_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root4_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT4_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root4_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl4;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT4_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root4_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT4_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root4_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT4_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root4_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root16;
#[doc = "Target Register"]
pub struct TARGET_ROOT16_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root16_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT16_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root16_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT16_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root16_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc16;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT16_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root16_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT16_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root16_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT16_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root16_tog;
#[doc = "Post Divider Register"]
pub struct POST16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post16;
#[doc = "Post Divider Register"]
pub struct POST_ROOT16_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root16_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT16_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root16_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT16_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root16_tog;
#[doc = "Pre Divider Register"]
pub struct PRE16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre16;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT16_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root16_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT16_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root16_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT16_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root16_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl16;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT16_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root16_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT16_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root16_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT16_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root16_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root17;
#[doc = "Target Register"]
pub struct TARGET_ROOT17_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root17_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT17_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root17_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT17_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root17_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc17;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT17_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root17_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT17_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root17_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT17_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root17_tog;
#[doc = "Post Divider Register"]
pub struct POST17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post17;
#[doc = "Post Divider Register"]
pub struct POST_ROOT17_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root17_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT17_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root17_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT17_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root17_tog;
#[doc = "Pre Divider Register"]
pub struct PRE17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre17;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT17_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root17_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT17_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root17_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT17_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root17_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl17;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT17_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root17_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT17_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root17_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT17_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root17_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root18;
#[doc = "Target Register"]
pub struct TARGET_ROOT18_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root18_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT18_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root18_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT18_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root18_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc18;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT18_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root18_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT18_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root18_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT18_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root18_tog;
#[doc = "Post Divider Register"]
pub struct POST18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post18;
#[doc = "Post Divider Register"]
pub struct POST_ROOT18_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root18_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT18_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root18_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT18_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root18_tog;
#[doc = "Pre Divider Register"]
pub struct PRE18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre18;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT18_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root18_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT18_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root18_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT18_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root18_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl18;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT18_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root18_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT18_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root18_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT18_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root18_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root19;
#[doc = "Target Register"]
pub struct TARGET_ROOT19_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root19_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT19_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root19_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT19_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root19_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc19;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT19_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root19_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT19_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root19_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT19_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root19_tog;
#[doc = "Post Divider Register"]
pub struct POST19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post19;
#[doc = "Post Divider Register"]
pub struct POST_ROOT19_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root19_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT19_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root19_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT19_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root19_tog;
#[doc = "Pre Divider Register"]
pub struct PRE19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre19;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT19_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root19_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT19_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root19_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT19_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root19_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl19;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT19_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root19_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT19_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root19_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT19_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root19_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root20;
#[doc = "Target Register"]
pub struct TARGET_ROOT20_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root20_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT20_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root20_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT20_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root20_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc20;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT20_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root20_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT20_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root20_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT20_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root20_tog;
#[doc = "Post Divider Register"]
pub struct POST20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post20;
#[doc = "Post Divider Register"]
pub struct POST_ROOT20_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root20_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT20_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root20_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT20_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root20_tog;
#[doc = "Pre Divider Register"]
pub struct PRE20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre20;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT20_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root20_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT20_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root20_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT20_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root20_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl20;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT20_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root20_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT20_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root20_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT20_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root20_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root21;
#[doc = "Target Register"]
pub struct TARGET_ROOT21_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root21_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT21_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root21_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT21_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root21_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc21;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT21_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root21_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT21_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root21_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT21_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root21_tog;
#[doc = "Post Divider Register"]
pub struct POST21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post21;
#[doc = "Post Divider Register"]
pub struct POST_ROOT21_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root21_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT21_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root21_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT21_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root21_tog;
#[doc = "Pre Divider Register"]
pub struct PRE21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre21;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT21_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root21_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT21_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root21_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT21_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root21_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl21;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT21_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root21_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT21_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root21_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT21_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root21_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root22;
#[doc = "Target Register"]
pub struct TARGET_ROOT22_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root22_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT22_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root22_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT22_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root22_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc22;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT22_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root22_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT22_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root22_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT22_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root22_tog;
#[doc = "Post Divider Register"]
pub struct POST22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post22;
#[doc = "Post Divider Register"]
pub struct POST_ROOT22_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root22_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT22_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root22_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT22_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root22_tog;
#[doc = "Pre Divider Register"]
pub struct PRE22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre22;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT22_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root22_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT22_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root22_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT22_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root22_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl22;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT22_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root22_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT22_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root22_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT22_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root22_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root23;
#[doc = "Target Register"]
pub struct TARGET_ROOT23_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root23_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT23_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root23_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT23_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root23_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc23;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT23_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root23_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT23_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root23_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT23_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root23_tog;
#[doc = "Post Divider Register"]
pub struct POST23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post23;
#[doc = "Post Divider Register"]
pub struct POST_ROOT23_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root23_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT23_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root23_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT23_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root23_tog;
#[doc = "Pre Divider Register"]
pub struct PRE23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre23;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT23_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root23_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT23_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root23_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT23_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root23_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl23;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT23_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root23_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT23_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root23_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT23_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root23_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root24;
#[doc = "Target Register"]
pub struct TARGET_ROOT24_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root24_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT24_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root24_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT24_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root24_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc24;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT24_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root24_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT24_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root24_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT24_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root24_tog;
#[doc = "Post Divider Register"]
pub struct POST24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post24;
#[doc = "Post Divider Register"]
pub struct POST_ROOT24_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root24_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT24_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root24_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT24_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root24_tog;
#[doc = "Pre Divider Register"]
pub struct PRE24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre24;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT24_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root24_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT24_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root24_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT24_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root24_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl24;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT24_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root24_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT24_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root24_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT24_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root24_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root25;
#[doc = "Target Register"]
pub struct TARGET_ROOT25_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root25_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT25_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root25_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT25_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root25_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc25;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT25_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root25_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT25_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root25_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT25_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root25_tog;
#[doc = "Post Divider Register"]
pub struct POST25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post25;
#[doc = "Post Divider Register"]
pub struct POST_ROOT25_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root25_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT25_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root25_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT25_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root25_tog;
#[doc = "Pre Divider Register"]
pub struct PRE25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre25;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT25_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root25_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT25_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root25_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT25_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root25_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl25;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT25_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root25_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT25_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root25_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT25_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root25_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root26;
#[doc = "Target Register"]
pub struct TARGET_ROOT26_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root26_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT26_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root26_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT26_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root26_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc26;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT26_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root26_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT26_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root26_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT26_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root26_tog;
#[doc = "Post Divider Register"]
pub struct POST26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post26;
#[doc = "Post Divider Register"]
pub struct POST_ROOT26_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root26_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT26_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root26_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT26_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root26_tog;
#[doc = "Pre Divider Register"]
pub struct PRE26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre26;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT26_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root26_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT26_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root26_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT26_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root26_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl26;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT26_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root26_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT26_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root26_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT26_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root26_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root27;
#[doc = "Target Register"]
pub struct TARGET_ROOT27_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root27_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT27_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root27_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT27_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root27_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc27;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT27_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root27_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT27_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root27_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT27_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root27_tog;
#[doc = "Post Divider Register"]
pub struct POST27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post27;
#[doc = "Post Divider Register"]
pub struct POST_ROOT27_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root27_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT27_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root27_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT27_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root27_tog;
#[doc = "Pre Divider Register"]
pub struct PRE27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre27;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT27_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root27_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT27_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root27_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT27_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root27_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl27;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT27_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root27_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT27_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root27_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT27_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root27_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root32;
#[doc = "Target Register"]
pub struct TARGET_ROOT32_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root32_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT32_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root32_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT32_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root32_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc32;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT32_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root32_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT32_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root32_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT32_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root32_tog;
#[doc = "Post Divider Register"]
pub struct POST32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post32;
#[doc = "Post Divider Register"]
pub struct POST_ROOT32_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root32_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT32_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root32_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT32_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root32_tog;
#[doc = "Pre Divider Register"]
pub struct PRE32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre32;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT32_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root32_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT32_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root32_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT32_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root32_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl32;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT32_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root32_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT32_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root32_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT32_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root32_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT33 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root33;
#[doc = "Target Register"]
pub struct TARGET_ROOT33_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root33_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT33_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root33_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT33_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root33_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC33 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc33;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT33_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root33_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT33_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root33_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT33_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root33_tog;
#[doc = "Post Divider Register"]
pub struct POST33 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post33;
#[doc = "Post Divider Register"]
pub struct POST_ROOT33_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root33_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT33_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root33_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT33_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root33_tog;
#[doc = "Pre Divider Register"]
pub struct PRE33 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre33;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT33_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root33_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT33_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root33_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT33_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root33_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL33 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl33;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT33_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root33_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT33_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root33_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT33_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root33_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root34;
#[doc = "Target Register"]
pub struct TARGET_ROOT34_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root34_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT34_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root34_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT34_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root34_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc34;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT34_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root34_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT34_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root34_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT34_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root34_tog;
#[doc = "Post Divider Register"]
pub struct POST34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post34;
#[doc = "Post Divider Register"]
pub struct POST_ROOT34_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root34_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT34_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root34_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT34_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root34_tog;
#[doc = "Pre Divider Register"]
pub struct PRE34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre34;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT34_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root34_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT34_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root34_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT34_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root34_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl34;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT34_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root34_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT34_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root34_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT34_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root34_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT35 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root35;
#[doc = "Target Register"]
pub struct TARGET_ROOT35_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root35_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT35_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root35_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT35_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root35_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC35 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc35;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT35_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root35_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT35_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root35_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT35_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root35_tog;
#[doc = "Post Divider Register"]
pub struct POST35 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post35;
#[doc = "Post Divider Register"]
pub struct POST_ROOT35_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root35_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT35_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root35_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT35_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root35_tog;
#[doc = "Pre Divider Register"]
pub struct PRE35 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre35;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT35_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root35_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT35_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root35_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT35_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root35_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL35 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl35;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT35_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root35_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT35_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root35_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT35_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root35_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT36 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root36;
#[doc = "Target Register"]
pub struct TARGET_ROOT36_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root36_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT36_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root36_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT36_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root36_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC36 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc36;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT36_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root36_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT36_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root36_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT36_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root36_tog;
#[doc = "Post Divider Register"]
pub struct POST36 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post36;
#[doc = "Post Divider Register"]
pub struct POST_ROOT36_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root36_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT36_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root36_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT36_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root36_tog;
#[doc = "Pre Divider Register"]
pub struct PRE36 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre36;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT36_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root36_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT36_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root36_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT36_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root36_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL36 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl36;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT36_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root36_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT36_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root36_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT36_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root36_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT37 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root37;
#[doc = "Target Register"]
pub struct TARGET_ROOT37_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root37_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT37_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root37_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT37_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root37_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC37 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc37;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT37_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root37_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT37_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root37_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT37_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root37_tog;
#[doc = "Post Divider Register"]
pub struct POST37 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post37;
#[doc = "Post Divider Register"]
pub struct POST_ROOT37_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root37_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT37_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root37_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT37_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root37_tog;
#[doc = "Pre Divider Register"]
pub struct PRE37 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre37;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT37_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root37_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT37_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root37_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT37_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root37_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL37 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl37;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT37_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root37_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT37_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root37_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT37_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root37_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT48 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root48;
#[doc = "Target Register"]
pub struct TARGET_ROOT48_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root48_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT48_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root48_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT48_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root48_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC48 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc48;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT48_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root48_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT48_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root48_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT48_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root48_tog;
#[doc = "Post Divider Register"]
pub struct POST48 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post48;
#[doc = "Post Divider Register"]
pub struct POST_ROOT48_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root48_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT48_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root48_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT48_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root48_tog;
#[doc = "Pre Divider Register"]
pub struct PRE48 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre48;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT48_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root48_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT48_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root48_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT48_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root48_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL48 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl48;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT48_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root48_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT48_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root48_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT48_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root48_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT49 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root49;
#[doc = "Target Register"]
pub struct TARGET_ROOT49_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root49_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT49_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root49_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT49_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root49_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC49 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc49;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT49_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root49_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT49_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root49_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT49_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root49_tog;
#[doc = "Post Divider Register"]
pub struct POST49 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post49;
#[doc = "Post Divider Register"]
pub struct POST_ROOT49_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root49_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT49_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root49_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT49_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root49_tog;
#[doc = "Pre Divider Register"]
pub struct PRE49 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre49;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT49_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root49_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT49_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root49_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT49_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root49_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL49 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl49;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT49_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root49_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT49_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root49_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT49_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root49_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT64 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root64;
#[doc = "Target Register"]
pub struct TARGET_ROOT64_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root64_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT64_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root64_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT64_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root64_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC64 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc64;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT64_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root64_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT64_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root64_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT64_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root64_tog;
#[doc = "Post Divider Register"]
pub struct POST64 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post64;
#[doc = "Post Divider Register"]
pub struct POST_ROOT64_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root64_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT64_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root64_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT64_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root64_tog;
#[doc = "Pre Divider Register"]
pub struct PRE64 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre64;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT64_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root64_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT64_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root64_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT64_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root64_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL64 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl64;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT64_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root64_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT64_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root64_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT64_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root64_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT65 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root65;
#[doc = "Target Register"]
pub struct TARGET_ROOT65_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root65_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT65_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root65_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT65_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root65_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC65 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc65;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT65_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root65_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT65_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root65_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT65_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root65_tog;
#[doc = "Post Divider Register"]
pub struct POST65 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post65;
#[doc = "Post Divider Register"]
pub struct POST_ROOT65_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root65_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT65_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root65_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT65_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root65_tog;
#[doc = "Pre Divider Register"]
pub struct PRE65 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre65;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT65_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root65_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT65_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root65_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT65_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root65_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL65 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl65;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT65_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root65_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT65_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root65_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT65_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root65_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT66 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root66;
#[doc = "Target Register"]
pub struct TARGET_ROOT66_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root66_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT66_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root66_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT66_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root66_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC66 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc66;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT66_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root66_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT66_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root66_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT66_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root66_tog;
#[doc = "Post Divider Register"]
pub struct POST66 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post66;
#[doc = "Post Divider Register"]
pub struct POST_ROOT66_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root66_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT66_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root66_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT66_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root66_tog;
#[doc = "Pre Divider Register"]
pub struct PRE66 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre66;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT66_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root66_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT66_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root66_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT66_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root66_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL66 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl66;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT66_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root66_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT66_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root66_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT66_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root66_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT67 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root67;
#[doc = "Target Register"]
pub struct TARGET_ROOT67_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root67_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT67_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root67_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT67_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root67_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC67 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc67;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT67_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root67_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT67_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root67_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT67_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root67_tog;
#[doc = "Post Divider Register"]
pub struct POST67 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post67;
#[doc = "Post Divider Register"]
pub struct POST_ROOT67_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root67_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT67_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root67_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT67_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root67_tog;
#[doc = "Pre Divider Register"]
pub struct PRE67 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre67;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT67_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root67_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT67_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root67_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT67_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root67_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL67 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl67;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT67_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root67_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT67_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root67_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT67_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root67_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT68 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root68;
#[doc = "Target Register"]
pub struct TARGET_ROOT68_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root68_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT68_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root68_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT68_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root68_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC68 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc68;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT68_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root68_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT68_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root68_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT68_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root68_tog;
#[doc = "Post Divider Register"]
pub struct POST68 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post68;
#[doc = "Post Divider Register"]
pub struct POST_ROOT68_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root68_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT68_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root68_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT68_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root68_tog;
#[doc = "Pre Divider Register"]
pub struct PRE68 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre68;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT68_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root68_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT68_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root68_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT68_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root68_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL68 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl68;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT68_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root68_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT68_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root68_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT68_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root68_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT69 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root69;
#[doc = "Target Register"]
pub struct TARGET_ROOT69_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root69_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT69_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root69_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT69_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root69_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC69 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc69;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT69_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root69_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT69_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root69_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT69_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root69_tog;
#[doc = "Post Divider Register"]
pub struct POST69 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post69;
#[doc = "Post Divider Register"]
pub struct POST_ROOT69_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root69_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT69_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root69_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT69_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root69_tog;
#[doc = "Pre Divider Register"]
pub struct PRE69 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre69;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT69_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root69_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT69_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root69_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT69_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root69_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL69 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl69;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT69_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root69_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT69_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root69_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT69_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root69_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT70 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root70;
#[doc = "Target Register"]
pub struct TARGET_ROOT70_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root70_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT70_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root70_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT70_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root70_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC70 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc70;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT70_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root70_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT70_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root70_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT70_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root70_tog;
#[doc = "Post Divider Register"]
pub struct POST70 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post70;
#[doc = "Post Divider Register"]
pub struct POST_ROOT70_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root70_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT70_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root70_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT70_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root70_tog;
#[doc = "Pre Divider Register"]
pub struct PRE70 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre70;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT70_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root70_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT70_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root70_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT70_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root70_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL70 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl70;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT70_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root70_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT70_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root70_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT70_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root70_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT71 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root71;
#[doc = "Target Register"]
pub struct TARGET_ROOT71_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root71_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT71_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root71_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT71_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root71_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC71 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc71;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT71_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root71_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT71_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root71_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT71_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root71_tog;
#[doc = "Post Divider Register"]
pub struct POST71 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post71;
#[doc = "Post Divider Register"]
pub struct POST_ROOT71_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root71_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT71_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root71_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT71_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root71_tog;
#[doc = "Pre Divider Register"]
pub struct PRE71 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre71;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT71_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root71_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT71_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root71_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT71_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root71_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL71 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl71;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT71_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root71_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT71_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root71_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT71_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root71_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT72 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root72;
#[doc = "Target Register"]
pub struct TARGET_ROOT72_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root72_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT72_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root72_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT72_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root72_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC72 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc72;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT72_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root72_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT72_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root72_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT72_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root72_tog;
#[doc = "Post Divider Register"]
pub struct POST72 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post72;
#[doc = "Post Divider Register"]
pub struct POST_ROOT72_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root72_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT72_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root72_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT72_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root72_tog;
#[doc = "Pre Divider Register"]
pub struct PRE72 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre72;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT72_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root72_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT72_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root72_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT72_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root72_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL72 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl72;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT72_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root72_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT72_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root72_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT72_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root72_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT73 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root73;
#[doc = "Target Register"]
pub struct TARGET_ROOT73_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root73_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT73_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root73_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT73_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root73_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC73 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc73;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT73_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root73_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT73_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root73_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT73_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root73_tog;
#[doc = "Post Divider Register"]
pub struct POST73 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post73;
#[doc = "Post Divider Register"]
pub struct POST_ROOT73_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root73_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT73_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root73_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT73_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root73_tog;
#[doc = "Pre Divider Register"]
pub struct PRE73 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre73;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT73_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root73_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT73_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root73_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT73_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root73_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL73 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl73;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT73_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root73_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT73_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root73_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT73_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root73_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT74 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root74;
#[doc = "Target Register"]
pub struct TARGET_ROOT74_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root74_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT74_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root74_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT74_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root74_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC74 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc74;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT74_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root74_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT74_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root74_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT74_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root74_tog;
#[doc = "Post Divider Register"]
pub struct POST74 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post74;
#[doc = "Post Divider Register"]
pub struct POST_ROOT74_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root74_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT74_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root74_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT74_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root74_tog;
#[doc = "Pre Divider Register"]
pub struct PRE74 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre74;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT74_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root74_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT74_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root74_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT74_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root74_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL74 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl74;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT74_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root74_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT74_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root74_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT74_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root74_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT75 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root75;
#[doc = "Target Register"]
pub struct TARGET_ROOT75_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root75_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT75_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root75_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT75_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root75_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC75 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc75;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT75_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root75_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT75_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root75_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT75_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root75_tog;
#[doc = "Post Divider Register"]
pub struct POST75 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post75;
#[doc = "Post Divider Register"]
pub struct POST_ROOT75_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root75_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT75_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root75_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT75_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root75_tog;
#[doc = "Pre Divider Register"]
pub struct PRE75 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre75;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT75_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root75_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT75_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root75_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT75_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root75_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL75 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl75;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT75_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root75_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT75_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root75_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT75_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root75_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT76 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root76;
#[doc = "Target Register"]
pub struct TARGET_ROOT76_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root76_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT76_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root76_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT76_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root76_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC76 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc76;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT76_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root76_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT76_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root76_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT76_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root76_tog;
#[doc = "Post Divider Register"]
pub struct POST76 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post76;
#[doc = "Post Divider Register"]
pub struct POST_ROOT76_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root76_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT76_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root76_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT76_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root76_tog;
#[doc = "Pre Divider Register"]
pub struct PRE76 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre76;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT76_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root76_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT76_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root76_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT76_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root76_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL76 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl76;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT76_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root76_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT76_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root76_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT76_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root76_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT77 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root77;
#[doc = "Target Register"]
pub struct TARGET_ROOT77_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root77_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT77_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root77_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT77_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root77_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC77 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc77;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT77_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root77_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT77_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root77_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT77_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root77_tog;
#[doc = "Post Divider Register"]
pub struct POST77 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post77;
#[doc = "Post Divider Register"]
pub struct POST_ROOT77_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root77_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT77_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root77_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT77_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root77_tog;
#[doc = "Pre Divider Register"]
pub struct PRE77 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre77;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT77_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root77_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT77_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root77_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT77_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root77_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL77 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl77;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT77_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root77_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT77_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root77_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT77_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root77_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT78 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root78;
#[doc = "Target Register"]
pub struct TARGET_ROOT78_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root78_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT78_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root78_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT78_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root78_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC78 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc78;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT78_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root78_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT78_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root78_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT78_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root78_tog;
#[doc = "Post Divider Register"]
pub struct POST78 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post78;
#[doc = "Post Divider Register"]
pub struct POST_ROOT78_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root78_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT78_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root78_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT78_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root78_tog;
#[doc = "Pre Divider Register"]
pub struct PRE78 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre78;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT78_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root78_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT78_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root78_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT78_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root78_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL78 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl78;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT78_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root78_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT78_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root78_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT78_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root78_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT79 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root79;
#[doc = "Target Register"]
pub struct TARGET_ROOT79_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root79_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT79_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root79_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT79_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root79_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC79 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc79;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT79_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root79_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT79_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root79_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT79_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root79_tog;
#[doc = "Post Divider Register"]
pub struct POST79 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post79;
#[doc = "Post Divider Register"]
pub struct POST_ROOT79_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root79_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT79_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root79_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT79_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root79_tog;
#[doc = "Pre Divider Register"]
pub struct PRE79 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre79;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT79_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root79_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT79_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root79_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT79_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root79_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL79 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl79;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT79_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root79_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT79_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root79_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT79_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root79_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT80 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root80;
#[doc = "Target Register"]
pub struct TARGET_ROOT80_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root80_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT80_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root80_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT80_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root80_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC80 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc80;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT80_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root80_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT80_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root80_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT80_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root80_tog;
#[doc = "Post Divider Register"]
pub struct POST80 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post80;
#[doc = "Post Divider Register"]
pub struct POST_ROOT80_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root80_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT80_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root80_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT80_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root80_tog;
#[doc = "Pre Divider Register"]
pub struct PRE80 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre80;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT80_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root80_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT80_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root80_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT80_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root80_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL80 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl80;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT80_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root80_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT80_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root80_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT80_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root80_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT81 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root81;
#[doc = "Target Register"]
pub struct TARGET_ROOT81_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root81_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT81_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root81_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT81_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root81_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC81 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc81;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT81_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root81_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT81_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root81_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT81_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root81_tog;
#[doc = "Post Divider Register"]
pub struct POST81 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post81;
#[doc = "Post Divider Register"]
pub struct POST_ROOT81_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root81_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT81_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root81_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT81_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root81_tog;
#[doc = "Pre Divider Register"]
pub struct PRE81 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre81;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT81_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root81_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT81_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root81_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT81_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root81_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL81 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl81;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT81_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root81_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT81_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root81_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT81_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root81_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT82 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root82;
#[doc = "Target Register"]
pub struct TARGET_ROOT82_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root82_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT82_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root82_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT82_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root82_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC82 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc82;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT82_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root82_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT82_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root82_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT82_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root82_tog;
#[doc = "Post Divider Register"]
pub struct POST82 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post82;
#[doc = "Post Divider Register"]
pub struct POST_ROOT82_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root82_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT82_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root82_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT82_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root82_tog;
#[doc = "Pre Divider Register"]
pub struct PRE82 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre82;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT82_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root82_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT82_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root82_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT82_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root82_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL82 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl82;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT82_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root82_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT82_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root82_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT82_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root82_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT83 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root83;
#[doc = "Target Register"]
pub struct TARGET_ROOT83_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root83_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT83_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root83_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT83_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root83_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC83 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc83;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT83_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root83_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT83_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root83_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT83_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root83_tog;
#[doc = "Post Divider Register"]
pub struct POST83 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post83;
#[doc = "Post Divider Register"]
pub struct POST_ROOT83_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root83_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT83_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root83_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT83_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root83_tog;
#[doc = "Pre Divider Register"]
pub struct PRE83 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre83;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT83_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root83_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT83_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root83_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT83_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root83_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL83 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl83;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT83_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root83_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT83_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root83_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT83_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root83_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT84 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root84;
#[doc = "Target Register"]
pub struct TARGET_ROOT84_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root84_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT84_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root84_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT84_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root84_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC84 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc84;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT84_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root84_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT84_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root84_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT84_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root84_tog;
#[doc = "Post Divider Register"]
pub struct POST84 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post84;
#[doc = "Post Divider Register"]
pub struct POST_ROOT84_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root84_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT84_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root84_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT84_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root84_tog;
#[doc = "Pre Divider Register"]
pub struct PRE84 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre84;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT84_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root84_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT84_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root84_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT84_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root84_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL84 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl84;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT84_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root84_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT84_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root84_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT84_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root84_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT85 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root85;
#[doc = "Target Register"]
pub struct TARGET_ROOT85_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root85_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT85_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root85_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT85_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root85_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC85 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc85;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT85_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root85_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT85_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root85_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT85_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root85_tog;
#[doc = "Post Divider Register"]
pub struct POST85 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post85;
#[doc = "Post Divider Register"]
pub struct POST_ROOT85_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root85_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT85_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root85_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT85_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root85_tog;
#[doc = "Pre Divider Register"]
pub struct PRE85 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre85;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT85_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root85_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT85_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root85_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT85_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root85_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL85 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl85;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT85_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root85_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT85_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root85_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT85_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root85_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT86 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root86;
#[doc = "Target Register"]
pub struct TARGET_ROOT86_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root86_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT86_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root86_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT86_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root86_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC86 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc86;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT86_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root86_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT86_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root86_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT86_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root86_tog;
#[doc = "Post Divider Register"]
pub struct POST86 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post86;
#[doc = "Post Divider Register"]
pub struct POST_ROOT86_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root86_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT86_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root86_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT86_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root86_tog;
#[doc = "Pre Divider Register"]
pub struct PRE86 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre86;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT86_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root86_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT86_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root86_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT86_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root86_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL86 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl86;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT86_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root86_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT86_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root86_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT86_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root86_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT87 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root87;
#[doc = "Target Register"]
pub struct TARGET_ROOT87_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root87_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT87_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root87_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT87_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root87_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC87 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc87;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT87_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root87_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT87_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root87_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT87_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root87_tog;
#[doc = "Post Divider Register"]
pub struct POST87 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post87;
#[doc = "Post Divider Register"]
pub struct POST_ROOT87_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root87_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT87_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root87_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT87_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root87_tog;
#[doc = "Pre Divider Register"]
pub struct PRE87 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre87;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT87_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root87_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT87_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root87_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT87_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root87_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL87 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl87;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT87_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root87_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT87_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root87_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT87_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root87_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT88 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root88;
#[doc = "Target Register"]
pub struct TARGET_ROOT88_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root88_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT88_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root88_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT88_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root88_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC88 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc88;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT88_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root88_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT88_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root88_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT88_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root88_tog;
#[doc = "Post Divider Register"]
pub struct POST88 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post88;
#[doc = "Post Divider Register"]
pub struct POST_ROOT88_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root88_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT88_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root88_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT88_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root88_tog;
#[doc = "Pre Divider Register"]
pub struct PRE88 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre88;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT88_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root88_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT88_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root88_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT88_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root88_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL88 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl88;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT88_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root88_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT88_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root88_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT88_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root88_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT89 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root89;
#[doc = "Target Register"]
pub struct TARGET_ROOT89_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root89_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT89_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root89_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT89_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root89_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC89 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc89;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT89_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root89_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT89_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root89_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT89_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root89_tog;
#[doc = "Post Divider Register"]
pub struct POST89 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post89;
#[doc = "Post Divider Register"]
pub struct POST_ROOT89_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root89_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT89_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root89_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT89_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root89_tog;
#[doc = "Pre Divider Register"]
pub struct PRE89 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre89;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT89_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root89_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT89_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root89_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT89_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root89_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL89 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl89;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT89_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root89_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT89_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root89_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT89_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root89_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT90 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root90;
#[doc = "Target Register"]
pub struct TARGET_ROOT90_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root90_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT90_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root90_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT90_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root90_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC90 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc90;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT90_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root90_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT90_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root90_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT90_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root90_tog;
#[doc = "Post Divider Register"]
pub struct POST90 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post90;
#[doc = "Post Divider Register"]
pub struct POST_ROOT90_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root90_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT90_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root90_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT90_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root90_tog;
#[doc = "Pre Divider Register"]
pub struct PRE90 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre90;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT90_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root90_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT90_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root90_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT90_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root90_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL90 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl90;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT90_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root90_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT90_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root90_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT90_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root90_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT91 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root91;
#[doc = "Target Register"]
pub struct TARGET_ROOT91_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root91_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT91_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root91_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT91_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root91_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC91 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc91;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT91_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root91_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT91_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root91_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT91_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root91_tog;
#[doc = "Post Divider Register"]
pub struct POST91 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post91;
#[doc = "Post Divider Register"]
pub struct POST_ROOT91_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root91_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT91_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root91_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT91_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root91_tog;
#[doc = "Pre Divider Register"]
pub struct PRE91 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre91;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT91_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root91_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT91_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root91_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT91_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root91_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL91 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl91;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT91_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root91_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT91_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root91_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT91_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root91_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT92 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root92;
#[doc = "Target Register"]
pub struct TARGET_ROOT92_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root92_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT92_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root92_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT92_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root92_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC92 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc92;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT92_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root92_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT92_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root92_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT92_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root92_tog;
#[doc = "Post Divider Register"]
pub struct POST92 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post92;
#[doc = "Post Divider Register"]
pub struct POST_ROOT92_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root92_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT92_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root92_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT92_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root92_tog;
#[doc = "Pre Divider Register"]
pub struct PRE92 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre92;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT92_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root92_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT92_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root92_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT92_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root92_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL92 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl92;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT92_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root92_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT92_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root92_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT92_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root92_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT93 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root93;
#[doc = "Target Register"]
pub struct TARGET_ROOT93_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root93_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT93_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root93_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT93_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root93_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC93 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc93;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT93_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root93_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT93_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root93_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT93_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root93_tog;
#[doc = "Post Divider Register"]
pub struct POST93 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post93;
#[doc = "Post Divider Register"]
pub struct POST_ROOT93_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root93_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT93_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root93_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT93_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root93_tog;
#[doc = "Pre Divider Register"]
pub struct PRE93 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre93;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT93_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root93_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT93_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root93_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT93_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root93_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL93 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl93;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT93_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root93_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT93_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root93_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT93_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root93_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT94 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root94;
#[doc = "Target Register"]
pub struct TARGET_ROOT94_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root94_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT94_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root94_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT94_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root94_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC94 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc94;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT94_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root94_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT94_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root94_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT94_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root94_tog;
#[doc = "Post Divider Register"]
pub struct POST94 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post94;
#[doc = "Post Divider Register"]
pub struct POST_ROOT94_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root94_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT94_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root94_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT94_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root94_tog;
#[doc = "Pre Divider Register"]
pub struct PRE94 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre94;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT94_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root94_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT94_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root94_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT94_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root94_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL94 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl94;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT94_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root94_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT94_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root94_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT94_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root94_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT95 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root95;
#[doc = "Target Register"]
pub struct TARGET_ROOT95_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root95_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT95_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root95_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT95_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root95_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC95 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc95;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT95_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root95_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT95_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root95_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT95_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root95_tog;
#[doc = "Post Divider Register"]
pub struct POST95 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post95;
#[doc = "Post Divider Register"]
pub struct POST_ROOT95_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root95_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT95_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root95_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT95_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root95_tog;
#[doc = "Pre Divider Register"]
pub struct PRE95 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre95;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT95_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root95_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT95_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root95_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT95_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root95_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL95 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl95;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT95_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root95_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT95_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root95_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT95_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root95_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT96 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root96;
#[doc = "Target Register"]
pub struct TARGET_ROOT96_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root96_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT96_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root96_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT96_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root96_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC96 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc96;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT96_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root96_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT96_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root96_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT96_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root96_tog;
#[doc = "Post Divider Register"]
pub struct POST96 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post96;
#[doc = "Post Divider Register"]
pub struct POST_ROOT96_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root96_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT96_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root96_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT96_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root96_tog;
#[doc = "Pre Divider Register"]
pub struct PRE96 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre96;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT96_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root96_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT96_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root96_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT96_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root96_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL96 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl96;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT96_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root96_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT96_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root96_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT96_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root96_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT97 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root97;
#[doc = "Target Register"]
pub struct TARGET_ROOT97_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root97_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT97_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root97_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT97_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root97_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC97 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc97;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT97_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root97_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT97_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root97_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT97_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root97_tog;
#[doc = "Post Divider Register"]
pub struct POST97 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post97;
#[doc = "Post Divider Register"]
pub struct POST_ROOT97_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root97_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT97_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root97_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT97_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root97_tog;
#[doc = "Pre Divider Register"]
pub struct PRE97 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre97;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT97_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root97_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT97_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root97_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT97_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root97_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL97 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl97;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT97_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root97_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT97_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root97_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT97_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root97_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT98 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root98;
#[doc = "Target Register"]
pub struct TARGET_ROOT98_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root98_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT98_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root98_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT98_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root98_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC98 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc98;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT98_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root98_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT98_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root98_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT98_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root98_tog;
#[doc = "Post Divider Register"]
pub struct POST98 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post98;
#[doc = "Post Divider Register"]
pub struct POST_ROOT98_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root98_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT98_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root98_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT98_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root98_tog;
#[doc = "Pre Divider Register"]
pub struct PRE98 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre98;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT98_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root98_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT98_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root98_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT98_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root98_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL98 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl98;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT98_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root98_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT98_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root98_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT98_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root98_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT99 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root99;
#[doc = "Target Register"]
pub struct TARGET_ROOT99_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root99_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT99_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root99_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT99_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root99_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC99 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc99;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT99_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root99_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT99_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root99_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT99_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root99_tog;
#[doc = "Post Divider Register"]
pub struct POST99 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post99;
#[doc = "Post Divider Register"]
pub struct POST_ROOT99_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root99_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT99_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root99_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT99_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root99_tog;
#[doc = "Pre Divider Register"]
pub struct PRE99 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre99;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT99_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root99_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT99_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root99_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT99_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root99_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL99 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl99;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT99_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root99_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT99_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root99_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT99_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root99_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT100 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root100;
#[doc = "Target Register"]
pub struct TARGET_ROOT100_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root100_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT100_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root100_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT100_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root100_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC100 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc100;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT100_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root100_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT100_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root100_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT100_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root100_tog;
#[doc = "Post Divider Register"]
pub struct POST100 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post100;
#[doc = "Post Divider Register"]
pub struct POST_ROOT100_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root100_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT100_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root100_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT100_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root100_tog;
#[doc = "Pre Divider Register"]
pub struct PRE100 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre100;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT100_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root100_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT100_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root100_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT100_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root100_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL100 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl100;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT100_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root100_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT100_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root100_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT100_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root100_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT101 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root101;
#[doc = "Target Register"]
pub struct TARGET_ROOT101_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root101_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT101_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root101_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT101_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root101_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC101 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc101;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT101_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root101_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT101_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root101_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT101_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root101_tog;
#[doc = "Post Divider Register"]
pub struct POST101 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post101;
#[doc = "Post Divider Register"]
pub struct POST_ROOT101_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root101_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT101_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root101_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT101_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root101_tog;
#[doc = "Pre Divider Register"]
pub struct PRE101 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre101;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT101_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root101_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT101_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root101_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT101_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root101_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL101 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl101;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT101_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root101_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT101_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root101_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT101_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root101_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT102 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root102;
#[doc = "Target Register"]
pub struct TARGET_ROOT102_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root102_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT102_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root102_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT102_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root102_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC102 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc102;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT102_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root102_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT102_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root102_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT102_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root102_tog;
#[doc = "Post Divider Register"]
pub struct POST102 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post102;
#[doc = "Post Divider Register"]
pub struct POST_ROOT102_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root102_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT102_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root102_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT102_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root102_tog;
#[doc = "Pre Divider Register"]
pub struct PRE102 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre102;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT102_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root102_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT102_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root102_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT102_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root102_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL102 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl102;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT102_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root102_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT102_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root102_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT102_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root102_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT103 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root103;
#[doc = "Target Register"]
pub struct TARGET_ROOT103_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root103_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT103_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root103_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT103_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root103_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC103 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc103;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT103_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root103_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT103_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root103_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT103_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root103_tog;
#[doc = "Post Divider Register"]
pub struct POST103 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post103;
#[doc = "Post Divider Register"]
pub struct POST_ROOT103_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root103_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT103_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root103_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT103_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root103_tog;
#[doc = "Pre Divider Register"]
pub struct PRE103 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre103;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT103_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root103_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT103_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root103_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT103_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root103_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL103 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl103;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT103_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root103_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT103_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root103_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT103_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root103_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT104 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root104;
#[doc = "Target Register"]
pub struct TARGET_ROOT104_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root104_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT104_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root104_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT104_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root104_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC104 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc104;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT104_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root104_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT104_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root104_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT104_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root104_tog;
#[doc = "Post Divider Register"]
pub struct POST104 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post104;
#[doc = "Post Divider Register"]
pub struct POST_ROOT104_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root104_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT104_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root104_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT104_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root104_tog;
#[doc = "Pre Divider Register"]
pub struct PRE104 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre104;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT104_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root104_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT104_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root104_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT104_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root104_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL104 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl104;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT104_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root104_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT104_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root104_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT104_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root104_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT105 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root105;
#[doc = "Target Register"]
pub struct TARGET_ROOT105_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root105_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT105_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root105_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT105_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root105_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC105 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc105;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT105_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root105_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT105_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root105_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT105_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root105_tog;
#[doc = "Post Divider Register"]
pub struct POST105 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post105;
#[doc = "Post Divider Register"]
pub struct POST_ROOT105_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root105_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT105_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root105_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT105_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root105_tog;
#[doc = "Pre Divider Register"]
pub struct PRE105 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre105;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT105_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root105_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT105_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root105_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT105_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root105_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL105 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl105;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT105_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root105_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT105_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root105_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT105_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root105_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT106 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root106;
#[doc = "Target Register"]
pub struct TARGET_ROOT106_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root106_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT106_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root106_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT106_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root106_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC106 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc106;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT106_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root106_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT106_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root106_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT106_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root106_tog;
#[doc = "Post Divider Register"]
pub struct POST106 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post106;
#[doc = "Post Divider Register"]
pub struct POST_ROOT106_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root106_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT106_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root106_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT106_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root106_tog;
#[doc = "Pre Divider Register"]
pub struct PRE106 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre106;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT106_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root106_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT106_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root106_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT106_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root106_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL106 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl106;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT106_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root106_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT106_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root106_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT106_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root106_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT107 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root107;
#[doc = "Target Register"]
pub struct TARGET_ROOT107_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root107_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT107_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root107_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT107_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root107_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC107 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc107;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT107_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root107_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT107_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root107_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT107_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root107_tog;
#[doc = "Post Divider Register"]
pub struct POST107 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post107;
#[doc = "Post Divider Register"]
pub struct POST_ROOT107_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root107_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT107_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root107_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT107_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root107_tog;
#[doc = "Pre Divider Register"]
pub struct PRE107 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre107;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT107_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root107_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT107_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root107_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT107_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root107_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL107 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl107;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT107_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root107_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT107_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root107_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT107_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root107_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT108 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root108;
#[doc = "Target Register"]
pub struct TARGET_ROOT108_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root108_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT108_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root108_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT108_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root108_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC108 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc108;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT108_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root108_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT108_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root108_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT108_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root108_tog;
#[doc = "Post Divider Register"]
pub struct POST108 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post108;
#[doc = "Post Divider Register"]
pub struct POST_ROOT108_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root108_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT108_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root108_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT108_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root108_tog;
#[doc = "Pre Divider Register"]
pub struct PRE108 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre108;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT108_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root108_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT108_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root108_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT108_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root108_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL108 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl108;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT108_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root108_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT108_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root108_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT108_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root108_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT109 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root109;
#[doc = "Target Register"]
pub struct TARGET_ROOT109_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root109_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT109_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root109_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT109_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root109_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC109 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc109;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT109_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root109_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT109_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root109_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT109_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root109_tog;
#[doc = "Post Divider Register"]
pub struct POST109 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post109;
#[doc = "Post Divider Register"]
pub struct POST_ROOT109_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root109_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT109_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root109_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT109_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root109_tog;
#[doc = "Pre Divider Register"]
pub struct PRE109 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre109;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT109_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root109_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT109_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root109_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT109_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root109_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL109 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl109;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT109_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root109_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT109_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root109_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT109_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root109_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT110 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root110;
#[doc = "Target Register"]
pub struct TARGET_ROOT110_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root110_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT110_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root110_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT110_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root110_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC110 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc110;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT110_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root110_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT110_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root110_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT110_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root110_tog;
#[doc = "Post Divider Register"]
pub struct POST110 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post110;
#[doc = "Post Divider Register"]
pub struct POST_ROOT110_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root110_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT110_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root110_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT110_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root110_tog;
#[doc = "Pre Divider Register"]
pub struct PRE110 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre110;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT110_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root110_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT110_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root110_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT110_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root110_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL110 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl110;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT110_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root110_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT110_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root110_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT110_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root110_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT111 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root111;
#[doc = "Target Register"]
pub struct TARGET_ROOT111_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root111_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT111_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root111_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT111_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root111_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC111 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc111;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT111_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root111_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT111_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root111_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT111_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root111_tog;
#[doc = "Post Divider Register"]
pub struct POST111 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post111;
#[doc = "Post Divider Register"]
pub struct POST_ROOT111_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root111_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT111_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root111_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT111_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root111_tog;
#[doc = "Pre Divider Register"]
pub struct PRE111 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre111;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT111_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root111_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT111_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root111_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT111_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root111_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL111 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl111;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT111_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root111_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT111_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root111_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT111_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root111_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT112 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root112;
#[doc = "Target Register"]
pub struct TARGET_ROOT112_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root112_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT112_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root112_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT112_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root112_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC112 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc112;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT112_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root112_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT112_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root112_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT112_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root112_tog;
#[doc = "Post Divider Register"]
pub struct POST112 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post112;
#[doc = "Post Divider Register"]
pub struct POST_ROOT112_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root112_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT112_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root112_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT112_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root112_tog;
#[doc = "Pre Divider Register"]
pub struct PRE112 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre112;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT112_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root112_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT112_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root112_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT112_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root112_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL112 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl112;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT112_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root112_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT112_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root112_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT112_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root112_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT113 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root113;
#[doc = "Target Register"]
pub struct TARGET_ROOT113_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root113_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT113_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root113_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT113_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root113_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC113 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc113;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT113_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root113_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT113_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root113_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT113_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root113_tog;
#[doc = "Post Divider Register"]
pub struct POST113 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post113;
#[doc = "Post Divider Register"]
pub struct POST_ROOT113_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root113_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT113_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root113_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT113_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root113_tog;
#[doc = "Pre Divider Register"]
pub struct PRE113 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre113;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT113_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root113_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT113_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root113_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT113_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root113_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL113 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl113;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT113_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root113_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT113_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root113_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT113_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root113_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT114 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root114;
#[doc = "Target Register"]
pub struct TARGET_ROOT114_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root114_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT114_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root114_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT114_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root114_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC114 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc114;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT114_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root114_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT114_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root114_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT114_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root114_tog;
#[doc = "Post Divider Register"]
pub struct POST114 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post114;
#[doc = "Post Divider Register"]
pub struct POST_ROOT114_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root114_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT114_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root114_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT114_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root114_tog;
#[doc = "Pre Divider Register"]
pub struct PRE114 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre114;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT114_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root114_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT114_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root114_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT114_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root114_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL114 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl114;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT114_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root114_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT114_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root114_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT114_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root114_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT115 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root115;
#[doc = "Target Register"]
pub struct TARGET_ROOT115_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root115_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT115_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root115_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT115_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root115_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC115 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc115;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT115_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root115_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT115_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root115_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT115_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root115_tog;
#[doc = "Post Divider Register"]
pub struct POST115 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post115;
#[doc = "Post Divider Register"]
pub struct POST_ROOT115_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root115_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT115_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root115_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT115_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root115_tog;
#[doc = "Pre Divider Register"]
pub struct PRE115 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre115;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT115_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root115_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT115_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root115_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT115_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root115_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL115 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl115;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT115_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root115_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT115_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root115_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT115_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root115_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT116 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root116;
#[doc = "Target Register"]
pub struct TARGET_ROOT116_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root116_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT116_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root116_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT116_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root116_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC116 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc116;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT116_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root116_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT116_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root116_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT116_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root116_tog;
#[doc = "Post Divider Register"]
pub struct POST116 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post116;
#[doc = "Post Divider Register"]
pub struct POST_ROOT116_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root116_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT116_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root116_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT116_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root116_tog;
#[doc = "Pre Divider Register"]
pub struct PRE116 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre116;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT116_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root116_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT116_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root116_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT116_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root116_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL116 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl116;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT116_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root116_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT116_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root116_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT116_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root116_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT117 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root117;
#[doc = "Target Register"]
pub struct TARGET_ROOT117_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root117_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT117_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root117_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT117_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root117_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC117 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc117;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT117_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root117_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT117_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root117_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT117_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root117_tog;
#[doc = "Post Divider Register"]
pub struct POST117 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post117;
#[doc = "Post Divider Register"]
pub struct POST_ROOT117_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root117_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT117_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root117_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT117_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root117_tog;
#[doc = "Pre Divider Register"]
pub struct PRE117 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre117;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT117_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root117_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT117_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root117_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT117_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root117_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL117 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl117;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT117_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root117_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT117_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root117_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT117_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root117_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT118 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root118;
#[doc = "Target Register"]
pub struct TARGET_ROOT118_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root118_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT118_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root118_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT118_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root118_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC118 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc118;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT118_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root118_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT118_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root118_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT118_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root118_tog;
#[doc = "Post Divider Register"]
pub struct POST118 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post118;
#[doc = "Post Divider Register"]
pub struct POST_ROOT118_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root118_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT118_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root118_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT118_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root118_tog;
#[doc = "Pre Divider Register"]
pub struct PRE118 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre118;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT118_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root118_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT118_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root118_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT118_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root118_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL118 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl118;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT118_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root118_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT118_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root118_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT118_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root118_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT119 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root119;
#[doc = "Target Register"]
pub struct TARGET_ROOT119_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root119_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT119_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root119_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT119_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root119_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC119 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc119;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT119_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root119_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT119_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root119_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT119_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root119_tog;
#[doc = "Post Divider Register"]
pub struct POST119 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post119;
#[doc = "Post Divider Register"]
pub struct POST_ROOT119_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root119_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT119_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root119_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT119_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root119_tog;
#[doc = "Pre Divider Register"]
pub struct PRE119 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre119;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT119_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root119_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT119_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root119_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT119_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root119_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL119 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl119;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT119_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root119_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT119_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root119_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT119_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root119_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT120 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root120;
#[doc = "Target Register"]
pub struct TARGET_ROOT120_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root120_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT120_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root120_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT120_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root120_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC120 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc120;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT120_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root120_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT120_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root120_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT120_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root120_tog;
#[doc = "Post Divider Register"]
pub struct POST120 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post120;
#[doc = "Post Divider Register"]
pub struct POST_ROOT120_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root120_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT120_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root120_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT120_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root120_tog;
#[doc = "Pre Divider Register"]
pub struct PRE120 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre120;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT120_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root120_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT120_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root120_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT120_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root120_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL120 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl120;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT120_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root120_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT120_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root120_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT120_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root120_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT121 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root121;
#[doc = "Target Register"]
pub struct TARGET_ROOT121_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root121_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT121_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root121_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT121_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root121_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC121 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc121;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT121_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root121_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT121_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root121_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT121_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root121_tog;
#[doc = "Post Divider Register"]
pub struct POST121 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post121;
#[doc = "Post Divider Register"]
pub struct POST_ROOT121_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root121_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT121_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root121_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT121_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root121_tog;
#[doc = "Pre Divider Register"]
pub struct PRE121 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre121;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT121_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root121_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT121_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root121_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT121_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root121_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL121 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl121;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT121_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root121_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT121_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root121_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT121_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root121_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT122 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root122;
#[doc = "Target Register"]
pub struct TARGET_ROOT122_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root122_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT122_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root122_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT122_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root122_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC122 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc122;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT122_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root122_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT122_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root122_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT122_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root122_tog;
#[doc = "Post Divider Register"]
pub struct POST122 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post122;
#[doc = "Post Divider Register"]
pub struct POST_ROOT122_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root122_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT122_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root122_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT122_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root122_tog;
#[doc = "Pre Divider Register"]
pub struct PRE122 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre122;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT122_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root122_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT122_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root122_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT122_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root122_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL122 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl122;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT122_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root122_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT122_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root122_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT122_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root122_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT123 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root123;
#[doc = "Target Register"]
pub struct TARGET_ROOT123_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root123_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT123_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root123_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT123_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root123_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC123 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc123;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT123_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root123_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT123_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root123_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT123_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root123_tog;
#[doc = "Post Divider Register"]
pub struct POST123 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post123;
#[doc = "Post Divider Register"]
pub struct POST_ROOT123_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root123_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT123_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root123_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT123_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root123_tog;
#[doc = "Pre Divider Register"]
pub struct PRE123 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre123;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT123_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root123_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT123_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root123_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT123_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root123_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL123 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl123;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT123_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root123_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT123_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root123_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT123_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root123_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT124 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root124;
#[doc = "Target Register"]
pub struct TARGET_ROOT124_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root124_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT124_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root124_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT124_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root124_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC124 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc124;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT124_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root124_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT124_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root124_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT124_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root124_tog;
#[doc = "Post Divider Register"]
pub struct POST124 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post124;
#[doc = "Post Divider Register"]
pub struct POST_ROOT124_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root124_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT124_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root124_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT124_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root124_tog;
#[doc = "Pre Divider Register"]
pub struct PRE124 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre124;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT124_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root124_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT124_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root124_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT124_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root124_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL124 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl124;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT124_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root124_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT124_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root124_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT124_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root124_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT125 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root125;
#[doc = "Target Register"]
pub struct TARGET_ROOT125_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root125_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT125_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root125_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT125_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root125_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC125 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc125;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT125_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root125_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT125_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root125_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT125_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root125_tog;
#[doc = "Post Divider Register"]
pub struct POST125 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post125;
#[doc = "Post Divider Register"]
pub struct POST_ROOT125_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root125_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT125_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root125_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT125_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root125_tog;
#[doc = "Pre Divider Register"]
pub struct PRE125 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre125;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT125_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root125_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT125_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root125_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT125_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root125_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL125 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl125;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT125_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root125_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT125_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root125_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT125_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root125_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT126 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root126;
#[doc = "Target Register"]
pub struct TARGET_ROOT126_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root126_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT126_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root126_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT126_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root126_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC126 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc126;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT126_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root126_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT126_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root126_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT126_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root126_tog;
#[doc = "Post Divider Register"]
pub struct POST126 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post126;
#[doc = "Post Divider Register"]
pub struct POST_ROOT126_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root126_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT126_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root126_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT126_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root126_tog;
#[doc = "Pre Divider Register"]
pub struct PRE126 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre126;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT126_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root126_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT126_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root126_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT126_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root126_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL126 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl126;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT126_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root126_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT126_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root126_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT126_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root126_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT127 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root127;
#[doc = "Target Register"]
pub struct TARGET_ROOT127_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root127_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT127_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root127_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT127_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root127_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC127 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc127;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT127_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root127_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT127_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root127_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT127_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root127_tog;
#[doc = "Post Divider Register"]
pub struct POST127 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post127;
#[doc = "Post Divider Register"]
pub struct POST_ROOT127_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root127_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT127_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root127_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT127_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root127_tog;
#[doc = "Pre Divider Register"]
pub struct PRE127 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre127;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT127_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root127_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT127_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root127_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT127_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root127_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL127 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl127;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT127_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root127_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT127_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root127_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT127_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root127_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT128 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root128;
#[doc = "Target Register"]
pub struct TARGET_ROOT128_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root128_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT128_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root128_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT128_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root128_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC128 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc128;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT128_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root128_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT128_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root128_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT128_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root128_tog;
#[doc = "Post Divider Register"]
pub struct POST128 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post128;
#[doc = "Post Divider Register"]
pub struct POST_ROOT128_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root128_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT128_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root128_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT128_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root128_tog;
#[doc = "Pre Divider Register"]
pub struct PRE128 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre128;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT128_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root128_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT128_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root128_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT128_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root128_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL128 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl128;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT128_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root128_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT128_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root128_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT128_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root128_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT129 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root129;
#[doc = "Target Register"]
pub struct TARGET_ROOT129_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root129_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT129_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root129_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT129_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root129_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC129 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc129;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT129_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root129_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT129_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root129_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT129_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root129_tog;
#[doc = "Post Divider Register"]
pub struct POST129 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post129;
#[doc = "Post Divider Register"]
pub struct POST_ROOT129_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root129_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT129_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root129_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT129_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root129_tog;
#[doc = "Pre Divider Register"]
pub struct PRE129 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre129;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT129_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root129_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT129_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root129_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT129_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root129_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL129 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl129;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT129_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root129_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT129_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root129_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT129_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root129_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT130 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root130;
#[doc = "Target Register"]
pub struct TARGET_ROOT130_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root130_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT130_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root130_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT130_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root130_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC130 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc130;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT130_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root130_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT130_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root130_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT130_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root130_tog;
#[doc = "Post Divider Register"]
pub struct POST130 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post130;
#[doc = "Post Divider Register"]
pub struct POST_ROOT130_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root130_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT130_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root130_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT130_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root130_tog;
#[doc = "Pre Divider Register"]
pub struct PRE130 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre130;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT130_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root130_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT130_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root130_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT130_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root130_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL130 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl130;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT130_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root130_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT130_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root130_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT130_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root130_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT131 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root131;
#[doc = "Target Register"]
pub struct TARGET_ROOT131_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root131_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT131_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root131_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT131_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root131_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC131 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc131;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT131_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root131_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT131_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root131_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT131_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root131_tog;
#[doc = "Post Divider Register"]
pub struct POST131 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post131;
#[doc = "Post Divider Register"]
pub struct POST_ROOT131_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root131_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT131_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root131_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT131_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root131_tog;
#[doc = "Pre Divider Register"]
pub struct PRE131 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre131;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT131_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root131_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT131_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root131_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT131_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root131_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL131 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl131;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT131_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root131_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT131_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root131_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT131_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root131_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT132 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root132;
#[doc = "Target Register"]
pub struct TARGET_ROOT132_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root132_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT132_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root132_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT132_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root132_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC132 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc132;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT132_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root132_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT132_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root132_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT132_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root132_tog;
#[doc = "Post Divider Register"]
pub struct POST132 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post132;
#[doc = "Post Divider Register"]
pub struct POST_ROOT132_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root132_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT132_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root132_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT132_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root132_tog;
#[doc = "Pre Divider Register"]
pub struct PRE132 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre132;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT132_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root132_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT132_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root132_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT132_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root132_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL132 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl132;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT132_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root132_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT132_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root132_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT132_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root132_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT133 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root133;
#[doc = "Target Register"]
pub struct TARGET_ROOT133_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root133_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT133_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root133_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT133_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root133_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC133 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc133;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT133_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root133_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT133_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root133_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT133_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root133_tog;
#[doc = "Post Divider Register"]
pub struct POST133 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post133;
#[doc = "Post Divider Register"]
pub struct POST_ROOT133_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root133_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT133_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root133_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT133_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root133_tog;
#[doc = "Pre Divider Register"]
pub struct PRE133 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre133;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT133_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root133_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT133_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root133_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT133_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root133_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL133 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl133;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT133_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root133_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT133_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root133_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT133_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root133_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT134 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root134;
#[doc = "Target Register"]
pub struct TARGET_ROOT134_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root134_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT134_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root134_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT134_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root134_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC134 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc134;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT134_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root134_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT134_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root134_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT134_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root134_tog;
#[doc = "Post Divider Register"]
pub struct POST134 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post134;
#[doc = "Post Divider Register"]
pub struct POST_ROOT134_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root134_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT134_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root134_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT134_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root134_tog;
#[doc = "Pre Divider Register"]
pub struct PRE134 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre134;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT134_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root134_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT134_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root134_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT134_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root134_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL134 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl134;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT134_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root134_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT134_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root134_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT134_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root134_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT135 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root135;
#[doc = "Target Register"]
pub struct TARGET_ROOT135_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root135_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT135_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root135_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT135_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root135_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC135 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc135;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT135_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root135_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT135_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root135_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT135_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root135_tog;
#[doc = "Post Divider Register"]
pub struct POST135 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post135;
#[doc = "Post Divider Register"]
pub struct POST_ROOT135_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root135_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT135_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root135_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT135_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root135_tog;
#[doc = "Pre Divider Register"]
pub struct PRE135 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre135;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT135_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root135_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT135_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root135_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT135_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root135_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL135 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl135;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT135_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root135_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT135_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root135_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT135_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root135_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT136 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root136;
#[doc = "Target Register"]
pub struct TARGET_ROOT136_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root136_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT136_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root136_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT136_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root136_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC136 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc136;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT136_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root136_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT136_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root136_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT136_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root136_tog;
#[doc = "Post Divider Register"]
pub struct POST136 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post136;
#[doc = "Post Divider Register"]
pub struct POST_ROOT136_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root136_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT136_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root136_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT136_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root136_tog;
#[doc = "Pre Divider Register"]
pub struct PRE136 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre136;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT136_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root136_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT136_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root136_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT136_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root136_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL136 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl136;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT136_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root136_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT136_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root136_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT136_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root136_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT137 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root137;
#[doc = "Target Register"]
pub struct TARGET_ROOT137_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root137_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT137_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root137_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT137_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root137_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC137 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc137;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT137_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root137_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT137_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root137_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT137_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root137_tog;
#[doc = "Post Divider Register"]
pub struct POST137 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post137;
#[doc = "Post Divider Register"]
pub struct POST_ROOT137_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root137_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT137_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root137_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT137_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root137_tog;
#[doc = "Pre Divider Register"]
pub struct PRE137 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre137;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT137_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root137_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT137_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root137_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT137_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root137_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL137 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl137;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT137_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root137_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT137_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root137_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT137_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root137_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT138 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root138;
#[doc = "Target Register"]
pub struct TARGET_ROOT138_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root138_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT138_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root138_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT138_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root138_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC138 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc138;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT138_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root138_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT138_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root138_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT138_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root138_tog;
#[doc = "Post Divider Register"]
pub struct POST138 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post138;
#[doc = "Post Divider Register"]
pub struct POST_ROOT138_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root138_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT138_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root138_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT138_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root138_tog;
#[doc = "Pre Divider Register"]
pub struct PRE138 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre138;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT138_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root138_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT138_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root138_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT138_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root138_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL138 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl138;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT138_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root138_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT138_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root138_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT138_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root138_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT139 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root139;
#[doc = "Target Register"]
pub struct TARGET_ROOT139_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root139_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT139_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root139_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT139_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root139_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC139 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc139;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT139_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root139_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT139_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root139_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT139_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root139_tog;
#[doc = "Post Divider Register"]
pub struct POST139 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post139;
#[doc = "Post Divider Register"]
pub struct POST_ROOT139_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root139_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT139_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root139_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT139_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root139_tog;
#[doc = "Pre Divider Register"]
pub struct PRE139 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre139;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT139_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root139_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT139_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root139_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT139_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root139_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL139 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl139;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT139_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root139_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT139_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root139_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT139_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root139_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT140 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root140;
#[doc = "Target Register"]
pub struct TARGET_ROOT140_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root140_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT140_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root140_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT140_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root140_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC140 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc140;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT140_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root140_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT140_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root140_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT140_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root140_tog;
#[doc = "Post Divider Register"]
pub struct POST140 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post140;
#[doc = "Post Divider Register"]
pub struct POST_ROOT140_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root140_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT140_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root140_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT140_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root140_tog;
#[doc = "Pre Divider Register"]
pub struct PRE140 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre140;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT140_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root140_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT140_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root140_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT140_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root140_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL140 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl140;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT140_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root140_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT140_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root140_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT140_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root140_tog;
#[doc = "Target Register"]
pub struct TARGET_ROOT141 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root141;
#[doc = "Target Register"]
pub struct TARGET_ROOT141_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root141_set;
#[doc = "Target Register"]
pub struct TARGET_ROOT141_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root141_clr;
#[doc = "Target Register"]
pub struct TARGET_ROOT141_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Target Register"]
pub mod target_root141_tog;
#[doc = "Miscellaneous Register"]
pub struct MISC141 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc141;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT141_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root141_set;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT141_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root141_clr;
#[doc = "Miscellaneous Register"]
pub struct MISC_ROOT141_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscellaneous Register"]
pub mod misc_root141_tog;
#[doc = "Post Divider Register"]
pub struct POST141 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post141;
#[doc = "Post Divider Register"]
pub struct POST_ROOT141_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root141_set;
#[doc = "Post Divider Register"]
pub struct POST_ROOT141_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root141_clr;
#[doc = "Post Divider Register"]
pub struct POST_ROOT141_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Post Divider Register"]
pub mod post_root141_tog;
#[doc = "Pre Divider Register"]
pub struct PRE141 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre141;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT141_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root141_set;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT141_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root141_clr;
#[doc = "Pre Divider Register"]
pub struct PRE_ROOT141_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre Divider Register"]
pub mod pre_root141_tog;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL141 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl141;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT141_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root141_set;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT141_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root141_clr;
#[doc = "Access Control Register"]
pub struct ACCESS_CTRL_ROOT141_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Control Register"]
pub mod access_ctrl_root141_tog;
