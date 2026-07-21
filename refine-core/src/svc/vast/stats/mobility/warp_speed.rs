use crate::{
    num::{PValue, Value},
    svc::{
        SvcCtx,
        calc::Calc,
        err::IntStatItemError,
        vast::{Vast, stats::item_checks::check_fighter_ship_no_struct},
    },
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_warp_speed(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<Option<PValue>, IntStatItemError<!>> {
        check_fighter_ship_no_struct(ctx.u_data, item_uid)?;
        let warp_speed = calc.get_item_oattr_ffb_extra(ctx, item_uid, ctx.ac().warp_speed_mult, Value::ZERO);
        let warp_speed = match warp_speed > Value::FLOAT_TOLERANCE {
            true => Some(PValue::from_value_unchecked(warp_speed)),
            false => None,
        };
        Ok(warp_speed)
    }
}
