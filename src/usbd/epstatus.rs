#[doc = "Reader of register EPSTATUS"]
pub type R = crate::R<u32, super::EPSTATUS>;
#[doc = "Writer for register EPSTATUS"]
pub type W = crate::W<u32, super::EPSTATUS>;
#[doc = "Register EPSTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::EPSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN0_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NODATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATADONE = 1,
}
impl From<EPIN0_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPIN0`"]
pub type EPIN0_R = crate::R<bool, EPIN0_A>;
impl EPIN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN0_A {
        match self.bits {
            false => EPIN0_A::NODATA,
            true => EPIN0_A::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN0_A::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN0_A::DATADONE
    }
}
#[doc = "Write proxy for field `EPIN0`"]
pub struct EPIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPIN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPIN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN0_A::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN0_A::DATADONE)
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
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN1_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NODATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATADONE = 1,
}
impl From<EPIN1_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPIN1`"]
pub type EPIN1_R = crate::R<bool, EPIN1_A>;
impl EPIN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN1_A {
        match self.bits {
            false => EPIN1_A::NODATA,
            true => EPIN1_A::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN1_A::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN1_A::DATADONE
    }
}
#[doc = "Write proxy for field `EPIN1`"]
pub struct EPIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPIN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPIN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN1_A::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN1_A::DATADONE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN2_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NODATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATADONE = 1,
}
impl From<EPIN2_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPIN2`"]
pub type EPIN2_R = crate::R<bool, EPIN2_A>;
impl EPIN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN2_A {
        match self.bits {
            false => EPIN2_A::NODATA,
            true => EPIN2_A::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN2_A::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN2_A::DATADONE
    }
}
#[doc = "Write proxy for field `EPIN2`"]
pub struct EPIN2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPIN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPIN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN2_A::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN2_A::DATADONE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN3_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NODATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATADONE = 1,
}
impl From<EPIN3_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPIN3`"]
pub type EPIN3_R = crate::R<bool, EPIN3_A>;
impl EPIN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN3_A {
        match self.bits {
            false => EPIN3_A::NODATA,
            true => EPIN3_A::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN3_A::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN3_A::DATADONE
    }
}
#[doc = "Write proxy for field `EPIN3`"]
pub struct EPIN3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPIN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPIN3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN3_A::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN3_A::DATADONE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN4_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NODATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATADONE = 1,
}
impl From<EPIN4_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPIN4`"]
pub type EPIN4_R = crate::R<bool, EPIN4_A>;
impl EPIN4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN4_A {
        match self.bits {
            false => EPIN4_A::NODATA,
            true => EPIN4_A::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN4_A::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN4_A::DATADONE
    }
}
#[doc = "Write proxy for field `EPIN4`"]
pub struct EPIN4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPIN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPIN4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN4_A::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN4_A::DATADONE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN5_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NODATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATADONE = 1,
}
impl From<EPIN5_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPIN5`"]
pub type EPIN5_R = crate::R<bool, EPIN5_A>;
impl EPIN5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN5_A {
        match self.bits {
            false => EPIN5_A::NODATA,
            true => EPIN5_A::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN5_A::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN5_A::DATADONE
    }
}
#[doc = "Write proxy for field `EPIN5`"]
pub struct EPIN5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPIN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPIN5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN5_A::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN5_A::DATADONE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN6_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NODATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATADONE = 1,
}
impl From<EPIN6_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPIN6`"]
pub type EPIN6_R = crate::R<bool, EPIN6_A>;
impl EPIN6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN6_A {
        match self.bits {
            false => EPIN6_A::NODATA,
            true => EPIN6_A::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN6_A::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN6_A::DATADONE
    }
}
#[doc = "Write proxy for field `EPIN6`"]
pub struct EPIN6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPIN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPIN6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN6_A::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN6_A::DATADONE)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN7_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NODATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATADONE = 1,
}
impl From<EPIN7_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPIN7`"]
pub type EPIN7_R = crate::R<bool, EPIN7_A>;
impl EPIN7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN7_A {
        match self.bits {
            false => EPIN7_A::NODATA,
            true => EPIN7_A::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN7_A::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN7_A::DATADONE
    }
}
#[doc = "Write proxy for field `EPIN7`"]
pub struct EPIN7_W<'a> {
    w: &'a mut W,
}
impl<'a> EPIN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPIN7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN7_A::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN7_A::DATADONE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIN8_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NODATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATADONE = 1,
}
impl From<EPIN8_A> for bool {
    #[inline(always)]
    fn from(variant: EPIN8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPIN8`"]
pub type EPIN8_R = crate::R<bool, EPIN8_A>;
impl EPIN8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIN8_A {
        match self.bits {
            false => EPIN8_A::NODATA,
            true => EPIN8_A::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPIN8_A::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPIN8_A::DATADONE
    }
}
#[doc = "Write proxy for field `EPIN8`"]
pub struct EPIN8_W<'a> {
    w: &'a mut W,
}
impl<'a> EPIN8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPIN8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPIN8_A::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPIN8_A::DATADONE)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT0_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NODATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATADONE = 1,
}
impl From<EPOUT0_A> for bool {
    #[inline(always)]
    fn from(variant: EPOUT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPOUT0`"]
pub type EPOUT0_R = crate::R<bool, EPOUT0_A>;
impl EPOUT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOUT0_A {
        match self.bits {
            false => EPOUT0_A::NODATA,
            true => EPOUT0_A::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT0_A::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT0_A::DATADONE
    }
}
#[doc = "Write proxy for field `EPOUT0`"]
pub struct EPOUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPOUT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPOUT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT0_A::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT0_A::DATADONE)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT1_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NODATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATADONE = 1,
}
impl From<EPOUT1_A> for bool {
    #[inline(always)]
    fn from(variant: EPOUT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPOUT1`"]
pub type EPOUT1_R = crate::R<bool, EPOUT1_A>;
impl EPOUT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOUT1_A {
        match self.bits {
            false => EPOUT1_A::NODATA,
            true => EPOUT1_A::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT1_A::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT1_A::DATADONE
    }
}
#[doc = "Write proxy for field `EPOUT1`"]
pub struct EPOUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPOUT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPOUT1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT1_A::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT1_A::DATADONE)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT2_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NODATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATADONE = 1,
}
impl From<EPOUT2_A> for bool {
    #[inline(always)]
    fn from(variant: EPOUT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPOUT2`"]
pub type EPOUT2_R = crate::R<bool, EPOUT2_A>;
impl EPOUT2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOUT2_A {
        match self.bits {
            false => EPOUT2_A::NODATA,
            true => EPOUT2_A::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT2_A::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT2_A::DATADONE
    }
}
#[doc = "Write proxy for field `EPOUT2`"]
pub struct EPOUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPOUT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPOUT2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT2_A::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT2_A::DATADONE)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT3_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NODATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATADONE = 1,
}
impl From<EPOUT3_A> for bool {
    #[inline(always)]
    fn from(variant: EPOUT3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPOUT3`"]
pub type EPOUT3_R = crate::R<bool, EPOUT3_A>;
impl EPOUT3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOUT3_A {
        match self.bits {
            false => EPOUT3_A::NODATA,
            true => EPOUT3_A::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT3_A::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT3_A::DATADONE
    }
}
#[doc = "Write proxy for field `EPOUT3`"]
pub struct EPOUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPOUT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPOUT3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT3_A::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT3_A::DATADONE)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT4_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NODATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATADONE = 1,
}
impl From<EPOUT4_A> for bool {
    #[inline(always)]
    fn from(variant: EPOUT4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPOUT4`"]
pub type EPOUT4_R = crate::R<bool, EPOUT4_A>;
impl EPOUT4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOUT4_A {
        match self.bits {
            false => EPOUT4_A::NODATA,
            true => EPOUT4_A::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT4_A::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT4_A::DATADONE
    }
}
#[doc = "Write proxy for field `EPOUT4`"]
pub struct EPOUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPOUT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPOUT4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT4_A::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT4_A::DATADONE)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT5_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NODATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATADONE = 1,
}
impl From<EPOUT5_A> for bool {
    #[inline(always)]
    fn from(variant: EPOUT5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPOUT5`"]
pub type EPOUT5_R = crate::R<bool, EPOUT5_A>;
impl EPOUT5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOUT5_A {
        match self.bits {
            false => EPOUT5_A::NODATA,
            true => EPOUT5_A::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT5_A::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT5_A::DATADONE
    }
}
#[doc = "Write proxy for field `EPOUT5`"]
pub struct EPOUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPOUT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPOUT5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT5_A::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT5_A::DATADONE)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT6_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NODATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATADONE = 1,
}
impl From<EPOUT6_A> for bool {
    #[inline(always)]
    fn from(variant: EPOUT6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPOUT6`"]
pub type EPOUT6_R = crate::R<bool, EPOUT6_A>;
impl EPOUT6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOUT6_A {
        match self.bits {
            false => EPOUT6_A::NODATA,
            true => EPOUT6_A::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT6_A::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT6_A::DATADONE
    }
}
#[doc = "Write proxy for field `EPOUT6`"]
pub struct EPOUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPOUT6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPOUT6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT6_A::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT6_A::DATADONE)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT7_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NODATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATADONE = 1,
}
impl From<EPOUT7_A> for bool {
    #[inline(always)]
    fn from(variant: EPOUT7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPOUT7`"]
pub type EPOUT7_R = crate::R<bool, EPOUT7_A>;
impl EPOUT7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOUT7_A {
        match self.bits {
            false => EPOUT7_A::NODATA,
            true => EPOUT7_A::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT7_A::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT7_A::DATADONE
    }
}
#[doc = "Write proxy for field `EPOUT7`"]
pub struct EPOUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> EPOUT7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPOUT7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT7_A::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT7_A::DATADONE)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Captured state of endpoint's EasyDMA registers. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPOUT8_A {
    #[doc = "0: EasyDMA registers have not been captured for this endpoint"]
    NODATA = 0,
    #[doc = "1: EasyDMA registers have been captured for this endpoint"]
    DATADONE = 1,
}
impl From<EPOUT8_A> for bool {
    #[inline(always)]
    fn from(variant: EPOUT8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPOUT8`"]
pub type EPOUT8_R = crate::R<bool, EPOUT8_A>;
impl EPOUT8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPOUT8_A {
        match self.bits {
            false => EPOUT8_A::NODATA,
            true => EPOUT8_A::DATADONE,
        }
    }
    #[doc = "Checks if the value of the field is `NODATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == EPOUT8_A::NODATA
    }
    #[doc = "Checks if the value of the field is `DATADONE`"]
    #[inline(always)]
    pub fn is_data_done(&self) -> bool {
        *self == EPOUT8_A::DATADONE
    }
}
#[doc = "Write proxy for field `EPOUT8`"]
pub struct EPOUT8_W<'a> {
    w: &'a mut W,
}
impl<'a> EPOUT8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPOUT8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "EasyDMA registers have not been captured for this endpoint"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut W {
        self.variant(EPOUT8_A::NODATA)
    }
    #[doc = "EasyDMA registers have been captured for this endpoint"]
    #[inline(always)]
    pub fn data_done(self) -> &'a mut W {
        self.variant(EPOUT8_A::DATADONE)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin0(&self) -> EPIN0_R {
        EPIN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin1(&self) -> EPIN1_R {
        EPIN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin2(&self) -> EPIN2_R {
        EPIN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin3(&self) -> EPIN3_R {
        EPIN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin4(&self) -> EPIN4_R {
        EPIN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin5(&self) -> EPIN5_R {
        EPIN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin6(&self) -> EPIN6_R {
        EPIN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin7(&self) -> EPIN7_R {
        EPIN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin8(&self) -> EPIN8_R {
        EPIN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout0(&self) -> EPOUT0_R {
        EPOUT0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout1(&self) -> EPOUT1_R {
        EPOUT1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout2(&self) -> EPOUT2_R {
        EPOUT2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout3(&self) -> EPOUT3_R {
        EPOUT3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout4(&self) -> EPOUT4_R {
        EPOUT4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout5(&self) -> EPOUT5_R {
        EPOUT5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout6(&self) -> EPOUT6_R {
        EPOUT6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout7(&self) -> EPOUT7_R {
        EPOUT7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout8(&self) -> EPOUT8_R {
        EPOUT8_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin0(&mut self) -> EPIN0_W {
        EPIN0_W { w: self }
    }
    #[doc = "Bit 1 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin1(&mut self) -> EPIN1_W {
        EPIN1_W { w: self }
    }
    #[doc = "Bit 2 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin2(&mut self) -> EPIN2_W {
        EPIN2_W { w: self }
    }
    #[doc = "Bit 3 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin3(&mut self) -> EPIN3_W {
        EPIN3_W { w: self }
    }
    #[doc = "Bit 4 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin4(&mut self) -> EPIN4_W {
        EPIN4_W { w: self }
    }
    #[doc = "Bit 5 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin5(&mut self) -> EPIN5_W {
        EPIN5_W { w: self }
    }
    #[doc = "Bit 6 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin6(&mut self) -> EPIN6_W {
        EPIN6_W { w: self }
    }
    #[doc = "Bit 7 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin7(&mut self) -> EPIN7_W {
        EPIN7_W { w: self }
    }
    #[doc = "Bit 8 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epin8(&mut self) -> EPIN8_W {
        EPIN8_W { w: self }
    }
    #[doc = "Bit 16 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout0(&mut self) -> EPOUT0_W {
        EPOUT0_W { w: self }
    }
    #[doc = "Bit 17 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout1(&mut self) -> EPOUT1_W {
        EPOUT1_W { w: self }
    }
    #[doc = "Bit 18 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout2(&mut self) -> EPOUT2_W {
        EPOUT2_W { w: self }
    }
    #[doc = "Bit 19 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout3(&mut self) -> EPOUT3_W {
        EPOUT3_W { w: self }
    }
    #[doc = "Bit 20 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout4(&mut self) -> EPOUT4_W {
        EPOUT4_W { w: self }
    }
    #[doc = "Bit 21 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout5(&mut self) -> EPOUT5_W {
        EPOUT5_W { w: self }
    }
    #[doc = "Bit 22 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout6(&mut self) -> EPOUT6_W {
        EPOUT6_W { w: self }
    }
    #[doc = "Bit 23 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout7(&mut self) -> EPOUT7_W {
        EPOUT7_W { w: self }
    }
    #[doc = "Bit 24 - Captured state of endpoint's EasyDMA registers. Write '1' to clear."]
    #[inline(always)]
    pub fn epout8(&mut self) -> EPOUT8_W {
        EPOUT8_W { w: self }
    }
}
