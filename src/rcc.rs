#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    pub cr: CR,
    #[doc = "0x04 - Internal clock sources calibration register"]
    pub icscr: ICSCR,
    #[doc = "0x08 - Clock configuration register"]
    pub cfgr: CFGR,
    #[doc = "0x0c - PLLSYS configuration register"]
    pub pllcfgr: PLLCFGR,
    #[doc = "0x10 - PLLSAI1 configuration register"]
    pub pllsai1cfgr: PLLSAI1CFGR,
    _reserved0: [u8; 4usize],
    #[doc = "0x18 - Clock interrupt enable register"]
    pub cier: CIER,
    #[doc = "0x1c - Clock interrupt flag register"]
    pub cifr: CIFR,
    #[doc = "0x20 - Clock interrupt clear register"]
    pub cicr: CICR,
    #[doc = "0x24 - Step Down converter control register"]
    pub smpscr: SMPSCR,
    #[doc = "0x28 - AHB1 peripheral reset register"]
    pub ahb1rstr: AHB1RSTR,
    #[doc = "0x2c - AHB2 peripheral reset register"]
    pub ahb2rstr: AHB2RSTR,
    #[doc = "0x30 - AHB3 peripheral reset register"]
    pub ahb3rstr: AHB3RSTR,
    _reserved1: [u8; 4usize],
    #[doc = "0x38 - APB1 peripheral reset register 1"]
    pub apb1rstr1: APB1RSTR1,
    #[doc = "0x3c - APB1 peripheral reset register 2"]
    pub apb1rstr2: APB1RSTR2,
    #[doc = "0x40 - APB2 peripheral reset register"]
    pub apb2rstr: APB2RSTR,
    #[doc = "0x44 - APB3 peripheral reset register"]
    pub apb3rstr: APB3RSTR,
    #[doc = "0x48 - AHB1 peripheral clock enable register"]
    pub ahb1enr: AHB1ENR,
    #[doc = "0x4c - AHB2 peripheral clock enable register"]
    pub ahb2enr: AHB2ENR,
    #[doc = "0x50 - AHB3 peripheral clock enable register"]
    pub ahb3enr: AHB3ENR,
    _reserved2: [u8; 4usize],
    #[doc = "0x58 - APB1ENR1"]
    pub apb1enr1: APB1ENR1,
    #[doc = "0x5c - APB1 peripheral clock enable register 2"]
    pub apb1enr2: APB1ENR2,
    #[doc = "0x60 - APB2ENR"]
    pub apb2enr: APB2ENR,
    _reserved3: [u8; 4usize],
    #[doc = "0x68 - AHB1 peripheral clocks enable in Sleep and Stop modes register"]
    pub ahb1smenr: AHB1SMENR,
    #[doc = "0x6c - AHB2 peripheral clocks enable in Sleep and Stop modes register"]
    pub ahb2smenr: AHB2SMENR,
    #[doc = "0x70 - AHB3 peripheral clocks enable in Sleep and Stop modes register"]
    pub ahb3smenr: AHB3SMENR,
    _reserved4: [u8; 4usize],
    #[doc = "0x78 - APB1SMENR1"]
    pub apb1smenr1: APB1SMENR1,
    #[doc = "0x7c - APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
    pub apb1smenr2: APB1SMENR2,
    #[doc = "0x80 - APB2SMENR"]
    pub apb2smenr: APB2SMENR,
    _reserved5: [u8; 4usize],
    #[doc = "0x88 - CCIPR"]
    pub ccipr: CCIPR,
    _reserved6: [u8; 4usize],
    #[doc = "0x90 - BDCR"]
    pub bdcr: BDCR,
    #[doc = "0x94 - CSR"]
    pub csr: CSR,
    #[doc = "0x98 - Clock recovery RC register"]
    pub crrcr: CRRCR,
    #[doc = "0x9c - Clock HSE register"]
    pub hsecr: HSECR,
    _reserved7: [u8; 104usize],
    #[doc = "0x108 - Extended clock recovery register"]
    pub extcfgr: EXTCFGR,
    _reserved8: [u8; 60usize],
    #[doc = "0x148 - CPU2 AHB1 peripheral clock enable register"]
    pub c2ahb1enr: C2AHB1ENR,
    #[doc = "0x14c - CPU2 AHB2 peripheral clock enable register"]
    pub c2ahb2enr: C2AHB2ENR,
    #[doc = "0x150 - CPU2 AHB3 peripheral clock enable register"]
    pub c2ahb3enr: C2AHB3ENR,
    _reserved9: [u8; 4usize],
    #[doc = "0x158 - CPU2 APB1ENR1"]
    pub c2apb1enr1: C2APB1ENR1,
    #[doc = "0x15c - CPU2 APB1 peripheral clock enable register 2"]
    pub c2apb1enr2: C2APB1ENR2,
    #[doc = "0x160 - CPU2 APB2ENR"]
    pub c2apb2enr: C2APB2ENR,
    #[doc = "0x164 - CPU2 APB3ENR"]
    pub c2apb3enr: C2APB3ENR,
    #[doc = "0x168 - CPU2 AHB1 peripheral clocks enable in Sleep and Stop modes register"]
    pub c2ahb1smenr: C2AHB1SMENR,
    #[doc = "0x16c - CPU2 AHB2 peripheral clocks enable in Sleep and Stop modes register"]
    pub c2ahb2smenr: C2AHB2SMENR,
    #[doc = "0x170 - CPU2 AHB3 peripheral clocks enable in Sleep and Stop modes register"]
    pub c2ahb3smenr: C2AHB3SMENR,
    _reserved10: [u8; 4usize],
    #[doc = "0x178 - CPU2 APB1SMENR1"]
    pub c2apb1smenr1: C2APB1SMENR1,
    #[doc = "0x17c - CPU2 APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
    pub c2apb1smenr2: C2APB1SMENR2,
    #[doc = "0x180 - CPU2 APB2SMENR"]
    pub c2apb2smenr: C2APB2SMENR,
    #[doc = "0x184 - CPU2 APB3SMENR"]
    pub c2apb3smenr: C2APB3SMENR,
}
#[doc = "Clock control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock control register"]
pub mod cr;
#[doc = "Internal clock sources calibration register"]
pub struct ICSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal clock sources calibration register"]
pub mod icscr;
#[doc = "Clock configuration register"]
pub struct CFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock configuration register"]
pub mod cfgr;
#[doc = "PLLSYS configuration register"]
pub struct PLLCFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLLSYS configuration register"]
pub mod pllcfgr;
#[doc = "PLLSAI1 configuration register"]
pub struct PLLSAI1CFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLLSAI1 configuration register"]
pub mod pllsai1cfgr;
#[doc = "Clock interrupt enable register"]
pub struct CIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock interrupt enable register"]
pub mod cier;
#[doc = "Clock interrupt flag register"]
pub struct CIFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock interrupt flag register"]
pub mod cifr;
#[doc = "Clock interrupt clear register"]
pub struct CICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock interrupt clear register"]
pub mod cicr;
#[doc = "Step Down converter control register"]
pub struct SMPSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Step Down converter control register"]
pub mod smpscr;
#[doc = "AHB1 peripheral reset register"]
pub struct AHB1RSTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB1 peripheral reset register"]
pub mod ahb1rstr;
#[doc = "AHB2 peripheral reset register"]
pub struct AHB2RSTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB2 peripheral reset register"]
pub mod ahb2rstr;
#[doc = "AHB3 peripheral reset register"]
pub struct AHB3RSTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB3 peripheral reset register"]
pub mod ahb3rstr;
#[doc = "APB1 peripheral reset register 1"]
pub struct APB1RSTR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB1 peripheral reset register 1"]
pub mod apb1rstr1;
#[doc = "APB1 peripheral reset register 2"]
pub struct APB1RSTR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB1 peripheral reset register 2"]
pub mod apb1rstr2;
#[doc = "APB2 peripheral reset register"]
pub struct APB2RSTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB2 peripheral reset register"]
pub mod apb2rstr;
#[doc = "APB3 peripheral reset register"]
pub struct APB3RSTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB3 peripheral reset register"]
pub mod apb3rstr;
#[doc = "AHB1 peripheral clock enable register"]
pub struct AHB1ENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB1 peripheral clock enable register"]
pub mod ahb1enr;
#[doc = "AHB2 peripheral clock enable register"]
pub struct AHB2ENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB2 peripheral clock enable register"]
pub mod ahb2enr;
#[doc = "AHB3 peripheral clock enable register"]
pub struct AHB3ENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB3 peripheral clock enable register"]
pub mod ahb3enr;
#[doc = "APB1ENR1"]
pub struct APB1ENR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB1ENR1"]
pub mod apb1enr1;
#[doc = "APB1 peripheral clock enable register 2"]
pub struct APB1ENR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB1 peripheral clock enable register 2"]
pub mod apb1enr2;
#[doc = "APB2ENR"]
pub struct APB2ENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB2ENR"]
pub mod apb2enr;
#[doc = "AHB1 peripheral clocks enable in Sleep and Stop modes register"]
pub struct AHB1SMENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB1 peripheral clocks enable in Sleep and Stop modes register"]
pub mod ahb1smenr;
#[doc = "AHB2 peripheral clocks enable in Sleep and Stop modes register"]
pub struct AHB2SMENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB2 peripheral clocks enable in Sleep and Stop modes register"]
pub mod ahb2smenr;
#[doc = "AHB3 peripheral clocks enable in Sleep and Stop modes register"]
pub struct AHB3SMENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB3 peripheral clocks enable in Sleep and Stop modes register"]
pub mod ahb3smenr;
#[doc = "APB1SMENR1"]
pub struct APB1SMENR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB1SMENR1"]
pub mod apb1smenr1;
#[doc = "APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
pub struct APB1SMENR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
pub mod apb1smenr2;
#[doc = "APB2SMENR"]
pub struct APB2SMENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB2SMENR"]
pub mod apb2smenr;
#[doc = "CCIPR"]
pub struct CCIPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCIPR"]
pub mod ccipr;
#[doc = "BDCR"]
pub struct BDCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BDCR"]
pub mod bdcr;
#[doc = "CSR"]
pub struct CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CSR"]
pub mod csr;
#[doc = "Clock recovery RC register"]
pub struct CRRCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock recovery RC register"]
pub mod crrcr;
#[doc = "Clock HSE register"]
pub struct HSECR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock HSE register"]
pub mod hsecr;
#[doc = "Extended clock recovery register"]
pub struct EXTCFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Extended clock recovery register"]
pub mod extcfgr;
#[doc = "CPU2 AHB1 peripheral clock enable register"]
pub struct C2AHB1ENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU2 AHB1 peripheral clock enable register"]
pub mod c2ahb1enr;
#[doc = "CPU2 AHB2 peripheral clock enable register"]
pub struct C2AHB2ENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU2 AHB2 peripheral clock enable register"]
pub mod c2ahb2enr;
#[doc = "CPU2 AHB3 peripheral clock enable register"]
pub struct C2AHB3ENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU2 AHB3 peripheral clock enable register"]
pub mod c2ahb3enr;
#[doc = "CPU2 APB1ENR1"]
pub struct C2APB1ENR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU2 APB1ENR1"]
pub mod c2apb1enr1;
#[doc = "CPU2 APB1 peripheral clock enable register 2"]
pub struct C2APB1ENR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU2 APB1 peripheral clock enable register 2"]
pub mod c2apb1enr2;
#[doc = "CPU2 APB2ENR"]
pub struct C2APB2ENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU2 APB2ENR"]
pub mod c2apb2enr;
#[doc = "CPU2 APB3ENR"]
pub struct C2APB3ENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU2 APB3ENR"]
pub mod c2apb3enr;
#[doc = "CPU2 AHB1 peripheral clocks enable in Sleep and Stop modes register"]
pub struct C2AHB1SMENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU2 AHB1 peripheral clocks enable in Sleep and Stop modes register"]
pub mod c2ahb1smenr;
#[doc = "CPU2 AHB2 peripheral clocks enable in Sleep and Stop modes register"]
pub struct C2AHB2SMENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU2 AHB2 peripheral clocks enable in Sleep and Stop modes register"]
pub mod c2ahb2smenr;
#[doc = "CPU2 AHB3 peripheral clocks enable in Sleep and Stop modes register"]
pub struct C2AHB3SMENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU2 AHB3 peripheral clocks enable in Sleep and Stop modes register"]
pub mod c2ahb3smenr;
#[doc = "CPU2 APB1SMENR1"]
pub struct C2APB1SMENR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU2 APB1SMENR1"]
pub mod c2apb1smenr1;
#[doc = "CPU2 APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
pub struct C2APB1SMENR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU2 APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
pub mod c2apb1smenr2;
#[doc = "CPU2 APB2SMENR"]
pub struct C2APB2SMENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU2 APB2SMENR"]
pub mod c2apb2smenr;
#[doc = "CPU2 APB3SMENR"]
pub struct C2APB3SMENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU2 APB3SMENR"]
pub mod c2apb3smenr;
