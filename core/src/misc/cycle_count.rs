use crate::def::Count;

pub(crate) enum CycleCount {
    Infinite,
    Count(Count),
}
