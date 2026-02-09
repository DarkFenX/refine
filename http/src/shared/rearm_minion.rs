use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HRearmMinion {
    Disabled,
    OnFirstEmpty,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HRearmMinion {
    pub(crate) fn from_core(core_optional_reload: rc::RearmMinion) -> Self {
        match core_optional_reload {
            rc::RearmMinion::Disabled => Self::Disabled,
            rc::RearmMinion::OnFirstEmpty => Self::OnFirstEmpty,
        }
    }
    pub(crate) fn into_core(self) -> rc::RearmMinion {
        match self {
            Self::Disabled => rc::RearmMinion::Disabled,
            Self::OnFirstEmpty => rc::RearmMinion::OnFirstEmpty,
        }
    }
}
