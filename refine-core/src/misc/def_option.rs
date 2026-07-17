/// An option which can inherit .
#[derive(Copy, Clone, Default)]
pub enum DefOption {
    #[default]
    Default,
    Disabled,
    Enabled,
}

/// An option which can have some default value.
#[derive(Copy, Clone, Default)]
pub enum DefOptionExt<T> {
    #[default]
    Default,
    Disabled,
    Enabled,
    EnabledExtended(T),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl DefOption {
    pub(crate) fn into_enabled(self, default: bool) -> bool {
        match self {
            Self::Default => default,
            Self::Enabled => true,
            Self::Disabled => false,
        }
    }
    pub(crate) fn is_enabled(&self, default: bool) -> bool {
        match self {
            Self::Default => default,
            Self::Enabled => true,
            Self::Disabled => false,
        }
    }
}

impl<T> DefOptionExt<T> {
    pub(crate) fn into_enabled(self, default: bool) -> Option<T>
    where
        T: Default,
    {
        match self {
            Self::Default => match default {
                false => None,
                true => Some(Default::default()),
            },
            Self::Disabled => None,
            Self::Enabled => Some(T::default()),
            Self::EnabledExtended(settings) => Some(settings),
        }
    }
    pub(crate) fn is_enabled(&self, default: bool) -> Option<T>
    where
        T: Default + Copy,
    {
        match self {
            Self::Default => match default {
                false => None,
                true => Some(T::default()),
            },
            Self::Disabled => None,
            Self::Enabled => Some(T::default()),
            Self::EnabledExtended(settings) => Some(*settings),
        }
    }
}
