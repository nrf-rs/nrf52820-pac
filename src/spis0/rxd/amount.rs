#[doc = "Reader of register AMOUNT"]
pub type R = crate::R<u32, super::AMOUNT>;
#[doc = "Reader of field `AMOUNT`"]
pub type AMOUNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:14 - Number of bytes received in the last granted transaction"]
    #[inline(always)]
    pub fn amount(&self) -> AMOUNT_R {
        AMOUNT_R::new((self.bits & 0x7fff) as u16)
    }
}
