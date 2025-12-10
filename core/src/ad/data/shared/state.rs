const STATES: [AState; 6] = [
    AState::Ghost,
    AState::Disabled,
    AState::Offline,
    AState::Online,
    AState::Active,
    AState::Overload,
];

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AState {
    Ghost,
    Disabled,
    Offline,
    Online,
    Active,
    Overload,
}
impl AState {
    pub fn iter() -> std::slice::Iter<'static, Self> {
        STATES.iter()
    }
}
