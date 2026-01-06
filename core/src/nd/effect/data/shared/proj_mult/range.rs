use crate::{
    def::SERVER_TICK_HZ,
    misc::{PValue, Value},
    rd::{RAttrId, REffect},
    svc::{SvcCtx, calc::Calc},
    ud::{UItemId, UProjData},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Regular optimal/falloff range calculation
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) fn get_simple_c2s_range_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    proj_data: UProjData,
) -> PValue {
    let affector_optimal = get_effect_range(ctx, calc, projector_uid, effect.range_attr_rid);
    match proj_data.get_range_c2s() <= affector_optimal {
        true => PValue::ONE,
        false => PValue::ZERO,
    }
}

pub(super) fn get_simple_s2s_range_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    proj_data: UProjData,
) -> PValue {
    let affector_optimal = get_effect_range(ctx, calc, projector_uid, effect.range_attr_rid);
    match proj_data.get_range_s2s() <= affector_optimal {
        true => PValue::ONE,
        false => PValue::ZERO,
    }
}

pub(super) fn get_full_restricted_range_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    proj_data: UProjData,
) -> PValue {
    get_full_range_mult(ctx, calc, projector_uid, effect, proj_data.get_range_s2s(), true)
}

pub(super) fn get_full_unrestricted_range_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    proj_data: UProjData,
) -> PValue {
    get_full_range_mult(ctx, calc, projector_uid, effect, proj_data.get_range_s2s(), false)
}

fn get_full_range_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    proj_range: PValue,
    restricted: bool,
) -> PValue {
    let affector_optimal = get_effect_range(ctx, calc, projector_uid, effect.range_attr_rid);
    let affector_falloff = get_effect_range(ctx, calc, projector_uid, effect.falloff_attr_rid);
    // Calculate actual range multiplier after collecting all the data
    match affector_falloff > PValue::FLOAT_TOLERANCE {
        true => {
            match restricted && proj_range > affector_optimal + PValue::from_f64_unchecked(3.0) * affector_falloff {
                true => PValue::ZERO,
                false => PValue::pow_pvalue(
                    PValue::from_f64_unchecked(0.5),
                    (PValue::from_val_clamped(proj_range - affector_optimal) / affector_falloff).powi(2),
                ),
            }
        }
        false => match proj_range <= affector_optimal {
            true => PValue::ONE,
            false => PValue::ZERO,
        },
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Missile-alike effect range calculation
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::nd::effect::data) fn get_missile_range_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    _effect: &REffect,
    _projectee_uid: UItemId,
    proj_data: UProjData,
) -> PValue {
    let max_velocity =
        PValue::from_val_clamped(calc.get_item_oattr_ffb_extra(ctx, projector_uid, ctx.ac().max_velocity, Value::ZERO));
    let flight_time = PValue::from_val_clamped(
        calc.get_item_oattr_ffb_extra(ctx, projector_uid, ctx.ac().explosion_delay, Value::ZERO) / Value::THOUSAND,
    );
    let mass = PValue::from_val_clamped(calc.get_item_oattr_ffb_extra(ctx, projector_uid, ctx.ac().mass, Value::ZERO));
    let agility =
        PValue::from_val_clamped(calc.get_item_oattr_ffb_extra(ctx, projector_uid, ctx.ac().agility, Value::ZERO));
    let flight_time_lower = flight_time.floor_tick();
    // Missiles appear in center of attacking ship and explode on surface of target ship
    let proj_range = proj_data.get_range_c2s();
    match flight_time_lower == flight_time {
        // When flight time is aligned to ticks, need to do fewer calculations
        true => {
            let flight_range = calc_flight_range(max_velocity, flight_time, mass, agility);
            match proj_range <= flight_range {
                true => PValue::ONE,
                false => PValue::ZERO,
            }
        }
        // When flight time is not aligned to ticks, any range which lies within lower flight time
        // receives full effect, any range past higher flight time receives no effect, and anything
        // in-between receives partial effect corresponding to flight time fraction part
        false => {
            let flight_range_lower = calc_flight_range(max_velocity, flight_time_lower, mass, agility);
            match proj_range <= flight_range_lower {
                true => PValue::ONE,
                false => {
                    let flight_time_higher = flight_time.ceil_tick();
                    let flight_range_higher = calc_flight_range(max_velocity, flight_time_higher, mass, agility);
                    match proj_range > flight_range_higher {
                        true => PValue::ZERO,
                        false => PValue::from_f64_unchecked((flight_time.into_f64() * SERVER_TICK_HZ as f64).fract()),
                    }
                }
            }
        }
    }
}

