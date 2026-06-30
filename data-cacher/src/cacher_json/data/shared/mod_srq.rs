use crate::cacher_json::data::AdaptedConv;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::cacher_json::data) enum CModifierSrq {
    SelfRef,
    ItemId(i32),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AdaptedConv for CModifierSrq {
    type AEntity = rc::ad::AModifierSrq;

    fn from_adapted(a_modifier_srq: &Self::AEntity) -> Self {
        match a_modifier_srq {
            Self::AEntity::SelfRef => Self::SelfRef,
            Self::AEntity::ItemId(item_id) => Self::ItemId(item_id.into_i32()),
        }
    }

    fn into_adapted(self) -> Self::AEntity {
        match self {
            Self::SelfRef => Self::AEntity::SelfRef,
            Self::ItemId(item_id) => Self::AEntity::ItemId(rc::ad::AItemId::from_i32(item_id)),
        }
    }
}
