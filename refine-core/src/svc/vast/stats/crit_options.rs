/// Some items can experience critical hits; this option controls if those crits are included in
/// final stat value or not.
///
/// When disabled, critical hit chance is just assumed to be 0.
#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum StatCritOptions {
    Include,
    Exclude,
}
const impl Default for StatCritOptions {
    fn default() -> Self {
        Self::Include
    }
}
