use crate::{
    num::{PValue, Value},
    svc::{
        SvcCtx,
        calc::Calc,
        err::StatItemCheckError,
        funcs,
        vast::{
            Vast,
            stats::item_checks::{
                check_drone_fighter_ship, check_drone_fighter_ship_no_struct, check_fighter_ship_no_struct,
                check_ship_no_struct,
            },
        },
    },
    ud::UItemId,
};

// Result of calculation of -math.log(0.25) / 1000000 using 64-bit python 2.7
pub(super) const AGILITY_CONST: PValue = PValue::from_f64_clamped(f64::from_bits(0x3eb74216c502a54f));

impl Vast {
    pub(in crate::svc) fn get_stat_item_speed(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, StatItemCheckError> {
        check_drone_fighter_ship_no_struct(ctx.u_data, item_uid)?;
        Ok(funcs::get_speed(ctx, calc, item_uid))
    }
    pub(in crate::svc) fn get_stat_item_agility(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<Option<PValue>, StatItemCheckError> {
        check_drone_fighter_ship_no_struct(ctx.u_data, item_uid)?;
        Ok(Vast::internal_get_stat_item_agility_unchecked(ctx, calc, item_uid))
    }
    fn internal_get_stat_item_agility_unchecked(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<PValue> {
        let attr_consts = ctx.ac();
        let agility = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, attr_consts.agility, Value::ZERO)
            .unwrap();
        let agility = match agility > Value::ZERO {
            true => PValue::from_value_unchecked(agility),
            false => return None,
        };
        let mass = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, attr_consts.mass, Value::ZERO)
            .unwrap();
        let mass = match mass > Value::ZERO {
            true => PValue::from_value_unchecked(mass),
            false => return None,
        };
        Some(AGILITY_CONST * agility * mass)
    }
    pub(in crate::svc) fn get_stat_item_align_time(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<Option<PValue>, StatItemCheckError> {
        check_drone_fighter_ship_no_struct(ctx.u_data, item_uid)?;
        let agility = Vast::internal_get_stat_item_agility_unchecked(ctx, calc, item_uid);
        let align_time = agility.map(PValue::ceil_tick);
        Ok(align_time)
    }
    pub(in crate::svc) fn get_stat_item_sig_radius(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, StatItemCheckError> {
        check_drone_fighter_ship(ctx.u_data, item_uid)?;
        Ok(Vast::internal_get_stat_item_sig_radius_unchecked(ctx, calc, item_uid))
    }
    pub(in crate::svc::vast::stats) fn internal_get_stat_item_sig_radius_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> PValue {
        funcs::get_sig_radius(ctx, calc, item_uid)
    }
    pub(in crate::svc) fn get_stat_item_mass(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, StatItemCheckError> {
        check_drone_fighter_ship(ctx.u_data, item_uid)?;
        Ok(Vast::internal_get_stat_item_mass_unchecked(ctx, calc, item_uid))
    }
    fn internal_get_stat_item_mass_unchecked(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> PValue {
        let mass = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().mass, Value::ZERO)
            .unwrap();
        PValue::from_value_clamped(mass)
    }
    pub(in crate::svc) fn get_stat_item_warp_speed(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<Option<PValue>, StatItemCheckError> {
        check_fighter_ship_no_struct(ctx.u_data, item_uid)?;
        let warp_speed = calc
            .get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().warp_speed_mult, Value::ZERO)
            .unwrap();
        let warp_speed = match warp_speed > Value::FLOAT_TOLERANCE {
            true => Some(PValue::from_value_unchecked(warp_speed)),
            false => None,
        };
        Ok(warp_speed)
    }
    pub(in crate::svc) fn get_stat_item_max_warp_range(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<Option<PValue>, StatItemCheckError> {
        check_ship_no_struct(ctx.u_data, item_uid)?;
        let cap = Vast::internal_get_stat_item_cap_amount_unchecked(ctx, calc, item_uid);
        let mass = Vast::internal_get_stat_item_mass_unchecked(ctx, calc, item_uid);
        let cap_need = PValue::from_value_clamped(
            calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().warp_capacitor_need, Value::ZERO)
                .unwrap(),
        );
        let warp_range = cap / mass / cap_need;
        let warp_range = match warp_range.is_finite() && warp_range > PValue::FLOAT_TOLERANCE {
            true => Some(warp_range),
            false => None,
        };
        Ok(warp_range)
    }
}
