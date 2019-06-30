#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HWCFGR2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct EVENT_TRGR {
    bits: u32,
}
impl EVENT_TRGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - HW configuration event trigger type"]
    #[inline]
    pub fn event_trg(&self) -> EVENT_TRGR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        EVENT_TRGR { bits }
    }
}
