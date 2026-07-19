#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[derive(Copy, Clone, Eq, PartialEq)]
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
