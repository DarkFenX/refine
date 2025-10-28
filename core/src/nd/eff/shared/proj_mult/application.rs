use crate::{
    ac,
    ad::AAttrId,
    def::{AttrVal, OF},
    rd::REffect,
    svc::{SvcCtx, calc::Calc, item_funcs},
    ud::{UItemKey, UProjData},
    util::Xyz,
};

pub(super) fn get_application_mult_turret(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    let angular_speed = calc_angular(ctx, calc, projector_key, projectee_key, proj_data);
    let turret_sig_radius = calc
        .get_item_attr_val_full(ctx, projector_key, &ac::attrs::OPTIMAL_SIG_RADIUS)
        .unwrap()
        .extra
        .max(OF(0.0));
    let turret_tracking_speed = match projector_effect.get_track_attr_id() {
        Some(tracking_speed_attr_id) => calc
            .get_item_attr_val_full(ctx, projector_key, &tracking_speed_attr_id)
            .unwrap()
            .extra
            .max(OF(0.0)),
        None => OF(0.0),
    };
    let tgt_sig_radius = item_funcs::get_sig_radius(ctx, calc, projectee_key).unwrap_or(OF(0.0));
    let result = ordered_float::Float::powf(
        OF(0.5),
        OF((angular_speed * turret_sig_radius / turret_tracking_speed / tgt_sig_radius).powi(2)),
    );
    match result.is_nan() {
        true => OF(0.0),
        false => result.clamp(OF(0.0), OF(1.0)),
    }
}

pub(super) fn get_application_mult_missile(
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
    let tgt_sig_radius = item_funcs::get_sig_radius(ctx, calc, projectee_key).unwrap_or(OF(0.0));
    let tgt_speed = proj_data.get_tgt_speed() * item_funcs::get_speed(ctx, calc, projectee_key).unwrap_or(OF(0.0));
    // "Static" part
    let radius_ratio = tgt_sig_radius / src_er;
    if radius_ratio.is_nan() {
        return OF(0.0);
    }
    // "Mobile" part
    let mobile_mult = OF((radius_ratio * src_ev / tgt_speed).powf(src_drf.into_inner()));
    if mobile_mult.is_nan() {
        return OF(0.0);
    }
    radius_ratio.min(mobile_mult).clamp(OF(0.0), OF(1.0))
}

pub(super) fn get_radius_ratio_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projectee_key: UItemKey,
    src_attr_id: &AAttrId,
) -> AttrVal {
    let src_effect_radius = calc
        .get_item_attr_val_full(ctx, projector_key, src_attr_id)
        .unwrap()
        .extra
        .max(OF(0.0));
    let tgt_sig_radius = item_funcs::get_sig_radius(ctx, calc, projectee_key).unwrap_or(OF(0.0));
    let radius_ratio = tgt_sig_radius / src_effect_radius;
    if radius_ratio.is_nan() {
        return OF(0.0);
    }
    radius_ratio.clamp(OF(0.0), OF(1.0))
}

// Utility
fn calc_angular(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    let coordinates = proj_data.get_tgt_coordinates() - proj_data.get_src_coordinates();
    let src_velocity = match ctx.u_data.get_physic_item_key(projector_key) {
        Some(projector_physic_key) => get_vector(
            ctx,
            calc,
            projector_physic_key,
            proj_data.get_src_direction(),
            proj_data.get_src_speed(),
        ),
        None => Xyz::default(),
    };
    let tgt_velocity = get_vector(
        ctx,
        calc,
        projectee_key,
        proj_data.get_tgt_direction(),
        proj_data.get_tgt_speed(),
    );
    let velocity = tgt_velocity - src_velocity;
    let radial_unit = coordinates.get_vector_unit();
    let radial_component = Xyz::get_vector_dot_product(velocity, radial_unit);
    let radial_velocity = radial_unit * radial_component;
    let transversal_velocity = velocity - radial_velocity;
    let result = transversal_velocity.get_vector_magnitude() / proj_data.get_range_c2c();
    match result.is_nan() {
        true => OF(0.0),
        false => result,
    }
}

fn get_vector(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey, direction: Xyz, speed_perc: AttrVal) -> Xyz {
    if speed_perc <= OF(0.0) {
        return Xyz::default();
    }
    let speed_max = item_funcs::get_speed(ctx, calc, item_key).unwrap_or(OF(0.0));
    if speed_max <= OF(0.0) {
        return Xyz::default();
    }
    direction * (speed_perc * speed_max)
}
