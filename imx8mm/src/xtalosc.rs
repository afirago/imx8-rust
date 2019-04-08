#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OSC Normal Clock Generation Control Register0"]
    pub sys_oscnml_ctl0: SYS_OSCNML_CTL0,
    #[doc = "0x04 - OSC Normal Clock Generation Control Register1"]
    pub sys_oscnml_ctl1: SYS_OSCNML_CTL1,
}
#[doc = "OSC Normal Clock Generation Control Register0"]
pub struct SYS_OSCNML_CTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OSC Normal Clock Generation Control Register0"]
pub mod sys_oscnml_ctl0;
#[doc = "OSC Normal Clock Generation Control Register1"]
pub struct SYS_OSCNML_CTL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OSC Normal Clock Generation Control Register1"]
pub mod sys_oscnml_ctl1;
