use crate::def::DefCount;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum InfCount {
    Count(DefCount),
    Infinite,
}
