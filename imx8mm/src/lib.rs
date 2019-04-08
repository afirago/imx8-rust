#![doc = "Peripheral access API for MIMX8MM6_CM4 microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn SDMA1();
    fn LCDIF();
    fn WDOG3();
    fn APBHDMA();
    fn BCH();
    fn GPMI();
    fn USDHC1();
    fn USDHC2();
    fn USDHC3();
    fn UART1();
    fn UART2();
    fn UART3();
    fn UART4();
    fn ECSPI1();
    fn ECSPI2();
    fn ECSPI3();
    fn SDMA3();
    fn I2C1();
    fn I2C2();
    fn I2C3();
    fn I2C4();
    fn RDC();
    fn GPT6();
    fn I2S3();
    fn GPT5();
    fn GPT4();
    fn GPT3();
    fn GPT2();
    fn GPT1();
    fn GPIO1_INT7();
    fn GPIO1_INT6();
    fn GPIO1_INT5();
    fn GPIO1_INT4();
    fn GPIO1_INT3();
    fn GPIO1_INT2();
    fn GPIO1_INT1();
    fn GPIO1_INT0();
    fn GPIO1_COMBINED_0_15();
    fn GPIO1_COMBINED_16_31();
    fn GPIO2_COMBINED_0_15();
    fn GPIO2_COMBINED_16_31();
    fn GPIO3_COMBINED_0_15();
    fn GPIO3_COMBINED_16_31();
    fn GPIO4_COMBINED_0_15();
    fn GPIO4_COMBINED_16_31();
    fn GPIO5_COMBINED_0_15();
    fn GPIO5_COMBINED_16_31();
    fn WDOG1();
    fn WDOG2();
    fn PWM1();
    fn PWM2();
    fn PWM3();
    fn PWM4();
    fn CCM_IRQ1();
    fn CCM_IRQ2();
    fn GPC();
    fn SRC();
    fn I2S56();
    fn SRC_COMBINED();
    fn I2S1();
    fn I2S2();
    fn MU_M4();
    fn SDMA2();
    fn ENET();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 121] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SDMA1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: LCDIF },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: WDOG3 },
    Vector { _reserved: 0 },
    Vector { _handler: APBHDMA },
    Vector { _reserved: 0 },
    Vector { _handler: BCH },
    Vector { _handler: GPMI },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: USDHC1 },
    Vector { _handler: USDHC2 },
    Vector { _handler: USDHC3 },
    Vector { _reserved: 0 },
    Vector { _handler: UART1 },
    Vector { _handler: UART2 },
    Vector { _handler: UART3 },
    Vector { _handler: UART4 },
    Vector { _reserved: 0 },
    Vector { _handler: ECSPI1 },
    Vector { _handler: ECSPI2 },
    Vector { _handler: ECSPI3 },
    Vector { _handler: SDMA3 },
    Vector { _handler: I2C1 },
    Vector { _handler: I2C2 },
    Vector { _handler: I2C3 },
    Vector { _handler: I2C4 },
    Vector { _handler: RDC },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: GPT6 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: I2S3 },
    Vector { _handler: GPT5 },
    Vector { _handler: GPT4 },
    Vector { _handler: GPT3 },
    Vector { _handler: GPT2 },
    Vector { _handler: GPT1 },
    Vector {
        _handler: GPIO1_INT7,
    },
    Vector {
        _handler: GPIO1_INT6,
    },
    Vector {
        _handler: GPIO1_INT5,
    },
    Vector {
        _handler: GPIO1_INT4,
    },
    Vector {
        _handler: GPIO1_INT3,
    },
    Vector {
        _handler: GPIO1_INT2,
    },
    Vector {
        _handler: GPIO1_INT1,
    },
    Vector {
        _handler: GPIO1_INT0,
    },
    Vector {
        _handler: GPIO1_COMBINED_0_15,
    },
    Vector {
        _handler: GPIO1_COMBINED_16_31,
    },
    Vector {
        _handler: GPIO2_COMBINED_0_15,
    },
    Vector {
        _handler: GPIO2_COMBINED_16_31,
    },
    Vector {
        _handler: GPIO3_COMBINED_0_15,
    },
    Vector {
        _handler: GPIO3_COMBINED_16_31,
    },
    Vector {
        _handler: GPIO4_COMBINED_0_15,
    },
    Vector {
        _handler: GPIO4_COMBINED_16_31,
    },
    Vector {
        _handler: GPIO5_COMBINED_0_15,
    },
    Vector {
        _handler: GPIO5_COMBINED_16_31,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: WDOG1 },
    Vector { _handler: WDOG2 },
    Vector { _reserved: 0 },
    Vector { _handler: PWM1 },
    Vector { _handler: PWM2 },
    Vector { _handler: PWM3 },
    Vector { _handler: PWM4 },
    Vector { _handler: CCM_IRQ1 },
    Vector { _handler: CCM_IRQ2 },
    Vector { _handler: GPC },
    Vector { _reserved: 0 },
    Vector { _handler: SRC },
    Vector { _handler: I2S56 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: SRC_COMBINED,
    },
    Vector { _handler: I2S1 },
    Vector { _handler: I2S2 },
    Vector { _handler: MU_M4 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SDMA2 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: ENET },
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "2 - SDMA1"]
    SDMA1,
    #[doc = "5 - LCDIF"]
    LCDIF,
    #[doc = "10 - WDOG3"]
    WDOG3,
    #[doc = "12 - APBHDMA"]
    APBHDMA,
    #[doc = "14 - BCH"]
    BCH,
    #[doc = "15 - GPMI"]
    GPMI,
    #[doc = "22 - USDHC1"]
    USDHC1,
    #[doc = "23 - USDHC2"]
    USDHC2,
    #[doc = "24 - USDHC3"]
    USDHC3,
    #[doc = "26 - UART1"]
    UART1,
    #[doc = "27 - UART2"]
    UART2,
    #[doc = "28 - UART3"]
    UART3,
    #[doc = "29 - UART4"]
    UART4,
    #[doc = "31 - ECSPI1"]
    ECSPI1,
    #[doc = "32 - ECSPI2"]
    ECSPI2,
    #[doc = "33 - ECSPI3"]
    ECSPI3,
    #[doc = "34 - SDMA3"]
    SDMA3,
    #[doc = "35 - I2C1"]
    I2C1,
    #[doc = "36 - I2C2"]
    I2C2,
    #[doc = "37 - I2C3"]
    I2C3,
    #[doc = "38 - I2C4"]
    I2C4,
    #[doc = "39 - RDC"]
    RDC,
    #[doc = "46 - GPT6"]
    GPT6,
    #[doc = "50 - I2S3"]
    I2S3,
    #[doc = "51 - GPT5"]
    GPT5,
    #[doc = "52 - GPT4"]
    GPT4,
    #[doc = "53 - GPT3"]
    GPT3,
    #[doc = "54 - GPT2"]
    GPT2,
    #[doc = "55 - GPT1"]
    GPT1,
    #[doc = "56 - GPIO1_INT7"]
    GPIO1_INT7,
    #[doc = "57 - GPIO1_INT6"]
    GPIO1_INT6,
    #[doc = "58 - GPIO1_INT5"]
    GPIO1_INT5,
    #[doc = "59 - GPIO1_INT4"]
    GPIO1_INT4,
    #[doc = "60 - GPIO1_INT3"]
    GPIO1_INT3,
    #[doc = "61 - GPIO1_INT2"]
    GPIO1_INT2,
    #[doc = "62 - GPIO1_INT1"]
    GPIO1_INT1,
    #[doc = "63 - GPIO1_INT0"]
    GPIO1_INT0,
    #[doc = "64 - GPIO1_Combined_0_15"]
    GPIO1_COMBINED_0_15,
    #[doc = "65 - GPIO1_Combined_16_31"]
    GPIO1_COMBINED_16_31,
    #[doc = "66 - GPIO2_Combined_0_15"]
    GPIO2_COMBINED_0_15,
    #[doc = "67 - GPIO2_Combined_16_31"]
    GPIO2_COMBINED_16_31,
    #[doc = "68 - GPIO3_Combined_0_15"]
    GPIO3_COMBINED_0_15,
    #[doc = "69 - GPIO3_Combined_16_31"]
    GPIO3_COMBINED_16_31,
    #[doc = "70 - GPIO4_Combined_0_15"]
    GPIO4_COMBINED_0_15,
    #[doc = "71 - GPIO4_Combined_16_31"]
    GPIO4_COMBINED_16_31,
    #[doc = "72 - GPIO5_Combined_0_15"]
    GPIO5_COMBINED_0_15,
    #[doc = "73 - GPIO5_Combined_16_31"]
    GPIO5_COMBINED_16_31,
    #[doc = "78 - WDOG1"]
    WDOG1,
    #[doc = "79 - WDOG2"]
    WDOG2,
    #[doc = "81 - PWM1"]
    PWM1,
    #[doc = "82 - PWM2"]
    PWM2,
    #[doc = "83 - PWM3"]
    PWM3,
    #[doc = "84 - PWM4"]
    PWM4,
    #[doc = "85 - CCM_IRQ1"]
    CCM_IRQ1,
    #[doc = "86 - CCM_IRQ2"]
    CCM_IRQ2,
    #[doc = "87 - GPC"]
    GPC,
    #[doc = "89 - SRC"]
    SRC,
    #[doc = "90 - I2S56"]
    I2S56,
    #[doc = "94 - SRC_Combined"]
    SRC_COMBINED,
    #[doc = "95 - I2S1"]
    I2S1,
    #[doc = "96 - I2S2"]
    I2S2,
    #[doc = "97 - MU_M4"]
    MU_M4,
    #[doc = "103 - SDMA2"]
    SDMA2,
    #[doc = "120 - ENET"]
    ENET,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::SDMA1 => 2,
            Interrupt::LCDIF => 5,
            Interrupt::WDOG3 => 10,
            Interrupt::APBHDMA => 12,
            Interrupt::BCH => 14,
            Interrupt::GPMI => 15,
            Interrupt::USDHC1 => 22,
            Interrupt::USDHC2 => 23,
            Interrupt::USDHC3 => 24,
            Interrupt::UART1 => 26,
            Interrupt::UART2 => 27,
            Interrupt::UART3 => 28,
            Interrupt::UART4 => 29,
            Interrupt::ECSPI1 => 31,
            Interrupt::ECSPI2 => 32,
            Interrupt::ECSPI3 => 33,
            Interrupt::SDMA3 => 34,
            Interrupt::I2C1 => 35,
            Interrupt::I2C2 => 36,
            Interrupt::I2C3 => 37,
            Interrupt::I2C4 => 38,
            Interrupt::RDC => 39,
            Interrupt::GPT6 => 46,
            Interrupt::I2S3 => 50,
            Interrupt::GPT5 => 51,
            Interrupt::GPT4 => 52,
            Interrupt::GPT3 => 53,
            Interrupt::GPT2 => 54,
            Interrupt::GPT1 => 55,
            Interrupt::GPIO1_INT7 => 56,
            Interrupt::GPIO1_INT6 => 57,
            Interrupt::GPIO1_INT5 => 58,
            Interrupt::GPIO1_INT4 => 59,
            Interrupt::GPIO1_INT3 => 60,
            Interrupt::GPIO1_INT2 => 61,
            Interrupt::GPIO1_INT1 => 62,
            Interrupt::GPIO1_INT0 => 63,
            Interrupt::GPIO1_COMBINED_0_15 => 64,
            Interrupt::GPIO1_COMBINED_16_31 => 65,
            Interrupt::GPIO2_COMBINED_0_15 => 66,
            Interrupt::GPIO2_COMBINED_16_31 => 67,
            Interrupt::GPIO3_COMBINED_0_15 => 68,
            Interrupt::GPIO3_COMBINED_16_31 => 69,
            Interrupt::GPIO4_COMBINED_0_15 => 70,
            Interrupt::GPIO4_COMBINED_16_31 => 71,
            Interrupt::GPIO5_COMBINED_0_15 => 72,
            Interrupt::GPIO5_COMBINED_16_31 => 73,
            Interrupt::WDOG1 => 78,
            Interrupt::WDOG2 => 79,
            Interrupt::PWM1 => 81,
            Interrupt::PWM2 => 82,
            Interrupt::PWM3 => 83,
            Interrupt::PWM4 => 84,
            Interrupt::CCM_IRQ1 => 85,
            Interrupt::CCM_IRQ2 => 86,
            Interrupt::GPC => 87,
            Interrupt::SRC => 89,
            Interrupt::I2S56 => 90,
            Interrupt::SRC_COMBINED => 94,
            Interrupt::I2S1 => 95,
            Interrupt::I2S2 => 96,
            Interrupt::MU_M4 => 97,
            Interrupt::SDMA2 => 103,
            Interrupt::ENET => 120,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "AIPSTZ Control Registers"]
