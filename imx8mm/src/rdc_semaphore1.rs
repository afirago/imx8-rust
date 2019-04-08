#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Gate Register"]
    pub gate: [GATE; 64],
    #[doc = "0x40 - Reset Gate Read"]
    pub rstgt_r: RSTGT_R,
}
#[doc = "Gate Register"]
pub struct GATE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Gate Register"]
pub mod gate;
#[doc = "Reset Gate Read"]
pub struct RSTGT_R {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Reset Gate Read"]
pub mod rstgt_r;
#[doc = "Reset Gate Write"]
pub struct RSTGT_W {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Reset Gate Write"]
pub mod rstgt_w;
