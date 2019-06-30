#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HWCFGR1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct NBINTR {
    bits: u8,
}
impl NBINTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NBSEMR {
    bits: u8,
}
impl NBSEMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 8:11 - Hardware Configuration number of interrupts supported number of master IDs"]
    #[inline]
    pub fn nbint(&self) -> NBINTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NBINTR { bits }
    }
    #[doc = "Bits 0:7 - Hardware Configuration number of semaphores"]
    #[inline]
    pub fn nbsem(&self) -> NBSEMR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NBSEMR { bits }
    }
}
