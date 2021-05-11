#[doc = "Reader of register VARIANT"]
pub type R = crate::R<u32, super::VARIANT>;
#[doc = "Build code (hardware version and production configuration). Encoded as ASCII.\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum VARIANT_A {
    #[doc = "1094795585: AAAA"]
    AAAA = 1094795585,
    #[doc = "1094795843: AABC"]
    AABC = 1094795843,
    #[doc = "1094796080: AAC0"]
    AAC0 = 1094796080,
    #[doc = "1094796081: AAC1"]
    AAC1 = 1094796081,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED = 4294967295,
}
impl From<VARIANT_A> for u32 {
    #[inline(always)]
    fn from(variant: VARIANT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VARIANT`"]
pub type VARIANT_R = crate::R<u32, VARIANT_A>;
impl VARIANT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, VARIANT_A> {
        use crate::Variant::*;
        match self.bits {
            1094795585 => Val(VARIANT_A::AAAA),
            1094795843 => Val(VARIANT_A::AABC),
            1094796080 => Val(VARIANT_A::AAC0),
            1094796081 => Val(VARIANT_A::AAC1),
            4294967295 => Val(VARIANT_A::UNSPECIFIED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AAAA`"]
    #[inline(always)]
    pub fn is_aaaa(&self) -> bool {
        *self == VARIANT_A::AAAA
    }
    #[doc = "Checks if the value of the field is `AABC`"]
    #[inline(always)]
    pub fn is_aabc(&self) -> bool {
        *self == VARIANT_A::AABC
    }
    #[doc = "Checks if the value of the field is `AAC0`"]
    #[inline(always)]
    pub fn is_aac0(&self) -> bool {
        *self == VARIANT_A::AAC0
    }
    #[doc = "Checks if the value of the field is `AAC1`"]
    #[inline(always)]
    pub fn is_aac1(&self) -> bool {
        *self == VARIANT_A::AAC1
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == VARIANT_A::UNSPECIFIED
    }
}
impl R {
    #[doc = "Bits 0:31 - Build code (hardware version and production configuration). Encoded as ASCII."]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
