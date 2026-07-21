use crate::{
    PValue, Value,
    svc::{Calc, SvcCtx, Vast, err::IntItemStatError, vast::stats::item_checks::check_fighter_ship},
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_lock_range(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, IntItemStatError<!>> {
        check_fighter_ship(ctx.u_data, item_uid)?;
        let lock_range = calc.get_item_oattr_ffb_extra(ctx, item_uid, ctx.ac().max_target_range, Value::ZERO);
        Ok(PValue::from_value_clamped(lock_range))
    }
}
