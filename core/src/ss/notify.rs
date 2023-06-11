use crate::{ad, consts::State, defs::ReeId, ssi};

use super::SsInnerData;

pub(in crate::ss) fn item_added(item: &ssi::SsItem) {}
pub(in crate::ss) fn item_removed(item: &ssi::SsItem) {}
pub(in crate::ss) fn state_activated(item: &ssi::SsItem, state: &State) {}
pub(in crate::ss) fn state_deactivated(item: &ssi::SsItem, state: &State) {}
pub(in crate::ss) fn item_loaded(item: &ssi::SsItem, ss_data: &mut SsInnerData) {
    ss_data.calc.item_loaded(item);
}
pub(in crate::ss) fn item_unloaded(item: &ssi::SsItem, ss_data: &mut SsInnerData) {
    ss_data.calc.item_unloaded(item);
}
pub(in crate::ss) fn state_activated_loaded(item: &ssi::SsItem, state: &State) {}
pub(in crate::ss) fn state_deactivated_loaded(item: &ssi::SsItem, state: &State) {}
pub(in crate::ss) fn effects_started(item: &ssi::SsItem, effects: &Vec<ad::ArcEffect>, ss_data: &mut SsInnerData) {
    ss_data.calc.effects_started(item, effects, ss_data.items);
}
pub(in crate::ss) fn effects_stopped(item: &ssi::SsItem, effects: &Vec<ad::ArcEffect>, ss_data: &mut SsInnerData) {
    ss_data.calc.effects_stopped(item, effects, ss_data.items);
}
pub(in crate::ss) fn attr_val_changed(item: &ssi::SsItem, attr_id: ReeId, ss_data: &mut SsInnerData) {}
