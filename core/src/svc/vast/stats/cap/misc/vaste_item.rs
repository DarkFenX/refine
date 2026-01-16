use crate::{
    num::{PValue, UnitInterval, Value},
    svc::{
        SvcCtx,
        calc::Calc,
        err::StatItemCheckError,
        vast::{Vast, stats::item_checks::check_ship},
    },
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_cap_amount(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, StatItemCheckError> {
        check_ship(ctx.u_data, item_uid)?;
        Ok(Vast::internal_get_stat_item_cap_amount_unchecked(ctx, calc, item_uid))
    }
    pub(in crate::svc::vast::stats) fn internal_get_stat_item_cap_amount_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> PValue {
        let cap_amount = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().capacitor_capacity, Value::ZERO)
            .unwrap();
        PValue::from_value_clamped(cap_amount)
    }
    pub(in crate::svc::vast::stats::cap) fn internal_get_stat_item_cap_recharge_time_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> PValue {
        let cap_recharge_duration = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().recharge_rate, Value::ZERO)
            .unwrap()
            / Value::THOUSAND;
        PValue::from_value_clamped(cap_recharge_duration)
    }
    pub(in crate::svc) fn get_stat_item_neut_resist(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<UnitInterval, StatItemCheckError> {
        check_ship(ctx.u_data, item_uid)?;
        let neut_resist = Value::ONE
            - calc
                .get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().energy_warfare_resist, Value::ZERO)
                .unwrap();
        Ok(UnitInterval::from_value_clamped(neut_resist))
    }
}
