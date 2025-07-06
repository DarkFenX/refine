use crate::{
    def::{AttrVal, ItemKey},
    misc::EffectSpec,
    svc::{SvcCtx, calc::Calc},
};

pub(crate) fn get_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_espec: EffectSpec,
    projectee_key: ItemKey,
) -> Option<AttrVal> {
    let prange = ctx.eprojs.get_range(projector_espec, projectee_key)?;
    let projector_a_effect = ctx.uad.src.get_a_effect(&projector_espec.a_effect_id)?;
    let proj_mult_getter = projector_a_effect.rt.get_proj_mult?;
    Some(proj_mult_getter(
        ctx,
        calc,
        projector_espec.item_key,
        &projector_a_effect.ae,
        prange,
    ))
}
