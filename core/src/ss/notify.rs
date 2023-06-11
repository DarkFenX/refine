use std::collections::HashMap;

use crate::{ad, consts::State, defs::ReeId, ssi};

use super::{calc::CalcSvc, SsInnerData};

pub(in crate::ss) fn item_added(item: &ssi::SsItem) {}
pub(in crate::ss) fn item_removed(item: &ssi::SsItem) {}
pub(in crate::ss) fn state_activated(item: &ssi::SsItem, state: &State) {}
pub(in crate::ss) fn state_deactivated(item: &ssi::SsItem, state: &State) {}
pub(in crate::ss) fn item_loaded(item: &ssi::SsItem, calc: &mut CalcSvc) {
    calc.item_loaded(item);
}
pub(in crate::ss) fn item_unloaded(item: &ssi::SsItem, calc: &mut CalcSvc) {
    calc.item_unloaded(item);
}
pub(in crate::ss) fn state_activated_loaded(item: &ssi::SsItem, state: &State) {}
pub(in crate::ss) fn state_deactivated_loaded(item: &ssi::SsItem, state: &State) {}
pub(in crate::ss) fn effects_started(
    item: &ssi::SsItem,
    effects: &Vec<ad::ArcEffect>,
    items: &HashMap<ReeId, ssi::SsItem>,
    calc: &mut CalcSvc,
) {
    calc.effects_started(item, effects, items);
}
pub(in crate::ss) fn effects_stopped(
    item: &ssi::SsItem,
    effects: &Vec<ad::ArcEffect>,
    items: &HashMap<ReeId, ssi::SsItem>,
    calc: &mut CalcSvc,
) {
    calc.effects_stopped(item, effects, items);
}
