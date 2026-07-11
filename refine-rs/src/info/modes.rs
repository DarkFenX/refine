#[derive(Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum SrcInfoMode {
    Partial,
    #[default]
    Full,
}
