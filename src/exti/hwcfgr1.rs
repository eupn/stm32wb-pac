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
pub struct NBEVENTSR {
    bits: u8,
}
impl NBEVENTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NBCPUSR {
    bits: u8,
}
impl NBCPUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CPUEVTENR {
    bits: u8,
}
impl CPUEVTENR {
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
    #[doc = "Bits 0:7 - HW configuration number of event"]
    #[inline]
    pub fn nbevents(&self) -> NBEVENTSR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NBEVENTSR { bits }
    }
    #[doc = "Bits 8:11 - HW configuration number of CPUs"]
    #[inline]
    pub fn nbcpus(&self) -> NBCPUSR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NBCPUSR { bits }
    }
    #[doc = "Bits 12:15 - HW configuration of CPU(m) event output enable"]
    #[inline]
    pub fn cpuevten(&self) -> CPUEVTENR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CPUEVTENR { bits }
    }
}
