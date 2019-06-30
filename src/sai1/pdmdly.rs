#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PDMDLY {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct DLYM4RR {
    bits: u8,
}
impl DLYM4RR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLYM4LR {
    bits: u8,
}
impl DLYM4LR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLYM3RR {
    bits: u8,
}
impl DLYM3RR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLYM3LR {
    bits: u8,
}
impl DLYM3LR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLYM2RR {
    bits: u8,
}
impl DLYM2RR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLYM2LR {
    bits: u8,
}
impl DLYM2LR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLYM1RR {
    bits: u8,
}
impl DLYM1RR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLYM1LR {
    bits: u8,
}
impl DLYM1LR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DLYM4RW<'a> {
    w: &'a mut W,
}
impl<'a> _DLYM4RW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DLYM4LW<'a> {
    w: &'a mut W,
}
impl<'a> _DLYM4LW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DLYM3RW<'a> {
    w: &'a mut W,
}
impl<'a> _DLYM3RW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DLYM3LW<'a> {
    w: &'a mut W,
}
impl<'a> _DLYM3LW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DLYM2RW<'a> {
    w: &'a mut W,
}
impl<'a> _DLYM2RW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DLYM2LW<'a> {
    w: &'a mut W,
}
impl<'a> _DLYM2LW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DLYM1RW<'a> {
    w: &'a mut W,
}
impl<'a> _DLYM1RW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DLYM1LW<'a> {
    w: &'a mut W,
}
impl<'a> _DLYM1LW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 28:30 - Delay line for second microphone of pair 4"]
    #[inline]
    pub fn dlym4r(&self) -> DLYM4RR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLYM4RR { bits }
    }
    #[doc = "Bits 24:26 - Delay line for first microphone of pair 4"]
    #[inline]
    pub fn dlym4l(&self) -> DLYM4LR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLYM4LR { bits }
    }
    #[doc = "Bits 20:22 - Delay line for second microphone of pair 3"]
    #[inline]
    pub fn dlym3r(&self) -> DLYM3RR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLYM3RR { bits }
    }
    #[doc = "Bits 16:18 - Delay line for first microphone of pair 3"]
    #[inline]
    pub fn dlym3l(&self) -> DLYM3LR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLYM3LR { bits }
    }
    #[doc = "Bits 12:14 - Delay line for second microphone of pair 2"]
    #[inline]
    pub fn dlym2r(&self) -> DLYM2RR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLYM2RR { bits }
    }
    #[doc = "Bits 8:10 - Delay line for first microphone of pair 2"]
    #[inline]
    pub fn dlym2l(&self) -> DLYM2LR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLYM2LR { bits }
    }
    #[doc = "Bits 4:6 - Delay line for second microphone of pair 1"]
    #[inline]
    pub fn dlym1r(&self) -> DLYM1RR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLYM1RR { bits }
    }
    #[doc = "Bits 0:2 - Delay line for first microphone of pair 1"]
    #[inline]
    pub fn dlym1l(&self) -> DLYM1LR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLYM1LR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 28:30 - Delay line for second microphone of pair 4"]
    #[inline]
    pub fn dlym4r(&mut self) -> _DLYM4RW {
        _DLYM4RW { w: self }
    }
    #[doc = "Bits 24:26 - Delay line for first microphone of pair 4"]
    #[inline]
    pub fn dlym4l(&mut self) -> _DLYM4LW {
        _DLYM4LW { w: self }
    }
    #[doc = "Bits 20:22 - Delay line for second microphone of pair 3"]
    #[inline]
    pub fn dlym3r(&mut self) -> _DLYM3RW {
        _DLYM3RW { w: self }
    }
    #[doc = "Bits 16:18 - Delay line for first microphone of pair 3"]
    #[inline]
    pub fn dlym3l(&mut self) -> _DLYM3LW {
        _DLYM3LW { w: self }
    }
    #[doc = "Bits 12:14 - Delay line for second microphone of pair 2"]
    #[inline]
    pub fn dlym2r(&mut self) -> _DLYM2RW {
        _DLYM2RW { w: self }
    }
    #[doc = "Bits 8:10 - Delay line for first microphone of pair 2"]
    #[inline]
    pub fn dlym2l(&mut self) -> _DLYM2LW {
        _DLYM2LW { w: self }
    }
    #[doc = "Bits 4:6 - Delay line for second microphone of pair 1"]
    #[inline]
    pub fn dlym1r(&mut self) -> _DLYM1RW {
        _DLYM1RW { w: self }
    }
    #[doc = "Bits 0:2 - Delay line for first microphone of pair 1"]
    #[inline]
    pub fn dlym1l(&mut self) -> _DLYM1LW {
        _DLYM1LW { w: self }
    }
}
