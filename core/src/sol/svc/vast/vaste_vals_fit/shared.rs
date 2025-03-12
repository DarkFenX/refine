use crate::{
    defs::{Count, EAttrId, SolItemId},
    sol::{
        svc::calc::{AttrCalcError, SolCalc},
        uad::SolUad,
    },
    util::TriOption,
};

pub(super) fn get_max_slots(
    uad: &SolUad,
    calc: &mut SolCalc,
    max_item_id: &Option<SolItemId>,
    max_attr_id: &EAttrId,
) -> TriOption<Count> {
    match max_item_id {
        Some(item_id) => match calc.get_item_attr_val_full(uad, item_id, max_attr_id) {
            Ok(val) => TriOption::Some(val.extra.into_inner().round() as Count),
            Err(error) => match error {
                AttrCalcError::ItemNotLoaded(_) => TriOption::Other,
                _ => TriOption::None,
            },
        },
        None => TriOption::None,
    }
}
