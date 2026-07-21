use crate::{
    PValue, Value,
    svc::{Calc, SvcCtx, Vast, err::IntItemStatError, vast::stats::item_checks::check_drone_fighter_ship},
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_mass(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, IntItemStatError<!>> {
        check_drone_fighter_ship(ctx.u_data, item_uid)?;
        Ok(Self::internal_get_stat_item_mass_unchecked(ctx, calc, item_uid))
    }
    pub(in crate::svc::vast::stats) fn internal_get_stat_item_mass_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> PValue {
        let mass = calc.get_item_oattr_ffb_extra(ctx, item_uid, ctx.ac().mass, Value::ZERO);
        PValue::from_value_clamped(mass)
    }
}
