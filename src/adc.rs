#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC interrupt and status register"]
    pub isr: ISR,
    #[doc = "0x04 - ADC interrupt enable register"]
    pub ier: IER,
    #[doc = "0x08 - ADC control register"]
    pub cr: CR,
    #[doc = "0x0c - ADC configuration register 1"]
    pub cfgr: CFGR,
    #[doc = "0x10 - ADC configuration register 2"]
    pub cfgr2: CFGR2,
    #[doc = "0x14 - ADC sampling time register 1"]
    pub smpr1: SMPR1,
    #[doc = "0x18 - ADC sampling time register 2"]
    pub smpr2: SMPR2,
    _reserved0: [u8; 4usize],
    #[doc = "0x20 - ADC analog watchdog 1 threshold register"]
    pub tr1: TR1,
    #[doc = "0x24 - ADC analog watchdog 2 threshold register"]
    pub tr2: TR2,
    #[doc = "0x28 - ADC analog watchdog 3 threshold register"]
    pub tr3: TR3,
    _reserved1: [u8; 4usize],
    #[doc = "0x30 - ADC group regular sequencer ranks register 1"]
    pub sqr1: SQR1,
    #[doc = "0x34 - ADC group regular sequencer ranks register 2"]
    pub sqr2: SQR2,
    #[doc = "0x38 - ADC group regular sequencer ranks register 3"]
    pub sqr3: SQR3,
    #[doc = "0x3c - ADC group regular sequencer ranks register 4"]
    pub sqr4: SQR4,
    #[doc = "0x40 - ADC group regular conversion data register"]
    pub dr: DR,
    _reserved2: [u8; 8usize],
    #[doc = "0x4c - ADC group injected sequencer register"]
    pub jsqr: JSQR,
    _reserved3: [u8; 16usize],
    #[doc = "0x60 - ADC offset number 1 register"]
    pub ofr1: OFR1,
    #[doc = "0x64 - ADC offset number 2 register"]
    pub ofr2: OFR2,
    #[doc = "0x68 - ADC offset number 3 register"]
    pub ofr3: OFR3,
    #[doc = "0x6c - ADC offset number 4 register"]
    pub ofr4: OFR4,
    _reserved4: [u8; 16usize],
    #[doc = "0x80 - ADC group injected sequencer rank 1 register"]
    pub jdr1: JDR1,
    #[doc = "0x84 - ADC group injected sequencer rank 2 register"]
    pub jdr2: JDR2,
    #[doc = "0x88 - ADC group injected sequencer rank 3 register"]
    pub jdr3: JDR3,
    #[doc = "0x8c - ADC group injected sequencer rank 4 register"]
    pub jdr4: JDR4,
    _reserved5: [u8; 16usize],
    #[doc = "0xa0 - ADC analog watchdog 2 configuration register"]
    pub awd2cr: AWD2CR,
    #[doc = "0xa4 - ADC analog watchdog 3 configuration register"]
    pub awd3cr: AWD3CR,
    _reserved6: [u8; 8usize],
    #[doc = "0xb0 - ADC channel differential or single-ended mode selection register"]
    pub difsel: DIFSEL,
    #[doc = "0xb4 - ADC calibration factors register"]
    pub calfact: CALFACT,
    _reserved7: [u8; 592usize],
    #[doc = "0x308 - ADC common control register"]
    pub ccr: CCR,
}
#[doc = "ADC interrupt and status register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC interrupt and status register"]
pub mod isr;
#[doc = "ADC interrupt enable register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC interrupt enable register"]
pub mod ier;
#[doc = "ADC control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC control register"]
pub mod cr;
#[doc = "ADC configuration register 1"]
pub struct CFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC configuration register 1"]
pub mod cfgr;
#[doc = "ADC configuration register 2"]
pub struct CFGR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC configuration register 2"]
pub mod cfgr2;
#[doc = "ADC sampling time register 1"]
pub struct SMPR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC sampling time register 1"]
pub mod smpr1;
#[doc = "ADC sampling time register 2"]
pub struct SMPR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC sampling time register 2"]
pub mod smpr2;
#[doc = "ADC analog watchdog 1 threshold register"]
pub struct TR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC analog watchdog 1 threshold register"]
pub mod tr1;
#[doc = "ADC analog watchdog 2 threshold register"]
pub struct TR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC analog watchdog 2 threshold register"]
pub mod tr2;
#[doc = "ADC analog watchdog 3 threshold register"]
pub struct TR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC analog watchdog 3 threshold register"]
pub mod tr3;
#[doc = "ADC group regular sequencer ranks register 1"]
pub struct SQR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC group regular sequencer ranks register 1"]
pub mod sqr1;
#[doc = "ADC group regular sequencer ranks register 2"]
pub struct SQR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC group regular sequencer ranks register 2"]
pub mod sqr2;
#[doc = "ADC group regular sequencer ranks register 3"]
pub struct SQR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC group regular sequencer ranks register 3"]
pub mod sqr3;
#[doc = "ADC group regular sequencer ranks register 4"]
pub struct SQR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC group regular sequencer ranks register 4"]
pub mod sqr4;
#[doc = "ADC group regular conversion data register"]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC group regular conversion data register"]
pub mod dr;
#[doc = "ADC group injected sequencer register"]
pub struct JSQR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC group injected sequencer register"]
pub mod jsqr;
#[doc = "ADC offset number 1 register"]
pub struct OFR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC offset number 1 register"]
pub mod ofr1;
#[doc = "ADC offset number 2 register"]
pub struct OFR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC offset number 2 register"]
pub mod ofr2;
#[doc = "ADC offset number 3 register"]
pub struct OFR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC offset number 3 register"]
pub mod ofr3;
#[doc = "ADC offset number 4 register"]
pub struct OFR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC offset number 4 register"]
pub mod ofr4;
#[doc = "ADC group injected sequencer rank 1 register"]
pub struct JDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC group injected sequencer rank 1 register"]
pub mod jdr1;
#[doc = "ADC group injected sequencer rank 2 register"]
pub struct JDR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC group injected sequencer rank 2 register"]
pub mod jdr2;
#[doc = "ADC group injected sequencer rank 3 register"]
pub struct JDR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC group injected sequencer rank 3 register"]
pub mod jdr3;
#[doc = "ADC group injected sequencer rank 4 register"]
pub struct JDR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC group injected sequencer rank 4 register"]
pub mod jdr4;
#[doc = "ADC analog watchdog 2 configuration register"]
pub struct AWD2CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC analog watchdog 2 configuration register"]
pub mod awd2cr;
#[doc = "ADC analog watchdog 3 configuration register"]
pub struct AWD3CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC analog watchdog 3 configuration register"]
pub mod awd3cr;
#[doc = "ADC channel differential or single-ended mode selection register"]
pub struct DIFSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC channel differential or single-ended mode selection register"]
pub mod difsel;
#[doc = "ADC calibration factors register"]
pub struct CALFACT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC calibration factors register"]
pub mod calfact;
#[doc = "ADC common control register"]
pub struct CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC common control register"]
pub mod ccr;
