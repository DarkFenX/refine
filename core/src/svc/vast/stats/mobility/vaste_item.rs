use super::{option::StatJumpRange, stat::StatJump};
use crate::{
    ad::AItemId,
    api::ItemTypeId,
    num::{PValue, Value},
    svc::{
        SvcCtx,
        calc::Calc,
        err::StatItemCheckError,
        funcs,
        vast::{
            Vast,
            stats::item_checks::{
                check_drone_fighter_ship, check_drone_fighter_ship_no_struct, check_fighter_ship_no_struct, check_ship,
                check_ship_no_struct,
            },
        },
    },
    ud::{UFitId, UItemId},
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
        Ok(Self::internal_get_stat_item_agility_unchecked(ctx, calc, item_uid))
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
        let agility = Self::internal_get_stat_item_agility_unchecked(ctx, calc, item_uid);
        let align_time = agility.map(PValue::ceil_tick);
        Ok(align_time)
    }
    pub(in crate::svc) fn get_stat_item_sig_radius(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<PValue, StatItemCheckError> {
        check_drone_fighter_ship(ctx.u_data, item_uid)?;
        Ok(Self::internal_get_stat_item_sig_radius_unchecked(ctx, calc, item_uid))
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
        Ok(Self::internal_get_stat_item_mass_unchecked(ctx, calc, item_uid))
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
        let cap = Self::internal_get_stat_item_cap_amount_unchecked(ctx, calc, item_uid);
        let mass = Self::internal_get_stat_item_mass_unchecked(ctx, calc, item_uid);
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
    pub(in crate::svc) fn get_stat_item_jump(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        range: StatJumpRange,
        passenger_fit_uids: &[UFitId],
    ) -> Result<Option<StatJump>, StatItemCheckError> {
        let fit_uid = check_ship(ctx.u_data, item_uid)?.get_fit_uid();
        Ok(Vast::internal_get_stat_item_jump_unchecked(
            ctx,
            calc,
            fit_uid,
            item_uid,
            range,
            passenger_fit_uids,
        ))
    }
    fn internal_get_stat_item_jump_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uid: UFitId,
        item_uid: UItemId,
        range: StatJumpRange,
        passenger_fit_uids: &[UFitId],
    ) -> Option<StatJump> {
        let max_range = PValue::from_value_clamped(
            calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().jump_drive_range, Value::ZERO)
                .unwrap(),
        );
        if max_range < PValue::FLOAT_TOLERANCE {
            return None;
        }
        let fuel_type_id = ItemTypeId::from_aid(AItemId::try_from_f64_rounded(
            calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().jump_drive_consumption_type, Value::ZERO)
                .unwrap()
                .into_f64(),
        )?);
        Some(StatJump {
            max_range,
            fuel_type_id,
            jump_self: None,
            jump_conduit: None,
            jump_bridge: Vec::new(),
        })
    }
}
