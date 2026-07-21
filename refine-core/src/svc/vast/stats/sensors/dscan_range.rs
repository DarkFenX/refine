use crate::{
    PValue, Value,
    svc::{Calc, SvcCtx, Vast, err::IntItemStatError, vast::stats::item_checks::check_ship},
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_dscan_range(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, IntItemStatError<!>> {
        check_ship(ctx.u_data, item_uid)?;
        let dscan_range =
            calc.get_item_oattr_ffb_extra(ctx, item_uid, ctx.ac().max_directional_scan_range, Value::ZERO) / Value::AU;
        Ok(PValue::from_value_clamped(dscan_range))
    }
}
