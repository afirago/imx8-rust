#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CTRL_1"]
    pub ctrl_1: CTRL_1,
    #[doc = "0x04 - CTRL_2"]
    pub ctrl_2: CTRL_2,
    #[doc = "0x08 - STAT"]
    pub stat: STAT,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - FIFO_CTRL"]
    pub fifo_ctrl: FIFO_CTRL,
    #[doc = "0x14 - FIFO_STAT"]
    pub fifo_stat: FIFO_STAT,
    _reserved1: [u8; 12usize],
    #[doc = "0x24 - DATACH0"]
    pub datach0: DATACH0,
    #[doc = "0x28 - DATACH1"]
    pub datach1: DATACH1,
    #[doc = "0x2c - DATACH2"]
    pub datach2: DATACH2,
    #[doc = "0x30 - DATACH3"]
    pub datach3: DATACH3,
    #[doc = "0x34 - DATACH4"]
    pub datach4: DATACH4,
    #[doc = "0x38 - DATACH5"]
    pub datach5: DATACH5,
    #[doc = "0x3c - DATACH6"]
    pub datach6: DATACH6,
    #[doc = "0x40 - DATACH7"]
    pub datach7: DATACH7,
    _reserved2: [u8; 32usize],
    #[doc = "0x64 - DC_CTRL"]
    pub dc_ctrl: DC_CTRL,
    _reserved3: [u8; 12usize],
    #[doc = "0x74 - OUT_CTRL"]
    pub out_ctrl: OUT_CTRL,
    _reserved4: [u8; 4usize],
    #[doc = "0x7c - OUT_STAT"]
    pub out_stat: OUT_STAT,
    _reserved5: [u8; 16usize],
    #[doc = "0x90 - VAD0_CTRL_1"]
    pub vad0_ctrl_1: VAD0_CTRL_1,
    #[doc = "0x94 - VAD0_CTRL_2"]
    pub vad0_ctrl_2: VAD0_CTRL_2,
    #[doc = "0x98 - VAD0_STAT"]
    pub vad0_stat: VAD0_STAT,
    #[doc = "0x9c - VAD0_SCONFIG"]
    pub vad0_sconfig: VAD0_SCONFIG,
    #[doc = "0xa0 - VAD0_NCONFIG"]
    pub vad0_nconfig: VAD0_NCONFIG,
    #[doc = "0xa4 - VAD0_NDATA"]
    pub vad0_ndata: VAD0_NDATA,
    #[doc = "0xa8 - VAD0_ZCD"]
    pub vad0_zcd: VAD0_ZCD,
}
#[doc = "CTRL_1"]
pub struct CTRL_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CTRL_1"]
pub mod ctrl_1;
#[doc = "CTRL_2"]
pub struct CTRL_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CTRL_2"]
pub mod ctrl_2;
#[doc = "STAT"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "STAT"]
pub mod stat;
#[doc = "FIFO_CTRL"]
pub struct FIFO_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO_CTRL"]
pub mod fifo_ctrl;
#[doc = "FIFO_STAT"]
pub struct FIFO_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO_STAT"]
pub mod fifo_stat;
#[doc = "DATACH0"]
pub struct DATACH0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DATACH0"]
pub mod datach0;
#[doc = "DATACH1"]
pub struct DATACH1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DATACH1"]
pub mod datach1;
#[doc = "DATACH2"]
pub struct DATACH2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DATACH2"]
pub mod datach2;
#[doc = "DATACH3"]
pub struct DATACH3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DATACH3"]
pub mod datach3;
#[doc = "DATACH4"]
pub struct DATACH4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DATACH4"]
pub mod datach4;
#[doc = "DATACH5"]
pub struct DATACH5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DATACH5"]
pub mod datach5;
#[doc = "DATACH6"]
pub struct DATACH6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DATACH6"]
pub mod datach6;
#[doc = "DATACH7"]
pub struct DATACH7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DATACH7"]
pub mod datach7;
#[doc = "DC_CTRL"]
pub struct DC_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DC_CTRL"]
pub mod dc_ctrl;
#[doc = "OUT_CTRL"]
pub struct OUT_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OUT_CTRL"]
pub mod out_ctrl;
#[doc = "OUT_STAT"]
pub struct OUT_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OUT_STAT"]
pub mod out_stat;
#[doc = "VAD0_CTRL_1"]
pub struct VAD0_CTRL_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VAD0_CTRL_1"]
pub mod vad0_ctrl_1;
#[doc = "VAD0_CTRL_2"]
pub struct VAD0_CTRL_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VAD0_CTRL_2"]
pub mod vad0_ctrl_2;
#[doc = "VAD0_STAT"]
pub struct VAD0_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VAD0_STAT"]
pub mod vad0_stat;
#[doc = "VAD0_SCONFIG"]
pub struct VAD0_SCONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VAD0_SCONFIG"]
pub mod vad0_sconfig;
#[doc = "VAD0_NCONFIG"]
pub struct VAD0_NCONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VAD0_NCONFIG"]
pub mod vad0_nconfig;
#[doc = "VAD0_NDATA"]
pub struct VAD0_NDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VAD0_NDATA"]
pub mod vad0_ndata;
#[doc = "VAD0_ZCD"]
pub struct VAD0_ZCD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VAD0_ZCD"]
pub mod vad0_zcd;
