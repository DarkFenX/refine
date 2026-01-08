use super::item_checks::{check_drone_fighter_ship, check_fighter_ship, check_ship};
use crate::{
    misc::{Count, PValue, Value},
    svc::{
        SvcCtx,
        calc::Calc,
        err::StatItemCheckError,
        vast::{StatSensors, StatSensorsKind, Vast},
    },
    ud::{UItem, UItemId, UShipKind},
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_locks(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<Count, StatItemCheckError> {
        check_drone_fighter_ship(ctx.u_data, item_uid)?;
        let attr_consts = ctx.ac();
        let mut item_locks = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, attr_consts.max_locked_targets, Value::ZERO)
            .unwrap();
        // Ship (ship kind) locks are limited by character locks. Anything else, including
        // structures, drones and fighter are not limited by it
        let u_item = ctx.u_data.items.get(item_uid);
        if let UItem::Ship(u_ship) = u_item
            && let UShipKind::Ship = u_ship.get_kind()
        {
            let u_fit = ctx.u_data.fits.get(u_ship.get_fit_uid());
            if let Some(character_uid) = u_fit.character
                && let Some(character_locks) =
                    calc.get_item_oattr_afb_oextra(ctx, character_uid, attr_consts.max_locked_targets, Value::ZERO)
            {
                item_locks = item_locks.min(character_locks)
            }
        }
        // Noninteger locks can happen in Pochven where locks are halved, halves are rounded up
        Ok(Count::from_value_rounded(item_locks))
    }
    pub(in crate::svc) fn get_stat_item_lock_range(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, StatItemCheckError> {
        check_fighter_ship(ctx.u_data, item_uid)?;
        let lock_range = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().max_target_range, Value::ZERO)
            .unwrap();
        Ok(PValue::from_value_clamped(lock_range))
    }
    pub(in crate::svc) fn get_stat_item_scan_res(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, StatItemCheckError> {
        check_fighter_ship(ctx.u_data, item_uid)?;
        let scan_res = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().scan_resolution, Value::ZERO)
            .unwrap();
        Ok(PValue::from_value_clamped(scan_res))
    }
    pub(in crate::svc) fn get_stat_item_sensors(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<StatSensors, StatItemCheckError> {
        check_drone_fighter_ship(ctx.u_data, item_uid)?;
        Ok(Vast::internal_get_stat_item_sensors_unchecked(ctx, calc, item_uid))
    }
    pub(super) fn internal_get_stat_item_sensors_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> StatSensors {
        // Strength ties are resolved using the following order:
        // Radar > ladar > magnetometric > gravimetric
        let str_radar = PValue::from_value_clamped(
            calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().scan_radar_strength, Value::ZERO)
                .unwrap(),
        );
        let str_ladar = PValue::from_value_clamped(
            calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().scan_ladar_strength, Value::ZERO)
                .unwrap(),
        );
        let str_magnet = PValue::from_value_clamped(
            calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().scan_magnetometric_strength, Value::ZERO)
                .unwrap(),
        );
        let str_grav = PValue::from_value_clamped(
            calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().scan_gravimetric_strength, Value::ZERO)
                .unwrap(),
        );
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
        item_uid: UItemId,
    ) -> Result<PValue, StatItemCheckError> {
        check_ship(ctx.u_data, item_uid)?;
        let dscan_range = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().max_directional_scan_range, Value::ZERO)
            .unwrap()
            / Value::AU;
        Ok(PValue::from_value_clamped(dscan_range))
    }
    pub(in crate::svc) fn get_stat_item_probing_size(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<Option<PValue>, StatItemCheckError> {
        check_drone_fighter_ship(ctx.u_data, item_uid)?;
        let sensor_str = Vast::internal_get_stat_item_sensors_unchecked(ctx, calc, item_uid).strength;
        let sig_radius = Vast::internal_get_stat_item_sig_radius_unchecked(ctx, calc, item_uid);
        let ratio = sig_radius / sensor_str;
        let probing_size = match ratio.is_finite() {
            true => Some(ratio.max(PValue::from_f64_unchecked(1.08))),
            false => None,
        };
        Ok(probing_size)
    }
}
