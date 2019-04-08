#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TMU Enable Register"]
    pub ter: TER,
    #[doc = "0x04 - TMU Status register"]
    pub tsr: TSR,
    #[doc = "0x08 - TMU Interrupt Enable register"]
    pub tier: TIER,
    #[doc = "0x0c - TMU Interrupt Detect register"]
    pub tidr: TIDR,
    #[doc = "0x10 - TMU Monitor High Temperature Immediate Threshold register"]
    pub tmhtitr: TMHTITR,
    #[doc = "0x14 - TMU Monitor High Temperature Average threshold register"]
    pub tmhtatr: TMHTATR,
    #[doc = "0x18 - TMU Monitor High Temperature Average Critical Threshold register"]
    pub tmhtactr: TMHTACTR,
    #[doc = "0x1c - TMU Sensor Calibration register"]
    pub tscr: TSCR,
    #[doc = "0x20 - TMU Report Immediate Temperature Site register n"]
    pub tritsr: TRITSR,
    #[doc = "0x24 - TMU Report Average Temperature Site register n"]
    pub tratsr: TRATSR,
    #[doc = "0x28 - no description available"]
    pub tasr: TASR,
    #[doc = "0x2c - no description available"]
    pub ttmc: TTMC,
    #[doc = "0x30 - no description available"]
    pub tcaliv: TCALIV,
}
#[doc = "TMU Enable Register"]
pub struct TER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMU Enable Register"]
pub mod ter;
#[doc = "TMU Status register"]
pub struct TSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMU Status register"]
pub mod tsr;
#[doc = "TMU Interrupt Enable register"]
pub struct TIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMU Interrupt Enable register"]
pub mod tier;
#[doc = "TMU Interrupt Detect register"]
pub struct TIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMU Interrupt Detect register"]
pub mod tidr;
#[doc = "TMU Monitor High Temperature Immediate Threshold register"]
pub struct TMHTITR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMU Monitor High Temperature Immediate Threshold register"]
pub mod tmhtitr;
#[doc = "TMU Monitor High Temperature Average threshold register"]
pub struct TMHTATR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMU Monitor High Temperature Average threshold register"]
pub mod tmhtatr;
#[doc = "TMU Monitor High Temperature Average Critical Threshold register"]
pub struct TMHTACTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMU Monitor High Temperature Average Critical Threshold register"]
pub mod tmhtactr;
#[doc = "TMU Sensor Calibration register"]
pub struct TSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMU Sensor Calibration register"]
pub mod tscr;
#[doc = "TMU Report Immediate Temperature Site register n"]
pub struct TRITSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMU Report Immediate Temperature Site register n"]
pub mod tritsr;
#[doc = "TMU Report Average Temperature Site register n"]
pub struct TRATSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMU Report Average Temperature Site register n"]
pub mod tratsr;
#[doc = "no description available"]
pub struct TASR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod tasr;
#[doc = "no description available"]
pub struct TTMC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod ttmc;
#[doc = "no description available"]
pub struct TCALIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod tcaliv;
