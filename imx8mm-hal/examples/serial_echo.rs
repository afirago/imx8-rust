#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use imx8mm_hal as hal;

use crate::hal::{prelude::*, serial::Serial, imx8mm};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let Some(p) = imx8mm::Peripherals::take() {
        cortex_m::interrupt::free(move |cs| {

            let mut serial = Serial::uart1(p.UART1, 115_200.bps());

            loop {
                // Wait for reception of a single byte
                let received = nb::block!(serial.read()).unwrap();

                // Send back previously received byte and wait for completion
                nb::block!(serial.write(received)).ok();
            }
        });
    }

    loop {
        continue;
    }
}
