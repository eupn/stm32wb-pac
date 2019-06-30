#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Multiplexer Channel 0 Control register"]
    pub c0cr: C0CR,
    #[doc = "0x04 - DMA Multiplexer Channel 1 Control register"]
    pub c1cr: C1CR,
    #[doc = "0x08 - DMA Multiplexer Channel 2 Control register"]
    pub c2cr: C2CR,
    #[doc = "0x0c - DMA Multiplexer Channel 3 Control register"]
    pub c3cr: C3CR,
    #[doc = "0x10 - DMA Multiplexer Channel 4 Control register"]
    pub c4cr: C4CR,
    #[doc = "0x14 - DMA Multiplexer Channel 5 Control register"]
    pub c5cr: C5CR,
    #[doc = "0x18 - DMA Multiplexer Channel 6 Control register"]
    pub c6cr: C6CR,
    #[doc = "0x1c - DMA Multiplexer Channel 7 Control register"]
    pub c7cr: C7CR,
    #[doc = "0x20 - DMA Multiplexer Channel 8 Control register"]
    pub c8cr: C8CR,
    #[doc = "0x24 - DMA Multiplexer Channel 9 Control register"]
    pub c9cr: C9CR,
    #[doc = "0x28 - DMA Multiplexer Channel 10 Control register"]
    pub c10cr: C10CR,
    #[doc = "0x2c - DMA Multiplexer Channel 11 Control register"]
    pub c11cr: C11CR,
    #[doc = "0x30 - DMA Multiplexer Channel 12 Control register"]
    pub c12cr: C12CR,
    #[doc = "0x34 - DMA Multiplexer Channel 13 Control register"]
    pub c13cr: C13CR,
    _reserved0: [u8; 72usize],
    #[doc = "0x80 - DMA Multiplexer Channel Status register"]
    pub csr: CSR,
    #[doc = "0x84 - DMA Channel Clear Flag Register"]
    pub cfr: CFR,
    _reserved1: [u8; 120usize],
    #[doc = "0x100 - DMA Request Generator 0 Control Register"]
    pub rg0cr: RG0CR,
    #[doc = "0x104 - DMA Request Generator 1 Control Register"]
    pub rg1cr: RG1CR,
    #[doc = "0x108 - DMA Request Generator 2 Control Register"]
    pub rg2cr: RG2CR,
    #[doc = "0x10c - DMA Request Generator 3 Control Register"]
    pub rg3cr: RG3CR,
    _reserved2: [u8; 48usize],
    #[doc = "0x140 - DMA Request Generator Status Register"]
    pub rgsr: RGSR,
    #[doc = "0x144 - DMA Request Generator Clear Flag Register"]
    pub rgcfr: RGCFR,
}
#[doc = "DMA Multiplexer Channel 0 Control register"]
pub struct C0CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Multiplexer Channel 0 Control register"]
pub mod c0cr;
#[doc = "DMA Multiplexer Channel 1 Control register"]
pub struct C1CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Multiplexer Channel 1 Control register"]
pub mod c1cr;
#[doc = "DMA Multiplexer Channel 2 Control register"]
pub struct C2CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Multiplexer Channel 2 Control register"]
pub mod c2cr;
#[doc = "DMA Multiplexer Channel 3 Control register"]
pub struct C3CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Multiplexer Channel 3 Control register"]
pub mod c3cr;
#[doc = "DMA Multiplexer Channel 4 Control register"]
pub struct C4CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Multiplexer Channel 4 Control register"]
pub mod c4cr;
#[doc = "DMA Multiplexer Channel 5 Control register"]
pub struct C5CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Multiplexer Channel 5 Control register"]
pub mod c5cr;
#[doc = "DMA Multiplexer Channel 6 Control register"]
pub struct C6CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Multiplexer Channel 6 Control register"]
pub mod c6cr;
#[doc = "DMA Multiplexer Channel 7 Control register"]
pub struct C7CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Multiplexer Channel 7 Control register"]
pub mod c7cr;
#[doc = "DMA Multiplexer Channel 8 Control register"]
pub struct C8CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Multiplexer Channel 8 Control register"]
pub mod c8cr;
#[doc = "DMA Multiplexer Channel 9 Control register"]
pub struct C9CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Multiplexer Channel 9 Control register"]
pub mod c9cr;
#[doc = "DMA Multiplexer Channel 10 Control register"]
pub struct C10CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Multiplexer Channel 10 Control register"]
pub mod c10cr;
#[doc = "DMA Multiplexer Channel 11 Control register"]
pub struct C11CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Multiplexer Channel 11 Control register"]
pub mod c11cr;
#[doc = "DMA Multiplexer Channel 12 Control register"]
pub struct C12CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Multiplexer Channel 12 Control register"]
pub mod c12cr;
#[doc = "DMA Multiplexer Channel 13 Control register"]
pub struct C13CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Multiplexer Channel 13 Control register"]
pub mod c13cr;
#[doc = "DMA Multiplexer Channel Status register"]
pub struct CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Multiplexer Channel Status register"]
pub mod csr;
#[doc = "DMA Channel Clear Flag Register"]
pub struct CFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Clear Flag Register"]
pub mod cfr;
#[doc = "DMA Request Generator 0 Control Register"]
pub struct RG0CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Request Generator 0 Control Register"]
pub mod rg0cr;
#[doc = "DMA Request Generator 1 Control Register"]
pub struct RG1CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Request Generator 1 Control Register"]
pub mod rg1cr;
#[doc = "DMA Request Generator 2 Control Register"]
pub struct RG2CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Request Generator 2 Control Register"]
pub mod rg2cr;
#[doc = "DMA Request Generator 3 Control Register"]
pub struct RG3CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Request Generator 3 Control Register"]
pub mod rg3cr;
#[doc = "DMA Request Generator Status Register"]
pub struct RGSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Request Generator Status Register"]
pub mod rgsr;
#[doc = "DMA Request Generator Clear Flag Register"]
pub struct RGCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Request Generator Clear Flag Register"]
pub mod rgcfr;
