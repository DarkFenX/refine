use crate::{
    def::{AttrVal, OF},
    rd::{RAttrId, REffect},
    svc::{SvcCtx, calc::Calc, funcs},
    ud::{UItemId, UProjData},
    util::Xyz,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::nd::effect::data) fn get_missile_application_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    _effect: &REffect,
    projectee_uid: UItemId,
    proj_data: UProjData,
) -> AttrVal {
    let src_er = calc
        .get_item_oattr_ffb_extra(ctx, projector_uid, ctx.ac().aoe_cloud_size, OF(0.0))
        .max(OF(0.0));
    let src_ev = calc
        .get_item_oattr_ffb_extra(ctx, projector_uid, ctx.ac().aoe_velocity, OF(0.0))
        .max(OF(0.0));
    let src_drf = calc
        .get_item_oattr_ffb_extra(ctx, projector_uid, ctx.ac().aoe_dmg_reduction_factor, OF(0.0))
        .max(OF(0.0));
    let tgt_sig_radius = funcs::get_sig_radius(ctx, calc, projectee_uid);
    let tgt_speed = proj_data.get_tgt_speed() * funcs::get_speed(ctx, calc, projectee_uid);
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

pub(in crate::nd::effect::data) fn get_bomb_application_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    _effect: &REffect,
    projectee_uid: UItemId,
    _proj_data: UProjData,
) -> AttrVal {
    get_radius_ratio_mult(ctx, calc, projector_uid, projectee_uid, ctx.ac().aoe_cloud_size)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) fn get_turret_application_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    projectee_uid: UItemId,
    proj_data: UProjData,
) -> AttrVal {
    let angular_speed = calc_angular(ctx, calc, projector_uid, projectee_uid, proj_data);
    let turret_sig_radius = calc
        .get_item_oattr_ffb_extra(ctx, projector_uid, ctx.ac().optimal_sig_radius, OF(0.0))
        .max(OF(0.0));
    let turret_tracking_speed = calc
        .get_item_oattr_ffb_extra(ctx, projector_uid, effect.track_attr_rid, OF(0.0))
        .max(OF(0.0));
    let tgt_sig_radius = funcs::get_sig_radius(ctx, calc, projectee_uid);
    let result = ordered_float::Float::powf(
        OF(0.5),
        OF((angular_speed * turret_sig_radius / turret_tracking_speed / tgt_sig_radius).powi(2)),
    );
    match result.is_nan() {
        true => OF(0.0),
        false => result.clamp(OF(0.0), OF(1.0)),
    }
}

pub(super) fn get_radius_ratio_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    projectee_uid: UItemId,
    effect_radius_rid: Option<RAttrId>,
) -> AttrVal {
    let src_effect_radius = calc
        .get_item_oattr_ffb_extra(ctx, projector_uid, effect_radius_rid, OF(0.0))
        .max(OF(0.0));
    let tgt_sig_radius = funcs::get_sig_radius(ctx, calc, projectee_uid);
    let radius_ratio = tgt_sig_radius / src_effect_radius;
    if radius_ratio.is_nan() {
        return OF(0.0);
    }
    radius_ratio.clamp(OF(0.0), OF(1.0))
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Utility
////////////////////////////////////////////////////////////////////////////////////////////////////
fn calc_angular(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    projectee_uid: UItemId,
    proj_data: UProjData,
) -> AttrVal {
    let coordinates = proj_data.get_tgt_coordinates() - proj_data.get_src_coordinates();
    let src_velocity = match ctx.u_data.get_physics_carrier(projector_uid) {
        Some(projector_carrier_key) => get_vector(
            ctx,
            calc,
            projector_carrier_key,
            proj_data.get_src_direction(),
            proj_data.get_src_speed(),
        ),
        None => Xyz::default(),
    };
    let tgt_velocity = get_vector(
        ctx,
        calc,
        projectee_uid,
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

fn get_vector(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId, direction: Xyz, speed_perc: AttrVal) -> Xyz {
    if speed_perc <= OF(0.0) {
        return Xyz::default();
    }
    let speed_max = funcs::get_speed(ctx, calc, item_uid);
    if speed_max <= OF(0.0) {
        return Xyz::default();
    }
    direction * (speed_perc * speed_max)
}
