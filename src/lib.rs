#![doc = "Peripheral access API for STM32WBXX_CM4 microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
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
    fn WWDG();
    fn PVD();
    fn RTC_TAMP();
    fn RTC_WKUP();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_CHANNEL1();
    fn DMA1_CHANNEL2();
    fn DMA1_CHANNEL3();
    fn DMA1_CHANNEL4();
    fn DMA1_CHANNEL5();
    fn DMA1_CHANNEL6();
    fn DMA1_CHANNEL7();
    fn ADC1();
    fn USB_HP();
    fn USB_LP();
    fn C2SEV();
    fn COMP();
    fn EXTI5_9();
    fn TIM1_BRK();
    fn TIM1_UP();
    fn TIM1_TRG_COM_TIM17();
    fn TIM1_CC();
    fn TIM2();
    fn PKA();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C3_EV();
    fn I2C3_ER();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn LPUART1();
    fn SAI1();
    fn TSC();
    fn EXTI10_15();
    fn RTC_ALARM();
    fn CRS_IT();
    fn PWR_SOTF();
    fn IPCC_C1_RX_IT();
    fn IPCC_C1_TX_IT();
    fn HSEM();
    fn LPTIM1();
    fn LPTIM2();
    fn LCD();
    fn QUADSPI();
    fn AES1();
    fn AES2();
    fn TRUE_RNG();
    fn FPU();
    fn DMA2_CH1();
    fn DMA2_CH2();
    fn DMA2_CH3();
    fn DMA2_CH4();
    fn DMA2_CH5();
    fn DMA2_CH6();
    fn DMA2_CH7();
    fn DMAMUX_OVR();
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
pub static __INTERRUPTS: [Vector; 63] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD },
    Vector { _handler: RTC_TAMP },
    Vector { _handler: RTC_WKUP },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector { _handler: EXTI2 },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector {
        _handler: DMA1_CHANNEL1,
    },
    Vector {
        _handler: DMA1_CHANNEL2,
    },
    Vector {
        _handler: DMA1_CHANNEL3,
    },
    Vector {
        _handler: DMA1_CHANNEL4,
    },
    Vector {
        _handler: DMA1_CHANNEL5,
    },
    Vector {
        _handler: DMA1_CHANNEL6,
    },
    Vector {
        _handler: DMA1_CHANNEL7,
    },
    Vector { _handler: ADC1 },
    Vector { _handler: USB_HP },
    Vector { _handler: USB_LP },
    Vector { _handler: C2SEV },
    Vector { _handler: COMP },
    Vector { _handler: EXTI5_9 },
    Vector { _handler: TIM1_BRK },
    Vector { _handler: TIM1_UP },
    Vector {
        _handler: TIM1_TRG_COM_TIM17,
    },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: PKA },
    Vector { _handler: I2C1_EV },
    Vector { _handler: I2C1_ER },
    Vector { _handler: I2C3_EV },
    Vector { _handler: I2C3_ER },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: LPUART1 },
    Vector { _handler: SAI1 },
    Vector { _handler: TSC },
    Vector {
        _handler: EXTI10_15,
    },
    Vector {
        _handler: RTC_ALARM,
    },
    Vector { _handler: CRS_IT },
    Vector { _handler: PWR_SOTF },
    Vector {
        _handler: IPCC_C1_RX_IT,
    },
    Vector {
        _handler: IPCC_C1_TX_IT,
    },
    Vector { _handler: HSEM },
    Vector { _handler: LPTIM1 },
    Vector { _handler: LPTIM2 },
    Vector { _handler: LCD },
    Vector { _handler: QUADSPI },
    Vector { _handler: AES1 },
    Vector { _handler: AES2 },
    Vector { _handler: TRUE_RNG },
    Vector { _handler: FPU },
    Vector { _handler: DMA2_CH1 },
    Vector { _handler: DMA2_CH2 },
    Vector { _handler: DMA2_CH3 },
    Vector { _handler: DMA2_CH4 },
    Vector { _handler: DMA2_CH5 },
    Vector { _handler: DMA2_CH6 },
    Vector { _handler: DMA2_CH7 },
    Vector {
        _handler: DMAMUX_OVR,
    },
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - Window Watchdog interrupt"]
    WWDG,
    #[doc = "1 - PVD through EXTI\\[16\\] (C1IMR2\\[20\\])"]
    PVD,
    #[doc = "2 - RTC/TAMP/CSS on LSE through EXTI line 19 interrupt"]
    RTC_TAMP,
    #[doc = "3 - RTC wakeup interrupt through EXTI\\[19\\]"]
    RTC_WKUP,
    #[doc = "4 - Flash global interrupt"]
    FLASH,
    #[doc = "5 - RCC global interrupt"]
    RCC,
    #[doc = "6 - EXTI line 0 interrupt through EXTI\\[0\\]"]
    EXTI0,
    #[doc = "7 - EXTI line 0 interrupt through EXTI\\[1\\]"]
    EXTI1,
    #[doc = "8 - EXTI line 0 interrupt through EXTI\\[2\\]"]
    EXTI2,
    #[doc = "9 - EXTI line 0 interrupt through EXTI\\[3\\]"]
    EXTI3,
    #[doc = "10 - EXTI line 0 interrupt through EXTI\\[4\\]"]
    EXTI4,
    #[doc = "11 - DMA1 Channel1 global interrupt"]
    DMA1_CHANNEL1,
    #[doc = "12 - DMA1 Channel2 global interrupt"]
    DMA1_CHANNEL2,
    #[doc = "13 - DMA1 Channel3 interrupt"]
    DMA1_CHANNEL3,
    #[doc = "14 - DMA1 Channel4 interrupt"]
    DMA1_CHANNEL4,
    #[doc = "15 - DMA1 Channel5 interrupt"]
    DMA1_CHANNEL5,
    #[doc = "16 - DMA1 Channel6 interrupt"]
    DMA1_CHANNEL6,
    #[doc = "17 - DMA1 Channel 7 interrupt"]
    DMA1_CHANNEL7,
    #[doc = "18 - ADC1 global interrupt"]
    ADC1,
    #[doc = "19 - USB high priority interrupt"]
    USB_HP,
    #[doc = "20 - USB low priority interrupt (including USB wakeup)"]
    USB_LP,
    #[doc = "21 - CPU2 SEV through EXTI\\[40\\]"]
    C2SEV,
    #[doc = "22 - COMP2 & COMP1 interrupt through AIEC\\[21:20\\]"]
    COMP,
    #[doc = "23 - EXTI line \\[9:5\\] interrupt through EXTI\\[9:5\\]"]
    EXTI5_9,
    #[doc = "24 - Timer 1 break interrupt"]
    TIM1_BRK,
    #[doc = "25 - Timer 1 Update"]
    TIM1_UP,
    #[doc = "26 - TIM1 Trigger and Commutation interrupts and TIM17 global interrupt"]
    TIM1_TRG_COM_TIM17,
    #[doc = "27 - TIM1 Capture Compare interrupt"]
    TIM1_CC,
    #[doc = "28 - TIM2 global interrupt"]
    TIM2,
    #[doc = "29 - Private key accelerator interrupt"]
    PKA,
    #[doc = "30 - I2C1 event interrupt"]
    I2C1_EV,
    #[doc = "31 - I2C1 error interrupt"]
    I2C1_ER,
    #[doc = "32 - I2C3 event interrupt"]
    I2C3_EV,
    #[doc = "33 - I2C3 error interrupt"]
    I2C3_ER,
    #[doc = "34 - SPI 1 global interrupt"]
    SPI1,
    #[doc = "35 - SPI1 global interrupt"]
    SPI2,
    #[doc = "36 - USART1 global interrupt"]
    USART1,
    #[doc = "37 - LPUART1 global interrupt"]
    LPUART1,
    #[doc = "38 - SAI1 global interrupt"]
    SAI1,
    #[doc = "39 - TSC global interrupt"]
    TSC,
    #[doc = "40 - EXTI line \\[15:10\\] interrupt through EXTI\\[15:10\\]"]
    EXTI10_15,
    #[doc = "41 - RTC Alarms (A and B) interrupt through AIEC"]
    RTC_ALARM,
    #[doc = "42 - CRS interrupt"]
    CRS_IT,
    #[doc = "43 - PWR switching on the fly interrupt"]
    PWR_SOTF,
    #[doc = "44 - IPCC CPU1 RX occupied interrupt"]
    IPCC_C1_RX_IT,
    #[doc = "45 - IPCC CPU1 TX free interrupt"]
    IPCC_C1_TX_IT,
    #[doc = "46 - Semaphore interrupt 0 to CPU1"]
    HSEM,
    #[doc = "47 - LPtimer 1 global interrupt"]
    LPTIM1,
    #[doc = "48 - LPtimer 2 global interrupt"]
    LPTIM2,
    #[doc = "49 - LCD global interrupt"]
    LCD,
    #[doc = "50 - QSPI global interrupt"]
    QUADSPI,
    #[doc = "51 - AES1 global interrupt"]
    AES1,
    #[doc = "52 - AES2 global interrupt"]
    AES2,
    #[doc = "53 - True random number generator interrupt"]
    TRUE_RNG,
    #[doc = "54 - Floating point unit interrupt"]
    FPU,
    #[doc = "55 - DMA2 channel 1 interrupt"]
    DMA2_CH1,
    #[doc = "56 - DMA2 channel 2 interrupt"]
    DMA2_CH2,
    #[doc = "57 - DMA2 channel 3 interrupt"]
    DMA2_CH3,
    #[doc = "58 - DMA2 channel 4 interrupt"]
    DMA2_CH4,
    #[doc = "59 - DMA2 channel 5 interrupt"]
    DMA2_CH5,
    #[doc = "60 - DMA2 channel 6 interrupt"]
    DMA2_CH6,
    #[doc = "61 - DMA2 channel 7 interrupt"]
    DMA2_CH7,
    #[doc = "62 - DMAMUX overrun interrupt"]
    DMAMUX_OVR,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::WWDG => 0,
            Interrupt::PVD => 1,
            Interrupt::RTC_TAMP => 2,
            Interrupt::RTC_WKUP => 3,
            Interrupt::FLASH => 4,
            Interrupt::RCC => 5,
            Interrupt::EXTI0 => 6,
            Interrupt::EXTI1 => 7,
            Interrupt::EXTI2 => 8,
            Interrupt::EXTI3 => 9,
            Interrupt::EXTI4 => 10,
            Interrupt::DMA1_CHANNEL1 => 11,
            Interrupt::DMA1_CHANNEL2 => 12,
            Interrupt::DMA1_CHANNEL3 => 13,
            Interrupt::DMA1_CHANNEL4 => 14,
            Interrupt::DMA1_CHANNEL5 => 15,
            Interrupt::DMA1_CHANNEL6 => 16,
            Interrupt::DMA1_CHANNEL7 => 17,
            Interrupt::ADC1 => 18,
            Interrupt::USB_HP => 19,
            Interrupt::USB_LP => 20,
            Interrupt::C2SEV => 21,
            Interrupt::COMP => 22,
            Interrupt::EXTI5_9 => 23,
            Interrupt::TIM1_BRK => 24,
            Interrupt::TIM1_UP => 25,
            Interrupt::TIM1_TRG_COM_TIM17 => 26,
            Interrupt::TIM1_CC => 27,
            Interrupt::TIM2 => 28,
            Interrupt::PKA => 29,
            Interrupt::I2C1_EV => 30,
            Interrupt::I2C1_ER => 31,
            Interrupt::I2C3_EV => 32,
            Interrupt::I2C3_ER => 33,
            Interrupt::SPI1 => 34,
            Interrupt::SPI2 => 35,
            Interrupt::USART1 => 36,
            Interrupt::LPUART1 => 37,
            Interrupt::SAI1 => 38,
            Interrupt::TSC => 39,
            Interrupt::EXTI10_15 => 40,
            Interrupt::RTC_ALARM => 41,
            Interrupt::CRS_IT => 42,
            Interrupt::PWR_SOTF => 43,
            Interrupt::IPCC_C1_RX_IT => 44,
            Interrupt::IPCC_C1_TX_IT => 45,
            Interrupt::HSEM => 46,
            Interrupt::LPTIM1 => 47,
            Interrupt::LPTIM2 => 48,
            Interrupt::LCD => 49,
            Interrupt::QUADSPI => 50,
            Interrupt::AES1 => 51,
            Interrupt::AES2 => 52,
            Interrupt::TRUE_RNG => 53,
            Interrupt::FPU => 54,
            Interrupt::DMA2_CH1 => 55,
            Interrupt::DMA2_CH2 => 56,
            Interrupt::DMA2_CH3 => 57,
            Interrupt::DMA2_CH4 => 58,
            Interrupt::DMA2_CH5 => 59,
            Interrupt::DMA2_CH6 => 60,
            Interrupt::DMA2_CH7 => 61,
            Interrupt::DMAMUX_OVR => 62,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "Direct memory access controller"]
