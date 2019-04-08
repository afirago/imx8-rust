#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - Crossbar Switch (AXBS) Slave Configuration"]
    pub plasc: PLASC,
    #[doc = "0x0a - Crossbar Switch (AXBS) Master Configuration"]
    pub plamc: PLAMC,
    #[doc = "0x0c - Crossbar Switch (AXBS) Control Register"]
    pub placr: PLACR,
    _reserved1: [u8; 16usize],
    #[doc = "0x20 - Fault address register"]
    pub fadr: FADR,
    #[doc = "0x24 - Fault attributes register"]
    pub fatr: FATR,
    #[doc = "0x28 - Fault data register"]
    pub fdr: FDR,
}
#[doc = "Crossbar Switch (AXBS) Slave Configuration"]
pub struct PLASC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Crossbar Switch (AXBS) Slave Configuration"]
pub mod plasc;
#[doc = "Crossbar Switch (AXBS) Master Configuration"]
pub struct PLAMC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Crossbar Switch (AXBS) Master Configuration"]
pub mod plamc;
#[doc = "Crossbar Switch (AXBS) Control Register"]
pub struct PLACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Crossbar Switch (AXBS) Control Register"]
pub mod placr;
#[doc = "Fault address register"]
pub struct FADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fault address register"]
pub mod fadr;
#[doc = "Fault attributes register"]
pub struct FATR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fault attributes register"]
pub mod fatr;
#[doc = "Fault data register"]
pub struct FDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fault data register"]
pub mod fdr;
