use crate::{
    ad::{AEffectId, AItemGrpId},
    ed::EEffectId,
    nd::{
        NEffect, NEffectDmgKind, NEffectProjOpcSpec,
        effect::data::shared::{
            base_opc::get_instant_dmg_base_opc,
            proj_mult::{get_bomb_application_mult, get_missile_application_mult, get_missile_range_mult},
        },
    },
    num::PValue,
    rd::REffect,
    svc::{SvcCtx, calc::Calc},
    ud::{UItem, UItemId, UProjData},
};

const EFFECT_EID: EEffectId = EEffectId::MISSILE_LAUNCHING;
const EFFECT_AID: AEffectId = AEffectId::MISSILE_LAUNCHING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
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
    projector_uid: UItemId,
    projector_effect: &REffect,
    projectee_uid: UItemId,
    proj_data: UProjData,
) -> PValue {
    let proj_mult_getter = match is_guided_bomb(ctx.u_data.items.get(projector_uid)) {
        true => get_bomb_application_mult,
        false => get_missile_application_mult,
    };
    proj_mult_getter(ctx, calc, projector_uid, projector_effect, projectee_uid, proj_data)
}

fn is_guided_bomb(u_item: &UItem) -> bool {
    // There seems to be no way to see the difference between regular missiles and guided bombs,
    // except for item type ID, group or some attributes. We stick to checking group, just because
    // it seems to be the easiest way
    matches!(u_item.get_group_id(), Some(AItemGrpId::GUIDED_BOMB))
}
