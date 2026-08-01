use crate::{
    PValue, Value,
    svc::{Calc, SvcCtx, Vast, err::IntStatItemError, vast::stats::item_checks::check_drone_fighter_ship},
    ud::UItemId,
};

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct StatSensors {
    pub kind: StatSensorsKind,
    pub strength: PValue,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone)]
pub enum StatSensorsKind {
    Radar,
    Magnetometric,
    Gravimetric,
    Ladar,
}

impl Vast {
    pub(in crate::svc) fn get_stat_item_sensors(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<StatSensors, IntStatItemError<!>> {
        check_drone_fighter_ship(ctx.u_data, item_uid)?;
        Ok(Self::internal_get_stat_item_sensors_unchecked(ctx, calc, item_uid))
    }
    pub(in crate::svc::vast::stats) fn internal_get_stat_item_sensors_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> StatSensors {
        // Strength ties are resolved using the following order:
        // Radar > ladar > magnetometric > gravimetric
        let str_radar = PValue::from_value_clamped(calc.get_item_oattr_ffb_extra(
            ctx,
            item_uid,
            ctx.ac().scan_radar_strength,
            Value::ZERO,
        ));
        let str_ladar = PValue::from_value_clamped(calc.get_item_oattr_ffb_extra(
            ctx,
            item_uid,
            ctx.ac().scan_ladar_strength,
            Value::ZERO,
        ));
        let str_magnet = PValue::from_value_clamped(calc.get_item_oattr_ffb_extra(
            ctx,
            item_uid,
            ctx.ac().scan_magnetometric_strength,
            Value::ZERO,
        ));
        let str_grav = PValue::from_value_clamped(calc.get_item_oattr_ffb_extra(
            ctx,
            item_uid,
            ctx.ac().scan_gravimetric_strength,
            Value::ZERO,
        ));
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
}
