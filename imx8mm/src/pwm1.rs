#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Control Register"]
    pub pwmcr: PWMCR,
    #[doc = "0x04 - PWM Status Register"]
    pub pwmsr: PWMSR,
    #[doc = "0x08 - PWM Interrupt Register"]
    pub pwmir: PWMIR,
    #[doc = "0x0c - PWM Sample Register"]
    pub pwmsar: PWMSAR,
    #[doc = "0x10 - PWM Period Register"]
    pub pwmpr: PWMPR,
    #[doc = "0x14 - PWM Counter Register"]
    pub pwmcnr: PWMCNR,
}
#[doc = "PWM Control Register"]
pub struct PWMCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Control Register"]
pub mod pwmcr;
#[doc = "PWM Status Register"]
pub struct PWMSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Status Register"]
pub mod pwmsr;
#[doc = "PWM Interrupt Register"]
pub struct PWMIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Interrupt Register"]
pub mod pwmir;
#[doc = "PWM Sample Register"]
pub struct PWMSAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Sample Register"]
pub mod pwmsar;
#[doc = "PWM Period Register"]
pub struct PWMPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Period Register"]
pub mod pwmpr;
#[doc = "PWM Counter Register"]
pub struct PWMCNR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Counter Register"]
pub mod pwmcnr;
