use crate::{
    ad::{AEffectId, AItemGrpId},
    ed::EEffectId,
    nd::{
        NEffect, NEffectDmgKind, NEffectProjMultGetterX, NEffectProjOpcSpec,
        effect::data::shared::base_opc::get_instant_dmg_base_opc,
    },
    ud::UItem,
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
            proj_mult_str: Some(NEffectProjMultGetterX::MissileOrBombApplication),
            proj_mult_chance: Some(NEffectProjMultGetterX::MissileRange),
            ..
        }),
        ..
    }
}

fn internal_get_dmg_kind(u_item: &UItem) -> NEffectDmgKind {
    match u_item.is_guided_bomb() {
        true => NEffectDmgKind::Bomb,
        false => NEffectDmgKind::Missile,
    }
}

impl UItem {
    pub(in crate::nd::effect) fn is_guided_bomb(&self) -> bool {
        // There seems to be no way to see the difference between regular missiles and guided bombs,
        // except for item type ID, group or some attributes. We stick to checking group, just because
        // it seems to be the easiest way
        matches!(self.get_group_id(), Some(AItemGrpId::GUIDED_BOMB))
    }
}
