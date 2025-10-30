use crate::{
    ac,
    def::{AttrVal, OF},
    svc::{
        SvcCtx,
        calc::Calc,
        err::{KeyedItemKindVsStatError, KeyedItemLoadedError, StatItemCheckError},
        vast::Vast,
    },
    ud::{UItem, UItemKey},
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_cap(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        item_check_ship(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_cap_unchecked(ctx, calc, item_key))
    }
    fn internal_get_stat_item_cap_unchecked(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> AttrVal {
        calc.get_item_attr_val_extra(ctx, item_key, &ac::attrs::CAPACITOR_CAPACITY)
            .unwrap()
    }
    pub(in crate::svc) fn get_stat_item_neut_resist(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        item_check_ship(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_neut_resist_unchecked(ctx, calc, item_key))
    }
    fn internal_get_stat_item_neut_resist_unchecked(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> AttrVal {
        OF(1.0)
            - calc
                .get_item_attr_val_extra(ctx, item_key, &ac::attrs::ENERGY_WARFARE_RESIST)
                .unwrap()
    }
}

fn item_check_ship(ctx: SvcCtx, item_key: UItemKey) -> Result<(), StatItemCheckError> {
    let u_item = ctx.u_data.items.get(item_key);
    let is_loaded = match u_item {
        UItem::Ship(u_ship) => u_ship.is_loaded(),
        _ => return Err(KeyedItemKindVsStatError { item_key }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(KeyedItemLoadedError { item_key }.into()),
    }
}