pub struct AIPSTZ {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AIPSTZ {}
impl AIPSTZ {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aipstz::RegisterBlock {
        805306368 as *const _
    }
}
impl Deref for AIPSTZ {
    type Target = aipstz::RegisterBlock;
    fn deref(&self) -> &aipstz::RegisterBlock {
        unsafe { &*AIPSTZ::ptr() }
    }
}
#[doc = "AIPSTZ Control Registers"]
pub mod aipstz;
#[doc = "SAI Registers"]
pub struct I2S1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S1 {}
impl I2S1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2s1::RegisterBlock {
        805371904 as *const _
    }
}
impl Deref for I2S1 {
    type Target = i2s1::RegisterBlock;
    fn deref(&self) -> &i2s1::RegisterBlock {
        unsafe { &*I2S1::ptr() }
    }
}
#[doc = "SAI Registers"]
pub mod i2s1;
#[doc = "SAI Registers"]
pub struct I2S2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S2 {}
impl I2S2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2s1::RegisterBlock {
        805437440 as *const _
    }
}
impl Deref for I2S2 {
    type Target = i2s1::RegisterBlock;
    fn deref(&self) -> &i2s1::RegisterBlock {
        unsafe { &*I2S2::ptr() }
    }
}
#[doc = "SAI Registers"]
pub struct I2S3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S3 {}
impl I2S3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2s1::RegisterBlock {
        805502976 as *const _
    }
}
impl Deref for I2S3 {
    type Target = i2s1::RegisterBlock;
    fn deref(&self) -> &i2s1::RegisterBlock {
        unsafe { &*I2S3::ptr() }
    }
}
#[doc = "SAI Registers"]
pub struct I2S5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S5 {}
impl I2S5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2s1::RegisterBlock {
        805634048 as *const _
    }
}
impl Deref for I2S5 {
    type Target = i2s1::RegisterBlock;
    fn deref(&self) -> &i2s1::RegisterBlock {
        unsafe { &*I2S5::ptr() }
    }
}
#[doc = "SAI Registers"]
pub struct I2S6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S6 {}
impl I2S6 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2s1::RegisterBlock {
        805699584 as *const _
    }
}
impl Deref for I2S6 {
    type Target = i2s1::RegisterBlock;
    fn deref(&self) -> &i2s1::RegisterBlock {
        unsafe { &*I2S6::ptr() }
    }
}
#[doc = "PDM Registers"]
pub struct PDM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDM {}
impl PDM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pdm::RegisterBlock {
        805830656 as *const _
    }
}
impl Deref for PDM {
    type Target = pdm::RegisterBlock;
    fn deref(&self) -> &pdm::RegisterBlock {
        unsafe { &*PDM::ptr() }
    }
}
#[doc = "PDM Registers"]
pub mod pdm;
#[doc = "GPIO"]
pub struct GPIO1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO1 {}
impl GPIO1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio1::RegisterBlock {
        807403520 as *const _
    }
}
impl Deref for GPIO1 {
    type Target = gpio1::RegisterBlock;
    fn deref(&self) -> &gpio1::RegisterBlock {
        unsafe { &*GPIO1::ptr() }
    }
}
#[doc = "GPIO"]
pub mod gpio1;
#[doc = "GPIO"]
pub struct GPIO2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO2 {}
impl GPIO2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio1::RegisterBlock {
        807469056 as *const _
    }
}
impl Deref for GPIO2 {
    type Target = gpio1::RegisterBlock;
    fn deref(&self) -> &gpio1::RegisterBlock {
        unsafe { &*GPIO2::ptr() }
    }
}
#[doc = "GPIO"]
pub struct GPIO3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO3 {}
impl GPIO3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio1::RegisterBlock {
        807534592 as *const _
    }
}
impl Deref for GPIO3 {
    type Target = gpio1::RegisterBlock;
    fn deref(&self) -> &gpio1::RegisterBlock {
        unsafe { &*GPIO3::ptr() }
    }
}
#[doc = "GPIO"]
pub struct GPIO4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO4 {}
impl GPIO4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio1::RegisterBlock {
        807600128 as *const _
    }
}
impl Deref for GPIO4 {
    type Target = gpio1::RegisterBlock;
    fn deref(&self) -> &gpio1::RegisterBlock {
        unsafe { &*GPIO4::ptr() }
    }
}
#[doc = "GPIO"]
pub struct GPIO5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO5 {}
impl GPIO5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio1::RegisterBlock {
        807665664 as *const _
    }
}
impl Deref for GPIO5 {
    type Target = gpio1::RegisterBlock;
    fn deref(&self) -> &gpio1::RegisterBlock {
        unsafe { &*GPIO5::ptr() }
    }
}
#[doc = "TMU"]
pub struct TMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMU {}
impl TMU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tmu::RegisterBlock {
        807796736 as *const _
    }
}
impl Deref for TMU {
    type Target = tmu::RegisterBlock;
    fn deref(&self) -> &tmu::RegisterBlock {
        unsafe { &*TMU::ptr() }
    }
}
#[doc = "TMU"]
pub mod tmu;
#[doc = "XTALOSC"]
pub struct XTALOSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for XTALOSC {}
impl XTALOSC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const xtalosc::RegisterBlock {
        807862272 as *const _
    }
}
impl Deref for XTALOSC {
    type Target = xtalosc::RegisterBlock;
    fn deref(&self) -> &xtalosc::RegisterBlock {
        unsafe { &*XTALOSC::ptr() }
    }
}
#[doc = "XTALOSC"]
pub mod xtalosc;
#[doc = "WDOG"]
pub struct WDOG1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDOG1 {}
impl WDOG1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdog1::RegisterBlock {
        807927808 as *const _
    }
}
impl Deref for WDOG1 {
    type Target = wdog1::RegisterBlock;
    fn deref(&self) -> &wdog1::RegisterBlock {
        unsafe { &*WDOG1::ptr() }
    }
}
#[doc = "WDOG"]
pub mod wdog1;
#[doc = "WDOG"]
pub struct WDOG2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDOG2 {}
impl WDOG2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdog1::RegisterBlock {
        807993344 as *const _
    }
}
impl Deref for WDOG2 {
    type Target = wdog1::RegisterBlock;
    fn deref(&self) -> &wdog1::RegisterBlock {
        unsafe { &*WDOG2::ptr() }
    }
}
#[doc = "WDOG"]
pub struct WDOG3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDOG3 {}
impl WDOG3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdog1::RegisterBlock {
        808058880 as *const _
    }
}
impl Deref for WDOG3 {
    type Target = wdog1::RegisterBlock;
    fn deref(&self) -> &wdog1::RegisterBlock {
        unsafe { &*WDOG3::ptr() }
    }
}
#[doc = "GPT"]
pub struct GPT1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPT1 {}
impl GPT1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpt1::RegisterBlock {
        808255488 as *const _
    }
}
impl Deref for GPT1 {
    type Target = gpt1::RegisterBlock;
    fn deref(&self) -> &gpt1::RegisterBlock {
        unsafe { &*GPT1::ptr() }
    }
}
#[doc = "GPT"]
pub mod gpt1;
#[doc = "GPT"]
pub struct GPT2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPT2 {}
impl GPT2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpt1::RegisterBlock {
        808321024 as *const _
    }
}
impl Deref for GPT2 {
    type Target = gpt1::RegisterBlock;
    fn deref(&self) -> &gpt1::RegisterBlock {
        unsafe { &*GPT2::ptr() }
    }
}
#[doc = "GPT"]
pub struct GPT3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPT3 {}
impl GPT3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpt1::RegisterBlock {
        808386560 as *const _
    }
}
impl Deref for GPT3 {
    type Target = gpt1::RegisterBlock;
    fn deref(&self) -> &gpt1::RegisterBlock {
        unsafe { &*GPT3::ptr() }
    }
}
#[doc = "GPT"]
pub struct GPT6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPT6 {}
impl GPT6 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpt1::RegisterBlock {
        812515328 as *const _
    }
}
impl Deref for GPT6 {
    type Target = gpt1::RegisterBlock;
    fn deref(&self) -> &gpt1::RegisterBlock {
        unsafe { &*GPT6::ptr() }
    }
}
#[doc = "GPT"]
pub struct GPT5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPT5 {}
impl GPT5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpt1::RegisterBlock {
        812580864 as *const _
    }
}
impl Deref for GPT5 {
    type Target = gpt1::RegisterBlock;
    fn deref(&self) -> &gpt1::RegisterBlock {
        unsafe { &*GPT5::ptr() }
    }
}
#[doc = "GPT"]
pub struct GPT4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPT4 {}
impl GPT4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpt1::RegisterBlock {
        812646400 as *const _
    }
}
impl Deref for GPT4 {
    type Target = gpt1::RegisterBlock;
    fn deref(&self) -> &gpt1::RegisterBlock {
        unsafe { &*GPT4::ptr() }
    }
}
#[doc = "ROMC"]
pub struct ROMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ROMC {}
impl ROMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const romc::RegisterBlock {
        808517632 as *const _
    }
}
impl Deref for ROMC {
    type Target = romc::RegisterBlock;
    fn deref(&self) -> &romc::RegisterBlock {
        unsafe { &*ROMC::ptr() }
    }
}
#[doc = "ROMC"]
pub mod romc;
#[doc = "IOMUXC"]
pub struct IOMUXC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOMUXC {}
impl IOMUXC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const iomuxc::RegisterBlock {
        808648704 as *const _
    }
}
impl Deref for IOMUXC {
    type Target = iomuxc::RegisterBlock;
    fn deref(&self) -> &iomuxc::RegisterBlock {
        unsafe { &*IOMUXC::ptr() }
    }
}
#[doc = "IOMUXC"]
pub mod iomuxc;
#[doc = "OCOTP Register Reference Index"]
pub struct OCOTP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OCOTP {}
impl OCOTP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ocotp::RegisterBlock {
        808779776 as *const _
    }
}
impl Deref for OCOTP {
    type Target = ocotp::RegisterBlock;
    fn deref(&self) -> &ocotp::RegisterBlock {
        unsafe { &*OCOTP::ptr() }
    }
}
#[doc = "OCOTP Register Reference Index"]
pub mod ocotp;
#[doc = "CCM_ANALOG"]
pub struct CCM_ANALOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCM_ANALOG {}
impl CCM_ANALOG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccm_analog::RegisterBlock {
        808845312 as *const _
    }
}
impl Deref for CCM_ANALOG {
    type Target = ccm_analog::RegisterBlock;
    fn deref(&self) -> &ccm_analog::RegisterBlock {
        unsafe { &*CCM_ANALOG::ptr() }
    }
}
#[doc = "CCM_ANALOG"]
pub mod ccm_analog;
#[doc = "CCM_UNIFIED"]
pub struct CCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCM {}
impl CCM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccm::RegisterBlock {
        808976384 as *const _
    }
}
impl Deref for CCM {
    type Target = ccm::RegisterBlock;
    fn deref(&self) -> &ccm::RegisterBlock {
        unsafe { &*CCM::ptr() }
    }
}
#[doc = "CCM_UNIFIED"]
pub mod ccm;
#[doc = "SRC"]
pub struct SRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SRC {}
impl SRC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const src::RegisterBlock {
        809041920 as *const _
    }
}
impl Deref for SRC {
    type Target = src::RegisterBlock;
    fn deref(&self) -> &src::RegisterBlock {
        unsafe { &*SRC::ptr() }
    }
}
#[doc = "SRC"]
pub mod src;
#[doc = "GPC"]
pub struct GPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPC {}
impl GPC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpc::RegisterBlock {
        809107456 as *const _
    }
}
impl Deref for GPC {
    type Target = gpc::RegisterBlock;
    fn deref(&self) -> &gpc::RegisterBlock {
        unsafe { &*GPC::ptr() }
    }
}
#[doc = "GPC"]
pub mod gpc;
#[doc = "GPC_PGC"]
pub struct GPC_PGC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPC_PGC {}
impl GPC_PGC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpc_pgc::RegisterBlock {
        809109504 as *const _
    }
}
impl Deref for GPC_PGC {
    type Target = gpc_pgc::RegisterBlock;
    fn deref(&self) -> &gpc_pgc::RegisterBlock {
        unsafe { &*GPC_PGC::ptr() }
    }
}
#[doc = "GPC_PGC"]
pub mod gpc_pgc;
#[doc = "SEMA42"]
pub struct RDC_SEMAPHORE1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RDC_SEMAPHORE1 {}
impl RDC_SEMAPHORE1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rdc_semaphore1::RegisterBlock {
        809172992 as *const _
    }
}
impl Deref for RDC_SEMAPHORE1 {
    type Target = rdc_semaphore1::RegisterBlock;
    fn deref(&self) -> &rdc_semaphore1::RegisterBlock {
        unsafe { &*RDC_SEMAPHORE1::ptr() }
    }
}
#[doc = "SEMA42"]
pub mod rdc_semaphore1;
#[doc = "SEMA42"]
pub struct RDC_SEMAPHORE2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RDC_SEMAPHORE2 {}
impl RDC_SEMAPHORE2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rdc_semaphore1::RegisterBlock {
        809238528 as *const _
    }
}
impl Deref for RDC_SEMAPHORE2 {
    type Target = rdc_semaphore1::RegisterBlock;
    fn deref(&self) -> &rdc_semaphore1::RegisterBlock {
        unsafe { &*RDC_SEMAPHORE2::ptr() }
    }
}
#[doc = "RDC"]
pub struct RDC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RDC {}
impl RDC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rdc::RegisterBlock {
        809304064 as *const _
    }
}
impl Deref for RDC {
    type Target = rdc::RegisterBlock;
    fn deref(&self) -> &rdc::RegisterBlock {
        unsafe { &*RDC::ptr() }
    }
}
#[doc = "RDC"]
pub mod rdc;
#[doc = "PWM"]
pub struct PWM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM1 {}
impl PWM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwm1::RegisterBlock {
        811991040 as *const _
    }
}
impl Deref for PWM1 {
    type Target = pwm1::RegisterBlock;
    fn deref(&self) -> &pwm1::RegisterBlock {
        unsafe { &*PWM1::ptr() }
    }
}
#[doc = "PWM"]
pub mod pwm1;
#[doc = "PWM"]
pub struct PWM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM2 {}
impl PWM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwm1::RegisterBlock {
        812056576 as *const _
    }
}
impl Deref for PWM2 {
    type Target = pwm1::RegisterBlock;
    fn deref(&self) -> &pwm1::RegisterBlock {
        unsafe { &*PWM2::ptr() }
    }
}
#[doc = "PWM"]
pub struct PWM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM3 {}
impl PWM3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwm1::RegisterBlock {
        812122112 as *const _
    }
}
impl Deref for PWM3 {
    type Target = pwm1::RegisterBlock;
    fn deref(&self) -> &pwm1::RegisterBlock {
        unsafe { &*PWM3::ptr() }
    }
}
#[doc = "PWM"]
pub struct PWM4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM4 {}
impl PWM4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwm1::RegisterBlock {
        812187648 as *const _
    }
}
impl Deref for PWM4 {
    type Target = pwm1::RegisterBlock;
    fn deref(&self) -> &pwm1::RegisterBlock {
        unsafe { &*PWM4::ptr() }
    }
}
#[doc = "ECSPI"]
pub struct ECSPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ECSPI1 {}
impl ECSPI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ecspi1::RegisterBlock {
        813826048 as *const _
    }
}
impl Deref for ECSPI1 {
    type Target = ecspi1::RegisterBlock;
    fn deref(&self) -> &ecspi1::RegisterBlock {
        unsafe { &*ECSPI1::ptr() }
    }
}
#[doc = "ECSPI"]
pub mod ecspi1;
#[doc = "ECSPI"]
pub struct ECSPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ECSPI2 {}
impl ECSPI2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ecspi1::RegisterBlock {
        813891584 as *const _
    }
}
impl Deref for ECSPI2 {
    type Target = ecspi1::RegisterBlock;
    fn deref(&self) -> &ecspi1::RegisterBlock {
        unsafe { &*ECSPI2::ptr() }
    }
}
#[doc = "ECSPI"]
pub struct ECSPI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ECSPI3 {}
impl ECSPI3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ecspi1::RegisterBlock {
        813957120 as *const _
    }
}
impl Deref for ECSPI3 {
    type Target = ecspi1::RegisterBlock;
    fn deref(&self) -> &ecspi1::RegisterBlock {
        unsafe { &*ECSPI3::ptr() }
    }
}
#[doc = "UARTv2"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart1::RegisterBlock {
        814088192 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart1::RegisterBlock;
    fn deref(&self) -> &uart1::RegisterBlock {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "UARTv2"]
