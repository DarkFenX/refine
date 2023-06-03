use std::{collections::HashMap, sync::Arc};

use crate::{ad, consts::State, defs::ReeId, ssi};

use super::{calc::CalcSvc, SsInnerData};

pub(in crate::ss) fn item_added(item: &ssi::Item) {}
pub(in crate::ss) fn item_removed(item: &ssi::Item) {}
pub(in crate::ss) fn state_activated(item: &ssi::Item, state: &State) {}
pub(in crate::ss) fn state_deactivated(item: &ssi::Item, state: &State) {}
pub(in crate::ss) fn item_loaded(item: &ssi::Item, calc: &mut CalcSvc) {
    calc.item_loaded(item);
}
pub(in crate::ss) fn item_unloaded(item: &ssi::Item, calc: &mut CalcSvc) {
    calc.item_unloaded(item);
}
pub(in crate::ss) fn state_activated_loaded(item: &ssi::Item, state: &State) {}
pub(in crate::ss) fn state_deactivated_loaded(item: &ssi::Item, state: &State) {}
pub(in crate::ss) fn effects_started(
    item: &ssi::Item,
    effects: &Vec<Arc<ad::AEffect>>,
    items: &HashMap<ReeId, ssi::Item>,
    calc: &mut CalcSvc,
) {
    calc.effects_started(item, effects, items);
}
pub(in crate::ss) fn effects_stopped(
    item: &ssi::Item,
    effects: &Vec<Arc<ad::AEffect>>,
    items: &HashMap<ReeId, ssi::Item>,
    calc: &mut CalcSvc,
) {
    calc.effects_stopped(item, effects, items);
}
