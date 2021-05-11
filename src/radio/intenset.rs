#[doc = "Reader of register INTENSET"]
pub type R = crate::R<u32, super::INTENSET>;
#[doc = "Writer for register INTENSET"]
pub type W = crate::W<u32, super::INTENSET>;
#[doc = "Register INTENSET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write '1' to enable interrupt for event READY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<READY_A> for bool {
    #[inline(always)]
    fn from(variant: READY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `READY`"]
pub type READY_R = crate::R<bool, READY_A>;
impl READY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_A {
        match self.bits {
            false => READY_A::DISABLED,
            true => READY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == READY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == READY_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event READY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<READY_AW> for bool {
    #[inline(always)]
    fn from(variant: READY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `READY`"]
pub struct READY_W<'a> {
    w: &'a mut W,
}
impl<'a> READY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READY_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(READY_AW::SET)
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
#[doc = "Write '1' to enable interrupt for event ADDRESS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESS_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ADDRESS_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDRESS`"]
pub type ADDRESS_R = crate::R<bool, ADDRESS_A>;
impl ADDRESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRESS_A {
        match self.bits {
            false => ADDRESS_A::DISABLED,
            true => ADDRESS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRESS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRESS_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event ADDRESS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESS_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<ADDRESS_AW> for bool {
    #[inline(always)]
    fn from(variant: ADDRESS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ADDRESS`"]
pub struct ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRESS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ADDRESS_AW::SET)
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
#[doc = "Write '1' to enable interrupt for event PAYLOAD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAYLOAD_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<PAYLOAD_A> for bool {
    #[inline(always)]
    fn from(variant: PAYLOAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAYLOAD`"]
pub type PAYLOAD_R = crate::R<bool, PAYLOAD_A>;
impl PAYLOAD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAYLOAD_A {
        match self.bits {
            false => PAYLOAD_A::DISABLED,
            true => PAYLOAD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PAYLOAD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PAYLOAD_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event PAYLOAD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAYLOAD_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<PAYLOAD_AW> for bool {
    #[inline(always)]
    fn from(variant: PAYLOAD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PAYLOAD`"]
pub struct PAYLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PAYLOAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAYLOAD_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(PAYLOAD_AW::SET)
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
#[doc = "Write '1' to enable interrupt for event END\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<END_A> for bool {
    #[inline(always)]
    fn from(variant: END_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `END`"]
pub type END_R = crate::R<bool, END_A>;
impl END_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> END_A {
        match self.bits {
            false => END_A::DISABLED,
            true => END_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == END_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == END_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event END\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<END_AW> for bool {
    #[inline(always)]
    fn from(variant: END_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `END`"]
pub struct END_W<'a> {
    w: &'a mut W,
}
impl<'a> END_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: END_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(END_AW::SET)
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
#[doc = "Write '1' to enable interrupt for event DISABLED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<DISABLED_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DISABLED`"]
pub type DISABLED_R = crate::R<bool, DISABLED_A>;
impl DISABLED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISABLED_A {
        match self.bits {
            false => DISABLED_A::DISABLED,
            true => DISABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISABLED_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event DISABLED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<DISABLED_AW> for bool {
    #[inline(always)]
    fn from(variant: DISABLED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DISABLED`"]
pub struct DISABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISABLED_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(DISABLED_AW::SET)
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
#[doc = "Write '1' to enable interrupt for event DEVMATCH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVMATCH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<DEVMATCH_A> for bool {
    #[inline(always)]
    fn from(variant: DEVMATCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEVMATCH`"]
pub type DEVMATCH_R = crate::R<bool, DEVMATCH_A>;
impl DEVMATCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEVMATCH_A {
        match self.bits {
            false => DEVMATCH_A::DISABLED,
            true => DEVMATCH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DEVMATCH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DEVMATCH_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event DEVMATCH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVMATCH_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<DEVMATCH_AW> for bool {
    #[inline(always)]
    fn from(variant: DEVMATCH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DEVMATCH`"]
pub struct DEVMATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVMATCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEVMATCH_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(DEVMATCH_AW::SET)
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
#[doc = "Write '1' to enable interrupt for event DEVMISS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVMISS_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<DEVMISS_A> for bool {
    #[inline(always)]
    fn from(variant: DEVMISS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEVMISS`"]
pub type DEVMISS_R = crate::R<bool, DEVMISS_A>;
impl DEVMISS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEVMISS_A {
        match self.bits {
            false => DEVMISS_A::DISABLED,
            true => DEVMISS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DEVMISS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DEVMISS_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event DEVMISS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVMISS_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<DEVMISS_AW> for bool {
    #[inline(always)]
    fn from(variant: DEVMISS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DEVMISS`"]
pub struct DEVMISS_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVMISS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEVMISS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(DEVMISS_AW::SET)
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
#[doc = "Write '1' to enable interrupt for event RSSIEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSSIEND_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RSSIEND_A> for bool {
    #[inline(always)]
    fn from(variant: RSSIEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSSIEND`"]
pub type RSSIEND_R = crate::R<bool, RSSIEND_A>;
impl RSSIEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSSIEND_A {
        match self.bits {
            false => RSSIEND_A::DISABLED,
            true => RSSIEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RSSIEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RSSIEND_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event RSSIEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSSIEND_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<RSSIEND_AW> for bool {
    #[inline(always)]
    fn from(variant: RSSIEND_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RSSIEND`"]
pub struct RSSIEND_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSIEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSSIEND_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RSSIEND_AW::SET)
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
#[doc = "Write '1' to enable interrupt for event BCMATCH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCMATCH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<BCMATCH_A> for bool {
    #[inline(always)]
    fn from(variant: BCMATCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BCMATCH`"]
pub type BCMATCH_R = crate::R<bool, BCMATCH_A>;
impl BCMATCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCMATCH_A {
        match self.bits {
            false => BCMATCH_A::DISABLED,
            true => BCMATCH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BCMATCH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BCMATCH_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event BCMATCH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCMATCH_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<BCMATCH_AW> for bool {
    #[inline(always)]
    fn from(variant: BCMATCH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `BCMATCH`"]
pub struct BCMATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> BCMATCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCMATCH_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BCMATCH_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Write '1' to enable interrupt for event CRCOK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCOK_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CRCOK_A> for bool {
    #[inline(always)]
    fn from(variant: CRCOK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRCOK`"]
pub type CRCOK_R = crate::R<bool, CRCOK_A>;
impl CRCOK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCOK_A {
        match self.bits {
            false => CRCOK_A::DISABLED,
            true => CRCOK_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRCOK_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRCOK_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event CRCOK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCOK_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CRCOK_AW> for bool {
    #[inline(always)]
    fn from(variant: CRCOK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CRCOK`"]
pub struct CRCOK_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCOK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCOK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CRCOK_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Write '1' to enable interrupt for event CRCERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCERROR_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CRCERROR_A> for bool {
    #[inline(always)]
    fn from(variant: CRCERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRCERROR`"]
pub type CRCERROR_R = crate::R<bool, CRCERROR_A>;
impl CRCERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCERROR_A {
        match self.bits {
            false => CRCERROR_A::DISABLED,
            true => CRCERROR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRCERROR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRCERROR_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event CRCERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCERROR_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CRCERROR_AW> for bool {
    #[inline(always)]
    fn from(variant: CRCERROR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CRCERROR`"]
pub struct CRCERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCERROR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCERROR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CRCERROR_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Write '1' to enable interrupt for event FRAMESTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAMESTART_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<FRAMESTART_A> for bool {
    #[inline(always)]
    fn from(variant: FRAMESTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRAMESTART`"]
pub type FRAMESTART_R = crate::R<bool, FRAMESTART_A>;
impl FRAMESTART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRAMESTART_A {
        match self.bits {
            false => FRAMESTART_A::DISABLED,
            true => FRAMESTART_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FRAMESTART_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FRAMESTART_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event FRAMESTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAMESTART_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<FRAMESTART_AW> for bool {
    #[inline(always)]
    fn from(variant: FRAMESTART_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `FRAMESTART`"]
pub struct FRAMESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMESTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRAMESTART_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(FRAMESTART_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Write '1' to enable interrupt for event EDEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDEND_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<EDEND_A> for bool {
    #[inline(always)]
    fn from(variant: EDEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDEND`"]
pub type EDEND_R = crate::R<bool, EDEND_A>;
impl EDEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDEND_A {
        match self.bits {
            false => EDEND_A::DISABLED,
            true => EDEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EDEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EDEND_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event EDEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDEND_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<EDEND_AW> for bool {
    #[inline(always)]
    fn from(variant: EDEND_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `EDEND`"]
pub struct EDEND_W<'a> {
    w: &'a mut W,
}
impl<'a> EDEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDEND_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(EDEND_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Write '1' to enable interrupt for event EDSTOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDSTOPPED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<EDSTOPPED_A> for bool {
    #[inline(always)]
    fn from(variant: EDSTOPPED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDSTOPPED`"]
pub type EDSTOPPED_R = crate::R<bool, EDSTOPPED_A>;
impl EDSTOPPED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDSTOPPED_A {
        match self.bits {
            false => EDSTOPPED_A::DISABLED,
            true => EDSTOPPED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EDSTOPPED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EDSTOPPED_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event EDSTOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDSTOPPED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<EDSTOPPED_AW> for bool {
    #[inline(always)]
    fn from(variant: EDSTOPPED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `EDSTOPPED`"]
pub struct EDSTOPPED_W<'a> {
    w: &'a mut W,
}
impl<'a> EDSTOPPED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDSTOPPED_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(EDSTOPPED_AW::SET)
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
#[doc = "Write '1' to enable interrupt for event CCAIDLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCAIDLE_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CCAIDLE_A> for bool {
    #[inline(always)]
    fn from(variant: CCAIDLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCAIDLE`"]
pub type CCAIDLE_R = crate::R<bool, CCAIDLE_A>;
impl CCAIDLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCAIDLE_A {
        match self.bits {
            false => CCAIDLE_A::DISABLED,
            true => CCAIDLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CCAIDLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CCAIDLE_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event CCAIDLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCAIDLE_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CCAIDLE_AW> for bool {
    #[inline(always)]
    fn from(variant: CCAIDLE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CCAIDLE`"]
pub struct CCAIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCAIDLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCAIDLE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CCAIDLE_AW::SET)
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
#[doc = "Write '1' to enable interrupt for event CCABUSY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCABUSY_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CCABUSY_A> for bool {
    #[inline(always)]
    fn from(variant: CCABUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCABUSY`"]
pub type CCABUSY_R = crate::R<bool, CCABUSY_A>;
impl CCABUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCABUSY_A {
        match self.bits {
            false => CCABUSY_A::DISABLED,
            true => CCABUSY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CCABUSY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CCABUSY_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event CCABUSY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCABUSY_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CCABUSY_AW> for bool {
    #[inline(always)]
    fn from(variant: CCABUSY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CCABUSY`"]
pub struct CCABUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> CCABUSY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCABUSY_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CCABUSY_AW::SET)
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
#[doc = "Write '1' to enable interrupt for event CCASTOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCASTOPPED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CCASTOPPED_A> for bool {
    #[inline(always)]
    fn from(variant: CCASTOPPED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCASTOPPED`"]
pub type CCASTOPPED_R = crate::R<bool, CCASTOPPED_A>;
impl CCASTOPPED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCASTOPPED_A {
        match self.bits {
            false => CCASTOPPED_A::DISABLED,
            true => CCASTOPPED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CCASTOPPED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CCASTOPPED_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event CCASTOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCASTOPPED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CCASTOPPED_AW> for bool {
    #[inline(always)]
    fn from(variant: CCASTOPPED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CCASTOPPED`"]
pub struct CCASTOPPED_W<'a> {
    w: &'a mut W,
}
impl<'a> CCASTOPPED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCASTOPPED_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CCASTOPPED_AW::SET)
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
#[doc = "Write '1' to enable interrupt for event RATEBOOST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RATEBOOST_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RATEBOOST_A> for bool {
    #[inline(always)]
    fn from(variant: RATEBOOST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RATEBOOST`"]
pub type RATEBOOST_R = crate::R<bool, RATEBOOST_A>;
impl RATEBOOST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RATEBOOST_A {
        match self.bits {
            false => RATEBOOST_A::DISABLED,
            true => RATEBOOST_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RATEBOOST_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RATEBOOST_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event RATEBOOST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RATEBOOST_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<RATEBOOST_AW> for bool {
    #[inline(always)]
    fn from(variant: RATEBOOST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RATEBOOST`"]
pub struct RATEBOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> RATEBOOST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RATEBOOST_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RATEBOOST_AW::SET)
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
#[doc = "Write '1' to enable interrupt for event TXREADY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXREADY_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TXREADY_A> for bool {
    #[inline(always)]
    fn from(variant: TXREADY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXREADY`"]
pub type TXREADY_R = crate::R<bool, TXREADY_A>;
impl TXREADY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXREADY_A {
        match self.bits {
            false => TXREADY_A::DISABLED,
            true => TXREADY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXREADY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXREADY_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event TXREADY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXREADY_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<TXREADY_AW> for bool {
    #[inline(always)]
    fn from(variant: TXREADY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TXREADY`"]
pub struct TXREADY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXREADY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXREADY_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TXREADY_AW::SET)
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
#[doc = "Write '1' to enable interrupt for event RXREADY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXREADY_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RXREADY_A> for bool {
    #[inline(always)]
    fn from(variant: RXREADY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXREADY`"]
pub type RXREADY_R = crate::R<bool, RXREADY_A>;
impl RXREADY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXREADY_A {
        match self.bits {
            false => RXREADY_A::DISABLED,
            true => RXREADY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXREADY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXREADY_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event RXREADY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXREADY_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<RXREADY_AW> for bool {
    #[inline(always)]
    fn from(variant: RXREADY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RXREADY`"]
pub struct RXREADY_W<'a> {
    w: &'a mut W,
}
impl<'a> RXREADY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXREADY_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RXREADY_AW::SET)
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
#[doc = "Write '1' to enable interrupt for event MHRMATCH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MHRMATCH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<MHRMATCH_A> for bool {
    #[inline(always)]
    fn from(variant: MHRMATCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MHRMATCH`"]
pub type MHRMATCH_R = crate::R<bool, MHRMATCH_A>;
impl MHRMATCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MHRMATCH_A {
        match self.bits {
            false => MHRMATCH_A::DISABLED,
            true => MHRMATCH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MHRMATCH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MHRMATCH_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event MHRMATCH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MHRMATCH_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<MHRMATCH_AW> for bool {
    #[inline(always)]
    fn from(variant: MHRMATCH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `MHRMATCH`"]
pub struct MHRMATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> MHRMATCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MHRMATCH_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(MHRMATCH_AW::SET)
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
#[doc = "Write '1' to enable interrupt for event SYNC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNC`"]
pub type SYNC_R = crate::R<bool, SYNC_A>;
impl SYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_A {
        match self.bits {
            false => SYNC_A::DISABLED,
            true => SYNC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYNC_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event SYNC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<SYNC_AW> for bool {
    #[inline(always)]
    fn from(variant: SYNC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SYNC`"]
pub struct SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(SYNC_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Write '1' to enable interrupt for event PHYEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHYEND_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<PHYEND_A> for bool {
    #[inline(always)]
    fn from(variant: PHYEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PHYEND`"]
pub type PHYEND_R = crate::R<bool, PHYEND_A>;
impl PHYEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHYEND_A {
        match self.bits {
            false => PHYEND_A::DISABLED,
            true => PHYEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PHYEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PHYEND_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event PHYEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHYEND_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<PHYEND_AW> for bool {
    #[inline(always)]
    fn from(variant: PHYEND_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PHYEND`"]
pub struct PHYEND_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHYEND_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(PHYEND_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Write '1' to enable interrupt for event CTEPRESENT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTEPRESENT_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CTEPRESENT_A> for bool {
    #[inline(always)]
    fn from(variant: CTEPRESENT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTEPRESENT`"]
pub type CTEPRESENT_R = crate::R<bool, CTEPRESENT_A>;
impl CTEPRESENT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTEPRESENT_A {
        match self.bits {
            false => CTEPRESENT_A::DISABLED,
            true => CTEPRESENT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTEPRESENT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTEPRESENT_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event CTEPRESENT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTEPRESENT_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CTEPRESENT_AW> for bool {
    #[inline(always)]
    fn from(variant: CTEPRESENT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CTEPRESENT`"]
pub struct CTEPRESENT_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEPRESENT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTEPRESENT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CTEPRESENT_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event READY"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event ADDRESS"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event PAYLOAD"]
    #[inline(always)]
    pub fn payload(&self) -> PAYLOAD_R {
        PAYLOAD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event END"]
    #[inline(always)]
    pub fn end(&self) -> END_R {
        END_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event DISABLED"]
    #[inline(always)]
    pub fn disabled(&self) -> DISABLED_R {
        DISABLED_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event DEVMATCH"]
    #[inline(always)]
    pub fn devmatch(&self) -> DEVMATCH_R {
        DEVMATCH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Write '1' to enable interrupt for event DEVMISS"]
    #[inline(always)]
    pub fn devmiss(&self) -> DEVMISS_R {
        DEVMISS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for event RSSIEND"]
    #[inline(always)]
    pub fn rssiend(&self) -> RSSIEND_R {
        RSSIEND_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Write '1' to enable interrupt for event BCMATCH"]
    #[inline(always)]
    pub fn bcmatch(&self) -> BCMATCH_R {
        BCMATCH_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Write '1' to enable interrupt for event CRCOK"]
    #[inline(always)]
    pub fn crcok(&self) -> CRCOK_R {
        CRCOK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Write '1' to enable interrupt for event CRCERROR"]
    #[inline(always)]
    pub fn crcerror(&self) -> CRCERROR_R {
        CRCERROR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Write '1' to enable interrupt for event FRAMESTART"]
    #[inline(always)]
    pub fn framestart(&self) -> FRAMESTART_R {
        FRAMESTART_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Write '1' to enable interrupt for event EDEND"]
    #[inline(always)]
    pub fn edend(&self) -> EDEND_R {
        EDEND_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Write '1' to enable interrupt for event EDSTOPPED"]
    #[inline(always)]
    pub fn edstopped(&self) -> EDSTOPPED_R {
        EDSTOPPED_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Write '1' to enable interrupt for event CCAIDLE"]
    #[inline(always)]
    pub fn ccaidle(&self) -> CCAIDLE_R {
        CCAIDLE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Write '1' to enable interrupt for event CCABUSY"]
    #[inline(always)]
    pub fn ccabusy(&self) -> CCABUSY_R {
        CCABUSY_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Write '1' to enable interrupt for event CCASTOPPED"]
    #[inline(always)]
    pub fn ccastopped(&self) -> CCASTOPPED_R {
        CCASTOPPED_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Write '1' to enable interrupt for event RATEBOOST"]
    #[inline(always)]
    pub fn rateboost(&self) -> RATEBOOST_R {
        RATEBOOST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Write '1' to enable interrupt for event TXREADY"]
    #[inline(always)]
    pub fn txready(&self) -> TXREADY_R {
        TXREADY_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Write '1' to enable interrupt for event RXREADY"]
    #[inline(always)]
    pub fn rxready(&self) -> RXREADY_R {
        RXREADY_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Write '1' to enable interrupt for event MHRMATCH"]
    #[inline(always)]
    pub fn mhrmatch(&self) -> MHRMATCH_R {
        MHRMATCH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Write '1' to enable interrupt for event SYNC"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Write '1' to enable interrupt for event PHYEND"]
    #[inline(always)]
    pub fn phyend(&self) -> PHYEND_R {
        PHYEND_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Write '1' to enable interrupt for event CTEPRESENT"]
    #[inline(always)]
    pub fn ctepresent(&self) -> CTEPRESENT_R {
        CTEPRESENT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event READY"]
    #[inline(always)]
    pub fn ready(&mut self) -> READY_W {
        READY_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event ADDRESS"]
    #[inline(always)]
    pub fn address(&mut self) -> ADDRESS_W {
        ADDRESS_W { w: self }
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event PAYLOAD"]
    #[inline(always)]
    pub fn payload(&mut self) -> PAYLOAD_W {
        PAYLOAD_W { w: self }
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event END"]
    #[inline(always)]
    pub fn end(&mut self) -> END_W {
        END_W { w: self }
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event DISABLED"]
    #[inline(always)]
    pub fn disabled(&mut self) -> DISABLED_W {
        DISABLED_W { w: self }
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event DEVMATCH"]
    #[inline(always)]
    pub fn devmatch(&mut self) -> DEVMATCH_W {
        DEVMATCH_W { w: self }
    }
    #[doc = "Bit 6 - Write '1' to enable interrupt for event DEVMISS"]
    #[inline(always)]
    pub fn devmiss(&mut self) -> DEVMISS_W {
        DEVMISS_W { w: self }
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for event RSSIEND"]
    #[inline(always)]
    pub fn rssiend(&mut self) -> RSSIEND_W {
        RSSIEND_W { w: self }
    }
    #[doc = "Bit 10 - Write '1' to enable interrupt for event BCMATCH"]
    #[inline(always)]
    pub fn bcmatch(&mut self) -> BCMATCH_W {
        BCMATCH_W { w: self }
    }
    #[doc = "Bit 12 - Write '1' to enable interrupt for event CRCOK"]
    #[inline(always)]
    pub fn crcok(&mut self) -> CRCOK_W {
        CRCOK_W { w: self }
    }
    #[doc = "Bit 13 - Write '1' to enable interrupt for event CRCERROR"]
    #[inline(always)]
    pub fn crcerror(&mut self) -> CRCERROR_W {
        CRCERROR_W { w: self }
    }
    #[doc = "Bit 14 - Write '1' to enable interrupt for event FRAMESTART"]
    #[inline(always)]
    pub fn framestart(&mut self) -> FRAMESTART_W {
        FRAMESTART_W { w: self }
    }
    #[doc = "Bit 15 - Write '1' to enable interrupt for event EDEND"]
    #[inline(always)]
    pub fn edend(&mut self) -> EDEND_W {
        EDEND_W { w: self }
    }
    #[doc = "Bit 16 - Write '1' to enable interrupt for event EDSTOPPED"]
    #[inline(always)]
    pub fn edstopped(&mut self) -> EDSTOPPED_W {
        EDSTOPPED_W { w: self }
    }
    #[doc = "Bit 17 - Write '1' to enable interrupt for event CCAIDLE"]
    #[inline(always)]
    pub fn ccaidle(&mut self) -> CCAIDLE_W {
        CCAIDLE_W { w: self }
    }
    #[doc = "Bit 18 - Write '1' to enable interrupt for event CCABUSY"]
    #[inline(always)]
    pub fn ccabusy(&mut self) -> CCABUSY_W {
        CCABUSY_W { w: self }
    }
    #[doc = "Bit 19 - Write '1' to enable interrupt for event CCASTOPPED"]
    #[inline(always)]
    pub fn ccastopped(&mut self) -> CCASTOPPED_W {
        CCASTOPPED_W { w: self }
    }
    #[doc = "Bit 20 - Write '1' to enable interrupt for event RATEBOOST"]
    #[inline(always)]
    pub fn rateboost(&mut self) -> RATEBOOST_W {
        RATEBOOST_W { w: self }
    }
    #[doc = "Bit 21 - Write '1' to enable interrupt for event TXREADY"]
    #[inline(always)]
    pub fn txready(&mut self) -> TXREADY_W {
        TXREADY_W { w: self }
    }
    #[doc = "Bit 22 - Write '1' to enable interrupt for event RXREADY"]
    #[inline(always)]
    pub fn rxready(&mut self) -> RXREADY_W {
        RXREADY_W { w: self }
    }
    #[doc = "Bit 23 - Write '1' to enable interrupt for event MHRMATCH"]
    #[inline(always)]
    pub fn mhrmatch(&mut self) -> MHRMATCH_W {
        MHRMATCH_W { w: self }
    }
    #[doc = "Bit 26 - Write '1' to enable interrupt for event SYNC"]
    #[inline(always)]
    pub fn sync(&mut self) -> SYNC_W {
        SYNC_W { w: self }
    }
    #[doc = "Bit 27 - Write '1' to enable interrupt for event PHYEND"]
    #[inline(always)]
    pub fn phyend(&mut self) -> PHYEND_W {
        PHYEND_W { w: self }
    }
    #[doc = "Bit 28 - Write '1' to enable interrupt for event CTEPRESENT"]
    #[inline(always)]
    pub fn ctepresent(&mut self) -> CTEPRESENT_W {
        CTEPRESENT_W { w: self }
    }
}
