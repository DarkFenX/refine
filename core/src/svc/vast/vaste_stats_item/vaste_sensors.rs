use crate::{
    ac,
    def::{AttrVal, Count, OF},
    svc::{
        SvcCtx,
        calc::Calc,
        err::{KeyedItemKindVsStatError, KeyedItemLoadedError, StatItemCheckError},
        vast::Vast,
    },
    ud::{UItem, UItemKey},
    util::round_unerr,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_locks(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<Count, StatItemCheckError> {
        item_check_sensors(ctx, item_key)?;
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
            if let Some(character_key) = u_fit.character
                && let Ok(character_locks) =
                    calc.get_item_attr_val_extra(ctx, character_key, &ac::attrs::MAX_LOCKED_TARGETS)
            {
                item_locks = item_locks.min(character_locks)
            }
        }
        // Non-integer locks can happen in Pochven where locks are halved, halves are rounded up
        round_unerr(item_locks).into_inner() as Count
    }
    pub(in crate::svc) fn get_stat_item_lock_range(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        item_check_sensors_no_drones(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_lock_range_unchecked(ctx, calc, item_key))
    }
    fn internal_get_stat_item_lock_range_unchecked(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> AttrVal {
        calc.get_item_attr_val_extra(ctx, item_key, &ac::attrs::MAX_TARGET_RANGE)
            .unwrap()
    }
    pub(in crate::svc) fn get_stat_item_scan_res(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        item_check_sensors_no_drones(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_scan_res_unchecked(ctx, calc, item_key))
    }
    fn internal_get_stat_item_scan_res_unchecked(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> AttrVal {
        calc.get_item_attr_val_extra(ctx, item_key, &ac::attrs::SCAN_RESOLUTION)
            .unwrap()
    }
    pub(in crate::svc) fn get_stat_item_sensor(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<Sensor, StatItemCheckError> {
        item_check_sensors(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_sensor_unchecked(ctx, calc, item_key))
    }
    fn internal_get_stat_item_sensor_unchecked(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> Sensor {
        // Strength ties are resolved using the following order:
        // Radar > ladar > magnetometric > gravimetric
        let str_radar = calc
            .get_item_attr_val_extra(ctx, item_key, &ac::attrs::SCAN_RADAR_STRENGTH)
            .unwrap();
        let str_ladar = calc
            .get_item_attr_val_extra(ctx, item_key, &ac::attrs::SCAN_LADAR_STRENGTH)
            .unwrap();
        let str_magnet = calc
            .get_item_attr_val_extra(ctx, item_key, &ac::attrs::SCAN_MAGNETOMETRIC_STRENGTH)
            .unwrap();
        let str_grav = calc
            .get_item_attr_val_extra(ctx, item_key, &ac::attrs::SCAN_GRAVIMETRIC_STRENGTH)
            .unwrap();
        let mut sensor = Sensor {
            kind: SensorKind::Radar,
            strength: str_radar,
        };
        if str_ladar > sensor.strength {
            sensor.kind = SensorKind::Ladar;
            sensor.strength = str_ladar;
        }
        if str_magnet > sensor.strength {
            sensor.kind = SensorKind::Magnetometric;
            sensor.strength = str_magnet;
        }
        if str_grav > sensor.strength {
            sensor.kind = SensorKind::Gravimetric;
            sensor.strength = str_grav;
        }
        sensor
    }
    pub(in crate::svc) fn get_stat_item_probing_size(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        item_check_sensors(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_probing_size_unchecked(ctx, calc, item_key))
    }
    fn internal_get_stat_item_probing_size_unchecked(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> AttrVal {
        let sensor_str = Vast::internal_get_stat_item_sensor_unchecked(ctx, calc, item_key).strength;
        let sig_radius = Vast::internal_get_stat_item_sig_radius_unchecked(ctx, calc, item_key);
        (sig_radius / sensor_str).max(OF(1.08))
    }
}

#[derive(Copy, Clone)]
pub struct Sensor {
    pub kind: SensorKind,
    pub strength: AttrVal,
}

#[derive(Copy, Clone)]
pub enum SensorKind {
    Radar,
    Gravimetric,
    Magnetometric,
    Ladar,
}

fn item_check_sensors(ctx: SvcCtx, item_key: UItemKey) -> Result<(), StatItemCheckError> {
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

fn item_check_sensors_no_drones(ctx: SvcCtx, item_key: UItemKey) -> Result<(), StatItemCheckError> {
    let u_item = ctx.u_data.items.get(item_key);
    let is_loaded = match u_item {
        UItem::Fighter(u_fighter) => u_fighter.is_loaded(),
        UItem::Ship(u_ship) => u_ship.is_loaded(),
        _ => return Err(KeyedItemKindVsStatError { item_key }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(KeyedItemLoadedError { item_key }.into()),
    }
}
