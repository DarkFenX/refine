use crate::ad;

pub(in crate::uad::item) fn bool_to_state_offline(bool_state: bool) -> ad::AState {
    match bool_state {
        true => ad::AState::Offline,
        false => ad::AState::Ghost,
    }
}

pub(in crate::uad::item) fn bool_to_state_active(bool_state: bool) -> ad::AState {
    match bool_state {
        true => ad::AState::Active,
        false => ad::AState::Ghost,
    }
}

pub(in crate::uad::item) fn state_to_bool(state: ad::AState) -> bool {
    !matches!(state, ad::AState::Ghost)
}
