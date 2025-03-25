const STATES: [AState; 5] = [
    AState::Ghost,
    AState::Offline,
    AState::Online,
    AState::Active,
    AState::Overload,
];

/// States which are used on items and effects.
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum AState {
    /// Item will receive modifications (thus its modified attributes can be checked), but will be
    /// considered as absent otherwise.
    Ghost,
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
