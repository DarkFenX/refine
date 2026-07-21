use crate::{
    PValue, Value,
    stats::err::StatError,
    svc::{Calc, SvcCtx, Vast, err::IntItemStatError, vast::stats::item_checks::check_drone_fighter_ship_no_struct},
    ud::UItemId,
};

// Result of calculation of -math.log(0.25) / 1000000 using 64-bit python 2.7
const AGILITY_CONST: PValue = PValue::from_f64_clamped(f64::from_bits(0x3eb74216c502a54f));

impl Vast {
    pub(in crate::svc) fn get_stat_item_agility(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, IntItemStatError<AgilityStatError>> {
        check_drone_fighter_ship_no_struct(ctx.u_data, item_uid)?;
        Self::internal_get_stat_item_agility_unchecked(ctx, calc, item_uid).map_err(IntItemStatError::StatSpecific)
    }
    fn internal_get_stat_item_agility_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, AgilityStatError> {
        let attr_consts = ctx.ac();
        let agility = calc.get_item_oattr_ffb_extra(ctx, item_uid, attr_consts.agility, Value::ZERO);
        let agility = match agility > Value::ZERO {
            true => PValue::from_value_unchecked(agility),
            false => return Err(AgilityStatError::AgilityError(agility)),
        };
        let mass = calc.get_item_oattr_ffb_extra(ctx, item_uid, attr_consts.mass, Value::ZERO);
        let mass = match mass > Value::ZERO {
            true => PValue::from_value_unchecked(mass),
            false => return Err(AgilityStatError::MassError(mass)),
        };
        Ok(AGILITY_CONST * agility * mass)
    }
    pub(in crate::svc) fn get_stat_item_align_time(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, IntItemStatError<AgilityStatError>> {
        check_drone_fighter_ship_no_struct(ctx.u_data, item_uid)?;
        Self::internal_get_stat_item_agility_unchecked(ctx, calc, item_uid)
            .map(PValue::ceil_tick)
            .map_err(IntItemStatError::StatSpecific)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AgilityStatError {
    #[error("agility {0} should be > 0")]
    AgilityError(Value),
    #[error("mass {0} should be > 0")]
    MassError(Value),
}
impl StatError for AgilityStatError {
    fn is_fatal(&self) -> bool {
        true
    }
}
