use crate::{
    ac,
    ad::AEffectId,
    def::{AttrVal, OF},
    ec,
    ed::EEffectId,
    misc::{DmgKinds, Spool},
    nd::{
        NEffect, NEffectDmgKind, NEffectHc,
        eff::shared::{dmg_opc::get_dmg_opc_missile, proj_mult::get_missile_proj_mult},
    },
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::{UItemKey, UProjData},
};

const E_EFFECT_ID: EEffectId = ec::effects::FOF_MISSILE_LAUNCHING;
const A_EFFECT_ID: AEffectId = ac::effects::FOF_MISSILE_LAUNCHING;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        hc: NEffectHc {
            dmg_kind: Some(NEffectDmgKind::Missile),
            normal_dmg_opc_getter: Some(internal_get_dmg_opc),
            ..
        },
        ..
    }
}

fn internal_get_dmg_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    _spool: Option<Spool>,
    projectee_key: Option<UItemKey>,
) -> Option<Output<DmgKinds<AttrVal>>> {
    get_dmg_opc_missile(
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
    let range_limit = calc
        .get_item_attr_val_full(ctx, projector_key, &ac::attrs::MAX_FOF_TGT_RANGE)
        .unwrap()
        .extra;
    // FoF missiles are limited by c2s range
    if proj_data.get_range_c2s() > range_limit {
        return OF(0.0);
    };
    get_missile_proj_mult(ctx, calc, projector_key, projector_effect, projectee_key, proj_data)
}
