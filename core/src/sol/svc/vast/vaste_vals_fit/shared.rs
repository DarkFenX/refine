use crate::{
    defs::{AttrVal, Count, EAttrId, OF, SolItemId},
    sol::{svc::calc::SolCalc, uad::SolUad},
};

pub(super) fn get_max_resource(
    uad: &SolUad,
    calc: &mut SolCalc,
    max_item_id: &Option<SolItemId>,
    max_attr_id: &EAttrId,
) -> Option<AttrVal> {
    calc.get_item_attr_val_simple_opt(uad, max_item_id, max_attr_id)
}

pub(super) fn get_max_slots(
    uad: &SolUad,
    calc: &mut SolCalc,
    max_item_id: &Option<SolItemId>,
    max_attr_id: &EAttrId,
) -> Option<Count> {
    calc.get_item_attr_val_simple_opt(uad, max_item_id, max_attr_id)
        .map(|v| v.into_inner().round() as Count)
}

pub(super) fn is_flag_set(uad: &SolUad, calc: &mut SolCalc, item_id: &SolItemId, attr_id: &EAttrId) -> bool {
    match calc.get_item_attr_val_simple(uad, item_id, attr_id) {
        Some(val) => val != OF(0.0),
        None => match uad.items.get_item(item_id).unwrap().get_attrs().unwrap().get(attr_id) {
            Some(val) => *val != OF(0.0),
            None => false,
        },
    }
}
