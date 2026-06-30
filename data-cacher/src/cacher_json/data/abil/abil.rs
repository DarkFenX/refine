use crate::cacher_json::data::AdaptedConv;

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::cacher_json::data) struct CAbil {
    id: i32,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    effect_id: rc::ad::AEffectId,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AdaptedConv for CAbil {
    type AEntity = rc::ad::AAbil;

    fn from_adapted(a_abil: &Self::AEntity) -> Self {
        Self {
            id: a_abil.id.into_i32(),
            effect_id: a_abil.effect_id,
        }
    }

    fn into_adapted(self) -> Self::AEntity {
        Self::AEntity {
            id: rc::ad::AAbilId::from_i32(self.id),
            effect_id: self.effect_id,
        }
    }
}
