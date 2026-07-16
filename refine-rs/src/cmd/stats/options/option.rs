#[derive(Copy, Clone, Default)]
pub enum StatOption {
    #[default]
    Default,
    Disabled,
    Enabled,
}

#[derive(Clone, Default)]
pub enum StatOptionExt<T>
where
    T: Clone,
{
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
    pub(in crate::cmd::stats) fn is_enabled(&self, default: bool) -> bool {
        match self {
            Self::Default => default,
            Self::Disabled => false,
            Self::Enabled => true,
        }
    }
}
