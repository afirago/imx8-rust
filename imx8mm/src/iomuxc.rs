#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 20usize],
    #[doc = "0x14 - Pad Mux Register"]
    pub sw_mux_ctl_pad_pmic_stby_req: SW_MUX_CTL_PAD_PMIC_STBY_REQ,
    #[doc = "0x18 - Pad Mux Register"]
    pub sw_mux_ctl_pad_pmic_on_req: SW_MUX_CTL_PAD_PMIC_ON_REQ,
    #[doc = "0x1c - Pad Mux Register"]
    pub sw_mux_ctl_pad_onoff: SW_MUX_CTL_PAD_ONOFF,
    #[doc = "0x20 - Pad Mux Register"]
    pub sw_mux_ctl_pad_por_b: SW_MUX_CTL_PAD_POR_B,
    #[doc = "0x24 - Pad Mux Register"]
    pub sw_mux_ctl_pad_rtc_reset_b: SW_MUX_CTL_PAD_RTC_RESET_B,
    #[doc = "0x28 - Pad Mux Register"]
    pub sw_mux_ctl_pad_gpio1_io00: SW_MUX_CTL_PAD_GPIO1_IO00,
    #[doc = "0x2c - Pad Mux Register"]
    pub sw_mux_ctl_pad_gpio1_io01: SW_MUX_CTL_PAD_GPIO1_IO01,
    #[doc = "0x30 - Pad Mux Register"]
    pub sw_mux_ctl_pad_gpio1_io02: SW_MUX_CTL_PAD_GPIO1_IO02,
    #[doc = "0x34 - Pad Mux Register"]
    pub sw_mux_ctl_pad_gpio1_io03: SW_MUX_CTL_PAD_GPIO1_IO03,
    #[doc = "0x38 - Pad Mux Register"]
    pub sw_mux_ctl_pad_gpio1_io04: SW_MUX_CTL_PAD_GPIO1_IO04,
    #[doc = "0x3c - Pad Mux Register"]
    pub sw_mux_ctl_pad_gpio1_io05: SW_MUX_CTL_PAD_GPIO1_IO05,
    #[doc = "0x40 - Pad Mux Register"]
    pub sw_mux_ctl_pad_gpio1_io06: SW_MUX_CTL_PAD_GPIO1_IO06,
    #[doc = "0x44 - Pad Mux Register"]
    pub sw_mux_ctl_pad_gpio1_io07: SW_MUX_CTL_PAD_GPIO1_IO07,
    #[doc = "0x48 - Pad Mux Register"]
    pub sw_mux_ctl_pad_gpio1_io08: SW_MUX_CTL_PAD_GPIO1_IO08,
    #[doc = "0x4c - Pad Mux Register"]
    pub sw_mux_ctl_pad_gpio1_io09: SW_MUX_CTL_PAD_GPIO1_IO09,
    #[doc = "0x50 - Pad Mux Register"]
    pub sw_mux_ctl_pad_gpio1_io10: SW_MUX_CTL_PAD_GPIO1_IO10,
    #[doc = "0x54 - Pad Mux Register"]
    pub sw_mux_ctl_pad_gpio1_io11: SW_MUX_CTL_PAD_GPIO1_IO11,
    #[doc = "0x58 - Pad Mux Register"]
    pub sw_mux_ctl_pad_gpio1_io12: SW_MUX_CTL_PAD_GPIO1_IO12,
    #[doc = "0x5c - Pad Mux Register"]
    pub sw_mux_ctl_pad_gpio1_io13: SW_MUX_CTL_PAD_GPIO1_IO13,
    #[doc = "0x60 - Pad Mux Register"]
    pub sw_mux_ctl_pad_gpio1_io14: SW_MUX_CTL_PAD_GPIO1_IO14,
    #[doc = "0x64 - Pad Mux Register"]
    pub sw_mux_ctl_pad_gpio1_io15: SW_MUX_CTL_PAD_GPIO1_IO15,
    #[doc = "0x68 - Pad Mux Register"]
    pub sw_mux_ctl_pad_enet_mdc: SW_MUX_CTL_PAD_ENET_MDC,
    #[doc = "0x6c - Pad Mux Register"]
    pub sw_mux_ctl_pad_enet_mdio: SW_MUX_CTL_PAD_ENET_MDIO,
    #[doc = "0x70 - Pad Mux Register"]
    pub sw_mux_ctl_pad_enet_td3: SW_MUX_CTL_PAD_ENET_TD3,
    #[doc = "0x74 - Pad Mux Register"]
    pub sw_mux_ctl_pad_enet_td2: SW_MUX_CTL_PAD_ENET_TD2,
    #[doc = "0x78 - Pad Mux Register"]
    pub sw_mux_ctl_pad_enet_td1: SW_MUX_CTL_PAD_ENET_TD1,
    #[doc = "0x7c - Pad Mux Register"]
    pub sw_mux_ctl_pad_enet_td0: SW_MUX_CTL_PAD_ENET_TD0,
    #[doc = "0x80 - Pad Mux Register"]
    pub sw_mux_ctl_pad_enet_tx_ctl: SW_MUX_CTL_PAD_ENET_TX_CTL,
    #[doc = "0x84 - Pad Mux Register"]
    pub sw_mux_ctl_pad_enet_txc: SW_MUX_CTL_PAD_ENET_TXC,
    #[doc = "0x88 - Pad Mux Register"]
    pub sw_mux_ctl_pad_enet_rx_ctl: SW_MUX_CTL_PAD_ENET_RX_CTL,
    #[doc = "0x8c - Pad Mux Register"]
    pub sw_mux_ctl_pad_enet_rxc: SW_MUX_CTL_PAD_ENET_RXC,
    #[doc = "0x90 - Pad Mux Register"]
    pub sw_mux_ctl_pad_enet_rd0: SW_MUX_CTL_PAD_ENET_RD0,
    #[doc = "0x94 - Pad Mux Register"]
    pub sw_mux_ctl_pad_enet_rd1: SW_MUX_CTL_PAD_ENET_RD1,
    #[doc = "0x98 - Pad Mux Register"]
    pub sw_mux_ctl_pad_enet_rd2: SW_MUX_CTL_PAD_ENET_RD2,
    #[doc = "0x9c - Pad Mux Register"]
    pub sw_mux_ctl_pad_enet_rd3: SW_MUX_CTL_PAD_ENET_RD3,
    #[doc = "0xa0 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sd1_clk: SW_MUX_CTL_PAD_SD1_CLK,
    #[doc = "0xa4 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sd1_cmd: SW_MUX_CTL_PAD_SD1_CMD,
    #[doc = "0xa8 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sd1_data0: SW_MUX_CTL_PAD_SD1_DATA0,
    #[doc = "0xac - Pad Mux Register"]
    pub sw_mux_ctl_pad_sd1_data1: SW_MUX_CTL_PAD_SD1_DATA1,
    #[doc = "0xb0 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sd1_data2: SW_MUX_CTL_PAD_SD1_DATA2,
    #[doc = "0xb4 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sd1_data3: SW_MUX_CTL_PAD_SD1_DATA3,
    #[doc = "0xb8 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sd1_data4: SW_MUX_CTL_PAD_SD1_DATA4,
    #[doc = "0xbc - Pad Mux Register"]
    pub sw_mux_ctl_pad_sd1_data5: SW_MUX_CTL_PAD_SD1_DATA5,
    #[doc = "0xc0 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sd1_data6: SW_MUX_CTL_PAD_SD1_DATA6,
    #[doc = "0xc4 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sd1_data7: SW_MUX_CTL_PAD_SD1_DATA7,
    #[doc = "0xc8 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sd1_reset_b: SW_MUX_CTL_PAD_SD1_RESET_B,
    #[doc = "0xcc - Pad Mux Register"]
    pub sw_mux_ctl_pad_sd1_strobe: SW_MUX_CTL_PAD_SD1_STROBE,
    #[doc = "0xd0 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sd2_cd_b: SW_MUX_CTL_PAD_SD2_CD_B,
    #[doc = "0xd4 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sd2_clk: SW_MUX_CTL_PAD_SD2_CLK,
    #[doc = "0xd8 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sd2_cmd: SW_MUX_CTL_PAD_SD2_CMD,
    #[doc = "0xdc - Pad Mux Register"]
    pub sw_mux_ctl_pad_sd2_data0: SW_MUX_CTL_PAD_SD2_DATA0,
    #[doc = "0xe0 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sd2_data1: SW_MUX_CTL_PAD_SD2_DATA1,
    #[doc = "0xe4 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sd2_data2: SW_MUX_CTL_PAD_SD2_DATA2,
    #[doc = "0xe8 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sd2_data3: SW_MUX_CTL_PAD_SD2_DATA3,
    #[doc = "0xec - Pad Mux Register"]
    pub sw_mux_ctl_pad_sd2_reset_b: SW_MUX_CTL_PAD_SD2_RESET_B,
    #[doc = "0xf0 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sd2_wp: SW_MUX_CTL_PAD_SD2_WP,
    #[doc = "0xf4 - Pad Mux Register"]
    pub sw_mux_ctl_pad_nand_ale: SW_MUX_CTL_PAD_NAND_ALE,
    #[doc = "0xf8 - Pad Mux Register"]
    pub sw_mux_ctl_pad_nand_ce0_b: SW_MUX_CTL_PAD_NAND_CE0_B,
    #[doc = "0xfc - Pad Mux Register"]
    pub sw_mux_ctl_pad_nand_ce1_b: SW_MUX_CTL_PAD_NAND_CE1_B,
    #[doc = "0x100 - Pad Mux Register"]
    pub sw_mux_ctl_pad_nand_ce2_b: SW_MUX_CTL_PAD_NAND_CE2_B,
    #[doc = "0x104 - Pad Mux Register"]
    pub sw_mux_ctl_pad_nand_ce3_b: SW_MUX_CTL_PAD_NAND_CE3_B,
    #[doc = "0x108 - Pad Mux Register"]
    pub sw_mux_ctl_pad_nand_cle: SW_MUX_CTL_PAD_NAND_CLE,
    #[doc = "0x10c - Pad Mux Register"]
    pub sw_mux_ctl_pad_nand_data00: SW_MUX_CTL_PAD_NAND_DATA00,
    #[doc = "0x110 - Pad Mux Register"]
    pub sw_mux_ctl_pad_nand_data01: SW_MUX_CTL_PAD_NAND_DATA01,
    #[doc = "0x114 - Pad Mux Register"]
    pub sw_mux_ctl_pad_nand_data02: SW_MUX_CTL_PAD_NAND_DATA02,
    #[doc = "0x118 - Pad Mux Register"]
    pub sw_mux_ctl_pad_nand_data03: SW_MUX_CTL_PAD_NAND_DATA03,
    #[doc = "0x11c - Pad Mux Register"]
    pub sw_mux_ctl_pad_nand_data04: SW_MUX_CTL_PAD_NAND_DATA04,
    #[doc = "0x120 - Pad Mux Register"]
    pub sw_mux_ctl_pad_nand_data05: SW_MUX_CTL_PAD_NAND_DATA05,
    #[doc = "0x124 - Pad Mux Register"]
    pub sw_mux_ctl_pad_nand_data06: SW_MUX_CTL_PAD_NAND_DATA06,
    #[doc = "0x128 - Pad Mux Register"]
    pub sw_mux_ctl_pad_nand_data07: SW_MUX_CTL_PAD_NAND_DATA07,
    #[doc = "0x12c - Pad Mux Register"]
    pub sw_mux_ctl_pad_nand_dqs: SW_MUX_CTL_PAD_NAND_DQS,
    #[doc = "0x130 - Pad Mux Register"]
    pub sw_mux_ctl_pad_nand_re_b: SW_MUX_CTL_PAD_NAND_RE_B,
    #[doc = "0x134 - Pad Mux Register"]
    pub sw_mux_ctl_pad_nand_ready_b: SW_MUX_CTL_PAD_NAND_READY_B,
    #[doc = "0x138 - Pad Mux Register"]
    pub sw_mux_ctl_pad_nand_we_b: SW_MUX_CTL_PAD_NAND_WE_B,
    #[doc = "0x13c - Pad Mux Register"]
    pub sw_mux_ctl_pad_nand_wp_b: SW_MUX_CTL_PAD_NAND_WP_B,
    #[doc = "0x140 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai5_rxfs: SW_MUX_CTL_PAD_SAI5_RXFS,
    #[doc = "0x144 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai5_rxc: SW_MUX_CTL_PAD_SAI5_RXC,
    #[doc = "0x148 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai5_rxd0: SW_MUX_CTL_PAD_SAI5_RXD0,
    #[doc = "0x14c - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai5_rxd1: SW_MUX_CTL_PAD_SAI5_RXD1,
    #[doc = "0x150 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai5_rxd2: SW_MUX_CTL_PAD_SAI5_RXD2,
    #[doc = "0x154 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai5_rxd3: SW_MUX_CTL_PAD_SAI5_RXD3,
    #[doc = "0x158 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai5_mclk: SW_MUX_CTL_PAD_SAI5_MCLK,
    #[doc = "0x15c - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai1_rxfs: SW_MUX_CTL_PAD_SAI1_RXFS,
    #[doc = "0x160 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai1_rxc: SW_MUX_CTL_PAD_SAI1_RXC,
    #[doc = "0x164 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai1_rxd0: SW_MUX_CTL_PAD_SAI1_RXD0,
    #[doc = "0x168 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai1_rxd1: SW_MUX_CTL_PAD_SAI1_RXD1,
    #[doc = "0x16c - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai1_rxd2: SW_MUX_CTL_PAD_SAI1_RXD2,
    #[doc = "0x170 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai1_rxd3: SW_MUX_CTL_PAD_SAI1_RXD3,
    #[doc = "0x174 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai1_rxd4: SW_MUX_CTL_PAD_SAI1_RXD4,
    #[doc = "0x178 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai1_rxd5: SW_MUX_CTL_PAD_SAI1_RXD5,
    #[doc = "0x17c - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai1_rxd6: SW_MUX_CTL_PAD_SAI1_RXD6,
    #[doc = "0x180 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai1_rxd7: SW_MUX_CTL_PAD_SAI1_RXD7,
    #[doc = "0x184 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai1_txfs: SW_MUX_CTL_PAD_SAI1_TXFS,
    #[doc = "0x188 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai1_txc: SW_MUX_CTL_PAD_SAI1_TXC,
    #[doc = "0x18c - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai1_txd0: SW_MUX_CTL_PAD_SAI1_TXD0,
    #[doc = "0x190 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai1_txd1: SW_MUX_CTL_PAD_SAI1_TXD1,
    #[doc = "0x194 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai1_txd2: SW_MUX_CTL_PAD_SAI1_TXD2,
    #[doc = "0x198 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai1_txd3: SW_MUX_CTL_PAD_SAI1_TXD3,
    #[doc = "0x19c - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai1_txd4: SW_MUX_CTL_PAD_SAI1_TXD4,
    #[doc = "0x1a0 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai1_txd5: SW_MUX_CTL_PAD_SAI1_TXD5,
    #[doc = "0x1a4 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai1_txd6: SW_MUX_CTL_PAD_SAI1_TXD6,
    #[doc = "0x1a8 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai1_txd7: SW_MUX_CTL_PAD_SAI1_TXD7,
    #[doc = "0x1ac - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai1_mclk: SW_MUX_CTL_PAD_SAI1_MCLK,
    #[doc = "0x1b0 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai2_rxfs: SW_MUX_CTL_PAD_SAI2_RXFS,
    #[doc = "0x1b4 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai2_rxc: SW_MUX_CTL_PAD_SAI2_RXC,
    #[doc = "0x1b8 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai2_rxd0: SW_MUX_CTL_PAD_SAI2_RXD0,
    #[doc = "0x1bc - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai2_txfs: SW_MUX_CTL_PAD_SAI2_TXFS,
    #[doc = "0x1c0 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai2_txc: SW_MUX_CTL_PAD_SAI2_TXC,
    #[doc = "0x1c4 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai2_txd0: SW_MUX_CTL_PAD_SAI2_TXD0,
    #[doc = "0x1c8 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai2_mclk: SW_MUX_CTL_PAD_SAI2_MCLK,
    #[doc = "0x1cc - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai3_rxfs: SW_MUX_CTL_PAD_SAI3_RXFS,
    #[doc = "0x1d0 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai3_rxc: SW_MUX_CTL_PAD_SAI3_RXC,
    #[doc = "0x1d4 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai3_rxd: SW_MUX_CTL_PAD_SAI3_RXD,
    #[doc = "0x1d8 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai3_txfs: SW_MUX_CTL_PAD_SAI3_TXFS,
    #[doc = "0x1dc - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai3_txc: SW_MUX_CTL_PAD_SAI3_TXC,
    #[doc = "0x1e0 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai3_txd: SW_MUX_CTL_PAD_SAI3_TXD,
    #[doc = "0x1e4 - Pad Mux Register"]
    pub sw_mux_ctl_pad_sai3_mclk: SW_MUX_CTL_PAD_SAI3_MCLK,
    #[doc = "0x1e8 - Pad Mux Register"]
    pub sw_mux_ctl_pad_spdif_tx: SW_MUX_CTL_PAD_SPDIF_TX,
    #[doc = "0x1ec - Pad Mux Register"]
    pub sw_mux_ctl_pad_spdif_rx: SW_MUX_CTL_PAD_SPDIF_RX,
    #[doc = "0x1f0 - Pad Mux Register"]
    pub sw_mux_ctl_pad_spdif_ext_clk: SW_MUX_CTL_PAD_SPDIF_EXT_CLK,
    #[doc = "0x1f4 - Pad Mux Register"]
    pub sw_mux_ctl_pad_ecspi1_sclk: SW_MUX_CTL_PAD_ECSPI1_SCLK,
    #[doc = "0x1f8 - Pad Mux Register"]
    pub sw_mux_ctl_pad_ecspi1_mosi: SW_MUX_CTL_PAD_ECSPI1_MOSI,
    #[doc = "0x1fc - Pad Mux Register"]
    pub sw_mux_ctl_pad_ecspi1_miso: SW_MUX_CTL_PAD_ECSPI1_MISO,
    #[doc = "0x200 - Pad Mux Register"]
    pub sw_mux_ctl_pad_ecspi1_ss0: SW_MUX_CTL_PAD_ECSPI1_SS0,
    #[doc = "0x204 - Pad Mux Register"]
    pub sw_mux_ctl_pad_ecspi2_sclk: SW_MUX_CTL_PAD_ECSPI2_SCLK,
    #[doc = "0x208 - Pad Mux Register"]
    pub sw_mux_ctl_pad_ecspi2_mosi: SW_MUX_CTL_PAD_ECSPI2_MOSI,
    #[doc = "0x20c - Pad Mux Register"]
    pub sw_mux_ctl_pad_ecspi2_miso: SW_MUX_CTL_PAD_ECSPI2_MISO,
    #[doc = "0x210 - Pad Mux Register"]
    pub sw_mux_ctl_pad_ecspi2_ss0: SW_MUX_CTL_PAD_ECSPI2_SS0,
    #[doc = "0x214 - Pad Mux Register"]
    pub sw_mux_ctl_pad_i2c1_scl: SW_MUX_CTL_PAD_I2C1_SCL,
    #[doc = "0x218 - Pad Mux Register"]
    pub sw_mux_ctl_pad_i2c1_sda: SW_MUX_CTL_PAD_I2C1_SDA,
    #[doc = "0x21c - Pad Mux Register"]
    pub sw_mux_ctl_pad_i2c2_scl: SW_MUX_CTL_PAD_I2C2_SCL,
    #[doc = "0x220 - Pad Mux Register"]
    pub sw_mux_ctl_pad_i2c2_sda: SW_MUX_CTL_PAD_I2C2_SDA,
    #[doc = "0x224 - Pad Mux Register"]
    pub sw_mux_ctl_pad_i2c3_scl: SW_MUX_CTL_PAD_I2C3_SCL,
    #[doc = "0x228 - Pad Mux Register"]
    pub sw_mux_ctl_pad_i2c3_sda: SW_MUX_CTL_PAD_I2C3_SDA,
    #[doc = "0x22c - Pad Mux Register"]
    pub sw_mux_ctl_pad_i2c4_scl: SW_MUX_CTL_PAD_I2C4_SCL,
    #[doc = "0x230 - Pad Mux Register"]
    pub sw_mux_ctl_pad_i2c4_sda: SW_MUX_CTL_PAD_I2C4_SDA,
    #[doc = "0x234 - Pad Mux Register"]
    pub sw_mux_ctl_pad_uart1_rxd: SW_MUX_CTL_PAD_UART1_RXD,
    #[doc = "0x238 - Pad Mux Register"]
    pub sw_mux_ctl_pad_uart1_txd: SW_MUX_CTL_PAD_UART1_TXD,
    #[doc = "0x23c - Pad Mux Register"]
    pub sw_mux_ctl_pad_uart2_rxd: SW_MUX_CTL_PAD_UART2_RXD,
    #[doc = "0x240 - Pad Mux Register"]
    pub sw_mux_ctl_pad_uart2_txd: SW_MUX_CTL_PAD_UART2_TXD,
    #[doc = "0x244 - Pad Mux Register"]
    pub sw_mux_ctl_pad_uart3_rxd: SW_MUX_CTL_PAD_UART3_RXD,
    #[doc = "0x248 - Pad Mux Register"]
    pub sw_mux_ctl_pad_uart3_txd: SW_MUX_CTL_PAD_UART3_TXD,
    #[doc = "0x24c - Pad Mux Register"]
    pub sw_mux_ctl_pad_uart4_rxd: SW_MUX_CTL_PAD_UART4_RXD,
    #[doc = "0x250 - Pad Mux Register"]
    pub sw_mux_ctl_pad_uart4_txd: SW_MUX_CTL_PAD_UART4_TXD,
    #[doc = "0x254 - Pad Control Register"]
    pub sw_pad_ctl_pad_test_mode: SW_PAD_CTL_PAD_TEST_MODE,
    #[doc = "0x258 - Pad Control Register"]
    pub sw_pad_ctl_pad_boot_mode0: SW_PAD_CTL_PAD_BOOT_MODE0,
    #[doc = "0x25c - Pad Control Register"]
    pub sw_pad_ctl_pad_boot_mode1: SW_PAD_CTL_PAD_BOOT_MODE1,
    #[doc = "0x260 - Pad Control Register"]
    pub sw_pad_ctl_pad_jtag_mod: SW_PAD_CTL_PAD_JTAG_MOD,
    #[doc = "0x264 - Pad Control Register"]
    pub sw_pad_ctl_pad_jtag_trst_b: SW_PAD_CTL_PAD_JTAG_TRST_B,
    #[doc = "0x268 - Pad Control Register"]
    pub sw_pad_ctl_pad_jtag_tdi: SW_PAD_CTL_PAD_JTAG_TDI,
    #[doc = "0x26c - Pad Control Register"]
    pub sw_pad_ctl_pad_jtag_tms: SW_PAD_CTL_PAD_JTAG_TMS,
    #[doc = "0x270 - Pad Control Register"]
    pub sw_pad_ctl_pad_jtag_tck: SW_PAD_CTL_PAD_JTAG_TCK,
    #[doc = "0x274 - Pad Control Register"]
    pub sw_pad_ctl_pad_jtag_tdo: SW_PAD_CTL_PAD_JTAG_TDO,
    #[doc = "0x278 - Pad Control Register"]
    pub sw_pad_ctl_pad_rtc: SW_PAD_CTL_PAD_RTC,
    #[doc = "0x27c - Pad Control Register"]
    pub sw_pad_ctl_pad_pmic_stby_req: SW_PAD_CTL_PAD_PMIC_STBY_REQ,
    #[doc = "0x280 - Pad Control Register"]
    pub sw_pad_ctl_pad_pmic_on_req: SW_PAD_CTL_PAD_PMIC_ON_REQ,
    #[doc = "0x284 - Pad Control Register"]
    pub sw_pad_ctl_pad_onoff: SW_PAD_CTL_PAD_ONOFF,
    #[doc = "0x288 - Pad Control Register"]
    pub sw_pad_ctl_pad_por_b: SW_PAD_CTL_PAD_POR_B,
    #[doc = "0x28c - Pad Control Register"]
    pub sw_pad_ctl_pad_rtc_reset_b: SW_PAD_CTL_PAD_RTC_RESET_B,
    #[doc = "0x290 - Pad Control Register"]
    pub sw_pad_ctl_pad_gpio1_io00: SW_PAD_CTL_PAD_GPIO1_IO00,
    #[doc = "0x294 - Pad Control Register"]
    pub sw_pad_ctl_pad_gpio1_io01: SW_PAD_CTL_PAD_GPIO1_IO01,
    #[doc = "0x298 - Pad Control Register"]
    pub sw_pad_ctl_pad_gpio1_io02: SW_PAD_CTL_PAD_GPIO1_IO02,
    #[doc = "0x29c - Pad Control Register"]
    pub sw_pad_ctl_pad_gpio1_io03: SW_PAD_CTL_PAD_GPIO1_IO03,
    #[doc = "0x2a0 - Pad Control Register"]
    pub sw_pad_ctl_pad_gpio1_io04: SW_PAD_CTL_PAD_GPIO1_IO04,
    #[doc = "0x2a4 - Pad Control Register"]
    pub sw_pad_ctl_pad_gpio1_io05: SW_PAD_CTL_PAD_GPIO1_IO05,
    #[doc = "0x2a8 - Pad Control Register"]
    pub sw_pad_ctl_pad_gpio1_io06: SW_PAD_CTL_PAD_GPIO1_IO06,
    #[doc = "0x2ac - Pad Control Register"]
    pub sw_pad_ctl_pad_gpio1_io07: SW_PAD_CTL_PAD_GPIO1_IO07,
    #[doc = "0x2b0 - Pad Control Register"]
    pub sw_pad_ctl_pad_gpio1_io08: SW_PAD_CTL_PAD_GPIO1_IO08,
    #[doc = "0x2b4 - Pad Control Register"]
    pub sw_pad_ctl_pad_gpio1_io09: SW_PAD_CTL_PAD_GPIO1_IO09,
    #[doc = "0x2b8 - Pad Control Register"]
    pub sw_pad_ctl_pad_gpio1_io10: SW_PAD_CTL_PAD_GPIO1_IO10,
    #[doc = "0x2bc - Pad Control Register"]
    pub sw_pad_ctl_pad_gpio1_io11: SW_PAD_CTL_PAD_GPIO1_IO11,
    #[doc = "0x2c0 - Pad Control Register"]
    pub sw_pad_ctl_pad_gpio1_io12: SW_PAD_CTL_PAD_GPIO1_IO12,
    #[doc = "0x2c4 - Pad Control Register"]
    pub sw_pad_ctl_pad_gpio1_io13: SW_PAD_CTL_PAD_GPIO1_IO13,
    #[doc = "0x2c8 - Pad Control Register"]
    pub sw_pad_ctl_pad_gpio1_io14: SW_PAD_CTL_PAD_GPIO1_IO14,
    #[doc = "0x2cc - Pad Control Register"]
    pub sw_pad_ctl_pad_gpio1_io15: SW_PAD_CTL_PAD_GPIO1_IO15,
    #[doc = "0x2d0 - Pad Control Register"]
    pub sw_pad_ctl_pad_enet_mdc: SW_PAD_CTL_PAD_ENET_MDC,
    #[doc = "0x2d4 - Pad Control Register"]
    pub sw_pad_ctl_pad_enet_mdio: SW_PAD_CTL_PAD_ENET_MDIO,
    #[doc = "0x2d8 - Pad Control Register"]
    pub sw_pad_ctl_pad_enet_td3: SW_PAD_CTL_PAD_ENET_TD3,
    #[doc = "0x2dc - Pad Control Register"]
    pub sw_pad_ctl_pad_enet_td2: SW_PAD_CTL_PAD_ENET_TD2,
    #[doc = "0x2e0 - Pad Control Register"]
    pub sw_pad_ctl_pad_enet_td1: SW_PAD_CTL_PAD_ENET_TD1,
    #[doc = "0x2e4 - Pad Control Register"]
    pub sw_pad_ctl_pad_enet_td0: SW_PAD_CTL_PAD_ENET_TD0,
    #[doc = "0x2e8 - Pad Control Register"]
    pub sw_pad_ctl_pad_enet_tx_ctl: SW_PAD_CTL_PAD_ENET_TX_CTL,
    #[doc = "0x2ec - Pad Control Register"]
    pub sw_pad_ctl_pad_enet_txc: SW_PAD_CTL_PAD_ENET_TXC,
    #[doc = "0x2f0 - Pad Control Register"]
    pub sw_pad_ctl_pad_enet_rx_ctl: SW_PAD_CTL_PAD_ENET_RX_CTL,
    #[doc = "0x2f4 - Pad Control Register"]
    pub sw_pad_ctl_pad_enet_rxc: SW_PAD_CTL_PAD_ENET_RXC,
    #[doc = "0x2f8 - Pad Control Register"]
    pub sw_pad_ctl_pad_enet_rd0: SW_PAD_CTL_PAD_ENET_RD0,
    #[doc = "0x2fc - Pad Control Register"]
    pub sw_pad_ctl_pad_enet_rd1: SW_PAD_CTL_PAD_ENET_RD1,
    #[doc = "0x300 - Pad Control Register"]
    pub sw_pad_ctl_pad_enet_rd2: SW_PAD_CTL_PAD_ENET_RD2,
    #[doc = "0x304 - Pad Control Register"]
    pub sw_pad_ctl_pad_enet_rd3: SW_PAD_CTL_PAD_ENET_RD3,
    #[doc = "0x308 - Pad Control Register"]
    pub sw_pad_ctl_pad_sd1_clk: SW_PAD_CTL_PAD_SD1_CLK,
    #[doc = "0x30c - Pad Control Register"]
    pub sw_pad_ctl_pad_sd1_cmd: SW_PAD_CTL_PAD_SD1_CMD,
    #[doc = "0x310 - Pad Control Register"]
    pub sw_pad_ctl_pad_sd1_data0: SW_PAD_CTL_PAD_SD1_DATA0,
    #[doc = "0x314 - Pad Control Register"]
    pub sw_pad_ctl_pad_sd1_data1: SW_PAD_CTL_PAD_SD1_DATA1,
    #[doc = "0x318 - Pad Control Register"]
    pub sw_pad_ctl_pad_sd1_data2: SW_PAD_CTL_PAD_SD1_DATA2,
    #[doc = "0x31c - Pad Control Register"]
    pub sw_pad_ctl_pad_sd1_data3: SW_PAD_CTL_PAD_SD1_DATA3,
    #[doc = "0x320 - Pad Control Register"]
    pub sw_pad_ctl_pad_sd1_data4: SW_PAD_CTL_PAD_SD1_DATA4,
    #[doc = "0x324 - Pad Control Register"]
    pub sw_pad_ctl_pad_sd1_data5: SW_PAD_CTL_PAD_SD1_DATA5,
    #[doc = "0x328 - Pad Control Register"]
    pub sw_pad_ctl_pad_sd1_data6: SW_PAD_CTL_PAD_SD1_DATA6,
    #[doc = "0x32c - Pad Control Register"]
    pub sw_pad_ctl_pad_sd1_data7: SW_PAD_CTL_PAD_SD1_DATA7,
    #[doc = "0x330 - Pad Control Register"]
    pub sw_pad_ctl_pad_sd1_reset_b: SW_PAD_CTL_PAD_SD1_RESET_B,
    #[doc = "0x334 - Pad Control Register"]
    pub sw_pad_ctl_pad_sd1_strobe: SW_PAD_CTL_PAD_SD1_STROBE,
    #[doc = "0x338 - Pad Control Register"]
    pub sw_pad_ctl_pad_sd2_cd_b: SW_PAD_CTL_PAD_SD2_CD_B,
    #[doc = "0x33c - Pad Control Register"]
    pub sw_pad_ctl_pad_sd2_clk: SW_PAD_CTL_PAD_SD2_CLK,
    #[doc = "0x340 - Pad Control Register"]
    pub sw_pad_ctl_pad_sd2_cmd: SW_PAD_CTL_PAD_SD2_CMD,
    #[doc = "0x344 - Pad Control Register"]
    pub sw_pad_ctl_pad_sd2_data0: SW_PAD_CTL_PAD_SD2_DATA0,
    #[doc = "0x348 - Pad Control Register"]
    pub sw_pad_ctl_pad_sd2_data1: SW_PAD_CTL_PAD_SD2_DATA1,
    #[doc = "0x34c - Pad Control Register"]
    pub sw_pad_ctl_pad_sd2_data2: SW_PAD_CTL_PAD_SD2_DATA2,
    #[doc = "0x350 - Pad Control Register"]
    pub sw_pad_ctl_pad_sd2_data3: SW_PAD_CTL_PAD_SD2_DATA3,
    #[doc = "0x354 - Pad Control Register"]
    pub sw_pad_ctl_pad_sd2_reset_b: SW_PAD_CTL_PAD_SD2_RESET_B,
    #[doc = "0x358 - Pad Control Register"]
    pub sw_pad_ctl_pad_sd2_wp: SW_PAD_CTL_PAD_SD2_WP,
    #[doc = "0x35c - Pad Control Register"]
    pub sw_pad_ctl_pad_nand_ale: SW_PAD_CTL_PAD_NAND_ALE,
    #[doc = "0x360 - Pad Control Register"]
    pub sw_pad_ctl_pad_nand_ce0_b: SW_PAD_CTL_PAD_NAND_CE0_B,
    #[doc = "0x364 - Pad Control Register"]
    pub sw_pad_ctl_pad_nand_ce1_b: SW_PAD_CTL_PAD_NAND_CE1_B,
    #[doc = "0x368 - Pad Control Register"]
    pub sw_pad_ctl_pad_nand_ce2_b: SW_PAD_CTL_PAD_NAND_CE2_B,
    #[doc = "0x36c - Pad Control Register"]
    pub sw_pad_ctl_pad_nand_ce3_b: SW_PAD_CTL_PAD_NAND_CE3_B,
    #[doc = "0x370 - Pad Control Register"]
    pub sw_pad_ctl_pad_nand_cle: SW_PAD_CTL_PAD_NAND_CLE,
    #[doc = "0x374 - Pad Control Register"]
    pub sw_pad_ctl_pad_nand_data00: SW_PAD_CTL_PAD_NAND_DATA00,
    #[doc = "0x378 - Pad Control Register"]
    pub sw_pad_ctl_pad_nand_data01: SW_PAD_CTL_PAD_NAND_DATA01,
    #[doc = "0x37c - Pad Control Register"]
    pub sw_pad_ctl_pad_nand_data02: SW_PAD_CTL_PAD_NAND_DATA02,
    #[doc = "0x380 - Pad Control Register"]
    pub sw_pad_ctl_pad_nand_data03: SW_PAD_CTL_PAD_NAND_DATA03,
    #[doc = "0x384 - Pad Control Register"]
    pub sw_pad_ctl_pad_nand_data04: SW_PAD_CTL_PAD_NAND_DATA04,
    #[doc = "0x388 - Pad Control Register"]
    pub sw_pad_ctl_pad_nand_data05: SW_PAD_CTL_PAD_NAND_DATA05,
    #[doc = "0x38c - Pad Control Register"]
    pub sw_pad_ctl_pad_nand_data06: SW_PAD_CTL_PAD_NAND_DATA06,
    #[doc = "0x390 - Pad Control Register"]
    pub sw_pad_ctl_pad_nand_data07: SW_PAD_CTL_PAD_NAND_DATA07,
    #[doc = "0x394 - Pad Control Register"]
    pub sw_pad_ctl_pad_nand_dqs: SW_PAD_CTL_PAD_NAND_DQS,
    #[doc = "0x398 - Pad Control Register"]
    pub sw_pad_ctl_pad_nand_re_b: SW_PAD_CTL_PAD_NAND_RE_B,
    #[doc = "0x39c - Pad Control Register"]
    pub sw_pad_ctl_pad_nand_ready_b: SW_PAD_CTL_PAD_NAND_READY_B,
    #[doc = "0x3a0 - Pad Control Register"]
    pub sw_pad_ctl_pad_nand_we_b: SW_PAD_CTL_PAD_NAND_WE_B,
    #[doc = "0x3a4 - Pad Control Register"]
    pub sw_pad_ctl_pad_nand_wp_b: SW_PAD_CTL_PAD_NAND_WP_B,
    #[doc = "0x3a8 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai5_rxfs: SW_PAD_CTL_PAD_SAI5_RXFS,
    #[doc = "0x3ac - Pad Control Register"]
    pub sw_pad_ctl_pad_sai5_rxc: SW_PAD_CTL_PAD_SAI5_RXC,
    #[doc = "0x3b0 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai5_rxd0: SW_PAD_CTL_PAD_SAI5_RXD0,
    #[doc = "0x3b4 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai5_rxd1: SW_PAD_CTL_PAD_SAI5_RXD1,
    #[doc = "0x3b8 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai5_rxd2: SW_PAD_CTL_PAD_SAI5_RXD2,
    #[doc = "0x3bc - Pad Control Register"]
    pub sw_pad_ctl_pad_sai5_rxd3: SW_PAD_CTL_PAD_SAI5_RXD3,
    #[doc = "0x3c0 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai5_mclk: SW_PAD_CTL_PAD_SAI5_MCLK,
    #[doc = "0x3c4 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai1_rxfs: SW_PAD_CTL_PAD_SAI1_RXFS,
    #[doc = "0x3c8 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai1_rxc: SW_PAD_CTL_PAD_SAI1_RXC,
    #[doc = "0x3cc - Pad Control Register"]
    pub sw_pad_ctl_pad_sai1_rxd0: SW_PAD_CTL_PAD_SAI1_RXD0,
    #[doc = "0x3d0 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai1_rxd1: SW_PAD_CTL_PAD_SAI1_RXD1,
    #[doc = "0x3d4 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai1_rxd2: SW_PAD_CTL_PAD_SAI1_RXD2,
    #[doc = "0x3d8 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai1_rxd3: SW_PAD_CTL_PAD_SAI1_RXD3,
    #[doc = "0x3dc - Pad Control Register"]
    pub sw_pad_ctl_pad_sai1_rxd4: SW_PAD_CTL_PAD_SAI1_RXD4,
    #[doc = "0x3e0 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai1_rxd5: SW_PAD_CTL_PAD_SAI1_RXD5,
    #[doc = "0x3e4 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai1_rxd6: SW_PAD_CTL_PAD_SAI1_RXD6,
    #[doc = "0x3e8 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai1_rxd7: SW_PAD_CTL_PAD_SAI1_RXD7,
    #[doc = "0x3ec - Pad Control Register"]
    pub sw_pad_ctl_pad_sai1_txfs: SW_PAD_CTL_PAD_SAI1_TXFS,
    #[doc = "0x3f0 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai1_txc: SW_PAD_CTL_PAD_SAI1_TXC,
    #[doc = "0x3f4 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai1_txd0: SW_PAD_CTL_PAD_SAI1_TXD0,
    #[doc = "0x3f8 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai1_txd1: SW_PAD_CTL_PAD_SAI1_TXD1,
    #[doc = "0x3fc - Pad Control Register"]
    pub sw_pad_ctl_pad_sai1_txd2: SW_PAD_CTL_PAD_SAI1_TXD2,
    #[doc = "0x400 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai1_txd3: SW_PAD_CTL_PAD_SAI1_TXD3,
    #[doc = "0x404 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai1_txd4: SW_PAD_CTL_PAD_SAI1_TXD4,
    #[doc = "0x408 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai1_txd5: SW_PAD_CTL_PAD_SAI1_TXD5,
    #[doc = "0x40c - Pad Control Register"]
    pub sw_pad_ctl_pad_sai1_txd6: SW_PAD_CTL_PAD_SAI1_TXD6,
    #[doc = "0x410 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai1_txd7: SW_PAD_CTL_PAD_SAI1_TXD7,
    #[doc = "0x414 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai1_mclk: SW_PAD_CTL_PAD_SAI1_MCLK,
    #[doc = "0x418 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai2_rxfs: SW_PAD_CTL_PAD_SAI2_RXFS,
    #[doc = "0x41c - Pad Control Register"]
    pub sw_pad_ctl_pad_sai2_rxc: SW_PAD_CTL_PAD_SAI2_RXC,
    #[doc = "0x420 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai2_rxd0: SW_PAD_CTL_PAD_SAI2_RXD0,
    #[doc = "0x424 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai2_txfs: SW_PAD_CTL_PAD_SAI2_TXFS,
    #[doc = "0x428 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai2_txc: SW_PAD_CTL_PAD_SAI2_TXC,
    #[doc = "0x42c - Pad Control Register"]
    pub sw_pad_ctl_pad_sai2_txd0: SW_PAD_CTL_PAD_SAI2_TXD0,
    #[doc = "0x430 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai2_mclk: SW_PAD_CTL_PAD_SAI2_MCLK,
    #[doc = "0x434 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai3_rxfs: SW_PAD_CTL_PAD_SAI3_RXFS,
    #[doc = "0x438 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai3_rxc: SW_PAD_CTL_PAD_SAI3_RXC,
    #[doc = "0x43c - Pad Control Register"]
    pub sw_pad_ctl_pad_sai3_rxd: SW_PAD_CTL_PAD_SAI3_RXD,
    #[doc = "0x440 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai3_txfs: SW_PAD_CTL_PAD_SAI3_TXFS,
    #[doc = "0x444 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai3_txc: SW_PAD_CTL_PAD_SAI3_TXC,
    #[doc = "0x448 - Pad Control Register"]
    pub sw_pad_ctl_pad_sai3_txd: SW_PAD_CTL_PAD_SAI3_TXD,
    #[doc = "0x44c - Pad Control Register"]
    pub sw_pad_ctl_pad_sai3_mclk: SW_PAD_CTL_PAD_SAI3_MCLK,
    #[doc = "0x450 - Pad Control Register"]
    pub sw_pad_ctl_pad_spdif_tx: SW_PAD_CTL_PAD_SPDIF_TX,
    #[doc = "0x454 - Pad Control Register"]
    pub sw_pad_ctl_pad_spdif_rx: SW_PAD_CTL_PAD_SPDIF_RX,
    #[doc = "0x458 - Pad Control Register"]
    pub sw_pad_ctl_pad_spdif_ext_clk: SW_PAD_CTL_PAD_SPDIF_EXT_CLK,
    #[doc = "0x45c - Pad Control Register"]
    pub sw_pad_ctl_pad_ecspi1_sclk: SW_PAD_CTL_PAD_ECSPI1_SCLK,
    #[doc = "0x460 - Pad Control Register"]
    pub sw_pad_ctl_pad_ecspi1_mosi: SW_PAD_CTL_PAD_ECSPI1_MOSI,
    #[doc = "0x464 - Pad Control Register"]
    pub sw_pad_ctl_pad_ecspi1_miso: SW_PAD_CTL_PAD_ECSPI1_MISO,
    #[doc = "0x468 - Pad Control Register"]
    pub sw_pad_ctl_pad_ecspi1_ss0: SW_PAD_CTL_PAD_ECSPI1_SS0,
    #[doc = "0x46c - Pad Control Register"]
    pub sw_pad_ctl_pad_ecspi2_sclk: SW_PAD_CTL_PAD_ECSPI2_SCLK,
    #[doc = "0x470 - Pad Control Register"]
    pub sw_pad_ctl_pad_ecspi2_mosi: SW_PAD_CTL_PAD_ECSPI2_MOSI,
    #[doc = "0x474 - Pad Control Register"]
    pub sw_pad_ctl_pad_ecspi2_miso: SW_PAD_CTL_PAD_ECSPI2_MISO,
    #[doc = "0x478 - Pad Control Register"]
    pub sw_pad_ctl_pad_ecspi2_ss0: SW_PAD_CTL_PAD_ECSPI2_SS0,
    #[doc = "0x47c - Pad Control Register"]
    pub sw_pad_ctl_pad_i2c1_scl: SW_PAD_CTL_PAD_I2C1_SCL,
    #[doc = "0x480 - Pad Control Register"]
    pub sw_pad_ctl_pad_i2c1_sda: SW_PAD_CTL_PAD_I2C1_SDA,
    #[doc = "0x484 - Pad Control Register"]
    pub sw_pad_ctl_pad_i2c2_scl: SW_PAD_CTL_PAD_I2C2_SCL,
    #[doc = "0x488 - Pad Control Register"]
    pub sw_pad_ctl_pad_i2c2_sda: SW_PAD_CTL_PAD_I2C2_SDA,
    #[doc = "0x48c - Pad Control Register"]
    pub sw_pad_ctl_pad_i2c3_scl: SW_PAD_CTL_PAD_I2C3_SCL,
    #[doc = "0x490 - Pad Control Register"]
    pub sw_pad_ctl_pad_i2c3_sda: SW_PAD_CTL_PAD_I2C3_SDA,
    #[doc = "0x494 - Pad Control Register"]
    pub sw_pad_ctl_pad_i2c4_scl: SW_PAD_CTL_PAD_I2C4_SCL,
    #[doc = "0x498 - Pad Control Register"]
    pub sw_pad_ctl_pad_i2c4_sda: SW_PAD_CTL_PAD_I2C4_SDA,
    #[doc = "0x49c - Pad Control Register"]
    pub sw_pad_ctl_pad_uart1_rxd: SW_PAD_CTL_PAD_UART1_RXD,
    #[doc = "0x4a0 - Pad Control Register"]
    pub sw_pad_ctl_pad_uart1_txd: SW_PAD_CTL_PAD_UART1_TXD,
    #[doc = "0x4a4 - Pad Control Register"]
    pub sw_pad_ctl_pad_uart2_rxd: SW_PAD_CTL_PAD_UART2_RXD,
    #[doc = "0x4a8 - Pad Control Register"]
    pub sw_pad_ctl_pad_uart2_txd: SW_PAD_CTL_PAD_UART2_TXD,
    #[doc = "0x4ac - Pad Control Register"]
    pub sw_pad_ctl_pad_uart3_rxd: SW_PAD_CTL_PAD_UART3_RXD,
    #[doc = "0x4b0 - Pad Control Register"]
    pub sw_pad_ctl_pad_uart3_txd: SW_PAD_CTL_PAD_UART3_TXD,
    #[doc = "0x4b4 - Pad Control Register"]
    pub sw_pad_ctl_pad_uart4_rxd: SW_PAD_CTL_PAD_UART4_RXD,
    #[doc = "0x4b8 - Pad Control Register"]
    pub sw_pad_ctl_pad_uart4_txd: SW_PAD_CTL_PAD_UART4_TXD,
    #[doc = "0x4bc - CCM_PMIC_READY_SELECT_INPUT DAISY Register"]
    pub ccm_pmic_ready_select_input: CCM_PMIC_READY_SELECT_INPUT,
    #[doc = "0x4c0 - Select Input Register"]
    pub enet1_mdio_select_input: ENET1_MDIO_SELECT_INPUT,
    #[doc = "0x4c4 - Select Input Register"]
    pub sai1_rx_sync_select_input: SAI1_RX_SYNC_SELECT_INPUT,
    #[doc = "0x4c8 - Select Input Register"]
    pub sai1_tx_bclk_select_input: SAI1_TX_BCLK_SELECT_INPUT,
    #[doc = "0x4cc - Select Input Register"]
    pub sai1_tx_sync_select_input: SAI1_TX_SYNC_SELECT_INPUT,
    #[doc = "0x4d0 - Select Input Register"]
    pub sai5_rx_bclk_select_input: SAI5_RX_BCLK_SELECT_INPUT,
    #[doc = "0x4d4 - Select Input Register"]
    pub sai5_rx_data_select_input_0: SAI5_RX_DATA_SELECT_INPUT_0,
    #[doc = "0x4d8 - Select Input Register"]
    pub sai5_rx_data_select_input_1: SAI5_RX_DATA_SELECT_INPUT_1,
    #[doc = "0x4dc - Select Input Register"]
    pub sai5_rx_data_select_input_2: SAI5_RX_DATA_SELECT_INPUT_2,
    #[doc = "0x4e0 - Select Input Register"]
    pub sai5_rx_data_select_input_3: SAI5_RX_DATA_SELECT_INPUT_3,
    #[doc = "0x4e4 - Select Input Register"]
    pub sai5_rx_sync_select_input: SAI5_RX_SYNC_SELECT_INPUT,
    #[doc = "0x4e8 - Select Input Register"]
    pub sai5_tx_bclk_select_input: SAI5_TX_BCLK_SELECT_INPUT,
    #[doc = "0x4ec - Select Input Register"]
    pub sai5_tx_sync_select_input: SAI5_TX_SYNC_SELECT_INPUT,
    #[doc = "0x4f0 - Select Input Register"]
    pub uart1_rts_b_select_input: UART1_RTS_B_SELECT_INPUT,
    #[doc = "0x4f4 - Select Input Register"]
    pub uart1_rxd_select_input: UART1_RXD_SELECT_INPUT,
    #[doc = "0x4f8 - Select Input Register"]
    pub uart2_rts_b_select_input: UART2_RTS_B_SELECT_INPUT,
    #[doc = "0x4fc - Select Input Register"]
    pub uart2_rxd_select_input: UART2_RXD_SELECT_INPUT,
    #[doc = "0x500 - Select Input Register"]
    pub uart3_rts_b_select_input: UART3_RTS_B_SELECT_INPUT,
    #[doc = "0x504 - Select Input Register"]
    pub uart3_rxd_select_input: UART3_RXD_SELECT_INPUT,
    #[doc = "0x508 - Select Input Register"]
    pub uart4_rts_b_select_input: UART4_RTS_B_SELECT_INPUT,
    #[doc = "0x50c - Select Input Register"]
    pub uart4_rxd_select_input: UART4_RXD_SELECT_INPUT,
    #[doc = "0x510 - Select Input Register"]
    pub sai6_rx_bclk_select_input: SAI6_RX_BCLK_SELECT_INPUT,
    #[doc = "0x514 - Select Input Register"]
    pub sai6_rx_data_select_input_0: SAI6_RX_DATA_SELECT_INPUT_0,
    #[doc = "0x518 - Select Input Register"]
    pub sai6_rx_sync_select_input: SAI6_RX_SYNC_SELECT_INPUT,
    #[doc = "0x51c - Select Input Register"]
    pub sai6_tx_bclk_select_input: SAI6_TX_BCLK_SELECT_INPUT,
    #[doc = "0x520 - Select Input Register"]
    pub sai6_tx_sync_select_input: SAI6_TX_SYNC_SELECT_INPUT,
    #[doc = "0x524 - Select Input Register"]
    pub pcie1_clkreq_b_select_input: PCIE1_CLKREQ_B_SELECT_INPUT,
    _reserved1: [u8; 4usize],
    #[doc = "0x52c - Select Input Register"]
    pub sai5_mclk_select_input: SAI5_MCLK_SELECT_INPUT,
    #[doc = "0x530 - Select Input Register"]
    pub sai6_mclk_select_input: SAI6_MCLK_SELECT_INPUT,
    #[doc = "0x534 - Select Input Register"]
    pub pdm_bit_stream_select_input_0: PDM_BIT_STREAM_SELECT_INPUT_0,
    #[doc = "0x538 - Select Input Register"]
    pub pdm_bit_stream_select_input_1: PDM_BIT_STREAM_SELECT_INPUT_1,
    #[doc = "0x53c - Select Input Register"]
    pub pdm_bit_stream_select_input_2: PDM_BIT_STREAM_SELECT_INPUT_2,
    #[doc = "0x540 - Select Input Register"]
    pub pdm_bit_stream_select_input_3: PDM_BIT_STREAM_SELECT_INPUT_3,
    #[doc = "0x544 - Select Input Register"]
    pub usdhc3_cd_b_select_input: USDHC3_CD_B_SELECT_INPUT,
    #[doc = "0x548 - Select Input Register"]
    pub usdhc3_wp_select_input: USDHC3_WP_SELECT_INPUT,
}
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_PMIC_STBY_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_pmic_stby_req;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_PMIC_ON_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_pmic_on_req;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_ONOFF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_onoff;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_POR_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_por_b;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_RTC_RESET_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_rtc_reset_b;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_GPIO1_IO00 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_gpio1_io00;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_GPIO1_IO01 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_gpio1_io01;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_GPIO1_IO02 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_gpio1_io02;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_GPIO1_IO03 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_gpio1_io03;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_GPIO1_IO04 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_gpio1_io04;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_GPIO1_IO05 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_gpio1_io05;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_GPIO1_IO06 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_gpio1_io06;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_GPIO1_IO07 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_gpio1_io07;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_GPIO1_IO08 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_gpio1_io08;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_GPIO1_IO09 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_gpio1_io09;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_GPIO1_IO10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_gpio1_io10;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_GPIO1_IO11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_gpio1_io11;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_GPIO1_IO12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_gpio1_io12;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_GPIO1_IO13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_gpio1_io13;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_GPIO1_IO14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_gpio1_io14;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_GPIO1_IO15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_gpio1_io15;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_ENET_MDC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_enet_mdc;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_ENET_MDIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_enet_mdio;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_ENET_TD3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_enet_td3;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_ENET_TD2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_enet_td2;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_ENET_TD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_enet_td1;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_ENET_TD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_enet_td0;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_ENET_TX_CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_enet_tx_ctl;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_ENET_TXC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_enet_txc;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_ENET_RX_CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_enet_rx_ctl;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_ENET_RXC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_enet_rxc;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_ENET_RD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_enet_rd0;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_ENET_RD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_enet_rd1;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_ENET_RD2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_enet_rd2;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_ENET_RD3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_enet_rd3;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SD1_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sd1_clk;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SD1_CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sd1_cmd;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SD1_DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sd1_data0;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SD1_DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sd1_data1;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SD1_DATA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sd1_data2;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SD1_DATA3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sd1_data3;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SD1_DATA4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sd1_data4;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SD1_DATA5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sd1_data5;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SD1_DATA6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sd1_data6;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SD1_DATA7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sd1_data7;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SD1_RESET_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sd1_reset_b;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SD1_STROBE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sd1_strobe;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SD2_CD_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sd2_cd_b;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SD2_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sd2_clk;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SD2_CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sd2_cmd;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SD2_DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sd2_data0;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SD2_DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sd2_data1;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SD2_DATA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sd2_data2;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SD2_DATA3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sd2_data3;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SD2_RESET_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sd2_reset_b;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SD2_WP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sd2_wp;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_NAND_ALE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_nand_ale;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_NAND_CE0_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_nand_ce0_b;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_NAND_CE1_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_nand_ce1_b;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_NAND_CE2_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_nand_ce2_b;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_NAND_CE3_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_nand_ce3_b;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_NAND_CLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_nand_cle;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_NAND_DATA00 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_nand_data00;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_NAND_DATA01 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_nand_data01;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_NAND_DATA02 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_nand_data02;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_NAND_DATA03 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_nand_data03;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_NAND_DATA04 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_nand_data04;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_NAND_DATA05 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_nand_data05;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_NAND_DATA06 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_nand_data06;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_NAND_DATA07 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_nand_data07;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_NAND_DQS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_nand_dqs;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_NAND_RE_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_nand_re_b;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_NAND_READY_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_nand_ready_b;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_NAND_WE_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_nand_we_b;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_NAND_WP_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_nand_wp_b;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI5_RXFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai5_rxfs;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI5_RXC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai5_rxc;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI5_RXD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai5_rxd0;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI5_RXD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai5_rxd1;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI5_RXD2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai5_rxd2;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI5_RXD3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai5_rxd3;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI5_MCLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai5_mclk;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI1_RXFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai1_rxfs;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI1_RXC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai1_rxc;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI1_RXD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai1_rxd0;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI1_RXD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai1_rxd1;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI1_RXD2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai1_rxd2;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI1_RXD3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai1_rxd3;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI1_RXD4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai1_rxd4;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI1_RXD5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai1_rxd5;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI1_RXD6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai1_rxd6;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI1_RXD7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai1_rxd7;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI1_TXFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai1_txfs;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI1_TXC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai1_txc;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI1_TXD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai1_txd0;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI1_TXD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai1_txd1;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI1_TXD2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai1_txd2;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI1_TXD3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai1_txd3;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI1_TXD4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai1_txd4;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI1_TXD5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai1_txd5;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI1_TXD6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai1_txd6;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI1_TXD7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai1_txd7;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI1_MCLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai1_mclk;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI2_RXFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai2_rxfs;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI2_RXC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai2_rxc;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI2_RXD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai2_rxd0;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI2_TXFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai2_txfs;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI2_TXC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai2_txc;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI2_TXD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai2_txd0;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI2_MCLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai2_mclk;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI3_RXFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai3_rxfs;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI3_RXC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai3_rxc;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI3_RXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai3_rxd;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI3_TXFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai3_txfs;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI3_TXC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai3_txc;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI3_TXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai3_txd;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SAI3_MCLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_sai3_mclk;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SPDIF_TX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_spdif_tx;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SPDIF_RX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_spdif_rx;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_SPDIF_EXT_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_spdif_ext_clk;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_ECSPI1_SCLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_ecspi1_sclk;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_ECSPI1_MOSI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_ecspi1_mosi;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_ECSPI1_MISO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_ecspi1_miso;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_ECSPI1_SS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_ecspi1_ss0;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_ECSPI2_SCLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_ecspi2_sclk;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_ECSPI2_MOSI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_ecspi2_mosi;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_ECSPI2_MISO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_ecspi2_miso;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_ECSPI2_SS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_ecspi2_ss0;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_I2C1_SCL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_i2c1_scl;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_I2C1_SDA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_i2c1_sda;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_I2C2_SCL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_i2c2_scl;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_I2C2_SDA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_i2c2_sda;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_I2C3_SCL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_i2c3_scl;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_I2C3_SDA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_i2c3_sda;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_I2C4_SCL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_i2c4_scl;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_I2C4_SDA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_i2c4_sda;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_UART1_RXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_uart1_rxd;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_UART1_TXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_uart1_txd;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_UART2_RXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_uart2_rxd;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_UART2_TXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_uart2_txd;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_UART3_RXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_uart3_rxd;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_UART3_TXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_uart3_txd;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_UART4_RXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_uart4_rxd;
#[doc = "Pad Mux Register"]
pub struct SW_MUX_CTL_PAD_UART4_TXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Mux Register"]
pub mod sw_mux_ctl_pad_uart4_txd;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_TEST_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_test_mode;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_BOOT_MODE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_boot_mode0;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_BOOT_MODE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_boot_mode1;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_JTAG_MOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_jtag_mod;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_JTAG_TRST_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_jtag_trst_b;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_JTAG_TDI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_jtag_tdi;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_JTAG_TMS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_jtag_tms;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_JTAG_TCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_jtag_tck;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_JTAG_TDO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_jtag_tdo;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_RTC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_rtc;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_PMIC_STBY_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_pmic_stby_req;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_PMIC_ON_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_pmic_on_req;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_ONOFF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_onoff;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_POR_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_por_b;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_RTC_RESET_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_rtc_reset_b;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_GPIO1_IO00 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_gpio1_io00;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_GPIO1_IO01 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_gpio1_io01;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_GPIO1_IO02 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_gpio1_io02;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_GPIO1_IO03 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_gpio1_io03;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_GPIO1_IO04 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_gpio1_io04;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_GPIO1_IO05 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_gpio1_io05;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_GPIO1_IO06 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_gpio1_io06;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_GPIO1_IO07 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_gpio1_io07;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_GPIO1_IO08 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_gpio1_io08;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_GPIO1_IO09 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_gpio1_io09;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_GPIO1_IO10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_gpio1_io10;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_GPIO1_IO11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_gpio1_io11;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_GPIO1_IO12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_gpio1_io12;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_GPIO1_IO13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_gpio1_io13;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_GPIO1_IO14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_gpio1_io14;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_GPIO1_IO15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_gpio1_io15;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_ENET_MDC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_enet_mdc;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_ENET_MDIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_enet_mdio;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_ENET_TD3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_enet_td3;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_ENET_TD2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_enet_td2;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_ENET_TD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_enet_td1;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_ENET_TD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_enet_td0;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_ENET_TX_CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_enet_tx_ctl;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_ENET_TXC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_enet_txc;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_ENET_RX_CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_enet_rx_ctl;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_ENET_RXC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_enet_rxc;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_ENET_RD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_enet_rd0;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_ENET_RD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_enet_rd1;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_ENET_RD2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_enet_rd2;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_ENET_RD3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_enet_rd3;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SD1_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sd1_clk;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SD1_CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sd1_cmd;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SD1_DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sd1_data0;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SD1_DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sd1_data1;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SD1_DATA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sd1_data2;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SD1_DATA3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sd1_data3;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SD1_DATA4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sd1_data4;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SD1_DATA5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sd1_data5;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SD1_DATA6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sd1_data6;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SD1_DATA7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sd1_data7;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SD1_RESET_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sd1_reset_b;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SD1_STROBE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sd1_strobe;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SD2_CD_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sd2_cd_b;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SD2_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sd2_clk;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SD2_CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sd2_cmd;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SD2_DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sd2_data0;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SD2_DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sd2_data1;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SD2_DATA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sd2_data2;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SD2_DATA3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sd2_data3;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SD2_RESET_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sd2_reset_b;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SD2_WP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sd2_wp;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_NAND_ALE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_nand_ale;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_NAND_CE0_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_nand_ce0_b;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_NAND_CE1_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_nand_ce1_b;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_NAND_CE2_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_nand_ce2_b;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_NAND_CE3_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_nand_ce3_b;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_NAND_CLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_nand_cle;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_NAND_DATA00 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_nand_data00;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_NAND_DATA01 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_nand_data01;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_NAND_DATA02 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_nand_data02;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_NAND_DATA03 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_nand_data03;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_NAND_DATA04 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_nand_data04;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_NAND_DATA05 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_nand_data05;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_NAND_DATA06 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_nand_data06;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_NAND_DATA07 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_nand_data07;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_NAND_DQS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_nand_dqs;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_NAND_RE_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_nand_re_b;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_NAND_READY_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_nand_ready_b;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_NAND_WE_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_nand_we_b;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_NAND_WP_B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_nand_wp_b;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI5_RXFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai5_rxfs;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI5_RXC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai5_rxc;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI5_RXD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai5_rxd0;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI5_RXD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai5_rxd1;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI5_RXD2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai5_rxd2;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI5_RXD3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai5_rxd3;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI5_MCLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai5_mclk;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI1_RXFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai1_rxfs;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI1_RXC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai1_rxc;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI1_RXD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai1_rxd0;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI1_RXD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai1_rxd1;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI1_RXD2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai1_rxd2;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI1_RXD3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai1_rxd3;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI1_RXD4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai1_rxd4;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI1_RXD5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai1_rxd5;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI1_RXD6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai1_rxd6;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI1_RXD7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai1_rxd7;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI1_TXFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai1_txfs;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI1_TXC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai1_txc;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI1_TXD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai1_txd0;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI1_TXD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai1_txd1;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI1_TXD2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai1_txd2;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI1_TXD3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai1_txd3;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI1_TXD4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai1_txd4;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI1_TXD5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai1_txd5;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI1_TXD6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai1_txd6;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI1_TXD7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai1_txd7;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI1_MCLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai1_mclk;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI2_RXFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai2_rxfs;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI2_RXC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai2_rxc;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI2_RXD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai2_rxd0;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI2_TXFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai2_txfs;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI2_TXC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai2_txc;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI2_TXD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai2_txd0;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI2_MCLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai2_mclk;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI3_RXFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai3_rxfs;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI3_RXC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai3_rxc;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI3_RXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai3_rxd;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI3_TXFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai3_txfs;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI3_TXC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai3_txc;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI3_TXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai3_txd;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SAI3_MCLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_sai3_mclk;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SPDIF_TX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_spdif_tx;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SPDIF_RX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_spdif_rx;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_SPDIF_EXT_CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_spdif_ext_clk;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_ECSPI1_SCLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_ecspi1_sclk;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_ECSPI1_MOSI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_ecspi1_mosi;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_ECSPI1_MISO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_ecspi1_miso;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_ECSPI1_SS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_ecspi1_ss0;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_ECSPI2_SCLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_ecspi2_sclk;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_ECSPI2_MOSI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_ecspi2_mosi;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_ECSPI2_MISO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_ecspi2_miso;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_ECSPI2_SS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_ecspi2_ss0;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_I2C1_SCL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_i2c1_scl;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_I2C1_SDA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_i2c1_sda;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_I2C2_SCL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_i2c2_scl;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_I2C2_SDA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_i2c2_sda;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_I2C3_SCL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_i2c3_scl;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_I2C3_SDA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_i2c3_sda;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_I2C4_SCL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_i2c4_scl;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_I2C4_SDA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_i2c4_sda;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_UART1_RXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_uart1_rxd;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_UART1_TXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_uart1_txd;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_UART2_RXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_uart2_rxd;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_UART2_TXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_uart2_txd;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_UART3_RXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_uart3_rxd;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_UART3_TXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_uart3_txd;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_UART4_RXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_uart4_rxd;
#[doc = "Pad Control Register"]
pub struct SW_PAD_CTL_PAD_UART4_TXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pad Control Register"]
pub mod sw_pad_ctl_pad_uart4_txd;
#[doc = "CCM_PMIC_READY_SELECT_INPUT DAISY Register"]
pub struct CCM_PMIC_READY_SELECT_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM_PMIC_READY_SELECT_INPUT DAISY Register"]
pub mod ccm_pmic_ready_select_input;
#[doc = "Select Input Register"]
pub struct ENET1_MDIO_SELECT_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod enet1_mdio_select_input;
#[doc = "Select Input Register"]
pub struct SAI1_RX_SYNC_SELECT_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod sai1_rx_sync_select_input;
#[doc = "Select Input Register"]
pub struct SAI1_TX_BCLK_SELECT_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod sai1_tx_bclk_select_input;
#[doc = "Select Input Register"]
pub struct SAI1_TX_SYNC_SELECT_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod sai1_tx_sync_select_input;
#[doc = "Select Input Register"]
pub struct SAI5_RX_BCLK_SELECT_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod sai5_rx_bclk_select_input;
#[doc = "Select Input Register"]
pub struct SAI5_RX_DATA_SELECT_INPUT_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod sai5_rx_data_select_input_0;
#[doc = "Select Input Register"]
pub struct SAI5_RX_DATA_SELECT_INPUT_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod sai5_rx_data_select_input_1;
#[doc = "Select Input Register"]
pub struct SAI5_RX_DATA_SELECT_INPUT_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod sai5_rx_data_select_input_2;
#[doc = "Select Input Register"]
pub struct SAI5_RX_DATA_SELECT_INPUT_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod sai5_rx_data_select_input_3;
#[doc = "Select Input Register"]
pub struct SAI5_RX_SYNC_SELECT_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod sai5_rx_sync_select_input;
#[doc = "Select Input Register"]
pub struct SAI5_TX_BCLK_SELECT_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod sai5_tx_bclk_select_input;
#[doc = "Select Input Register"]
pub struct SAI5_TX_SYNC_SELECT_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod sai5_tx_sync_select_input;
#[doc = "Select Input Register"]
pub struct UART1_RTS_B_SELECT_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod uart1_rts_b_select_input;
#[doc = "Select Input Register"]
pub struct UART1_RXD_SELECT_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod uart1_rxd_select_input;
#[doc = "Select Input Register"]
pub struct UART2_RTS_B_SELECT_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod uart2_rts_b_select_input;
#[doc = "Select Input Register"]
pub struct UART2_RXD_SELECT_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod uart2_rxd_select_input;
#[doc = "Select Input Register"]
pub struct UART3_RTS_B_SELECT_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod uart3_rts_b_select_input;
#[doc = "Select Input Register"]
pub struct UART3_RXD_SELECT_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod uart3_rxd_select_input;
#[doc = "Select Input Register"]
pub struct UART4_RTS_B_SELECT_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod uart4_rts_b_select_input;
#[doc = "Select Input Register"]
pub struct UART4_RXD_SELECT_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod uart4_rxd_select_input;
#[doc = "Select Input Register"]
pub struct SAI6_RX_BCLK_SELECT_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod sai6_rx_bclk_select_input;
#[doc = "Select Input Register"]
pub struct SAI6_RX_DATA_SELECT_INPUT_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod sai6_rx_data_select_input_0;
#[doc = "Select Input Register"]
pub struct SAI6_RX_SYNC_SELECT_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod sai6_rx_sync_select_input;
#[doc = "Select Input Register"]
pub struct SAI6_TX_BCLK_SELECT_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod sai6_tx_bclk_select_input;
#[doc = "Select Input Register"]
pub struct SAI6_TX_SYNC_SELECT_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod sai6_tx_sync_select_input;
#[doc = "Select Input Register"]
pub struct PCIE1_CLKREQ_B_SELECT_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod pcie1_clkreq_b_select_input;
#[doc = "Select Input Register"]
pub struct SAI5_MCLK_SELECT_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod sai5_mclk_select_input;
#[doc = "Select Input Register"]
pub struct SAI6_MCLK_SELECT_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod sai6_mclk_select_input;
#[doc = "Select Input Register"]
pub struct PDM_BIT_STREAM_SELECT_INPUT_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod pdm_bit_stream_select_input_0;
#[doc = "Select Input Register"]
pub struct PDM_BIT_STREAM_SELECT_INPUT_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod pdm_bit_stream_select_input_1;
#[doc = "Select Input Register"]
pub struct PDM_BIT_STREAM_SELECT_INPUT_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod pdm_bit_stream_select_input_2;
#[doc = "Select Input Register"]
pub struct PDM_BIT_STREAM_SELECT_INPUT_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod pdm_bit_stream_select_input_3;
#[doc = "Select Input Register"]
pub struct USDHC3_CD_B_SELECT_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod usdhc3_cd_b_select_input;
#[doc = "Select Input Register"]
pub struct USDHC3_WP_SELECT_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select Input Register"]
pub mod usdhc3_wp_select_input;
