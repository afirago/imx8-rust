//! API for the integrated UART ports

use embedded_hal::prelude::*;

use nb;

use core::marker::PhantomData;

use core::{
    fmt::{Result, Write},
    ops::Deref,
};

use crate::{time::Bps};

use void;

/// Serial error
#[derive(Debug)]
pub enum Error {
    /// Framing error
    Framing,
    /// Noise error
    Noise,
    /// RX buffer overrun
    Overrun,
    /// Parity check error
    Parity,
    #[doc(hidden)]
    _Extensible,
}

/// Interrupt event
pub enum Event {
    /// New data has been received
    Rxne,
    /// New data can be sent
    Txe,
    /// Idle line state detected
    Idle,
}

/// Serial abstraction
pub struct Serial<UART> {
    uart: UART,
    // TODO: pins: (TXPIN, RXPIN),
}

// Common register
type SerialRegisterBlock = crate::imx8mm::uart1::RegisterBlock;

/// Serial receiver
pub struct Rx<UART> {
    uart: *const SerialRegisterBlock,
    _instance: PhantomData<UART>,
}

// NOTE(unsafe) Required to allow protected shared access in handlers
unsafe impl<UART> Send for Rx<UART> {}

/// Serial transmitter
pub struct Tx<UART> {
    uart: *const SerialRegisterBlock,
    _instance: PhantomData<UART>,
}

// NOTE(unsafe) Required to allow protected shared access in handlers
unsafe impl<UART> Send for Tx<UART> {}

macro_rules! uart {
    ($($UART:ident: ($uart:ident),)+) => {
        $(
            use crate::imx8mm::$UART;
            impl Serial<$UART>
            {
                /// Creates a new serial instance
                pub fn $uart(uart: $UART, baud_rate: Bps) -> Self
                {
                    let mut serial = Serial { uart };

                    // Use init procedure from U-Boot

                    // set to default POR state
                    serial.uart.ucr1.reset();
                    serial.uart.ucr2.reset();

                    // wait for reset
                    // TODO: can we wait here?
                    while !serial.uart.ucr2.read().srst().bit_is_set() {}

                    // RXDMUXSEL must be set
                    // Set ADNIMP to disable autobaud detection
                    serial.uart.ucr3.write(|w| w.rxdmuxsel().set_bit().adnimp().set_bit());

                    // Set CTS trigger level to 32 chars
                    serial.uart.ucr4.write(|w| w.ctstl().ctstl_32());

                    // Set escape character
                    serial.uart.uesc.write(|w| unsafe {w.esc_char().bits(0x2b)});

                    // Reset registers with unused settings
                    serial.uart.utim.reset();
                    serial.uart.uts.reset();

                    // configure baudrate
                    serial.configure(baud_rate);

                    // Enable UART
                    serial.uart.ucr1.modify(|_, w| w.uarten().set_bit());
                    serial
                }
            }

            impl Serial<$UART> {
                fn configure(&mut self, baud_rate: Bps) {
                    // TODO: initialize required clocks

                    self.uart.ubir.write(|w| unsafe {w.inc().bits(0xf)});
                    // set to 8-bit, ignore RTS, enable RX, enable TX, clear reset bit
                    self.uart.ucr2.write(|w| w.ws().set_bit().irts().set_bit().
                            rxen().set_bit().txen().set_bit().srst().set_bit());
                }

                /// Starts listening for an interrupt event
                pub fn listen(&mut self, event: Event) {
                    match event {
                        Event::Rxne => {
                            self.uart.ucr1.modify(|_, w| w.rrdyen().set_bit())
                        },
                        Event::Txe => {
                            self.uart.ucr1.modify(|_, w| w.trdyen().set_bit())
                        },
                        Event::Idle => {
                            self.uart.ucr1.modify(|_, w| w.iden().set_bit())
                        },
                    }
                }

                /// Stop listening for an interrupt event
                pub fn unlisten(&mut self, event: Event) {
                    match event {
                        Event::Rxne => {
                            self.uart.ucr1.modify(|_, w| w.rrdyen().clear_bit())
                        },
                        Event::Txe => {
                            self.uart.ucr1.modify(|_, w| w.trdyen().clear_bit())
                        },
                        Event::Idle => {
                            self.uart.ucr1.modify(|_, w| w.iden().clear_bit())
                        },
                    }
                }
            }
        )+
    }
}

