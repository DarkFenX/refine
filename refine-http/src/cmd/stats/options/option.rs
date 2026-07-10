use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
pub(in crate::cmd::stats) enum HStatOption<T>
where
    T: Clone + Default,
{
    Simple(bool),
    Extended(bool, Vec<T>),
}
impl<T> HStatOption<T>
where
    T: Clone + Default,
{
    pub(in crate::cmd::stats) fn is_enabled(&self) -> bool {
        match self {
            Self::Simple(enabled) => *enabled,
            Self::Extended(enabled, _) => *enabled,
        }
    }
    pub(in crate::cmd::stats) fn get_extended_options(&self) -> Vec<T> {
        match self {
            Self::Simple(_) => vec![T::default()],
            Self::Extended(_, extended_options) => extended_options.clone(),
        }
    }
}
