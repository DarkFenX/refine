use crate::{
    ac,
    ad::AEffectId,
    def::AttrVal,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectDmgKind, NEffectProjOpcSpec,
        effect::data::shared::{
            base_opc::get_instant_dmg_base_opc,
            proj_mult::{get_bomb_application_mult, get_missile_application_mult, get_missile_range_mult},
        },
    },
    rd::REffect,
    svc::{SvcCtx, calc::Calc},
    ud::{UItem, UItemId, UProjData},
};

const E_EFFECT_ID: EEffectId = ec::effects::MISSILE_LAUNCHING;
const A_EFFECT_ID: AEffectId = ac::effects::MISSILE_LAUNCHING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        dmg_kind_getter: Some(internal_get_dmg_kind),
        normal_dmg_opc_spec: Some(NEffectProjOpcSpec {
            base: get_instant_dmg_base_opc,
            proj_mult_str: Some(internal_get_missile_application_mult),
            proj_mult_chance: Some(get_missile_range_mult),
            ..
        }),
        ..
    }
}

fn internal_get_dmg_kind(u_item: &UItem) -> NEffectDmgKind {
    match is_guided_bomb(u_item) {
        true => NEffectDmgKind::Bomb,
        false => NEffectDmgKind::Missile,
    }
}

fn internal_get_missile_application_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemId,
    projector_effect: &REffect,
    projectee_key: UItemId,
    proj_data: UProjData,
) -> AttrVal {
    let proj_mult_getter = match is_guided_bomb(ctx.u_data.items.get(projector_key)) {
        true => get_bomb_application_mult,
        false => get_missile_application_mult,
    };
    proj_mult_getter(ctx, calc, projector_key, projector_effect, projectee_key, proj_data)
}

fn is_guided_bomb(u_item: &UItem) -> bool {
    // There seems to be no way to see the difference between regular missiles and guided bombs,
    // except for item type ID, group or some attributes. We stick to checking group, just because
    // it seems to be the easiest way
    matches!(u_item.get_group_id(), Some(ac::itemgrps::GUIDED_BOMB))
}
