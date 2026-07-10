use crate::ad::{AAbil, AAbilId, AAbils, ADataGenerator, AEffectId, generator::get_abil_effect};

impl ADataGenerator {
    pub(super) fn conv_abils(&mut self) {
        // Abilities not known to the getter should've been removed during validation, so we just unwrap
        let a_abils = self
            .e_data
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
            .collect();
        self.a_data.abils = AAbils { data: a_abils };
    }
}