pub mod uart1;
#[doc = "UARTv2"]
pub struct UART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART3 {}
impl UART3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart1::RegisterBlock {
        814219264 as *const _
    }
}
impl Deref for UART3 {
    type Target = uart1::RegisterBlock;
    fn deref(&self) -> &uart1::RegisterBlock {
        unsafe { &*UART3::ptr() }
    }
}
#[doc = "UARTv2"]
pub struct UART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART2 {}
impl UART2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart1::RegisterBlock {
        814284800 as *const _
    }
}
impl Deref for UART2 {
    type Target = uart1::RegisterBlock;
    fn deref(&self) -> &uart1::RegisterBlock {
        unsafe { &*UART2::ptr() }
    }
}
#[doc = "UARTv2"]
pub struct UART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART4 {}
impl UART4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart1::RegisterBlock {
        816185344 as *const _
    }
}
impl Deref for UART4 {
    type Target = uart1::RegisterBlock;
    fn deref(&self) -> &uart1::RegisterBlock {
        unsafe { &*UART4::ptr() }
    }
}
#[doc = "Temperature Monitor"]
pub struct SPBA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPBA1 {}
impl SPBA1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spba1::RegisterBlock {
        814678016 as *const _
    }
}
impl Deref for SPBA1 {
    type Target = spba1::RegisterBlock;
    fn deref(&self) -> &spba1::RegisterBlock {
        unsafe { &*SPBA1::ptr() }
    }
}
#[doc = "Temperature Monitor"]
pub mod spba1;
#[doc = "Temperature Monitor"]
pub struct SPBA2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPBA2 {}
impl SPBA2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spba1::RegisterBlock {
        806289408 as *const _
    }
}
impl Deref for SPBA2 {
    type Target = spba1::RegisterBlock;
    fn deref(&self) -> &spba1::RegisterBlock {
        unsafe { &*SPBA2::ptr() }
    }
}
#[doc = "I2C"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c1::RegisterBlock {
        815923200 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "I2C"]
