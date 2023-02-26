use std::collections::HashMap;

use crate::{consts::State, ss::item::Item, ReeFloat, ReeId, ReeInt};

use super::affection_reg::AffectionRegister;

pub(in crate::ss) struct CalcSvc {
    attrs: HashMap<ReeId, ReeFloat>,
    affection: AffectionRegister,
}
impl CalcSvc {
    pub(in crate::ss) fn new() -> CalcSvc {
        CalcSvc {
            attrs: HashMap::new(),
            affection: AffectionRegister::new(),
        }
    }
    // Query methods
    pub(in crate::ss) fn get_modifications(&mut self, afectee_item: &Item, afectee_attr_id: ReeInt) {}
    // Maintenance methods
    pub(in crate::ss) fn add_item(&mut self, item: &Item) {
        self.affection.add_item(item);
    }
    pub(in crate::ss) fn rm_item(&mut self, item: &Item) {
        self.affection.rm_item(item);
    }
    pub(in crate::ss) fn activate_item_state(&mut self, item: &Item, state: State) {
        self.affection.activate_item_state(item, state);
    }
    pub(in crate::ss) fn deactivate_item_state(&mut self, item: &Item, state: State) {
        self.affection.deactivate_item_state(item, state);
    }
}
