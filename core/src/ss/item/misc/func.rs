use super::SsItemState;

pub(in crate::ss::item) fn bool_to_state(bool_state: bool) -> SsItemState {
    match bool_state {
        true => SsItemState::Active,
        false => SsItemState::Ghost,
    }
}

pub(in crate::ss::item) fn state_to_bool(state: SsItemState) -> bool {
    match state {
        SsItemState::Ghost => false,
        _ => true,
    }
}