pub struct DMA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA1 {}
impl DMA1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma1::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for DMA1 {
    type Target = dma1::RegisterBlock;
    fn deref(&self) -> &dma1::RegisterBlock {
        unsafe { &*DMA1::ptr() }
    }
}
#[doc = "Direct memory access controller"]
pub mod dma1;
#[doc = "Direct memory access controller"]
pub struct DMA2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA2 {}
impl DMA2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma2::RegisterBlock {
        1073873920 as *const _
    }
}
impl Deref for DMA2 {
    type Target = dma2::RegisterBlock;
    fn deref(&self) -> &dma2::RegisterBlock {
        unsafe { &*DMA2::ptr() }
    }
}
#[doc = "Direct memory access controller"]
pub mod dma2;
#[doc = "Direct memory access Multiplexer"]
pub struct DMAMUX1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAMUX1 {}
impl DMAMUX1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dmamux1::RegisterBlock {
        1073874944 as *const _
    }
}
impl Deref for DMAMUX1 {
    type Target = dmamux1::RegisterBlock;
    fn deref(&self) -> &dmamux1::RegisterBlock {
        unsafe { &*DMAMUX1::ptr() }
    }
}
#[doc = "Direct memory access Multiplexer"]
pub mod dmamux1;
#[doc = "Cyclic redundancy check calculation unit"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crc::RegisterBlock {
        1073885184 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    fn deref(&self) -> &crc::RegisterBlock {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "Cyclic redundancy check calculation unit"]
