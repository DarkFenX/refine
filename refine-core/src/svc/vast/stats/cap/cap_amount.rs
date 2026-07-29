use crate::{
    PValue, Value,
    svc::{Calc, SvcCtx, Vast, err::IntItemStatError, vast::stats::item_checks::check_ship},
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_cap_amount(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, IntItemStatError<!>> {
        check_ship(ctx.u_data, item_uid)?;
        Ok(Self::internal_get_stat_item_cap_amount_unchecked(ctx, calc, item_uid))
    }
    pub(in crate::svc::vast::stats) fn internal_get_stat_item_cap_amount_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> PValue {
        let cap_amount = calc.get_item_oattr_ffb_extra(ctx, item_uid, ctx.ac().capacitor_capacity, Value::ZERO);
        PValue::from_value_clamped(cap_amount)
    }
}
