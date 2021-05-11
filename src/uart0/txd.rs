#[doc = "Writer for register TXD"]
pub type W = crate::W<u32, super::TXD>;
#[doc = "Register TXD `reset()`'s with value 0"]
impl crate::ResetValue for super::TXD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TXD`"]
pub struct TXD_W<'a> {
    w: &'a mut W,
}
impl<'a> TXD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - TX data to be transferred"]
    #[inline(always)]
    pub fn txd(&mut self) -> TXD_W {
        TXD_W { w: self }
    }
}
