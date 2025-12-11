use crate::ad::{AAbilId, AEffectId};

#[derive(Clone)]
pub struct AAbil {
    pub id: AAbilId,
    pub effect_id: AEffectId,
}
