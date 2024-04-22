use crate::sol::svc::svce_calc::SolAttrMod;

pub(in crate::sol::svc::svce_calc) struct SolFleetUpdates {
    pub(in crate::sol::svc::svce_calc) incoming: Vec<SolAttrMod>,
    pub(in crate::sol::svc::svce_calc) outgoing: Vec<SolAttrMod>,
}
impl SolFleetUpdates {
    pub(in crate::sol::svc::svce_calc::registers::modifier) fn new() -> Self {
        Self {
            incoming: Vec::new(),
            outgoing: Vec::new(),
        }
    }
}
