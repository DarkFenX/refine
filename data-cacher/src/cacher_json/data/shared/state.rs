#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::cacher_json::data) enum CState {
    Disabled,
    Offline,
    Online,
    Active,
    Overload,
}
impl CState {
    pub(in crate::cacher_json::data) fn from_adapted(a_state: &rc::ad::AState) -> Self {
        match a_state {
            rc::ad::AState::Disabled => Self::Disabled,
            rc::ad::AState::Offline => Self::Offline,
            rc::ad::AState::Online => Self::Online,
            rc::ad::AState::Active => Self::Active,
            rc::ad::AState::Overload => Self::Overload,
        }
    }
    pub(in crate::cacher_json::data) fn into_adapted(self) -> rc::ad::AState {
        match self {
            Self::Disabled => rc::ad::AState::Disabled,
            Self::Offline => rc::ad::AState::Offline,
            Self::Online => rc::ad::AState::Online,
            Self::Active => rc::ad::AState::Active,
            Self::Overload => rc::ad::AState::Overload,
        }
    }
}
