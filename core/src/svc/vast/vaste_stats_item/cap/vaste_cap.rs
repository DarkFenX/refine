use super::super::checks::check_item_key_ship;
use crate::{
    ac,
    def::{AttrVal, OF},
    svc::{SvcCtx, calc::Calc, err::StatItemCheckError, vast::Vast},
    ud::UItemKey,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_cap_amount(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        check_item_key_ship(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_cap_unchecked(ctx, calc, item_key))
    }
    pub(in crate::svc::vast) fn internal_get_stat_item_cap_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> AttrVal {
        calc.get_item_attr_val_extra(ctx, item_key, &ac::attrs::CAPACITOR_CAPACITY)
            .unwrap()
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
