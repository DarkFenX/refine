#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone)]
pub enum ValInfoMode {
    Simple,
    Detailed,
}
const impl Default for ValInfoMode {
    fn default() -> Self {
        Self::Detailed
    }
}
