use std::cmp::Ordering;

use crate::ad;

static STATES: [SolItemState; 5] = [
    SolItemState::Ghost,
    SolItemState::Offline,
    SolItemState::Online,
    SolItemState::Active,
    SolItemState::Overload,
];

/// States which are used by all items internally, and are exposed for some items such as modules
/// and drones.
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub(in crate::sol) enum SolItemState {
    /// Item will receive modifications (thus its modified attributes can be checked), but will be
    /// considered as absent otherwise.
    Ghost,
    /// For modules it means offline state, for drones it means that it is in drone bay.
    Offline,
    /// For modules it means online state, for drones it means that drone is in space.
    Online,
    /// For modules it means active state, for drones it means that drone is engaging its target.
    Active,
    /// For modules it means overloaded state, for drones it doesn't mean anything special.
    Overload,
}
impl SolItemState {
    pub(in crate::sol) fn iter() -> std::slice::Iter<'static, Self> {
        STATES.iter()
    }
}
impl PartialEq<ad::AState> for SolItemState {
    fn eq(&self, other: &ad::AState) -> bool {
        matches!(
            (self, other),
            (Self::Offline, ad::AState::Offline)
                | (Self::Online, ad::AState::Online)
                | (Self::Active, ad::AState::Active)
                | (Self::Overload, ad::AState::Overload)
        )
    }
}
impl PartialOrd<ad::AState> for SolItemState {
    fn partial_cmp(&self, other: &ad::AState) -> Option<Ordering> {
        match self {
            Self::Ghost => Some(Ordering::Less),
            Self::Offline => match other {
                ad::AState::Offline => Some(Ordering::Equal),
                _ => Some(Ordering::Less),
            },
            Self::Online => match other {
                ad::AState::Offline => Some(Ordering::Greater),
                ad::AState::Online => Some(Ordering::Equal),
                _ => Some(Ordering::Less),
            },
            Self::Active => match other {
                ad::AState::Overload => Some(Ordering::Less),
                ad::AState::Active => Some(Ordering::Equal),
                _ => Some(Ordering::Greater),
            },
            Self::Overload => match other {
                ad::AState::Overload => Some(Ordering::Equal),
                _ => Some(Ordering::Greater),
            },
        }
    }
}
impl PartialEq<SolItemState> for ad::AState {
    fn eq(&self, other: &SolItemState) -> bool {
        other.eq(self)
    }
}
impl PartialOrd<SolItemState> for ad::AState {
    fn partial_cmp(&self, other: &SolItemState) -> Option<Ordering> {
        other.partial_cmp(self).map(|v| v.reverse())
    }
}
