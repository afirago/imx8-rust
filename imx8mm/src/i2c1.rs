#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C Address Register"]
    pub iadr: IADR,
    _reserved0: [u8; 2usize],
    #[doc = "0x04 - I2C Frequency Divider Register"]
    pub ifdr: IFDR,
    _reserved1: [u8; 2usize],
    #[doc = "0x08 - I2C Control Register"]
    pub i2cr: I2CR,
    _reserved2: [u8; 2usize],
    #[doc = "0x0c - I2C Status Register"]
    pub i2sr: I2SR,
    _reserved3: [u8; 2usize],
    #[doc = "0x10 - I2C Data I/O Register"]
    pub i2dr: I2DR,
}
#[doc = "I2C Address Register"]
pub struct IADR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "I2C Address Register"]
pub mod iadr;
#[doc = "I2C Frequency Divider Register"]
pub struct IFDR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "I2C Frequency Divider Register"]
pub mod ifdr;
#[doc = "I2C Control Register"]
pub struct I2CR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "I2C Control Register"]
pub mod i2cr;
#[doc = "I2C Status Register"]
pub struct I2SR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "I2C Status Register"]
pub mod i2sr;
#[doc = "I2C Data I/O Register"]
pub struct I2DR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "I2C Data I/O Register"]
pub mod i2dr;
