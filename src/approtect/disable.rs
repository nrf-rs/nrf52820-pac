#[doc = "Reader of register DISABLE"]
pub type R = crate::R<u32, super::DISABLE>;
#[doc = "Writer for register DISABLE"]
pub type W = crate::W<u32, super::DISABLE>;
#[doc = "Register DISABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::DISABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Software disable APPROTECT mechanism\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DISABLE_A {
    #[doc = "90: Software disable APPROTECT mechanism"]
    SWDISABLE = 90,
}
impl From<DISABLE_A> for u8 {
    #[inline(always)]
    fn from(variant: DISABLE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DISABLE`"]
pub type DISABLE_R = crate::R<u8, DISABLE_A>;
impl DISABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DISABLE_A> {
        use crate::Variant::*;
        match self.bits {
            90 => Val(DISABLE_A::SWDISABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SWDISABLE`"]
    #[inline(always)]
    pub fn is_sw_disable(&self) -> bool {
        *self == DISABLE_A::SWDISABLE
    }
}
#[doc = "Write proxy for field `DISABLE`"]
pub struct DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISABLE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software disable APPROTECT mechanism"]
    #[inline(always)]
    pub fn sw_disable(self) -> &'a mut W {
        self.variant(DISABLE_A::SWDISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Software disable APPROTECT mechanism"]
    #[inline(always)]
    pub fn disable(&self) -> DISABLE_R {
        DISABLE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Software disable APPROTECT mechanism"]
    #[inline(always)]
    pub fn disable(&mut self) -> DISABLE_W {
        DISABLE_W { w: self }
    }
}
