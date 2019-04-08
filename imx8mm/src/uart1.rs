#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART Receiver Register"]
    pub urxd: URXD,
    _reserved0: [u8; 60usize],
    #[doc = "0x40 - UART Transmitter Register"]
    pub utxd: UTXD,
    _reserved1: [u8; 60usize],
    #[doc = "0x80 - UART Control Register 1"]
    pub ucr1: UCR1,
    #[doc = "0x84 - UART Control Register 2"]
    pub ucr2: UCR2,
    #[doc = "0x88 - UART Control Register 3"]
    pub ucr3: UCR3,
    #[doc = "0x8c - UART Control Register 4"]
    pub ucr4: UCR4,
    #[doc = "0x90 - UART FIFO Control Register"]
    pub ufcr: UFCR,
    #[doc = "0x94 - UART Status Register 1"]
    pub usr1: USR1,
    #[doc = "0x98 - UART Status Register 2"]
    pub usr2: USR2,
    #[doc = "0x9c - UART Escape Character Register"]
    pub uesc: UESC,
    #[doc = "0xa0 - UART Escape Timer Register"]
    pub utim: UTIM,
    #[doc = "0xa4 - UART BRM Incremental Register"]
    pub ubir: UBIR,
    #[doc = "0xa8 - UART BRM Modulator Register"]
    pub ubmr: UBMR,
    #[doc = "0xac - UART Baud Rate Count Register"]
    pub ubrc: UBRC,
    #[doc = "0xb0 - UART One Millisecond Register"]
    pub onems: ONEMS,
    #[doc = "0xb4 - UART Test Register"]
    pub uts: UTS,
    #[doc = "0xb8 - UART RS-485 Mode Control Register"]
    pub umcr: UMCR,
}
#[doc = "UART Receiver Register"]
pub struct URXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART Receiver Register"]
pub mod urxd;
#[doc = "UART Transmitter Register"]
pub struct UTXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART Transmitter Register"]
pub mod utxd;
#[doc = "UART Control Register 1"]
pub struct UCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART Control Register 1"]
pub mod ucr1;
#[doc = "UART Control Register 2"]
pub struct UCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART Control Register 2"]
pub mod ucr2;
#[doc = "UART Control Register 3"]
pub struct UCR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART Control Register 3"]
pub mod ucr3;
#[doc = "UART Control Register 4"]
pub struct UCR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART Control Register 4"]
pub mod ucr4;
#[doc = "UART FIFO Control Register"]
pub struct UFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART FIFO Control Register"]
pub mod ufcr;
#[doc = "UART Status Register 1"]
pub struct USR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART Status Register 1"]
pub mod usr1;
#[doc = "UART Status Register 2"]
pub struct USR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART Status Register 2"]
pub mod usr2;
#[doc = "UART Escape Character Register"]
pub struct UESC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART Escape Character Register"]
pub mod uesc;
#[doc = "UART Escape Timer Register"]
pub struct UTIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART Escape Timer Register"]
pub mod utim;
#[doc = "UART BRM Incremental Register"]
pub struct UBIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART BRM Incremental Register"]
pub mod ubir;
#[doc = "UART BRM Modulator Register"]
pub struct UBMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART BRM Modulator Register"]
pub mod ubmr;
#[doc = "UART Baud Rate Count Register"]
pub struct UBRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART Baud Rate Count Register"]
pub mod ubrc;
#[doc = "UART One Millisecond Register"]
pub struct ONEMS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART One Millisecond Register"]
pub mod onems;
#[doc = "UART Test Register"]
pub struct UTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART Test Register"]
pub mod uts;
#[doc = "UART RS-485 Mode Control Register"]
pub struct UMCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART RS-485 Mode Control Register"]
pub mod umcr;
