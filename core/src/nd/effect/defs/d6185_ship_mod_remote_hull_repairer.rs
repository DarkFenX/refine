use crate::{
    ad::{AAttrId, AEffectId},
    nd::{NEffect, NEffectGeneralOutputGetter, NEffectProjGetter, NEffectProjOpcSpec, NEffectResist},
};

const EFFECT_AID: AEffectId = AEffectId::SHIP_MOD_REMOTE_HULL_REPAIRER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        outgoing_hull_rep: Some(NEffectProjOpcSpec {
            base: NEffectGeneralOutputGetter::RepHull,
            proj_mult_str: Some(NEffectProjGetter::GenericRangeFullStsRestricted),
            resist: Some(NEffectResist::Standard),
            remote_limit_attr_id: Some(AAttrId::HP),
            ..
        }),
        ..
    }
}
