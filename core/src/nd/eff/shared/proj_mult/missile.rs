use crate::{
    ac,
    ad::{AAttrId, AEffect},
    def::{AttrVal, OF, SERVER_TICK_HZ},
    rd::REffect,
    svc::{SvcCtx, calc::Calc},
    ud::{UItemKey, UProjRange},
    util::{ceil_tick, floor_tick},
};

pub(crate) fn get_proj_attrs_missile(_a_effect: &AEffect) -> [Option<AAttrId>; 2] {
    // Technically mass and agility are also part of attributes which define missile projection, but
    // this is used only in calculator to process modifiers, and there are no missiles which apply
    // any, so array is not extended to 4 attributes just because of this
    [Some(ac::attrs::MAX_VELOCITY), Some(ac::attrs::EXPLOSION_DELAY)]
}

pub(crate) fn get_proj_mult_missile(
    ctx: SvcCtx,
    calc: &mut Calc,
    affector_key: UItemKey,
    _r_effect: &REffect,
    prange: UProjRange,
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
    let flight_time_lower = floor_tick(flight_time);
    // Missiles appear in center of attacking ship and explode on surface of target ship
    let proj_range = prange.get_c2s();
    match flight_time_lower == flight_time {
        // When flight time is integer, need to do fewer calculations
        true => {
            let range = calc_range(max_velocity, flight_time, mass, agility);
            match proj_range <= range {
                true => OF(1.0),
                false => OF(0.0),
            }
        }
        // When flight time is non-integer, any range which lies within lower flight time receives
        // full effect, any range past higher flight time receives no effect, and anything
        // in-between receives partial effect corresponding to flight time fraction part
        false => {
            let range_lower = calc_range(max_velocity, flight_time_lower, mass, agility);
            match proj_range <= range_lower {
                true => OF(1.0),
                false => {
                    let flight_time_higher = ceil_tick(flight_time);
                    let range_higher = calc_range(max_velocity, flight_time_higher, mass, agility);
                    match proj_range > range_higher {
                        true => OF(0.0),
                        false => OF((flight_time * SERVER_TICK_HZ as f64).fract()),
                    }
                }
            }
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
