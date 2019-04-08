#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - VERID"]
    pub verid: VERID,
    #[doc = "0x04 - PARAM"]
    pub param: PARAM,
    #[doc = "0x08 - TCSR"]
    pub tcsr: TCSR,
    #[doc = "0x0c - TCR1"]
    pub tcr1: TCR1,
    #[doc = "0x10 - TCR2"]
    pub tcr2: TCR2,
    #[doc = "0x14 - TCR3"]
    pub tcr3: TCR3,
    #[doc = "0x18 - TCR4"]
    pub tcr4: TCR4,
    #[doc = "0x1c - TCR5"]
    pub tcr5: TCR5,
    #[doc = "0x20 - TDR0"]
    pub tdr0: TDR0,
    #[doc = "0x24 - TDR1"]
    pub tdr1: TDR1,
    #[doc = "0x28 - TDR2"]
    pub tdr2: TDR2,
    #[doc = "0x2c - TDR3"]
    pub tdr3: TDR3,
    #[doc = "0x30 - TDR4"]
    pub tdr4: TDR4,
    #[doc = "0x34 - TDR5"]
    pub tdr5: TDR5,
    #[doc = "0x38 - TDR6"]
    pub tdr6: TDR6,
    #[doc = "0x3c - TDR7"]
    pub tdr7: TDR7,
    #[doc = "0x40 - TFR0"]
    pub tfr0: TFR0,
    #[doc = "0x44 - TFR1"]
    pub tfr1: TFR1,
    #[doc = "0x48 - TFR2"]
    pub tfr2: TFR2,
    #[doc = "0x4c - TFR3"]
    pub tfr3: TFR3,
    #[doc = "0x50 - TFR4"]
    pub tfr4: TFR4,
    #[doc = "0x54 - TFR5"]
    pub tfr5: TFR5,
    #[doc = "0x58 - TFR6"]
    pub tfr6: TFR6,
    #[doc = "0x5c - TFR7"]
    pub tfr7: TFR7,
    #[doc = "0x60 - TMR"]
    pub tmr: TMR,
    _reserved0: [u8; 12usize],
    #[doc = "0x70 - TTCR"]
    pub ttcr: TTCR,
    #[doc = "0x74 - TTSR"]
    pub ttsr: TTSR,
    #[doc = "0x78 - TBCR"]
    pub tbcr: TBCR,
    #[doc = "0x7c - TBCTR"]
    pub tbctr: TBCTR,
    #[doc = "0x80 - RCSR"]
    pub rcsr: RCSR,
    #[doc = "0x84 - RCR1"]
    pub rcr1: RCR1,
    #[doc = "0x88 - RCR2"]
    pub rcr2: RCR2,
    #[doc = "0x8c - RCR3"]
    pub rcr3: RCR3,
    #[doc = "0x90 - RCR4"]
    pub rcr4: RCR4,
    #[doc = "0x94 - RCR5"]
    pub rcr5: RCR5,
    _reserved1: [u8; 8usize],
    #[doc = "0xa0 - RDR0"]
    pub rdr0: RDR0,
    #[doc = "0xa4 - RDR1"]
    pub rdr1: RDR1,
    #[doc = "0xa8 - RDR2"]
    pub rdr2: RDR2,
    #[doc = "0xac - RDR3"]
    pub rdr3: RDR3,
    #[doc = "0xb0 - RDR4"]
    pub rdr4: RDR4,
    #[doc = "0xb4 - RDR5"]
    pub rdr5: RDR5,
    #[doc = "0xb8 - RDR6"]
    pub rdr6: RDR6,
    #[doc = "0xbc - RDR7"]
    pub rdr7: RDR7,
    #[doc = "0xc0 - RFR0"]
    pub rfr0: RFR0,
    #[doc = "0xc4 - RFR1"]
    pub rfr1: RFR1,
    #[doc = "0xc8 - RFR2"]
    pub rfr2: RFR2,
    #[doc = "0xcc - RFR3"]
    pub rfr3: RFR3,
    #[doc = "0xd0 - RFR4"]
    pub rfr4: RFR4,
    #[doc = "0xd4 - RFR5"]
    pub rfr5: RFR5,
    #[doc = "0xd8 - RFR6"]
    pub rfr6: RFR6,
    #[doc = "0xdc - RFR7"]
    pub rfr7: RFR7,
    #[doc = "0xe0 - RMR"]
    pub rmr: RMR,
    _reserved2: [u8; 12usize],
    #[doc = "0xf0 - RTCR"]
    pub rtcr: RTCR,
    #[doc = "0xf4 - RTSR"]
    pub rtsr: RTSR,
    #[doc = "0xf8 - RBCR"]
    pub rbcr: RBCR,
    #[doc = "0xfc - RBCTR"]
    pub rbctr: RBCTR,
    #[doc = "0x100 - MCR"]
    pub mcr: MCR,
    #[doc = "0x104 - MDR"]
    pub mdr: MDR,
}
#[doc = "VERID"]
pub struct VERID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VERID"]
pub mod verid;
#[doc = "PARAM"]
pub struct PARAM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PARAM"]
pub mod param;
#[doc = "TCSR"]
pub struct TCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCSR"]
pub mod tcsr;
#[doc = "TCR1"]
pub struct TCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCR1"]
pub mod tcr1;
#[doc = "TCR2"]
pub struct TCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCR2"]
pub mod tcr2;
#[doc = "TCR3"]
pub struct TCR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCR3"]
pub mod tcr3;
#[doc = "TCR4"]
pub struct TCR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCR4"]
pub mod tcr4;
#[doc = "TCR5"]
pub struct TCR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCR5"]
pub mod tcr5;
#[doc = "TDR0"]
pub struct TDR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TDR0"]
pub mod tdr0;
#[doc = "TDR1"]
pub struct TDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TDR1"]
pub mod tdr1;
#[doc = "TDR2"]
pub struct TDR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TDR2"]
pub mod tdr2;
#[doc = "TDR3"]
pub struct TDR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TDR3"]
pub mod tdr3;
#[doc = "TDR4"]
pub struct TDR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TDR4"]
pub mod tdr4;
#[doc = "TDR5"]
pub struct TDR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TDR5"]
pub mod tdr5;
#[doc = "TDR6"]
pub struct TDR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TDR6"]
pub mod tdr6;
#[doc = "TDR7"]
pub struct TDR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TDR7"]
pub mod tdr7;
#[doc = "TFR0"]
pub struct TFR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TFR0"]
pub mod tfr0;
#[doc = "TFR1"]
pub struct TFR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TFR1"]
pub mod tfr1;
#[doc = "TFR2"]
pub struct TFR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TFR2"]
pub mod tfr2;
#[doc = "TFR3"]
pub struct TFR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TFR3"]
pub mod tfr3;
#[doc = "TFR4"]
pub struct TFR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TFR4"]
pub mod tfr4;
#[doc = "TFR5"]
pub struct TFR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TFR5"]
pub mod tfr5;
#[doc = "TFR6"]
pub struct TFR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TFR6"]
pub mod tfr6;
#[doc = "TFR7"]
pub struct TFR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TFR7"]
pub mod tfr7;
#[doc = "TMR"]
pub struct TMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMR"]
pub mod tmr;
#[doc = "TTCR"]
pub struct TTCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TTCR"]
pub mod ttcr;
#[doc = "TTSR"]
pub struct TTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TTSR"]
pub mod ttsr;
#[doc = "TBCR"]
pub struct TBCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TBCR"]
pub mod tbcr;
#[doc = "TBCTR"]
pub struct TBCTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TBCTR"]
pub mod tbctr;
#[doc = "RCSR"]
pub struct RCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCSR"]
pub mod rcsr;
#[doc = "RCR1"]
pub struct RCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCR1"]
pub mod rcr1;
#[doc = "RCR2"]
pub struct RCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCR2"]
pub mod rcr2;
#[doc = "RCR3"]
pub struct RCR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCR3"]
pub mod rcr3;
#[doc = "RCR4"]
pub struct RCR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCR4"]
pub mod rcr4;
#[doc = "RCR5"]
pub struct RCR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCR5"]
pub mod rcr5;
#[doc = "RDR0"]
pub struct RDR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RDR0"]
pub mod rdr0;
#[doc = "RDR1"]
pub struct RDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RDR1"]
pub mod rdr1;
#[doc = "RDR2"]
pub struct RDR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RDR2"]
pub mod rdr2;
#[doc = "RDR3"]
pub struct RDR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RDR3"]
pub mod rdr3;
#[doc = "RDR4"]
pub struct RDR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RDR4"]
pub mod rdr4;
#[doc = "RDR5"]
pub struct RDR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RDR5"]
pub mod rdr5;
#[doc = "RDR6"]
pub struct RDR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RDR6"]
pub mod rdr6;
#[doc = "RDR7"]
pub struct RDR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RDR7"]
pub mod rdr7;
#[doc = "RFR0"]
pub struct RFR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RFR0"]
pub mod rfr0;
#[doc = "RFR1"]
pub struct RFR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RFR1"]
pub mod rfr1;
#[doc = "RFR2"]
pub struct RFR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RFR2"]
pub mod rfr2;
#[doc = "RFR3"]
pub struct RFR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RFR3"]
pub mod rfr3;
#[doc = "RFR4"]
pub struct RFR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RFR4"]
pub mod rfr4;
#[doc = "RFR5"]
pub struct RFR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RFR5"]
pub mod rfr5;
#[doc = "RFR6"]
pub struct RFR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RFR6"]
pub mod rfr6;
#[doc = "RFR7"]
pub struct RFR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RFR7"]
pub mod rfr7;
#[doc = "RMR"]
pub struct RMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RMR"]
pub mod rmr;
#[doc = "RTCR"]
pub struct RTCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTCR"]
pub mod rtcr;
#[doc = "RTSR"]
pub struct RTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTSR"]
pub mod rtsr;
#[doc = "RBCR"]
pub struct RBCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RBCR"]
pub mod rbcr;
#[doc = "RBCTR"]
pub struct RBCTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RBCTR"]
pub mod rbctr;
#[doc = "MCR"]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCR"]
pub mod mcr;
#[doc = "MDR"]
pub struct MDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDR"]
pub mod mdr;
