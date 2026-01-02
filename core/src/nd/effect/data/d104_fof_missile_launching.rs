use crate::{
    ac,
    ad::AEffectId,
    def::{AttrVal, OF},
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectDmgKind, NEffectProjOpcSpec,
        effect::data::shared::{
            base_opc::get_instant_dmg_base_opc,
            proj_mult::{get_missile_application_mult, get_missile_range_mult},
        },
    },
    rd::REffect,
    svc::{SvcCtx, calc::Calc},
    ud::{UItem, UItemId, UProjData},
};

const E_EFFECT_ID: EEffectId = ec::effects::FOF_MISSILE_LAUNCHING;
const A_EFFECT_ID: AEffectId = ac::effects::FOF_MISSILE_LAUNCHING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        dmg_kind_getter: Some(internal_get_dmg_kind),
        normal_dmg_opc_spec: Some(NEffectProjOpcSpec {
            base: get_instant_dmg_base_opc,
            proj_mult_str: Some(get_missile_application_mult),
            proj_mult_chance: Some(internal_get_fof_missile_range_mult),
            ..
        }),
        ..
    }
}

fn internal_get_dmg_kind(_u_item: &UItem) -> NEffectDmgKind {
    NEffectDmgKind::Missile
}

fn internal_get_fof_missile_range_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemId,
    projector_effect: &REffect,
    projectee_key: UItemId,
    proj_data: UProjData,
) -> AttrVal {
    // FoF missiles are limited by c2s range
    if let Some(range_limit) = calc.get_item_oattr_oextra(ctx, projector_key, ctx.ac().max_fof_tgt_range)
        && proj_data.get_range_c2s() > range_limit
    {
        return OF(0.0);
    }
    get_missile_range_mult(ctx, calc, projector_key, projector_effect, projectee_key, proj_data)
}
