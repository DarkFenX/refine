use crate::sol::svc::svce_calc::SolModifier;

pub(in crate::sol::svc::svce_calc) struct SolFleetUpdates {
    pub(in crate::sol::svc::svce_calc) incoming: Vec<SolModifier>,
    pub(in crate::sol::svc::svce_calc) outgoing: Vec<SolModifier>,
}
impl SolFleetUpdates {
    pub(in crate::sol::svc::svce_calc::registers::modifier) fn new() -> Self {
        Self {
            incoming: Vec::new(),
            outgoing: Vec::new(),
        }
    }
}
