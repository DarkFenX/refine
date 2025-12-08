use crate::{
    ad::{AAbil, AAbilId, AEffectId},
    rd::REffectKey,
    util::{GetId, Named, RMap},
};

// Represents a fighter ability.
pub(crate) struct RAbil {
    pub(crate) id: AAbilId,
    pub(crate) effect_id: AEffectId,
    // Fields which depend on slab keys
    pub(crate) effect_key: REffectKey,
}
impl RAbil {
    pub(in crate::rd) fn from_a_abil(a_abil: &AAbil) -> Self {
        Self {
            id: a_abil.id,
            effect_id: a_abil.effect_id,
            // Fields which depend on slab keys
            effect_key: Default::default(),
        }
    }
    pub(in crate::rd) fn fill_key_dependents(&mut self, effect_id_key_map: &RMap<AEffectId, REffectKey>) {
        // Adapted data guarantees that all abilities will have a backing effect
        self.effect_key = *effect_id_key_map.get(&self.effect_id).unwrap()
    }
}
impl GetId<AAbilId> for RAbil {
    fn get_id(&self) -> AAbilId {
        self.id
    }
}
impl Named for RAbil {
    fn get_name() -> &'static str {
        "RAbil"
    }
}
