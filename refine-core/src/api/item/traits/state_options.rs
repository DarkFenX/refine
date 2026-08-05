/// Items apply their effects when they reach high enough state. This setting controls if item state
/// is kept as-is, or it is switched up to apply its effect for purpose of the stat being fetched.
#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum StatItemStateOptions {
    Retain,
    Switch,
}
const impl Default for StatItemStateOptions {
    fn default() -> Self {
        Self::Retain
    }
}
