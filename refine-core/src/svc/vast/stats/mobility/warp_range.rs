use crate::{
    PValue, Value,
    svc::{Calc, SvcCtx, Vast, err::IntStatItemError, vast::stats::item_checks::check_ship_no_struct},
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_max_warp_range(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, IntStatItemError<StatMaxWarpRangeError>> {
        check_ship_no_struct(ctx.u_data, item_uid)?;
        Self::internal_get_stat_item_max_warp_range_unchecked(ctx, calc, item_uid)
            .map_err(IntStatItemError::StatSpecific)
    }
    fn internal_get_stat_item_max_warp_range_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, StatMaxWarpRangeError> {
        let cap = Self::internal_get_stat_item_cap_amount_unchecked(ctx, calc, item_uid);
        let cap_need = PValue::from_value_clamped(calc.get_item_oattr_ffb_extra(
            ctx,
            item_uid,
            ctx.ac().warp_capacitor_need,
            Value::ZERO,
        ));
        let mass = Self::internal_get_stat_item_mass_unchecked(ctx, calc, item_uid);
        let warp_range = cap / cap_need / mass;
        if !warp_range.is_finite() {
            if cap_need == PValue::ZERO {
                return Err(StatMaxWarpRangeError::CapNeedError(cap_need));
            }
            return Err(StatMaxWarpRangeError::MassError(mass));
        }
        if warp_range < PValue::FLOAT_TOLERANCE {
            return Err(StatMaxWarpRangeError::CapAmountError(cap));
        }
        Ok(warp_range)
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum StatMaxWarpRangeError {
    #[error("capacitor capacity should be > 0, but is {0}")]
    CapAmountError(PValue),
    #[error("warp capacitor need should be > 0, but is {0}")]
    CapNeedError(PValue),
    #[error("mass should be > 0, but is {0}")]
    MassError(PValue),
}
