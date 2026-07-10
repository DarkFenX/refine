use crate::num::Count;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum InfCount {
    Count(Count),
    Infinite,
}
