use crate::{
    ac,
    def::{AttrVal, OF},
    svc::{SvcCtx, calc::Calc},
    ud::{UItemKey, UProjData},
};

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
