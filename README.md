# imx8-rust
Rust on the NXP i.MX8 SoC.

> **IMPORTANT**: This repository contains experimental crates to play with Rust on Cortex-M4 core(s) of the NXP i.MX8 SoCs

# Structure
* imx8mm - contains svd2rust interface
* imx8mm-hal - contains embedded-hal traits implementation which uses svd2rust interface from imx8mm

# Building

```
$ cd imx8mm-hal
$ cargo build --target=thumbv7em-none-eabihf --examples
```
# TODO
* Implement proper clock settings for UART (for now it is assumed that clocks have been set by bootloader)
* Implement proper HAL interface around IOMUXC (for example like here - https://github.com/riscv-rust/e310x-hal/blob/master/src/gpio.rs) 

# References
* SVD files for iMX8 - https://community.nxp.com/thread/496583
