#[doc = r"Register block"]
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
    _reserved14: [u8; 72usize],
    #[doc = "0x80 - DMA Multiplexer Channel Status register"]
    pub csr: CSR,
    #[doc = "0x84 - DMA Channel Clear Flag Register"]
    pub cfr: CFR,
    _reserved16: [u8; 120usize],
    #[doc = "0x100 - DMA Request Generator 0 Control Register"]
    pub rg0cr: RG0CR,
    #[doc = "0x104 - DMA Request Generator 1 Control Register"]
    pub rg1cr: RG1CR,
    #[doc = "0x108 - DMA Request Generator 2 Control Register"]
    pub rg2cr: RG2CR,
    #[doc = "0x10c - DMA Request Generator 3 Control Register"]
    pub rg3cr: RG3CR,
    _reserved20: [u8; 48usize],
    #[doc = "0x140 - DMA Request Generator Status Register"]
    pub rgsr: RGSR,
    #[doc = "0x144 - DMA Request Generator Clear Flag Register"]
    pub rgcfr: RGCFR,
}
#[doc = "DMA Multiplexer Channel 0 Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0cr](c0cr) module"]
pub type C0CR = crate::Reg<u32, _C0CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C0CR;
#[doc = "`read()` method returns [c0cr::R](c0cr::R) reader structure"]
impl crate::Readable for C0CR {}
#[doc = "`write(|w| ..)` method takes [c0cr::W](c0cr::W) writer structure"]
impl crate::Writable for C0CR {}
#[doc = "DMA Multiplexer Channel 0 Control register"]
pub mod c0cr;
#[doc = "DMA Multiplexer Channel 1 Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1cr](c1cr) module"]
pub type C1CR = crate::Reg<u32, _C1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1CR;
#[doc = "`read()` method returns [c1cr::R](c1cr::R) reader structure"]
impl crate::Readable for C1CR {}
#[doc = "`write(|w| ..)` method takes [c1cr::W](c1cr::W) writer structure"]
impl crate::Writable for C1CR {}
#[doc = "DMA Multiplexer Channel 1 Control register"]
pub mod c1cr;
#[doc = "DMA Multiplexer Channel 2 Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2cr](c2cr) module"]
pub type C2CR = crate::Reg<u32, _C2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2CR;
#[doc = "`read()` method returns [c2cr::R](c2cr::R) reader structure"]
impl crate::Readable for C2CR {}
#[doc = "`write(|w| ..)` method takes [c2cr::W](c2cr::W) writer structure"]
impl crate::Writable for C2CR {}
#[doc = "DMA Multiplexer Channel 2 Control register"]
pub mod c2cr;
#[doc = "DMA Multiplexer Channel 3 Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c3cr](c3cr) module"]
pub type C3CR = crate::Reg<u32, _C3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C3CR;
#[doc = "`read()` method returns [c3cr::R](c3cr::R) reader structure"]
impl crate::Readable for C3CR {}
#[doc = "`write(|w| ..)` method takes [c3cr::W](c3cr::W) writer structure"]
impl crate::Writable for C3CR {}
#[doc = "DMA Multiplexer Channel 3 Control register"]
pub mod c3cr;
#[doc = "DMA Multiplexer Channel 4 Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c4cr](c4cr) module"]
pub type C4CR = crate::Reg<u32, _C4CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C4CR;
#[doc = "`read()` method returns [c4cr::R](c4cr::R) reader structure"]
impl crate::Readable for C4CR {}
#[doc = "`write(|w| ..)` method takes [c4cr::W](c4cr::W) writer structure"]
impl crate::Writable for C4CR {}
#[doc = "DMA Multiplexer Channel 4 Control register"]
pub mod c4cr;
#[doc = "DMA Multiplexer Channel 5 Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c5cr](c5cr) module"]
pub type C5CR = crate::Reg<u32, _C5CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C5CR;
#[doc = "`read()` method returns [c5cr::R](c5cr::R) reader structure"]
impl crate::Readable for C5CR {}
#[doc = "`write(|w| ..)` method takes [c5cr::W](c5cr::W) writer structure"]
impl crate::Writable for C5CR {}
#[doc = "DMA Multiplexer Channel 5 Control register"]
pub mod c5cr;
#[doc = "DMA Multiplexer Channel 6 Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c6cr](c6cr) module"]
pub type C6CR = crate::Reg<u32, _C6CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C6CR;
#[doc = "`read()` method returns [c6cr::R](c6cr::R) reader structure"]
impl crate::Readable for C6CR {}
#[doc = "`write(|w| ..)` method takes [c6cr::W](c6cr::W) writer structure"]
impl crate::Writable for C6CR {}
#[doc = "DMA Multiplexer Channel 6 Control register"]
pub mod c6cr;
#[doc = "DMA Multiplexer Channel 7 Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c7cr](c7cr) module"]
pub type C7CR = crate::Reg<u32, _C7CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C7CR;
#[doc = "`read()` method returns [c7cr::R](c7cr::R) reader structure"]
impl crate::Readable for C7CR {}
#[doc = "`write(|w| ..)` method takes [c7cr::W](c7cr::W) writer structure"]
impl crate::Writable for C7CR {}
#[doc = "DMA Multiplexer Channel 7 Control register"]
pub mod c7cr;
#[doc = "DMA Multiplexer Channel 8 Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c8cr](c8cr) module"]
pub type C8CR = crate::Reg<u32, _C8CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C8CR;
#[doc = "`read()` method returns [c8cr::R](c8cr::R) reader structure"]
impl crate::Readable for C8CR {}
#[doc = "`write(|w| ..)` method takes [c8cr::W](c8cr::W) writer structure"]
impl crate::Writable for C8CR {}
#[doc = "DMA Multiplexer Channel 8 Control register"]
pub mod c8cr;
#[doc = "DMA Multiplexer Channel 9 Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c9cr](c9cr) module"]
pub type C9CR = crate::Reg<u32, _C9CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C9CR;
#[doc = "`read()` method returns [c9cr::R](c9cr::R) reader structure"]
impl crate::Readable for C9CR {}
#[doc = "`write(|w| ..)` method takes [c9cr::W](c9cr::W) writer structure"]
impl crate::Writable for C9CR {}
#[doc = "DMA Multiplexer Channel 9 Control register"]
pub mod c9cr;
#[doc = "DMA Multiplexer Channel 10 Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c10cr](c10cr) module"]
pub type C10CR = crate::Reg<u32, _C10CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C10CR;
#[doc = "`read()` method returns [c10cr::R](c10cr::R) reader structure"]
impl crate::Readable for C10CR {}
#[doc = "`write(|w| ..)` method takes [c10cr::W](c10cr::W) writer structure"]
impl crate::Writable for C10CR {}
#[doc = "DMA Multiplexer Channel 10 Control register"]
pub mod c10cr;
#[doc = "DMA Multiplexer Channel 11 Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c11cr](c11cr) module"]
pub type C11CR = crate::Reg<u32, _C11CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C11CR;
#[doc = "`read()` method returns [c11cr::R](c11cr::R) reader structure"]
impl crate::Readable for C11CR {}
#[doc = "`write(|w| ..)` method takes [c11cr::W](c11cr::W) writer structure"]
impl crate::Writable for C11CR {}
#[doc = "DMA Multiplexer Channel 11 Control register"]
pub mod c11cr;
#[doc = "DMA Multiplexer Channel 12 Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c12cr](c12cr) module"]
pub type C12CR = crate::Reg<u32, _C12CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C12CR;
#[doc = "`read()` method returns [c12cr::R](c12cr::R) reader structure"]
impl crate::Readable for C12CR {}
#[doc = "`write(|w| ..)` method takes [c12cr::W](c12cr::W) writer structure"]
impl crate::Writable for C12CR {}
#[doc = "DMA Multiplexer Channel 12 Control register"]
pub mod c12cr;
#[doc = "DMA Multiplexer Channel 13 Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c13cr](c13cr) module"]
pub type C13CR = crate::Reg<u32, _C13CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C13CR;
#[doc = "`read()` method returns [c13cr::R](c13cr::R) reader structure"]
impl crate::Readable for C13CR {}
#[doc = "`write(|w| ..)` method takes [c13cr::W](c13cr::W) writer structure"]
impl crate::Writable for C13CR {}
#[doc = "DMA Multiplexer Channel 13 Control register"]
pub mod c13cr;
#[doc = "DMA Multiplexer Channel Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](csr) module"]
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
#[doc = "`read()` method returns [csr::R](csr::R) reader structure"]
impl crate::Readable for CSR {}
#[doc = "DMA Multiplexer Channel Status register"]
pub mod csr;
#[doc = "DMA Channel Clear Flag Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfr](cfr) module"]
pub type CFR = crate::Reg<u32, _CFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFR;
#[doc = "`write(|w| ..)` method takes [cfr::W](cfr::W) writer structure"]
impl crate::Writable for CFR {}
#[doc = "DMA Channel Clear Flag Register"]
pub mod cfr;
#[doc = "DMA Request Generator 0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rg0cr](rg0cr) module"]
pub type RG0CR = crate::Reg<u32, _RG0CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RG0CR;
#[doc = "`read()` method returns [rg0cr::R](rg0cr::R) reader structure"]
impl crate::Readable for RG0CR {}
#[doc = "`write(|w| ..)` method takes [rg0cr::W](rg0cr::W) writer structure"]
impl crate::Writable for RG0CR {}
#[doc = "DMA Request Generator 0 Control Register"]
pub mod rg0cr;
#[doc = "DMA Request Generator 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rg1cr](rg1cr) module"]
pub type RG1CR = crate::Reg<u32, _RG1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RG1CR;
#[doc = "`read()` method returns [rg1cr::R](rg1cr::R) reader structure"]
impl crate::Readable for RG1CR {}
#[doc = "`write(|w| ..)` method takes [rg1cr::W](rg1cr::W) writer structure"]
impl crate::Writable for RG1CR {}
#[doc = "DMA Request Generator 1 Control Register"]
pub mod rg1cr;
#[doc = "DMA Request Generator 2 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rg2cr](rg2cr) module"]
pub type RG2CR = crate::Reg<u32, _RG2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RG2CR;
#[doc = "`read()` method returns [rg2cr::R](rg2cr::R) reader structure"]
impl crate::Readable for RG2CR {}
#[doc = "`write(|w| ..)` method takes [rg2cr::W](rg2cr::W) writer structure"]
impl crate::Writable for RG2CR {}
#[doc = "DMA Request Generator 2 Control Register"]
pub mod rg2cr;
#[doc = "DMA Request Generator 3 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rg3cr](rg3cr) module"]
pub type RG3CR = crate::Reg<u32, _RG3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RG3CR;
#[doc = "`read()` method returns [rg3cr::R](rg3cr::R) reader structure"]
impl crate::Readable for RG3CR {}
#[doc = "`write(|w| ..)` method takes [rg3cr::W](rg3cr::W) writer structure"]
impl crate::Writable for RG3CR {}
#[doc = "DMA Request Generator 3 Control Register"]
pub mod rg3cr;
#[doc = "DMA Request Generator Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rgsr](rgsr) module"]
pub type RGSR = crate::Reg<u32, _RGSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RGSR;
#[doc = "`read()` method returns [rgsr::R](rgsr::R) reader structure"]
impl crate::Readable for RGSR {}
#[doc = "DMA Request Generator Status Register"]
pub mod rgsr;
#[doc = "DMA Request Generator Clear Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rgcfr](rgcfr) module"]
pub type RGCFR = crate::Reg<u32, _RGCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RGCFR;
#[doc = "`read()` method returns [rgcfr::R](rgcfr::R) reader structure"]
impl crate::Readable for RGCFR {}
#[doc = "DMA Request Generator Clear Flag Register"]
pub mod rgcfr;
