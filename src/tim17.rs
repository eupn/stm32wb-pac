#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - control register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - DMA/Interrupt enable register"]
    pub dier: DIER,
    #[doc = "0x0c - status register"]
    pub sr: SR,
    #[doc = "0x10 - event generation register"]
    pub egr: EGR,
    #[doc = "0x14 - capture/compare mode register (output mode)"]
    pub ccmr1_output: CCMR1_OUTPUT,
    #[doc = "0x18 - capture/compare mode register 1 (input mode)"]
    pub ccmr1_input: CCMR1_INPUT,
    #[doc = "0x1c - capture/compare enable register"]
    pub ccer: CCER,
    #[doc = "0x20 - counter"]
    pub cnt: CNT,
    #[doc = "0x24 - prescaler"]
    pub psc: PSC,
    #[doc = "0x28 - auto-reload register"]
    pub arr: ARR,
    #[doc = "0x2c - repetition counter register"]
    pub rcr: RCR,
    #[doc = "0x30 - capture/compare register 1"]
    pub ccr1: CCR1,
    #[doc = "0x34 - break and dead-time register"]
    pub bdtr: BDTR,
    #[doc = "0x38 - DMA control register"]
    pub dcr: DCR,
    #[doc = "0x3c - DMA address for full transfer"]
    pub dmar: DMAR,
    #[doc = "0x40 - TIM16 option register 1"]
    pub or: OR,
    #[doc = "0x44 - TIM17 option register 1"]
    pub af1: AF1,
}
#[doc = "control register 1"]
pub struct CR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register 1"]
pub mod cr1;
#[doc = "control register 2"]
pub struct CR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register 2"]
pub mod cr2;
#[doc = "DMA/Interrupt enable register"]
pub struct DIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA/Interrupt enable register"]
pub mod dier;
#[doc = "status register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "status register"]
pub mod sr;
#[doc = "event generation register"]
pub struct EGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "event generation register"]
pub mod egr;
#[doc = "capture/compare mode register (output mode)"]
pub struct CCMR1_OUTPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "capture/compare mode register (output mode)"]
pub mod ccmr1_output;
#[doc = "capture/compare mode register 1 (input mode)"]
pub struct CCMR1_INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "capture/compare mode register 1 (input mode)"]
pub mod ccmr1_input;
#[doc = "capture/compare enable register"]
pub struct CCER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "capture/compare enable register"]
pub mod ccer;
#[doc = "counter"]
pub struct CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "counter"]
pub mod cnt;
#[doc = "prescaler"]
pub struct PSC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "prescaler"]
pub mod psc;
#[doc = "auto-reload register"]
pub struct ARR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "auto-reload register"]
pub mod arr;
#[doc = "repetition counter register"]
pub struct RCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "repetition counter register"]
pub mod rcr;
#[doc = "capture/compare register 1"]
pub struct CCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "capture/compare register 1"]
pub mod ccr1;
#[doc = "break and dead-time register"]
pub struct BDTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "break and dead-time register"]
pub mod bdtr;
#[doc = "DMA control register"]
pub struct DCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA control register"]
pub mod dcr;
#[doc = "DMA address for full transfer"]
pub struct DMAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA address for full transfer"]
pub mod dmar;
#[doc = "TIM16 option register 1"]
pub struct OR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TIM16 option register 1"]
pub mod or;
#[doc = "TIM17 option register 1"]
pub struct AF1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TIM17 option register 1"]
pub mod af1;
