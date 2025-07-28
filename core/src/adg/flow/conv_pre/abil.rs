use crate::{
    ad::{AAbil, AAbilId, AEffectId},
    adg::get_abil_effect,
    ed::EData,
    util::RMap,
};

pub(in crate::adg::flow::conv_pre) fn conv_abils(e_data: &EData) -> RMap<AAbilId, AAbil> {
    // Abilities not known to the getter have been cleaned up by now, so we just unwrap
    e_data
        .abils
        .data
        .iter()
        .map(|e_abil| {
            (
                e_abil.id,
                AAbil {
                    id: e_abil.id,
                    effect_id: AEffectId::Dogma(get_abil_effect(e_abil.id).unwrap()),
                },
            )
        })
        .collect()
}
