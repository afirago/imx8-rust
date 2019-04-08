//! HAL for the Cortex-M4 core of NXP iMX8MM SoC

#![no_std]

pub use embedded_hal as hal;

extern crate bare_metal;
extern crate cortex_m;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;

pub use nb;
pub use nb::block;

pub use imx8mm;

pub mod prelude;

pub mod time;

pub mod serial;