pub mod i2c1;
#[doc = "I2C"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c1::RegisterBlock {
        815988736 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "I2C"]
pub struct I2C3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C3 {}
impl I2C3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c1::RegisterBlock {
        816054272 as *const _
    }
}
impl Deref for I2C3 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        unsafe { &*I2C3::ptr() }
    }
}
#[doc = "I2C"]
pub struct I2C4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C4 {}
impl I2C4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c1::RegisterBlock {
        816119808 as *const _
    }
}
impl Deref for I2C4 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        unsafe { &*I2C4::ptr() }
    }
}
#[doc = "MU"]
pub struct MUB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MUB {}
impl MUB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mub::RegisterBlock {
        816513024 as *const _
    }
}
impl Deref for MUB {
    type Target = mub::RegisterBlock;
    fn deref(&self) -> &mub::RegisterBlock {
        unsafe { &*MUB::ptr() }
    }
}
#[doc = "MU"]
pub mod mub;
#[doc = "IPS_Semaphores"]
pub struct SEMA4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEMA4 {}
impl SEMA4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sema4::RegisterBlock {
        816578560 as *const _
    }
}
impl Deref for SEMA4 {
    type Target = sema4::RegisterBlock;
    fn deref(&self) -> &sema4::RegisterBlock {
        unsafe { &*SEMA4::ptr() }
    }
}
#[doc = "IPS_Semaphores"]
pub mod sema4;
#[doc = "uSDHC"]
pub struct USDHC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USDHC1 {}
impl USDHC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const u_sdhc1::RegisterBlock {
        817102848 as *const _
    }
}
impl Deref for USDHC1 {
    type Target = u_sdhc1::RegisterBlock;
    fn deref(&self) -> &u_sdhc1::RegisterBlock {
        unsafe { &*USDHC1::ptr() }
    }
}
#[doc = "uSDHC"]
pub mod u_sdhc1;
#[doc = "uSDHC"]
pub struct USDHC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USDHC2 {}
impl USDHC2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const u_sdhc1::RegisterBlock {
        817168384 as *const _
    }
}
impl Deref for USDHC2 {
    type Target = u_sdhc1::RegisterBlock;
    fn deref(&self) -> &u_sdhc1::RegisterBlock {
        unsafe { &*USDHC2::ptr() }
    }
}
#[doc = "uSDHC"]
pub struct USDHC3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USDHC3 {}
impl USDHC3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const u_sdhc1::RegisterBlock {
        817233920 as *const _
    }
}
impl Deref for USDHC3 {
    type Target = u_sdhc1::RegisterBlock;
    fn deref(&self) -> &u_sdhc1::RegisterBlock {
        unsafe { &*USDHC3::ptr() }
    }
}
#[doc = "SDMA"]
pub struct SDMAARM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDMAARM1 {}
impl SDMAARM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sdmaarm1::RegisterBlock {
        817692672 as *const _
    }
}
impl Deref for SDMAARM1 {
    type Target = sdmaarm1::RegisterBlock;
    fn deref(&self) -> &sdmaarm1::RegisterBlock {
        unsafe { &*SDMAARM1::ptr() }
    }
}
#[doc = "SDMA"]
pub mod sdmaarm1;
#[doc = "SDMA"]
pub struct SDMAARM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDMAARM3 {}
impl SDMAARM3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sdmaarm1::RegisterBlock {
        808124416 as *const _
    }
}
impl Deref for SDMAARM3 {
    type Target = sdmaarm1::RegisterBlock;
    fn deref(&self) -> &sdmaarm1::RegisterBlock {
        unsafe { &*SDMAARM3::ptr() }
    }
}
#[doc = "SDMA"]
pub struct SDMAARM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDMAARM2 {}
impl SDMAARM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sdmaarm1::RegisterBlock {
        808189952 as *const _
    }
}
impl Deref for SDMAARM2 {
    type Target = sdmaarm1::RegisterBlock;
    fn deref(&self) -> &sdmaarm1::RegisterBlock {
        unsafe { &*SDMAARM2::ptr() }
    }
}
#[doc = "Ethernet MAC-NET Core"]
pub struct ENET {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ENET {}
impl ENET {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const enet::RegisterBlock {
        817758208 as *const _
    }
}
impl Deref for ENET {
    type Target = enet::RegisterBlock;
    fn deref(&self) -> &enet::RegisterBlock {
        unsafe { &*ENET::ptr() }
    }
}
#[doc = "Ethernet MAC-NET Core"]
pub mod enet;
#[doc = "LCDIF Register Reference Index"]
pub struct LCDIF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LCDIF {}
impl LCDIF {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lcdif::RegisterBlock {
        853540864 as *const _
    }
}
impl Deref for LCDIF {
    type Target = lcdif::RegisterBlock;
    fn deref(&self) -> &lcdif::RegisterBlock {
        unsafe { &*LCDIF::ptr() }
    }
}
#[doc = "LCDIF Register Reference Index"]
pub mod lcdif;
#[doc = "CSI"]
pub struct CSI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CSI {}
impl CSI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const csi::RegisterBlock {
        853671936 as *const _
    }
}
impl Deref for CSI {
    type Target = csi::RegisterBlock;
    fn deref(&self) -> &csi::RegisterBlock {
        unsafe { &*CSI::ptr() }
    }
}
#[doc = "CSI"]
pub mod csi;
#[doc = "USB"]
pub struct USBNC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBNC {}
impl USBNC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usbnc::RegisterBlock {
        853803008 as *const _
    }
}
impl Deref for USBNC {
    type Target = usbnc::RegisterBlock;
    fn deref(&self) -> &usbnc::RegisterBlock {
        unsafe { &*USBNC::ptr() }
    }
}
#[doc = "USB"]
pub mod usbnc;
#[doc = "USB"]
pub struct USB_OTG1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB_OTG1 {}
impl USB_OTG1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb_otg1::RegisterBlock {
        853803008 as *const _
    }
}
impl Deref for USB_OTG1 {
    type Target = usb_otg1::RegisterBlock;
    fn deref(&self) -> &usb_otg1::RegisterBlock {
        unsafe { &*USB_OTG1::ptr() }
    }
}
#[doc = "USB"]
pub mod usb_otg1;
#[doc = "USB"]
pub struct USB_OTG2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB_OTG2 {}
impl USB_OTG2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb_otg1::RegisterBlock {
        853868544 as *const _
    }
}
impl Deref for USB_OTG2 {
    type Target = usb_otg1::RegisterBlock;
    fn deref(&self) -> &usb_otg1::RegisterBlock {
        unsafe { &*USB_OTG2::ptr() }
    }
}
#[doc = "APBH Register Reference Index"]
pub struct APBH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for APBH {}
impl APBH {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const apbh::RegisterBlock {
        855638016 as *const _
    }
}
impl Deref for APBH {
    type Target = apbh::RegisterBlock;
    fn deref(&self) -> &apbh::RegisterBlock {
        unsafe { &*APBH::ptr() }
    }
}
#[doc = "APBH Register Reference Index"]
pub mod apbh;
#[doc = "GPMI"]
pub struct GPMI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPMI {}
impl GPMI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpmi::RegisterBlock {
        855646208 as *const _
    }
}
impl Deref for GPMI {
    type Target = gpmi::RegisterBlock;
    fn deref(&self) -> &gpmi::RegisterBlock {
        unsafe { &*GPMI::ptr() }
    }
}
#[doc = "GPMI"]
pub mod gpmi;
#[doc = "BCH Register Reference Index"]
pub struct BCH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BCH {}
impl BCH {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const bch::RegisterBlock {
        855654400 as *const _
    }
}
impl Deref for BCH {
    type Target = bch::RegisterBlock;
    fn deref(&self) -> &bch::RegisterBlock {
        unsafe { &*BCH::ptr() }
    }
}
#[doc = "BCH Register Reference Index"]
pub mod bch;
#[doc = "Core Platform Miscellaneous Control Module"]
pub struct MCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCM {}
impl MCM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mcm::RegisterBlock {
        3758620672 as *const _
    }
}
impl Deref for MCM {
    type Target = mcm::RegisterBlock;
    fn deref(&self) -> &mcm::RegisterBlock {
        unsafe { &*MCM::ptr() }
    }
}
#[doc = "Core Platform Miscellaneous Control Module"]
pub mod mcm;
#[doc = "Local Memory Controller"]
pub struct LMEM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LMEM {}
impl LMEM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lmem::RegisterBlock {
        3758628864 as *const _
    }
}
impl Deref for LMEM {
    type Target = lmem::RegisterBlock;
    fn deref(&self) -> &lmem::RegisterBlock {
        unsafe { &*LMEM::ptr() }
    }
}
#[doc = "Local Memory Controller"]
pub mod lmem;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "AIPSTZ"]
    pub AIPSTZ: AIPSTZ,
    #[doc = "I2S1"]
    pub I2S1: I2S1,
    #[doc = "I2S2"]
    pub I2S2: I2S2,
    #[doc = "I2S3"]
    pub I2S3: I2S3,
    #[doc = "I2S5"]
    pub I2S5: I2S5,
    #[doc = "I2S6"]
    pub I2S6: I2S6,
    #[doc = "PDM"]
    pub PDM: PDM,
    #[doc = "GPIO1"]
    pub GPIO1: GPIO1,
    #[doc = "GPIO2"]
    pub GPIO2: GPIO2,
    #[doc = "GPIO3"]
    pub GPIO3: GPIO3,
    #[doc = "GPIO4"]
    pub GPIO4: GPIO4,
    #[doc = "GPIO5"]
    pub GPIO5: GPIO5,
    #[doc = "TMU"]
    pub TMU: TMU,
    #[doc = "XTALOSC"]
    pub XTALOSC: XTALOSC,
    #[doc = "WDOG1"]
    pub WDOG1: WDOG1,
    #[doc = "WDOG2"]
    pub WDOG2: WDOG2,
    #[doc = "WDOG3"]
    pub WDOG3: WDOG3,
    #[doc = "GPT1"]
    pub GPT1: GPT1,
    #[doc = "GPT2"]
    pub GPT2: GPT2,
    #[doc = "GPT3"]
    pub GPT3: GPT3,
    #[doc = "GPT6"]
    pub GPT6: GPT6,
    #[doc = "GPT5"]
    pub GPT5: GPT5,
    #[doc = "GPT4"]
    pub GPT4: GPT4,
    #[doc = "ROMC"]
    pub ROMC: ROMC,
    #[doc = "IOMUXC"]
    pub IOMUXC: IOMUXC,
    #[doc = "OCOTP"]
    pub OCOTP: OCOTP,
    #[doc = "CCM_ANALOG"]
    pub CCM_ANALOG: CCM_ANALOG,
    #[doc = "CCM"]
    pub CCM: CCM,
    #[doc = "SRC"]
    pub SRC: SRC,
    #[doc = "GPC"]
    pub GPC: GPC,
    #[doc = "GPC_PGC"]
    pub GPC_PGC: GPC_PGC,
    #[doc = "RDC_SEMAPHORE1"]
    pub RDC_SEMAPHORE1: RDC_SEMAPHORE1,
    #[doc = "RDC_SEMAPHORE2"]
    pub RDC_SEMAPHORE2: RDC_SEMAPHORE2,
    #[doc = "RDC"]
    pub RDC: RDC,
    #[doc = "PWM1"]
    pub PWM1: PWM1,
    #[doc = "PWM2"]
    pub PWM2: PWM2,
    #[doc = "PWM3"]
    pub PWM3: PWM3,
    #[doc = "PWM4"]
    pub PWM4: PWM4,
    #[doc = "ECSPI1"]
    pub ECSPI1: ECSPI1,
    #[doc = "ECSPI2"]
    pub ECSPI2: ECSPI2,
    #[doc = "ECSPI3"]
    pub ECSPI3: ECSPI3,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "UART3"]
    pub UART3: UART3,
    #[doc = "UART2"]
    pub UART2: UART2,
    #[doc = "UART4"]
    pub UART4: UART4,
    #[doc = "SPBA1"]
    pub SPBA1: SPBA1,
    #[doc = "SPBA2"]
    pub SPBA2: SPBA2,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "I2C3"]
    pub I2C3: I2C3,
    #[doc = "I2C4"]
    pub I2C4: I2C4,
    #[doc = "MUB"]
    pub MUB: MUB,
    #[doc = "SEMA4"]
    pub SEMA4: SEMA4,
    #[doc = "USDHC1"]
    pub USDHC1: USDHC1,
    #[doc = "USDHC2"]
    pub USDHC2: USDHC2,
    #[doc = "USDHC3"]
    pub USDHC3: USDHC3,
    #[doc = "SDMAARM1"]
    pub SDMAARM1: SDMAARM1,
    #[doc = "SDMAARM3"]
    pub SDMAARM3: SDMAARM3,
    #[doc = "SDMAARM2"]
    pub SDMAARM2: SDMAARM2,
    #[doc = "ENET"]
    pub ENET: ENET,
    #[doc = "LCDIF"]
    pub LCDIF: LCDIF,
    #[doc = "CSI"]
    pub CSI: CSI,
    #[doc = "USBNC"]
    pub USBNC: USBNC,
    #[doc = "USB_OTG1"]
    pub USB_OTG1: USB_OTG1,
    #[doc = "USB_OTG2"]
    pub USB_OTG2: USB_OTG2,
    #[doc = "APBH"]
    pub APBH: APBH,
    #[doc = "GPMI"]
    pub GPMI: GPMI,
    #[doc = "BCH"]
    pub BCH: BCH,
    #[doc = "MCM"]
    pub MCM: MCM,
    #[doc = "LMEM"]
    pub LMEM: LMEM,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            AIPSTZ: AIPSTZ {
                _marker: PhantomData,
            },
            I2S1: I2S1 {
                _marker: PhantomData,
            },
            I2S2: I2S2 {
                _marker: PhantomData,
            },
            I2S3: I2S3 {
                _marker: PhantomData,
            },
            I2S5: I2S5 {
                _marker: PhantomData,
            },
            I2S6: I2S6 {
                _marker: PhantomData,
            },
            PDM: PDM {
                _marker: PhantomData,
            },
            GPIO1: GPIO1 {
                _marker: PhantomData,
            },
            GPIO2: GPIO2 {
                _marker: PhantomData,
            },
            GPIO3: GPIO3 {
                _marker: PhantomData,
            },
            GPIO4: GPIO4 {
                _marker: PhantomData,
            },
            GPIO5: GPIO5 {
                _marker: PhantomData,
            },
            TMU: TMU {
                _marker: PhantomData,
            },
            XTALOSC: XTALOSC {
                _marker: PhantomData,
            },
            WDOG1: WDOG1 {
                _marker: PhantomData,
            },
            WDOG2: WDOG2 {
                _marker: PhantomData,
            },
            WDOG3: WDOG3 {
                _marker: PhantomData,
            },
            GPT1: GPT1 {
                _marker: PhantomData,
            },
            GPT2: GPT2 {
                _marker: PhantomData,
            },
            GPT3: GPT3 {
                _marker: PhantomData,
            },
            GPT6: GPT6 {
                _marker: PhantomData,
            },
            GPT5: GPT5 {
                _marker: PhantomData,
            },
            GPT4: GPT4 {
                _marker: PhantomData,
            },
            ROMC: ROMC {
                _marker: PhantomData,
            },
            IOMUXC: IOMUXC {
                _marker: PhantomData,
            },
            OCOTP: OCOTP {
                _marker: PhantomData,
            },
            CCM_ANALOG: CCM_ANALOG {
                _marker: PhantomData,
            },
            CCM: CCM {
                _marker: PhantomData,
            },
            SRC: SRC {
                _marker: PhantomData,
            },
            GPC: GPC {
                _marker: PhantomData,
            },
            GPC_PGC: GPC_PGC {
                _marker: PhantomData,
            },
            RDC_SEMAPHORE1: RDC_SEMAPHORE1 {
                _marker: PhantomData,
            },
            RDC_SEMAPHORE2: RDC_SEMAPHORE2 {
                _marker: PhantomData,
            },
            RDC: RDC {
                _marker: PhantomData,
            },
            PWM1: PWM1 {
                _marker: PhantomData,
            },
            PWM2: PWM2 {
                _marker: PhantomData,
            },
            PWM3: PWM3 {
                _marker: PhantomData,
            },
            PWM4: PWM4 {
                _marker: PhantomData,
            },
            ECSPI1: ECSPI1 {
                _marker: PhantomData,
            },
            ECSPI2: ECSPI2 {
                _marker: PhantomData,
            },
            ECSPI3: ECSPI3 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            UART3: UART3 {
                _marker: PhantomData,
            },
            UART2: UART2 {
                _marker: PhantomData,
            },
            UART4: UART4 {
                _marker: PhantomData,
            },
            SPBA1: SPBA1 {
                _marker: PhantomData,
            },
            SPBA2: SPBA2 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            I2C3: I2C3 {
                _marker: PhantomData,
            },
            I2C4: I2C4 {
                _marker: PhantomData,
            },
            MUB: MUB {
                _marker: PhantomData,
            },
            SEMA4: SEMA4 {
                _marker: PhantomData,
            },
            USDHC1: USDHC1 {
                _marker: PhantomData,
            },
            USDHC2: USDHC2 {
                _marker: PhantomData,
            },
            USDHC3: USDHC3 {
                _marker: PhantomData,
            },
            SDMAARM1: SDMAARM1 {
                _marker: PhantomData,
            },
            SDMAARM3: SDMAARM3 {
                _marker: PhantomData,
            },
            SDMAARM2: SDMAARM2 {
                _marker: PhantomData,
            },
            ENET: ENET {
                _marker: PhantomData,
            },
            LCDIF: LCDIF {
                _marker: PhantomData,
            },
            CSI: CSI {
                _marker: PhantomData,
            },
            USBNC: USBNC {
                _marker: PhantomData,
            },
            USB_OTG1: USB_OTG1 {
                _marker: PhantomData,
            },
            USB_OTG2: USB_OTG2 {
                _marker: PhantomData,
            },
            APBH: APBH {
                _marker: PhantomData,
            },
            GPMI: GPMI {
                _marker: PhantomData,
            },
            BCH: BCH {
                _marker: PhantomData,
            },
            MCM: MCM {
                _marker: PhantomData,
            },
            LMEM: LMEM {
                _marker: PhantomData,
            },
        }
    }
}
