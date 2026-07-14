#[derive(Copy, Clone, Eq, PartialEq)]
pub enum NpcProp {
    Cruise,
    Chase,
}

#[derive(Copy, Clone)]
pub struct ItemNpcPropInfo {
    /// Effective value of item's "NPC prop" setting.
    pub value: NpcProp,
    /// True if setting is defined directly on item, false if inherited from sol.
    pub overridden: bool,
}