pub mod crc;
#[doc = "Liquid crystal display controller"]
pub struct LCD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LCD {}
impl LCD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lcd::RegisterBlock {
        1073751040 as *const _
    }
}
impl Deref for LCD {
    type Target = lcd::RegisterBlock;
    fn deref(&self) -> &lcd::RegisterBlock {
        unsafe { &*LCD::ptr() }
    }
}
#[doc = "Liquid crystal display controller"]
pub mod lcd;
#[doc = "Touch sensing controller"]
pub struct TSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TSC {}
impl TSC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tsc::RegisterBlock {
        1073889280 as *const _
    }
}
impl Deref for TSC {
    type Target = tsc::RegisterBlock;
    fn deref(&self) -> &tsc::RegisterBlock {
        unsafe { &*TSC::ptr() }
    }
}
#[doc = "Touch sensing controller"]
pub mod tsc;
#[doc = "Independent watchdog"]
pub struct IWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IWDG {}
impl IWDG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const iwdg::RegisterBlock {
        1073754112 as *const _
    }
}
impl Deref for IWDG {
    type Target = iwdg::RegisterBlock;
    fn deref(&self) -> &iwdg::RegisterBlock {
        unsafe { &*IWDG::ptr() }
    }
}
#[doc = "Independent watchdog"]
pub mod iwdg;
#[doc = "System window watchdog"]
pub struct WWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDG {}
impl WWDG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wwdg::RegisterBlock {
        1073753088 as *const _
    }
}
impl Deref for WWDG {
    type Target = wwdg::RegisterBlock;
    fn deref(&self) -> &wwdg::RegisterBlock {
        unsafe { &*WWDG::ptr() }
    }
}
#[doc = "System window watchdog"]
pub mod wwdg;
#[doc = "Comparator instance 1"]
pub struct COMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMP {}
impl COMP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const comp::RegisterBlock {
        1073807872 as *const _
    }
}
impl Deref for COMP {
    type Target = comp::RegisterBlock;
    fn deref(&self) -> &comp::RegisterBlock {
        unsafe { &*COMP::ptr() }
    }
}
#[doc = "Comparator instance 1"]
pub mod comp;
#[doc = "Inter-integrated circuit"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c1::RegisterBlock {
        1073763328 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Inter-integrated circuit"]
