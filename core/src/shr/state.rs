/// Contains states which can be assigned to several entities.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum State {
    Ghost,
    Offline,
    Online,
    Active,
    Overload,
}
impl State {
    pub(crate) fn iter() -> std::slice::Iter<'static, State> {
        static STATES: [State; 5] = [
            State::Ghost,
            State::Offline,
            State::Online,
            State::Active,
            State::Overload,
        ];
        STATES.iter()
    }
}
