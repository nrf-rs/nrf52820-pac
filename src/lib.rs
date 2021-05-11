#![doc = "Peripheral access API for NRF52820 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn POWER_CLOCK();
    fn RADIO();
    fn UARTE0_UART0();
    fn SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0();
    fn SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1();
    fn GPIOTE();
    fn TIMER0();
    fn TIMER1();
    fn TIMER2();
    fn RTC0();
    fn TEMP();
    fn RNG();
    fn ECB();
    fn CCM_AAR();
    fn WDT();
    fn RTC1();
    fn QDEC();
    fn COMP();
    fn SWI0_EGU0();
    fn SWI1_EGU1();
    fn SWI2_EGU2();
    fn SWI3_EGU3();
    fn SWI4_EGU4();
    fn SWI5_EGU5();
    fn TIMER3();
    fn USBD();
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
pub static __INTERRUPTS: [Vector; 40] = [
    Vector {
        _handler: POWER_CLOCK,
    },
    Vector { _handler: RADIO },
    Vector {
        _handler: UARTE0_UART0,
    },
    Vector {
        _handler: SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0,
    },
    Vector {
        _handler: SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1,
    },
    Vector { _reserved: 0 },
    Vector { _handler: GPIOTE },
    Vector { _reserved: 0 },
    Vector { _handler: TIMER0 },
    Vector { _handler: TIMER1 },
    Vector { _handler: TIMER2 },
    Vector { _handler: RTC0 },
    Vector { _handler: TEMP },
    Vector { _handler: RNG },
    Vector { _handler: ECB },
    Vector { _handler: CCM_AAR },
    Vector { _handler: WDT },
    Vector { _handler: RTC1 },
    Vector { _handler: QDEC },
    Vector { _handler: COMP },
    Vector {
        _handler: SWI0_EGU0,
    },
    Vector {
        _handler: SWI1_EGU1,
    },
    Vector {
        _handler: SWI2_EGU2,
    },
    Vector {
        _handler: SWI3_EGU3,
    },
    Vector {
        _handler: SWI4_EGU4,
    },
    Vector {
        _handler: SWI5_EGU5,
    },
    Vector { _handler: TIMER3 },
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
    Vector { _handler: USBD },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - POWER_CLOCK"]
    POWER_CLOCK = 0,
    #[doc = "1 - RADIO"]
    RADIO = 1,
    #[doc = "2 - UARTE0_UART0"]
    UARTE0_UART0 = 2,
    #[doc = "3 - SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0"]
    SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 = 3,
    #[doc = "4 - SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1"]
    SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1 = 4,
    #[doc = "6 - GPIOTE"]
    GPIOTE = 6,
    #[doc = "8 - TIMER0"]
    TIMER0 = 8,
    #[doc = "9 - TIMER1"]
    TIMER1 = 9,
    #[doc = "10 - TIMER2"]
    TIMER2 = 10,
    #[doc = "11 - RTC0"]
    RTC0 = 11,
    #[doc = "12 - TEMP"]
    TEMP = 12,
    #[doc = "13 - RNG"]
    RNG = 13,
    #[doc = "14 - ECB"]
    ECB = 14,
    #[doc = "15 - CCM_AAR"]
    CCM_AAR = 15,
    #[doc = "16 - WDT"]
    WDT = 16,
    #[doc = "17 - RTC1"]
    RTC1 = 17,
    #[doc = "18 - QDEC"]
    QDEC = 18,
    #[doc = "19 - COMP"]
    COMP = 19,
    #[doc = "20 - SWI0_EGU0"]
    SWI0_EGU0 = 20,
    #[doc = "21 - SWI1_EGU1"]
    SWI1_EGU1 = 21,
    #[doc = "22 - SWI2_EGU2"]
    SWI2_EGU2 = 22,
    #[doc = "23 - SWI3_EGU3"]
    SWI3_EGU3 = 23,
    #[doc = "24 - SWI4_EGU4"]
    SWI4_EGU4 = 24,
    #[doc = "25 - SWI5_EGU5"]
    SWI5_EGU5 = 25,
    #[doc = "26 - TIMER3"]
    TIMER3 = 26,
    #[doc = "39 - USBD"]
    USBD = 39,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Factory information configuration registers"]
pub struct FICR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FICR {}
impl FICR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ficr::RegisterBlock {
        0x1000_0000 as *const _
    }
}
impl Deref for FICR {
    type Target = ficr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FICR::ptr() }
    }
}
#[doc = "Factory information configuration registers"]
pub mod ficr;
#[doc = "User information configuration registers"]
pub struct UICR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UICR {}
impl UICR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uicr::RegisterBlock {
        0x1000_1000 as *const _
    }
}
impl Deref for UICR {
    type Target = uicr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UICR::ptr() }
    }
}
#[doc = "User information configuration registers"]
pub mod uicr;
#[doc = "Access Port Protection"]
pub struct APPROTECT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for APPROTECT {}
impl APPROTECT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const approtect::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for APPROTECT {
    type Target = approtect::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*APPROTECT::ptr() }
    }
}
#[doc = "Access Port Protection"]
pub mod approtect;
#[doc = "Clock control"]
pub struct CLOCK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CLOCK {}
impl CLOCK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const clock::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for CLOCK {
    type Target = clock::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CLOCK::ptr() }
    }
}
#[doc = "Clock control"]
pub mod clock;
#[doc = "Power control"]
pub struct POWER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for POWER {}
impl POWER {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const power::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for POWER {
    type Target = power::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*POWER::ptr() }
    }
}
#[doc = "Power control"]
pub mod power;
#[doc = "GPIO Port 1"]
pub struct P0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P0 {}
impl P0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const p0::RegisterBlock {
        0x5000_0000 as *const _
    }
}
impl Deref for P0 {
    type Target = p0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*P0::ptr() }
    }
}
#[doc = "GPIO Port 1"]
pub mod p0;
#[doc = "2.4 GHz radio"]
pub struct RADIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RADIO {}
impl RADIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const radio::RegisterBlock {
        0x4000_1000 as *const _
    }
}
impl Deref for RADIO {
    type Target = radio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RADIO::ptr() }
    }
}
#[doc = "2.4 GHz radio"]
pub mod radio;
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4000_2000 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub mod uart0;
#[doc = "UART with EasyDMA"]
pub struct UARTE0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UARTE0 {}
impl UARTE0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uarte0::RegisterBlock {
        0x4000_2000 as *const _
    }
}
impl Deref for UARTE0 {
    type Target = uarte0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UARTE0::ptr() }
    }
}
#[doc = "UART with EasyDMA"]
pub mod uarte0;
#[doc = "Serial Peripheral Interface 0"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4000_3000 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "Serial Peripheral Interface 0"]
pub mod spi0;
#[doc = "Serial Peripheral Interface Master with EasyDMA 0"]
pub struct SPIM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIM0 {}
impl SPIM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spim0::RegisterBlock {
        0x4000_3000 as *const _
    }
}
impl Deref for SPIM0 {
    type Target = spim0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPIM0::ptr() }
    }
}
#[doc = "Serial Peripheral Interface Master with EasyDMA 0"]
pub mod spim0;
#[doc = "SPI Slave 0"]
pub struct SPIS0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIS0 {}
impl SPIS0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spis0::RegisterBlock {
        0x4000_3000 as *const _
    }
}
impl Deref for SPIS0 {
    type Target = spis0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPIS0::ptr() }
    }
}
#[doc = "SPI Slave 0"]
pub mod spis0;
#[doc = "I2C compatible Two-Wire Interface 0"]
pub struct TWI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWI0 {}
impl TWI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twi0::RegisterBlock {
        0x4000_3000 as *const _
    }
}
impl Deref for TWI0 {
    type Target = twi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWI0::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Interface 0"]
pub mod twi0;
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 0"]
pub struct TWIM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIM0 {}
impl TWIM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twim0::RegisterBlock {
        0x4000_3000 as *const _
    }
}
impl Deref for TWIM0 {
    type Target = twim0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWIM0::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 0"]
pub mod twim0;
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 0"]
pub struct TWIS0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIS0 {}
impl TWIS0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twis0::RegisterBlock {
        0x4000_3000 as *const _
    }
}
impl Deref for TWIS0 {
    type Target = twis0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWIS0::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 0"]
pub mod twis0;
#[doc = "Serial Peripheral Interface 1"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Serial Peripheral Interface Master with EasyDMA 1"]
pub struct SPIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIM1 {}
impl SPIM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spim0::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for SPIM1 {
    type Target = spim0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPIM1::ptr() }
    }
}
#[doc = "SPI Slave 1"]
pub struct SPIS1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIS1 {}
impl SPIS1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spis0::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for SPIS1 {
    type Target = spis0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPIS1::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Interface 1"]
pub struct TWI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWI1 {}
impl TWI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twi0::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for TWI1 {
    type Target = twi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWI1::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 1"]
pub struct TWIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIM1 {}
impl TWIM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twim0::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for TWIM1 {
    type Target = twim0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWIM1::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 1"]
pub struct TWIS1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIS1 {}
impl TWIS1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twis0::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for TWIS1 {
    type Target = twis0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWIS1::ptr() }
    }
}
#[doc = "GPIO Tasks and Events"]
pub struct GPIOTE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOTE {}
impl GPIOTE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiote::RegisterBlock {
        0x4000_6000 as *const _
    }
}
impl Deref for GPIOTE {
    type Target = gpiote::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOTE::ptr() }
    }
}
#[doc = "GPIO Tasks and Events"]
pub mod gpiote;
#[doc = "Timer/Counter 0"]
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0 {}
impl TIMER0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER0::ptr() }
    }
}
#[doc = "Timer/Counter 0"]
pub mod timer0;
#[doc = "Timer/Counter 1"]
pub struct TIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1 {}
impl TIMER1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0x4000_9000 as *const _
    }
}
impl Deref for TIMER1 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER1::ptr() }
    }
}
#[doc = "Timer/Counter 2"]
pub struct TIMER2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER2 {}
impl TIMER2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0x4000_a000 as *const _
    }
}
impl Deref for TIMER2 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER2::ptr() }
    }
}
#[doc = "Real time counter 0"]
pub struct RTC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC0 {}
impl RTC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc0::RegisterBlock {
        0x4000_b000 as *const _
    }
}
impl Deref for RTC0 {
    type Target = rtc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC0::ptr() }
    }
}
#[doc = "Real time counter 0"]
pub mod rtc0;
#[doc = "Temperature Sensor"]
pub struct TEMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TEMP {}
impl TEMP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const temp::RegisterBlock {
        0x4000_c000 as *const _
    }
}
impl Deref for TEMP {
    type Target = temp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TEMP::ptr() }
    }
}
#[doc = "Temperature Sensor"]
pub mod temp;
#[doc = "Random Number Generator"]
pub struct RNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG {}
impl RNG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rng::RegisterBlock {
        0x4000_d000 as *const _
    }
}
impl Deref for RNG {
    type Target = rng::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RNG::ptr() }
    }
}
#[doc = "Random Number Generator"]
pub mod rng;
#[doc = "AES ECB Mode Encryption"]
pub struct ECB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ECB {}
impl ECB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ecb::RegisterBlock {
        0x4000_e000 as *const _
    }
}
impl Deref for ECB {
    type Target = ecb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ECB::ptr() }
    }
}
#[doc = "AES ECB Mode Encryption"]
pub mod ecb;
#[doc = "Accelerated Address Resolver"]
pub struct AAR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AAR {}
impl AAR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aar::RegisterBlock {
        0x4000_f000 as *const _
    }
}
impl Deref for AAR {
    type Target = aar::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AAR::ptr() }
    }
}
#[doc = "Accelerated Address Resolver"]
pub mod aar;
#[doc = "AES CCM Mode Encryption"]
pub struct CCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCM {}
impl CCM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccm::RegisterBlock {
        0x4000_f000 as *const _
    }
}
impl Deref for CCM {
    type Target = ccm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCM::ptr() }
    }
}
#[doc = "AES CCM Mode Encryption"]
pub mod ccm;
#[doc = "Watchdog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod wdt;
#[doc = "Real time counter 1"]
pub struct RTC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC1 {}
impl RTC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc0::RegisterBlock {
        0x4001_1000 as *const _
    }
}
impl Deref for RTC1 {
    type Target = rtc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC1::ptr() }
    }
}
#[doc = "Quadrature Decoder"]
pub struct QDEC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QDEC {}
impl QDEC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const qdec::RegisterBlock {
        0x4001_2000 as *const _
    }
}
impl Deref for QDEC {
    type Target = qdec::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*QDEC::ptr() }
    }
}
#[doc = "Quadrature Decoder"]
pub mod qdec;
#[doc = "Comparator"]
pub struct COMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMP {}
impl COMP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const comp::RegisterBlock {
        0x4001_3000 as *const _
    }
}
impl Deref for COMP {
    type Target = comp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*COMP::ptr() }
    }
}
#[doc = "Comparator"]
pub mod comp;
#[doc = "Event generator unit 0"]
pub struct EGU0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU0 {}
impl EGU0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const egu0::RegisterBlock {
        0x4001_4000 as *const _
    }
}
impl Deref for EGU0 {
    type Target = egu0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EGU0::ptr() }
    }
}
#[doc = "Event generator unit 0"]
pub mod egu0;
#[doc = "Software interrupt 0"]
pub struct SWI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWI0 {}
impl SWI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const swi0::RegisterBlock {
        0x4001_4000 as *const _
    }
}
impl Deref for SWI0 {
    type Target = swi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SWI0::ptr() }
    }
}
#[doc = "Software interrupt 0"]
pub mod swi0;
#[doc = "Event generator unit 1"]
pub struct EGU1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU1 {}
impl EGU1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const egu0::RegisterBlock {
        0x4001_5000 as *const _
    }
}
impl Deref for EGU1 {
    type Target = egu0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EGU1::ptr() }
    }
}
#[doc = "Software interrupt 1"]
pub struct SWI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWI1 {}
impl SWI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const swi0::RegisterBlock {
        0x4001_5000 as *const _
    }
}
impl Deref for SWI1 {
    type Target = swi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SWI1::ptr() }
    }
}
#[doc = "Event generator unit 2"]
pub struct EGU2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU2 {}
impl EGU2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const egu0::RegisterBlock {
        0x4001_6000 as *const _
    }
}
impl Deref for EGU2 {
    type Target = egu0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EGU2::ptr() }
    }
}
#[doc = "Software interrupt 2"]
pub struct SWI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWI2 {}
impl SWI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const swi0::RegisterBlock {
        0x4001_6000 as *const _
    }
}
impl Deref for SWI2 {
    type Target = swi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SWI2::ptr() }
    }
}
#[doc = "Event generator unit 3"]
pub struct EGU3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU3 {}
impl EGU3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const egu0::RegisterBlock {
        0x4001_7000 as *const _
    }
}
impl Deref for EGU3 {
    type Target = egu0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EGU3::ptr() }
    }
}
#[doc = "Software interrupt 3"]
pub struct SWI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWI3 {}
impl SWI3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const swi0::RegisterBlock {
        0x4001_7000 as *const _
    }
}
impl Deref for SWI3 {
    type Target = swi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SWI3::ptr() }
    }
}
#[doc = "Event generator unit 4"]
pub struct EGU4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU4 {}
impl EGU4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const egu0::RegisterBlock {
        0x4001_8000 as *const _
    }
}
impl Deref for EGU4 {
    type Target = egu0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EGU4::ptr() }
    }
}
#[doc = "Software interrupt 4"]
pub struct SWI4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWI4 {}
impl SWI4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const swi0::RegisterBlock {
        0x4001_8000 as *const _
    }
}
impl Deref for SWI4 {
    type Target = swi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SWI4::ptr() }
    }
}
#[doc = "Event generator unit 5"]
pub struct EGU5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU5 {}
impl EGU5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const egu0::RegisterBlock {
        0x4001_9000 as *const _
    }
}
impl Deref for EGU5 {
    type Target = egu0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EGU5::ptr() }
    }
}
#[doc = "Software interrupt 5"]
pub struct SWI5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWI5 {}
impl SWI5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const swi0::RegisterBlock {
        0x4001_9000 as *const _
    }
}
impl Deref for SWI5 {
    type Target = swi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SWI5::ptr() }
    }
}
#[doc = "Timer/Counter 3"]
pub struct TIMER3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER3 {}
impl TIMER3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0x4001_a000 as *const _
    }
}
impl Deref for TIMER3 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER3::ptr() }
    }
}
#[doc = "Access control lists"]
pub struct ACL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACL {}
impl ACL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const acl::RegisterBlock {
        0x4001_e000 as *const _
    }
}
impl Deref for ACL {
    type Target = acl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ACL::ptr() }
    }
}
#[doc = "Access control lists"]
pub mod acl;
#[doc = "Non Volatile Memory Controller"]
pub struct NVMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVMC {}
impl NVMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nvmc::RegisterBlock {
        0x4001_e000 as *const _
    }
}
impl Deref for NVMC {
    type Target = nvmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*NVMC::ptr() }
    }
}
#[doc = "Non Volatile Memory Controller"]
pub mod nvmc;
#[doc = "Programmable Peripheral Interconnect"]
pub struct PPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PPI {}
impl PPI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ppi::RegisterBlock {
        0x4001_f000 as *const _
    }
}
impl Deref for PPI {
    type Target = ppi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PPI::ptr() }
    }
}
#[doc = "Programmable Peripheral Interconnect"]
pub mod ppi;
#[doc = "Universal serial bus device"]
pub struct USBD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBD {}
impl USBD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbd::RegisterBlock {
        0x4002_7000 as *const _
    }
}
impl Deref for USBD {
    type Target = usbd::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBD::ptr() }
    }
}
#[doc = "Universal serial bus device"]
pub mod usbd;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "FICR"]
    pub FICR: FICR,
    #[doc = "UICR"]
    pub UICR: UICR,
    #[doc = "APPROTECT"]
    pub APPROTECT: APPROTECT,
    #[doc = "CLOCK"]
    pub CLOCK: CLOCK,
    #[doc = "POWER"]
    pub POWER: POWER,
    #[doc = "P0"]
    pub P0: P0,
    #[doc = "RADIO"]
    pub RADIO: RADIO,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UARTE0"]
    pub UARTE0: UARTE0,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPIM0"]
    pub SPIM0: SPIM0,
    #[doc = "SPIS0"]
    pub SPIS0: SPIS0,
    #[doc = "TWI0"]
    pub TWI0: TWI0,
    #[doc = "TWIM0"]
    pub TWIM0: TWIM0,
    #[doc = "TWIS0"]
    pub TWIS0: TWIS0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPIM1"]
    pub SPIM1: SPIM1,
    #[doc = "SPIS1"]
    pub SPIS1: SPIS1,
    #[doc = "TWI1"]
    pub TWI1: TWI1,
    #[doc = "TWIM1"]
    pub TWIM1: TWIM1,
    #[doc = "TWIS1"]
    pub TWIS1: TWIS1,
    #[doc = "GPIOTE"]
    pub GPIOTE: GPIOTE,
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "TIMER1"]
    pub TIMER1: TIMER1,
    #[doc = "TIMER2"]
    pub TIMER2: TIMER2,
    #[doc = "RTC0"]
    pub RTC0: RTC0,
    #[doc = "TEMP"]
    pub TEMP: TEMP,
    #[doc = "RNG"]
    pub RNG: RNG,
    #[doc = "ECB"]
    pub ECB: ECB,
    #[doc = "AAR"]
    pub AAR: AAR,
    #[doc = "CCM"]
    pub CCM: CCM,
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "RTC1"]
    pub RTC1: RTC1,
    #[doc = "QDEC"]
    pub QDEC: QDEC,
    #[doc = "COMP"]
    pub COMP: COMP,
    #[doc = "EGU0"]
    pub EGU0: EGU0,
    #[doc = "SWI0"]
    pub SWI0: SWI0,
    #[doc = "EGU1"]
    pub EGU1: EGU1,
    #[doc = "SWI1"]
    pub SWI1: SWI1,
    #[doc = "EGU2"]
    pub EGU2: EGU2,
    #[doc = "SWI2"]
    pub SWI2: SWI2,
    #[doc = "EGU3"]
    pub EGU3: EGU3,
    #[doc = "SWI3"]
    pub SWI3: SWI3,
    #[doc = "EGU4"]
    pub EGU4: EGU4,
    #[doc = "SWI4"]
    pub SWI4: SWI4,
    #[doc = "EGU5"]
    pub EGU5: EGU5,
    #[doc = "SWI5"]
    pub SWI5: SWI5,
    #[doc = "TIMER3"]
    pub TIMER3: TIMER3,
    #[doc = "ACL"]
    pub ACL: ACL,
    #[doc = "NVMC"]
    pub NVMC: NVMC,
    #[doc = "PPI"]
    pub PPI: PPI,
    #[doc = "USBD"]
    pub USBD: USBD,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
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
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            FICR: FICR {
                _marker: PhantomData,
            },
            UICR: UICR {
                _marker: PhantomData,
            },
            APPROTECT: APPROTECT {
                _marker: PhantomData,
            },
            CLOCK: CLOCK {
                _marker: PhantomData,
            },
            POWER: POWER {
                _marker: PhantomData,
            },
            P0: P0 {
                _marker: PhantomData,
            },
            RADIO: RADIO {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UARTE0: UARTE0 {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SPIM0: SPIM0 {
                _marker: PhantomData,
            },
            SPIS0: SPIS0 {
                _marker: PhantomData,
            },
            TWI0: TWI0 {
                _marker: PhantomData,
            },
            TWIM0: TWIM0 {
                _marker: PhantomData,
            },
            TWIS0: TWIS0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SPIM1: SPIM1 {
                _marker: PhantomData,
            },
            SPIS1: SPIS1 {
                _marker: PhantomData,
            },
            TWI1: TWI1 {
                _marker: PhantomData,
            },
            TWIM1: TWIM1 {
                _marker: PhantomData,
            },
            TWIS1: TWIS1 {
                _marker: PhantomData,
            },
            GPIOTE: GPIOTE {
                _marker: PhantomData,
            },
            TIMER0: TIMER0 {
                _marker: PhantomData,
            },
            TIMER1: TIMER1 {
                _marker: PhantomData,
            },
            TIMER2: TIMER2 {
                _marker: PhantomData,
            },
            RTC0: RTC0 {
                _marker: PhantomData,
            },
            TEMP: TEMP {
                _marker: PhantomData,
            },
            RNG: RNG {
                _marker: PhantomData,
            },
            ECB: ECB {
                _marker: PhantomData,
            },
            AAR: AAR {
                _marker: PhantomData,
            },
            CCM: CCM {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
            RTC1: RTC1 {
                _marker: PhantomData,
            },
            QDEC: QDEC {
                _marker: PhantomData,
            },
            COMP: COMP {
                _marker: PhantomData,
            },
            EGU0: EGU0 {
                _marker: PhantomData,
            },
            SWI0: SWI0 {
                _marker: PhantomData,
            },
            EGU1: EGU1 {
                _marker: PhantomData,
            },
            SWI1: SWI1 {
                _marker: PhantomData,
            },
            EGU2: EGU2 {
                _marker: PhantomData,
            },
            SWI2: SWI2 {
                _marker: PhantomData,
            },
            EGU3: EGU3 {
                _marker: PhantomData,
            },
            SWI3: SWI3 {
                _marker: PhantomData,
            },
            EGU4: EGU4 {
                _marker: PhantomData,
            },
            SWI4: SWI4 {
                _marker: PhantomData,
            },
            EGU5: EGU5 {
                _marker: PhantomData,
            },
            SWI5: SWI5 {
                _marker: PhantomData,
            },
            TIMER3: TIMER3 {
                _marker: PhantomData,
            },
            ACL: ACL {
                _marker: PhantomData,
            },
            NVMC: NVMC {
                _marker: PhantomData,
            },
            PPI: PPI {
                _marker: PhantomData,
            },
            USBD: USBD {
                _marker: PhantomData,
            },
        }
    }
}
