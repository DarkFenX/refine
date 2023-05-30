use std::{collections::HashMap, sync::Arc};

use crate::{consts::State, ct, defs::ReeId};

use super::{calc::CalcSvc, item::Item};

pub(in crate::ss) fn item_added(item: &Item) {}
pub(in crate::ss) fn item_removed(item: &Item) {}
pub(in crate::ss) fn state_activated(item: &Item, state: &State) {}
pub(in crate::ss) fn state_deactivated(item: &Item, state: &State) {}
pub(in crate::ss) fn item_loaded(item: &Item, calc: &mut CalcSvc) {
    calc.item_loaded(item);
}
pub(in crate::ss) fn item_unloaded(item: &Item, calc: &mut CalcSvc) {
    calc.item_unloaded(item);
}
pub(in crate::ss) fn state_activated_loaded(item: &Item, state: &State) {}
pub(in crate::ss) fn state_deactivated_loaded(item: &Item, state: &State) {}
pub(in crate::ss) fn effects_started(
    item: &Item,
    effects: &Vec<Arc<ct::Effect>>,
    items: &HashMap<ReeId, Item>,
    calc: &mut CalcSvc,
) {
    calc.effects_started(item, effects, items);
}
pub(in crate::ss) fn effects_stopped(
    item: &Item,
    effects: &Vec<Arc<ct::Effect>>,
    items: &HashMap<ReeId, Item>,
    calc: &mut CalcSvc,
) {
    calc.effects_stopped(item, effects, items);
}
