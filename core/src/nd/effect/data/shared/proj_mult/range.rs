use crate::{
    def::{AttrVal, OF, SERVER_TICK_HZ},
    rd::{RAttrKey, REffect},
    svc::{SvcCtx, calc::Calc},
    ud::{UItemKey, UProjData},
    util::{FLOAT_TOLERANCE, ceil_tick, floor_tick},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Regular optimal/falloff range calculation
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) fn get_simple_c2s_range_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    proj_data: UProjData,
) -> AttrVal {
    let affector_optimal = get_effect_range(ctx, calc, projector_key, projector_effect.range_attr_key);
    match proj_data.get_range_c2s() <= affector_optimal {
        true => OF(1.0),
        false => OF(0.0),
    }
}

pub(super) fn get_simple_s2s_range_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    proj_data: UProjData,
) -> AttrVal {
    let affector_optimal = get_effect_range(ctx, calc, projector_key, projector_effect.range_attr_key);
    match proj_data.get_range_s2s() <= affector_optimal {
        true => OF(1.0),
        false => OF(0.0),
    }
}

pub(super) fn get_full_restricted_range_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    proj_data: UProjData,
) -> AttrVal {
    get_full_range_mult(
        ctx,
        calc,
        projector_key,
        projector_effect,
        proj_data.get_range_s2s(),
        true,
    )
}

pub(super) fn get_full_unrestricted_range_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    proj_data: UProjData,
) -> AttrVal {
    get_full_range_mult(
        ctx,
        calc,
        projector_key,
        projector_effect,
        proj_data.get_range_s2s(),
        false,
    )
}

fn get_full_range_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    proj_range: AttrVal,
    restricted: bool,
) -> AttrVal {
    let affector_optimal = get_effect_range(ctx, calc, projector_key, projector_effect.range_attr_key);
    let affector_falloff = get_effect_range(ctx, calc, projector_key, projector_effect.falloff_attr_key);
    // Calculate actual range multiplier after collecting all the data
    match affector_falloff > FLOAT_TOLERANCE {
        true => match restricted && proj_range > affector_optimal + OF(3.0) * affector_falloff {
            true => OF(0.0),
            false => ordered_float::Float::powf(
                OF(0.5),
                OF((ordered_float::Float::max(OF(0.0), proj_range - affector_optimal) / affector_falloff).powi(2)),
            ),
        },
        false => match proj_range <= affector_optimal {
            true => OF(1.0),
            false => OF(0.0),
        },
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Missile-alike effect range calculation
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::nd::effect::data) fn get_missile_range_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    _projector_effect: &REffect,
    _projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    let attr_consts = ctx.ac();
    let max_velocity = calc
        .get_item_oattr_ffb_extra(ctx, projector_key, attr_consts.max_velocity, OF(0.0))
        .max(OF(0.0));
    let flight_time = calc
        .get_item_oattr_ffb_extra(ctx, projector_key, attr_consts.explosion_delay, OF(0.0))
        .max(OF(0.0))
        / OF(1000.0);
    let mass = calc
        .get_item_oattr_ffb_extra(ctx, projector_key, attr_consts.mass, OF(0.0))
        .max(OF(0.0));
    let agility = calc
        .get_item_oattr_ffb_extra(ctx, projector_key, attr_consts.agility, OF(0.0))
        .max(OF(0.0));
    let flight_time_lower = floor_tick(flight_time);
    // Missiles appear in center of attacking ship and explode on surface of target ship
    let proj_range = proj_data.get_range_c2s();
    match flight_time_lower == flight_time {
        // When flight time is aligned to ticks, need to do fewer calculations
        true => {
            let flight_range = calc_flight_range(max_velocity, flight_time, mass, agility);
            match proj_range <= flight_range {
                true => OF(1.0),
                false => OF(0.0),
            }
        }
        // When flight time is not aligned to ticks, any range which lies within lower flight time
        // receives full effect, any range past higher flight time receives no effect, and anything
        // in-between receives partial effect corresponding to flight time fraction part
        false => {
            let flight_range_lower = calc_flight_range(max_velocity, flight_time_lower, mass, agility);
            match proj_range <= flight_range_lower {
                true => OF(1.0),
                false => {
                    let flight_time_higher = ceil_tick(flight_time);
                    let flight_range_higher = calc_flight_range(max_velocity, flight_time_higher, mass, agility);
                    match proj_range > flight_range_higher {
                        true => OF(0.0),
                        false => OF((flight_time * SERVER_TICK_HZ as f64).fract()),
                    }
                }
            }
        }
    }
}