pub(in crate::nd::effect::data) fn get_bomb_range_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    _effect: &REffect,
    _projectee_uid: UItemId,
    proj_data: UProjData,
) -> PValue {
    // Bomb is similar to missile, but they have fixed flight range and AoE effect
    let max_velocity =
        PValue::from_val_clamped(calc.get_item_oattr_ffb_extra(ctx, projector_uid, ctx.ac().max_velocity, Value::ZERO));
    let flight_time = PValue::from_val_clamped(
        calc.get_item_oattr_ffb_extra(ctx, projector_uid, ctx.ac().explosion_delay, Value::ZERO) / Value::THOUSAND,
    );
    let mass = PValue::from_val_clamped(calc.get_item_oattr_ffb_extra(ctx, projector_uid, ctx.ac().mass, Value::ZERO));
    let agility =
        PValue::from_val_clamped(calc.get_item_oattr_ffb_extra(ctx, projector_uid, ctx.ac().agility, Value::ZERO));
    let aoe_range = PValue::from_val_clamped(calc.get_item_oattr_ffb_extra(
        ctx,
        projector_uid,
        ctx.ac().emp_field_range,
        Value::ZERO,
    ));
    let flight_time_lower = flight_time.floor_tick();
    // Bombs appear in center of attacking ship
    let proj_range = proj_data.get_range_c2c();
    match flight_time_lower == flight_time {
        // When flight time is aligned to ticks, need to do fewer calculations
        true => {
            let flight_range = calc_flight_range(max_velocity, flight_time, mass, agility);
            let short_range = PValue::from_val_clamped(flight_range - aoe_range - proj_data.get_tgt_radius());
            let long_range = flight_range + aoe_range + proj_data.get_tgt_radius();
            match proj_range >= short_range && proj_range <= long_range {
                true => PValue::ONE,
                false => PValue::ZERO,
            }
        }
        // When flight time is not aligned to ticks, calculate 2 outcomes separately, and sum their
        // results up
        false => {
            let flight_time_higher = flight_time.ceil_tick();
            let flight_range_lower = calc_flight_range(max_velocity, flight_time_lower, mass, agility);
            let flight_range_higher = calc_flight_range(max_velocity, flight_time_higher, mass, agility);
            let chance_higher = (flight_time.into_f64() * SERVER_TICK_HZ as f64).fract();
            let chance_lower = 1.0 - chance_higher;
            let lower_short_range =
                PValue::from_val_clamped(flight_range_lower - aoe_range - proj_data.get_tgt_radius());
            let lower_long_range = flight_range_lower + aoe_range + proj_data.get_tgt_radius();
            let higher_short_range =
                PValue::from_val_clamped(flight_range_higher - aoe_range - proj_data.get_tgt_radius());
            let higher_long_range = flight_range_higher + aoe_range + proj_data.get_tgt_radius();
            let mut mult = 0.0;
            if proj_range >= lower_short_range && proj_range <= lower_long_range {
                mult += chance_lower;
            };
            if proj_range >= higher_short_range && proj_range <= higher_long_range {
                mult += chance_higher;
            };
            PValue::from_f64_unchecked(mult)
        }
    }
}

fn calc_flight_range(max_velocity: PValue, flight_time: PValue, mass: PValue, agility: PValue) -> PValue {
    // Missiles use the regular object acceleration formula:
    // https://wiki.eveuniversity.org/Acceleration#Mathematics_and_formulae
    // Here, integral of this formula is calculated
    let inertia_factor = 1000000.0 / (mass.into_f64() * agility.into_f64());
    let flight_time = flight_time.into_f64();
    // With all the non-negative inputs, non-negative result is guaranteed - third part of formula
    // should always be equal to or smaller than (flight time - 1) even with the float inaccuracy
    // involved
    let range_units = flight_time - 1.0 + f64::exp(-inertia_factor * flight_time) / inertia_factor;
    PValue::from_f64_unchecked(max_velocity.into_f64() * range_units)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Misc effects
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) fn get_aoe_burst_range_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    proj_data: UProjData,
) -> PValue {
    // Doomsday projectiles are launched from center of the ship, and range is extended by aoe range
    let affector_optimal = get_effect_range(ctx, calc, projector_uid, effect.range_attr_rid);
    let affector_aoe = get_effect_range(ctx, calc, projector_uid, ctx.ac().doomsday_aoe_range);
    match proj_data.get_range_c2s() <= affector_optimal + affector_aoe {
        true => PValue::ONE,
        false => PValue::ZERO,
    }
}

pub(super) fn get_aoe_dd_range_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    proj_data: UProjData,
) -> PValue {
    // AoE doomsdays' effects do not specify range attribute ID, so it is hardcoded here. Their
    // effect starts at the edge of attacker, and goes up to specified range
    let affector_optimal = get_effect_range(ctx, calc, projector_uid, ctx.ac().max_range);
    if proj_data.get_range_s2s() > affector_optimal {
        return PValue::ZERO;
    }
    // Targets which are completely in attacker's model receive no damage
    match proj_data.get_range_c2c() + proj_data.get_tgt_radius() < proj_data.get_src_radius() {
        true => PValue::ZERO,
        false => PValue::ONE,
    }
}

pub(super) fn get_dd_neut_range_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    proj_data: UProjData,
) -> PValue {
    let neut_optimal = get_effect_range(ctx, calc, projector_uid, ctx.ac().doomsday_energy_neut_radius);
    match proj_data.get_range_s2s() <= neut_optimal {
        true => PValue::ONE,
        false => PValue::ZERO,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Utility
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_effect_range(ctx: SvcCtx, calc: &mut Calc, projector_uid: UItemId, attr_rid: Option<RAttrId>) -> PValue {
    match attr_rid {
        Some(attr_rid) => match calc.get_item_attr_rfull(ctx, projector_uid, attr_rid) {
            Ok(val) => PValue::from_val_clamped(val.extra),
            _ => PValue::ZERO,
        },
        None => PValue::ZERO,
    }
}
