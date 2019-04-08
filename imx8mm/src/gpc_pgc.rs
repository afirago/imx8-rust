#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPC PGC Control Register"]
    pub a53core0_ctrl: A53CORE0_CTRL,
    #[doc = "0x04 - GPC PGC Up Sequence Control Register"]
    pub a53core0_pupscr: A53CORE0_PUPSCR,
    #[doc = "0x08 - GPC PGC Down Sequence Control Register"]
    pub a53core0_pdnscr: A53CORE0_PDNSCR,
    #[doc = "0x0c - GPC PGC Status Register"]
    pub a53core0_sr: A53CORE0_SR,
    _reserved0: [u8; 48usize],
    #[doc = "0x40 - GPC PGC Control Register"]
    pub a53core1_ctrl: A53CORE1_CTRL,
    #[doc = "0x44 - GPC PGC Up Sequence Control Register"]
    pub a53core1_pupscr: A53CORE1_PUPSCR,
    #[doc = "0x48 - GPC PGC Down Sequence Control Register"]
    pub a53core1_pdnscr: A53CORE1_PDNSCR,
    #[doc = "0x4c - GPC PGC Status Register"]
    pub a53core1_sr: A53CORE1_SR,
    _reserved1: [u8; 48usize],
    #[doc = "0x80 - GPC PGC Control Register"]
    pub a53core2_ctrl: A53CORE2_CTRL,
    #[doc = "0x84 - GPC PGC Up Sequence Control Register"]
    pub a53core2_pupscr: A53CORE2_PUPSCR,
    #[doc = "0x88 - GPC PGC Down Sequence Control Register"]
    pub a53core2_pdnscr: A53CORE2_PDNSCR,
    #[doc = "0x8c - GPC PGC Status Register"]
    pub a53core2_sr: A53CORE2_SR,
    _reserved2: [u8; 48usize],
    #[doc = "0xc0 - GPC PGC Control Register"]
    pub a53core3_ctrl: A53CORE3_CTRL,
    #[doc = "0xc4 - GPC PGC Up Sequence Control Register"]
    pub a53core3_pupscr: A53CORE3_PUPSCR,
    #[doc = "0xc8 - GPC PGC Down Sequence Control Register"]
    pub a53core3_pdnscr: A53CORE3_PDNSCR,
    #[doc = "0xcc - GPC PGC Status Register"]
    pub a53core3_sr: A53CORE3_SR,
    _reserved3: [u8; 48usize],
    #[doc = "0x100 - GPC PGC Control Register"]
    pub a53scu_ctrl: A53SCU_CTRL,
    #[doc = "0x104 - GPC PGC Up Sequence Control Register"]
    pub a53scu_pupscr: A53SCU_PUPSCR,
    #[doc = "0x108 - GPC PGC Down Sequence Control Register"]
    pub a53scu_pdnscr: A53SCU_PDNSCR,
    #[doc = "0x10c - GPC PGC Status Register"]
    pub a53scu_sr: A53SCU_SR,
    #[doc = "0x110 - GPC PGC Auxiliary Power Switch Control Register"]
    pub a53scu_auxsw: A53SCU_AUXSW,
    _reserved4: [u8; 236usize],
    #[doc = "0x200 - GPC PGC Control Register"]
    pub mf_mix_ctrl: MIX_CTRL,
    #[doc = "0x204 - GPC PGC Up Sequence Control Register"]
    pub mf_mix_pupscr: MIX_PUPSCR,
    #[doc = "0x208 - GPC PGC Down Sequence Control Register"]
    pub mf_mix_pdnscr: MIX_PDNSCR,
    #[doc = "0x20c - GPC PGC Status Register"]
    pub mf_mix_sr: MIX_SR,
    _reserved5: [u8; 48usize],
    #[doc = "0x240 - GPC PGC Control Register"]
    pub noc_mix_ctrl: MIX_CTRL,
    #[doc = "0x244 - GPC PGC Up Sequence Control Register"]
    pub noc_mix_pupscr: MIX_PUPSCR,
    #[doc = "0x248 - GPC PGC Down Sequence Control Register"]
    pub noc_mix_pdnscr: MIX_PDNSCR,
    #[doc = "0x24c - GPC PGC Status Register"]
    pub noc_mix_sr: MIX_SR,
    _reserved6: [u8; 432usize],
    #[doc = "0x400 - GPC PGC Control Register"]
    pub pu0_ctrl: PU0_CTRL,
    #[doc = "0x404 - GPC PGC Up Sequence Control Register"]
    pub pu0_pupscr: PU0_PUPSCR,
    #[doc = "0x408 - GPC PGC Down Sequence Control Register"]
    pub pu0_pdnscr: PU0_PDNSCR,
    #[doc = "0x40c - GPC PGC Status Register"]
    pub pu0_sr: PU0_SR,
    _reserved7: [u8; 48usize],
    #[doc = "0x440 - GPC PGC Control Register"]
    pub pu1_ctrl: PU1_CTRL,
    #[doc = "0x444 - GPC PGC Up Sequence Control Register"]
    pub pu1_pupscr: PU1_PUPSCR,
    #[doc = "0x448 - GPC PGC Down Sequence Control Register"]
    pub pu1_pdnscr: PU1_PDNSCR,
    #[doc = "0x44c - GPC PGC Status Register"]
    pub pu1_sr: PU1_SR,
    _reserved8: [u8; 48usize],
    #[doc = "0x480 - GPC PGC Control Register"]
    pub pu2_ctrl: PU2_CTRL,
    #[doc = "0x484 - GPC PGC Up Sequence Control Register"]
    pub pu2_pupscr: PU2_PUPSCR,
    #[doc = "0x488 - GPC PGC Down Sequence Control Register"]
    pub pu2_pdnscr: PU2_PDNSCR,
    #[doc = "0x48c - GPC PGC Status Register"]
    pub pu2_sr: PU2_SR,
    _reserved9: [u8; 48usize],
    #[doc = "0x4c0 - GPC PGC Control Register"]
    pub pu3_ctrl: PU3_CTRL,
    #[doc = "0x4c4 - GPC PGC Up Sequence Control Register"]
    pub pu3_pupscr: PU3_PUPSCR,
    #[doc = "0x4c8 - GPC PGC Down Sequence Control Register"]
    pub pu3_pdnscr: PU3_PDNSCR,
    #[doc = "0x4cc - GPC PGC Status Register"]
    pub pu3_sr: PU3_SR,
    _reserved10: [u8; 48usize],
    #[doc = "0x500 - GPC PGC Control Register"]
    pub pu4_ctrl: PU4_CTRL,
    #[doc = "0x504 - GPC PGC Up Sequence Control Register"]
    pub pu4_pupscr: PU4_PUPSCR,
    #[doc = "0x508 - GPC PGC Down Sequence Control Register"]
    pub pu4_pdnscr: PU4_PDNSCR,
    #[doc = "0x50c - GPC PGC Status Register"]
    pub pu4_sr: PU4_SR,
    _reserved11: [u8; 48usize],
    #[doc = "0x540 - GPC PGC Control Register"]
    pub pu5_ctrl: PU5_CTRL,
    #[doc = "0x544 - GPC PGC Up Sequence Control Register"]
    pub pu5_pupscr: PU5_PUPSCR,
    #[doc = "0x548 - GPC PGC Down Sequence Control Register"]
    pub pu5_pdnscr: PU5_PDNSCR,
    #[doc = "0x54c - GPC PGC Status Register"]
    pub pu5_sr: PU5_SR,
    _reserved12: [u8; 48usize],
    #[doc = "0x580 - GPC PGC Control Register"]
    pub pu6_ctrl: PU6_CTRL,
    #[doc = "0x584 - GPC PGC Up Sequence Control Register"]
    pub pu6_pupscr: PU6_PUPSCR,
    #[doc = "0x588 - GPC PGC Down Sequence Control Register"]
    pub pu6_pdnscr: PU6_PDNSCR,
    #[doc = "0x58c - GPC PGC Status Register"]
    pub pu6_sr: PU6_SR,
    _reserved13: [u8; 48usize],
    #[doc = "0x5c0 - GPC PGC Control Register"]
    pub pu7_ctrl: PU7_CTRL,
    #[doc = "0x5c4 - GPC PGC Up Sequence Control Register"]
    pub pu7_pupscr: PU7_PUPSCR,
    #[doc = "0x5c8 - GPC PGC Down Sequence Control Register"]
    pub pu7_pdnscr: PU7_PDNSCR,
    #[doc = "0x5cc - GPC PGC Status Register"]
    pub pu7_sr: PU7_SR,
    _reserved14: [u8; 48usize],
    #[doc = "0x600 - GPC PGC Control Register"]
    pub pu8_ctrl: PU8_CTRL,
    #[doc = "0x604 - GPC PGC Up Sequence Control Register"]
    pub pu8_pupscr: PU8_PUPSCR,
    #[doc = "0x608 - GPC PGC Down Sequence Control Register"]
    pub pu8_pdnscr: PU8_PDNSCR,
    #[doc = "0x60c - GPC PGC Status Register"]
    pub pu8_sr: PU8_SR,
    _reserved15: [u8; 48usize],
    #[doc = "0x640 - GPC PGC Control Register"]
    pub pu9_ctrl: PU9_CTRL,
    #[doc = "0x644 - GPC PGC Up Sequence Control Register"]
    pub pu9_pupscr: PU9_PUPSCR,
    #[doc = "0x648 - GPC PGC Down Sequence Control Register"]
    pub pu9_pdnscr: PU9_PDNSCR,
    #[doc = "0x64c - GPC PGC Status Register"]
    pub pu9_sr: PU9_SR,
    _reserved16: [u8; 48usize],
    #[doc = "0x680 - GPC PGC Control Register"]
    pub pu10_ctrl: PU10_CTRL,
    #[doc = "0x684 - GPC PGC Up Sequence Control Register"]
    pub pu10_pupscr: PU10_PUPSCR,
    #[doc = "0x688 - GPC PGC Down Sequence Control Register"]
    pub pu10_pdnscr: PU10_PDNSCR,
    #[doc = "0x68c - GPC PGC Status Register"]
    pub pu10_sr: PU10_SR,
    _reserved17: [u8; 48usize],
    #[doc = "0x6c0 - GPC PGC Control Register"]
    pub pu11_ctrl: PU11_CTRL,
    #[doc = "0x6c4 - GPC PGC Up Sequence Control Register"]
    pub pu11_pupscr: PU11_PUPSCR,
    #[doc = "0x6c8 - GPC PGC Down Sequence Control Register"]
    pub pu11_pdnscr: PU11_PDNSCR,
    #[doc = "0x6cc - GPC PGC Status Register"]
    pub pu11_sr: PU11_SR,
    _reserved18: [u8; 48usize],
    #[doc = "0x700 - GPC PGC Control Register"]
    pub pu12_ctrl: PU12_CTRL,
    #[doc = "0x704 - GPC PGC Up Sequence Control Register"]
    pub pu12_pupscr: PU12_PUPSCR,
    #[doc = "0x708 - GPC PGC Down Sequence Control Register"]
    pub pu12_pdnscr: PU12_PDNSCR,
    #[doc = "0x70c - GPC PGC Status Register"]
    pub pu12_sr: PU12_SR,
    _reserved19: [u8; 48usize],
    #[doc = "0x740 - GPC PGC Control Register"]
    pub pu13_ctrl: PU13_CTRL,
    #[doc = "0x744 - GPC PGC Up Sequence Control Register"]
    pub pu13_pupscr: PU13_PUPSCR,
    #[doc = "0x748 - GPC PGC Down Sequence Control Register"]
    pub pu13_pdnscr: PU13_PDNSCR,
    #[doc = "0x74c - GPC PGC Status Register"]
    pub pu13_sr: PU13_SR,
}
#[doc = "GPC PGC Control Register"]
pub struct A53CORE0_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Control Register"]
pub mod a53core0_ctrl;
#[doc = "GPC PGC Up Sequence Control Register"]
pub struct A53CORE0_PUPSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Up Sequence Control Register"]
pub mod a53core0_pupscr;
#[doc = "GPC PGC Down Sequence Control Register"]
pub struct A53CORE0_PDNSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Down Sequence Control Register"]
pub mod a53core0_pdnscr;
#[doc = "GPC PGC Status Register"]
pub struct A53CORE0_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Status Register"]
pub mod a53core0_sr;
#[doc = "GPC PGC Control Register"]
pub struct A53CORE1_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Control Register"]
pub mod a53core1_ctrl;
#[doc = "GPC PGC Up Sequence Control Register"]
pub struct A53CORE1_PUPSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Up Sequence Control Register"]
pub mod a53core1_pupscr;
#[doc = "GPC PGC Down Sequence Control Register"]
pub struct A53CORE1_PDNSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Down Sequence Control Register"]
pub mod a53core1_pdnscr;
#[doc = "GPC PGC Status Register"]
pub struct A53CORE1_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Status Register"]
pub mod a53core1_sr;
#[doc = "GPC PGC Control Register"]
pub struct A53CORE2_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Control Register"]
pub mod a53core2_ctrl;
#[doc = "GPC PGC Up Sequence Control Register"]
pub struct A53CORE2_PUPSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Up Sequence Control Register"]
pub mod a53core2_pupscr;
#[doc = "GPC PGC Down Sequence Control Register"]
pub struct A53CORE2_PDNSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Down Sequence Control Register"]
pub mod a53core2_pdnscr;
#[doc = "GPC PGC Status Register"]
pub struct A53CORE2_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Status Register"]
pub mod a53core2_sr;
#[doc = "GPC PGC Control Register"]
pub struct A53CORE3_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Control Register"]
pub mod a53core3_ctrl;
#[doc = "GPC PGC Up Sequence Control Register"]
pub struct A53CORE3_PUPSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Up Sequence Control Register"]
pub mod a53core3_pupscr;
#[doc = "GPC PGC Down Sequence Control Register"]
pub struct A53CORE3_PDNSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Down Sequence Control Register"]
pub mod a53core3_pdnscr;
#[doc = "GPC PGC Status Register"]
pub struct A53CORE3_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Status Register"]
pub mod a53core3_sr;
#[doc = "GPC PGC Control Register"]
pub struct A53SCU_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Control Register"]
pub mod a53scu_ctrl;
#[doc = "GPC PGC Up Sequence Control Register"]
pub struct A53SCU_PUPSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Up Sequence Control Register"]
pub mod a53scu_pupscr;
#[doc = "GPC PGC Down Sequence Control Register"]
pub struct A53SCU_PDNSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Down Sequence Control Register"]
pub mod a53scu_pdnscr;
#[doc = "GPC PGC Status Register"]
pub struct A53SCU_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Status Register"]
pub mod a53scu_sr;
#[doc = "GPC PGC Auxiliary Power Switch Control Register"]
pub struct A53SCU_AUXSW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Auxiliary Power Switch Control Register"]
pub mod a53scu_auxsw;
#[doc = "GPC PGC Control Register"]
pub struct MIX_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Control Register"]
pub mod mix_ctrl;
#[doc = "GPC PGC Up Sequence Control Register"]
pub struct MIX_PUPSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Up Sequence Control Register"]
pub mod mix_pupscr;
#[doc = "GPC PGC Down Sequence Control Register"]
pub struct MIX_PDNSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Down Sequence Control Register"]
pub mod mix_pdnscr;
#[doc = "GPC PGC Status Register"]
pub struct MIX_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Status Register"]
pub mod mix_sr;
#[doc = "GPC PGC Control Register"]
pub struct PU0_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Control Register"]
pub mod pu0_ctrl;
#[doc = "GPC PGC Up Sequence Control Register"]
pub struct PU0_PUPSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Up Sequence Control Register"]
pub mod pu0_pupscr;
#[doc = "GPC PGC Down Sequence Control Register"]
pub struct PU0_PDNSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Down Sequence Control Register"]
pub mod pu0_pdnscr;
#[doc = "GPC PGC Status Register"]
pub struct PU0_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Status Register"]
pub mod pu0_sr;
#[doc = "GPC PGC Control Register"]
pub struct PU1_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Control Register"]
pub mod pu1_ctrl;
#[doc = "GPC PGC Up Sequence Control Register"]
pub struct PU1_PUPSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Up Sequence Control Register"]
pub mod pu1_pupscr;
#[doc = "GPC PGC Down Sequence Control Register"]
pub struct PU1_PDNSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Down Sequence Control Register"]
pub mod pu1_pdnscr;
#[doc = "GPC PGC Status Register"]
pub struct PU1_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Status Register"]
pub mod pu1_sr;
#[doc = "GPC PGC Control Register"]
pub struct PU2_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Control Register"]
pub mod pu2_ctrl;
#[doc = "GPC PGC Up Sequence Control Register"]
pub struct PU2_PUPSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Up Sequence Control Register"]
pub mod pu2_pupscr;
#[doc = "GPC PGC Down Sequence Control Register"]
pub struct PU2_PDNSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Down Sequence Control Register"]
pub mod pu2_pdnscr;
#[doc = "GPC PGC Status Register"]
pub struct PU2_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Status Register"]
pub mod pu2_sr;
#[doc = "GPC PGC Control Register"]
pub struct PU3_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Control Register"]
pub mod pu3_ctrl;
#[doc = "GPC PGC Up Sequence Control Register"]
pub struct PU3_PUPSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Up Sequence Control Register"]
pub mod pu3_pupscr;
#[doc = "GPC PGC Down Sequence Control Register"]
pub struct PU3_PDNSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Down Sequence Control Register"]
pub mod pu3_pdnscr;
#[doc = "GPC PGC Status Register"]
pub struct PU3_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Status Register"]
pub mod pu3_sr;
#[doc = "GPC PGC Control Register"]
pub struct PU4_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Control Register"]
pub mod pu4_ctrl;
#[doc = "GPC PGC Up Sequence Control Register"]
pub struct PU4_PUPSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Up Sequence Control Register"]
pub mod pu4_pupscr;
#[doc = "GPC PGC Down Sequence Control Register"]
pub struct PU4_PDNSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Down Sequence Control Register"]
pub mod pu4_pdnscr;
#[doc = "GPC PGC Status Register"]
pub struct PU4_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Status Register"]
pub mod pu4_sr;
#[doc = "GPC PGC Control Register"]
pub struct PU5_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Control Register"]
pub mod pu5_ctrl;
#[doc = "GPC PGC Up Sequence Control Register"]
pub struct PU5_PUPSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Up Sequence Control Register"]
pub mod pu5_pupscr;
#[doc = "GPC PGC Down Sequence Control Register"]
pub struct PU5_PDNSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Down Sequence Control Register"]
pub mod pu5_pdnscr;
#[doc = "GPC PGC Status Register"]
pub struct PU5_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Status Register"]
pub mod pu5_sr;
#[doc = "GPC PGC Control Register"]
pub struct PU6_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Control Register"]
pub mod pu6_ctrl;
#[doc = "GPC PGC Up Sequence Control Register"]
pub struct PU6_PUPSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Up Sequence Control Register"]
pub mod pu6_pupscr;
#[doc = "GPC PGC Down Sequence Control Register"]
pub struct PU6_PDNSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Down Sequence Control Register"]
pub mod pu6_pdnscr;
#[doc = "GPC PGC Status Register"]
pub struct PU6_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Status Register"]
pub mod pu6_sr;
#[doc = "GPC PGC Control Register"]
pub struct PU7_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Control Register"]
pub mod pu7_ctrl;
#[doc = "GPC PGC Up Sequence Control Register"]
pub struct PU7_PUPSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Up Sequence Control Register"]
pub mod pu7_pupscr;
#[doc = "GPC PGC Down Sequence Control Register"]
pub struct PU7_PDNSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Down Sequence Control Register"]
pub mod pu7_pdnscr;
#[doc = "GPC PGC Status Register"]
pub struct PU7_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Status Register"]
pub mod pu7_sr;
#[doc = "GPC PGC Control Register"]
pub struct PU8_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Control Register"]
pub mod pu8_ctrl;
#[doc = "GPC PGC Up Sequence Control Register"]
pub struct PU8_PUPSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Up Sequence Control Register"]
pub mod pu8_pupscr;
#[doc = "GPC PGC Down Sequence Control Register"]
pub struct PU8_PDNSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Down Sequence Control Register"]
pub mod pu8_pdnscr;
#[doc = "GPC PGC Status Register"]
pub struct PU8_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Status Register"]
pub mod pu8_sr;
#[doc = "GPC PGC Control Register"]
pub struct PU9_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Control Register"]
pub mod pu9_ctrl;
#[doc = "GPC PGC Up Sequence Control Register"]
pub struct PU9_PUPSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Up Sequence Control Register"]
pub mod pu9_pupscr;
#[doc = "GPC PGC Down Sequence Control Register"]
pub struct PU9_PDNSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Down Sequence Control Register"]
pub mod pu9_pdnscr;
#[doc = "GPC PGC Status Register"]
pub struct PU9_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Status Register"]
pub mod pu9_sr;
#[doc = "GPC PGC Control Register"]
pub struct PU10_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Control Register"]
pub mod pu10_ctrl;
#[doc = "GPC PGC Up Sequence Control Register"]
pub struct PU10_PUPSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Up Sequence Control Register"]
pub mod pu10_pupscr;
#[doc = "GPC PGC Down Sequence Control Register"]
pub struct PU10_PDNSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Down Sequence Control Register"]
pub mod pu10_pdnscr;
#[doc = "GPC PGC Status Register"]
pub struct PU10_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Status Register"]
pub mod pu10_sr;
#[doc = "GPC PGC Control Register"]
pub struct PU11_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Control Register"]
pub mod pu11_ctrl;
#[doc = "GPC PGC Up Sequence Control Register"]
pub struct PU11_PUPSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Up Sequence Control Register"]
pub mod pu11_pupscr;
#[doc = "GPC PGC Down Sequence Control Register"]
pub struct PU11_PDNSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Down Sequence Control Register"]
pub mod pu11_pdnscr;
#[doc = "GPC PGC Status Register"]
pub struct PU11_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Status Register"]
pub mod pu11_sr;
#[doc = "GPC PGC Control Register"]
pub struct PU12_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Control Register"]
pub mod pu12_ctrl;
#[doc = "GPC PGC Up Sequence Control Register"]
pub struct PU12_PUPSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Up Sequence Control Register"]
pub mod pu12_pupscr;
#[doc = "GPC PGC Down Sequence Control Register"]
pub struct PU12_PDNSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Down Sequence Control Register"]
pub mod pu12_pdnscr;
#[doc = "GPC PGC Status Register"]
pub struct PU12_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Status Register"]
pub mod pu12_sr;
#[doc = "GPC PGC Control Register"]
pub struct PU13_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Control Register"]
pub mod pu13_ctrl;
#[doc = "GPC PGC Up Sequence Control Register"]
pub struct PU13_PUPSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Up Sequence Control Register"]
pub mod pu13_pupscr;
#[doc = "GPC PGC Down Sequence Control Register"]
pub struct PU13_PDNSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Down Sequence Control Register"]
pub mod pu13_pdnscr;
#[doc = "GPC PGC Status Register"]
pub struct PU13_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPC PGC Status Register"]
pub mod pu13_sr;
