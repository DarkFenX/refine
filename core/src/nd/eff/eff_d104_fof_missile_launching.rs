use crate::{
    ac,
    ad::AEffectId,
    def::{AttrVal, OF},
    ec,
    ed::EEffectId,
    misc::{DmgKinds, Spool},
    nd::{
        NEffect, NEffectDmgKind, NEffectHc,
        eff::shared::{dmg_opc::get_dmg_opc_missile, proj_mult::get_proj_mult_missile},
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
    projector_r_effect: &REffect,
    _spool: Option<Spool>,
    projectee_key: Option<UItemKey>,
) -> Option<Output<DmgKinds<AttrVal>>> {
    get_dmg_opc_missile(
        ctx,
        calc,
        projector_key,
        projector_r_effect,
        projectee_key,
        get_dmg_proj_mult_fof_missile,
    )
}

fn get_dmg_proj_mult_fof_missile(
    ctx: SvcCtx,
    calc: &mut Calc,
    affector_key: UItemKey,
    r_effect: &REffect,
    u_proj_data: UProjData,
) -> AttrVal {
    let range_limit = calc
        .get_item_attr_val_full(ctx, affector_key, &ac::attrs::MAX_FOF_TGT_RANGE)
        .unwrap()
        .extra;
    // FoF missile is limited by c2s range. Tested on 2025-08-12 on Thunderdome, using civilian LML
    // Minokawa (3k radius) with HG hydra + MGCs + hydraulics vs chremoas and dagon at 96900 and
    // 97100 overview range
    if u_proj_data.get_range_c2s() > range_limit {
        return OF(0.0);
    };
    get_proj_mult_missile(ctx, calc, affector_key, r_effect, u_proj_data)
}
