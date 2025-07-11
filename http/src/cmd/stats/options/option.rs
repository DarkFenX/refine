#[serde_with::serde_as]
#[derive(serde::Deserialize)]
#[serde(untagged)]
pub(in crate::cmd) enum HStatOption<T>
where
    T: Copy + Clone + Default,
{
    Simple(bool),
    Extended(bool, Vec<T>),
}
impl<T> HStatOption<T>
where
    T: Copy + Clone + Default,
{
    pub(in crate::cmd) fn is_enabled(&self) -> bool {
        match self {
            Self::Simple(enabled) => *enabled,
            Self::Extended(enabled, _) => *enabled,
        }
    }
    pub(in crate::cmd) fn get_extended_options(&self) -> Vec<T> {
        match self {
            Self::Simple(_) => vec![T::default()],
            Self::Extended(_, extended_options) => extended_options.clone(),
        }
    }
}
