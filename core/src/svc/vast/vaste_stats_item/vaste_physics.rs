use crate::{
    ac,
    def::{AttrVal, OF},
    svc::{
        SvcCtx,
        calc::Calc,
        err::{KeyedItemKindVsStatError, KeyedItemLoadedError, StatItemCheckError},
        item_funcs,
        vast::Vast,
    },
    ud::{UItem, UItemKey, UShipKind},
    util::ceil_tick,
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
    pub(super) fn internal_get_stat_item_sig_radius_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> AttrVal {
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
    pub(in crate::svc) fn get_stat_item_warp_speed(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<Option<AttrVal>, StatItemCheckError> {
        item_check_warpable(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_warp_speed_unchecked(ctx, calc, item_key))
    }
    fn internal_get_stat_item_warp_speed_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Option<AttrVal> {
        let base = calc
            .get_item_attr_val_extra(ctx, item_key, &ac::attrs::BASE_WARP_SPEED)
            .unwrap();
        let mult = calc
            .get_item_attr_val_extra(ctx, item_key, &ac::attrs::WARP_SPEED_MULTIPLIER)
            .unwrap();
        let result = base * mult;
        match result > OF(0.0) {
            true => Some(result),
            false => None,
        }
    }
    pub(in crate::svc) fn get_stat_item_max_warp_range(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<Option<AttrVal>, StatItemCheckError> {
        item_check_warpable(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_max_warp_range_unchecked(
            ctx, calc, item_key,
        ))
    }
    fn internal_get_stat_item_max_warp_range_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Option<AttrVal> {
        // TODO: switch to using unchecked capacitor stat instead of direct attribute fetch, once
        // TODO: the stat is implemented
        let cap = calc
            .get_item_attr_val_extra(ctx, item_key, &ac::attrs::CAPACITOR_CAPACITY)
            .unwrap();
        let mass = Vast::internal_get_stat_item_mass_unchecked(ctx, calc, item_key);
        let warp_cap = calc
            .get_item_attr_val_extra(ctx, item_key, &ac::attrs::WARP_CAPACITOR_NEED)
            .unwrap();
        let result = cap / mass / warp_cap;
        match result.is_finite() && result > OF(0.0) {
            true => Some(result),
            false => None,
        }
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

fn item_check_warpable(ctx: SvcCtx, item_key: UItemKey) -> Result<(), StatItemCheckError> {
    let u_item = ctx.u_data.items.get(item_key);
    let is_loaded = match u_item {
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