pub mod i2c1;
#[doc = "I2C3"]
pub struct I2C3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C3 {}
impl I2C3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c1::RegisterBlock {
        1073765376 as *const _
    }
}
impl Deref for I2C3 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        unsafe { &*I2C3::ptr() }
    }
}
#[doc = "Flash"]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const flash::RegisterBlock {
        1476411392 as *const _
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    fn deref(&self) -> &flash::RegisterBlock {
        unsafe { &*FLASH::ptr() }
    }
}
#[doc = "Flash"]
pub mod flash;
#[doc = "QuadSPI interface"]
pub struct QUADSPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QUADSPI {}
impl QUADSPI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const quadspi::RegisterBlock {
        2684358656 as *const _
    }
}
impl Deref for QUADSPI {
    type Target = quadspi::RegisterBlock;
    fn deref(&self) -> &quadspi::RegisterBlock {
        unsafe { &*QUADSPI::ptr() }
    }
}
#[doc = "QuadSPI interface"]
pub mod quadspi;
#[doc = "Reset and clock control"]
pub struct RCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCC {}
impl RCC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rcc::RegisterBlock {
        1476395008 as *const _
    }
}
impl Deref for RCC {
    type Target = rcc::RegisterBlock;
    fn deref(&self) -> &rcc::RegisterBlock {
        unsafe { &*RCC::ptr() }
    }
}
#[doc = "Reset and clock control"]
pub mod rcc;
#[doc = "Power control"]
pub struct PWR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWR {}
impl PWR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwr::RegisterBlock {
        1476396032 as *const _
    }
}
impl Deref for PWR {
    type Target = pwr::RegisterBlock;
    fn deref(&self) -> &pwr::RegisterBlock {
        unsafe { &*PWR::ptr() }
    }
}
#[doc = "Power control"]
pub mod pwr;
#[doc = "System configuration controller"]
pub struct SYSCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCFG {}
impl SYSCFG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const syscfg::RegisterBlock {
        1073807616 as *const _
    }
}
impl Deref for SYSCFG {
    type Target = syscfg::RegisterBlock;
    fn deref(&self) -> &syscfg::RegisterBlock {
        unsafe { &*SYSCFG::ptr() }
    }
}
#[doc = "System configuration controller"]
pub mod syscfg;
#[doc = "Random number generator"]
pub struct RNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG {}
impl RNG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rng::RegisterBlock {
        1476399104 as *const _
    }
}
impl Deref for RNG {
    type Target = rng::RegisterBlock;
    fn deref(&self) -> &rng::RegisterBlock {
        unsafe { &*RNG::ptr() }
    }
}
#[doc = "Random number generator"]
pub mod rng;
#[doc = "Advanced encryption standard hardware accelerator 1"]
pub struct AES1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AES1 {}
impl AES1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aes1::RegisterBlock {
        1342570496 as *const _
    }
}
impl Deref for AES1 {
    type Target = aes1::RegisterBlock;
    fn deref(&self) -> &aes1::RegisterBlock {
        unsafe { &*AES1::ptr() }
    }
}
#[doc = "Advanced encryption standard hardware accelerator 1"]
pub mod aes1;
#[doc = "Advanced encryption standard hardware accelerator 1"]
pub struct AES2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AES2 {}
impl AES2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aes2::RegisterBlock {
        1476401152 as *const _
    }
}
impl Deref for AES2 {
    type Target = aes2::RegisterBlock;
    fn deref(&self) -> &aes2::RegisterBlock {
        unsafe { &*AES2::ptr() }
    }
}
#[doc = "Advanced encryption standard hardware accelerator 1"]
pub mod aes2;
#[doc = "HSEM"]
pub struct HSEM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HSEM {}
impl HSEM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const hsem::RegisterBlock {
        1476400128 as *const _
    }
}
impl Deref for HSEM {
    type Target = hsem::RegisterBlock;
    fn deref(&self) -> &hsem::RegisterBlock {
        unsafe { &*HSEM::ptr() }
    }
}
#[doc = "HSEM"]
pub mod hsem;
#[doc = "Analog to Digital Converter instance 1"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc::RegisterBlock {
        1342439424 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    fn deref(&self) -> &adc::RegisterBlock {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "Analog to Digital Converter instance 1"]