pub(in crate::nd::effect::data) fn get_bomb_range_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    _projector_effect: &REffect,
    _projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    // Bomb is similar to missile, but they have fixed flight range and AoE effect
    let attr_consts = ctx.ac();
    let max_velocity = calc
        .get_item_oattr_ffb_extra(ctx, projector_key, attr_consts.max_velocity, OF(0.0))
        .max(OF(0.0));
    let flight_time = calc
        .get_item_oattr_ffb_extra(ctx, projector_key, attr_consts.explosion_delay, OF(0.0))
        .max(OF(0.0))
        / OF(1000.0);
    let mass = calc
        .get_item_oattr_ffb_extra(ctx, projector_key, attr_consts.mass, OF(0.0))
        .max(OF(0.0));
    let agility = calc
        .get_item_oattr_ffb_extra(ctx, projector_key, attr_consts.agility, OF(0.0))
        .max(OF(0.0));
    let aoe_range = calc
        .get_item_oattr_ffb_extra(ctx, projector_key, attr_consts.emp_field_range, OF(0.0))
        .max(OF(0.0));
    let flight_time_lower = floor_tick(flight_time);
    // Bombs appear in center of attacking ship
    let proj_range = proj_data.get_range_c2c();
    match flight_time_lower == flight_time {
        // When flight time is aligned to ticks, need to do fewer calculations
        true => {
            let flight_range = calc_flight_range(max_velocity, flight_time, mass, agility);
            let short_range = flight_range - aoe_range - proj_data.get_tgt_radius();
            let long_range = flight_range + aoe_range + proj_data.get_tgt_radius();
            match proj_range >= short_range && proj_range <= long_range {
                true => OF(1.0),
                false => OF(0.0),
            }
        }
        // When flight time is not aligned to ticks, calculate 2 outcomes separately, and sum their
        // results up
        false => {
            let flight_time_higher = ceil_tick(flight_time);
            let flight_range_lower = calc_flight_range(max_velocity, flight_time_lower, mass, agility);
            let flight_range_higher = calc_flight_range(max_velocity, flight_time_higher, mass, agility);
            let chance_higher = OF((flight_time * SERVER_TICK_HZ as f64).fract());
            let chance_lower = OF(1.0) - chance_higher;
            let lower_short_range = flight_range_lower - aoe_range - proj_data.get_tgt_radius();
            let lower_long_range = flight_range_lower + aoe_range + proj_data.get_tgt_radius();
            let higher_short_range = flight_range_higher - aoe_range - proj_data.get_tgt_radius();
            let higher_long_range = flight_range_higher + aoe_range + proj_data.get_tgt_radius();
            let mut mult = OF(0.0);
            if proj_range >= lower_short_range && proj_range <= lower_long_range {
                mult += chance_lower;
            };
            if proj_range >= higher_short_range && proj_range <= higher_long_range {
                mult += chance_higher;
            };
            mult
        }
    }
}

fn calc_flight_range(max_velocity: AttrVal, flight_time: AttrVal, mass: AttrVal, agility: AttrVal) -> AttrVal {
    // Source: http://www.eveonline.com/ingameboard.asp?a=topic&threadID=1307419&page=1#15
    // Link is broken for ages
    // D_m = V_m * (T_m + T_0*[exp(- T_m/T_0)-1])
    let accel_time = flight_time.min(mass * agility / OF(1000000.0));
    // Optimized expression of:
    // acceleration distance = max velocity * acceleration time / 2
    // full speed distance = max velocity * (flight time - acceleration time)
    // total distance = acceleration distance + full speed distance
    max_velocity * (flight_time - accel_time / OF(2.0))
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Misc effects
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) fn get_aoe_burst_range_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    proj_data: UProjData,
) -> AttrVal {
    // Doomsday projectiles are launched from center of the ship, and range is extended by aoe range
    let affector_optimal = get_effect_range(ctx, calc, projector_key, projector_effect.range_attr_key);
    let affector_aoe = get_effect_range(ctx, calc, projector_key, ctx.ac().doomsday_aoe_range);
    match proj_data.get_range_c2s() <= affector_optimal + affector_aoe {
        true => OF(1.0),
        false => OF(0.0),
    }
}

pub(super) fn get_aoe_dd_range_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    // AoE doomsdays' effects do not specify range attribute ID, so it is hardcoded here. Their
    // effect starts at the edge of attacker, and goes up to specified range
    let affector_optimal = get_effect_range(ctx, calc, projector_key, ctx.ac().max_range);
    if proj_data.get_range_s2s() > affector_optimal {
        return OF(0.0);
    }
    // Targets which are completely in attacker's model receive no damage
    match proj_data.get_range_c2c() + proj_data.get_tgt_radius() < proj_data.get_src_radius() {
        true => OF(0.0),
        false => OF(1.0),
    }
}

pub(super) fn get_dd_neut_range_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    let neut_optimal = get_effect_range(ctx, calc, projector_key, ctx.ac().doomsday_energy_neut_radius);
    // TODO: check if side-effect AoE neut range is s2s (could be c2s)
    match proj_data.get_range_s2s() <= neut_optimal {
        true => OF(1.0),
        false => OF(0.0),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Utility
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_effect_range(ctx: SvcCtx, calc: &mut Calc, projector_key: UItemKey, attr_key: Option<RAttrKey>) -> AttrVal {
    match attr_key {
        Some(attr_key) => match calc.get_item_attr_rfull(ctx, projector_key, attr_key) {
            Ok(val) => val.extra,
            _ => OF(0.0),
        },
        None => OF(0.0),
    }
}
