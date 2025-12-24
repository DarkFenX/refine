use crate::{
    ac,
    ad::AEffectId,
    def::AttrVal,
    ec,
    ed::EEffectId,
    misc::{DmgKinds, Spool},
    nd::{
        NEffect, NEffectDmgKind,
        effect::data::shared::{
            base_opc::get_missile_dmg_opc,
            proj_mult::{get_guided_bomb_proj_mult, get_missile_proj_mult},
        },
    },
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::{UItem, UItemKey},
};

const E_EFFECT_ID: EEffectId = ec::effects::MISSILE_LAUNCHING;
const A_EFFECT_ID: AEffectId = ac::effects::MISSILE_LAUNCHING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        dmg_kind_getter: Some(internal_get_dmg_kind),
        normal_dmg_opc_getter: Some(internal_get_dmg_opc),
        ..
    }
}

fn internal_get_dmg_kind(u_item: &UItem) -> NEffectDmgKind {
    match is_guided_bomb(u_item) {
        true => NEffectDmgKind::Bomb,
        false => NEffectDmgKind::Missile,
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
    let proj_mult_getter = match is_guided_bomb(ctx.u_data.items.get(projector_key)) {
        true => get_guided_bomb_proj_mult,
        false => get_missile_proj_mult,
    };
    get_missile_dmg_opc(
        ctx,
        calc,
        projector_key,
        projector_effect,
        projectee_key,
        proj_mult_getter,
    )
}

fn is_guided_bomb(u_item: &UItem) -> bool {
    // There seems to be no way to see the difference between regular missiles and guided bombs,
    // except for item type ID, group or some attributes. We stick to checking group, just because
    // it seems to be the easiest way
    matches!(u_item.get_group_id(), Some(ac::itemgrps::GUIDED_BOMB))
}
