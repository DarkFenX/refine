use super::checks::check_item_key_character;
use crate::{
    ac,
    def::AttrVal,
    svc::{SvcCtx, calc::Calc, err::StatItemCheckError, vast::Vast},
    ud::UItemKey,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_drone_control_range(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        check_item_key_character(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_drone_control_range_unchecked(
            ctx, calc, item_key,
        ))
    }
    fn internal_get_stat_item_drone_control_range_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> AttrVal {
        calc.get_item_attr_val_extra(ctx, item_key, &ac::attrs::DRONE_CONTROL_DISTANCE)
            .unwrap()
    }
}
