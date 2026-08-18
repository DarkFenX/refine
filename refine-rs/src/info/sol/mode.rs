#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub enum SolInfoMode {
    Id,
    #[default]
    Full,
}
