#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register CPU1"]
    pub c1cr: C1CR,
    #[doc = "0x04 - Mask register CPU1"]
    pub c1mr: C1MR,
    #[doc = "0x08 - Status Set or Clear register CPU1"]
    pub c1scr: C1SCR,
    #[doc = "0x0c - CPU1 to CPU2 status register"]
    pub c1to2sr: C1TO2SR,
    #[doc = "0x10 - Control register CPU2"]
    pub c2cr: C2CR,
    #[doc = "0x14 - Mask register CPU2"]
    pub c2mr: C2MR,
    #[doc = "0x18 - Status Set or Clear register CPU2"]
    pub c2scr: C2SCR,
    #[doc = "0x1c - CPU2 to CPU1 status register"]
    pub c2toc1sr: C2TOC1SR,
    _reserved0: [u8; 976usize],
    #[doc = "0x3f0 - IPCC Hardware configuration register"]
    pub hwcfgr: HWCFGR,
    #[doc = "0x3f4 - IPCC version register"]
    pub verr: VERR,
    #[doc = "0x3f8 - IPCC indentification register"]
    pub ipidr: IPIDR,
    #[doc = "0x3fc - IPCC size indentification register"]
    pub sidr: SIDR,
}
#[doc = "Control register CPU1"]
pub struct C1CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register CPU1"]
pub mod c1cr;
#[doc = "Mask register CPU1"]
pub struct C1MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask register CPU1"]
pub mod c1mr;
#[doc = "Status Set or Clear register CPU1"]
pub struct C1SCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Set or Clear register CPU1"]
pub mod c1scr;
#[doc = "CPU1 to CPU2 status register"]
pub struct C1TO2SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU1 to CPU2 status register"]
pub mod c1to2sr;
#[doc = "Control register CPU2"]
pub struct C2CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register CPU2"]
pub mod c2cr;
#[doc = "Mask register CPU2"]
pub struct C2MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask register CPU2"]
pub mod c2mr;
#[doc = "Status Set or Clear register CPU2"]
pub struct C2SCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Set or Clear register CPU2"]
pub mod c2scr;
#[doc = "CPU2 to CPU1 status register"]
pub struct C2TOC1SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU2 to CPU1 status register"]
pub mod c2toc1sr;
#[doc = "IPCC Hardware configuration register"]
pub struct HWCFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPCC Hardware configuration register"]
pub mod hwcfgr;
#[doc = "IPCC version register"]
pub struct VERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPCC version register"]
pub mod verr;
#[doc = "IPCC indentification register"]
pub struct IPIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPCC indentification register"]
pub mod ipidr;
#[doc = "IPCC size indentification register"]
pub struct SIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPCC size indentification register"]
pub mod sidr;
