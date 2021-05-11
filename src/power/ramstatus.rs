#[doc = "Reader of register RAMSTATUS"]
pub type R = crate::R<u32, super::RAMSTATUS>;
#[doc = "RAM block 0 is on or off/powering up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMBLOCK0_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<RAMBLOCK0_A> for bool {
    #[inline(always)]
    fn from(variant: RAMBLOCK0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RAMBLOCK0`"]
pub type RAMBLOCK0_R = crate::R<bool, RAMBLOCK0_A>;
impl RAMBLOCK0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMBLOCK0_A {
        match self.bits {
            false => RAMBLOCK0_A::OFF,
            true => RAMBLOCK0_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == RAMBLOCK0_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == RAMBLOCK0_A::ON
    }
}
#[doc = "RAM block 1 is on or off/powering up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMBLOCK1_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<RAMBLOCK1_A> for bool {
    #[inline(always)]
    fn from(variant: RAMBLOCK1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RAMBLOCK1`"]
pub type RAMBLOCK1_R = crate::R<bool, RAMBLOCK1_A>;
impl RAMBLOCK1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMBLOCK1_A {
        match self.bits {
            false => RAMBLOCK1_A::OFF,
            true => RAMBLOCK1_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == RAMBLOCK1_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == RAMBLOCK1_A::ON
    }
}
impl R {
    #[doc = "Bit 0 - RAM block 0 is on or off/powering up"]
    #[inline(always)]
    pub fn ramblock0(&self) -> RAMBLOCK0_R {
        RAMBLOCK0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RAM block 1 is on or off/powering up"]
    #[inline(always)]
    pub fn ramblock1(&self) -> RAMBLOCK1_R {
        RAMBLOCK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
