use crate::{
    PValue,
    svc::{Calc, SvcCtx, Vast, err::IntStatItemError, vast::stats::item_checks::check_drone_fighter_ship},
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_probing_size(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, IntStatItemError<StatProbingSizeError>> {
        check_drone_fighter_ship(ctx.u_data, item_uid)?;
        Self::internal_get_stat_item_probing_size_unchecked(ctx, calc, item_uid).map_err(IntStatItemError::StatSpecific)
    }
    fn internal_get_stat_item_probing_size_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, StatProbingSizeError> {
        let sensor_str = Self::internal_get_stat_item_sensors_unchecked(ctx, calc, item_uid).strength;
        let sig_radius = Self::internal_get_stat_item_sig_radius_unchecked(ctx, calc, item_uid);
        let ratio = sig_radius / sensor_str;
        let probing_size = match ratio.is_finite() {
            true => ratio.max(PValue::from_f64_unchecked(1.08)),
            false => return Err(StatProbingSizeError::SensorStrError(sensor_str)),
        };
        Ok(probing_size)
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum StatProbingSizeError {
    #[error("sensor strength should be > 0, but is {0}")]
    SensorStrError(PValue),
}
