use crate::{
    PValue, Value,
    svc::{Calc, SvcCtx, Vast, err::IntItemStatError, vast::stats::item_checks::check_character},
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_drone_control_range(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, IntItemStatError<!>> {
        check_character(ctx.u_data, item_uid)?;
        let drone_control_range =
            calc.get_item_oattr_ffb_extra(ctx, item_uid, ctx.ac().drone_control_distance, Value::ZERO);
        Ok(PValue::from_value_clamped(drone_control_range))
    }
}
