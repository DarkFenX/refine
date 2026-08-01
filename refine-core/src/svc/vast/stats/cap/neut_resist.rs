use crate::{
    UnitInterval, Value,
    svc::{Calc, SvcCtx, Vast, err::IntStatItemError, vast::stats::item_checks::check_ship},
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_neut_resist(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<UnitInterval, IntStatItemError<!>> {
        check_ship(ctx.u_data, item_uid)?;
        let neut_resist =
            Value::ONE - calc.get_item_oattr_ffb_extra(ctx, item_uid, ctx.ac().energy_warfare_resist, Value::ZERO);
        Ok(UnitInterval::from_value_clamped(neut_resist))
    }
}
