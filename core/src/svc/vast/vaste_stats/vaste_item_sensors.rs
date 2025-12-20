use super::item_checks::{check_drone_fighter_ship, check_fighter_ship, check_ship};
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
        check_drone_fighter_ship(ctx.u_data, item_key)?;
        let attr_consts = ctx.ac();
        let mut item_locks = calc
            .get_item_oattr_afb_oextra(ctx, item_key, attr_consts.max_locked_targets, OF(0.0))
            .unwrap();
        // Ship (ship kind) locks are limited by character locks. Anything else, including
        // structures, drones and fighter are not limited by it
        let u_item = ctx.u_data.items.get(item_key);
        if let UItem::Ship(u_ship) = u_item
            && let UShipKind::Ship = u_ship.get_kind()
        {
            let u_fit = ctx.u_data.fits.get(u_ship.get_fit_key());
            if let Some(character_key) = u_fit.character
                && let Some(character_locks) =
                    calc.get_item_oattr_afb_oextra(ctx, character_key, attr_consts.max_locked_targets, OF(0.0))
            {
                item_locks = item_locks.min(character_locks)
            }
        }
        // Non-integer locks can happen in Pochven where locks are halved, halves are rounded up
        let locks = round_unerr(item_locks).into_inner() as Count;
        Ok(locks)
    }
    pub(in crate::svc) fn get_stat_item_lock_range(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        check_fighter_ship(ctx.u_data, item_key)?;
        let lock_range = calc
            .get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().max_target_range, OF(0.0))
            .unwrap();
        Ok(lock_range)
    }
    pub(in crate::svc) fn get_stat_item_scan_res(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<AttrVal, StatItemCheckError> {
        check_fighter_ship(ctx.u_data, item_key)?;
        let scan_res = calc
            .get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().scan_resolution, OF(0.0))
            .unwrap();
        Ok(scan_res)
    }
    pub(in crate::svc) fn get_stat_item_sensors(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<StatSensors, StatItemCheckError> {
        check_drone_fighter_ship(ctx.u_data, item_key)?;
        Ok(Vast::internal_get_stat_item_sensors_unchecked(ctx, calc, item_key))
    }
    pub(super) fn internal_get_stat_item_sensors_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> StatSensors {
        let attr_consts = ctx.ac();
        // Strength ties are resolved using the following order:
        // Radar > ladar > magnetometric > gravimetric
        let str_radar = calc
            .get_item_oattr_afb_oextra(ctx, item_key, attr_consts.scan_radar_strength, OF(0.0))
            .unwrap();
        let str_ladar = calc
            .get_item_oattr_afb_oextra(ctx, item_key, attr_consts.scan_ladar_strength, OF(0.0))
            .unwrap();
        let str_magnet = calc
            .get_item_oattr_afb_oextra(ctx, item_key, attr_consts.scan_magnetometric_strength, OF(0.0))
            .unwrap();
        let str_grav = calc
            .get_item_oattr_afb_oextra(ctx, item_key, attr_consts.scan_gravimetric_strength, OF(0.0))
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
        check_ship(ctx.u_data, item_key)?;
        let dscan_range = calc
            .get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().max_directional_scan_range, OF(0.0))
            .unwrap()
            / ac::extras::AU;
        Ok(dscan_range)
    }
    pub(in crate::svc) fn get_stat_item_probing_size(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<Option<AttrVal>, StatItemCheckError> {
        check_drone_fighter_ship(ctx.u_data, item_key)?;
        let sensor_str = Vast::internal_get_stat_item_sensors_unchecked(ctx, calc, item_key).strength;
        let sig_radius = Vast::internal_get_stat_item_sig_radius_unchecked(ctx, calc, item_key);
        let ratio = sig_radius / sensor_str;
        let probing_size = match ratio.is_finite() {
            true => Some(ratio.max(OF(1.08))),
            false => None,
        };
        Ok(probing_size)
    }
}
