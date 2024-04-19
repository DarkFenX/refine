use crate::ss::svc::svce_calc::SsAttrMod;

pub(in crate::ss::svc::svce_calc) struct SsFleetUpdates {
    pub(in crate::ss::svc::svce_calc) incoming: Vec<SsAttrMod>,
    pub(in crate::ss::svc::svce_calc) outgoing: Vec<SsAttrMod>,
}
impl SsFleetUpdates {
    pub(in crate::ss::svc::svce_calc::registers::modifier) fn new() -> Self {
        Self {
            incoming: Vec::new(),
            outgoing: Vec::new(),
        }
    }
}
