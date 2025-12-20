use crate::{
    def::{AttrVal, OF},
    svc::{
        SvcCtx,
        calc::Calc,
        err::StatItemCheckError,
        vast::{Vast, vaste_stats::item_checks::check_ship},
    },
    ud::UItemKey,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_cap_amount(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        check_ship(ctx.u_data, item_key)?;
        Ok(Vast::internal_get_stat_item_cap_unchecked(ctx, calc, item_key))
    }
    pub(in crate::svc::vast::vaste_stats) fn internal_get_stat_item_cap_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> AttrVal {
        calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().capacitor_capacity, OF(0.0))
            .unwrap()
    }
    pub(in crate::svc) fn get_stat_item_neut_resist(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        check_ship(ctx.u_data, item_key)?;
        let neut_resist = OF(1.0)
            - calc
                .get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().energy_warfare_resist, OF(0.0))
                .unwrap();
        Ok(neut_resist)
    }
}
