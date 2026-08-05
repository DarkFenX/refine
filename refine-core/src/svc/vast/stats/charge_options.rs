/// Some chargeable items can carry "meaningful" effect (e.g. effect which applies damage) on item
/// itself, or on charge. When fetching item stat, this setting controls if on-charge effects would
/// be included in item's stats or not.
#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum StatItemChargeOptions {
    Include,
    Exclude,
}
const impl Default for StatItemChargeOptions {
    fn default() -> Self {
        Self::Exclude
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatItemChargeOptions {
    pub(crate) fn is_enabled(&self) -> bool {
        match self {
            Self::Include => true,
            Self::Exclude => false,
        }
    }
}
