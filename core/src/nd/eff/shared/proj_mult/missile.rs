use crate::{
    ac,
    def::{AttrVal, OF, SERVER_TICK_HZ},
    rd::REffect,
    svc::{SvcCtx, calc::Calc},
    ud::{UItemKey, UProjData},
    util::{ceil_tick, floor_tick},
};

pub(in crate::nd::eff) fn get_proj_mult_missile(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    _r_effect: &REffect,
    u_proj_data: UProjData,
) -> AttrVal {
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
    let proj_range = u_proj_data.get_range_c2s();
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

// Bomb is similar to missile, but they have fixed flight range and AoE effect
pub(in crate::nd::eff) fn get_proj_mult_bomb(
    ctx: SvcCtx,
    calc: &mut Calc,
    affector_key: UItemKey,
    _r_effect: &REffect,
    u_proj_data: UProjData,
) -> AttrVal {
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
    let proj_range = u_proj_data.get_range_c2c();
    match flight_time_lower == flight_time {
        // When flight time is aligned to ticks, need to do fewer calculations
        true => {
            let flight_range = calc_range(max_velocity, flight_time, mass, agility);
            let short_range = flight_range - aoe_range - u_proj_data.get_tgt_rad();
            let long_range = flight_range + aoe_range + u_proj_data.get_tgt_rad();
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
            let lower_short_range = flight_range_lower - aoe_range - u_proj_data.get_tgt_rad();
            let lower_long_range = flight_range_lower + aoe_range + u_proj_data.get_tgt_rad();
            let higher_short_range = flight_range_higher - aoe_range - u_proj_data.get_tgt_rad();
            let higher_long_range = flight_range_higher + aoe_range + u_proj_data.get_tgt_rad();
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
