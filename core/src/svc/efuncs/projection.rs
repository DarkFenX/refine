use crate::{
    ad,
    def::{AttrVal, ItemKey},
    misc::EffectSpec,
    svc::{SvcCtx, calc::Calc},
};

pub(crate) fn get_espec_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_espec: EffectSpec,
    projectee_key: ItemKey,
) -> Option<AttrVal> {
    let projector_a_effect = ctx.uad.src.get_a_effect(&projector_espec.a_effect_id)?;
    get_effect_proj_mult(ctx, calc, projector_espec.item_key, projector_a_effect, projectee_key)
}

pub(crate) fn get_effect_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: ItemKey,
    projector_a_effect: &ad::AEffectRt,
    projectee_key: ItemKey,
) -> Option<AttrVal> {
    let prange = ctx
        .eprojs
        .get_range(EffectSpec::new(projector_key, projector_a_effect.ae.id), projectee_key)?;
    let proj_mult_getter = projector_a_effect.hc.get_proj_mult?;
    Some(proj_mult_getter(
        ctx,
        calc,
        projector_key,
        &projector_a_effect.ae,
        prange,
    ))
}
