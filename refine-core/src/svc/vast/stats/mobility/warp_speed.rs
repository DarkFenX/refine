use crate::{
    PValue, Value,
    svc::{Calc, SvcCtx, Vast, err::IntStatItemError, vast::stats::item_checks::check_fighter_ship_no_struct},
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_warp_speed(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, IntStatItemError<StatWarpSpeedError>> {
        check_fighter_ship_no_struct(ctx.u_data, item_uid)?;
        Self::internal_get_stat_item_warp_speed_unchecked(ctx, calc, item_uid).map_err(IntStatItemError::StatSpecific)
    }
    fn internal_get_stat_item_warp_speed_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, StatWarpSpeedError> {
        let warp_speed = calc.get_item_oattr_ffb_extra(ctx, item_uid, ctx.ac().warp_speed_mult, Value::ZERO);
        match warp_speed > Value::FLOAT_TOLERANCE {
            true => Ok(PValue::from_value_unchecked(warp_speed)),
            false => Err(StatWarpSpeedError::WarpSpeedError(warp_speed)),
        }
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum StatWarpSpeedError {
    #[error("warp speed should be > 0, but is {0}")]
    WarpSpeedError(Value),
}
