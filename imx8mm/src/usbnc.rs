#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 512usize],
    #[doc = "0x200 - USB OTG Control 1 Register"]
    pub otg1_ctrl1: _CTRL1,
    #[doc = "0x204 - USB OTG Control 2 Register"]
    pub otg1_ctrl2: _CTRL2,
    _reserved1: [u8; 1528usize],
    #[doc = "0x800 - USB OTG1 Control Register"]
    pub usb_otg1_ctrl: USB_OTG1_CTRL,
    #[doc = "0x804 - USB OTG2 Control Register"]
    pub usb_otg2_ctrl: USB_OTG2_CTRL,
    _reserved2: [u8; 16usize],
    #[doc = "0x818 - OTG1 UTMI PHY Control 0 Register"]
    pub usb_otg1_phy_ctrl_0: USB_OTG1_PHY_CTRL_0,
    #[doc = "0x81c - OTG2 UTMI PHY Control 0 Register"]
    pub usb_otg2_phy_ctrl_0: USB_OTG2_PHY_CTRL_0,
    _reserved3: [u8; 63968usize],
    #[doc = "0x10200 - USB OTG Control 1 Register"]
    pub otg2_ctrl1: _CTRL1,
    #[doc = "0x10204 - USB OTG Control 2 Register"]
    pub otg2_ctrl2: _CTRL2,
}
#[doc = "USB OTG Control 1 Register"]
pub struct _CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB OTG Control 1 Register"]
pub mod _ctrl1;
#[doc = "USB OTG Control 2 Register"]
pub struct _CTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB OTG Control 2 Register"]
pub mod _ctrl2;
#[doc = "USB OTG1 Control Register"]
pub struct USB_OTG1_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB OTG1 Control Register"]
pub mod usb_otg1_ctrl;
#[doc = "USB OTG2 Control Register"]
pub struct USB_OTG2_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB OTG2 Control Register"]
pub mod usb_otg2_ctrl;
#[doc = "OTG1 UTMI PHY Control 0 Register"]
pub struct USB_OTG1_PHY_CTRL_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG1 UTMI PHY Control 0 Register"]
pub mod usb_otg1_phy_ctrl_0;
#[doc = "OTG2 UTMI PHY Control 0 Register"]
pub struct USB_OTG2_PHY_CTRL_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG2 UTMI PHY Control 0 Register"]
pub mod usb_otg2_phy_ctrl_0;
