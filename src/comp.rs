#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start comparator"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop comparator"]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x08 - Sample comparator value"]
    pub tasks_sample: TASKS_SAMPLE,
    _reserved3: [u8; 244usize],
    #[doc = "0x100 - COMP is ready and output is valid"]
    pub events_ready: EVENTS_READY,
    #[doc = "0x104 - Downward crossing"]
    pub events_down: EVENTS_DOWN,
    #[doc = "0x108 - Upward crossing"]
    pub events_up: EVENTS_UP,
    #[doc = "0x10c - Downward or upward crossing"]
    pub events_cross: EVENTS_CROSS,
    _reserved7: [u8; 240usize],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: SHORTS,
    _reserved8: [u8; 252usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved11: [u8; 244usize],
    #[doc = "0x400 - Compare result"]
    pub result: RESULT,
    _reserved12: [u8; 252usize],
    #[doc = "0x500 - COMP enable"]
    pub enable: ENABLE,
    #[doc = "0x504 - Pin select"]
    pub psel: PSEL,
    #[doc = "0x508 - Reference source select for single-ended mode"]
    pub refsel: REFSEL,
    #[doc = "0x50c - External reference select"]
    pub extrefsel: EXTREFSEL,
    _reserved16: [u8; 32usize],
    #[doc = "0x530 - Threshold configuration for hysteresis unit"]
    pub th: TH,
    #[doc = "0x534 - Mode configuration"]
    pub mode: MODE,
    #[doc = "0x538 - Comparator hysteresis enable"]
    pub hyst: HYST,
}
#[doc = "Start comparator\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_start](tasks_start) module"]
pub type TASKS_START = crate::Reg<u32, _TASKS_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_START;
#[doc = "`write(|w| ..)` method takes [tasks_start::W](tasks_start::W) writer structure"]
impl crate::Writable for TASKS_START {}
#[doc = "Start comparator"]
pub mod tasks_start;
#[doc = "Stop comparator\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_stop](tasks_stop) module"]
pub type TASKS_STOP = crate::Reg<u32, _TASKS_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOP;
#[doc = "`write(|w| ..)` method takes [tasks_stop::W](tasks_stop::W) writer structure"]
impl crate::Writable for TASKS_STOP {}
#[doc = "Stop comparator"]
pub mod tasks_stop;
#[doc = "Sample comparator value\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_sample](tasks_sample) module"]
pub type TASKS_SAMPLE = crate::Reg<u32, _TASKS_SAMPLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_SAMPLE;
#[doc = "`write(|w| ..)` method takes [tasks_sample::W](tasks_sample::W) writer structure"]
impl crate::Writable for TASKS_SAMPLE {}
#[doc = "Sample comparator value"]
pub mod tasks_sample;
#[doc = "COMP is ready and output is valid\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_ready](events_ready) module"]
pub type EVENTS_READY = crate::Reg<u32, _EVENTS_READY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_READY;
#[doc = "`read()` method returns [events_ready::R](events_ready::R) reader structure"]
impl crate::Readable for EVENTS_READY {}
#[doc = "`write(|w| ..)` method takes [events_ready::W](events_ready::W) writer structure"]
impl crate::Writable for EVENTS_READY {}
#[doc = "COMP is ready and output is valid"]
pub mod events_ready;
#[doc = "Downward crossing\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_down](events_down) module"]
pub type EVENTS_DOWN = crate::Reg<u32, _EVENTS_DOWN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_DOWN;
#[doc = "`read()` method returns [events_down::R](events_down::R) reader structure"]
impl crate::Readable for EVENTS_DOWN {}
#[doc = "`write(|w| ..)` method takes [events_down::W](events_down::W) writer structure"]
impl crate::Writable for EVENTS_DOWN {}
#[doc = "Downward crossing"]
pub mod events_down;
#[doc = "Upward crossing\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_up](events_up) module"]
pub type EVENTS_UP = crate::Reg<u32, _EVENTS_UP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_UP;
#[doc = "`read()` method returns [events_up::R](events_up::R) reader structure"]
impl crate::Readable for EVENTS_UP {}
#[doc = "`write(|w| ..)` method takes [events_up::W](events_up::W) writer structure"]
impl crate::Writable for EVENTS_UP {}
#[doc = "Upward crossing"]
pub mod events_up;
#[doc = "Downward or upward crossing\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_cross](events_cross) module"]
pub type EVENTS_CROSS = crate::Reg<u32, _EVENTS_CROSS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_CROSS;
#[doc = "`read()` method returns [events_cross::R](events_cross::R) reader structure"]
impl crate::Readable for EVENTS_CROSS {}
#[doc = "`write(|w| ..)` method takes [events_cross::W](events_cross::W) writer structure"]
impl crate::Writable for EVENTS_CROSS {}
#[doc = "Downward or upward crossing"]
pub mod events_cross;
#[doc = "Shortcuts between local events and tasks\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shorts](shorts) module"]
pub type SHORTS = crate::Reg<u32, _SHORTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHORTS;
#[doc = "`read()` method returns [shorts::R](shorts::R) reader structure"]
impl crate::Readable for SHORTS {}
#[doc = "`write(|w| ..)` method takes [shorts::W](shorts::W) writer structure"]
impl crate::Writable for SHORTS {}
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "Enable or disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "Enable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u32, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u32, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "Compare result\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result](result) module"]
pub type RESULT = crate::Reg<u32, _RESULT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESULT;
#[doc = "`read()` method returns [result::R](result::R) reader structure"]
impl crate::Readable for RESULT {}
#[doc = "Compare result"]
pub mod result;
#[doc = "COMP enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "COMP enable"]
pub mod enable;
#[doc = "Pin select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psel](psel) module"]
pub type PSEL = crate::Reg<u32, _PSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSEL;
#[doc = "`read()` method returns [psel::R](psel::R) reader structure"]
impl crate::Readable for PSEL {}
#[doc = "`write(|w| ..)` method takes [psel::W](psel::W) writer structure"]
impl crate::Writable for PSEL {}
#[doc = "Pin select"]
pub mod psel;
#[doc = "Reference source select for single-ended mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refsel](refsel) module"]
pub type REFSEL = crate::Reg<u32, _REFSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFSEL;
#[doc = "`read()` method returns [refsel::R](refsel::R) reader structure"]
impl crate::Readable for REFSEL {}
#[doc = "`write(|w| ..)` method takes [refsel::W](refsel::W) writer structure"]
impl crate::Writable for REFSEL {}
#[doc = "Reference source select for single-ended mode"]
pub mod refsel;
#[doc = "External reference select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extrefsel](extrefsel) module"]
pub type EXTREFSEL = crate::Reg<u32, _EXTREFSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTREFSEL;
#[doc = "`read()` method returns [extrefsel::R](extrefsel::R) reader structure"]
impl crate::Readable for EXTREFSEL {}
#[doc = "`write(|w| ..)` method takes [extrefsel::W](extrefsel::W) writer structure"]
impl crate::Writable for EXTREFSEL {}
#[doc = "External reference select"]
pub mod extrefsel;
#[doc = "Threshold configuration for hysteresis unit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [th](th) module"]
pub type TH = crate::Reg<u32, _TH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TH;
#[doc = "`read()` method returns [th::R](th::R) reader structure"]
impl crate::Readable for TH {}
#[doc = "`write(|w| ..)` method takes [th::W](th::W) writer structure"]
impl crate::Writable for TH {}
#[doc = "Threshold configuration for hysteresis unit"]
pub mod th;
#[doc = "Mode configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](mode) module"]
pub type MODE = crate::Reg<u32, _MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE;
#[doc = "`read()` method returns [mode::R](mode::R) reader structure"]
impl crate::Readable for MODE {}
#[doc = "`write(|w| ..)` method takes [mode::W](mode::W) writer structure"]
impl crate::Writable for MODE {}
#[doc = "Mode configuration"]
pub mod mode;
#[doc = "Comparator hysteresis enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hyst](hyst) module"]
pub type HYST = crate::Reg<u32, _HYST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HYST;
#[doc = "`read()` method returns [hyst::R](hyst::R) reader structure"]
impl crate::Readable for HYST {}
#[doc = "`write(|w| ..)` method takes [hyst::W](hyst::W) writer structure"]
impl crate::Writable for HYST {}
#[doc = "Comparator hysteresis enable"]
pub mod hyst;
