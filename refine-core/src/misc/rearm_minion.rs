/// Controls when fighters are recalled for rearming/refueling.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[derive(Copy, Clone)]
pub enum RearmMinion {
    /// No rearm - fighter is kept out even when some of its abilities are out of charges.
    Disabled,
    /// Recall fighter when first of its abilities runs out of charges, after final cycle is
    /// completed.
    OnFirstEmpty,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct ItemRearmMinionInfo {
    /// Effective value of item's "rearm minion" setting.
    pub value: RearmMinion,
    /// True if setting is defined directly on item, false if inherited from sol.
    pub overridden: bool,
}
