use crate::{
    num::{PValue, Value},
    svc::{
        SvcCtx,
        calc::Calc,
        err::IntStatItemError,
        vast::{Vast, stats::item_checks::check_ship_no_struct},
    },
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_max_warp_range(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<Option<PValue>, IntStatItemError<!>> {
        check_ship_no_struct(ctx.u_data, item_uid)?;
        let cap = Self::internal_get_stat_item_cap_amount_unchecked(ctx, calc, item_uid);
        let mass = Self::internal_get_stat_item_mass_unchecked(ctx, calc, item_uid);
        let cap_need = PValue::from_value_clamped(calc.get_item_oattr_ffb_extra(
            ctx,
            item_uid,
            ctx.ac().warp_capacitor_need,
            Value::ZERO,
        ));
        let warp_range = cap / mass / cap_need;
        let warp_range = match warp_range.is_finite() && warp_range > PValue::FLOAT_TOLERANCE {
            true => Some(warp_range),
            false => None,
        };
        Ok(warp_range)
    }
}
