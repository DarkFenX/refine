use super::SolItemState;

pub(in crate::sol::uad::item) fn bool_to_state(bool_state: bool) -> SolItemState {
    match bool_state {
        true => SolItemState::Active,
        false => SolItemState::Ghost,
    }
}

pub(in crate::sol::uad::item) fn state_to_bool(state: SolItemState) -> bool {
    match state {
        SolItemState::Ghost => false,
        _ => true,
    }
}
