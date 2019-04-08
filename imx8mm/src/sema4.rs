#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Semaphores Gate 0 Register"]
    pub gate00: GATE00,
    #[doc = "0x01 - Semaphores Gate 1 Register"]
    pub gate01: GATE01,
    #[doc = "0x02 - Semaphores Gate 2 Register"]
    pub gate02: GATE02,
    #[doc = "0x03 - Semaphores Gate 3 Register"]
    pub gate03: GATE03,
    #[doc = "0x04 - Semaphores Gate 4 Register"]
    pub gate04: GATE04,
    #[doc = "0x05 - Semaphores Gate 5 Register"]
    pub gate05: GATE05,
    #[doc = "0x06 - Semaphores Gate 6 Register"]
    pub gate06: GATE06,
    #[doc = "0x07 - Semaphores Gate 7 Register"]
    pub gate07: GATE07,
    #[doc = "0x08 - Semaphores Gate 8 Register"]
    pub gate08: GATE08,
    #[doc = "0x09 - Semaphores Gate 9 Register"]
    pub gate09: GATE09,
    #[doc = "0x0a - Semaphores Gate 10 Register"]
    pub gate10: GATE10,
    #[doc = "0x0b - Semaphores Gate 11 Register"]
    pub gate11: GATE11,
    #[doc = "0x0c - Semaphores Gate 12 Register"]
    pub gate12: GATE12,
    #[doc = "0x0d - Semaphores Gate 13 Register"]
    pub gate13: GATE13,
    #[doc = "0x0e - Semaphores Gate 14 Register"]
    pub gate14: GATE14,
    #[doc = "0x0f - Semaphores Gate 15 Register"]
    pub gate15: GATE15,
    _reserved0: [u8; 48usize],
    #[doc = "0x40 - Semaphores Processor n IRQ Notification Enable"]
    pub cp0ine: CPINE,
    _reserved1: [u8; 6usize],
    #[doc = "0x48 - Semaphores Processor n IRQ Notification Enable"]
    pub cp1ine: CPINE,
    _reserved2: [u8; 54usize],
    #[doc = "0x80 - Semaphores Processor n IRQ Notification"]
    pub cp0ntf: CPNTF,
    _reserved3: [u8; 6usize],
    #[doc = "0x88 - Semaphores Processor n IRQ Notification"]
    pub cp1ntf: CPNTF,
    _reserved4: [u8; 118usize],
    #[doc = "0x100 - Semaphores (Secure) Reset Gate n"]
    pub rstgt: RSTGT,
    _reserved5: [u8; 2usize],
    #[doc = "0x104 - Semaphores (Secure) Reset IRQ Notification"]
    pub rstntf: RSTNTF,
}
#[doc = "Semaphores Gate 0 Register"]
pub struct GATE00 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Semaphores Gate 0 Register"]
pub mod gate00;
#[doc = "Semaphores Gate 1 Register"]
pub struct GATE01 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Semaphores Gate 1 Register"]
pub mod gate01;
#[doc = "Semaphores Gate 2 Register"]
pub struct GATE02 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Semaphores Gate 2 Register"]
pub mod gate02;
#[doc = "Semaphores Gate 3 Register"]
pub struct GATE03 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Semaphores Gate 3 Register"]
pub mod gate03;
#[doc = "Semaphores Gate 4 Register"]
pub struct GATE04 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Semaphores Gate 4 Register"]
pub mod gate04;
#[doc = "Semaphores Gate 5 Register"]
pub struct GATE05 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Semaphores Gate 5 Register"]
pub mod gate05;
#[doc = "Semaphores Gate 6 Register"]
pub struct GATE06 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Semaphores Gate 6 Register"]
pub mod gate06;
#[doc = "Semaphores Gate 7 Register"]
pub struct GATE07 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Semaphores Gate 7 Register"]
pub mod gate07;
#[doc = "Semaphores Gate 8 Register"]
pub struct GATE08 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Semaphores Gate 8 Register"]
pub mod gate08;
#[doc = "Semaphores Gate 9 Register"]
pub struct GATE09 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Semaphores Gate 9 Register"]
pub mod gate09;
#[doc = "Semaphores Gate 10 Register"]
pub struct GATE10 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Semaphores Gate 10 Register"]
pub mod gate10;
#[doc = "Semaphores Gate 11 Register"]
pub struct GATE11 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Semaphores Gate 11 Register"]
pub mod gate11;
#[doc = "Semaphores Gate 12 Register"]
pub struct GATE12 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Semaphores Gate 12 Register"]
pub mod gate12;
#[doc = "Semaphores Gate 13 Register"]
pub struct GATE13 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Semaphores Gate 13 Register"]
pub mod gate13;
#[doc = "Semaphores Gate 14 Register"]
pub struct GATE14 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Semaphores Gate 14 Register"]
pub mod gate14;
#[doc = "Semaphores Gate 15 Register"]
pub struct GATE15 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Semaphores Gate 15 Register"]
pub mod gate15;
#[doc = "Semaphores Processor n IRQ Notification Enable"]
pub struct CPINE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Semaphores Processor n IRQ Notification Enable"]
pub mod cpine;
#[doc = "Semaphores Processor n IRQ Notification"]
pub struct CPNTF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Semaphores Processor n IRQ Notification"]
pub mod cpntf;
#[doc = "Semaphores (Secure) Reset Gate n"]
pub struct RSTGT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Semaphores (Secure) Reset Gate n"]
pub mod rstgt;
#[doc = "Semaphores (Secure) Reset IRQ Notification"]
pub struct RSTNTF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Semaphores (Secure) Reset IRQ Notification"]
pub mod rstntf;
