use crate::src::SrcInfoMode;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct SrcInfoArgs {
    #[cfg_attr(feature = "serde", serde(default))]
    pub src: SrcInfoMode,
}
