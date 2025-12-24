use crate::{
    ac,
    ad::AEffectId,
    def::{AttrVal, OF},
    ec,
    ed::EEffectId,
    misc::{DmgKinds, Spool},
    nd::{
        NEffect, NEffectDmgKind,
        effect::data::shared::{base_opc::get_missile_dmg_opc, proj_mult::get_missile_proj_mult},
    },
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::{UItem, UItemKey, UProjData},
};

const E_EFFECT_ID: EEffectId = ec::effects::FOF_MISSILE_LAUNCHING;
const A_EFFECT_ID: AEffectId = ac::effects::FOF_MISSILE_LAUNCHING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        dmg_kind_getter: Some(internal_get_dmg_kind),
        normal_dmg_opc_getter: Some(internal_get_dmg_opc),
        ..
    }
}

fn internal_get_dmg_kind(_u_item: &UItem) -> NEffectDmgKind {
    NEffectDmgKind::Missile
}

fn internal_get_dmg_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    _spool: Option<Spool>,
    projectee_key: Option<UItemKey>,
) -> Option<Output<DmgKinds<AttrVal>>> {
    get_missile_dmg_opc(
        ctx,
        calc,
        projector_key,
        projector_effect,
        projectee_key,
        get_dmg_proj_mult_fof_missile,
    )
}

fn get_dmg_proj_mult_fof_missile(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    // FoF missiles are limited by c2s range
    if let Some(range_limit) = calc.get_item_oattr_oextra(ctx, projector_key, ctx.ac().max_fof_tgt_range)
        && proj_data.get_range_c2s() > range_limit
    {
        return OF(0.0);
    }
    get_missile_proj_mult(ctx, calc, projector_key, projector_effect, projectee_key, proj_data)
}
