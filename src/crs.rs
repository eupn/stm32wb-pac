#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRS control register"]
    pub cr: CR,
    #[doc = "0x04 - CRS configuration register"]
    pub cfgr: CFGR,
    #[doc = "0x08 - CRS interrupt and status register"]
    pub isr: ISR,
    #[doc = "0x0c - CRS interrupt flag clear register"]
    pub icr: ICR,
}
#[doc = "CRS control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRS control register"]
pub mod cr;
#[doc = "CRS configuration register"]
pub struct CFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRS configuration register"]
pub mod cfgr;
#[doc = "CRS interrupt and status register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRS interrupt and status register"]
pub mod isr;
#[doc = "CRS interrupt flag clear register"]
pub struct ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRS interrupt flag clear register"]
pub mod icr;
