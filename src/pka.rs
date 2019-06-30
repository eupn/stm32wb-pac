#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub cr: CR,
    #[doc = "0x04 - PKA status register"]
    pub sr: SR,
    #[doc = "0x08 - PKA clear flag register"]
    pub clrfr: CLRFR,
    _reserved0: [u8; 8168usize],
    #[doc = "0x1ff4 - PKA version register"]
    pub verr: VERR,
    #[doc = "0x1ff8 - PKA identification register"]
    pub ipidr: IPIDR,
    #[doc = "0x1ffc - PKA size ID register"]
    pub sidr: SIDR,
}
#[doc = "Control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register"]
pub mod cr;
#[doc = "PKA status register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PKA status register"]
pub mod sr;
#[doc = "PKA clear flag register"]
pub struct CLRFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PKA clear flag register"]
pub mod clrfr;
#[doc = "PKA version register"]
pub struct VERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PKA version register"]
pub mod verr;
#[doc = "PKA identification register"]
pub struct IPIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PKA identification register"]
pub mod ipidr;
#[doc = "PKA size ID register"]
pub struct SIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PKA size ID register"]
pub mod sidr;
