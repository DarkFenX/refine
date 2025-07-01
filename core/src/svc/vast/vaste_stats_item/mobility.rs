use ordered_float::Float;

use crate::{
    ac,
    def::{AttrVal, ItemKey, OF},
    svc::{SvcCtx, calc::Calc, vast::Vast},
};

// Result of calculation of -math.log(0.25) / 1000000 using 64-bit python 2.7
pub(super) const AGILITY_CONST: AttrVal = OF(f64::from_bits(0x3eb74216c502a54f));

impl Vast {
    pub(in crate::svc) fn get_stat_item_speed(ctx: SvcCtx, calc: &mut Calc, item_key: ItemKey) -> Option<AttrVal> {
        calc.get_item_attr_val_extra(ctx, item_key, &ac::attrs::MAX_VELOCITY)
    }
    pub(in crate::svc) fn get_stat_item_agility(ctx: SvcCtx, calc: &mut Calc, item_key: ItemKey) -> Option<AttrVal> {
        let agility = calc.get_item_attr_val_extra(ctx, item_key, &ac::attrs::AGILITY)?;
        if agility == OF(0.0) {
            return None;
        }
        let mass = calc.get_item_attr_val_extra(ctx, item_key, &ac::attrs::MASS)?;
        if mass == OF(0.0) {
            return None;
        }
        Some(AGILITY_CONST * agility * mass)
    }
    pub(in crate::svc) fn get_stat_item_align_time(ctx: SvcCtx, calc: &mut Calc, item_key: ItemKey) -> Option<AttrVal> {
        Vast::get_stat_item_agility(ctx, calc, item_key).map(|v| v.ceil())
    }
}
