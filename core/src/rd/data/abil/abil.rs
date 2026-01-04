use crate::{
    ad::{AAbil, AAbilId, AEffectId},
    rd::REffectId,
    util::{GetId, RMap},
};

// Represents a fighter ability.
pub(crate) struct RAbil {
    pub(crate) aid: AAbilId,
    pub(crate) effect_aid: AEffectId,
    pub(crate) effect_rid: REffectId,
}
impl RAbil {
    pub(in crate::rd) fn from_a_abil(a_abil: &AAbil) -> Self {
        Self {
            aid: a_abil.id,
            effect_aid: a_abil.effect_id,
            // Fields which depend on data not available during instantiation
            effect_rid: Default::default(),
        }
    }
    pub(in crate::rd) fn fill_runtime(&mut self, effect_aid_rid_map: &RMap<AEffectId, REffectId>) {
        // Adapted data generator guarantees that all abilities will have a backing effect
        self.effect_rid = *effect_aid_rid_map.get(&self.effect_aid).unwrap()
    }
}
impl GetId<AAbilId> for RAbil {
    fn get_id(&self) -> AAbilId {
        self.aid
    }
}
