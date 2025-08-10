use crate::def::Count;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum InfCount {
    Count(Count),
    Infinite,
}
