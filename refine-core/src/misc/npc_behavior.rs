/// Defines if entities with their own AI (like drones) are using their propulsion mode or not.
///
/// It does not apply to all the entities. Some, like mining drones, do not have a "propulsion
/// mode", and this setting will have no effect. On others, it will affect speed and signature
/// radius.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, derive_more::Display)]
#[display(rename_all = "snake_case")]
pub enum NpcProp {
    Cruise,
    Chase,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct ItemNpcPropInfo {
    /// Effective value of item's "NPC prop" setting.
    pub value: NpcProp,
    /// True if setting is defined directly on item, false if inherited from sol.
    pub overridden: bool,
}
