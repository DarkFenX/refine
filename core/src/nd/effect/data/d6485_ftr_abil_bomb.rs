use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{NEffect, NEffectCharge, NEffectChargeLoc},
};

const EFFECT_EID: EEffectId = EEffectId::FTR_ABIL_BOMB;
const EFFECT_AID: AEffectId = AEffectId::FTR_ABIL_BOMB;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        charge: Some(NEffectCharge {
            location: NEffectChargeLoc::Autocharge(AAttrId::FTR_ABIL_BOMB_TYPE),
            activates_charge: true,
        }),
        ..
    }
}
