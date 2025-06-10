use ordered_float::{Float, OrderedFloat as OF};

use crate::{
    ac,
    sol::{
        AttrVal, ItemKey,
        svc::{calc::Calc, vast::Vast},
        uad::Uad,
    },
};

// Result of calculation of -math.log(0.25) / 1000000 using 64-bit python 2.7
pub(super) const AGILITY_CONST: AttrVal = OF(f64::from_bits(0x3eb74216c502a54f));

impl Vast {
    pub(in crate::sol) fn get_item_agility_factor(uad: &Uad, calc: &mut Calc, item_key: ItemKey) -> Option<AttrVal> {
        let agility = calc.get_item_attr_val_extra(uad, item_key, &ac::attrs::AGILITY)?;
        if agility == OF(0.0) {
            return None;
        }
        let mass = calc.get_item_attr_val_extra(uad, item_key, &ac::attrs::MASS)?;
        if mass == OF(0.0) {
            return None;
        }
        Some(AGILITY_CONST * agility * mass)
    }
    pub(in crate::sol) fn get_align_time(uad: &Uad, calc: &mut Calc, item_key: ItemKey) -> Option<AttrVal> {
        Vast::get_item_agility_factor(uad, calc, item_key).map(|v| v.ceil())
    }
}
