use crate::{
    PValue, Value,
    svc::{Calc, SvcCtx, Vast, err::IntItemStatError, vast::stats::item_checks::check_fighter_ship_no_struct},
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_warp_speed(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, IntItemStatError<WarpSpeedStatError>> {
        check_fighter_ship_no_struct(ctx.u_data, item_uid)?;
        Self::internal_get_stat_item_warp_speed_unchecked(ctx, calc, item_uid).map_err(IntItemStatError::StatSpecific)
    }
    fn internal_get_stat_item_warp_speed_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, WarpSpeedStatError> {
        let warp_speed = calc.get_item_oattr_ffb_extra(ctx, item_uid, ctx.ac().warp_speed_mult, Value::ZERO);
        match warp_speed > Value::FLOAT_TOLERANCE {
            true => Ok(PValue::from_value_unchecked(warp_speed)),
            false => Err(WarpSpeedStatError::WarpSpeedError(warp_speed)),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum WarpSpeedStatError {
    #[error("warp speed should be > 0, but is {0}")]
    WarpSpeedError(Value),
}
