#[doc = "Reader of register A4"]
pub type R = crate::R<u32, super::A4>;
#[doc = "Writer for register A4"]
pub type W = crate::W<u32, super::A4>;
#[doc = "Register A4 `reset()`'s with value 0x04bd"]
impl crate::ResetValue for super::A4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04bd
    }
}
#[doc = "Reader of field `A4`"]
pub type A4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `A4`"]
pub struct A4_W<'a> {
    w: &'a mut W,
}
impl<'a> A4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Slope of fifth piecewise linear function"]
    #[inline(always)]
    pub fn a4(&self) -> A4_R {
        A4_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Slope of fifth piecewise linear function"]
    #[inline(always)]
    pub fn a4(&mut self) -> A4_W {
        A4_W { w: self }
    }
}
