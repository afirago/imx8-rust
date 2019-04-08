#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Processor B Transmit Register 0"]
    pub btr0: BTR0,
    #[doc = "0x04 - Processor B Transmit Register 1"]
    pub btr1: BTR1,
    #[doc = "0x08 - Processor B Transmit Register 2"]
    pub btr2: BTR2,
    #[doc = "0x0c - Processor B Transmit Register 3"]
    pub btr3: BTR3,
    #[doc = "0x10 - Processor B Receive Register 0"]
    pub brr0: BRR0,
    #[doc = "0x14 - Processor B Receive Register 1"]
    pub brr1: BRR1,
    #[doc = "0x18 - Processor B Receive Register 2"]
    pub brr2: BRR2,
    #[doc = "0x1c - Processor B Receive Register 3"]
    pub brr3: BRR3,
    #[doc = "0x20 - Processor B Status Register"]
    pub bsr: BSR,
    #[doc = "0x24 - Processor B Control Register"]
    pub bcr: BCR,
}
#[doc = "Processor B Transmit Register 0"]
pub struct BTR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Processor B Transmit Register 0"]
pub mod btr0;
#[doc = "Processor B Transmit Register 1"]
pub struct BTR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Processor B Transmit Register 1"]
pub mod btr1;
#[doc = "Processor B Transmit Register 2"]
pub struct BTR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Processor B Transmit Register 2"]
pub mod btr2;
#[doc = "Processor B Transmit Register 3"]
pub struct BTR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Processor B Transmit Register 3"]
pub mod btr3;
#[doc = "Processor B Receive Register 0"]
pub struct BRR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Processor B Receive Register 0"]
pub mod brr0;
#[doc = "Processor B Receive Register 1"]
pub struct BRR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Processor B Receive Register 1"]
pub mod brr1;
#[doc = "Processor B Receive Register 2"]
pub struct BRR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Processor B Receive Register 2"]
pub mod brr2;
#[doc = "Processor B Receive Register 3"]
pub struct BRR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Processor B Receive Register 3"]
pub mod brr3;
#[doc = "Processor B Status Register"]
pub struct BSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Processor B Status Register"]
pub mod bsr;
#[doc = "Processor B Control Register"]
pub struct BCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Processor B Control Register"]
pub mod bcr;
