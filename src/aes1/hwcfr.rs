#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HWCFR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CFG4R {
    bits: u8,
}
impl CFG4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CFG3R {
    bits: u8,
}
impl CFG3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CFG2R {
    bits: u8,
}
impl CFG2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CFG1R {
    bits: u8,
}
impl CFG1R {
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
    #[doc = "Bits 12:15 - HW Generic 4"]
    #[inline]
    pub fn cfg4(&self) -> CFG4R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CFG4R { bits }
    }
    #[doc = "Bits 8:11 - HW Generic 3"]
    #[inline]
    pub fn cfg3(&self) -> CFG3R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CFG3R { bits }
    }
    #[doc = "Bits 4:7 - HW Generic 2"]
    #[inline]
    pub fn cfg2(&self) -> CFG2R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CFG2R { bits }
    }
    #[doc = "Bits 0:3 - HW Generic 1"]
    #[inline]
    pub fn cfg1(&self) -> CFG1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CFG1R { bits }
    }
}
