use crate::def::Count;

#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum InfCount {
    Count(Count),
    Infinite,
}
