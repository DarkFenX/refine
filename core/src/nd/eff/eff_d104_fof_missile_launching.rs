use crate::{
    ac,
    ad::AEffectId,
    def::{AttrVal, OF},
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectDmgKind, NEffectHc,
        eff::shared::{
            dmg_opc::get_dmg_opc_missile,
            proj_mult::{get_proj_attrs_missile, get_proj_mult_missile},
        },
    },
    rd::REffect,
    svc::{SvcCtx, calc::Calc},
    ud::{UItemKey, UProjRange},
};

const E_EFFECT_ID: EEffectId = ec::effects::FOF_MISSILE_LAUNCHING;
const A_EFFECT_ID: AEffectId = ac::effects::FOF_MISSILE_LAUNCHING;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        xt_get_proj_attrs: Some(get_proj_attrs_missile),
        hc: NEffectHc {
            dmg_kind: Some(NEffectDmgKind::Missile),
            proj_mult_getter: Some(get_proj_mult_fof_missile),
            normal_dmg_opc_getter: Some(get_dmg_opc_missile),
            ..
        },
        ..
    }
}

pub(crate) fn get_proj_mult_fof_missile(
    ctx: SvcCtx,
    calc: &mut Calc,
    affector_key: UItemKey,
    r_effect: &REffect,
    prange: UProjRange,
) -> AttrVal {
    let range_limit = calc
        .get_item_attr_val_full(ctx, affector_key, &ac::attrs::MAX_FOF_TGT_RANGE)
        .unwrap()
        .extra;
    // TODO: test if FoF range limit is calculated s2s
    if prange.get_s2s() > range_limit {
        return OF(0.0);
    };
    get_proj_mult_missile(ctx, calc, affector_key, r_effect, prange)
}
