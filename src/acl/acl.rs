#[doc = "Description cluster: Start address of region to protect. The start address must be word-aligned.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr](addr) module"]
pub type ADDR = crate::Reg<u32, _ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR;
#[doc = "`read()` method returns [addr::R](addr::R) reader structure"]
impl crate::Readable for ADDR {}
#[doc = "`write(|w| ..)` method takes [addr::W](addr::W) writer structure"]
impl crate::Writable for ADDR {}
#[doc = "Description cluster: Start address of region to protect. The start address must be word-aligned."]
pub mod addr;
#[doc = "Description cluster: Size of region to protect counting from address ACL\\[n\\].ADDR. Write '0' as no effect.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [size](size) module"]
pub type SIZE = crate::Reg<u32, _SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIZE;
#[doc = "`read()` method returns [size::R](size::R) reader structure"]
impl crate::Readable for SIZE {}
#[doc = "`write(|w| ..)` method takes [size::W](size::W) writer structure"]
impl crate::Writable for SIZE {}
#[doc = "Description cluster: Size of region to protect counting from address ACL\\[n\\].ADDR. Write '0' as no effect."]
pub mod size;
#[doc = "Description cluster: Access permissions for region n as defined by start address ACL\\[n\\].ADDR and size ACL\\[n\\].SIZE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perm](perm) module"]
pub type PERM = crate::Reg<u32, _PERM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERM;
#[doc = "`read()` method returns [perm::R](perm::R) reader structure"]
impl crate::Readable for PERM {}
#[doc = "`write(|w| ..)` method takes [perm::W](perm::W) writer structure"]
impl crate::Writable for PERM {}
#[doc = "Description cluster: Access permissions for region n as defined by start address ACL\\[n\\].ADDR and size ACL\\[n\\].SIZE"]
pub mod perm;
