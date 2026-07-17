#[derive(Copy, Clone, Default)]
pub enum StatOption {
    #[default]
    Default,
    Disabled,
    Enabled,
}

#[derive(Clone, Default)]
pub enum StatOptionExt<T> {
    #[default]
    Default,
    Disabled,
    Enabled,
    EnabledExtended(Vec<T>),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatOption {
    pub(in crate::cmd::stats) fn into_enabled(self, default: bool) -> bool {
        match self {
            Self::Default => default,
            Self::Disabled => false,
            Self::Enabled => true,
        }
    }
}

impl<T> StatOptionExt<T> {
    pub(in crate::cmd::stats) fn into_enabled(self, default: bool) -> Option<Vec<T>>
    where
        T: Default,
    {
        match self {
            Self::Default => match default {
                true => Some(vec![T::default()]),
                false => None,
            },
            Self::Disabled => None,
            Self::Enabled => Some(vec![T::default()]),
            Self::EnabledExtended(options) => Some(options),
        }
    }
}
