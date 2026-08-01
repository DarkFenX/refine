use crate::{
    PValue, Value,
    svc::{Calc, SvcCtx, Vast, err::IntStatItemError, vast::stats::item_checks::check_fighter_ship},
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_scan_res(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, IntStatItemError<!>> {
        check_fighter_ship(ctx.u_data, item_uid)?;
        let scan_res = calc.get_item_oattr_ffb_extra(ctx, item_uid, ctx.ac().scan_resolution, Value::ZERO);
        Ok(PValue::from_value_clamped(scan_res))
    }
}
