use crate::{
    ad::{AEffectId, AItemGrpId},
    ed::EEffectId,
    nd::{NEffect, NEffectDmgKindGetter, NEffectDmgOutputGetter, NEffectProjGetter, NEffectProjOpcSpec},
    ud::UItem,
};

const EFFECT_EID: EEffectId = EEffectId::MISSILE_LAUNCHING;
const EFFECT_AID: AEffectId = AEffectId::MISSILE_LAUNCHING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        dmg_kind: Some(NEffectDmgKindGetter::MissileLaunching),
        normal_dmg: Some(NEffectProjOpcSpec {
            base: NEffectDmgOutputGetter::Regular,
            proj_mult_str: Some(NEffectProjGetter::MissileLaunchingApplication),
            proj_mult_chance: Some(NEffectProjGetter::MissileRange),
            ..
        }),
        ..
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
