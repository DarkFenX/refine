use crate::{
    ad,
    sol::{Count, ItemKey, svc::calc::Calc, uad::Uad},
};

pub(in crate::sol::svc::vast) fn get_attr_as_count(
    uad: &Uad,
    calc: &mut Calc,
    max_item_key: Option<ItemKey>,
    max_a_attr_id: &ad::AAttrId,
) -> Option<Count> {
    calc.get_item_attr_val_extra_opt(uad, max_item_key, max_a_attr_id)
        .map(|v| v.round() as Count)
}
