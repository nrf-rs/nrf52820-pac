#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1360usize],
    #[doc = "0x550 - Software force enable APPROTECT mechanism until next reset."]
    pub forceprotect: FORCEPROTECT,
    _reserved1: [u8; 4usize],
    #[doc = "0x558 - Software disable APPROTECT mechanism"]
    pub disable: DISABLE,
}
#[doc = "Software force enable APPROTECT mechanism until next reset.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [forceprotect](forceprotect) module"]
pub type FORCEPROTECT = crate::Reg<u32, _FORCEPROTECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FORCEPROTECT;
#[doc = "`read()` method returns [forceprotect::R](forceprotect::R) reader structure"]
impl crate::Readable for FORCEPROTECT {}
#[doc = "`write(|w| ..)` method takes [forceprotect::W](forceprotect::W) writer structure"]
impl crate::Writable for FORCEPROTECT {}
#[doc = "Software force enable APPROTECT mechanism until next reset."]
pub mod forceprotect;
#[doc = "Software disable APPROTECT mechanism\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [disable](disable) module"]
pub type DISABLE = crate::Reg<u32, _DISABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DISABLE;
#[doc = "`read()` method returns [disable::R](disable::R) reader structure"]
impl crate::Readable for DISABLE {}
#[doc = "`write(|w| ..)` method takes [disable::W](disable::W) writer structure"]
impl crate::Writable for DISABLE {}
#[doc = "Software disable APPROTECT mechanism"]
pub mod disable;
