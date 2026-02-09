use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HOptionalReload {
    Disabled,
    OnEmpty,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HOptionalReload {
    pub(crate) fn from_core(core_optional_reload: rc::OptionalReload) -> Self {
        match core_optional_reload {
            rc::OptionalReload::Disabled => Self::Disabled,
            rc::OptionalReload::OnEmpty => Self::OnEmpty,
        }
    }
    pub(crate) fn into_core(self) -> rc::OptionalReload {
        match self {
            Self::Disabled => rc::OptionalReload::Disabled,
            Self::OnEmpty => rc::OptionalReload::OnEmpty,
        }
    }
}
