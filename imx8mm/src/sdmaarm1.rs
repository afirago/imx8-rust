#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Arm platform Channel 0 Pointer"]
    pub mc0ptr: MC0PTR,
    #[doc = "0x04 - Channel Interrupts"]
    pub intr: INTR,
    #[doc = "0x08 - Channel Stop/Channel Status"]
    pub stop_stat: STOP_STAT,
    #[doc = "0x0c - Channel Start"]
    pub hstart: HSTART,
    #[doc = "0x10 - Channel Event Override"]
    pub evtovr: EVTOVR,
    #[doc = "0x14 - Channel BP Override"]
    pub dspovr: DSPOVR,
    #[doc = "0x18 - Channel Arm platform Override"]
    pub hostovr: HOSTOVR,
    #[doc = "0x1c - Channel Event Pending"]
    pub evtpend: EVTPEND,
    _reserved0: [u8; 4usize],
    #[doc = "0x24 - Reset Register"]
    pub reset: RESET,
    #[doc = "0x28 - DMA Request Error Register"]
    pub evterr: EVTERR,
    #[doc = "0x2c - Channel Arm platform Interrupt Mask"]
    pub intrmask: INTRMASK,
    #[doc = "0x30 - Schedule Status"]
    pub psw: PSW,
    #[doc = "0x34 - DMA Request Error Register"]
    pub evterrdbg: EVTERRDBG,
    #[doc = "0x38 - Configuration Register"]
    pub config: CONFIG,
    #[doc = "0x3c - SDMA LOCK"]
    pub sdma_lock: SDMA_LOCK,
    #[doc = "0x40 - OnCE Enable"]
    pub once_enb: ONCE_ENB,
    #[doc = "0x44 - OnCE Data Register"]
    pub once_data: ONCE_DATA,
    #[doc = "0x48 - OnCE Instruction Register"]
    pub once_instr: ONCE_INSTR,
    #[doc = "0x4c - OnCE Status Register"]
    pub once_stat: ONCE_STAT,
    #[doc = "0x50 - OnCE Command Register"]
    pub once_cmd: ONCE_CMD,
    _reserved1: [u8; 4usize],
    #[doc = "0x58 - Illegal Instruction Trap Address"]
    pub illinstaddr: ILLINSTADDR,
    #[doc = "0x5c - Channel 0 Boot Address"]
    pub chn0addr: CHN0ADDR,
    #[doc = "0x60 - DMA Requests"]
    pub evt_mirror: EVT_MIRROR,
    #[doc = "0x64 - DMA Requests 2"]
    pub evt_mirror2: EVT_MIRROR2,
    _reserved2: [u8; 8usize],
    #[doc = "0x70 - Cross-Trigger Events Configuration Register 1"]
    pub xtrig_conf1: XTRIG_CONF1,
    #[doc = "0x74 - Cross-Trigger Events Configuration Register 2"]
    pub xtrig_conf2: XTRIG_CONF2,
    _reserved3: [u8; 136usize],
    #[doc = "0x100 - Channel Priority Registers"]
    pub sdma_chnpri: [SDMA_CHNPRI; 32],
    _reserved4: [u8; 128usize],
    #[doc = "0x200 - Channel Enable RAM"]
    pub chnenbl: [CHNENBL; 48],
    _reserved5: [u8; 3392usize],
    #[doc = "0x1000 - Cross-Trigger Events Configuration Register"]
    pub done_conf: DONE_CONF,
}
#[doc = "Arm platform Channel 0 Pointer"]
pub struct MC0PTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Arm platform Channel 0 Pointer"]
pub mod mc0ptr;
#[doc = "Channel Interrupts"]
pub struct INTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Interrupts"]
pub mod intr;
#[doc = "Channel Stop/Channel Status"]
pub struct STOP_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Stop/Channel Status"]
pub mod stop_stat;
#[doc = "Channel Start"]
pub struct HSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Start"]
pub mod hstart;
#[doc = "Channel Event Override"]
pub struct EVTOVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Event Override"]
pub mod evtovr;
#[doc = "Channel BP Override"]
pub struct DSPOVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel BP Override"]
pub mod dspovr;
#[doc = "Channel Arm platform Override"]
pub struct HOSTOVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Arm platform Override"]
pub mod hostovr;
#[doc = "Channel Event Pending"]
pub struct EVTPEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Event Pending"]
pub mod evtpend;
#[doc = "Reset Register"]
pub struct RESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Register"]
pub mod reset;
#[doc = "DMA Request Error Register"]
pub struct EVTERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Request Error Register"]
pub mod evterr;
#[doc = "Channel Arm platform Interrupt Mask"]
pub struct INTRMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Arm platform Interrupt Mask"]
pub mod intrmask;
#[doc = "Schedule Status"]
pub struct PSW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Schedule Status"]
pub mod psw;
#[doc = "DMA Request Error Register"]
pub struct EVTERRDBG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Request Error Register"]
pub mod evterrdbg;
#[doc = "Configuration Register"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Register"]
pub mod config;
#[doc = "SDMA LOCK"]
pub struct SDMA_LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDMA LOCK"]
pub mod sdma_lock;
#[doc = "OnCE Enable"]
pub struct ONCE_ENB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OnCE Enable"]
pub mod once_enb;
#[doc = "OnCE Data Register"]
pub struct ONCE_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OnCE Data Register"]
pub mod once_data;
#[doc = "OnCE Instruction Register"]
pub struct ONCE_INSTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OnCE Instruction Register"]
pub mod once_instr;
#[doc = "OnCE Status Register"]
pub struct ONCE_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OnCE Status Register"]
pub mod once_stat;
#[doc = "OnCE Command Register"]
pub struct ONCE_CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OnCE Command Register"]
pub mod once_cmd;
#[doc = "Illegal Instruction Trap Address"]
pub struct ILLINSTADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Illegal Instruction Trap Address"]
pub mod illinstaddr;
#[doc = "Channel 0 Boot Address"]
pub struct CHN0ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 0 Boot Address"]
pub mod chn0addr;
#[doc = "DMA Requests"]
pub struct EVT_MIRROR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Requests"]
pub mod evt_mirror;
#[doc = "DMA Requests 2"]
pub struct EVT_MIRROR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Requests 2"]
pub mod evt_mirror2;
#[doc = "Cross-Trigger Events Configuration Register 1"]
pub struct XTRIG_CONF1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cross-Trigger Events Configuration Register 1"]
pub mod xtrig_conf1;
#[doc = "Cross-Trigger Events Configuration Register 2"]
pub struct XTRIG_CONF2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cross-Trigger Events Configuration Register 2"]
pub mod xtrig_conf2;
#[doc = "Channel Priority Registers"]
pub struct SDMA_CHNPRI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Priority Registers"]
pub mod sdma_chnpri;
#[doc = "Channel Enable RAM"]
pub struct CHNENBL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Enable RAM"]
pub mod chnenbl;
#[doc = "Cross-Trigger Events Configuration Register"]
pub struct DONE_CONF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cross-Trigger Events Configuration Register"]
pub mod done_conf;
