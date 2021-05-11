#[doc = "Reader of register PSEL"]
pub type R = crate::R<u32, super::PSEL>;
#[doc = "Writer for register PSEL"]
pub type W = crate::W<u32, super::PSEL>;
#[doc = "Register PSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::PSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Analog pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSEL_A {
    #[doc = "0: AIN0 selected as analog input"]
    ANALOGINPUT0 = 0,
    #[doc = "1: AIN1 selected as analog input"]
    ANALOGINPUT1 = 1,
    #[doc = "2: AIN2 selected as analog input"]
    ANALOGINPUT2 = 2,
    #[doc = "3: AIN3 selected as analog input"]
    ANALOGINPUT3 = 3,
    #[doc = "7: VDDH/5 selected as analog input"]
    VDDHDIV5 = 7,
}
impl From<PSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PSEL`"]
pub type PSEL_R = crate::R<u8, PSEL_A>;
impl PSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PSEL_A::ANALOGINPUT0),
            1 => Val(PSEL_A::ANALOGINPUT1),
            2 => Val(PSEL_A::ANALOGINPUT2),
            3 => Val(PSEL_A::ANALOGINPUT3),
            7 => Val(PSEL_A::VDDHDIV5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT0`"]
    #[inline(always)]
    pub fn is_analog_input0(&self) -> bool {
        *self == PSEL_A::ANALOGINPUT0
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT1`"]
    #[inline(always)]
    pub fn is_analog_input1(&self) -> bool {
        *self == PSEL_A::ANALOGINPUT1
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT2`"]
    #[inline(always)]
    pub fn is_analog_input2(&self) -> bool {
        *self == PSEL_A::ANALOGINPUT2
    }
    #[doc = "Checks if the value of the field is `ANALOGINPUT3`"]
    #[inline(always)]
    pub fn is_analog_input3(&self) -> bool {
        *self == PSEL_A::ANALOGINPUT3
    }
    #[doc = "Checks if the value of the field is `VDDHDIV5`"]
    #[inline(always)]
    pub fn is_vddh_div5(&self) -> bool {
        *self == PSEL_A::VDDHDIV5
    }
}
#[doc = "Write proxy for field `PSEL`"]
pub struct PSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "AIN0 selected as analog input"]
    #[inline(always)]
    pub fn analog_input0(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOGINPUT0)
    }
    #[doc = "AIN1 selected as analog input"]
    #[inline(always)]
    pub fn analog_input1(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOGINPUT1)
    }
    #[doc = "AIN2 selected as analog input"]
    #[inline(always)]
    pub fn analog_input2(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOGINPUT2)
    }
    #[doc = "AIN3 selected as analog input"]
    #[inline(always)]
    pub fn analog_input3(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOGINPUT3)
    }
    #[doc = "VDDH/5 selected as analog input"]
    #[inline(always)]
    pub fn vddh_div5(self) -> &'a mut W {
        self.variant(PSEL_A::VDDHDIV5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Analog pin select"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Analog pin select"]
    #[inline(always)]
    pub fn psel(&mut self) -> PSEL_W {
        PSEL_W { w: self }
    }
}
