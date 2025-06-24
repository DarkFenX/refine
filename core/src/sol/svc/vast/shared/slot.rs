use crate::{
    ad,
    sol::{
        Count, ItemKey,
        svc::{calc::Calc, eprojs::EProjs},
        uad::Uad,
    },
};

pub(in crate::sol::svc::vast) fn get_attr_as_count(
    uad: &Uad,
    eprojs: &EProjs,
    calc: &mut Calc,
    max_item_key: Option<ItemKey>,
    max_a_attr_id: &ad::AAttrId,
) -> Option<Count> {
    calc.get_item_attr_val_extra_opt(uad, eprojs, max_item_key, max_a_attr_id)
        .map(|v| v.round() as Count)
}
