#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::C2IMR2 {
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
pub struct DMA1_CH1_IMR {
    bits: bool,
}
impl DMA1_CH1_IMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DMA1_CH2_IMR {
    bits: bool,
}
impl DMA1_CH2_IMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DMA1_CH3_IMR {
    bits: bool,
}
impl DMA1_CH3_IMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DMA1_CH4_IMR {
    bits: bool,
}
impl DMA1_CH4_IMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DMA1_CH5_IMR {
    bits: bool,
}
impl DMA1_CH5_IMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DMA1_CH6_IMR {
    bits: bool,
}
impl DMA1_CH6_IMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DMA1_CH7_IMR {
    bits: bool,
}
impl DMA1_CH7_IMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DMA2_CH1_IMR {
    bits: bool,
}
impl DMA2_CH1_IMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DMA2_CH2_IMR {
    bits: bool,
}
impl DMA2_CH2_IMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DMA2_CH3_IMR {
    bits: bool,
}
impl DMA2_CH3_IMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DMA2_CH4_IMR {
    bits: bool,
}
impl DMA2_CH4_IMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DMA2_CH5_IMR {
    bits: bool,
}
impl DMA2_CH5_IMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DMA2_CH6_IMR {
    bits: bool,
}
impl DMA2_CH6_IMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DMA2_CH7_IMR {
    bits: bool,
}
impl DMA2_CH7_IMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DMAM_UX1_IMR {
    bits: bool,
}
impl DMAM_UX1_IMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct PVM1IMR {
    bits: bool,
}
impl PVM1IMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct PVM3IMR {
    bits: bool,
}
impl PVM3IMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct PVDIMR {
    bits: bool,
}
impl PVDIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct TSCIMR {
    bits: bool,
}
impl TSCIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct LCDIMR {
    bits: bool,
}
impl LCDIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Proxy"]
pub struct _DMA1_CH1_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA1_CH1_IMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMA1_CH2_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA1_CH2_IMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMA1_CH3_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA1_CH3_IMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMA1_CH4_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA1_CH4_IMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMA1_CH5_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA1_CH5_IMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMA1_CH6_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA1_CH6_IMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMA1_CH7_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA1_CH7_IMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMA2_CH1_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA2_CH1_IMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMA2_CH2_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA2_CH2_IMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMA2_CH3_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA2_CH3_IMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMA2_CH4_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA2_CH4_IMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMA2_CH5_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA2_CH5_IMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMA2_CH6_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA2_CH6_IMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMA2_CH7_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA2_CH7_IMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMAM_UX1_IMW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAM_UX1_IMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PVM1IMW<'a> {
    w: &'a mut W,
}
impl<'a> _PVM1IMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PVM3IMW<'a> {
    w: &'a mut W,
}
impl<'a> _PVM3IMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PVDIMW<'a> {
    w: &'a mut W,
}
impl<'a> _PVDIMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TSCIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TSCIMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LCDIMW<'a> {
    w: &'a mut W,
}
impl<'a> _LCDIMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
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
    #[doc = "Bit 0 - Peripheral DMA1 CH1 interrupt mask to CPU2"]
    #[inline]
    pub fn dma1_ch1_im(&self) -> DMA1_CH1_IMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA1_CH1_IMR { bits }
    }
    #[doc = "Bit 1 - Peripheral DMA1 CH2 interrupt mask to CPU2"]
    #[inline]
    pub fn dma1_ch2_im(&self) -> DMA1_CH2_IMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA1_CH2_IMR { bits }
    }
    #[doc = "Bit 2 - Peripheral DMA1 CH3 interrupt mask to CPU2"]
    #[inline]
    pub fn dma1_ch3_im(&self) -> DMA1_CH3_IMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA1_CH3_IMR { bits }
    }
    #[doc = "Bit 3 - Peripheral DMA1 CH4 interrupt mask to CPU2"]
    #[inline]
    pub fn dma1_ch4_im(&self) -> DMA1_CH4_IMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA1_CH4_IMR { bits }
    }
    #[doc = "Bit 4 - Peripheral DMA1 CH5 interrupt mask to CPU2"]
    #[inline]
    pub fn dma1_ch5_im(&self) -> DMA1_CH5_IMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA1_CH5_IMR { bits }
    }
    #[doc = "Bit 5 - Peripheral DMA1 CH6 interrupt mask to CPU2"]
    #[inline]
    pub fn dma1_ch6_im(&self) -> DMA1_CH6_IMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA1_CH6_IMR { bits }
    }
    #[doc = "Bit 6 - Peripheral DMA1 CH7 interrupt mask to CPU2"]
    #[inline]
    pub fn dma1_ch7_im(&self) -> DMA1_CH7_IMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA1_CH7_IMR { bits }
    }
    #[doc = "Bit 8 - Peripheral DMA2 CH1 interrupt mask to CPU1"]
    #[inline]
    pub fn dma2_ch1_im(&self) -> DMA2_CH1_IMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA2_CH1_IMR { bits }
    }
    #[doc = "Bit 9 - Peripheral DMA2 CH2 interrupt mask to CPU1"]
    #[inline]
    pub fn dma2_ch2_im(&self) -> DMA2_CH2_IMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA2_CH2_IMR { bits }
    }
    #[doc = "Bit 10 - Peripheral DMA2 CH3 interrupt mask to CPU1"]
    #[inline]
    pub fn dma2_ch3_im(&self) -> DMA2_CH3_IMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA2_CH3_IMR { bits }
    }
    #[doc = "Bit 11 - Peripheral DMA2 CH4 interrupt mask to CPU1"]
    #[inline]
    pub fn dma2_ch4_im(&self) -> DMA2_CH4_IMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA2_CH4_IMR { bits }
    }
    #[doc = "Bit 12 - Peripheral DMA2 CH5 interrupt mask to CPU1"]
    #[inline]
    pub fn dma2_ch5_im(&self) -> DMA2_CH5_IMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA2_CH5_IMR { bits }
    }
    #[doc = "Bit 13 - Peripheral DMA2 CH6 interrupt mask to CPU1"]
    #[inline]
    pub fn dma2_ch6_im(&self) -> DMA2_CH6_IMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA2_CH6_IMR { bits }
    }
    #[doc = "Bit 14 - Peripheral DMA2 CH7 interrupt mask to CPU1"]
    #[inline]
    pub fn dma2_ch7_im(&self) -> DMA2_CH7_IMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA2_CH7_IMR { bits }
    }
    #[doc = "Bit 15 - Peripheral DMAM UX1 interrupt mask to CPU1"]
    #[inline]
    pub fn dmam_ux1_im(&self) -> DMAM_UX1_IMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMAM_UX1_IMR { bits }
    }
    #[doc = "Bit 16 - Peripheral PVM1IM interrupt mask to CPU1"]
    #[inline]
    pub fn pvm1im(&self) -> PVM1IMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PVM1IMR { bits }
    }
    #[doc = "Bit 18 - Peripheral PVM3IM interrupt mask to CPU1"]
    #[inline]
    pub fn pvm3im(&self) -> PVM3IMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PVM3IMR { bits }
    }
    #[doc = "Bit 20 - Peripheral PVDIM interrupt mask to CPU1"]
    #[inline]
    pub fn pvdim(&self) -> PVDIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PVDIMR { bits }
    }
    #[doc = "Bit 21 - Peripheral TSCIM interrupt mask to CPU1"]
    #[inline]
    pub fn tscim(&self) -> TSCIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSCIMR { bits }
    }
    #[doc = "Bit 22 - Peripheral LCDIM interrupt mask to CPU1"]
    #[inline]
    pub fn lcdim(&self) -> LCDIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LCDIMR { bits }
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
    #[doc = "Bit 0 - Peripheral DMA1 CH1 interrupt mask to CPU2"]
    #[inline]
    pub fn dma1_ch1_im(&mut self) -> _DMA1_CH1_IMW {
        _DMA1_CH1_IMW { w: self }
    }
    #[doc = "Bit 1 - Peripheral DMA1 CH2 interrupt mask to CPU2"]
    #[inline]
    pub fn dma1_ch2_im(&mut self) -> _DMA1_CH2_IMW {
        _DMA1_CH2_IMW { w: self }
    }
    #[doc = "Bit 2 - Peripheral DMA1 CH3 interrupt mask to CPU2"]
    #[inline]
    pub fn dma1_ch3_im(&mut self) -> _DMA1_CH3_IMW {
        _DMA1_CH3_IMW { w: self }
    }
    #[doc = "Bit 3 - Peripheral DMA1 CH4 interrupt mask to CPU2"]
    #[inline]
    pub fn dma1_ch4_im(&mut self) -> _DMA1_CH4_IMW {
        _DMA1_CH4_IMW { w: self }
    }
    #[doc = "Bit 4 - Peripheral DMA1 CH5 interrupt mask to CPU2"]
    #[inline]
    pub fn dma1_ch5_im(&mut self) -> _DMA1_CH5_IMW {
        _DMA1_CH5_IMW { w: self }
    }
    #[doc = "Bit 5 - Peripheral DMA1 CH6 interrupt mask to CPU2"]
    #[inline]
    pub fn dma1_ch6_im(&mut self) -> _DMA1_CH6_IMW {
        _DMA1_CH6_IMW { w: self }
    }
    #[doc = "Bit 6 - Peripheral DMA1 CH7 interrupt mask to CPU2"]
    #[inline]
    pub fn dma1_ch7_im(&mut self) -> _DMA1_CH7_IMW {
        _DMA1_CH7_IMW { w: self }
    }
    #[doc = "Bit 8 - Peripheral DMA2 CH1 interrupt mask to CPU1"]
    #[inline]
    pub fn dma2_ch1_im(&mut self) -> _DMA2_CH1_IMW {
        _DMA2_CH1_IMW { w: self }
    }
    #[doc = "Bit 9 - Peripheral DMA2 CH2 interrupt mask to CPU1"]
    #[inline]
    pub fn dma2_ch2_im(&mut self) -> _DMA2_CH2_IMW {
        _DMA2_CH2_IMW { w: self }
    }
    #[doc = "Bit 10 - Peripheral DMA2 CH3 interrupt mask to CPU1"]
    #[inline]
    pub fn dma2_ch3_im(&mut self) -> _DMA2_CH3_IMW {
        _DMA2_CH3_IMW { w: self }
    }
    #[doc = "Bit 11 - Peripheral DMA2 CH4 interrupt mask to CPU1"]
    #[inline]
    pub fn dma2_ch4_im(&mut self) -> _DMA2_CH4_IMW {
        _DMA2_CH4_IMW { w: self }
    }
    #[doc = "Bit 12 - Peripheral DMA2 CH5 interrupt mask to CPU1"]
    #[inline]
    pub fn dma2_ch5_im(&mut self) -> _DMA2_CH5_IMW {
        _DMA2_CH5_IMW { w: self }
    }
    #[doc = "Bit 13 - Peripheral DMA2 CH6 interrupt mask to CPU1"]
    #[inline]
    pub fn dma2_ch6_im(&mut self) -> _DMA2_CH6_IMW {
        _DMA2_CH6_IMW { w: self }
    }
    #[doc = "Bit 14 - Peripheral DMA2 CH7 interrupt mask to CPU1"]
    #[inline]
    pub fn dma2_ch7_im(&mut self) -> _DMA2_CH7_IMW {
        _DMA2_CH7_IMW { w: self }
    }
    #[doc = "Bit 15 - Peripheral DMAM UX1 interrupt mask to CPU1"]
    #[inline]
    pub fn dmam_ux1_im(&mut self) -> _DMAM_UX1_IMW {
        _DMAM_UX1_IMW { w: self }
    }
    #[doc = "Bit 16 - Peripheral PVM1IM interrupt mask to CPU1"]
    #[inline]
    pub fn pvm1im(&mut self) -> _PVM1IMW {
        _PVM1IMW { w: self }
    }
    #[doc = "Bit 18 - Peripheral PVM3IM interrupt mask to CPU1"]
    #[inline]
    pub fn pvm3im(&mut self) -> _PVM3IMW {
        _PVM3IMW { w: self }
    }
    #[doc = "Bit 20 - Peripheral PVDIM interrupt mask to CPU1"]
    #[inline]
    pub fn pvdim(&mut self) -> _PVDIMW {
        _PVDIMW { w: self }
    }
    #[doc = "Bit 21 - Peripheral TSCIM interrupt mask to CPU1"]
    #[inline]
    pub fn tscim(&mut self) -> _TSCIMW {
        _TSCIMW { w: self }
    }
    #[doc = "Bit 22 - Peripheral LCDIM interrupt mask to CPU1"]
    #[inline]
    pub fn lcdim(&mut self) -> _LCDIMW {
        _LCDIMW { w: self }
    }
}
