#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL1_TOG {
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
pub struct CH0_CMDCMPLT_IRQR {
    bits: bool,
}
impl CH0_CMDCMPLT_IRQR {
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
pub struct CH1_CMDCMPLT_IRQR {
    bits: bool,
}
impl CH1_CMDCMPLT_IRQR {
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
pub struct CH2_CMDCMPLT_IRQR {
    bits: bool,
}
impl CH2_CMDCMPLT_IRQR {
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
pub struct CH3_CMDCMPLT_IRQR {
    bits: bool,
}
impl CH3_CMDCMPLT_IRQR {
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
pub struct CH4_CMDCMPLT_IRQR {
    bits: bool,
}
impl CH4_CMDCMPLT_IRQR {
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
pub struct CH5_CMDCMPLT_IRQR {
    bits: bool,
}
impl CH5_CMDCMPLT_IRQR {
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
pub struct CH6_CMDCMPLT_IRQR {
    bits: bool,
}
impl CH6_CMDCMPLT_IRQR {
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
pub struct CH7_CMDCMPLT_IRQR {
    bits: bool,
}
impl CH7_CMDCMPLT_IRQR {
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
pub struct CH8_CMDCMPLT_IRQR {
    bits: bool,
}
impl CH8_CMDCMPLT_IRQR {
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
pub struct CH9_CMDCMPLT_IRQR {
    bits: bool,
}
impl CH9_CMDCMPLT_IRQR {
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
pub struct CH10_CMDCMPLT_IRQR {
    bits: bool,
}
impl CH10_CMDCMPLT_IRQR {
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
pub struct CH11_CMDCMPLT_IRQR {
    bits: bool,
}
impl CH11_CMDCMPLT_IRQR {
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
pub struct CH12_CMDCMPLT_IRQR {
    bits: bool,
}
impl CH12_CMDCMPLT_IRQR {
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
pub struct CH13_CMDCMPLT_IRQR {
    bits: bool,
}
impl CH13_CMDCMPLT_IRQR {
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
pub struct CH14_CMDCMPLT_IRQR {
    bits: bool,
}
impl CH14_CMDCMPLT_IRQR {
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
pub struct CH15_CMDCMPLT_IRQR {
    bits: bool,
}
impl CH15_CMDCMPLT_IRQR {
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
pub struct CH0_CMDCMPLT_IRQ_ENR {
    bits: bool,
}
impl CH0_CMDCMPLT_IRQ_ENR {
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
pub struct CH1_CMDCMPLT_IRQ_ENR {
    bits: bool,
}
impl CH1_CMDCMPLT_IRQ_ENR {
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
pub struct CH2_CMDCMPLT_IRQ_ENR {
    bits: bool,
}
impl CH2_CMDCMPLT_IRQ_ENR {
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
pub struct CH3_CMDCMPLT_IRQ_ENR {
    bits: bool,
}
impl CH3_CMDCMPLT_IRQ_ENR {
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
pub struct CH4_CMDCMPLT_IRQ_ENR {
    bits: bool,
}
impl CH4_CMDCMPLT_IRQ_ENR {
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
pub struct CH5_CMDCMPLT_IRQ_ENR {
    bits: bool,
}
impl CH5_CMDCMPLT_IRQ_ENR {
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
pub struct CH6_CMDCMPLT_IRQ_ENR {
    bits: bool,
}
impl CH6_CMDCMPLT_IRQ_ENR {
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
pub struct CH7_CMDCMPLT_IRQ_ENR {
    bits: bool,
}
impl CH7_CMDCMPLT_IRQ_ENR {
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
pub struct CH8_CMDCMPLT_IRQ_ENR {
    bits: bool,
}
impl CH8_CMDCMPLT_IRQ_ENR {
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
pub struct CH9_CMDCMPLT_IRQ_ENR {
    bits: bool,
}
impl CH9_CMDCMPLT_IRQ_ENR {
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
pub struct CH10_CMDCMPLT_IRQ_ENR {
    bits: bool,
}
impl CH10_CMDCMPLT_IRQ_ENR {
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
pub struct CH11_CMDCMPLT_IRQ_ENR {
    bits: bool,
}
impl CH11_CMDCMPLT_IRQ_ENR {
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
pub struct CH12_CMDCMPLT_IRQ_ENR {
    bits: bool,
}
impl CH12_CMDCMPLT_IRQ_ENR {
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
pub struct CH13_CMDCMPLT_IRQ_ENR {
    bits: bool,
}
impl CH13_CMDCMPLT_IRQ_ENR {
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
pub struct CH14_CMDCMPLT_IRQ_ENR {
    bits: bool,
}
impl CH14_CMDCMPLT_IRQ_ENR {
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
pub struct CH15_CMDCMPLT_IRQ_ENR {
    bits: bool,
}
impl CH15_CMDCMPLT_IRQ_ENR {
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
pub struct _CH0_CMDCMPLT_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0_CMDCMPLT_IRQW<'a> {
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
pub struct _CH1_CMDCMPLT_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1_CMDCMPLT_IRQW<'a> {
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
pub struct _CH2_CMDCMPLT_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2_CMDCMPLT_IRQW<'a> {
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
pub struct _CH3_CMDCMPLT_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3_CMDCMPLT_IRQW<'a> {
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
pub struct _CH4_CMDCMPLT_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4_CMDCMPLT_IRQW<'a> {
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
pub struct _CH5_CMDCMPLT_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5_CMDCMPLT_IRQW<'a> {
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
pub struct _CH6_CMDCMPLT_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6_CMDCMPLT_IRQW<'a> {
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
pub struct _CH7_CMDCMPLT_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH7_CMDCMPLT_IRQW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH8_CMDCMPLT_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH8_CMDCMPLT_IRQW<'a> {
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
pub struct _CH9_CMDCMPLT_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH9_CMDCMPLT_IRQW<'a> {
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
pub struct _CH10_CMDCMPLT_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH10_CMDCMPLT_IRQW<'a> {
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
pub struct _CH11_CMDCMPLT_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH11_CMDCMPLT_IRQW<'a> {
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
pub struct _CH12_CMDCMPLT_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH12_CMDCMPLT_IRQW<'a> {
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
pub struct _CH13_CMDCMPLT_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH13_CMDCMPLT_IRQW<'a> {
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
pub struct _CH14_CMDCMPLT_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH14_CMDCMPLT_IRQW<'a> {
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
pub struct _CH15_CMDCMPLT_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CH15_CMDCMPLT_IRQW<'a> {
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
pub struct _CH0_CMDCMPLT_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0_CMDCMPLT_IRQ_ENW<'a> {
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
pub struct _CH1_CMDCMPLT_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1_CMDCMPLT_IRQ_ENW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH2_CMDCMPLT_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2_CMDCMPLT_IRQ_ENW<'a> {
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
pub struct _CH3_CMDCMPLT_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3_CMDCMPLT_IRQ_ENW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH4_CMDCMPLT_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4_CMDCMPLT_IRQ_ENW<'a> {
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
pub struct _CH5_CMDCMPLT_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5_CMDCMPLT_IRQ_ENW<'a> {
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
pub struct _CH6_CMDCMPLT_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6_CMDCMPLT_IRQ_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _CH7_CMDCMPLT_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CH7_CMDCMPLT_IRQ_ENW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH8_CMDCMPLT_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CH8_CMDCMPLT_IRQ_ENW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH9_CMDCMPLT_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CH9_CMDCMPLT_IRQ_ENW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH10_CMDCMPLT_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CH10_CMDCMPLT_IRQ_ENW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH11_CMDCMPLT_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CH11_CMDCMPLT_IRQ_ENW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH12_CMDCMPLT_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CH12_CMDCMPLT_IRQ_ENW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH13_CMDCMPLT_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CH13_CMDCMPLT_IRQ_ENW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH14_CMDCMPLT_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CH14_CMDCMPLT_IRQ_ENW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH15_CMDCMPLT_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CH15_CMDCMPLT_IRQ_ENW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Interrupt request status bit for APBH DMA channel 0"]
    #[inline]
    pub fn ch0_cmdcmplt_irq(&self) -> CH0_CMDCMPLT_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH0_CMDCMPLT_IRQR { bits }
    }
    #[doc = "Bit 1 - Interrupt request status bit for APBH DMA channel 1"]
    #[inline]
    pub fn ch1_cmdcmplt_irq(&self) -> CH1_CMDCMPLT_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH1_CMDCMPLT_IRQR { bits }
    }
    #[doc = "Bit 2 - Interrupt request status bit for APBH DMA channel 2"]
    #[inline]
    pub fn ch2_cmdcmplt_irq(&self) -> CH2_CMDCMPLT_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH2_CMDCMPLT_IRQR { bits }
    }
    #[doc = "Bit 3 - Interrupt request status bit for APBH DMA channel 3"]
    #[inline]
    pub fn ch3_cmdcmplt_irq(&self) -> CH3_CMDCMPLT_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH3_CMDCMPLT_IRQR { bits }
    }
    #[doc = "Bit 4 - Interrupt request status bit for APBH DMA channel 4"]
    #[inline]
    pub fn ch4_cmdcmplt_irq(&self) -> CH4_CMDCMPLT_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH4_CMDCMPLT_IRQR { bits }
    }
    #[doc = "Bit 5 - Interrupt request status bit for APBH DMA channel 5"]
    #[inline]
    pub fn ch5_cmdcmplt_irq(&self) -> CH5_CMDCMPLT_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH5_CMDCMPLT_IRQR { bits }
    }
    #[doc = "Bit 6 - Interrupt request status bit for APBH DMA channel 6"]
    #[inline]
    pub fn ch6_cmdcmplt_irq(&self) -> CH6_CMDCMPLT_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH6_CMDCMPLT_IRQR { bits }
    }
    #[doc = "Bit 7 - Interrupt request status bit for APBH DMA channel 7"]
    #[inline]
    pub fn ch7_cmdcmplt_irq(&self) -> CH7_CMDCMPLT_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH7_CMDCMPLT_IRQR { bits }
    }
    #[doc = "Bit 8 - Interrupt request status bit for APBH DMA Channel 8"]
    #[inline]
    pub fn ch8_cmdcmplt_irq(&self) -> CH8_CMDCMPLT_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH8_CMDCMPLT_IRQR { bits }
    }
    #[doc = "Bit 9 - Interrupt request status bit for APBH DMA Channel 9"]
    #[inline]
    pub fn ch9_cmdcmplt_irq(&self) -> CH9_CMDCMPLT_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH9_CMDCMPLT_IRQR { bits }
    }
    #[doc = "Bit 10 - Interrupt request status bit for APBH DMA Channel 10"]
    #[inline]
    pub fn ch10_cmdcmplt_irq(&self) -> CH10_CMDCMPLT_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH10_CMDCMPLT_IRQR { bits }
    }
    #[doc = "Bit 11 - Interrupt request status bit for APBH DMA Channel 11"]
    #[inline]
    pub fn ch11_cmdcmplt_irq(&self) -> CH11_CMDCMPLT_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH11_CMDCMPLT_IRQR { bits }
    }
    #[doc = "Bit 12 - Interrupt request status bit for APBH DMA Channel 12"]
    #[inline]
    pub fn ch12_cmdcmplt_irq(&self) -> CH12_CMDCMPLT_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH12_CMDCMPLT_IRQR { bits }
    }
    #[doc = "Bit 13 - Interrupt request status bit for APBH DMA Channel 13"]
    #[inline]
    pub fn ch13_cmdcmplt_irq(&self) -> CH13_CMDCMPLT_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH13_CMDCMPLT_IRQR { bits }
    }
    #[doc = "Bit 14 - Interrupt request status bit for APBH DMA Channel 14"]
    #[inline]
    pub fn ch14_cmdcmplt_irq(&self) -> CH14_CMDCMPLT_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH14_CMDCMPLT_IRQR { bits }
    }
    #[doc = "Bit 15 - Interrupt request status bit for APBH DMA Channel 15"]
    #[inline]
    pub fn ch15_cmdcmplt_irq(&self) -> CH15_CMDCMPLT_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH15_CMDCMPLT_IRQR { bits }
    }
    #[doc = "Bit 16 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 0."]
    #[inline]
    pub fn ch0_cmdcmplt_irq_en(&self) -> CH0_CMDCMPLT_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH0_CMDCMPLT_IRQ_ENR { bits }
    }
    #[doc = "Bit 17 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 1."]
    #[inline]
    pub fn ch1_cmdcmplt_irq_en(&self) -> CH1_CMDCMPLT_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH1_CMDCMPLT_IRQ_ENR { bits }
    }
    #[doc = "Bit 18 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 2."]
    #[inline]
    pub fn ch2_cmdcmplt_irq_en(&self) -> CH2_CMDCMPLT_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH2_CMDCMPLT_IRQ_ENR { bits }
    }
    #[doc = "Bit 19 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 3."]
    #[inline]
    pub fn ch3_cmdcmplt_irq_en(&self) -> CH3_CMDCMPLT_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH3_CMDCMPLT_IRQ_ENR { bits }
    }
    #[doc = "Bit 20 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 4."]
    #[inline]
    pub fn ch4_cmdcmplt_irq_en(&self) -> CH4_CMDCMPLT_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH4_CMDCMPLT_IRQ_ENR { bits }
    }
    #[doc = "Bit 21 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 5."]
    #[inline]
    pub fn ch5_cmdcmplt_irq_en(&self) -> CH5_CMDCMPLT_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH5_CMDCMPLT_IRQ_ENR { bits }
    }
    #[doc = "Bit 22 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 6."]
    #[inline]
    pub fn ch6_cmdcmplt_irq_en(&self) -> CH6_CMDCMPLT_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH6_CMDCMPLT_IRQ_ENR { bits }
    }
    #[doc = "Bit 23 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 7."]
    #[inline]
    pub fn ch7_cmdcmplt_irq_en(&self) -> CH7_CMDCMPLT_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH7_CMDCMPLT_IRQ_ENR { bits }
    }
    #[doc = "Bit 24 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 8."]
    #[inline]
    pub fn ch8_cmdcmplt_irq_en(&self) -> CH8_CMDCMPLT_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH8_CMDCMPLT_IRQ_ENR { bits }
    }
    #[doc = "Bit 25 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 9."]
    #[inline]
    pub fn ch9_cmdcmplt_irq_en(&self) -> CH9_CMDCMPLT_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH9_CMDCMPLT_IRQ_ENR { bits }
    }
    #[doc = "Bit 26 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 10."]
    #[inline]
    pub fn ch10_cmdcmplt_irq_en(&self) -> CH10_CMDCMPLT_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH10_CMDCMPLT_IRQ_ENR { bits }
    }
    #[doc = "Bit 27 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 11."]
    #[inline]
    pub fn ch11_cmdcmplt_irq_en(&self) -> CH11_CMDCMPLT_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH11_CMDCMPLT_IRQ_ENR { bits }
    }
    #[doc = "Bit 28 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 12."]
    #[inline]
    pub fn ch12_cmdcmplt_irq_en(&self) -> CH12_CMDCMPLT_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH12_CMDCMPLT_IRQ_ENR { bits }
    }
    #[doc = "Bit 29 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 13."]
    #[inline]
    pub fn ch13_cmdcmplt_irq_en(&self) -> CH13_CMDCMPLT_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH13_CMDCMPLT_IRQ_ENR { bits }
    }
    #[doc = "Bit 30 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 14."]
    #[inline]
    pub fn ch14_cmdcmplt_irq_en(&self) -> CH14_CMDCMPLT_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH14_CMDCMPLT_IRQ_ENR { bits }
    }
    #[doc = "Bit 31 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 15."]
    #[inline]
    pub fn ch15_cmdcmplt_irq_en(&self) -> CH15_CMDCMPLT_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH15_CMDCMPLT_IRQ_ENR { bits }
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
    #[doc = "Bit 0 - Interrupt request status bit for APBH DMA channel 0"]
    #[inline]
    pub fn ch0_cmdcmplt_irq(&mut self) -> _CH0_CMDCMPLT_IRQW {
        _CH0_CMDCMPLT_IRQW { w: self }
    }
    #[doc = "Bit 1 - Interrupt request status bit for APBH DMA channel 1"]
    #[inline]
    pub fn ch1_cmdcmplt_irq(&mut self) -> _CH1_CMDCMPLT_IRQW {
        _CH1_CMDCMPLT_IRQW { w: self }
    }
    #[doc = "Bit 2 - Interrupt request status bit for APBH DMA channel 2"]
    #[inline]
    pub fn ch2_cmdcmplt_irq(&mut self) -> _CH2_CMDCMPLT_IRQW {
        _CH2_CMDCMPLT_IRQW { w: self }
    }
    #[doc = "Bit 3 - Interrupt request status bit for APBH DMA channel 3"]
    #[inline]
    pub fn ch3_cmdcmplt_irq(&mut self) -> _CH3_CMDCMPLT_IRQW {
        _CH3_CMDCMPLT_IRQW { w: self }
    }
    #[doc = "Bit 4 - Interrupt request status bit for APBH DMA channel 4"]
    #[inline]
    pub fn ch4_cmdcmplt_irq(&mut self) -> _CH4_CMDCMPLT_IRQW {
        _CH4_CMDCMPLT_IRQW { w: self }
    }
    #[doc = "Bit 5 - Interrupt request status bit for APBH DMA channel 5"]
    #[inline]
    pub fn ch5_cmdcmplt_irq(&mut self) -> _CH5_CMDCMPLT_IRQW {
        _CH5_CMDCMPLT_IRQW { w: self }
    }
    #[doc = "Bit 6 - Interrupt request status bit for APBH DMA channel 6"]
    #[inline]
    pub fn ch6_cmdcmplt_irq(&mut self) -> _CH6_CMDCMPLT_IRQW {
        _CH6_CMDCMPLT_IRQW { w: self }
    }
    #[doc = "Bit 7 - Interrupt request status bit for APBH DMA channel 7"]
    #[inline]
    pub fn ch7_cmdcmplt_irq(&mut self) -> _CH7_CMDCMPLT_IRQW {
        _CH7_CMDCMPLT_IRQW { w: self }
    }
    #[doc = "Bit 8 - Interrupt request status bit for APBH DMA Channel 8"]
    #[inline]
    pub fn ch8_cmdcmplt_irq(&mut self) -> _CH8_CMDCMPLT_IRQW {
        _CH8_CMDCMPLT_IRQW { w: self }
    }
    #[doc = "Bit 9 - Interrupt request status bit for APBH DMA Channel 9"]
    #[inline]
    pub fn ch9_cmdcmplt_irq(&mut self) -> _CH9_CMDCMPLT_IRQW {
        _CH9_CMDCMPLT_IRQW { w: self }
    }
    #[doc = "Bit 10 - Interrupt request status bit for APBH DMA Channel 10"]
    #[inline]
    pub fn ch10_cmdcmplt_irq(&mut self) -> _CH10_CMDCMPLT_IRQW {
        _CH10_CMDCMPLT_IRQW { w: self }
    }
    #[doc = "Bit 11 - Interrupt request status bit for APBH DMA Channel 11"]
    #[inline]
    pub fn ch11_cmdcmplt_irq(&mut self) -> _CH11_CMDCMPLT_IRQW {
        _CH11_CMDCMPLT_IRQW { w: self }
    }
    #[doc = "Bit 12 - Interrupt request status bit for APBH DMA Channel 12"]
    #[inline]
    pub fn ch12_cmdcmplt_irq(&mut self) -> _CH12_CMDCMPLT_IRQW {
        _CH12_CMDCMPLT_IRQW { w: self }
    }
    #[doc = "Bit 13 - Interrupt request status bit for APBH DMA Channel 13"]
    #[inline]
    pub fn ch13_cmdcmplt_irq(&mut self) -> _CH13_CMDCMPLT_IRQW {
        _CH13_CMDCMPLT_IRQW { w: self }
    }
    #[doc = "Bit 14 - Interrupt request status bit for APBH DMA Channel 14"]
    #[inline]
    pub fn ch14_cmdcmplt_irq(&mut self) -> _CH14_CMDCMPLT_IRQW {
        _CH14_CMDCMPLT_IRQW { w: self }
    }
    #[doc = "Bit 15 - Interrupt request status bit for APBH DMA Channel 15"]
    #[inline]
    pub fn ch15_cmdcmplt_irq(&mut self) -> _CH15_CMDCMPLT_IRQW {
        _CH15_CMDCMPLT_IRQW { w: self }
    }
    #[doc = "Bit 16 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 0."]
    #[inline]
    pub fn ch0_cmdcmplt_irq_en(&mut self) -> _CH0_CMDCMPLT_IRQ_ENW {
        _CH0_CMDCMPLT_IRQ_ENW { w: self }
    }
    #[doc = "Bit 17 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 1."]
    #[inline]
    pub fn ch1_cmdcmplt_irq_en(&mut self) -> _CH1_CMDCMPLT_IRQ_ENW {
        _CH1_CMDCMPLT_IRQ_ENW { w: self }
    }
    #[doc = "Bit 18 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 2."]
    #[inline]
    pub fn ch2_cmdcmplt_irq_en(&mut self) -> _CH2_CMDCMPLT_IRQ_ENW {
        _CH2_CMDCMPLT_IRQ_ENW { w: self }
    }
    #[doc = "Bit 19 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 3."]
    #[inline]
    pub fn ch3_cmdcmplt_irq_en(&mut self) -> _CH3_CMDCMPLT_IRQ_ENW {
        _CH3_CMDCMPLT_IRQ_ENW { w: self }
    }
    #[doc = "Bit 20 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 4."]
    #[inline]
    pub fn ch4_cmdcmplt_irq_en(&mut self) -> _CH4_CMDCMPLT_IRQ_ENW {
        _CH4_CMDCMPLT_IRQ_ENW { w: self }
    }
    #[doc = "Bit 21 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 5."]
    #[inline]
    pub fn ch5_cmdcmplt_irq_en(&mut self) -> _CH5_CMDCMPLT_IRQ_ENW {
        _CH5_CMDCMPLT_IRQ_ENW { w: self }
    }
    #[doc = "Bit 22 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 6."]
    #[inline]
    pub fn ch6_cmdcmplt_irq_en(&mut self) -> _CH6_CMDCMPLT_IRQ_ENW {
        _CH6_CMDCMPLT_IRQ_ENW { w: self }
    }
    #[doc = "Bit 23 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 7."]
    #[inline]
    pub fn ch7_cmdcmplt_irq_en(&mut self) -> _CH7_CMDCMPLT_IRQ_ENW {
        _CH7_CMDCMPLT_IRQ_ENW { w: self }
    }
    #[doc = "Bit 24 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 8."]
    #[inline]
    pub fn ch8_cmdcmplt_irq_en(&mut self) -> _CH8_CMDCMPLT_IRQ_ENW {
        _CH8_CMDCMPLT_IRQ_ENW { w: self }
    }
    #[doc = "Bit 25 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 9."]
    #[inline]
    pub fn ch9_cmdcmplt_irq_en(&mut self) -> _CH9_CMDCMPLT_IRQ_ENW {
        _CH9_CMDCMPLT_IRQ_ENW { w: self }
    }
    #[doc = "Bit 26 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 10."]
    #[inline]
    pub fn ch10_cmdcmplt_irq_en(&mut self) -> _CH10_CMDCMPLT_IRQ_ENW {
        _CH10_CMDCMPLT_IRQ_ENW { w: self }
    }
    #[doc = "Bit 27 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 11."]
    #[inline]
    pub fn ch11_cmdcmplt_irq_en(&mut self) -> _CH11_CMDCMPLT_IRQ_ENW {
        _CH11_CMDCMPLT_IRQ_ENW { w: self }
    }
    #[doc = "Bit 28 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 12."]
    #[inline]
    pub fn ch12_cmdcmplt_irq_en(&mut self) -> _CH12_CMDCMPLT_IRQ_ENW {
        _CH12_CMDCMPLT_IRQ_ENW { w: self }
    }
    #[doc = "Bit 29 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 13."]
    #[inline]
    pub fn ch13_cmdcmplt_irq_en(&mut self) -> _CH13_CMDCMPLT_IRQ_ENW {
        _CH13_CMDCMPLT_IRQ_ENW { w: self }
    }
    #[doc = "Bit 30 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 14."]
    #[inline]
    pub fn ch14_cmdcmplt_irq_en(&mut self) -> _CH14_CMDCMPLT_IRQ_ENW {
        _CH14_CMDCMPLT_IRQ_ENW { w: self }
    }
    #[doc = "Bit 31 - Setting this bit enables the generation of an interrupt request for APBH DMA channel 15."]
    #[inline]
    pub fn ch15_cmdcmplt_irq_en(&mut self) -> _CH15_CMDCMPLT_IRQ_ENW {
        _CH15_CMDCMPLT_IRQ_ENW { w: self }
    }
}
