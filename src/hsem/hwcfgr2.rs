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
pub struct MASTERID4R {
    bits: u8,
}
impl MASTERID4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MASTERID3R {
    bits: u8,
}
impl MASTERID3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MASTERID2R {
    bits: u8,
}
impl MASTERID2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MASTERID1R {
    bits: u8,
}
impl MASTERID1R {
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
    #[doc = "Bits 12:15 - Hardware Configuration valid bus masters ID4"]
    #[inline]
    pub fn masterid4(&self) -> MASTERID4R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MASTERID4R { bits }
    }
    #[doc = "Bits 8:11 - Hardware Configuration valid bus masters ID3"]
    #[inline]
    pub fn masterid3(&self) -> MASTERID3R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MASTERID3R { bits }
    }
    #[doc = "Bits 4:7 - Hardware Configuration valid bus masters ID2"]
    #[inline]
    pub fn masterid2(&self) -> MASTERID2R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MASTERID2R { bits }
    }
    #[doc = "Bits 0:3 - Hardware Configuration valid bus masters ID1"]
    #[inline]
    pub fn masterid1(&self) -> MASTERID1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MASTERID1R { bits }
    }
}
