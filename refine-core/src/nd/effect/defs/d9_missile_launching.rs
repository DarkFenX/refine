use crate::{
    ad::{AEffectId, AItemGrpId},
    nd::{NEffect, NEffectDmgKindGetter, NEffectDmgOutputGetter, NEffectProjMultGetter, NEffectProjOpcSpec},
    ud::UItem,
};

const EFFECT_AID: AEffectId = AEffectId::MISSILE_LAUNCHING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        dmg_kind: Some(NEffectDmgKindGetter::MissileLaunching),
        normal_dmg: Some(NEffectProjOpcSpec {
            base: NEffectDmgOutputGetter::Regular,
            proj_mult_str: Some(NEffectProjMultGetter::MissileLaunchingApplication),
            proj_mult_chance: Some(NEffectProjMultGetter::MissileRange),
            ..
        }),
        ..
    }
}

impl UItem {
    pub(in crate::nd::effect) fn is_guided_bomb(&self) -> bool {
        let group_id = match self.get_r_item_base() {
            Some(rib) => rib.grp_id,
            None => return false,
        };
        // There seems to be no way to see the difference between regular missiles and guided bombs,
        // except for item type ID, group or some attributes. We stick to checking group, just
        // because it seems to be the easiest way
        group_id == AItemGrpId::GUIDED_BOMB
    }
}
