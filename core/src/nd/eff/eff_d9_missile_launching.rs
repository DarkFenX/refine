use crate::{
    ac,
    ad::AEffectId,
    def::AttrVal,
    ec,
    ed::EEffectId,
    misc::{DmgKinds, Spool},
    nd::{
        NEffect, NEffectDmgKind, NEffectHc,
        eff::shared::{dmg_opc::get_dmg_opc_missile, proj_mult::get_missile_proj_mult},
    },
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::{UItem, UItemKey},
};

const E_EFFECT_ID: EEffectId = ec::effects::MISSILE_LAUNCHING;
const A_EFFECT_ID: AEffectId = ac::effects::MISSILE_LAUNCHING;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        hc: NEffectHc {
            dmg_kind_getter: Some(internal_get_dmg_kind),
            normal_dmg_opc_getter: Some(internal_get_dmg_opc),
            ..
        },
        ..
    }
}

fn internal_get_dmg_kind(u_item: &UItem) -> NEffectDmgKind {
    // There seems to be no way to see the difference between regular missiles and guided bombs,
    // except for item type ID, group or some attributes. We stick to checking group, just because
    // it seems to be the easiest way
    match u_item.get_group_id() {
        Some(ac::itemgrps::GUIDED_BOMB) => NEffectDmgKind::Bomb,
        _ => NEffectDmgKind::Missile,
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
        get_missile_proj_mult,
    )
}
