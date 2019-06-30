#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCU Device ID Code Register"]
    pub idcode: IDCODE,
    #[doc = "0x04 - Debug MCU Configuration Register"]
    pub cr: CR,
    _reserved0: [u8; 52usize],
    #[doc = "0x3c - APB1 Low Freeze Register CPU1"]
    pub apb1fzr1: APB1FZR1,
    #[doc = "0x40 - APB1 Low Freeze Register CPU2"]
    pub c2ap_b1fzr1: C2AP_B1FZR1,
    #[doc = "0x44 - APB1 High Freeze Register CPU1"]
    pub apb1fzr2: APB1FZR2,
    #[doc = "0x48 - APB1 High Freeze Register CPU2"]
    pub c2apb1fzr2: C2APB1FZR2,
    #[doc = "0x4c - APB2 Freeze Register CPU1"]
    pub apb2fzr: APB2FZR,
}
#[doc = "MCU Device ID Code Register"]
pub struct IDCODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU Device ID Code Register"]
pub mod idcode;
#[doc = "Debug MCU Configuration Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug MCU Configuration Register"]
pub mod cr;
#[doc = "APB1 Low Freeze Register CPU1"]
pub struct APB1FZR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB1 Low Freeze Register CPU1"]
pub mod apb1fzr1;
#[doc = "APB1 Low Freeze Register CPU2"]
pub struct C2AP_B1FZR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB1 Low Freeze Register CPU2"]
pub mod c2ap_b1fzr1;
#[doc = "APB1 High Freeze Register CPU1"]
pub struct APB1FZR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB1 High Freeze Register CPU1"]
pub mod apb1fzr2;
#[doc = "APB1 High Freeze Register CPU2"]
pub struct C2APB1FZR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB1 High Freeze Register CPU2"]
pub mod c2apb1fzr2;
#[doc = "APB2 Freeze Register CPU1"]
pub struct APB2FZR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB2 Freeze Register CPU1"]
pub mod apb2fzr;
#[doc = "APB2 Freeze Register CPU2"]
pub struct C2APB2FZR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB2 Freeze Register CPU2"]
pub mod c2apb2fzr;
