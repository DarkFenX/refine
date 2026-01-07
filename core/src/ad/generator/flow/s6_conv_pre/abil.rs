use crate::{
    ad::{AAbil, AAbilId, AEffectId, generator::get_abil_effect},
    ed::EData,
    util::RMap,
};

pub(in crate::ad::generator::flow::s6_conv_pre) fn conv_abils(e_data: &EData) -> RMap<AAbilId, AAbil> {
    // Abilities not known to the getter should've been removed during validation, so we just unwrap
    e_data
        .abils
        .data
        .iter()
        .map(|e_abil| {
            let abil_aid = AAbilId::from_eid(e_abil.id);
            let effect_aid = AEffectId::from_eid(get_abil_effect(e_abil.id).unwrap());
            (
                abil_aid,
                AAbil {
                    id: abil_aid,
                    effect_id: effect_aid,
                },
            )
        })
        .collect()
}
