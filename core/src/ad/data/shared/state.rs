const STATES: [AState; 6] = [
    AState::Ghost,
    AState::Disabled,
    AState::Offline,
    AState::Online,
    AState::Active,
    AState::Overload,
];

/// States which are used on items and effects.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AState {
    /// Item will receive modifications (thus its modified attributes can be checked), but its
    /// effects will not be active, regardless of their mode.
    Ghost,
    /// State internal to the lib; only effects which are force-run will be active.
    Disabled,
    /// For modules/services it means offline state, for drones/fighters it means that it is in
    /// drone bay.
    Offline,
    /// For modules/services it means online state, for drones/fighters it means that drone is in
    /// space.
    Online,
    /// For modules it means active state, for drones/fighters it means state of engaging a target.
    Active,
    /// For modules it means overloaded state.
    Overload,
}
impl AState {
    pub fn iter() -> std::slice::Iter<'static, Self> {
        STATES.iter()
    }
}
