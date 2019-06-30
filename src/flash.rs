#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access control register"]
    pub acr: ACR,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - Flash key register"]
    pub keyr: KEYR,
    #[doc = "0x0c - Option byte key register"]
    pub optkeyr: OPTKEYR,
    #[doc = "0x10 - Status register"]
    pub sr: SR,
    #[doc = "0x14 - Flash control register"]
    pub cr: CR,
    #[doc = "0x18 - Flash ECC register"]
    pub eccr: ECCR,
    _reserved1: [u8; 4usize],
    #[doc = "0x20 - Flash option register"]
    pub optr: OPTR,
    #[doc = "0x24 - Flash Bank 1 PCROP Start address zone A register"]
    pub pcrop1asr: PCROP1ASR,
    #[doc = "0x28 - Flash Bank 1 PCROP End address zone A register"]
    pub pcrop1aer: PCROP1AER,
    #[doc = "0x2c - Flash Bank 1 WRP area A address register"]
    pub wrp1ar: WRP1AR,
    #[doc = "0x30 - Flash Bank 1 WRP area B address register"]
    pub wrp1br: WRP1BR,
    #[doc = "0x34 - Flash Bank 1 PCROP Start address area B register"]
    pub pcrop1bsr: PCROP1BSR,
    #[doc = "0x38 - Flash Bank 1 PCROP End address area B register"]
    pub pcrop1ber: PCROP1BER,
    #[doc = "0x3c - IPCC mailbox data buffer address register"]
    pub ipccbr: IPCCBR,
    _reserved2: [u8; 28usize],
    #[doc = "0x5c - CPU2 cortex M0 access control register"]
    pub c2acr: C2ACR,
    #[doc = "0x60 - CPU2 cortex M0 status register"]
    pub c2sr: C2SR,
    #[doc = "0x64 - CPU2 cortex M0 control register"]
    pub c2cr: C2CR,
    _reserved3: [u8; 24usize],
    #[doc = "0x80 - Secure flash start address register"]
    pub sfr: SFR,
    #[doc = "0x84 - Secure SRAM2 start address and cortex M0 reset vector register"]
    pub srrvr: SRRVR,
}
#[doc = "Access control register"]
pub struct ACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access control register"]
pub mod acr;
#[doc = "Flash key register"]
pub struct KEYR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash key register"]
pub mod keyr;
#[doc = "Option byte key register"]
pub struct OPTKEYR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Option byte key register"]
pub mod optkeyr;
#[doc = "Status register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register"]
pub mod sr;
#[doc = "Flash control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash control register"]
pub mod cr;
#[doc = "Flash ECC register"]
pub struct ECCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash ECC register"]
pub mod eccr;
#[doc = "Flash option register"]
pub struct OPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash option register"]
pub mod optr;
#[doc = "Flash Bank 1 PCROP Start address zone A register"]
pub struct PCROP1ASR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Bank 1 PCROP Start address zone A register"]
pub mod pcrop1asr;
#[doc = "Flash Bank 1 PCROP End address zone A register"]
pub struct PCROP1AER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Bank 1 PCROP End address zone A register"]
pub mod pcrop1aer;
#[doc = "Flash Bank 1 WRP area A address register"]
pub struct WRP1AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Bank 1 WRP area A address register"]
pub mod wrp1ar;
#[doc = "Flash Bank 1 WRP area B address register"]
pub struct WRP1BR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Bank 1 WRP area B address register"]
pub mod wrp1br;
#[doc = "Flash Bank 1 PCROP Start address area B register"]
pub struct PCROP1BSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Bank 1 PCROP Start address area B register"]
pub mod pcrop1bsr;
#[doc = "Flash Bank 1 PCROP End address area B register"]
pub struct PCROP1BER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Bank 1 PCROP End address area B register"]
pub mod pcrop1ber;
#[doc = "IPCC mailbox data buffer address register"]
pub struct IPCCBR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPCC mailbox data buffer address register"]
pub mod ipccbr;
#[doc = "CPU2 cortex M0 access control register"]
pub struct C2ACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU2 cortex M0 access control register"]
pub mod c2acr;
#[doc = "CPU2 cortex M0 status register"]
pub struct C2SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU2 cortex M0 status register"]
pub mod c2sr;
#[doc = "CPU2 cortex M0 control register"]
pub struct C2CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU2 cortex M0 control register"]
pub mod c2cr;
#[doc = "Secure flash start address register"]
pub struct SFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Secure flash start address register"]
pub mod sfr;
#[doc = "Secure SRAM2 start address and cortex M0 reset vector register"]
pub struct SRRVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Secure SRAM2 start address and cortex M0 reset vector register"]
pub mod srrvr;