impl<UART> Serial<UART>
where
    UART: Deref<Target = SerialRegisterBlock>,
{
    /// Splits the UART Peripheral in a Tx and an Rx part
    /// This is required for sending/receiving
    pub fn split(self) -> (Tx<UART>, Rx<UART>)
    {
        (
            Tx {
                uart: &*self.uart,
                _instance: PhantomData,
            },
            Rx {
                uart: &*self.uart,
                _instance: PhantomData,
            },
        )
    }

    pub fn release(self) -> (UART) {
        (self.uart)
    }
}

impl<UART> embedded_hal::serial::Read<u8> for Rx<UART>
where
    UART: Deref<Target = SerialRegisterBlock>,
{
    type Error = Error;

    /// Tries to read a byte from the uart
    fn read(&mut self) -> nb::Result<u8, Error> {
        read(self.uart)
    }
}

impl<UART> embedded_hal::serial::Read<u8> for Serial<UART>
where
    UART: Deref<Target = SerialRegisterBlock>,
{
    type Error = Error;

    /// Tries to read a byte from the uart
    fn read(&mut self) -> nb::Result<u8, Error> {
        read(&*self.uart)
    }
}

impl<UART> Write for Tx<UART>
where
    Tx<UART>: embedded_hal::serial::Write<u8>,
{
    fn write_str(&mut self, s: &str) -> Result {
        s.as_bytes()
            .iter()
            .try_for_each(|c| nb::block!(self.write(*c)))
            .map_err(|_| core::fmt::Error)
    }
}

impl<UART> embedded_hal::serial::Write<u8> for Tx<UART>
where
    UART: Deref<Target = SerialRegisterBlock>,
{
    type Error = void::Void;

    /// Ensures that none of the previously written words are still buffered
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        flush(self.uart)
    }

    /// Tries to write a byte to the uart
    /// Fails if the transmit buffer is full
    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        write(self.uart, byte)
    }
}

impl<UART> embedded_hal::serial::Write<u8> for Serial<UART>
where
    UART: Deref<Target = SerialRegisterBlock>,
{
    type Error = void::Void;

    /// Ensures that none of the previously written words are still buffered
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        flush(&*self.uart)
    }

    /// Tries to write a byte to the uart
    /// Fails if the transmit buffer is full
    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        write(&*self.uart, byte)
    }
}

impl<UART> Write for Serial<UART>
where
    UART: Deref<Target = SerialRegisterBlock>,
{
    fn write_str(&mut self, s: &str) -> Result {
        s.as_bytes()
            .iter()
            .try_for_each(|c| nb::block!(self.write(*c)))
            .map_err(|_| core::fmt::Error)
    }
}

/// Ensures that none of the previously written words are still buffered
fn flush(uart: *const SerialRegisterBlock) -> nb::Result<(), void::Void> {
    // NOTE(unsafe) atomic read with no side effects
    let uts = unsafe { (*uart).uts.read() };

    if uts.txempty().bit_is_set() {
        Ok(())
    } else {
        Err(nb::Error::WouldBlock)
    }
}

/// Tries to write a byte to the UART
/// Fails if the transmit buffer is full
fn write(uart: *const SerialRegisterBlock, byte: u8) -> nb::Result<(), void::Void> {
    // NOTE(unsafe) atomic read with no side effects
    let uts = unsafe { (*uart).uts.read() };

    if !uts.txfull().bit_is_set() {
        // NOTE(unsafe) atomic write to stateless register
        unsafe { (*uart).utxd.write(|w| w.tx_data().bits(byte)) }
        Ok(())
    } else {
        Err(nb::Error::WouldBlock)
    }
}

/// Tries to read a byte from the UART
fn read(uart: *const SerialRegisterBlock) -> nb::Result<u8, Error> {
    // NOTE(unsafe) atomic read with no side effects
    let uts = unsafe { (*uart).uts.read() };

    if uts.rxempty().bit_is_set() {
        Err(nb::Error::WouldBlock)
    } else {
        // NOTE(unsafe) atomic read with no side effects
        let urxd = unsafe { (*uart).urxd.read() };

        if urxd.prerr().bit_is_set() {
            Err(nb::Error::Other(Error::Parity))
        } else if urxd.frmerr().bit_is_set() {
            Err(nb::Error::Other(Error::Framing))
        } else if urxd.ovrrun().bit_is_set() {
            Err(nb::Error::Other(Error::Overrun))
        } else {
            Ok(urxd.rx_data().bits())
        }
    }
}

uart! {
    UART1: (uart1),
}
