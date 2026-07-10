use crate::{
    ad::{AAttrId, AEffectId},
    nd::{NEffect, NEffectCharge, NEffectChargeLoc},
};

const EFFECT_AID: AEffectId = AEffectId::FTR_ABIL_LAUNCH_BOMB;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        charge: Some(NEffectCharge {
            location: NEffectChargeLoc::Autocharge(AAttrId::FTR_ABIL_BOMB_TYPE),
            activates_charge: true,
        }),
        ..
    }
}
