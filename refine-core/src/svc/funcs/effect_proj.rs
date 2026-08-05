use crate::{
    PValue,
    misc::EffectSpec,
    svc::{Calc, SvcCtx},
    ud::UItemId,
};

pub(in crate::svc) fn get_espec_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_espec: EffectSpec,
    projectee_uid: UItemId,
    include_crits: bool,
) -> Option<PValue> {
    let projector_effect = ctx.u_data.r_data.get_effect_by_rid(projector_espec.effect_rid);
    let proj_mult_getter = projector_effect.proj_mod?.proj_mult?;
    let proj_data = ctx.eff_projs.get_proj_data(projector_espec, projectee_uid)?;
    Some(proj_mult_getter.get_proj_mult(
        ctx,
        calc,
        projector_espec.item_uid,
        projector_effect,
        projectee_uid,
        proj_data,
        include_crits,
    ))
}
