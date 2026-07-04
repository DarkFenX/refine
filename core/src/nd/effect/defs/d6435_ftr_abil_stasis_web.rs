use crate::{
    ad::{
        AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModStrength, AEffectModifier, AOp,
    },
    nd::{NEffect, NEffectProjGetter, NEffectProjModSpec, NEffectResist},
};

const EFFECT_AID: AEffectId = AEffectId::FTR_ABIL_STASIS_WEB;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        proj_mod: Some(NEffectProjModSpec {
            // Effect has both optimal and falloff defined, even if fighters themselves have 0
            // falloff
            proj_mult: Some(NEffectProjGetter::GenericRangeFullStsRestricted),
            resist: NEffectResist::AttrRef(AAttrId::FTR_ABIL_STASIS_WEB_RESIST_ID),
        }),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect, adg_warnings: &mut Vec<String>) {
    if !a_effect.modifiers.is_empty() {
        let warning = format!("effect {EFFECT_AID}: fighter web effect has modifiers, overwriting them");
        adg_warnings.push(warning);
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.insert(AEffectModifier {
        strength: AEffectModStrength::Attr(AAttrId::FTR_ABIL_STASIS_WEB_SPEED_PENALTY_INTERIM),
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Target),
        affectee_attr_id: AAttrId::MAX_VELOCITY,
    });
}
