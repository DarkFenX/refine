use crate::cacher_json::data::{CAbilId, CEffectId};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::cacher_json) struct CAbil {
    id: CAbilId,
    effect_id: CEffectId,
}
impl From<&rc::ad::AAbil> for CAbil {
    fn from(a_abil: &rc::ad::AAbil) -> Self {
        CAbil {
            id: a_abil.id,
            effect_id: (&a_abil.effect_id).into(),
        }
    }
}
impl From<&CAbil> for rc::ad::AAbil {
    fn from(c_abil: &CAbil) -> Self {
        Self {
            id: c_abil.id,
            effect_id: (&c_abil.effect_id).into(),
        }
    }
}
