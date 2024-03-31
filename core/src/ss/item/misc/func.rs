use crate::shr::State;

pub(in crate::ss::item) fn bool_to_state(bool_state: bool) -> State {
    match bool_state {
        true => State::Active,
        false => State::Ghost,
    }
}

pub(in crate::ss::item) fn state_to_bool(state: State) -> bool {
    match state {
        State::Ghost => false,
        _ => true,
    }
}
