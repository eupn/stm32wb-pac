#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - endpoint 0 register"]
    pub ep0r: EP0R,
    _reserved0: [u8; 2usize],
    #[doc = "0x04 - endpoint 1 register"]
    pub ep1r: EP1R,
    _reserved1: [u8; 2usize],
    #[doc = "0x08 - endpoint 2 register"]
    pub ep2r: EP2R,
    _reserved2: [u8; 2usize],
    #[doc = "0x0c - endpoint 3 register"]
    pub ep3r: EP3R,
    _reserved3: [u8; 2usize],
    #[doc = "0x10 - endpoint 4 register"]
    pub ep4r: EP4R,
    _reserved4: [u8; 2usize],
    #[doc = "0x14 - endpoint 5 register"]
    pub ep5r: EP5R,
    _reserved5: [u8; 2usize],
    #[doc = "0x18 - endpoint 6 register"]
    pub ep6r: EP6R,
    _reserved6: [u8; 2usize],
    #[doc = "0x1c - endpoint 7 register"]
    pub ep7r: EP7R,
    _reserved7: [u8; 34usize],
    #[doc = "0x40 - control register"]
    pub cntr: CNTR,
    _reserved8: [u8; 2usize],
    #[doc = "0x44 - interrupt status register"]
    pub istr: ISTR,
    _reserved9: [u8; 2usize],
    #[doc = "0x48 - frame number register"]
    pub fnr: FNR,
    _reserved10: [u8; 2usize],
    #[doc = "0x4c - device address"]
    pub daddr: DADDR,
    _reserved11: [u8; 2usize],
    #[doc = "0x50 - Buffer table address"]
    pub btable: BTABLE,
    #[doc = "0x52 - Transmission byte count 0"]
    pub count0_tx: COUNT0_TX,
    #[doc = "0x54 - Reception buffer address 0"]
    pub addr0_rx: ADDR0_RX,
    #[doc = "0x56 - Reception byte count 0"]
    pub count0_rx: COUNT0_RX,
    #[doc = "0x58 - Battery charging detector("]
    pub bcdr: BCDR,
    #[doc = "0x5a - Transmission byte count 0"]
    pub count1_tx: COUNT1_TX,
    #[doc = "0x5c - Reception buffer address 0"]
    pub addr1_rx: ADDR1_RX,
    #[doc = "0x5e - Reception byte count 0"]
    pub count1_rx: COUNT1_RX,
    _reserved12: [u8; 2usize],
    #[doc = "0x62 - Transmission byte count 0"]
    pub count2_tx: COUNT2_TX,
    #[doc = "0x64 - Reception buffer address 0"]
    pub addr2_rx: ADDR2_RX,
    #[doc = "0x66 - Reception byte count 0"]
    pub count2_rx: COUNT2_RX,
    _reserved13: [u8; 2usize],
    #[doc = "0x6a - Transmission byte count 0"]
    pub count3_tx: COUNT3_TX,
    #[doc = "0x6c - Reception buffer address 0"]
    pub addr3_rx: ADDR3_RX,
    #[doc = "0x6e - Reception byte count 0"]
    pub count3_rx: COUNT3_RX,
    _reserved14: [u8; 2usize],
    #[doc = "0x72 - Transmission byte count 0"]
    pub count4_tx: COUNT4_TX,
    #[doc = "0x74 - Reception buffer address 0"]
    pub addr4_rx: ADDR4_RX,
    #[doc = "0x76 - Reception byte count 0"]
    pub count4_rx: COUNT4_RX,
    _reserved15: [u8; 2usize],
    #[doc = "0x7a - Transmission byte count 0"]
    pub count5_tx: COUNT5_TX,
    #[doc = "0x7c - Reception buffer address 0"]
    pub addr5_rx: ADDR5_RX,
    #[doc = "0x7e - Reception byte count 0"]
    pub count5_rx: COUNT5_RX,
    _reserved16: [u8; 2usize],
    #[doc = "0x82 - Transmission byte count 0"]
    pub count6_tx: COUNT6_TX,
    #[doc = "0x84 - Reception buffer address 0"]
    pub addr6_rx: ADDR6_RX,
    #[doc = "0x86 - Reception byte count 0"]
    pub count6_rx: COUNT6_RX,
    _reserved17: [u8; 2usize],
    #[doc = "0x8a - Transmission byte count 0"]
    pub count7_tx: COUNT7_TX,
    #[doc = "0x8c - Reception buffer address 0"]
    pub addr7_rx: ADDR7_RX,
    #[doc = "0x8e - Reception byte count 0"]
    pub count7_rx: COUNT7_RX,
}
#[doc = "endpoint 0 register"]
pub struct EP0R {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "endpoint 0 register"]
pub mod ep0r;
#[doc = "endpoint 1 register"]
pub struct EP1R {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "endpoint 1 register"]
pub mod ep1r;
#[doc = "endpoint 2 register"]
pub struct EP2R {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "endpoint 2 register"]
pub mod ep2r;
#[doc = "endpoint 3 register"]
pub struct EP3R {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "endpoint 3 register"]
pub mod ep3r;
#[doc = "endpoint 4 register"]
pub struct EP4R {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "endpoint 4 register"]
pub mod ep4r;
#[doc = "endpoint 5 register"]
pub struct EP5R {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "endpoint 5 register"]
pub mod ep5r;
#[doc = "endpoint 6 register"]
pub struct EP6R {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "endpoint 6 register"]
pub mod ep6r;
#[doc = "endpoint 7 register"]
pub struct EP7R {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "endpoint 7 register"]
pub mod ep7r;
#[doc = "control register"]
pub struct CNTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "control register"]
pub mod cntr;
#[doc = "interrupt status register"]
pub struct ISTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "interrupt status register"]
pub mod istr;
#[doc = "frame number register"]
pub struct FNR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "frame number register"]
pub mod fnr;
#[doc = "device address"]
pub struct DADDR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "device address"]
pub mod daddr;
#[doc = "Buffer table address"]
pub struct BTABLE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Buffer table address"]
pub mod btable;
#[doc = "Transmission byte count 0"]
pub struct COUNT0_TX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Transmission byte count 0"]
pub mod count0_tx;
#[doc = "Transmission byte count 0"]
pub struct COUNT1_TX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Transmission byte count 0"]
pub mod count1_tx;
#[doc = "Transmission byte count 0"]
pub struct COUNT2_TX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Transmission byte count 0"]
pub mod count2_tx;
#[doc = "Transmission byte count 0"]
pub struct COUNT3_TX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Transmission byte count 0"]
pub mod count3_tx;
#[doc = "Transmission byte count 0"]
pub struct COUNT4_TX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Transmission byte count 0"]
pub mod count4_tx;
#[doc = "Transmission byte count 0"]
pub struct COUNT5_TX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Transmission byte count 0"]
pub mod count5_tx;
#[doc = "Transmission byte count 0"]
pub struct COUNT6_TX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Transmission byte count 0"]
pub mod count6_tx;
#[doc = "Transmission byte count 0"]
pub struct COUNT7_TX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Transmission byte count 0"]
pub mod count7_tx;
#[doc = "Reception buffer address 0"]
pub struct ADDR0_RX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Reception buffer address 0"]
pub mod addr0_rx;
#[doc = "Reception buffer address 0"]
pub struct ADDR1_RX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Reception buffer address 0"]
pub mod addr1_rx;
#[doc = "Reception buffer address 0"]
pub struct ADDR2_RX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Reception buffer address 0"]
pub mod addr2_rx;
#[doc = "Reception buffer address 0"]
pub struct ADDR3_RX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Reception buffer address 0"]
pub mod addr3_rx;
#[doc = "Reception buffer address 0"]
pub struct ADDR4_RX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Reception buffer address 0"]
pub mod addr4_rx;
#[doc = "Reception buffer address 0"]
pub struct ADDR5_RX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Reception buffer address 0"]
pub mod addr5_rx;
#[doc = "Reception buffer address 0"]
pub struct ADDR6_RX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Reception buffer address 0"]
pub mod addr6_rx;
#[doc = "Reception buffer address 0"]
pub struct ADDR7_RX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Reception buffer address 0"]
pub mod addr7_rx;
#[doc = "Reception byte count 0"]
pub struct COUNT0_RX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Reception byte count 0"]
pub mod count0_rx;
#[doc = "Reception byte count 0"]
pub struct COUNT1_RX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Reception byte count 0"]
pub mod count1_rx;
#[doc = "Reception byte count 0"]
pub struct COUNT2_RX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Reception byte count 0"]
pub mod count2_rx;
#[doc = "Reception byte count 0"]
pub struct COUNT3_RX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Reception byte count 0"]
pub mod count3_rx;
#[doc = "Reception byte count 0"]
pub struct COUNT4_RX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Reception byte count 0"]
pub mod count4_rx;
#[doc = "Reception byte count 0"]
pub struct COUNT5_RX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Reception byte count 0"]
pub mod count5_rx;
#[doc = "Reception byte count 0"]
pub struct COUNT6_RX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Reception byte count 0"]
pub mod count6_rx;
#[doc = "Reception byte count 0"]
pub struct COUNT7_RX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Reception byte count 0"]
pub mod count7_rx;
#[doc = "control and status register"]
pub struct LPMCSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "control and status register"]
pub mod lpmcsr;
#[doc = "Battery charging detector("]
pub struct BCDR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Battery charging detector("]
pub mod bcdr;
