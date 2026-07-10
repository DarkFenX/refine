use crate::{
    ad::{AAttrId, AEffectId},
    nd::{
        NEffect, NEffectGeneralOutputGetter, NEffectProjGetter, NEffectProjOpcSpec, NEffectResist, NEffectSpoolAttrs,
    },
};

const EFFECT_AID: AEffectId = AEffectId::SHIP_MOD_REMOTE_ARMOR_MUTADAPTIVE_REPAIRER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        spool_attrs: Some(NEffectSpoolAttrs {
            step_attr_id: AAttrId::REP_MULT_BONUS_PER_CYCLE,
            max_attr_id: AAttrId::REP_MULT_BONUS_MAX,
        }),
        outgoing_armor_rep: Some(NEffectProjOpcSpec {
            base: NEffectGeneralOutputGetter::RepArmor,
            spoolable: true,
            proj_mult_str: Some(NEffectProjGetter::GenericRangeSimpleSts),
            resist: Some(NEffectResist::Standard),
            remote_limit_attr_id: Some(AAttrId::ARMOR_HP),
            ..
        }),
        ..
    }
}
