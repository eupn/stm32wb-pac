#[doc = "Reader of register C1ISR"]
pub type R = crate::R<u32, super::C1ISR>;
#[doc = "Reader of field `ISFm`"]
pub type ISFM_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CPU(n) semaphore m status bit before enable (mask)"]
    #[inline(always)]
    pub fn isfm(&self) -> ISFM_R {
        ISFM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
