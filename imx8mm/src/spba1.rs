#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Peripheral Rights Register"]
    pub prr: [PRR; 32],
}
#[doc = "Peripheral Rights Register"]
pub struct PRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Rights Register"]
pub mod prr;
