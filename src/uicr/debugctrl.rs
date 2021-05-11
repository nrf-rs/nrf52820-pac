#[doc = "Reader of register DEBUGCTRL"]
pub type R = crate::R<u32, super::DEBUGCTRL>;
#[doc = "Writer for register DEBUGCTRL"]
pub type W = crate::W<u32, super::DEBUGCTRL>;
#[doc = "Register DEBUGCTRL `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::DEBUGCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Configure CPU flash patch and breakpoint (FPB) unit behavior\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPUFPBEN_A {
    #[doc = "255: Enable CPU FPB unit (default behavior)"]
    ENABLED = 255,
    #[doc = "0: Disable CPU FPB unit. Writes into the FPB registers will be ignored."]
    DISABLED = 0,
}
impl From<CPUFPBEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CPUFPBEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CPUFPBEN`"]
pub type CPUFPBEN_R = crate::R<u8, CPUFPBEN_A>;
impl CPUFPBEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CPUFPBEN_A> {
        use crate::Variant::*;
        match self.bits {
            255 => Val(CPUFPBEN_A::ENABLED),
            0 => Val(CPUFPBEN_A::DISABLED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CPUFPBEN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CPUFPBEN_A::DISABLED
    }
}
#[doc = "Write proxy for field `CPUFPBEN`"]
pub struct CPUFPBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUFPBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPUFPBEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Enable CPU FPB unit (default behavior)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CPUFPBEN_A::ENABLED)
    }
    #[doc = "Disable CPU FPB unit. Writes into the FPB registers will be ignored."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CPUFPBEN_A::DISABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Configure CPU flash patch and breakpoint (FPB) unit behavior"]
    #[inline(always)]
    pub fn cpufpben(&self) -> CPUFPBEN_R {
        CPUFPBEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Configure CPU flash patch and breakpoint (FPB) unit behavior"]
    #[inline(always)]
    pub fn cpufpben(&mut self) -> CPUFPBEN_W {
        CPUFPBEN_W { w: self }
    }
}
