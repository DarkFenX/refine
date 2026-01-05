use crate::rd::RState;

pub(in crate::ud::item) fn bool_to_state_offline(bool_state: bool) -> RState {
    match bool_state {
        true => RState::Offline,
        false => RState::Disabled,
    }
}

pub(in crate::ud::item) fn bool_to_state_active(bool_state: bool) -> RState {
    match bool_state {
        true => RState::Active,
        false => RState::Disabled,
    }
}

pub(in crate::ud::item) fn state_to_bool(state: RState) -> bool {
    match state {
        RState::Ghost | RState::Disabled => false,
        RState::Offline | RState::Online | RState::Active | RState::Overload => true,
    }
}
