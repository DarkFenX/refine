use crate::{consts::State, ss::item::Item, ReeId, ReeInt};

pub(in crate::ss::calc) struct AffectionRegister {}
impl AffectionRegister {
    pub(in crate::ss::calc) fn new() -> AffectionRegister {
        AffectionRegister {}
    }
    // Query methods
    pub(in crate::ss::calc) fn get_local_affectee_items(&mut self, affector_spec: ReeId) {}
    pub(in crate::ss::calc) fn get_projected_affectee_items(&mut self, affector_spec: ReeId, tgt_items: ReeId) {}
    pub(in crate::ss::calc) fn get_affector_specs(&mut self, affectee_item: ReeId) {}
    // Maintenance methods
    pub(in crate::ss::calc) fn add_item(&mut self, item: &Item) {}
    pub(in crate::ss::calc) fn rm_item(&mut self, item: &Item) {}
    pub(in crate::ss::calc) fn activate_item_state(&mut self, item: &Item, state: State) {}
    pub(in crate::ss::calc) fn deactivate_item_state(&mut self, item: &Item, state: State) {}
}
