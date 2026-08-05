/// Some chargeable items can carry "meaningful" effect (e.g. effect which applies damage) on item
/// itself, or on charge. When fetching item stat, this setting controls if on-charge effects would
/// be included in item's stats or not.
#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum StatChargeOptions {
    Include,
    Exclude,
}
const impl Default for StatChargeOptions {
    fn default() -> Self {
        StatChargeOptions::Exclude
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatChargeOptions {
    pub(crate) fn is_enabled(&self) -> bool {
        match self {
            StatChargeOptions::Include => true,
            StatChargeOptions::Exclude => false,
        }
    }
}
