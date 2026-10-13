use crate::ad::AState;

const STATES: [RState; 6] = [
    RState::Ghost,
    RState::Disabled,
    RState::Offline,
    RState::Online,
    RState::Active,
    RState::Overload,
];

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum RState {
    Ghost,
    Disabled,
    Offline,
    Online,
    Active,
    Overload,
}
impl RState {
    pub(crate) fn above_up_to(low: Self, high: Self) -> &'static [Self] {
        &STATES[low as usize + 1..=high as usize]
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RState {
    pub(in crate::rd::data) fn from_a_state(a_state: &AState) -> Self {
        match a_state {
            AState::Disabled => Self::Disabled,
            AState::Offline => Self::Offline,
            AState::Online => Self::Online,
            AState::Active => Self::Active,
            AState::Overload => Self::Overload,
        }
    }
}
