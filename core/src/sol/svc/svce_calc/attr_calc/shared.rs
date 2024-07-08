use crate::{defs::EItemCatId, ec};

const PENALTY_IMMUNE_CATS: [EItemCatId; 5] = [
    ec::itemcats::SHIP,
    ec::itemcats::CHARGE,
    ec::itemcats::SKILL,
    ec::itemcats::IMPLANT,
    ec::itemcats::SUBSYSTEM,
];
// Source expression: 1 / e^((1 / 2.67)^2)
pub(super) const PENALTY_BASE: f64 = 0.86911998080039742919922218788997270166873931884765625;

pub(in crate::sol::svc::svce_calc) fn is_penal(attr_penalizable: bool, affector_item_cat_id: &EItemCatId) -> bool {
    attr_penalizable && !PENALTY_IMMUNE_CATS.contains(affector_item_cat_id)
}
