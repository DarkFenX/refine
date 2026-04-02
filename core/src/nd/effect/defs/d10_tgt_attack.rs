use crate::{
    ad::{AAttrId, AEffectId},
    ed::EEffectId,
    nd::{
        NEffect, NEffectCharge, NEffectChargeLoc, NEffectDmgKindGetter, NEffectDmgOutputGetter, NEffectProjMultGetter,
        NEffectProjOpcSpec,
    },
};

const EFFECT_EID: EEffectId = EEffectId::TGT_ATTACK;
const EFFECT_AID: AEffectId = AEffectId::TGT_ATTACK;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        charge: Some(NEffectCharge {
            // Autocharge attribute ID is defined just for completeness of data. CCP Kestrel
            // confirmed civilian guns use on-gun damage attributes, and ammo is possibly loaded
            // just for various side effects (e.g. ammo affecting module attributes, or shot
            // graphics). The library doesn't implement on-module autocharges just for this
            // effect.
            location: NEffectChargeLoc::TargetAttack(AAttrId::AMMO_LOADED),
            activates_charge: false,
        }),
        dmg_kind: Some(NEffectDmgKindGetter::Turret),
        normal_dmg: Some(NEffectProjOpcSpec {
            base: NEffectDmgOutputGetter::TargetAttack,
            proj_mult_str: Some(NEffectProjMultGetter::Turret),
            ..
        }),
        ..
    }
}