pub mod adc;
#[doc = "General-purpose I/Os"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1207959552 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioa;
#[doc = "General-purpose I/Os"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiob::RegisterBlock {
        1207960576 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    fn deref(&self) -> &gpiob::RegisterBlock {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpiob;
#[doc = "General-purpose I/Os"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioc::RegisterBlock {
        1207961600 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpioc::RegisterBlock;
    fn deref(&self) -> &gpioc::RegisterBlock {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioc;
#[doc = "GPIOD"]
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioc::RegisterBlock {
        1207962624 as *const _
    }
}
impl Deref for GPIOD {
    type Target = gpioc::RegisterBlock;
    fn deref(&self) -> &gpioc::RegisterBlock {
        unsafe { &*GPIOD::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOE {}
impl GPIOE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioe::RegisterBlock {
        1207963648 as *const _
    }
}
impl Deref for GPIOE {
    type Target = gpioe::RegisterBlock;
    fn deref(&self) -> &gpioe::RegisterBlock {
        unsafe { &*GPIOE::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioe;
#[doc = "General-purpose I/Os"]
pub struct GPIOH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOH {}
impl GPIOH {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioh::RegisterBlock {
        1207966720 as *const _
    }
}
impl Deref for GPIOH {
    type Target = gpioh::RegisterBlock;
    fn deref(&self) -> &gpioh::RegisterBlock {
        unsafe { &*GPIOH::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioh;
#[doc = "Serial audio interface"]
pub struct SAI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAI1 {}
impl SAI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sai1::RegisterBlock {
        1073828864 as *const _
    }
}
impl Deref for SAI1 {
    type Target = sai1::RegisterBlock;
    fn deref(&self) -> &sai1::RegisterBlock {
        unsafe { &*SAI1::ptr() }
    }
}
#[doc = "Serial audio interface"]
pub mod sai1;
#[doc = "General-purpose-timers"]
pub struct TIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM2 {}
impl TIM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim2::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for TIM2 {
    type Target = tim2::RegisterBlock;
    fn deref(&self) -> &tim2::RegisterBlock {
        unsafe { &*TIM2::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod tim2;
#[doc = "General purpose timers"]
pub struct TIM16 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM16 {}
impl TIM16 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim16::RegisterBlock {
        1073824768 as *const _
    }
}
impl Deref for TIM16 {
    type Target = tim16::RegisterBlock;
    fn deref(&self) -> &tim16::RegisterBlock {
        unsafe { &*TIM16::ptr() }
    }
}
#[doc = "General purpose timers"]
pub mod tim16;
#[doc = "General purpose timers"]
pub struct TIM17 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM17 {}
impl TIM17 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim17::RegisterBlock {
        1073825792 as *const _
    }
}
impl Deref for TIM17 {
    type Target = tim17::RegisterBlock;
    fn deref(&self) -> &tim17::RegisterBlock {
        unsafe { &*TIM17::ptr() }
    }
}
#[doc = "General purpose timers"]
pub mod tim17;
#[doc = "Advanced-timers"]
pub struct TIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM1 {}
impl TIM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim1::RegisterBlock {
        1073818624 as *const _
    }
}
impl Deref for TIM1 {
    type Target = tim1::RegisterBlock;
    fn deref(&self) -> &tim1::RegisterBlock {
        unsafe { &*TIM1::ptr() }
    }
}
#[doc = "Advanced-timers"]
pub mod tim1;
#[doc = "Low power timer"]
pub struct LPTIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM1 {}
impl LPTIM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lptim1::RegisterBlock {
        1073773568 as *const _
    }
}
impl Deref for LPTIM1 {
    type Target = lptim1::RegisterBlock;
    fn deref(&self) -> &lptim1::RegisterBlock {
        unsafe { &*LPTIM1::ptr() }
    }
}
#[doc = "Low power timer"]
pub mod lptim1;
#[doc = "LPTIM2"]
pub struct LPTIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM2 {}
impl LPTIM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lptim1::RegisterBlock {
        1073779712 as *const _
    }
}
impl Deref for LPTIM2 {
    type Target = lptim1::RegisterBlock;
    fn deref(&self) -> &lptim1::RegisterBlock {
        unsafe { &*LPTIM2::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073821696 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod usart1;
#[doc = "LPUART1"]
pub struct LPUART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART1 {}
impl LPUART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for LPUART1 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*LPUART1::ptr() }
    }
}
#[doc = "Serial peripheral interface/Inter-IC sound"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073819648 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Serial peripheral interface/Inter-IC sound"]
pub mod spi1;
#[doc = "SPI2"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073756160 as *const _
    }
}
impl Deref for SPI2 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI2::ptr() }
    }
}
#[doc = "Voltage reference buffer"]
pub struct VREFBUF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VREFBUF {}
impl VREFBUF {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const vrefbuf::RegisterBlock {
        1073807408 as *const _
    }
}
impl Deref for VREFBUF {
    type Target = vrefbuf::RegisterBlock;
    fn deref(&self) -> &vrefbuf::RegisterBlock {
        unsafe { &*VREFBUF::ptr() }
    }
}
#[doc = "Voltage reference buffer"]
pub mod vrefbuf;
#[doc = "Real-time clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1073752064 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-time clock"]
pub mod rtc;
#[doc = "Debug support"]
pub struct DBGMCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DBGMCU {}
impl DBGMCU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dbgmcu::RegisterBlock {
        3758366720 as *const _
    }
}
impl Deref for DBGMCU {
    type Target = dbgmcu::RegisterBlock;
    fn deref(&self) -> &dbgmcu::RegisterBlock {
        unsafe { &*DBGMCU::ptr() }
    }
}
#[doc = "Debug support"]
pub mod dbgmcu;
#[doc = "PKA"]
pub struct PKA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PKA {}
impl PKA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pka::RegisterBlock {
        1476403200 as *const _
    }
}
impl Deref for PKA {
    type Target = pka::RegisterBlock;
    fn deref(&self) -> &pka::RegisterBlock {
        unsafe { &*PKA::ptr() }
    }
}
#[doc = "PKA"]
pub mod pka;
#[doc = "IPCC"]
pub struct IPCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IPCC {}
impl IPCC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ipcc::RegisterBlock {
        1476398080 as *const _
    }
}
impl Deref for IPCC {
    type Target = ipcc::RegisterBlock;
    fn deref(&self) -> &ipcc::RegisterBlock {
        unsafe { &*IPCC::ptr() }
    }
}
#[doc = "IPCC"]
pub mod ipcc;
#[doc = "External interrupt/event controller"]
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTI {}
impl EXTI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const exti::RegisterBlock {
        1476397056 as *const _
    }
}
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    fn deref(&self) -> &exti::RegisterBlock {
        unsafe { &*EXTI::ptr() }
    }
}
#[doc = "External interrupt/event controller"]
pub mod exti;
#[doc = "Clock recovery system"]
pub struct CRS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRS {}
impl CRS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crs::RegisterBlock {
        1073766400 as *const _
    }
}
impl Deref for CRS {
    type Target = crs::RegisterBlock;
    fn deref(&self) -> &crs::RegisterBlock {
        unsafe { &*CRS::ptr() }
    }
}
#[doc = "Clock recovery system"]
pub mod crs;
#[doc = "Universal serial bus full-speed device interface"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb::RegisterBlock {
        1073768448 as *const _
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    fn deref(&self) -> &usb::RegisterBlock {
        unsafe { &*USB::ptr() }
    }
}
#[doc = "Universal serial bus full-speed device interface"]
pub mod usb;
#[doc = "SysTick timer"]
pub struct STK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for STK {}
impl STK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const stk::RegisterBlock {
        3758153744 as *const _
    }
}
impl Deref for STK {
    type Target = stk::RegisterBlock;
    fn deref(&self) -> &stk::RegisterBlock {
        unsafe { &*STK::ptr() }
    }
}
#[doc = "SysTick timer"]
pub mod stk;
#[doc = "Nested vectored interrupt controller"]
pub struct NVIC_STIR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVIC_STIR {}
impl NVIC_STIR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const nvic_stir::RegisterBlock {
        3758157568 as *const _
    }
}
impl Deref for NVIC_STIR {
    type Target = nvic_stir::RegisterBlock;
    fn deref(&self) -> &nvic_stir::RegisterBlock {
        unsafe { &*NVIC_STIR::ptr() }
    }
}
#[doc = "Nested vectored interrupt controller"]
pub mod nvic_stir;
#[doc = "System control block ACTLR"]
pub struct SCB_ACTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB_ACTRL {}
impl SCB_ACTRL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const scb_actrl::RegisterBlock {
        3758153736 as *const _
    }
}
impl Deref for SCB_ACTRL {
    type Target = scb_actrl::RegisterBlock;
    fn deref(&self) -> &scb_actrl::RegisterBlock {
        unsafe { &*SCB_ACTRL::ptr() }
    }
}
#[doc = "System control block ACTLR"]
pub mod scb_actrl;
#[doc = "Floating point unit CPACR"]
pub struct FPU_CPACR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FPU_CPACR {}
impl FPU_CPACR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fpu_cpacr::RegisterBlock {
        3758157192 as *const _
    }
}
impl Deref for FPU_CPACR {
    type Target = fpu_cpacr::RegisterBlock;
    fn deref(&self) -> &fpu_cpacr::RegisterBlock {
        unsafe { &*FPU_CPACR::ptr() }
    }
}
#[doc = "Floating point unit CPACR"]
pub mod fpu_cpacr;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "DMA1"]
    pub DMA1: DMA1,
    #[doc = "DMA2"]
    pub DMA2: DMA2,
    #[doc = "DMAMUX1"]
    pub DMAMUX1: DMAMUX1,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "LCD"]
    pub LCD: LCD,
    #[doc = "TSC"]
    pub TSC: TSC,
    #[doc = "IWDG"]
    pub IWDG: IWDG,
    #[doc = "WWDG"]
    pub WWDG: WWDG,
    #[doc = "COMP"]
    pub COMP: COMP,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2C3"]
    pub I2C3: I2C3,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "QUADSPI"]
    pub QUADSPI: QUADSPI,
    #[doc = "RCC"]
    pub RCC: RCC,
    #[doc = "PWR"]
    pub PWR: PWR,
    #[doc = "SYSCFG"]
    pub SYSCFG: SYSCFG,
    #[doc = "RNG"]
    pub RNG: RNG,
    #[doc = "AES1"]
    pub AES1: AES1,
    #[doc = "AES2"]
    pub AES2: AES2,
    #[doc = "HSEM"]
    pub HSEM: HSEM,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOD"]
    pub GPIOD: GPIOD,
    #[doc = "GPIOE"]
    pub GPIOE: GPIOE,
    #[doc = "GPIOH"]
    pub GPIOH: GPIOH,
    #[doc = "SAI1"]
    pub SAI1: SAI1,
    #[doc = "TIM2"]
    pub TIM2: TIM2,
    #[doc = "TIM16"]
    pub TIM16: TIM16,
    #[doc = "TIM17"]
    pub TIM17: TIM17,
    #[doc = "TIM1"]
    pub TIM1: TIM1,
    #[doc = "LPTIM1"]
    pub LPTIM1: LPTIM1,
    #[doc = "LPTIM2"]
    pub LPTIM2: LPTIM2,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "LPUART1"]
    pub LPUART1: LPUART1,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "VREFBUF"]
    pub VREFBUF: VREFBUF,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "DBGMCU"]
    pub DBGMCU: DBGMCU,
    #[doc = "PKA"]
    pub PKA: PKA,
    #[doc = "IPCC"]
    pub IPCC: IPCC,
    #[doc = "EXTI"]
    pub EXTI: EXTI,
    #[doc = "CRS"]
    pub CRS: CRS,
    #[doc = "USB"]
    pub USB: USB,
    #[doc = "STK"]
    pub STK: STK,
    #[doc = "NVIC_STIR"]
    pub NVIC_STIR: NVIC_STIR,
    #[doc = "SCB_ACTRL"]
    pub SCB_ACTRL: SCB_ACTRL,
    #[doc = "FPU_CPACR"]
    pub FPU_CPACR: FPU_CPACR,
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
            DMA1: DMA1 {
                _marker: PhantomData,
            },
            DMA2: DMA2 {
                _marker: PhantomData,
            },
            DMAMUX1: DMAMUX1 {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            LCD: LCD {
                _marker: PhantomData,
            },
            TSC: TSC {
                _marker: PhantomData,
            },
            IWDG: IWDG {
                _marker: PhantomData,
            },
            WWDG: WWDG {
                _marker: PhantomData,
            },
            COMP: COMP {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            I2C3: I2C3 {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            QUADSPI: QUADSPI {
                _marker: PhantomData,
            },
            RCC: RCC {
                _marker: PhantomData,
            },
            PWR: PWR {
                _marker: PhantomData,
            },
            SYSCFG: SYSCFG {
                _marker: PhantomData,
            },
            RNG: RNG {
                _marker: PhantomData,
            },
            AES1: AES1 {
                _marker: PhantomData,
            },
            AES2: AES2 {
                _marker: PhantomData,
            },
            HSEM: HSEM {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            GPIOD: GPIOD {
                _marker: PhantomData,
            },
            GPIOE: GPIOE {
                _marker: PhantomData,
            },
            GPIOH: GPIOH {
                _marker: PhantomData,
            },
            SAI1: SAI1 {
                _marker: PhantomData,
            },
            TIM2: TIM2 {
                _marker: PhantomData,
            },
            TIM16: TIM16 {
                _marker: PhantomData,
            },
            TIM17: TIM17 {
                _marker: PhantomData,
            },
            TIM1: TIM1 {
                _marker: PhantomData,
            },
            LPTIM1: LPTIM1 {
                _marker: PhantomData,
            },
            LPTIM2: LPTIM2 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            LPUART1: LPUART1 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            VREFBUF: VREFBUF {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            DBGMCU: DBGMCU {
                _marker: PhantomData,
            },
            PKA: PKA {
                _marker: PhantomData,
            },
            IPCC: IPCC {
                _marker: PhantomData,
            },
            EXTI: EXTI {
                _marker: PhantomData,
            },
            CRS: CRS {
                _marker: PhantomData,
            },
            USB: USB {
                _marker: PhantomData,
            },
            STK: STK {
                _marker: PhantomData,
            },
            NVIC_STIR: NVIC_STIR {
                _marker: PhantomData,
            },
            SCB_ACTRL: SCB_ACTRL {
                _marker: PhantomData,
            },
            FPU_CPACR: FPU_CPACR {
                _marker: PhantomData,
            },
        }
    }
}
