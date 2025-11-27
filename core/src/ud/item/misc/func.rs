use crate::ad;

pub(in crate::ud::item) fn bool_to_state_offline(bool_state: bool) -> ad::AState {
    match bool_state {
        true => ad::AState::Offline,
        false => ad::AState::Disabled,
    }
}

pub(in crate::ud::item) fn bool_to_state_active(bool_state: bool) -> ad::AState {
    match bool_state {
        true => ad::AState::Active,
        false => ad::AState::Disabled,
    }
}

pub(in crate::ud::item) fn state_to_bool(state: ad::AState) -> bool {
    match state {
        ad::AState::Ghost | ad::AState::Disabled => false,
        ad::AState::Offline | ad::AState::Online | ad::AState::Active | ad::AState::Overload => true,
    }
}
