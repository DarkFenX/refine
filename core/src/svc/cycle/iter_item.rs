use crate::def::AttrVal;

#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleIterItem {
    // Time until next cycle starts
    pub(in crate::svc) time: AttrVal,
    // Is cycle sequence interrupted after this one or not
    pub(in crate::svc) interrupt: bool,
    // How charged current cycle is
    pub(in crate::svc) charged: Option<AttrVal>,
}
impl CycleIterItem {
    pub(super) fn new(time: AttrVal, interrupt: bool, charged: Option<AttrVal>) -> Self {
        Self {
            time,
            interrupt,
            charged,
        }
    }
}
