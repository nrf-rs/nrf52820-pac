#[doc = "Writer for register TASKS_TRIGOVRFLW"]
pub type W = crate::W<u32, super::TASKS_TRIGOVRFLW>;
#[doc = "Register TASKS_TRIGOVRFLW `reset()`'s with value 0"]
impl crate::ResetValue for super::TASKS_TRIGOVRFLW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Set COUNTER to 0xFFFFF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASKS_TRIGOVRFLW_AW {
    #[doc = "1: Trigger task"]
    TRIGGER = 1,
}
impl From<TASKS_TRIGOVRFLW_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_TRIGOVRFLW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TASKS_TRIGOVRFLW`"]
pub struct TASKS_TRIGOVRFLW_W<'a> {
    w: &'a mut W,
}
impl<'a> TASKS_TRIGOVRFLW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TASKS_TRIGOVRFLW_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_TRIGOVRFLW_AW::TRIGGER)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Set COUNTER to 0xFFFFF0"]
    #[inline(always)]
    pub fn tasks_trigovrflw(&mut self) -> TASKS_TRIGOVRFLW_W {
        TASKS_TRIGOVRFLW_W { w: self }
    }
}
