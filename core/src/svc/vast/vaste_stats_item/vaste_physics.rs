use crate::{
    ac,
    def::{AttrVal, Count, OF},
    svc::{
        SvcCtx,
        calc::Calc,
        err::{KeyedItemKindVsStatError, KeyedItemLoadedError, StatItemCheckError},
        item_funcs,
        vast::Vast,
    },
    ud::{UItem, UItemKey, UShipKind},
    util::{ceil_tick, ceil_unerr},
};

// Result of calculation of -math.log(0.25) / 1000000 using 64-bit python 2.7
pub(super) const AGILITY_CONST: AttrVal = OF(f64::from_bits(0x3eb74216c502a54f));

impl Vast {
    pub(in crate::svc) fn get_stat_item_speed(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        item_check_physic_and_movable(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_speed_unchecked(ctx, calc, item_key))
    }
    fn internal_get_stat_item_speed_unchecked(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> AttrVal {
        item_funcs::get_speed(ctx, calc, item_key).unwrap()
    }
    pub(in crate::svc) fn get_stat_item_agility(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<Option<AttrVal>, StatItemCheckError> {
        item_check_physic_and_movable(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_agility_unchecked(ctx, calc, item_key))
    }
    fn internal_get_stat_item_agility_unchecked(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> Option<AttrVal> {
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
        item_check_physic_and_movable(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_align_time_unchecked(ctx, calc, item_key))
    }
    fn internal_get_stat_item_align_time_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Option<AttrVal> {
        Vast::internal_get_stat_item_agility_unchecked(ctx, calc, item_key).map(ceil_tick)
    }
    pub(in crate::svc) fn get_stat_item_sig_radius(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        item_check_physic(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_sig_radius_unchecked(ctx, calc, item_key))
    }
    fn internal_get_stat_item_sig_radius_unchecked(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> AttrVal {
        item_funcs::get_sig_radius(ctx, calc, item_key).unwrap()
    }
    pub(in crate::svc) fn get_stat_item_mass(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        item_check_physic(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_mass_unchecked(ctx, calc, item_key))
    }
    fn internal_get_stat_item_mass_unchecked(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> AttrVal {
        calc.get_item_attr_val_extra(ctx, item_key, &ac::attrs::MASS).unwrap()
    }
    pub(in crate::svc) fn get_stat_item_locks(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<Count, StatItemCheckError> {
        item_check_physic(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_locks(ctx, calc, item_key))
    }
    fn internal_get_stat_item_locks(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> Count {
        let mut item_locks = calc
            .get_item_attr_val_extra(ctx, item_key, &ac::attrs::MAX_LOCKED_TARGETS)
            .unwrap();
        // Ship locks are limited by character locks. Drone/fighter locks are not limited by it
        let u_item = ctx.u_data.items.get(item_key);
        if let UItem::Ship(u_ship) = u_item {
            let u_fit = ctx.u_data.fits.get(u_ship.get_fit_key());
            if let Some(character_key) = u_fit.character {
                let character_locks = calc
                    .get_item_attr_val_extra(ctx, character_key, &ac::attrs::MAX_LOCKED_TARGETS)
                    .unwrap();
                item_locks = item_locks.min(character_locks)
            }
        }
        // Non-integer locks can happen in Pochven where locks are halved
        ceil_unerr(item_locks).into_inner() as Count
    }
}

fn item_check_physic(ctx: SvcCtx, item_key: UItemKey) -> Result<(), StatItemCheckError> {
    let u_item = ctx.u_data.items.get(item_key);
    let is_loaded = match u_item {
        UItem::Drone(u_drone) => u_drone.is_loaded(),
        UItem::Fighter(u_fighter) => u_fighter.is_loaded(),
        UItem::Ship(u_ship) => u_ship.is_loaded(),
        _ => return Err(KeyedItemKindVsStatError { item_key }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(KeyedItemLoadedError { item_key }.into()),
    }
}

fn item_check_physic_and_movable(ctx: SvcCtx, item_key: UItemKey) -> Result<(), StatItemCheckError> {
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
