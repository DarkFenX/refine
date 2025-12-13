use super::checks::{check_item_key_character, check_item_key_fighter_ship_no_struct};
use crate::{
    def::{AttrVal, OF},
    svc::{SvcCtx, calc::Calc, err::StatItemCheckError, vast::Vast},
    ud::UItemKey,
    util::FLOAT_TOLERANCE,
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
        calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().drone_control_distance, OF(0.0))
            .unwrap()
    }
    pub(in crate::svc) fn get_stat_item_can_warp(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<bool, StatItemCheckError> {
        check_item_key_fighter_ship_no_struct(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_can_warp_unchecked(ctx, calc, item_key))
    }
    fn internal_get_stat_item_can_warp_unchecked(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> bool {
        let scram_status = calc
            .get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().warp_scramble_status, OF(0.0))
            .unwrap();
        if scram_status > FLOAT_TOLERANCE {
            return false;
        }
        // Prevent warp if speed is 0, except for the case when speed attribute is not defined
        if let Some(max_speed) = calc.get_item_oattr_oextra(ctx, item_key, ctx.ac().max_velocity)
            && max_speed < FLOAT_TOLERANCE
        {
            return false;
        }
        true
    }
}
