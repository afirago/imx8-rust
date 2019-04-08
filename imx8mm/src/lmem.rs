#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cache control register"]
    pub pcccr: PCCCR,
    #[doc = "0x04 - Cache line control register"]
    pub pcclcr: PCCLCR,
    #[doc = "0x08 - Cache search address register"]
    pub pccsar: PCCSAR,
    #[doc = "0x0c - Cache read/write value register"]
    pub pcccvr: PCCCVR,
    _reserved0: [u8; 2032usize],
    #[doc = "0x800 - Cache control register"]
    pub psccr: PSCCR,
    #[doc = "0x804 - Cache line control register"]
    pub psclcr: PSCLCR,
    #[doc = "0x808 - Cache search address register"]
    pub pscsar: PSCSAR,
    #[doc = "0x80c - Cache read/write value register"]
    pub psccvr: PSCCVR,
}
#[doc = "Cache control register"]
pub struct PCCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache control register"]
pub mod pcccr;
#[doc = "Cache line control register"]
pub struct PCCLCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache line control register"]
pub mod pcclcr;
#[doc = "Cache search address register"]
pub struct PCCSAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache search address register"]
pub mod pccsar;
#[doc = "Cache read/write value register"]
pub struct PCCCVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache read/write value register"]
pub mod pcccvr;
#[doc = "Cache control register"]
pub struct PSCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache control register"]
pub mod psccr;
#[doc = "Cache line control register"]
pub struct PSCLCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache line control register"]
pub mod psclcr;
#[doc = "Cache search address register"]
pub struct PSCSAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache search address register"]
pub mod pscsar;
#[doc = "Cache read/write value register"]
pub struct PSCCVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache read/write value register"]
pub mod psccvr;
