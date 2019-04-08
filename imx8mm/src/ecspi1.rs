#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Receive Data Register"]
    pub rxdata: RXDATA,
    #[doc = "0x04 - Transmit Data Register"]
    pub txdata: TXDATA,
    #[doc = "0x08 - Control Register"]
    pub conreg: CONREG,
    #[doc = "0x0c - Config Register"]
    pub configreg: CONFIGREG,
    #[doc = "0x10 - Interrupt Control Register"]
    pub intreg: INTREG,
    #[doc = "0x14 - DMA Control Register"]
    pub dmareg: DMAREG,
    #[doc = "0x18 - Status Register"]
    pub statreg: STATREG,
    #[doc = "0x1c - Sample Period Control Register"]
    pub periodreg: PERIODREG,
    #[doc = "0x20 - Test Control Register"]
    pub testreg: TESTREG,
    _reserved0: [u8; 28usize],
    #[doc = "0x40 - Message Data Register"]
    pub msgdata: MSGDATA,
}
#[doc = "Receive Data Register"]
pub struct RXDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Data Register"]
pub mod rxdata;
#[doc = "Transmit Data Register"]
pub struct TXDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Data Register"]
pub mod txdata;
#[doc = "Control Register"]
pub struct CONREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod conreg;
#[doc = "Config Register"]
pub struct CONFIGREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Config Register"]
pub mod configreg;
#[doc = "Interrupt Control Register"]
pub struct INTREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Control Register"]
pub mod intreg;
#[doc = "DMA Control Register"]
pub struct DMAREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Control Register"]
pub mod dmareg;
#[doc = "Status Register"]
pub struct STATREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod statreg;
#[doc = "Sample Period Control Register"]
pub struct PERIODREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sample Period Control Register"]
pub mod periodreg;
#[doc = "Test Control Register"]
pub struct TESTREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Test Control Register"]
pub mod testreg;
#[doc = "Message Data Register"]
pub struct MSGDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Data Register"]
pub mod msgdata;
