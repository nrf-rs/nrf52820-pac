#[doc = "Reader of register EXTREFSEL"]
pub type R = crate::R<u32, super::EXTREFSEL>;
#[doc = "Writer for register EXTREFSEL"]
pub type W = crate::W<u32, super::EXTREFSEL>;
#[doc = "Register EXTREFSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTREFSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "External analog reference select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTREFSEL_A {
    #[doc = "0: Use AIN0 as external analog reference"]
    ANALOGREFERENCE0 = 0,
    #[doc = "1: Use AIN1 as external analog reference"]
    ANALOGREFERENCE1 = 1,
    #[doc = "2: Use AIN2 as external analog reference"]
    ANALOGREFERENCE2 = 2,
    #[doc = "3: Use AIN3 as external analog reference"]
    ANALOGREFERENCE3 = 3,
}
impl From<EXTREFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTREFSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTREFSEL`"]
pub type EXTREFSEL_R = crate::R<u8, EXTREFSEL_A>;
impl EXTREFSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTREFSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTREFSEL_A::ANALOGREFERENCE0),
            1 => Val(EXTREFSEL_A::ANALOGREFERENCE1),
            2 => Val(EXTREFSEL_A::ANALOGREFERENCE2),
            3 => Val(EXTREFSEL_A::ANALOGREFERENCE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ANALOGREFERENCE0`"]
    #[inline(always)]
    pub fn is_analog_reference0(&self) -> bool {
        *self == EXTREFSEL_A::ANALOGREFERENCE0
    }
    #[doc = "Checks if the value of the field is `ANALOGREFERENCE1`"]
    #[inline(always)]
    pub fn is_analog_reference1(&self) -> bool {
        *self == EXTREFSEL_A::ANALOGREFERENCE1
    }
    #[doc = "Checks if the value of the field is `ANALOGREFERENCE2`"]
    #[inline(always)]
    pub fn is_analog_reference2(&self) -> bool {
        *self == EXTREFSEL_A::ANALOGREFERENCE2
    }
    #[doc = "Checks if the value of the field is `ANALOGREFERENCE3`"]
    #[inline(always)]
    pub fn is_analog_reference3(&self) -> bool {
        *self == EXTREFSEL_A::ANALOGREFERENCE3
    }
}
#[doc = "Write proxy for field `EXTREFSEL`"]
pub struct EXTREFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTREFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTREFSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Use AIN0 as external analog reference"]
    #[inline(always)]
    pub fn analog_reference0(self) -> &'a mut W {
        self.variant(EXTREFSEL_A::ANALOGREFERENCE0)
    }
    #[doc = "Use AIN1 as external analog reference"]
    #[inline(always)]
    pub fn analog_reference1(self) -> &'a mut W {
        self.variant(EXTREFSEL_A::ANALOGREFERENCE1)
    }
    #[doc = "Use AIN2 as external analog reference"]
    #[inline(always)]
    pub fn analog_reference2(self) -> &'a mut W {
        self.variant(EXTREFSEL_A::ANALOGREFERENCE2)
    }
    #[doc = "Use AIN3 as external analog reference"]
    #[inline(always)]
    pub fn analog_reference3(self) -> &'a mut W {
        self.variant(EXTREFSEL_A::ANALOGREFERENCE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - External analog reference select"]
    #[inline(always)]
    pub fn extrefsel(&self) -> EXTREFSEL_R {
        EXTREFSEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - External analog reference select"]
    #[inline(always)]
    pub fn extrefsel(&mut self) -> EXTREFSEL_W {
        EXTREFSEL_W { w: self }
    }
}
