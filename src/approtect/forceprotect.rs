#[doc = "Reader of register FORCEPROTECT"]
pub type R = crate::R<u32, super::FORCEPROTECT>;
#[doc = "Writer for register FORCEPROTECT"]
pub type W = crate::W<u32, super::FORCEPROTECT>;
#[doc = "Register FORCEPROTECT `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::FORCEPROTECT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Write 0x0 to force enable APPROTECT mechanism\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FORCEPROTECT_A {
    #[doc = "0: Software force enable APPROTECT mechanism"]
    FORCE = 0,
}
impl From<FORCEPROTECT_A> for u8 {
    #[inline(always)]
    fn from(variant: FORCEPROTECT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FORCEPROTECT`"]
pub type FORCEPROTECT_R = crate::R<u8, FORCEPROTECT_A>;
impl FORCEPROTECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FORCEPROTECT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FORCEPROTECT_A::FORCE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FORCE`"]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        *self == FORCEPROTECT_A::FORCE
    }
}
#[doc = "Write proxy for field `FORCEPROTECT`"]
pub struct FORCEPROTECT_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEPROTECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCEPROTECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Software force enable APPROTECT mechanism"]
    #[inline(always)]
    pub fn force(self) -> &'a mut W {
        self.variant(FORCEPROTECT_A::FORCE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Write 0x0 to force enable APPROTECT mechanism"]
    #[inline(always)]
    pub fn forceprotect(&self) -> FORCEPROTECT_R {
        FORCEPROTECT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Write 0x0 to force enable APPROTECT mechanism"]
    #[inline(always)]
    pub fn forceprotect(&mut self) -> FORCEPROTECT_W {
        FORCEPROTECT_W { w: self }
    }
}
