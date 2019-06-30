#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - rising trigger selection register"]
    pub rtsr1: RTSR1,
    #[doc = "0x04 - falling trigger selection register"]
    pub ftsr1: FTSR1,
    #[doc = "0x08 - software interrupt event register"]
    pub swier1: SWIER1,
    #[doc = "0x0c - EXTI pending register"]
    pub pr1: PR1,
    _reserved0: [u8; 16usize],
    #[doc = "0x20 - rising trigger selection register"]
    pub rtsr2: RTSR2,
    #[doc = "0x24 - falling trigger selection register"]
    pub ftsr2: FTSR2,
    #[doc = "0x28 - software interrupt event register"]
    pub swier2: SWIER2,
    #[doc = "0x2c - pending register"]
    pub pr2: PR2,
    _reserved1: [u8; 80usize],
    #[doc = "0x80 - CPUm wakeup with interrupt mask register"]
    pub c1imr1: C1IMR1,
    #[doc = "0x84 - CPUm wakeup with event mask register"]
    pub c1emr1: C1EMR1,
    _reserved2: [u8; 8usize],
    #[doc = "0x90 - CPUm wakeup with interrupt mask register"]
    pub c1imr2: C1IMR2,
    #[doc = "0x94 - CPUm wakeup with event mask register"]
    pub c1emr2: C1EMR2,
    _reserved3: [u8; 40usize],
    #[doc = "0xc0 - CPUm wakeup with interrupt mask register"]
    pub c2imr1: C2IMR1,
    #[doc = "0xc4 - CPUm wakeup with event mask register"]
    pub c2emr1: C2EMR1,
    _reserved4: [u8; 8usize],
    #[doc = "0xd0 - CPUm wakeup with interrupt mask register"]
    pub c2imr2: C2IMR2,
    #[doc = "0xd4 - CPUm wakeup with event mask register"]
    pub c2emr2: C2EMR2,
    _reserved5: [u8; 768usize],
    #[doc = "0x3d8 - EXTI Hardware configuration registers"]
    pub hwcfgr7: HWCFGR7,
    #[doc = "0x3dc - Hardware configuration registers"]
    pub hwcfgr6: HWCFGR6,
    #[doc = "0x3e0 - Hardware configuration registers"]
    pub hwcfgr5: HWCFGR5,
    #[doc = "0x3e4 - Hardware configuration registers"]
    pub hwcfgr4: HWCFGR4,
    #[doc = "0x3e8 - Hardware configuration registers"]
    pub hwcfgr3: HWCFGR3,
    #[doc = "0x3ec - Hardware configuration registers"]
    pub hwcfgr2: HWCFGR2,
    #[doc = "0x3f0 - Hardware configuration register 1"]
    pub hwcfgr1: HWCFGR1,
    #[doc = "0x3f4 - EXTI IP Version register"]
    pub verr: VERR,
    #[doc = "0x3f8 - Identification register"]
    pub ipidr: IPIDR,
    #[doc = "0x3fc - Size ID register"]
    pub sidr: SIDR,
}
#[doc = "rising trigger selection register"]
pub struct RTSR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "rising trigger selection register"]
pub mod rtsr1;
#[doc = "falling trigger selection register"]
pub struct FTSR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "falling trigger selection register"]
pub mod ftsr1;
#[doc = "software interrupt event register"]
pub struct SWIER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "software interrupt event register"]
pub mod swier1;
#[doc = "EXTI pending register"]
pub struct PR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI pending register"]
pub mod pr1;
#[doc = "rising trigger selection register"]
pub struct RTSR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "rising trigger selection register"]
pub mod rtsr2;
#[doc = "falling trigger selection register"]
pub struct FTSR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "falling trigger selection register"]
pub mod ftsr2;
#[doc = "software interrupt event register"]
pub struct SWIER2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "software interrupt event register"]
pub mod swier2;
#[doc = "pending register"]
pub struct PR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "pending register"]
pub mod pr2;
#[doc = "CPUm wakeup with interrupt mask register"]
pub struct C1IMR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPUm wakeup with interrupt mask register"]
pub mod c1imr1;
#[doc = "CPUm wakeup with interrupt mask register"]
pub struct C2IMR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPUm wakeup with interrupt mask register"]
pub mod c2imr1;
#[doc = "CPUm wakeup with event mask register"]
pub struct C1EMR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPUm wakeup with event mask register"]
pub mod c1emr1;
#[doc = "CPUm wakeup with event mask register"]
pub struct C2EMR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPUm wakeup with event mask register"]
pub mod c2emr1;
#[doc = "CPUm wakeup with interrupt mask register"]
pub struct C1IMR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPUm wakeup with interrupt mask register"]
pub mod c1imr2;
#[doc = "CPUm wakeup with interrupt mask register"]
pub struct C2IMR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPUm wakeup with interrupt mask register"]
pub mod c2imr2;
#[doc = "CPUm wakeup with event mask register"]
pub struct C1EMR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPUm wakeup with event mask register"]
pub mod c1emr2;
#[doc = "CPUm wakeup with event mask register"]
pub struct C2EMR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPUm wakeup with event mask register"]
pub mod c2emr2;
#[doc = "Hardware configuration registers"]
pub struct HWCFGR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware configuration registers"]
pub mod hwcfgr5;
#[doc = "Hardware configuration registers"]
pub struct HWCFGR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware configuration registers"]
pub mod hwcfgr6;
#[doc = "EXTI Hardware configuration registers"]
pub struct HWCFGR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI Hardware configuration registers"]
pub mod hwcfgr7;
#[doc = "Hardware configuration registers"]
pub struct HWCFGR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware configuration registers"]
pub mod hwcfgr2;
#[doc = "Hardware configuration registers"]
pub struct HWCFGR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware configuration registers"]
pub mod hwcfgr3;
#[doc = "Hardware configuration registers"]
pub struct HWCFGR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware configuration registers"]
pub mod hwcfgr4;
#[doc = "Hardware configuration register 1"]
pub struct HWCFGR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware configuration register 1"]
pub mod hwcfgr1;
#[doc = "EXTI IP Version register"]
pub struct VERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI IP Version register"]
pub mod verr;
#[doc = "Identification register"]
pub struct IPIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Identification register"]
pub mod ipidr;
#[doc = "Size ID register"]
pub struct SIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Size ID register"]
pub mod sidr;
