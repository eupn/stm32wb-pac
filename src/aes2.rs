#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    #[doc = "0x04 - status register"]
    pub sr: SR,
    #[doc = "0x08 - data input register"]
    pub dinr: DINR,
    #[doc = "0x0c - data output register"]
    pub doutr: DOUTR,
    #[doc = "0x10 - key register 0"]
    pub keyr0: KEYR0,
    #[doc = "0x14 - key register 1"]
    pub keyr1: KEYR1,
    #[doc = "0x18 - key register 2"]
    pub keyr2: KEYR2,
    #[doc = "0x1c - key register 3"]
    pub keyr3: KEYR3,
    #[doc = "0x20 - initialization vector register 0"]
    pub ivr0: IVR0,
    #[doc = "0x24 - initialization vector register 1"]
    pub ivr1: IVR1,
    #[doc = "0x28 - initialization vector register 2"]
    pub ivr2: IVR2,
    #[doc = "0x2c - initialization vector register 3"]
    pub ivr3: IVR3,
    #[doc = "0x30 - key register 4"]
    pub keyr4: KEYR4,
    #[doc = "0x34 - key register 5"]
    pub keyr5: KEYR5,
    #[doc = "0x38 - key register 6"]
    pub keyr6: KEYR6,
    #[doc = "0x3c - key register 7"]
    pub keyr7: KEYR7,
    #[doc = "0x40 - AES suspend register 0"]
    pub susp0r: SUSP0R,
    #[doc = "0x44 - AES suspend register 1"]
    pub susp1r: SUSP1R,
    #[doc = "0x48 - AES suspend register 2"]
    pub susp2r: SUSP2R,
    #[doc = "0x4c - AES suspend register 3"]
    pub susp3r: SUSP3R,
    #[doc = "0x50 - AES suspend register 4"]
    pub susp4r: SUSP4R,
    #[doc = "0x54 - AES suspend register 5"]
    pub susp5r: SUSP5R,
    #[doc = "0x58 - AES suspend register 6"]
    pub susp6r: SUSP6R,
    #[doc = "0x5c - AES suspend register 7"]
    pub susp7r: SUSP7R,
    #[doc = "0x60 - AES hardware configuration register"]
    pub hwcfr: HWCFR,
    #[doc = "0x64 - AES version register"]
    pub verr: VERR,
    #[doc = "0x68 - AES identification register"]
    pub ipidr: IPIDR,
    #[doc = "0x6c - AES size ID register"]
    pub sidr: SIDR,
}
#[doc = "control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register"]
pub mod cr;
#[doc = "status register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "status register"]
pub mod sr;
#[doc = "data input register"]
pub struct DINR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data input register"]
pub mod dinr;
#[doc = "data output register"]
pub struct DOUTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data output register"]
pub mod doutr;
#[doc = "key register 0"]
pub struct KEYR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key register 0"]
pub mod keyr0;
#[doc = "key register 1"]
pub struct KEYR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key register 1"]
pub mod keyr1;
#[doc = "key register 2"]
pub struct KEYR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key register 2"]
pub mod keyr2;
#[doc = "key register 3"]
pub struct KEYR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key register 3"]
pub mod keyr3;
#[doc = "initialization vector register 0"]
pub struct IVR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "initialization vector register 0"]
pub mod ivr0;
#[doc = "initialization vector register 1"]
pub struct IVR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "initialization vector register 1"]
pub mod ivr1;
#[doc = "initialization vector register 2"]
pub struct IVR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "initialization vector register 2"]
pub mod ivr2;
#[doc = "initialization vector register 3"]
pub struct IVR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "initialization vector register 3"]
pub mod ivr3;
#[doc = "key register 4"]
pub struct KEYR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key register 4"]
pub mod keyr4;
#[doc = "key register 5"]
pub struct KEYR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key register 5"]
pub mod keyr5;
#[doc = "key register 6"]
pub struct KEYR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key register 6"]
pub mod keyr6;
#[doc = "key register 7"]
pub struct KEYR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key register 7"]
pub mod keyr7;
#[doc = "AES suspend register 0"]
pub struct SUSP0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES suspend register 0"]
pub mod susp0r;
#[doc = "AES suspend register 1"]
pub struct SUSP1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES suspend register 1"]
pub mod susp1r;
#[doc = "AES suspend register 2"]
pub struct SUSP2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES suspend register 2"]
pub mod susp2r;
#[doc = "AES suspend register 3"]
pub struct SUSP3R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES suspend register 3"]
pub mod susp3r;
#[doc = "AES suspend register 4"]
pub struct SUSP4R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES suspend register 4"]
pub mod susp4r;
#[doc = "AES suspend register 5"]
pub struct SUSP5R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES suspend register 5"]
pub mod susp5r;
#[doc = "AES suspend register 6"]
pub struct SUSP6R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES suspend register 6"]
pub mod susp6r;
#[doc = "AES suspend register 7"]
pub struct SUSP7R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES suspend register 7"]
pub mod susp7r;
#[doc = "AES hardware configuration register"]
pub struct HWCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES hardware configuration register"]
pub mod hwcfr;
#[doc = "AES version register"]
pub struct VERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES version register"]
pub mod verr;
#[doc = "AES identification register"]
pub struct IPIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES identification register"]
pub mod ipidr;
#[doc = "AES size ID register"]
pub struct SIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES size ID register"]
pub mod sidr;
