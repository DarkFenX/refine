use crate::{
    ad::{AAbil, AAbilId, ADogmaEffectId, AEffectId, generator::get_abil_effect},
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
            let abil_aid = AAbilId::new(e_abil.id.into_inner());
            let effect_aid = AEffectId::Dogma(ADogmaEffectId::new(get_abil_effect(e_abil.id).unwrap().into_inner()));
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
