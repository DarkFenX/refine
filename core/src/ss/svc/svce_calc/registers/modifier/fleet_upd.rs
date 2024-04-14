use crate::ss::svc::svce_calc::modifier::SsAttrMod;

pub(in crate::ss::svc::svce_calc) struct FleetUpdates {
    pub(in crate::ss::svc::svce_calc) incoming: Vec<SsAttrMod>,
    pub(in crate::ss::svc::svce_calc) outgoing: Vec<SsAttrMod>,
}
impl FleetUpdates {
    pub(in crate::ss::svc::svce_calc::registers::modifier) fn new() -> Self {
        Self {
            incoming: Vec::new(),
            outgoing: Vec::new(),
        }
    }
}
