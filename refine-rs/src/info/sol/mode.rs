#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum SolInfoMode {
    Id,
    Partial,
    Full,
}
const impl Default for SolInfoMode {
    fn default() -> Self {
        Self::Id
    }
}
