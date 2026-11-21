#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone)]
pub enum ValResultMode {
    Simple,
    Detailed,
}
const impl Default for ValResultMode {
    fn default() -> Self {
        Self::Simple
    }
}
