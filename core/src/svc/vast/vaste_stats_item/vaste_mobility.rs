use ordered_float::Float;

use crate::{
    ac,
    def::{AttrVal, OF},
    svc::{
        SvcCtx,
        calc::Calc,
        err::{KeyedItemKindVsStatError, KeyedItemLoadedError, StatItemCheckError},
        vast::Vast,
    },
    ud::{UItem, UItemKey, UShipKind},
};

// Result of calculation of -math.log(0.25) / 1000000 using 64-bit python 2.7
pub(super) const AGILITY_CONST: AttrVal = OF(f64::from_bits(0x3eb74216c502a54f));

impl Vast {
    pub(in crate::svc) fn get_stat_item_speed(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        item_check(ctx, item_key)?;
        Ok(Vast::get_stat_item_speed_unchecked(ctx, calc, item_key))
    }
    fn get_stat_item_speed_unchecked(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> AttrVal {
        calc.get_item_attr_val_extra(ctx, item_key, &ac::attrs::MAX_VELOCITY)
            .unwrap()
    }
    pub(in crate::svc) fn get_stat_item_agility(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<Option<AttrVal>, StatItemCheckError> {
        item_check(ctx, item_key)?;
        Ok(Vast::get_stat_item_agility_unchecked(ctx, calc, item_key))
    }
    fn get_stat_item_agility_unchecked(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> Option<AttrVal> {
        let agility = calc
            .get_item_attr_val_extra(ctx, item_key, &ac::attrs::AGILITY)
            .unwrap();
        if agility <= OF(0.0) {
            return None;
        }
        let mass = calc.get_item_attr_val_extra(ctx, item_key, &ac::attrs::MASS).unwrap();
        if mass <= OF(0.0) {
            return None;
        }
        Some(AGILITY_CONST * agility * mass)
    }
    pub(in crate::svc) fn get_stat_item_align_time(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<Option<AttrVal>, StatItemCheckError> {
        item_check(ctx, item_key)?;
        Ok(Vast::get_stat_item_align_time_unchecked(ctx, calc, item_key))
    }
    fn get_stat_item_align_time_unchecked(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> Option<AttrVal> {
        Vast::get_stat_item_agility_unchecked(ctx, calc, item_key).map(|v| v.ceil())
    }
}

fn item_check(ctx: SvcCtx, item_key: UItemKey) -> Result<(), StatItemCheckError> {
    let u_item = ctx.u_data.items.get(item_key);
    let is_loaded = match u_item {
        UItem::Drone(u_drone) => u_drone.is_loaded(),
        UItem::Fighter(u_fighter) => u_fighter.is_loaded(),
        UItem::Ship(u_ship) => match u_ship.get_kind() {
            UShipKind::Ship | UShipKind::Unknown => u_ship.is_loaded(),
            UShipKind::Structure => return Err(KeyedItemKindVsStatError { item_key }.into()),
        },
        _ => return Err(KeyedItemKindVsStatError { item_key }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(KeyedItemLoadedError { item_key }.into()),
    }
}
