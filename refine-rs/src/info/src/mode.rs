#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone)]
pub enum SrcInfoMode {
    Alias,
    Partial,
    Full,
}
const impl Default for SrcInfoMode {
    fn default() -> Self {
        Self::Alias
    }
}
