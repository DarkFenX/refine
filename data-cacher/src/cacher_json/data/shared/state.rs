use crate::cacher_json::data::AdaptedConv;

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::cacher_json::data) enum CState {
    Disabled,
    Offline,
    Online,
    Active,
    Overload,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AdaptedConv for CState {
    type AEntity = rc::ad::AState;

    fn from_adapted(a_state: &Self::AEntity) -> Self {
        match a_state {
            Self::AEntity::Disabled => Self::Disabled,
            Self::AEntity::Offline => Self::Offline,
            Self::AEntity::Online => Self::Online,
            Self::AEntity::Active => Self::Active,
            Self::AEntity::Overload => Self::Overload,
        }
    }

    fn into_adapted(self) -> Self::AEntity {
        match self {
            Self::Disabled => Self::AEntity::Disabled,
            Self::Offline => Self::AEntity::Offline,
            Self::Online => Self::AEntity::Online,
            Self::Active => Self::AEntity::Active,
            Self::Overload => Self::AEntity::Overload,
        }
    }
}
