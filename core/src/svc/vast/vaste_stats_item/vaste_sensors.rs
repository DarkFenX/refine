use super::checks::{check_item_key_drone_fighter_ship, check_item_key_fighter_ship, check_item_key_ship};
use crate::{
    ac,
    def::{AttrVal, Count, OF},
    svc::{
        SvcCtx,
        calc::Calc,
        err::StatItemCheckError,
        vast::{StatSensors, StatSensorsKind, Vast},
    },
    ud::{UItem, UItemKey, UShipKind},
    util::round_unerr,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_locks(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<Count, StatItemCheckError> {
        check_item_key_drone_fighter_ship(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_locks(ctx, calc, item_key))
    }
    fn internal_get_stat_item_locks(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> Count {
        let mut item_locks = calc
            .get_item_attr_val_extra(ctx, item_key, &ac::attrs::MAX_LOCKED_TARGETS)
            .unwrap();
        // Ship (ship kind) locks are limited by character locks. Anything else, including
        // structures, drones and fighter are not limited by it
        let u_item = ctx.u_data.items.get(item_key);
        if let UItem::Ship(u_ship) = u_item
            && let UShipKind::Ship = u_ship.get_kind()
        {
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
        check_item_key_fighter_ship(ctx, item_key)?;
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
        check_item_key_fighter_ship(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_scan_res_unchecked(ctx, calc, item_key))
    }
    fn internal_get_stat_item_scan_res_unchecked(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> AttrVal {
        calc.get_item_attr_val_extra(ctx, item_key, &ac::attrs::SCAN_RESOLUTION)
            .unwrap()
    }
    pub(in crate::svc) fn get_stat_item_sensors(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<StatSensors, StatItemCheckError> {
        check_item_key_drone_fighter_ship(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_sensors_unchecked(ctx, calc, item_key))
    }
    pub(super) fn internal_get_stat_item_sensors_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> StatSensors {
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
        let mut sensors = StatSensors {
            kind: StatSensorsKind::Radar,
            strength: str_radar,
        };
        if str_ladar > sensors.strength {
            sensors.kind = StatSensorsKind::Ladar;
            sensors.strength = str_ladar;
        }
        if str_magnet > sensors.strength {
            sensors.kind = StatSensorsKind::Magnetometric;
            sensors.strength = str_magnet;
        }
        if str_grav > sensors.strength {
            sensors.kind = StatSensorsKind::Gravimetric;
            sensors.strength = str_grav;
        }
        sensors
    }
    pub(in crate::svc) fn get_stat_item_dscan_range(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        check_item_key_ship(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_dscan_range_unchecked(ctx, calc, item_key))
    }
    fn internal_get_stat_item_dscan_range_unchecked(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> AttrVal {
        calc.get_item_attr_val_extra(ctx, item_key, &ac::attrs::MAX_DIRECTIONAL_SCAN_RANGE)
            .unwrap()
            / ac::extras::AU
    }
    pub(in crate::svc) fn get_stat_item_probing_size(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<Option<AttrVal>, StatItemCheckError> {
        check_item_key_drone_fighter_ship(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_probing_size_unchecked(ctx, calc, item_key))
    }
    fn internal_get_stat_item_probing_size_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Option<AttrVal> {
        let sensor_str = Vast::internal_get_stat_item_sensors_unchecked(ctx, calc, item_key).strength;
        let sig_radius = Vast::internal_get_stat_item_sig_radius_unchecked(ctx, calc, item_key);
        let ratio = sig_radius / sensor_str;
        match ratio.is_finite() {
            true => Some(ratio.max(OF(1.08))),
            false => None,
        }
    }
}
