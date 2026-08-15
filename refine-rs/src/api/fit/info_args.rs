use crate::val::ValInfoMode;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct ValFitInfoArgs {
    #[cfg_attr(feature = "serde", serde(default))]
    pub validation: ValInfoMode,
}
