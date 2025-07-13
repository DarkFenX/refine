use ordered_float::Float;

use crate::{
    ac,
    def::{AttrVal, ItemKey, OF},
    svc::{
        SvcCtx,
        calc::Calc,
        err::{KeyedItemKindVsStatError, KeyedItemLoadedError, StatItemCheckError},
        vast::Vast,
    },
    uad::{ShipKind, UadItem},
};

// Result of calculation of -math.log(0.25) / 1000000 using 64-bit python 2.7
pub(super) const AGILITY_CONST: AttrVal = OF(f64::from_bits(0x3eb74216c502a54f));

impl Vast {
    pub(in crate::svc) fn get_stat_item_speed_checked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: ItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        item_check(ctx, item_key)?;
        Ok(Vast::get_stat_item_speed_unchecked(ctx, calc, item_key))
    }
    fn get_stat_item_speed_unchecked(ctx: SvcCtx, calc: &mut Calc, item_key: ItemKey) -> AttrVal {
        calc.get_item_attr_val_extra(ctx, item_key, &ac::attrs::MAX_VELOCITY)
            .unwrap()
    }
    pub(in crate::svc) fn get_stat_item_agility_checked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: ItemKey,
    ) -> Result<Option<AttrVal>, StatItemCheckError> {
        item_check(ctx, item_key)?;
        Ok(Vast::get_stat_item_agility_unchecked(ctx, calc, item_key))
    }
    fn get_stat_item_agility_unchecked(ctx: SvcCtx, calc: &mut Calc, item_key: ItemKey) -> Option<AttrVal> {
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
    pub(in crate::svc) fn get_stat_item_align_time_checked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: ItemKey,
    ) -> Result<Option<AttrVal>, StatItemCheckError> {
        item_check(ctx, item_key)?;
        Ok(Vast::get_stat_item_align_time_unchecked(ctx, calc, item_key))
    }
    fn get_stat_item_align_time_unchecked(ctx: SvcCtx, calc: &mut Calc, item_key: ItemKey) -> Option<AttrVal> {
        Vast::get_stat_item_agility_unchecked(ctx, calc, item_key).map(|v| v.ceil())
    }
}

fn item_check(ctx: SvcCtx, item_key: ItemKey) -> Result<(), StatItemCheckError> {
    let uad_item = ctx.uad.items.get(item_key);
    let is_loaded = match uad_item {
        UadItem::Drone(uad_drone) => uad_drone.is_loaded(),
        UadItem::Fighter(uad_fighter) => uad_fighter.is_loaded(),
        UadItem::Ship(uad_ship) => match uad_ship.get_kind() {
            ShipKind::Ship | ShipKind::Unknown => uad_ship.is_loaded(),
            ShipKind::Structure => return Err(KeyedItemKindVsStatError { item_key }.into()),
        },
        _ => return Err(KeyedItemKindVsStatError { item_key }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(KeyedItemLoadedError { item_key }.into()),
    }
}
