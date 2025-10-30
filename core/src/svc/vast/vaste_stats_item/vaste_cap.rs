use super::checks::check_item_key_ship;
use crate::{
    ac,
    def::{AttrVal, OF},
    svc::{SvcCtx, calc::Calc, err::StatItemCheckError, vast::Vast},
    ud::UItemKey,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_cap(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        check_item_key_ship(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_cap_unchecked(ctx, calc, item_key))
    }
    pub(super) fn internal_get_stat_item_cap_unchecked(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> AttrVal {
        calc.get_item_attr_val_extra(ctx, item_key, &ac::attrs::CAPACITOR_CAPACITY)
            .unwrap()
    }
    pub(in crate::svc) fn get_stat_item_cap_regen(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        cap_perc: Option<AttrVal>,
    ) -> Result<Option<AttrVal>, StatItemCheckError> {
        check_item_key_ship(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_cap_regen_unchecked(
            ctx, calc, item_key, cap_perc,
        ))
    }
    fn internal_get_stat_item_cap_regen_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        cap_perc: Option<AttrVal>,
    ) -> Option<AttrVal> {
        let max_amount = Vast::internal_get_stat_item_cap_unchecked(ctx, calc, item_key);
        let cap_regen_time = calc
            .get_item_attr_val_extra(ctx, item_key, &ac::attrs::RECHARGE_RATE)
            .unwrap()
            / OF(1000.0);
        let cap_perc = match cap_perc {
            Some(cap_perc) => cap_perc.clamp(OF(0.0), OF(1.0)),
            None => OF(0.25),
        };
        let result = OF(10.0) * max_amount / cap_regen_time * (OF(cap_perc.sqrt()) - cap_perc);
        match result.is_finite() {
            true => Some(result),
            false => None,
        }
    }
    pub(in crate::svc) fn get_stat_item_neut_resist(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        check_item_key_ship(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_neut_resist_unchecked(ctx, calc, item_key))
    }
    fn internal_get_stat_item_neut_resist_unchecked(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> AttrVal {
        OF(1.0)
            - calc
                .get_item_attr_val_extra(ctx, item_key, &ac::attrs::ENERGY_WARFARE_RESIST)
                .unwrap()
    }
}
