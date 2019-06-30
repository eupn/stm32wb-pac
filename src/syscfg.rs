#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - memory remap register"]
    pub memrmp: MEMRMP,
    #[doc = "0x04 - configuration register 1"]
    pub cfgr1: CFGR1,
    #[doc = "0x08 - external interrupt configuration register 1"]
    pub exticr1: EXTICR1,
    #[doc = "0x0c - external interrupt configuration register 2"]
    pub exticr2: EXTICR2,
    #[doc = "0x10 - external interrupt configuration register 3"]
    pub exticr3: EXTICR3,
    #[doc = "0x14 - external interrupt configuration register 4"]
    pub exticr4: EXTICR4,
    #[doc = "0x18 - SCSR"]
    pub scsr: SCSR,
    #[doc = "0x1c - CFGR2"]
    pub cfgr2: CFGR2,
    #[doc = "0x20 - SRAM2 write protection register"]
    pub swpr: SWPR,
    #[doc = "0x24 - SKR"]
    pub skr: SKR,
    #[doc = "0x28 - SRAM2 write protection register 2"]
    pub swpr2: SWPR2,
    #[doc = "0x2c - CPU1 interrupt mask register 1"]
    pub imr1: IMR1,
    #[doc = "0x30 - CPU1 interrupt mask register 2"]
    pub imr2: IMR2,
    #[doc = "0x34 - CPU2 interrupt mask register 1"]
    pub c2imr1: C2IMR1,
    #[doc = "0x38 - CPU2 interrupt mask register 1"]
    pub c2imr2: C2IMR2,
    #[doc = "0x3c - secure IP control register"]
    pub sipcr: SIPCR,
}
#[doc = "memory remap register"]
pub struct MEMRMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "memory remap register"]
pub mod memrmp;
#[doc = "configuration register 1"]
pub struct CFGR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "configuration register 1"]
pub mod cfgr1;
#[doc = "external interrupt configuration register 1"]
pub struct EXTICR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "external interrupt configuration register 1"]
pub mod exticr1;
#[doc = "external interrupt configuration register 2"]
pub struct EXTICR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "external interrupt configuration register 2"]
pub mod exticr2;
#[doc = "external interrupt configuration register 3"]
pub struct EXTICR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "external interrupt configuration register 3"]
pub mod exticr3;
#[doc = "external interrupt configuration register 4"]
pub struct EXTICR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "external interrupt configuration register 4"]
pub mod exticr4;
#[doc = "SCSR"]
pub struct SCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCSR"]
pub mod scsr;
#[doc = "CFGR2"]
pub struct CFGR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CFGR2"]
pub mod cfgr2;
#[doc = "SRAM2 write protection register"]
pub struct SWPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM2 write protection register"]
pub mod swpr;
#[doc = "SKR"]
pub struct SKR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SKR"]
pub mod skr;
#[doc = "SRAM2 write protection register 2"]
pub struct SWPR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM2 write protection register 2"]
pub mod swpr2;
#[doc = "CPU1 interrupt mask register 1"]
pub struct IMR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU1 interrupt mask register 1"]
pub mod imr1;
#[doc = "CPU1 interrupt mask register 2"]
pub struct IMR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU1 interrupt mask register 2"]
pub mod imr2;
#[doc = "CPU2 interrupt mask register 1"]
pub struct C2IMR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU2 interrupt mask register 1"]
pub mod c2imr1;
#[doc = "CPU2 interrupt mask register 1"]
pub struct C2IMR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU2 interrupt mask register 1"]
pub mod c2imr2;
#[doc = "secure IP control register"]
pub struct SIPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "secure IP control register"]
pub mod sipcr;
