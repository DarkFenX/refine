use super::shared::{
    mk_cannot_cloak_mod_hardcoded, mk_disallow_drive_jump_mod_hardcoded, mk_disallow_warp_mod_hardcoded,
};
use crate::{
    ad::{AAttrId, AEffect, AEffectId},
    nd::{
        NEffect, NEffectGeneralOutputGetter, NEffectNeut, NEffectNeutKind, NEffectProjGetter, NEffectProjOpcSpec,
        NEffectResist,
    },
};

const EFFECT_AID: AEffectId = AEffectId::DOOMSDAY_AOE_NEUT;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        neut: Some(NEffectNeut {
            kind: NEffectNeutKind::Module,
            checker: None,
            ospec: NEffectProjOpcSpec {
                base: NEffectGeneralOutputGetter::NeutAoe,
                proj_mult_str: Some(NEffectProjGetter::AoeBurst),
                resist: Some(NEffectResist::Standard),
                remote_limit_attr_id: Some(AAttrId::CAPACITOR_CAPACITY),
                ..
            },
        }),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {EFFECT_AID}: neut projector effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.extend([
        mk_disallow_warp_mod_hardcoded(),
        mk_disallow_drive_jump_mod_hardcoded(),
        mk_cannot_cloak_mod_hardcoded(),
    ]);
}
