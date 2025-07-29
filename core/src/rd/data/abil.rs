use crate::{
    ad::{AAbil, AAbilId, AEffectId},
    rd::REffectKey,
    util::{GetId, Named, RMap},
};

// Represents a fighter ability.
pub(crate) struct RAbil {
    a_abil: AAbil,
    effect_key: Option<REffectKey>,
}
impl RAbil {
    pub(in crate::rd) fn new(a_abil: AAbil) -> Self {
        Self {
            a_abil,
            effect_key: None,
        }
    }
    pub(in crate::rd) fn fill_key_dependents(&mut self, effect_id_key_map: &RMap<AEffectId, REffectKey>) {
        self.effect_key = effect_id_key_map.get(&self.a_abil.effect_id).copied()
    }
    pub(crate) fn get_effect_key(&self) -> REffectKey {
        // Adapted data guarantees that all abilities will have a backing effect
        self.effect_key.unwrap()
    }
}
impl GetId<AAbilId> for RAbil {
    fn get_id(&self) -> AAbilId {
        self.a_abil.id
    }
}
impl Named for RAbil {
    fn get_name() -> &'static str {
        "RAbil"
    }
}
