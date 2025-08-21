use crate::{
    ac,
    def::{AttrVal, OF, SERVER_TICK_HZ},
    rd::REffect,
    svc::{SvcCtx, calc::Calc},
    ud::{UItemKey, UProjData},
    util::{ceil_tick, floor_tick},
};

pub(in crate::nd::eff) fn get_missile_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    _projector_effect: &REffect,
    projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    get_missile_range_mult(ctx, calc, projector_key, proj_data)
        * get_missile_application_mult(ctx, calc, projector_key, projectee_key, proj_data)
}

fn get_missile_range_mult(ctx: SvcCtx, calc: &mut Calc, projector_key: UItemKey, proj_data: UProjData) -> AttrVal {
    let max_velocity = calc
        .get_item_attr_val_full(ctx, projector_key, &ac::attrs::MAX_VELOCITY)
        .unwrap()
        .extra
        .max(OF(0.0));
    let flight_time = calc
        .get_item_attr_val_full(ctx, projector_key, &ac::attrs::EXPLOSION_DELAY)
        .unwrap()
        .extra
        .max(OF(0.0))
        / OF(1000.0);
    let mass = calc
        .get_item_attr_val_full(ctx, projector_key, &ac::attrs::MASS)
        .unwrap()
        .extra
        .max(OF(0.0));
    let agility = calc
        .get_item_attr_val_full(ctx, projector_key, &ac::attrs::AGILITY)
        .unwrap()
        .extra
        .max(OF(0.0));
    let flight_time_lower = floor_tick(flight_time);
    // Missiles appear in center of attacking ship and explode on surface of target ship
    let proj_range = proj_data.get_range_c2s();
    match flight_time_lower == flight_time {
        // When flight time is aligned to ticks, need to do fewer calculations
        true => {
            let flight_range = calc_range(max_velocity, flight_time, mass, agility);
            match proj_range <= flight_range {
                true => OF(1.0),
                false => OF(0.0),
            }
        }
        // When flight time is not aligned to ticks, any range which lies within lower flight time
        // receives full effect, any range past higher flight time receives no effect, and anything
        // in-between receives partial effect corresponding to flight time fraction part
        false => {
            let flight_range_lower = calc_range(max_velocity, flight_time_lower, mass, agility);
            match proj_range <= flight_range_lower {
                true => OF(1.0),
                false => {
                    let flight_time_higher = ceil_tick(flight_time);
                    let flight_range_higher = calc_range(max_velocity, flight_time_higher, mass, agility);
                    match proj_range > flight_range_higher {
                        true => OF(0.0),
                        false => OF((flight_time * SERVER_TICK_HZ as f64).fract()),
                    }
                }
            }
        }
    }
}

fn get_missile_application_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    let src_er = calc
        .get_item_attr_val_full(ctx, projector_key, &ac::attrs::AOE_CLOUD_SIZE)
        .unwrap()
        .extra
        .max(OF(0.0));
    let src_ev = calc
        .get_item_attr_val_full(ctx, projector_key, &ac::attrs::AOE_VELOCITY)
        .unwrap()
        .extra
        .max(OF(0.0));
    let src_drf = calc
        .get_item_attr_val_full(ctx, projector_key, &ac::attrs::AOE_DAMAGE_REDUCTION_FACTOR)
        .unwrap()
        .extra
        .max(OF(0.0));
    let tgt_sig_radius = calc
        .get_item_attr_val_full(ctx, projectee_key, &ac::attrs::MAX_VELOCITY)
        .unwrap()
        .extra
        .max(OF(0.0));
    let tgt_speed = proj_data.get_tgt_speed()
        * calc
            .get_item_attr_val_full(ctx, projectee_key, &ac::attrs::SIG_RADIUS)
            .unwrap()
            .extra
            .max(OF(0.0));
    // "Static" part
    let radius_ratio = tgt_sig_radius / src_er;
    // "Mobile" part
    let mobile_mult = OF((radius_ratio * src_ev / tgt_speed).powf(src_drf.into_inner()));
    let mult = radius_ratio.min(mobile_mult);
    match mult.is_nan() {
        true => OF(1.0),
        // Value also cannot get higher than 1
        false => mult.clamp(OF(0.0), OF(1.0)),
    }
}

pub(in crate::nd::eff) fn get_bomb_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    _projector_effect: &REffect,
    _projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    get_bomb_proj_range_mult(ctx, calc, projector_key, proj_data)
}

// Bomb is similar to missile, but they have fixed flight range and AoE effect
fn get_bomb_proj_range_mult(ctx: SvcCtx, calc: &mut Calc, affector_key: UItemKey, proj_data: UProjData) -> AttrVal {
    let max_velocity = calc
        .get_item_attr_val_full(ctx, affector_key, &ac::attrs::MAX_VELOCITY)
        .unwrap()
        .extra
        .max(OF(0.0));
    let flight_time = calc
        .get_item_attr_val_full(ctx, affector_key, &ac::attrs::EXPLOSION_DELAY)
        .unwrap()
        .extra
        .max(OF(0.0))
        / OF(1000.0);
    let mass = calc
        .get_item_attr_val_full(ctx, affector_key, &ac::attrs::MASS)
        .unwrap()
        .extra
        .max(OF(0.0));
    let agility = calc
        .get_item_attr_val_full(ctx, affector_key, &ac::attrs::AGILITY)
        .unwrap()
        .extra
        .max(OF(0.0));
    let aoe_range = calc
        .get_item_attr_val_full(ctx, affector_key, &ac::attrs::EMP_FIELD_RANGE)
        .unwrap()
        .extra
        .max(OF(0.0));
    let flight_time_lower = floor_tick(flight_time);
    // Bombs appear in center of attacking ship
    let proj_range = proj_data.get_range_c2c();
    match flight_time_lower == flight_time {
        // When flight time is aligned to ticks, need to do fewer calculations
        true => {
            let flight_range = calc_range(max_velocity, flight_time, mass, agility);
            let short_range = flight_range - aoe_range - proj_data.get_tgt_rad();
            let long_range = flight_range + aoe_range + proj_data.get_tgt_rad();
            match proj_range >= short_range && proj_range <= long_range {
                true => OF(1.0),
                false => OF(0.0),
            }
        }
        // When flight time is not aligned to ticks, calculate 2 outcomes separately, and sum their
        // results up
        false => {
            let flight_time_higher = ceil_tick(flight_time);
            let flight_range_lower = calc_range(max_velocity, flight_time_lower, mass, agility);
            let flight_range_higher = calc_range(max_velocity, flight_time_higher, mass, agility);
            let chance_higher = OF((flight_time * SERVER_TICK_HZ as f64).fract());
            let chance_lower = OF(1.0) - chance_higher;
            let lower_short_range = flight_range_lower - aoe_range - proj_data.get_tgt_rad();
            let lower_long_range = flight_range_lower + aoe_range + proj_data.get_tgt_rad();
            let higher_short_range = flight_range_higher - aoe_range - proj_data.get_tgt_rad();
            let higher_long_range = flight_range_higher + aoe_range + proj_data.get_tgt_rad();
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

fn calc_range(max_velocity: AttrVal, flight_time: AttrVal, mass: AttrVal, agility: AttrVal) -> AttrVal {
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
