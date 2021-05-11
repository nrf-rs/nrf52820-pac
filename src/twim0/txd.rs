#[doc = "Data pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptr](ptr) module"]
pub type PTR = crate::Reg<u32, _PTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTR;
#[doc = "`read()` method returns [ptr::R](ptr::R) reader structure"]
impl crate::Readable for PTR {}
#[doc = "`write(|w| ..)` method takes [ptr::W](ptr::W) writer structure"]
impl crate::Writable for PTR {}
#[doc = "Data pointer"]
pub mod ptr;
#[doc = "Maximum number of bytes in transmit buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maxcnt](maxcnt) module"]
pub type MAXCNT = crate::Reg<u32, _MAXCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAXCNT;
#[doc = "`read()` method returns [maxcnt::R](maxcnt::R) reader structure"]
impl crate::Readable for MAXCNT {}
#[doc = "`write(|w| ..)` method takes [maxcnt::W](maxcnt::W) writer structure"]
impl crate::Writable for MAXCNT {}
#[doc = "Maximum number of bytes in transmit buffer"]
pub mod maxcnt;
#[doc = "Number of bytes transferred in the last transaction\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amount](amount) module"]
pub type AMOUNT = crate::Reg<u32, _AMOUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AMOUNT;
#[doc = "`read()` method returns [amount::R](amount::R) reader structure"]
impl crate::Readable for AMOUNT {}
#[doc = "Number of bytes transferred in the last transaction"]
pub mod amount;
#[doc = "EasyDMA list type\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [list](list) module"]
pub type LIST = crate::Reg<u32, _LIST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LIST;
#[doc = "`read()` method returns [list::R](list::R) reader structure"]
impl crate::Readable for LIST {}
#[doc = "`write(|w| ..)` method takes [list::W](list::W) writer structure"]
impl crate::Writable for LIST {}
#[doc = "EasyDMA list type"]
pub mod list;
