use super::{
    option::StatJumpRange,
    stat::{StatJump, StatJumpConduit, StatJumpPassenger, StatJumpSelf},
};
use crate::{
    ad::AAttrId,
    api::ItemTypeId,
    num::{Count, PValue, Value},
    rd::RAttrId,
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
    ud::{UFit, UFitId, UItemId, UShip, UShipKind},
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
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        range: StatJumpRange,
        passenger_fit_uids: &[UFitId],
    ) -> Result<Option<StatJump>, StatItemCheckError> {
        let ship = check_ship(ctx.u_data, item_uid)?;
        Ok(self.internal_get_stat_item_jump_unchecked(ctx, calc, item_uid, ship, range, passenger_fit_uids))
    }
    fn internal_get_stat_item_jump_unchecked(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        ship_uid: UItemId,
        ship: &UShip,
        range: StatJumpRange,
        passenger_fit_uids: &[UFitId],
    ) -> Option<StatJump> {
        let fuel_type_id = ItemTypeId::from_aid(ship.get_axt().unwrap().jump_fuel_type_id?);
        let max_range = PValue::from_value_clamped(calc.get_item_oattr_ffb_extra(
            ctx,
            ship_uid,
            ctx.ac().jump_drive_range,
            Value::ZERO,
        ));
        if max_range < PValue::FLOAT_TOLERANCE {
            return None;
        }
        let mut stat = StatJump {
            max_range,
            fuel_type_id,
            jump_self: None,
            jump_conduit: None,
            jump_bridges: Vec::new(),
        };
        let range = match range {
            // If requested range was higher than item allows, exclude all the jump stats besides
            // basic ones
            StatJumpRange::LightYears(range) => match range > max_range + PValue::FLOAT_TOLERANCE {
                true => return Some(stat),
                false => range,
            },
            StatJumpRange::Max => max_range,
        };
        // Make self-jump stat for all ships with jump drive, except of structure kind, not to
        // expose self-jump stats for ansiblexes
        if !matches!(ship.get_kind(), UShipKind::Structure) {
            let self_fuel_need =
                calc.get_item_oattr_ffb_extra(ctx, ship_uid, ctx.ac().jump_drive_consumption_amount, Value::ZERO);
            stat.jump_self = Some(StatJumpSelf {
                fuel_use: Count::from_value_ceiled(self_fuel_need * range),
            });
        }
        let fit_data = self.get_fit_data(ship.get_fit_uid());
        // Expose conduit stats only if fit has any conduit enablers (ship or online bridges)
        if !fit_data.conduit_enablers.is_empty() {
            let conduit_fuel_need = calc.get_item_oattr_ffb_extra(
                ctx,
                ship_uid,
                ctx.ac().conduit_jump_drive_consumption_amount,
                Value::ZERO,
            );
            let max_passengers = Count::from_value_rounded(calc.get_item_oattr_ffb_extra(
                ctx,
                ship_uid,
                ctx.ac().conduit_jump_passenger_count,
                Value::ZERO,
            ));
            let mut passengers = Vec::with_capacity(passenger_fit_uids.len());
            if !passenger_fit_uids.is_empty() {
                let pass_attr_rid =
                    get_pass_attr_rid(ctx, calc, ship_uid, ctx.ac().jump_conduit_passenger_required_attr_id);
                for &passenger_fit_uid in passenger_fit_uids {
                    let passenger_u_fit = ctx.u_data.fits.get(passenger_fit_uid);
                    let passenger = match is_passenger(ctx, calc, passenger_u_fit, pass_attr_rid) {
                        true => StatJumpPassenger {
                            fit_id: passenger_u_fit.id,
                            fuel_use: Some(Count::ZERO),
                        },
                        false => StatJumpPassenger {
                            fit_id: passenger_u_fit.id,
                            fuel_use: None,
                        },
                    };
                    passengers.push(passenger);
                }
            }
            stat.jump_conduit = Some(StatJumpConduit {
                max_passengers,
                fuel_use_self: Count::from_value_ceiled(conduit_fuel_need * range),
                fuel_use_passengers: passengers,
            })
        }
        Some(stat)
    }
}

fn get_pass_attr_rid(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    ref_attr_rid: Option<RAttrId>,
) -> Option<RAttrId> {
    let ref_val = calc.get_item_attr_oextra(ctx, item_uid, ref_attr_rid?)?;
    let pass_attr_aid = AAttrId::try_eve_from_f64_rounded(ref_val.into_f64())?;
    ctx.u_data.r_data.get_attr_rid_by_aid(&pass_attr_aid)
}

fn is_passenger(ctx: SvcCtx, calc: &mut Calc, pass_fit: &UFit, pass_attr_rid: Option<RAttrId>) -> bool {
    let Some(pass_ship_uid) = pass_fit.ship else {
        return false;
    };
    funcs::is_oattr_flag_set(ctx, calc, pass_ship_uid, pass_attr_rid).unwrap_or(false)
}
