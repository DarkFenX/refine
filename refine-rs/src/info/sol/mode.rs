#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone)]
pub enum SolInfoMode {
    Id,
    Full,
}
const impl Default for SolInfoMode {
    fn default() -> Self {
        Self::Full
    }
}
