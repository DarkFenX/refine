/// An option which can have some default value.
#[derive(Copy, Clone, Default)]
pub enum DefOption {
    #[default]
    Default,
    Enabled,
    Disabled,
}

impl DefOption {
    pub(crate) fn is_enabled(&self, default: bool) -> bool {
        match self {
            Self::Default => default,
            Self::Enabled => true,
            Self::Disabled => false,
        }
    }
}
